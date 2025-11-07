// Example schema implementation using emix-diesel
// This file demonstrates how to use the emix-diesel crate to create Diesel models

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::tables::image_tags;

// Example: ImageTag junction model
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable,
)]
#[diesel(table_name = image_tags)]
#[diesel(primary_key(image_id, tag_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ImageTagModel {
    pub image_id: i64,
    pub tag_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = image_tags)]
pub struct NewImageTagModel {
    pub image_id: i64,
    pub tag_id: i64,
}

// Re-export table references for convenience
pub use image_tags::dsl as image_tag_dsl;
pub use image_tags::table as image_tags_table;
