// Example schema implementation using emixdb
// This file demonstrates how to use the emixdb crate to create SeaORM entities

use sea_orm::{EntityTrait, NotSet, Set, prelude::*};
use serde::{Deserialize, Serialize};

use emixdb::schema::Merge;

// Example: Tag entity
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
}

// In a real implementation, replace these with your actual module paths:
// use super::image;
// use super::image_tag;
use crate::image;
use crate::image_tag;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::image_tag::Entity")]
    // In a real implementation, use: #[sea_orm(has_many = "super::image_tag::Entity")]
    ImageTag,
}

impl Related<image::Entity> for Entity {
    fn to() -> RelationDef {
        image_tag::Relation::ImageEntity.def()
    }
    fn via() -> Option<RelationDef> {
        Some(image_tag::Relation::TagEntity.def().rev())
    }
}

impl Related<Entity> for image_tag::Entity {
    fn to() -> RelationDef {
        image_tag::Relation::TagEntity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateTagDto {
    pub name: String,
}

impl From<CreateTagDto> for Model {
    fn from(req: CreateTagDto) -> Self {
        Self {
            id: 0,
            name: req.name,
        }
    }
}

impl From<CreateTagDto> for ActiveModel {
    fn from(req: CreateTagDto) -> Self {
        Self {
            id: NotSet,
            name: Set(req.name),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagDto {
    pub name: Option<String>,
}

impl Merge<ActiveModel> for UpdateTagDto {
    fn merge(&self, model: &mut ActiveModel) {
        if let Some(name) = self.name.as_ref() {
            model.name = Set(name.clone());
        }
    }
}

pub use ActiveModel as TagModelDto;
pub use Column as TagColumn;
pub use Entity as TagEntity;
pub use Model as TagModel;

