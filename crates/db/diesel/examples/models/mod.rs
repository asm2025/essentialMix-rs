// Example schema module
// This file demonstrates how to organize schema definitions

// Table definitions - Diesel standard approach (all tables in one place)
pub mod tables;

// Model definitions organized by entity
pub mod image;
pub mod image_tag;
pub mod tag;

// Re-export models and DTOs
pub use image::{CreateImageDto, ImageModel, NewImageModel, UpdateImageDto, UpdateImageModel};
pub use image_tag::NewImageTagModel;
pub use tag::{CreateTagDto, NewTagModel, TagModel, UpdateTagDto, UpdateTagModel};
