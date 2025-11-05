// Example schema implementation using emixdb
// This file demonstrates how to use the emixdb crate to create SeaORM entities

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{EntityTrait, NotSet, Set, prelude::*};
use serde::{Deserialize, Serialize};

use emixdb::schema::Merge;

// Example: Image entity
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub extension: String,
    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::image_tag::Entity")]
    // In a real implementation, use: #[sea_orm(has_many = "super::image_tag::Entity")]
    ImageTag,
}

// In a real implementation, replace these with your actual module paths:
// use super::tag;
// use super::image_tag;
use crate::image_tag;
use crate::tag;

impl Related<tag::Entity> for Entity {
    fn to() -> RelationDef {
        image_tag::Relation::TagEntity.def()
    }
    fn via() -> Option<RelationDef> {
        Some(image_tag::Relation::ImageEntity.def().rev())
    }
}

impl Related<Entity> for image_tag::Entity {
    fn to() -> RelationDef {
        image_tag::Relation::ImageEntity.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: Set(now),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now();

        if insert {
            self.created_at = Set(now);
            self.updated_at = Set(now);
        } else {
            self.updated_at = Set(now);
        }
        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateImageDto {
    pub title: String,
    pub description: Option<String>,
    pub extension: String,
    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub tags: Option<String>,
}

impl From<CreateImageDto> for Model {
    fn from(req: CreateImageDto) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            title: req.title,
            description: req.description,
            extension: req.extension,
            file_size: req.file_size,
            mime_type: req.mime_type,
            width: req.width,
            height: req.height,
            alt_text: req.alt_text,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<CreateImageDto> for ActiveModel {
    fn from(req: CreateImageDto) -> Self {
        Self {
            id: NotSet,
            title: Set(req.title),
            description: Set(req.description),
            extension: Set(req.extension),
            file_size: Set(req.file_size),
            mime_type: Set(req.mime_type),
            width: Set(req.width),
            height: Set(req.height),
            alt_text: Set(req.alt_text),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateImageDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub extension: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
}

impl Merge<ActiveModel> for UpdateImageDto {
    fn merge(&self, model: &mut ActiveModel) -> bool {
        // can also use: if let Some(title) = self.title.as_ref() {
        let mut changed = false;

        if let Some(ref title) = self.title {
            model.title = Set(title.clone());
            changed = true;
        }

        if let Some(ref description) = self.description {
            model.description = Set(Some(description.clone()));
            changed = true;
        }

        if let Some(ref extension) = self.extension {
            model.extension = Set(extension.clone());
            changed = true;
        }

        if let Some(file_size) = self.file_size {
            model.file_size = Set(file_size);
            changed = true;
        }

        if let Some(ref mime_type) = self.mime_type {
            model.mime_type = Set(mime_type.clone());
            changed = true;
        }

        if let Some(width) = self.width {
            model.width = Set(Some(width));
            changed = true;
        }

        if let Some(height) = self.height {
            model.height = Set(Some(height));
            changed = true;
        }

        if let Some(ref alt_text) = self.alt_text {
            model.alt_text = Set(Some(alt_text.clone()));
            changed = true;
        }

        changed
    }
}

pub use ActiveModel as ImageModelDto;
pub use Column as ImageColumn;
pub use Entity as ImageEntity;
pub use Model as ImageModel;
