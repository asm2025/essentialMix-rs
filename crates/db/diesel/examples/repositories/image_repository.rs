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
    tables::*, CreateImageDto, ImageModel, NewImageModel, NewImageTagModel, NewTagModel,
    TagModel, UpdateImageDto, UpdateImageModel,
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

/// Image repository with proper connection pooling
pub struct ImageRepository {
    pool: DbPool,
}

impl ImageRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Helper to get a connection from the pool
    async fn get_conn(
        &self,
    ) -> Result<diesel_async::pooled_connection::deadpool::Object<DbConnection>> {
        self.pool.get().await.map_err(|e| Error::from_std_error(e))
    }
}

#[async_trait]
pub trait IImageRepository {
    /// List all images with optional pagination
    async fn list(&self, pagination: Option<Pagination>) -> Result<ResultSet<ImageModel>>;

    /// Count all images
    async fn count(&self) -> Result<u64>;

    /// Get a single image by ID
    async fn get(&self, id: ImageId) -> Result<Option<ImageModel>>;

    /// Create a new image
    async fn create(&self, dto: CreateImageDto) -> Result<ImageModel>;

    /// Update an image by ID
    async fn update(&self, id: ImageId, dto: UpdateImageDto) -> Result<ImageModel>;

    /// Delete an image by ID
    async fn delete(&self, id: ImageId) -> Result<()>;

    /// Get image with its tags
    async fn get_with_tags(
        &self,
        id: ImageId,
    ) -> Result<Option<ModelWithRelated<ImageModel, TagModel>>>;

    /// List images with their tags
    async fn list_with_tags(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<ImageModel, TagModel>>>;

    /// Create image with tags from comma-separated string
    async fn create_with_tags(&self, dto: CreateImageDto) -> Result<ImageModel>;

    /// Add a tag to an image
    async fn add_tag(&self, image_id: ImageId, tag_id: TagId) -> Result<()>;

    /// Remove a tag from an image
    async fn remove_tag(&self, image_id: ImageId, tag_id: TagId) -> Result<u64>;

    /// Add multiple tags to an image
    async fn add_tags(&self, image_id: ImageId, tag_ids: Vec<TagId>) -> Result<u64>;

    /// Remove multiple tags from an image
    async fn remove_tags(&self, image_id: ImageId, tag_ids: Vec<TagId>) -> Result<u64>;

    /// Add tags from comma-separated string
    async fn add_tags_from_str(&self, image_id: ImageId, tags_str: &str) -> Result<u64>;

    /// Delete all tags for an image
    async fn delete_all_tags(&self, image_id: ImageId) -> Result<()>;
}

#[async_trait]
impl IImageRepository for ImageRepository {
    async fn list(&self, pagination: Option<Pagination>) -> Result<ResultSet<ImageModel>> {
        let mut conn = self.get_conn().await?;

        // Get total count
        let total = images::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = images::table.into_boxed();
        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let data = query
            .select(ImageModel::as_select())
            .load::<ImageModel>(&mut conn)
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
        images::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map(|c| c as u64)
            .map_err(Error::from_std_error)
    }

    async fn get(&self, id: ImageId) -> Result<Option<ImageModel>> {
        let mut conn = self.get_conn().await?;
        images::table
            .find(id)
            .select(ImageModel::as_select())
            .first::<ImageModel>(&mut conn)
            .await
            .optional()
            .map_err(Error::from_std_error)
    }

    async fn create(&self, dto: CreateImageDto) -> Result<ImageModel> {
        let mut conn = self.get_conn().await?;
        let new_model: NewImageModel = dto.into();

        diesel::insert_into(images::table)
            .values(&new_model)
            .returning(ImageModel::as_returning())
            .get_result::<ImageModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn update(&self, id: ImageId, dto: UpdateImageDto) -> Result<ImageModel> {
        let mut conn = self.get_conn().await?;

        // Verify the image exists
        let _existing = images::table
            .find(id)
            .select(ImageModel::as_select())
            .first::<ImageModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        let update_model = UpdateImageModel {
            title: dto.title,
            description: dto.description,
            extension: dto.extension,
            file_size: dto.file_size,
            mime_type: dto.mime_type,
            width: dto.width,
            height: dto.height,
            alt_text: dto.alt_text,
            updated_at: chrono::Utc::now().naive_utc(),
        };

        diesel::update(images::table.find(id))
            .set(&update_model)
            .returning(ImageModel::as_returning())
            .get_result::<ImageModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn delete(&self, id: ImageId) -> Result<()> {
        let mut conn = self.get_conn().await?;

        // Delete related tags first
        diesel::delete(image_tags::table.filter(image_tags::image_id.eq(id)))
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        // Delete the image
        diesel::delete(images::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }

    async fn get_with_tags(
        &self,
        id: ImageId,
    ) -> Result<Option<ModelWithRelated<ImageModel, TagModel>>> {
        let mut conn = self.get_conn().await?;

        let image = images::table
            .find(id)
            .select(ImageModel::as_select())
            .first::<ImageModel>(&mut conn)
            .await
            .optional()
            .map_err(Error::from_std_error)?;

        let Some(image) = image else {
            return Ok(None);
        };

        let related_tags = image_tags::table
            .inner_join(tags::table)
            .filter(image_tags::image_id.eq(id))
            .select(TagModel::as_select())
            .load::<TagModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(Some(ModelWithRelated {
            item: image,
            related: related_tags,
        }))
    }

    async fn list_with_tags(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<ImageModel, TagModel>>> {
        let mut conn = self.get_conn().await?;

        // Get total count
        let total = images::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = images::table.into_boxed();
        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let images_list = query
            .select(ImageModel::as_select())
            .load::<ImageModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        // Load related tags for each image
        let mut result_data = Vec::new();
        for image in images_list {
            let related_tags = image_tags::table
                .inner_join(tags::table)
                .filter(image_tags::image_id.eq(image.id))
                .select(TagModel::as_select())
                .load::<TagModel>(&mut conn)
                .await
                .map_err(Error::from_std_error)?;

            result_data.push(ModelWithRelated {
                item: image,
                related: related_tags,
            });
        }

        Ok(ResultSet {
            data: result_data,
            total,
            pagination,
        })
    }

    async fn create_with_tags(&self, dto: CreateImageDto) -> Result<ImageModel> {
        let tags_str = dto.tags.clone();
        let new_model: NewImageModel = dto.into();

        let mut conn = self.get_conn().await?;

        let result = diesel::insert_into(images::table)
            .values(&new_model)
            .returning(ImageModel::as_returning())
            .get_result::<ImageModel>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        if let Some(tags) = tags_str {
            self.add_tags_from_str(result.id, &tags).await?;
        }

        Ok(result)
    }

    async fn add_tag(&self, image_id: ImageId, tag_id: TagId) -> Result<()> {
        let mut conn = self.get_conn().await?;

        let new_image_tag = NewImageTagModel { image_id, tag_id };

        diesel::insert_into(image_tags::table)
            .values(&new_image_tag)
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }

    async fn remove_tag(&self, image_id: ImageId, tag_id: TagId) -> Result<u64> {
        let mut conn = self.get_conn().await?;

        let rows_affected = diesel::delete(
            image_tags::table
                .filter(image_tags::image_id.eq(image_id))
                .filter(image_tags::tag_id.eq(tag_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }

    async fn add_tags(&self, image_id: ImageId, tag_ids: Vec<TagId>) -> Result<u64> {
        if tag_ids.is_empty() {
            return Ok(0);
        }

        let mut conn = self.get_conn().await?;
        
        // SQLite doesn't support batch inserts, so we insert one at a time
        let mut total_inserted = 0;
        for tag_id in tag_ids {
            let new_image_tag = NewImageTagModel { image_id, tag_id };
            let rows = diesel::insert_into(image_tags::table)
                .values(&new_image_tag)
                .execute(&mut conn)
                .await
                .map_err(Error::from_std_error)?;
            total_inserted += rows;
        }

        Ok(total_inserted as u64)
    }

    async fn remove_tags(&self, image_id: ImageId, tag_ids: Vec<TagId>) -> Result<u64> {
        if tag_ids.is_empty() {
            return Ok(0);
        }

        let mut conn = self.get_conn().await?;

        let rows_affected = diesel::delete(
            image_tags::table
                .filter(image_tags::image_id.eq(image_id))
                .filter(image_tags::tag_id.eq_any(tag_ids)),
        )
        .execute(&mut conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }

    async fn add_tags_from_str(&self, image_id: ImageId, tags_str: &str) -> Result<u64> {
        if tags_str.is_empty() {
            return Ok(0);
        }

        let tag_names: Vec<&str> = tags_str
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        if tag_names.is_empty() {
            return Ok(0);
        }

        let mut conn = self.get_conn().await?;

        // Insert tags (ignore conflicts for existing tags)
        for &name in &tag_names {
            let new_tag = NewTagModel {
                name: name.to_string(),
            };

            let _ = diesel::insert_into(tags::table)
                .values(&new_tag)
                .execute(&mut conn)
                .await
                .ok(); // Ignore errors from duplicate tags
        }

        // Get tag IDs
        let tag_ids = tags::table
            .filter(tags::name.eq_any(&tag_names))
            .select(tags::id)
            .load::<TagId>(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        if tag_ids.is_empty() {
            return Ok(0);
        }

        // Add image-tag associations
        self.add_tags(image_id, tag_ids).await
    }

    async fn delete_all_tags(&self, image_id: ImageId) -> Result<()> {
        let mut conn = self.get_conn().await?;

        diesel::delete(image_tags::table.filter(image_tags::image_id.eq(image_id)))
            .execute(&mut conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }
}
