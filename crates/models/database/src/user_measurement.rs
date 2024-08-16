//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use async_graphql::{InputObject, SimpleObject};
use fitness_models::UserMeasurementStats;
use schematic::Schematic;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An export of a measurement taken at a point in time.
#[skip_serializing_none]
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    SimpleObject,
    InputObject,
    Schematic,
)]
#[graphql(name = "UserMeasurement", input_name = "UserMeasurementInput")]
#[schematic(rename = "UserMeasurement", rename_all = "snake_case")]
#[sea_orm(table_name = "user_measurement")]
pub struct Model {
    /// The date and time this measurement was made.
    #[sea_orm(primary_key, auto_increment = false)]
    pub timestamp: DateTimeUtc,
    #[graphql(skip)]
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip)]
    pub user_id: String,
    /// The name given to this measurement by the user.
    pub name: Option<String>,
    /// Any comment associated entered by the user.
    pub comment: Option<String>,
    /// The contents of the actual measurement.
    pub stats: UserMeasurementStats,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}