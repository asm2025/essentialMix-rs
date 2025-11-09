# emixdiesel - Diesel ORM Examples

This directory contains comprehensive examples demonstrating how to use the emixdiesel crate with all three supported database backends: SQLite, PostgreSQL, and MySQL.

## Overview

The examples demonstrate:
- Proper connection pooling using `deadpool`
- Multi-database support with conditional compilation
- Repository pattern implementation
- CRUD operations
- Many-to-many relationships
- Transaction handling

## Project Structure

```
examples/
├── main.rs                  # Main example application
├── models/                  # Data models and schema
│   ├── mod.rs
│   ├── tables.rs           # Diesel schema definitions
│   ├── image.rs            # Image model, DTOs, and type conversions
│   ├── tag.rs              # Tag model, DTOs, and type conversions
│   └── image_tag.rs        # Junction table model
├── repositories/            # Repository implementations
│   ├── mod.rs
│   ├── image_repository.rs # Image repository with connection pooling
│   └── tag_repository.rs   # Tag repository with connection pooling
├── migrations/              # Database migration SQL files
│   ├── sqlite/
│   │   ├── up.sql          # SQLite schema creation
│   │   └── down.sql        # SQLite schema teardown
│   ├── postgres/
│   │   ├── up.sql          # PostgreSQL schema creation
│   │   └── down.sql        # PostgreSQL schema teardown
│   └── mysql/
│       ├── up.sql          # MySQL schema creation
│       └── down.sql        # MySQL schema teardown
├── README.md               # This file
├── QUICKSTART.md           # Quick start guide
└── CHANGES.md              # Detailed changelog
```

## Features and Database Support

The crate supports three database backends through Cargo features:

- **SQLite**: `sqlite` (with optional `sqlite-bundled`)
- **PostgreSQL**: `postgres` (with optional `postgres-bundled`) 
- **MySQL**: `mysql` (with optional `mysql-bundled`)

### Feature Flags

- `sqlite` - Enable SQLite support
- `sqlite-bundled` - Enable SQLite with bundled libsqlite3
- `postgres` - Enable PostgreSQL support
- `postgres-bundled` - Enable PostgreSQL with bundled libpq and OpenSSL
- `mysql` - Enable MySQL support
- `mysql-bundled` - Enable MySQL with bundled libmysqlclient
- `full` - Enable all database backends
- `full-bundled` - Enable all backends with bundled dependencies

## Prerequisites

### For SQLite
SQLite is bundled by default when using the `sqlite-bundled` feature (which is the default).

### For PostgreSQL
You need PostgreSQL installed and running:

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib libpq-dev
```

**macOS (Homebrew):**
```bash
brew install postgresql
brew services start postgresql
```

**Windows:**
Download and install from [postgresql.org](https://www.postgresql.org/download/windows/)

Or use the bundled feature: `postgres-bundled`

### For MySQL
You need MySQL or MariaDB installed and running:

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install mysql-server libmysqlclient-dev
```

**macOS (Homebrew):**
```bash
brew install mysql
brew services start mysql
```

**Windows:**
Download and install from [mysql.com](https://dev.mysql.com/downloads/installer/)

Or use the bundled feature: `mysql-bundled`

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
# SQLite (default)
emixdiesel = { path = "crates/db/diesel", features = ["sqlite-bundled"] }

# PostgreSQL
emixdiesel = { path = "crates/db/diesel", features = ["postgres-bundled"] }

# MySQL
emixdiesel = { path = "crates/db/diesel", features = ["mysql-bundled"] }

# All databases
emixdiesel = { path = "crates/db/diesel", features = ["full-bundled"] }
```

## Database Setup

### 1. Set up your database

#### SQLite
```bash
# SQLite will create the database file automatically
touch database.db
```

#### PostgreSQL
```bash
# Create a database
createdb diesel_demo

# Or using psql
psql -U postgres
CREATE DATABASE diesel_demo;
\q
```

#### MySQL
```bash
# Create a database
mysql -u root -p
CREATE DATABASE diesel_demo;
EXIT;
```

### 2. Create a `.env` file

Create a `.env` file in the `examples` directory:

#### For SQLite:
```bash
DATABASE_URL=database.db
```

#### For PostgreSQL:
```bash
DATABASE_URL=postgres://username:password@localhost/diesel_demo
```

#### For MySQL:
```bash
DATABASE_URL=mysql://username:password@localhost/diesel_demo
```

### 3. Run migrations

For a production application, you would use `diesel_cli` to manage migrations:

```bash
# Install Diesel CLI (if not already installed)
cargo install diesel_cli --no-default-features --features sqlite  # or postgres, mysql

# Run migrations
diesel migration run --database-url=DATABASE_URL
```

#### SQLite Migration

**up.sql:**
```sql
CREATE TABLE images (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    extension TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type TEXT NOT NULL,
    width INTEGER,
    height INTEGER,
    alt_text TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE image_tags (
    image_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (image_id, tag_id),
    FOREIGN KEY (image_id) REFERENCES images(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_image_tags_image_id ON image_tags(image_id);
CREATE INDEX idx_image_tags_tag_id ON image_tags(tag_id);
```

**down.sql:**
```sql
DROP TABLE IF EXISTS image_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS images;
```

#### PostgreSQL Migration

**up.sql:**
```sql
CREATE TABLE images (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    extension TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type TEXT NOT NULL,
    width INTEGER,
    height INTEGER,
    alt_text TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE image_tags (
    image_id BIGINT NOT NULL REFERENCES images(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (image_id, tag_id)
);

CREATE INDEX idx_image_tags_image_id ON image_tags(image_id);
CREATE INDEX idx_image_tags_tag_id ON image_tags(tag_id);
```

**down.sql:**
```sql
DROP TABLE IF EXISTS image_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS images;
```

#### MySQL Migration

**up.sql:**
```sql
CREATE TABLE images (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    extension VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    width INTEGER,
    height INTEGER,
    alt_text TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE tags (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE image_tags (
    image_id BIGINT NOT NULL,
    tag_id BIGINT NOT NULL,
    PRIMARY KEY (image_id, tag_id),
    FOREIGN KEY (image_id) REFERENCES images(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_image_tags_image_id ON image_tags(image_id);
CREATE INDEX idx_image_tags_tag_id ON image_tags(tag_id);
```

**down.sql:**
```sql
DROP TABLE IF EXISTS image_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS images;
```

## Running the Examples

### SQLite (default)
```bash
cd crates/db/diesel
cargo run --example main --features sqlite-bundled
```

### PostgreSQL
```bash
cd crates/db/diesel
cargo run --example main --features postgres-bundled --no-default-features
```

### MySQL
```bash
cd crates/db/diesel
cargo run --example main --features mysql-bundled --no-default-features
```

## Code Examples

### Creating a Connection Pool

```rust
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;

#[cfg(feature = "sqlite")]
use diesel_async::AsyncSqliteConnection as Connection;

#[cfg(feature = "postgres")]
use diesel_async::AsyncPgConnection as Connection;

#[cfg(feature = "mysql")]
use diesel_async::AsyncMysqlConnection as Connection;

async fn create_pool(database_url: &str) -> Pool<Connection> {
    let config = AsyncDieselConnectionManager::<Connection>::new(database_url);
    Pool::builder(config)
        .build()
        .expect("Failed to create connection pool")
}
```

### Using the Repository

```rust
// Create repository with connection pool
let pool = create_pool(&database_url).await;
let image_repo = ImageRepository::new(pool.clone());

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
}).await?;

// List images with pagination
let images = image_repo.list(Some(Pagination {
    page: 1,
    page_size: 10,
})).await?;

// Get image with related tags
let image_with_tags = image_repo.get_with_tags(image.id).await?;

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
let tag_repo = TagRepository::new(pool.clone());

// Create a tag
let tag = tag_repo.create(CreateTagDto {
    name: "Nature".to_string(),
}).await?;

// Add tag to image
image_repo.add_tag(image.id, tag.id).await?;

// Get tag with its images
let tag_with_images = tag_repo.get_with_images(tag.id).await?;

// List images for a tag
let images = tag_repo.list_images(tag.id, Some(Pagination {
    page: 1,
    page_size: 10,
})).await?;
```

## Key Patterns and Best Practices

### 1. Connection Pooling
Always use connection pooling in production. The examples use `deadpool` which is the recommended pool for `diesel-async`.

### 2. Conditional Compilation
The schema and models use `#[cfg(feature = "...")]` attributes to support multiple databases. Each database has slightly different column types:

- **SQLite**: `Integer` for IDs, `Text` for strings
- **PostgreSQL**: `Int8` (i64) for IDs, `Text` for strings, `Timestamp` for datetime
- **MySQL**: `Bigint` for IDs, `Varchar` for limited strings, `Text` for unlimited strings

### 3. Type Safety
Use type aliases for IDs that change based on the database:
```rust
#[cfg(feature = "sqlite")]
pub type ImageId = i32;

#[cfg(feature = "postgres")]
pub type ImageId = i64;

#[cfg(feature = "mysql")]
pub type ImageId = i64;
```

### 4. Error Handling
The examples use the emixcore `Result` type which wraps errors properly:
```rust
use emixdiesel::{Error, Result};

async fn some_operation(&self) -> Result<Model> {
    // ...
    .map_err(Error::from_std_error)
}
```

### 5. Transactions
For operations that need atomicity, use transactions:
```rust
conn.transaction::<_, Error, _>(|conn| async move {
    // Multiple operations here
    Ok(result)
}.scope_boxed()).await?;
```

### 6. Batch Operations
When inserting multiple records, use batch inserts:
```rust
let items: Vec<NewModel> = // ... create items
diesel::insert_into(table)
    .values(&items)
    .execute(&mut conn)
    .await?;
```

## Troubleshooting

### Connection Errors

**SQLite:**
- Make sure the database file exists and is writable
- Check file permissions

**PostgreSQL:**
- Verify PostgreSQL is running: `pg_isready`
- Check connection string format
- Ensure user has proper permissions

**MySQL:**
- Verify MySQL is running: `mysqladmin ping`
- Check connection string format
- Ensure user has proper permissions

### Compilation Errors

If you get linker errors:
- Make sure you're using the `-bundled` features
- Or install the native libraries for your database

### Migration Errors

If migrations fail:
- Check SQL syntax for your specific database
- Verify database permissions
- Make sure the database exists

## References

- [Diesel Official Guide](https://diesel.rs/guides/getting-started)
- [Diesel API Documentation](https://docs.rs/diesel)
- [Diesel Async Documentation](https://docs.rs/diesel-async)
- [Deadpool Documentation](https://docs.rs/deadpool)

## License

This project is licensed under the MIT License.

