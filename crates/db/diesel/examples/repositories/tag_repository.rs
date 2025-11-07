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
pub trait ITagRepository:
    IRepositoryWithRelated<TagModel, UpdateTagDto, ImageModel, i64, DbConnection>
{
    async fn list_images(
        &self,
        id: i64,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<ImageModel, TagModel>>>;
    async fn add_image(&self, id: i64, related_id: i64) -> Result<ImageTagModel>;
    async fn remove_image(&self, id: i64, related_id: i64) -> Result<u64>;
    async fn add_images(&self, id: i64, images: Vec<i64>) -> Result<u64>;
    async fn remove_images(&self, id: i64, images: Vec<i64>) -> Result<u64>;
}

pub struct TagRepository {
    conn: DbConnection,
}

impl TagRepository {
    pub fn new(conn: DbConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl IHasConnection<DbConnection> for TagRepository {
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
impl IRepository<TagModel, UpdateTagDto, i64, DbConnection> for TagRepository {
    async fn list(&self, pagination: Option<Pagination>) -> Result<ResultSet<TagModel>> {
        use crate::schema::tags;

        let conn = self.connection().await?;

        // Get total count
        let total = tags::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = tags::table.into_boxed();
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

    async fn count(&self) -> Result<u64> {
        use crate::schema::tags;

        let conn = self.connection().await?;
        tags::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map(|c| c as u64)
            .map_err(Error::from_std_error)
    }

    async fn get(&self, id: i64) -> Result<Option<TagModel>> {
        use crate::schema::tags;

        let conn = self.connection().await?;
        tags::table
            .find(id)
            .first::<TagModel>(conn)
            .await
            .optional()
            .map_err(Error::from_std_error)
    }

    async fn create(&self, model: TagModel) -> Result<TagModel> {
        use crate::schema::tags;

        let conn = self.connection().await?;
        let new_model = NewTagModel { name: model.name };

        insert_into(tags::table)
            .values(&new_model)
            .get_result::<TagModel>(conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn update(&self, id: i64, model: UpdateTagDto) -> Result<TagModel> {
        use crate::schema::tags;

        let conn = self.connection().await?;

        // First, verify the tag exists
        let _existing = tags::table
            .find(id)
            .first::<TagModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        let mut update_model = UpdateTagModel { name: None };

        // Apply updates from the DTO
        if let Some(ref name) = model.name {
            update_model.name = Some(name.clone());
        }

        update(tags::table.find(id))
            .set(&update_model)
            .get_result::<TagModel>(conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn delete(&self, id: i64) -> Result<()> {
        use crate::schema::tags;

        let conn = self.connection().await?;
        delete(tags::table.find(id))
            .execute(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }
}

#[async_trait]
impl IRepositoryWithRelated<TagModel, UpdateTagDto, ImageModel, i64, DbConnection>
    for TagRepository
{
    async fn list_with_related(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<TagModel, ImageModel>>> {
        use crate::schema::image_tags;
        use crate::schema::images;
        use crate::schema::tags;

        let conn = self.connection().await?;

        // Get total count
        let total = tags::table
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = tags::table.into_boxed();
        if let Some(p) = pagination {
            let offset = ((p.page - 1) * p.page_size) as i64;
            query = query.limit(p.page_size as i64).offset(offset);
        }

        let tags_list = query
            .load::<TagModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        // Load related images for each tag
        let mut result_data = Vec::new();
        for tag in tags_list {
            let related_images = image_tags::table
                .inner_join(images::table)
                .filter(image_tags::tag_id.eq(tag.id))
                .select(ImageModel::as_select())
                .load::<ImageModel>(conn)
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

    async fn get_with_related(
        &self,
        id: i64,
    ) -> Result<Option<ModelWithRelated<TagModel, ImageModel>>> {
        use crate::schema::image_tags;
        use crate::schema::images;
        use crate::schema::tags;

        let conn = self.connection().await?;
        let tag = tags::table
            .find(id)
            .first::<TagModel>(conn)
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
            .load::<ImageModel>(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(Some(ModelWithRelated {
            item: tag,
            related: related_images,
        }))
    }

    async fn delete_related(&self, id: i64) -> Result<()> {
        use crate::schema::image_tags;

        let conn = self.connection().await?;
        delete(image_tags::table.filter(image_tags::tag_id.eq(id)))
            .execute(conn)
            .await
            .map_err(Error::from_std_error)?;

        Ok(())
    }
}

#[async_trait]
impl ITagRepository for TagRepository {
    async fn list_images(
        &self,
        id: i64,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<ImageModel, TagModel>>> {
        use crate::schema::image_tags;
        use crate::schema::images;
        use crate::schema::tags;

        let conn = self.connection().await?;

        // Get image IDs associated with this tag
        let image_ids_query = image_tags::table
            .filter(image_tags::tag_id.eq(id))
            .select(image_tags::image_id);

        // Get total count
        let total = images::table
            .filter(
                images::id.eq_any(
                    image_tags::table
                        .filter(image_tags::tag_id.eq(id))
                        .select(image_tags::image_id),
                ),
            )
            .count()
            .get_result::<i64>(conn)
            .await
            .map_err(Error::from_std_error)? as u64;

        // Apply pagination
        let mut query = images::table
            .filter(images::id.eq_any(image_ids_query))
            .into_boxed();
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

    async fn add_image(&self, id: i64, related_id: i64) -> Result<ImageTagModel> {
        use crate::schema::image_tags;

        let conn = self.connection().await?;
        let new_image_tag = NewImageTagModel {
            tag_id: id,
            image_id: related_id,
        };

        insert_into(image_tags::table)
            .values(&new_image_tag)
            .get_result::<ImageTagModel>(conn)
            .await
            .map_err(Error::from_std_error)
    }

    async fn remove_image(&self, id: i64, related_id: i64) -> Result<u64> {
        use crate::schema::image_tags;

        let conn = self.connection().await?;
        let rows_affected = delete(
            image_tags::table
                .filter(image_tags::tag_id.eq(id))
                .filter(image_tags::image_id.eq(related_id)),
        )
        .execute(conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }

    async fn add_images(&self, id: i64, images: Vec<i64>) -> Result<u64> {
        if images.is_empty() {
            return Ok(0);
        }

        use crate::schema::image_tags;

        let conn = self.connection().await?;

        let mut rows_inserted: usize = 0;
        for image_id in images {
            let new_image_tag = NewImageTagModel {
                tag_id: id,
                image_id,
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

    async fn remove_images(&self, id: i64, images: Vec<i64>) -> Result<u64> {
        if images.is_empty() {
            return Ok(0);
        }

        use crate::schema::image_tags;

        let conn = self.connection().await?;
        let rows_affected = delete(
            image_tags::table
                .filter(image_tags::tag_id.eq(id))
                .filter(image_tags::image_id.eq_any(images)),
        )
        .execute(conn)
        .await
        .map_err(Error::from_std_error)?;

        Ok(rows_affected as u64)
    }
}
