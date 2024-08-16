use async_graphql::{Enum, SimpleObject};
use enums::MediaLot;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

/// The various steps in which media importing can fail
#[derive(Debug, Enum, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum ImportFailStep {
    /// Failed to get details from the source itself (for eg: MediaTracker, Goodreads etc.)
    ItemDetailsFromSource,
    /// Failed to get metadata from the provider (for eg: Openlibrary, IGDB etc.)
    MediaDetailsFromProvider,
    /// Failed to transform the data into the required format
    InputTransformation,
    /// Failed to save a seen history item
    SeenHistoryConversion,
    /// Failed to save a review/rating item
    ReviewConversion,
}

#[derive(
    Debug, SimpleObject, FromJsonQueryResult, Serialize, Deserialize, Eq, PartialEq, Clone,
)]
pub struct ImportFailedItem {
    pub lot: Option<MediaLot>,
    pub step: ImportFailStep,
    pub identifier: String,
    pub error: Option<String>,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ImportDetails {
    pub total: usize,
}

#[derive(
    Debug, SimpleObject, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq, Clone,
)]
pub struct ImportResultResponse {
    pub import: ImportDetails,
    pub failed_items: Vec<ImportFailedItem>,
}