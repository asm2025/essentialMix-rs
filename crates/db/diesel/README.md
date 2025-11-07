# emix-diesel

A Rust library providing repository pattern abstractions for Diesel ORM, part of the essentialMix-rs ecosystem.

## Overview

`emix-diesel` provides a structured approach to building repositories with Diesel ORM. It offers:

- **Repository Traits**: Standard interfaces for CRUD operations
- **Relationship Support**: Built-in support for entities with relationships
- **Filtering and Pagination**: Flexible query filtering and pagination
- **Type Safety**: Leverages Rust's type system for compile-time safety
- **Async Support**: Built on `diesel-async` for async operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
emix-diesel = "0.5"
diesel = { version = "2", features = ["sqlite", "chrono"] }
diesel-async = { version = "0", features = ["sqlite", "async-connection-wrapper"] }
```

## Features

### Core Traits

- **`IHasConnection<Conn>`**: Database connection management
- **`IRepository<T, M, U, I, Conn>`**: Basic CRUD operations
- **`IRepositoryWithRelated<T, M, U, R, I, Conn>`**: Extended operations for entities with relationships

### Supporting Types

- **`ModelWithRelated<M, R>`**: Wrapper for entities with their related data
- **`Pagination`**: Page-based pagination configuration
- **`ResultSet<T>`**: Paginated results with total count
- **`FilterQuery<T>`**: Trait for applying filters to queries
- **`Merge<T>`**: Trait for partial updates

## Quick Start

### 1. Define Your Schema

```rust
diesel::table! {
    users (id) {
        id -> BigInt,
        name -> Text,
        email -> Text,
    }
}
```

### 2. Define Your Models

```rust
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
```

### 3. Define Your DTOs

```rust
use emix_diesel::schema::Merge;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Merge<UpdateUser> for UpdateUserDto {
    fn merge(&self, model: &mut UpdateUser) -> bool {
        let mut changed = false;
        if let Some(ref name) = self.name {
            model.name = Some(name.clone());
            changed = true;
        }
        if let Some(ref email) = self.email {
            model.email = Some(email.clone());
            changed = true;
        }
        changed
    }
}
```

### 4. Implement Your Repository

```rust
use emix_diesel::prelude::*;
use async_trait::async_trait;

pub struct UserRepository {
    // In production, use a connection pool
    conn: diesel_async::AsyncSqliteConnection,
}

#[async_trait]
impl IRepository<UserTable, User, UpdateUserDto, i64, AsyncSqliteConnection> 
    for UserRepository 
{
    async fn list(
        &self,
        filter: Option<Box<dyn FilterQuery<...> + Send + Sync>>,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<User>> {
        // Implementation...
    }
    
    // ... implement other methods
}
```

### 5. Use Your Repository

```rust
let repository = UserRepository::new(connection);

// List users with pagination
let users = repository.list(
    None,
    Some(Pagination { page: 1, page_size: 10 })
).await?;

// Get a specific user
let user = repository.get(1).await?;

// Update a user
let update_dto = UpdateUserDto {
    name: Some("New Name".to_string()),
    email: None,
};
let updated_user = repository.update(1, update_dto).await?;

// Delete a user
repository.delete(1).await?;
```

## Examples

See the `examples/` directory for complete examples including:

- Image management with tags (many-to-many relationships)
- Tag management with images
- Custom repository methods
- Filtering and pagination

## Comparison with emix-sea-orm

| Feature | emix-diesel | emix-sea-orm |
|---------|-------------|--------------|
| Type Parameters | More explicit (Table, Model, DTO, ID, Conn) | Simpler (Entity, DTO) |
| Relationships | Manual joins | Built-in relationship methods |
| Active Model | Separate Insert/Update structs | Unified ActiveModel |
| Query Building | BoxedQuery | Select/SelectTwoMany |
| Async Runtime | diesel-async | Built-in async |

Choose `emix-diesel` if you:
- Prefer Diesel's compile-time query checking
- Need fine-grained control over SQL generation
- Are already using Diesel in your project

Choose `emix-sea-orm` if you:
- Want simpler APIs with less boilerplate
- Prefer built-in relationship handling
- Need more database backend flexibility

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Related Crates

- [`emix-sea-orm`](../sea-orm) - Repository patterns for SeaORM
- [`emixcore`](../../core) - Core utilities and error types
- [`diesel`](https://diesel.rs/) - The underlying ORM
- [`diesel-async`](https://docs.rs/diesel-async/) - Async support for Diesel

