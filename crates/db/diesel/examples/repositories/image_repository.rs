// Example repository implementation using emix-diesel
// This file demonstrates how to use the emix-diesel crate to create repository implementations

use async_trait::async_trait;
use diesel::{delete, insert_into, update};

use emixdiesel::{Error, Result, prelude::*, repositories::*};

// In a real implementation, you would import your schema entities from your own crate
// For this example, we assume the schema entities are available in scope
// Replace these with your actual schema imports:
// use your_crate::schema::*;
use crate::schema::*;

// Type alias for connection (adjust based on your database)
type DbConnection = diesel_async::AsyncPgConnection;

#[async_trait]
pub trait IImageRepository:
    IRepositoryWithRelated<ImageModel, UpdateImageDto, TagModel, i64, DbConnection>
{
    async fn create_with_tags(&self, model: CreateImageDto) -> Result<ImageModel>;
    async fn list_tags(
        &self,
        id: i64,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<TagModel>>;
    async fn add_tag(&self, id: i64, related_id: i64) -> Result<()>;
    async fn remove_tag(&self, id: i64, related_id: i64) -> Result<u64>;
    async fn add_tags(&self, id: i64, tags: Vec<i64>) -> Result<u64>;
    async fn remove_tags(&self, id: i64, tags: Vec<i64>) -> Result<u64>;
    async fn add_tags_from_str(&self, id: i64, tags: &str) -> Result<u64>;
}

pub struct ImageRepository {
    conn: DbConnection,
}

impl ImageRepository {
    pub fn new(conn: DbConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl IHasConnection<DbConnection> for ImageRepository {
    #[allow(invalid_reference_casting)]
    async fn connection(&self) -> Result<&mut DbConnection> {
        // WARNING: This is a simplified example implementation!
        // In a real application, you should use proper connection pooling with interior mutability
        // (e.g., Arc<Mutex<DbConnection>> or a connection pool like deadpool/r2d2)
        // This unsafe cast is only for demonstration purposes and should NOT be used in production
        Ok(unsafe { &mut *((&self.conn) as *const _ as *mut _) })
    }

    async fn begin_transaction(&self) -> Result<&mut DbConnection> {
        let conn = self.connection().await?;
        conn.begin_test_transaction()
            .await
            .map_err(Error::from_std_error)?;
        Ok(conn)
    }
}

#[async_trait]
impl IRepository<ImageModel, UpdateImageDto, i64, DbConnection> for ImageRepository {
    async fn list(&self, pagination: Option<Pagination>) -> Result<ResultSet<ImageModel>> {
        use crate::schema::images;

        let conn = self.connection().await?;

        // Get total count
        let total = images::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = images::table.into_boxed();
        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let data = query
            .load::<ImageModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(ResultSet {
            data,
            total,
            pagination,
        })
    }

    async fn count(&self) -> Result<u64> {
        use crate::schema::images;

        let conn = self.connection().await?;
        images::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map(|c| c as u64)
            .map_err(Error::from_std_error)
    }

    async fn get(&self, id: i64) -> Result<Option<ImageModel>> {
        use crate::schema::images;

        let conn = self.connection().await?;
        images::table
            .find(id)
            .first::<ImageModel>(conn)
            .await
            .optional()
            .map_err(Error::from_std_error)
    }

    async fn create(&self, model: ImageModel) -> Result<ImageModel> {
        use crate::schema::images;

        let conn = self.connection().await?;
        let new_model = NewImageModel {
            title: model.title,
            description: model.description,
            extension: model.extension,
            file_size: model.file_size,
            mime_type: model.mime_type,
            width: model.width,
            height: model.height,
            alt_text: model.alt_text,
            created_at: model.created_at,
            updated_at: model.updated_at,
        };

        insert_into(images::table)
            .values(&new_model)
            .get_result::<ImageModel>(conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn update(&self, id: i64, model: UpdateImageDto) -> Result<ImageModel> {
        use crate::schema::images;

        let conn = self.connection().await?;

        // First, verify the image exists
        let _existing = images::table
            .find(id)
            .first::<ImageModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        let mut update_model = UpdateImageModel {
            title: None,
            description: None,
            extension: None,
            file_size: None,
            mime_type: None,
            width: None,
            height: None,
            alt_text: None,
            updated_at: chrono::Utc::now().naive_utc(),
        };

        // Apply the updates from the DTO
        if let Some(ref title) = model.title {
            update_model.title = Some(title.clone());
        }
        if let Some(ref description) = model.description {
            update_model.description = Some(description.clone());
        }
        if let Some(ref extension) = model.extension {
            update_model.extension = Some(extension.clone());
        }
        if let Some(file_size) = model.file_size {
            update_model.file_size = Some(file_size);
        }
        if let Some(ref mime_type) = model.mime_type {
            update_model.mime_type = Some(mime_type.clone());
        }
        if let Some(width) = model.width {
            update_model.width = Some(width);
        }
        if let Some(height) = model.height {
            update_model.height = Some(height);
        }
        if let Some(ref alt_text) = model.alt_text {
            update_model.alt_text = Some(alt_text.clone());
        }

        update(images::table.find(id))
            .set(&update_model)
            .get_result::<ImageModel>(conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn delete(&self, id: i64) -> Result<()> {
        use crate::schema::images;

        let conn = self.connection().await?;
        delete(images::table.find(id))
            .execute(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }
}

#[async_trait]
impl IRepositoryWithRelated<ImageModel, UpdateImageDto, TagModel, i64, DbConnection>
    for ImageRepository
{
    async fn list_with_related(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<ImageModel, TagModel>>> {
        use crate::schema::image_tags;
        use crate::schema::images;
        use crate::schema::tags;

        let conn = self.connection().await?;

        // Get total count
        let total = images::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = images::table.into_boxed();
        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let images_list = query
            .load::<ImageModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        // Load related tags for each image
        let mut result_data = Vec::new();
        for image in images_list {
            let related_tags = image_tags::table
                .inner_join(tags::table)
                .filter(image_tags::image_id.eq(image.id))
                .select(TagModel::as_select())
                .load::<TagModel>(conn)
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

    async fn get_with_related(
        &self,
        id: i64,
    ) -> Result<Option<ModelWithRelated<ImageModel, TagModel>>> {
        use crate::schema::image_tags;
        use crate::schema::images;
        use crate::schema::tags;

        let conn = self.connection().await?;
        let image = images::table
            .find(id)
            .first::<ImageModel>(conn)
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
            .load::<TagModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(Some(ModelWithRelated {
            item: image,
            related: related_tags,
        }))
    }

    async fn delete_related(&self, id: i64) -> Result<()> {
        use crate::schema::image_tags;

        let conn = self.connection().await?;
        delete(image_tags::table.filter(image_tags::image_id.eq(id)))
            .execute(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }
}

#[async_trait]
impl IImageRepository for ImageRepository {
    async fn create_with_tags(&self, model: CreateImageDto) -> Result<ImageModel> {
        let tags = model.tags.clone();
        let new_model: NewImageModel = model.into();

        let conn = self.connection().await?;
        let result = insert_into(images::table)
            .values(&new_model)
            .get_result::<ImageModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        let Some(tags) = tags else {
            return Ok(result);
        };

        self.add_tags_from_str(result.id, &tags).await?;
        Ok(result)
    }

    async fn list_tags(
        &self,
        id: i64,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<TagModel>> {
        use crate::schema::image_tags;
        use crate::schema::tags;

        let conn = self.connection().await?;

        let total = tags::table
            .inner_join(image_tags::table)
            .filter(image_tags::image_id.eq(id))
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        let mut query = tags::table
            .inner_join(image_tags::table)
            .filter(image_tags::image_id.eq(id))
            .select(TagModel::as_select())
            .into_boxed();

        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let data = query
            .load::<TagModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(ResultSet {
            data,
            total,
            pagination,
        })
    }

    async fn add_tag(&self, id: i64, related_id: i64) -> Result<()> {
        use crate::schema::image_tags;

        let conn = self.connection().await?;
        let new_image_tag = NewImageTagModel {
            image_id: id,
            tag_id: related_id,
        };

        insert_into(image_tags::table)
            .values(&new_image_tag)
            .execute(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }

    async fn remove_tag(&self, id: i64, related_id: i64) -> Result<u64> {
        use crate::schema::image_tags;

        let conn = self.connection().await?;
        let rows_affected = delete(
            image_tags::table
                .filter(image_tags::image_id.eq(id))
                .filter(image_tags::tag_id.eq(related_id)),
        )
        .execute(conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }

    async fn add_tags(&self, id: i64, tags: Vec<i64>) -> Result<u64> {
        if tags.is_empty() {
            return Ok(0);
        }

        use crate::schema::image_tags;

        let conn = self.connection().await?;

        let mut rows_inserted: usize = 0;
        for tag_id in tags {
            let new_image_tag = NewImageTagModel {
                image_id: id,
                tag_id,
            };

            let result = insert_into(image_tags::table)
                .values(&new_image_tag)
                .execute(conn)
                .await
                .map_err(Error::from_std_error)?;

            rows_inserted += result;
        }

        Ok(rows_inserted as u64)
    }

    async fn remove_tags(&self, id: i64, tags: Vec<i64>) -> Result<u64> {
        if tags.is_empty() {
            return Ok(0);
        }

        use crate::schema::image_tags;

        let conn = self.connection().await?;
        let rows_affected = delete(
            image_tags::table
                .filter(image_tags::image_id.eq(id))
                .filter(image_tags::tag_id.eq_any(tags)),
        )
        .execute(conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }

    async fn add_tags_from_str(&self, id: i64, tags_str: &str) -> Result<u64> {
        if tags_str.is_empty() {
            return Ok(0);
        }

        use crate::schema::tags;

        let tag_names: Vec<&str> = tags_str
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        if tag_names.is_empty() {
            return Ok(0);
        }

        let conn = self.connection().await?;

        // Insert tags (ignore conflicts for existing tags)
        for &name in &tag_names {
            let new_tag = NewTagModel {
                name: name.to_string(),
            };

            let _ = insert_into(tags::table)
                .values(&new_tag)
                .execute(conn)
                .await
                .ok(); // Ignore errors from duplicate tags
        }

        // Get tag IDs
        let tag_ids = tags::table
            .filter(tags::name.eq_any(&tag_names))
            .select(tags::id)
            .load::<i64>(conn)
            .await
            .map_err(Error::from_std_error)?;

        if tag_ids.is_empty() {
            return Ok(0);
        }

        // Add image-tag associations
        self.add_tags(id, tag_ids).await
    }
}
