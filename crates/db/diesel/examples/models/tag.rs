// Example schema implementation using emixdiesel
// This file demonstrates how to use the emixdiesel crate to create Diesel models

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::tables::tags;
use emixdb::models::TMerge;

// Define the ID type based on the database backend
#[cfg(feature = "sqlite")]
pub type TagId = i32;

#[cfg(feature = "postgres")]
pub type TagId = i64;

#[cfg(feature = "mysql")]
pub type TagId = i64;

// Example: Tag model
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = tags)]
#[cfg_attr(feature = "sqlite", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[cfg_attr(feature = "postgres", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "mysql", diesel(check_for_backend(diesel::mysql::Mysql)))]
pub struct TagModel {
    pub id: TagId,
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

impl TMerge<UpdateTagModel> for UpdateTagDto {
    fn merge(&self, model: &mut UpdateTagModel) -> bool {
        let mut changed = false;

        if let Some(ref name) = self.name {
            model.name = Some(name.clone());
            changed = true;
        }

        changed
    }
}

