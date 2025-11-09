// Example schema implementation using emixdiesel
// This file demonstrates how to use the emixdiesel crate to create Diesel models

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::tables::image_tags;
use super::image::ImageId;
use super::tag::TagId;

// Example: ImageTag junction model
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = image_tags)]
#[diesel(primary_key(image_id, tag_id))]
#[cfg_attr(feature = "sqlite", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[cfg_attr(feature = "postgres", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "mysql", diesel(check_for_backend(diesel::mysql::Mysql)))]
pub struct ImageTagModel {
    pub image_id: ImageId,
    pub tag_id: TagId,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = image_tags)]
pub struct NewImageTagModel {
    pub image_id: ImageId,
    pub tag_id: TagId,
}

