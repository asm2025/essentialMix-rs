# emix-diesel Examples

This directory contains example implementations demonstrating how to use the `emix-diesel` crate to create repository patterns with Diesel ORM.

## Structure

### Schema Examples (`schema/`)

The schema directory contains example Diesel model definitions:

- **`image.rs`**: Image entity with Queryable, Insertable, and AsChangeset derives
- **`tag.rs`**: Tag entity demonstrating a simple model
- **`image_tag.rs`**: Junction table for many-to-many relationship between images and tags

### Repository Examples (`repositories/`)

The repositories directory contains example repository implementations:

- **`image_repository.rs`**: Full implementation of `IImageRepository` with CRUD operations and tag management
- **`tag_repository.rs`**: Full implementation of `ITagRepository` with CRUD operations and image management

## Key Concepts

### Repository Traits

The crate provides several traits for building repositories:

1. **`IHasConnection<Conn>`**: Base trait for database connection management
2. **`IRepository<T, M, U, I, Conn>`**: Standard CRUD operations (list, count, get, create, update, delete)
3. **`IRepositoryWithRelated<T, M, U, R, I, Conn>`**: Extended trait for entities with relationships

### Models and DTOs

For each entity, you typically define:

- **Model**: The main struct with `Queryable` and `Selectable` derives (e.g., `ImageModel`)
- **NewModel**: Insert struct with `Insertable` derive (e.g., `NewImageModel`)
- **UpdateModel**: Update struct with `AsChangeset` derive (e.g., `UpdateImageModel`)
- **CreateDto**: DTO for creation requests
- **UpdateDto**: DTO for update requests implementing the `Merge` trait

### Filtering and Pagination

The crate provides:

- **`FilterQuery<T>`**: Trait for applying filters to queries
- **`Pagination`**: Struct for page-based pagination
- **`ResultSet<T>`**: Wrapper for paginated results with total count

## Usage Example

```rust
use emix_diesel::prelude::*;
use diesel_async::AsyncSqliteConnection;

// Define your schema
diesel::table! {
    images (id) {
        id -> BigInt,
        title -> Text,
        // ... other fields
    }
}

// Define your models
#[derive(Queryable, Selectable)]
#[diesel(table_name = images)]
pub struct ImageModel {
    pub id: i64,
    pub title: String,
    // ... other fields
}

// Create your repository
pub struct ImageRepository {
    conn: AsyncSqliteConnection,
}

// Implement the repository traits
#[async_trait]
impl IRepository<ImageTable, ImageModel, UpdateImageDto, i64, AsyncSqliteConnection> 
    for ImageRepository 
{
    // Implement required methods...
}

// Use the repository
let repository = ImageRepository::new(conn);
let images = repository.list(None, Some(Pagination::default())).await?;
```

## Differences from SeaORM Version

The Diesel version has some key differences from the SeaORM version:

1. **Type Parameters**: Diesel repositories need more type parameters (Table, Model, Update DTO, ID type, Connection)
2. **Query Types**: Diesel uses `BoxedQuery` instead of SeaORM's `Select`
3. **Relationships**: Diesel requires manual joins, while SeaORM has built-in relationship methods
4. **Active Model**: Diesel uses separate `Insertable` and `AsChangeset` structs instead of SeaORM's unified `ActiveModel`
5. **Connection**: Diesel-async provides async connection types that differ from SeaORM's approach

## Notes

- These examples use SQLite for simplicity, but can be adapted for PostgreSQL, MySQL, etc.
- The connection management in these examples is simplified; production code should use connection pooling (e.g., `deadpool` or `r2d2`)
- Error handling uses the `emixcore::Error` type for consistency with the rest of the emix ecosystem
- The examples demonstrate common patterns but may need adaptation for your specific use case

## Additional Resources

- [Diesel Documentation](https://diesel.rs/)
- [Diesel-async Documentation](https://docs.rs/diesel-async/)
- [emixcore Documentation](https://docs.rs/emixcore/)

