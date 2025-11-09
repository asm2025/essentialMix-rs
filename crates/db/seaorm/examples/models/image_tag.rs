// Example schema implementation using emixdb
// This file demonstrates how to use the emixdb crate to create SeaORM entities

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// Example: ImageTag junction entity
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "image_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub image_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i64,
}

// In a real implementation, replace these with your actual module paths:
// use super::image;
// use super::tag;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::image::Entity",
        from = "Column::ImageId",
        to = "super::image::Column::Id"
    )]
    ImageEntity,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    TagEntity,
}

impl ActiveModelBehavior for ActiveModel {}

pub use ActiveModel as ImageTagModelDto;
pub use Column as ImageTagColumn;
pub use Entity as ImageTagEntity;
pub use Model as ImageTagModel;
