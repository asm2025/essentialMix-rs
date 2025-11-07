// Example schema implementation using emix-diesel
// This file demonstrates how to use the emix-diesel crate to create Diesel models

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::tables::images;
use emixdiesel::prelude::*;

// Example: Image model
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable,
)]
#[diesel(table_name = images)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ImageModel {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub extension: String,
    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = images)]
pub struct NewImageModel {
    pub title: String,
    pub description: Option<String>,
    pub extension: String,
    pub file_size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = images)]
pub struct UpdateImageModel {
    pub title: Option<String>,
    pub description: Option<String>,
    pub extension: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub updated_at: NaiveDateTime,
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

impl From<CreateImageDto> for NewImageModel {
    fn from(req: CreateImageDto) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
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

impl Merge<UpdateImageModel> for UpdateImageDto {
    fn merge(&self, model: &mut UpdateImageModel) -> bool {
        let mut changed = false;

        if let Some(ref title) = self.title {
            model.title = Some(title.clone());
            changed = true;
        }

        if let Some(ref description) = self.description {
            model.description = Some(description.clone());
            changed = true;
        }

        if let Some(ref extension) = self.extension {
            model.extension = Some(extension.clone());
            changed = true;
        }

        if let Some(file_size) = self.file_size {
            model.file_size = Some(file_size);
            changed = true;
        }

        if let Some(ref mime_type) = self.mime_type {
            model.mime_type = Some(mime_type.clone());
            changed = true;
        }

        if let Some(width) = self.width {
            model.width = Some(width);
            changed = true;
        }

        if let Some(height) = self.height {
            model.height = Some(height);
            changed = true;
        }

        if let Some(ref alt_text) = self.alt_text {
            model.alt_text = Some(alt_text.clone());
            changed = true;
        }

        changed
    }
}
