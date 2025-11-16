// Example repository implementation using emixdiesel
// This file demonstrates how to use the emixdiesel crate to create repository implementations
// with proper connection pooling for production use

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::RunQueryDsl;

use emixdb::dto::*;
use emixdiesel::{Error, Result};

// Import models and their type aliases
use crate::models::{
    tables::*, CreateTagDto, ImageModel, NewImageTagModel, NewTagModel, TagModel, UpdateTagDto,
    UpdateTagModel,
};

// Import ID type aliases
use crate::models::image::ImageId;
use crate::models::tag::TagId;

// Type aliases for each database backend
// Note: SQLite doesn't support true async, so we use SyncConnectionWrapper
#[cfg(feature = "sqlite")]
pub type DbConnection =
    diesel_async::sync_connection_wrapper::SyncConnectionWrapper<diesel::SqliteConnection>;

#[cfg(feature = "postgres")]
pub type DbConnection = diesel_async::AsyncPgConnection;

#[cfg(feature = "mysql")]
pub type DbConnection = diesel_async::AsyncMysqlConnection;

pub type DbPool = Pool<DbConnection>;

/// Tag repository with proper connection pooling
pub struct TagRepository {
    pool: DbPool,
}

impl TagRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Helper to get a connection from the pool
    async fn get_conn(&self) -> Result<diesel_async::pooled_connection::deadpool::Object<DbConnection>> {
        self.pool
            .get()
            .await
            .map_err(|e| Error::from_std_error(e))
    }
}

#[async_trait]
pub trait TagRepositoryExt {
    /// List all tags with optional pagination
    async fn list(&self, pagination: Option<Pagination>) -> Result<ResultSet<TagModel>>;

    /// Count all tags
    async fn count(&self) -> Result<u64>;

    /// Get a single tag by ID
    async fn get(&self, id: TagId) -> Result<Option<TagModel>>;

    /// Get a tag by name
    async fn get_by_name(&self, name: &str) -> Result<Option<TagModel>>;

    /// Create a new tag
    async fn create(&self, dto: CreateTagDto) -> Result<TagModel>;

    /// Update a tag by ID
    async fn update(&self, id: TagId, dto: UpdateTagDto) -> Result<TagModel>;

    /// Delete a tag by ID
    async fn delete(&self, id: TagId) -> Result<()>;

    /// Get tag with its images
    async fn get_with_images(&self, id: TagId) -> Result<Option<ModelWithRelated<TagModel, ImageModel>>>;

    /// List tags with their images
    async fn list_with_images(&self, pagination: Option<Pagination>) -> Result<ResultSet<ModelWithRelated<TagModel, ImageModel>>>;

    /// List images for a specific tag
    async fn list_images(&self, tag_id: TagId, pagination: Option<Pagination>) -> Result<ResultSet<ImageModel>>;

    /// Add an image to a tag
    async fn add_image(&self, tag_id: TagId, image_id: ImageId) -> Result<()>;

    /// Remove an image from a tag
    async fn remove_image(&self, tag_id: TagId, image_id: ImageId) -> Result<u64>;

    /// Add multiple images to a tag
    async fn add_images(&self, tag_id: TagId, image_ids: Vec<ImageId>) -> Result<u64>;

    /// Remove multiple images from a tag
    async fn remove_images(&self, tag_id: TagId, image_ids: Vec<ImageId>) -> Result<u64>;

    /// Delete all images for a tag
    async fn delete_all_images(&self, tag_id: TagId) -> Result<()>;
}

#[async_trait]
impl TagRepositoryExt for TagRepository {
    async fn list(&self, pagination: Option<Pagination>) -> Result<ResultSet<TagModel>> {
        let mut conn = self.get_conn().await?;

        // Get total count
        let total = tags::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = tags::table.into_boxed();
        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let data = query
            .select(TagModel::as_select())
            .load::<TagModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(ResultSet {
            data,
            total,
            pagination,
        })
    }

    async fn count(&self) -> Result<u64> {
        let mut conn = self.get_conn().await?;
        tags::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map(|c| c as u64)
            .map_err(Error::from_std_error)
    }

    async fn get(&self, id: TagId) -> Result<Option<TagModel>> {
        let mut conn = self.get_conn().await?;
        tags::table
            .find(id)
            .select(TagModel::as_select())
            .first::<TagModel>(&mut conn)
            .await
            .optional()
            .map_err(Error::from_std_error)
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<TagModel>> {
        let mut conn = self.get_conn().await?;
        tags::table
            .filter(tags::name.eq(name))
            .select(TagModel::as_select())
            .first::<TagModel>(&mut conn)
            .await
            .optional()
            .map_err(Error::from_std_error)
    }

    async fn create(&self, dto: CreateTagDto) -> Result<TagModel> {
        let mut conn = self.get_conn().await?;
        let new_model: NewTagModel = dto.into();

        diesel::insert_into(tags::table)
            .values(&new_model)
            .returning(TagModel::as_returning())
            .get_result::<TagModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn update(&self, id: TagId, dto: UpdateTagDto) -> Result<TagModel> {
        let mut conn = self.get_conn().await?;

        // Verify the tag exists
        let _existing = tags::table
            .find(id)
            .select(TagModel::as_select())
            .first::<TagModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        let update_model = UpdateTagModel {
            name: dto.name,
        };

        diesel::update(tags::table.find(id))
            .set(&update_model)
            .returning(TagModel::as_returning())
            .get_result::<TagModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn delete(&self, id: TagId) -> Result<()> {
        let mut conn = self.get_conn().await?;
        
        // Delete related image associations first
        diesel::delete(image_tags::table.filter(image_tags::tag_id.eq(id)))
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        // Delete the tag
        diesel::delete(tags::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }

    async fn get_with_images(&self, id: TagId) -> Result<Option<ModelWithRelated<TagModel, ImageModel>>> {
        let mut conn = self.get_conn().await?;
        
        let tag = tags::table
            .find(id)
            .select(TagModel::as_select())
            .first::<TagModel>(&mut conn)
            .await
            .optional()
            .map_err(Error::from_std_error)?;

        let Some(tag) = tag else {
            return Ok(None);
        };

        let related_images = image_tags::table
            .inner_join(images::table)
            .filter(image_tags::tag_id.eq(id))
            .select(ImageModel::as_select())
            .load::<ImageModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(Some(ModelWithRelated {
            item: tag,
            related: related_images,
        }))
    }

    async fn list_with_images(&self, pagination: Option<Pagination>) -> Result<ResultSet<ModelWithRelated<TagModel, ImageModel>>> {
        let mut conn = self.get_conn().await?;

        // Get total count
        let total = tags::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = tags::table.into_boxed();
        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let tags_list = query
            .select(TagModel::as_select())
            .load::<TagModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        // Load related images for each tag
        let mut result_data = Vec::new();
        for tag in tags_list {
            let related_images = image_tags::table
                .inner_join(images::table)
                .filter(image_tags::tag_id.eq(tag.id))
                .select(ImageModel::as_select())
                .load::<ImageModel>(&mut conn)
                .await
                .map_err(Error::from_std_error)?;

            result_data.push(ModelWithRelated {
                item: tag,
                related: related_images,
            });
        }

        Ok(ResultSet {
            data: result_data,
            total,
            pagination,
        })
    }

    async fn list_images(&self, tag_id: TagId, pagination: Option<Pagination>) -> Result<ResultSet<ImageModel>> {
        let mut conn = self.get_conn().await?;

        // Get total count
        let total = images::table
            .inner_join(image_tags::table)
            .filter(image_tags::tag_id.eq(tag_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = images::table
            .inner_join(image_tags::table)
            .filter(image_tags::tag_id.eq(tag_id))
            .select(ImageModel::as_select())
            .into_boxed();

        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let data = query
            .load::<ImageModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(ResultSet {
            data,
            total,
            pagination,
        })
    }

    async fn add_image(&self, tag_id: TagId, image_id: ImageId) -> Result<()> {
        let mut conn = self.get_conn().await?;
        
        let new_image_tag = NewImageTagModel {
            tag_id,
            image_id,
        };

        diesel::insert_into(image_tags::table)
            .values(&new_image_tag)
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }

    async fn remove_image(&self, tag_id: TagId, image_id: ImageId) -> Result<u64> {
        let mut conn = self.get_conn().await?;
        
        let rows_affected = diesel::delete(
            image_tags::table
                .filter(image_tags::tag_id.eq(tag_id))
                .filter(image_tags::image_id.eq(image_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }

    async fn add_images(&self, tag_id: TagId, image_ids: Vec<ImageId>) -> Result<u64> {
        if image_ids.is_empty() {
            return Ok(0);
        }

        let mut conn = self.get_conn().await?;
        
        // SQLite doesn't support batch inserts, so we insert one at a time
        let mut total_inserted = 0;
        for image_id in image_ids {
            let new_image_tag = NewImageTagModel { tag_id, image_id };
            let rows = diesel::insert_into(image_tags::table)
                .values(&new_image_tag)
                .execute(&mut conn)
                .await
                .map_err(Error::from_std_error)?;
            total_inserted += rows;
        }

        Ok(total_inserted as u64)
    }

    async fn remove_images(&self, tag_id: TagId, image_ids: Vec<ImageId>) -> Result<u64> {
        if image_ids.is_empty() {
            return Ok(0);
        }

        let mut conn = self.get_conn().await?;
        
        let rows_affected = diesel::delete(
            image_tags::table
                .filter(image_tags::tag_id.eq(tag_id))
                .filter(image_tags::image_id.eq_any(image_ids)),
        )
        .execute(&mut conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }

    async fn delete_all_images(&self, tag_id: TagId) -> Result<()> {
        let mut conn = self.get_conn().await?;
        
        diesel::delete(image_tags::table.filter(image_tags::tag_id.eq(tag_id)))
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }
}
