# Quick Start Guide - emixseaorm Examples

This guide will get you up and running with the emixseaorm examples in 5 minutes.

## 1. Choose Your Database

Pick one of the supported databases. SQLite is the easiest to get started with.

### Option A: SQLite (Recommended for Testing)

No installation needed! SQLite is bundled with SeaORM.

### Option B: PostgreSQL

Install PostgreSQL first:
```bash
# Ubuntu/Debian
sudo apt-get install postgresql postgresql-contrib

# macOS
brew install postgresql@16
brew services start postgresql@16

# Windows
# Download from https://www.postgresql.org/download/windows/
```

### Option C: MySQL

Install MySQL first:
```bash
# Ubuntu/Debian
sudo apt-get install mysql-server

# macOS
brew install mysql
brew services start mysql

# Windows
# Download from https://dev.mysql.com/downloads/installer/
```

### Option D: MariaDB

Install MariaDB first:
```bash
# Ubuntu/Debian
sudo apt-get install mariadb-server

# macOS
brew install mariadb
brew services start mariadb

# Windows
# Download from https://mariadb.org/download/
```

## 2. Set Up Database (If Not Using SQLite)

### For SQLite:

No setup needed! The example uses in-memory database by default, or you can specify a file.

### For PostgreSQL:

```bash
# Create database
createdb seaorm_demo

# Run migration
psql -U postgres -d seaorm_demo < examples/migrations/postgres/up.sql
```

### For MySQL/MariaDB:

```bash
# Create database
mysql -u root -p -e "CREATE DATABASE seaorm_demo"

# Run migration
mysql -u root -p seaorm_demo < examples/migrations/mysql/up.sql
```

## 3. Run the Example

```bash
# Navigate to the crate root
cd crates/db/seaorm

# Run with SQLite (default - uses in-memory database)
cargo run --example main

# Run with PostgreSQL
DATABASE_URL="postgres://postgres:password@localhost/seaorm_demo" \
  cargo run --example main --features postgres --no-default-features

# Run with MySQL
DATABASE_URL="mysql://root:password@localhost/seaorm_demo" \
  cargo run --example main --features mysql --no-default-features

# Run with MariaDB (uses MySQL driver)
DATABASE_URL="mysql://root:password@localhost/seaorm_demo" \
  cargo run --example main --features mariadb --no-default-features
```

## Expected Output

You should see output similar to:

```
=== emixseaorm Example ===

Database backend: SQLite

Database URL: sqlite::memory:

--- Creating Tags ---
Created tag: Nature (ID: 1)
Created tag: Landscape (ID: 2)

--- Creating Image with Tags ---
Created image: Mountain Sunset (ID: 1)

--- Getting Image with Tags ---
Image: Mountain Sunset
Tags:
  - Nature
  - Landscape

--- Listing Images (Paginated) ---
Found 1 total images
  - Mountain Sunset (1)

--- Listing Tags with Images ---
Found 2 total tags
Tag: Nature
  Images:
    - Mountain Sunset
Tag: Landscape
  Images:
    - Mountain Sunset

--- Updating Image ---
Updated image title to: Beautiful Mountain Sunset

--- Adding Tag to Image ---
Added tag 'Photography' to image 'Beautiful Mountain Sunset'

--- Counting Records ---
Total images: 1
Total tags: 3

--- Listing Tags for Image ---
Image 'Beautiful Mountain Sunset' has 3 tags:
  - Nature
  - Landscape
  - Photography

--- Cleanup ---
Deleted image: Beautiful Mountain Sunset
Deleted all tags

=== Example completed successfully! ===
```

## Troubleshooting

### SQLite: Database connection error

If using a file-based database:
```bash
# Make sure the file is writable
touch database.db
chmod 666 database.db
```

For in-memory database (default), no file is needed.

### PostgreSQL: "connection refused"

Make sure PostgreSQL is running:
```bash
# Check status
pg_isready

# Start if not running
# Ubuntu/Debian
sudo service postgresql start

# macOS
brew services start postgresql@16

# Windows
# Use Services app to start PostgreSQL service
```

### MySQL/MariaDB: "Access denied"

Make sure your credentials are correct:
```bash
# Test connection
mysql -u root -p

# If you need to reset password
sudo mysql_secure_installation
```

### "table not found" error

Make sure you've run the migrations:

**PostgreSQL:**
```bash
psql -U postgres -d seaorm_demo < examples/migrations/postgres/up.sql
```

**MySQL/MariaDB:**
```bash
mysql -u root -p seaorm_demo < examples/migrations/mysql/up.sql
```

**SQLite (if using file-based):**
```bash
sqlite3 database.db < examples/migrations/sqlite/up.sql
```

### Feature compilation errors

Make sure you're using the correct feature flag:
```bash
# Only one database feature should be enabled at a time
cargo run --example main --features postgres --no-default-features
```

## Next Steps

1. **Read the README.md** for detailed documentation
2. **Explore the code** in `examples/models/` and `examples/repositories/`
3. **Modify the example** to add your own entities
4. **Try different databases** to see how SeaORM handles them

## Project Structure

```
examples/
├── QUICKSTART.md          ← You are here
├── README.md              ← Full documentation
├── main.rs                ← Example application
├── models/                ← Data models (entities)
│   ├── image.rs           ← Image entity
│   ├── tag.rs             ← Tag entity
│   └── image_tag.rs       ← Junction table
├── repositories/          ← Repository pattern
│   ├── image_repository.rs
│   └── tag_repository.rs
└── migrations/            ← Database migrations
    ├── sqlite/
    ├── postgres/
    └── mysql/
```

## Common Tasks

### Add a New Field to Image

1. Update the migration SQL file (`migrations/{db}/up.sql`)
2. Update `Model` struct in `models/image.rs`
3. Update `CreateImageDto` and `UpdateImageDto`
4. Update the `Merge` implementation
5. Re-run migration

### Create a New Entity

1. Create model file in `models/` (e.g., `user.rs`)
2. Define the entity using `DeriveEntityModel`
3. Create repository in `repositories/`
4. Update `mod.rs` files to export
5. Create migration SQL

### Use in Your Own Project

Add to your `Cargo.toml`:
```toml
[dependencies]
emixseaorm = { path = "path/to/crates/db/seaorm", features = ["sqlite"] }
sea-orm = { version = "1", features = ["macros", "sqlx-sqlite", "runtime-tokio-rustls"] }
tokio = { version = "1", features = ["full"] }
```

Basic usage:
```rust
use emixseaorm::prelude::*;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create connection
    let db = Database::connect("sqlite::memory:").await?;
    
    // Create repository
    let repo = ImageRepository::new(db);
    
    // Use repository...
    let images = repo.list(None, None).await?;
    
    Ok(())
}
```

## Key Differences from Diesel

If you're coming from Diesel (or emixdiesel):

1. **Async-first**: SeaORM is fully async with Tokio
2. **Active Record**: Uses `ActiveModel` instead of separate insert/update structs
3. **Derive-based**: Entities are defined with derive macros
4. **Simpler relations**: Relationships are easier to define
5. **No schema file**: Tables are defined directly on structs

Example comparison:

**Diesel:**
```rust
// Separate schema definition
table! {
    images (id) {
        id -> Int8,
        title -> Text,
    }
}

// Manual struct
#[derive(Queryable)]
pub struct Image {
    pub id: i64,
    pub title: String,
}
```

**SeaORM:**
```rust
// All-in-one entity definition
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
}
```

## Database Connection Strings

### SQLite
```
# In-memory (default)
sqlite::memory:

# File-based
sqlite:path/to/database.db?mode=rwc
```

### PostgreSQL
```
postgres://username:password@localhost:5432/database_name

# With SSL
postgres://username:password@localhost:5432/database_name?sslmode=require
```

### MySQL/MariaDB
```
mysql://username:password@localhost:3306/database_name

# With SSL
mysql://username:password@localhost:3306/database_name?ssl-mode=required
```

## Performance Tips

1. **Use connection pooling** in production (SeaORM does this automatically)
2. **Batch inserts** for multiple records
3. **Use select-specific columns** when you don't need all fields
4. **Eager load relations** when you know you'll need them
5. **Use indexes** on frequently queried columns

## Need Help?

- Check the [full README](README.md) for detailed documentation
- See [SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/index/)
- Review [SeaORM tutorial](https://www.sea-ql.org/sea-orm-tutorial/)
- Look at [SeaORM examples on GitHub](https://github.com/SeaQL/sea-orm/tree/master/examples)

Happy coding! 🚀

