# emixseaorm - SeaORM Examples

This directory contains comprehensive examples demonstrating how to use the emixseaorm crate with all supported database backends: SQLite, PostgreSQL, MySQL, and MariaDB.

## Overview

The examples demonstrate:
- Proper database connection management with SeaORM
- Multi-database support with conditional compilation
- Repository pattern implementation
- CRUD operations using SeaORM
- Many-to-many relationships
- Transaction handling
- Active Model patterns

## Project Structure

```
examples/
├── main.rs                  # Main example application
├── models/                  # Data models and entities
│   ├── mod.rs
│   ├── image.rs            # Image entity with SeaORM derives
│   ├── tag.rs              # Tag entity with SeaORM derives
│   └── image_tag.rs        # Junction table entity
├── repositories/            # Repository implementations
│   ├── mod.rs
│   ├── image_repository.rs # Image repository with SeaORM
│   └── tag_repository.rs   # Tag repository with SeaORM
├── migrations/              # Database migration SQL files
│   ├── sqlite/
│   │   ├── up.sql          # SQLite schema creation
│   │   └── down.sql        # SQLite schema teardown
│   ├── postgres/
│   │   ├── up.sql          # PostgreSQL schema creation
│   │   └── down.sql        # PostgreSQL schema teardown
│   └── mysql/
│       ├── up.sql          # MySQL/MariaDB schema creation
│       └── down.sql        # MySQL/MariaDB schema teardown
├── README.md               # This file
└── QUICKSTART.md           # Quick start guide
```

## Features and Database Support

SeaORM supports multiple database backends through Cargo features:

- **SQLite**: `sqlite` (using sqlx-sqlite)
- **PostgreSQL**: `postgres` (using sqlx-postgres)
- **MySQL**: `mysql` (using sqlx-mysql)
- **MariaDB**: `mariadb` (alias for `mysql`, as MariaDB is MySQL-compatible)

### Feature Flags

- `sqlite` - Enable SQLite support (default)
- `postgres` - Enable PostgreSQL support
- `mysql` - Enable MySQL support
- `mariadb` - Enable MariaDB support (uses MySQL driver)
- `full` - Enable all database backends

## Prerequisites

### For SQLite
SQLite is bundled with SeaORM, so no installation is needed.

### For PostgreSQL
You need PostgreSQL installed and running:

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib
```

**macOS (Homebrew):**
```bash
brew install postgresql@16
brew services start postgresql@16
```

**Windows:**
Download and install from [postgresql.org](https://www.postgresql.org/download/windows/)

### For MySQL/MariaDB
You need MySQL or MariaDB installed and running:

**Ubuntu/Debian:**
```bash
# For MySQL
sudo apt-get update
sudo apt-get install mysql-server

# For MariaDB
sudo apt-get update
sudo apt-get install mariadb-server
```

**macOS (Homebrew):**
```bash
# For MySQL
brew install mysql
brew services start mysql

# For MariaDB
brew install mariadb
brew services start mariadb
```

**Windows:**
- MySQL: [mysql.com](https://dev.mysql.com/downloads/installer/)
- MariaDB: [mariadb.org](https://mariadb.org/download/)

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
# SQLite (default)
emixseaorm = { path = "crates/db/seaorm", features = ["sqlite"] }

# PostgreSQL
emixseaorm = { path = "crates/db/seaorm", features = ["postgres"] }

# MySQL
emixseaorm = { path = "crates/db/seaorm", features = ["mysql"] }

# MariaDB (uses MySQL driver)
emixseaorm = { path = "crates/db/seaorm", features = ["mariadb"] }

# All databases
emixseaorm = { path = "crates/db/seaorm", features = ["full"] }
```

## Database Setup

### 1. Set up your database

#### SQLite
```bash
# SQLite will create the database file automatically
# Or use in-memory database: "sqlite::memory:"
```

#### PostgreSQL
```bash
# Create a database
createdb seaorm_demo

# Or using psql
psql -U postgres
CREATE DATABASE seaorm_demo;
\q
```

#### MySQL/MariaDB
```bash
# Create a database
mysql -u root -p
CREATE DATABASE seaorm_demo;
EXIT;
```

### 2. Run migrations

SeaORM doesn't have built-in migrations like Diesel, but you can use `sea-orm-cli` or run SQL directly:

#### Using sea-orm-cli (recommended)

```bash
# Install sea-orm-cli
cargo install sea-orm-cli

# Run migrations (you'll need to set up migrations directory)
sea-orm-cli migrate up
```

#### Manual SQL execution

**SQLite:**
```bash
sqlite3 database.db < examples/migrations/sqlite/up.sql
```

**PostgreSQL:**
```bash
psql -U postgres -d seaorm_demo < examples/migrations/postgres/up.sql
```

**MySQL/MariaDB:**
```bash
mysql -u root -p seaorm_demo < examples/migrations/mysql/up.sql
```

## Running the Examples

### SQLite (default)
```bash
cd crates/db/seaorm
cargo run --example main
```

### PostgreSQL
```bash
cd crates/db/seaorm
DATABASE_URL="postgres://postgres:password@localhost/seaorm_demo" \
  cargo run --example main --features postgres --no-default-features
```

### MySQL
```bash
cd crates/db/seaorm
DATABASE_URL="mysql://root:password@localhost/seaorm_demo" \
  cargo run --example main --features mysql --no-default-features
```

### MariaDB
```bash
cd crates/db/seaorm
DATABASE_URL="mysql://root:password@localhost/seaorm_demo" \
  cargo run --example main --features mariadb --no-default-features
```

## Code Examples

### Creating a Database Connection

```rust
use sea_orm::{Database, DatabaseConnection, DbErr};

async fn create_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}
```

### Defining Entities

SeaORM uses derive macros to generate entities:

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::image_tag::Entity")]
    ImageTag,
}

impl ActiveModelBehavior for ActiveModel {}
```

### Using the Repository

```rust
// Create repository with connection
let db = Database::connect(&database_url).await?;
let image_repo = ImageRepository::new(db.clone());

// Create an image
let image = image_repo.create(CreateImageDto {
    title: "My Image".to_string(),
    description: Some("A description".to_string()),
    extension: "jpg".to_string(),
    file_size: 1024,
    mime_type: "image/jpeg".to_string(),
    width: Some(800),
    height: Some(600),
    alt_text: Some("Alt text".to_string()),
    tags: None,
}.into()).await?;

// List images with pagination
let images = image_repo.list(None, Some(Pagination {
    page: 1,
    page_size: 10,
})).await?;

// Get image with related tags
let image_with_tags = image_repo.get_with_related(image.id).await?;

// Update an image
let updated = image_repo.update(image.id, UpdateImageDto {
    title: Some("Updated Title".to_string()),
    ..Default::default()
}).await?;

// Delete an image
image_repo.delete(image.id).await?;
```

### Working with Tags

```rust
// Create tag repository
let tag_repo = TagRepository::new(db.clone());

// Create a tag
let tag = tag_repo.create(CreateTagDto {
    name: "Nature".to_string(),
}.into()).await?;

// Add tag to image
image_repo.add_tag(image.id, tag.id).await?;

// Get tag with its images
let tag_with_images = tag_repo.get_with_related(tag.id).await?;

// List images for a tag
let images = tag_repo.list_images(tag.id, None, None, Some(Pagination {
    page: 1,
    page_size: 10,
})).await?;
```

## Key Patterns and Best Practices

### 1. Entity Definition

SeaORM uses the Active Record pattern. Each entity has:
- **Model**: The data structure
- **ActiveModel**: For inserts and updates (with `Set`, `NotSet`, `Unchanged`)
- **Entity**: The table representation
- **Column**: Column enum for queries

### 2. Active Model Pattern

```rust
use sea_orm::{Set, NotSet, ActiveModelTrait};

let active_model = ActiveModel {
    id: NotSet,
    title: Set("My Title".to_string()),
    description: Set(Some("Description".to_string())),
    ..Default::default()
};

let result = active_model.insert(&db).await?;
```

### 3. Relationships

Define relationships using derive macros:

```rust
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tag::Entity")]
    Tags,
}

impl Related<tag::Entity> for Entity {
    fn to() -> RelationDef {
        image_tag::Relation::TagEntity.def()
    }
    fn via() -> Option<RelationDef> {
        Some(image_tag::Relation::ImageEntity.def().rev())
    }
}
```

### 4. Queries with Filters

```rust
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

let images = ImageEntity::find()
    .filter(ImageColumn::Title.contains("sunset"))
    .filter(ImageColumn::Width.gte(1000))
    .all(&db)
    .await?;
```

### 5. Transactions

```rust
use sea_orm::TransactionTrait;

let txn = db.begin().await?;

// Perform operations
let image = image_repo.create(create_dto).await?;
let tag = tag_repo.create(tag_dto).await?;

// Commit or rollback
txn.commit().await?;
```

### 6. Error Handling

The examples use the emixcore `Result` type:

```rust
use emixseaorm::{Error, Result};

async fn some_operation(&self) -> Result<Model> {
    ImageEntity::find_by_id(id)
        .one(&self.db)
        .await
        .map_err(Error::from_std_error)?
        .ok_or_else(|| /* handle not found */)
}
```

### 7. Batch Operations

```rust
use sea_orm::Insert;

let items = vec![active_model1, active_model2, active_model3];
ImageEntity::insert_many(items)
    .exec(&db)
    .await?;
```

## Troubleshooting

### Connection Errors

**SQLite:**
- Check file permissions if using file-based database
- For in-memory databases, connection string is `sqlite::memory:`

**PostgreSQL:**
- Verify PostgreSQL is running: `pg_isready`
- Check connection string format: `postgres://user:pass@host/db`
- Ensure user has proper permissions

**MySQL/MariaDB:**
- Verify server is running: `mysqladmin ping`
- Check connection string format: `mysql://user:pass@host/db`
- Ensure user has proper permissions

### Migration Errors

If migrations fail:
- Check SQL syntax for your specific database
- Verify database permissions
- Make sure the database exists

### Compilation Errors

If you get feature-related errors:
- Make sure you're using the correct feature flag
- Only enable one database feature at a time (unless using `full`)

## Comparison with Diesel

### Similarities
- Both use type-safe query builders
- Both support multiple databases
- Both have connection pooling support

### Differences

**SeaORM:**
- Uses Active Record pattern
- Async-first design with Tokio
- Uses derive macros for entity definition
- Simpler relationship handling
- Built on SQLx

**Diesel:**
- Uses Query Builder pattern
- Synchronous with diesel-async wrapper
- Schema-first with `schema.rs`
- More explicit type system
- Native implementation

## References

- [SeaORM Official Documentation](https://www.sea-ql.org/SeaORM/docs/index/)
- [SeaORM Tutorial](https://www.sea-ql.org/sea-orm-tutorial/)
- [SeaORM API Documentation](https://docs.rs/sea-orm/)
- [SeaORM Examples on GitHub](https://github.com/SeaQL/sea-orm/tree/master/examples)
- [SeaQL Organization](https://www.sea-ql.org/)

## License

This project is licensed under the MIT License.

