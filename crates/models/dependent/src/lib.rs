use async_graphql::{InputObject, OutputType, SimpleObject, Union};
use common_models::{BackendError, SearchDetails};
use config::FrontendConfig;
use database_models::{
    collection, exercise, metadata, metadata_group, person, seen, user, user_measurement,
    user_to_entity, workout, workout_template,
};
use enums::{
    ExerciseEquipment, ExerciseForce, ExerciseLevel, ExerciseLot, ExerciseMechanic, ExerciseMuscle,
    MediaLot, MediaSource, UserToMediaReason, WorkoutSetPersonalBest,
};
use fitness_models::{UserToExerciseHistoryExtraInformation, UserWorkoutInput};
use importer_models::ImportFailedItem;
use media_models::{
    CreateOrUpdateCollectionInput, EntityWithLot, GenreListItem, GraphqlMediaAssets,
    ImportOrExportExerciseItem, ImportOrExportMetadataGroupItem, ImportOrExportMetadataItem,
    ImportOrExportPersonItem, MetadataCreatorGroupedByRole, PersonDetailsGroupedByRole, ReviewItem,
    UserDetailsError, UserMediaNextEntry, UserMetadataDetailsEpisodeProgress,
    UserMetadataDetailsShowSeasonProgress,
};
use rust_decimal::Decimal;
use schematic::Schematic;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::Display;

#[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
#[graphql(concrete(name = "ExerciseListResults", params(fitness_models::ExerciseListItem)))]
#[graphql(concrete(
    name = "MediaCollectionContentsResults",
    params(media_models::EntityWithLot)
))]
#[graphql(concrete(
    name = "MetadataSearchResults",
    params(media_models::MetadataSearchItemResponse)
))]
#[graphql(concrete(name = "PeopleSearchResults", params(media_models::PeopleSearchItem)))]
#[graphql(concrete(
    name = "MetadataGroupSearchResults",
    params(media_models::MetadataGroupSearchItem)
))]
#[graphql(concrete(name = "GenreListResults", params(media_models::GenreListItem)))]
#[graphql(concrete(name = "WorkoutListResults", params(workout::Model)))]
#[graphql(concrete(name = "WorkoutTemplateListResults", params(workout_template::Model)))]
#[graphql(concrete(name = "IdResults", params(String)))]
pub struct SearchResults<T: OutputType> {
    pub details: SearchDetails,
    pub items: Vec<T>,
}

/// Details about a specific exercise item that needs to be exported.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Schematic)]
#[serde(rename_all = "snake_case")]
pub struct ImportOrExportWorkoutItem {
    /// The details of the workout.
    pub details: workout::Model,
    /// The collections this entity was added to.
    pub collections: Vec<String>,
}

#[derive(Debug, SimpleObject, Clone, Serialize, Deserialize, Schematic)]
pub struct ImportOrExportWorkoutTemplateItem {
    pub details: workout_template::Model,
    pub collections: Vec<String>,
}

/// Complete export of the user.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Schematic)]
#[serde(rename_all = "snake_case")]
pub struct CompleteExport {
    /// Data about user's media.
    pub media: Option<Vec<media_models::ImportOrExportMetadataItem>>,
    /// Data about user's people.
    pub people: Option<Vec<media_models::ImportOrExportPersonItem>>,
    /// Data about user's measurements.
    pub measurements: Option<Vec<user_measurement::Model>>,
    /// Data about user's workouts.
    pub workouts: Option<Vec<ImportOrExportWorkoutItem>>,
    /// Data about user's media groups.
    pub media_groups: Option<Vec<media_models::ImportOrExportMetadataGroupItem>>,
    /// Data about user's exercises.
    pub exercises: Option<Vec<ImportOrExportExerciseItem>>,
    /// Data about user's workout templates.
    pub workout_templates: Option<Vec<ImportOrExportWorkoutTemplateItem>>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct UserWorkoutDetails {
    pub details: workout::Model,
    pub collections: Vec<collection::Model>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct UserExerciseDetails {
    pub details: Option<user_to_entity::Model>,
    pub history: Option<Vec<UserToExerciseHistoryExtraInformation>>,
    pub collections: Vec<collection::Model>,
    pub reviews: Vec<ReviewItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize, InputObject)]
pub struct UpdateCustomExerciseInput {
    pub old_id: String,
    pub should_delete: Option<bool>,
    #[graphql(flatten)]
    pub update: exercise::Model,
}

#[derive(Union)]
pub enum UserDetailsResult {
    Ok(Box<user::Model>),
    Error(UserDetailsError),
}

#[derive(Debug, SimpleObject)]
pub struct CollectionContents {
    pub details: collection::Model,
    pub results: SearchResults<EntityWithLot>,
    pub reviews: Vec<ReviewItem>,
    pub user: user::Model,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct PersonDetails {
    pub details: person::Model,
    pub contents: Vec<PersonDetailsGroupedByRole>,
    pub source_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct MetadataGroupDetails {
    pub details: metadata_group::Model,
    pub source_url: Option<String>,
    pub contents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct GenreDetails {
    pub details: GenreListItem,
    pub contents: SearchResults<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataBaseData {
    pub model: metadata::Model,
    pub suggestions: Vec<String>,
    pub genres: Vec<GenreListItem>,
    pub assets: GraphqlMediaAssets,
    pub creators: Vec<MetadataCreatorGroupedByRole>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct ExerciseParametersLotMapping {
    pub lot: ExerciseLot,
    pub bests: Vec<WorkoutSetPersonalBest>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct ExerciseFilters {
    #[graphql(name = "type")]
    pub lot: Vec<ExerciseLot>,
    pub level: Vec<ExerciseLevel>,
    pub force: Vec<ExerciseForce>,
    pub mechanic: Vec<ExerciseMechanic>,
    pub equipment: Vec<ExerciseEquipment>,
    pub muscle: Vec<ExerciseMuscle>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct ExerciseParameters {
    /// All filters applicable to an exercises query.
    pub filters: ExerciseFilters,
    pub download_required: bool,
    /// Exercise type mapped to the personal bests possible.
    pub lot_mapping: Vec<ExerciseParametersLotMapping>,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct ProviderLanguageInformation {
    pub source: MediaSource,
    pub supported: Vec<String>,
    pub default: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct MetadataLotSourceMappings {
    pub lot: MediaLot,
    pub sources: Vec<MediaSource>,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct CoreDetails {
    pub page_size: i32,
    pub version: String,
    pub docs_link: String,
    pub oidc_enabled: bool,
    pub smtp_enabled: bool,
    pub website_url: String,
    pub signup_allowed: bool,
    pub disable_telemetry: bool,
    pub repository_link: String,
    pub frontend: FrontendConfig,
    pub token_valid_for_days: i32,
    pub local_auth_disabled: bool,
    pub file_storage_enabled: bool,
    pub is_server_key_validated: bool,
    pub backend_errors: Vec<BackendError>,
    pub exercise_parameters: ExerciseParameters,
    pub metadata_lot_source_mappings: Vec<MetadataLotSourceMappings>,
    pub metadata_provider_languages: Vec<ProviderLanguageInformation>,
}

#[derive(SimpleObject)]
pub struct UserPersonDetails {
    pub recently_consumed: bool,
    pub reviews: Vec<ReviewItem>,
    pub collections: Vec<collection::Model>,
}

#[derive(SimpleObject)]
pub struct UserMetadataGroupDetails {
    pub recently_consumed: bool,
    pub reviews: Vec<ReviewItem>,
    pub collections: Vec<collection::Model>,
}

#[derive(SimpleObject)]
pub struct UserMetadataDetails {
    /// Whether this media has been interacted with
    pub has_interacted: bool,
    /// Whether this media has been recently interacted with
    pub recently_consumed: bool,
    /// The public reviews of this media.
    pub reviews: Vec<ReviewItem>,
    /// The number of users who have seen this media.
    pub seen_by_all_count: usize,
    /// The number of times this user has seen this media.
    pub seen_by_user_count: usize,
    /// The seen history of this media.
    pub history: Vec<seen::Model>,
    /// The average rating of this media in this service.
    pub average_rating: Option<Decimal>,
    /// The seen item if it is in progress.
    pub in_progress: Option<seen::Model>,
    /// The collections in which this media is present.
    pub collections: Vec<collection::Model>,
    /// The next episode/chapter of this media.
    pub next_entry: Option<UserMediaNextEntry>,
    /// The reasons why this metadata is related to this user
    pub media_reason: Option<Vec<UserToMediaReason>>,
    /// The seen progress of this media if it is a show.
    pub show_progress: Option<Vec<UserMetadataDetailsShowSeasonProgress>>,
    /// The seen progress of this media if it is a podcast.
    pub podcast_progress: Option<Vec<UserMetadataDetailsEpisodeProgress>>,
}

#[derive(Debug, Default, Display, Clone, Serialize)]
pub enum ImportCompletedItem {
    #[default]
    Empty,
    Workout(UserWorkoutInput),
    Exercise(exercise::Model),
    Person(ImportOrExportPersonItem),
    Metadata(ImportOrExportMetadataItem),
    Measurement(user_measurement::Model),
    Collection(CreateOrUpdateCollectionInput),
    MetadataGroup(ImportOrExportMetadataGroupItem),
    ApplicationWorkout(ImportOrExportWorkoutItem),
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ImportResult {
    pub failed: Vec<ImportFailedItem>,
    pub completed: Vec<ImportCompletedItem>,
}

#[derive(Debug, SimpleObject, Clone, Serialize, Deserialize, Schematic)]
pub struct UserWorkoutTemplateDetails {
    pub details: workout_template::Model,
    pub collections: Vec<collection::Model>,
}
