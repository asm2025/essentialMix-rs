// Example schema implementation using emix-diesel
// This file demonstrates how to use the emix-diesel crate to create Diesel models

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::tables::tags;
use emixdiesel::prelude::*;

// Example: Tag model
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable,
)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TagModel {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTagModel {
    pub name: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = tags)]
pub struct UpdateTagModel {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTagDto {
    pub name: String,
}

impl From<CreateTagDto> for NewTagModel {
    fn from(req: CreateTagDto) -> Self {
        Self { name: req.name }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagDto {
    pub name: Option<String>,
}

impl Merge<UpdateTagModel> for UpdateTagDto {
    fn merge(&self, model: &mut UpdateTagModel) -> bool {
        let mut changed = false;

        if let Some(ref name) = self.name {
            model.name = Some(name.clone());
            changed = true;
        }

        changed
    }
}

// Re-export table references for convenience
pub use tags::dsl as tag_dsl;
pub use tags::table as tags_table;
