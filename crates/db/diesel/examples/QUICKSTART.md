# Quick Start Guide - emixdiesel Examples

This guide will get you up and running with the emixdiesel examples in 5 minutes.

## 1. Choose Your Database

Pick one of the three supported databases. SQLite is the easiest to get started with.

### Option A: SQLite (Recommended for Testing)

No installation needed! The bundled feature includes everything.

### Option B: PostgreSQL

Install PostgreSQL first:
```bash
# Ubuntu/Debian
sudo apt-get install postgresql

# macOS
brew install postgresql

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

# Windows
# Download from https://dev.mysql.com/downloads/installer/
```

## 2. Set Up Environment

Create a `.env` file in the `examples` directory:

```bash
# Navigate to examples directory
cd crates/db/diesel/examples

# Create .env file (choose one):

# For SQLite:
echo "DATABASE_URL=test.db" > .env

# For PostgreSQL:
echo "DATABASE_URL=postgres://postgres:password@localhost/diesel_demo" > .env

# For MySQL:
echo "DATABASE_URL=mysql://root:password@localhost/diesel_demo" > .env
```

## 3. Create the Database Schema

### For SQLite:

The example will create the database file automatically. Just run the migration:

```bash
# From examples directory
sqlite3 test.db < migrations/sqlite/up.sql
```

Or let the example create the tables (if you modify main.rs to include migrations).

### For PostgreSQL:

```bash
# Create database
createdb diesel_demo

# Run migration
psql -d diesel_demo < migrations/postgres/up.sql
```

### For MySQL:

```bash
# Create database
mysql -u root -p -e "CREATE DATABASE diesel_demo"

# Run migration
mysql -u root -p diesel_demo < migrations/mysql/up.sql
```

## 4. Run the Example

```bash
# Navigate to the crate root
cd crates/db/diesel

# Run with SQLite (default)
cargo run --example main

# Run with PostgreSQL
cargo run --example main --features postgres --no-default-features

# Run with MySQL
cargo run --example main --features mysql --no-default-features
```

## Expected Output

You should see output similar to:

```
=== emixdiesel Example ===

Database backend: SQLite
Database URL: test.db

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

... (more output)

=== Example completed successfully! ===
```

## Troubleshooting

### SQLite: "unable to open database file"

Make sure you have write permissions in the current directory:
```bash
touch test.db
chmod 666 test.db
```

### PostgreSQL: "connection refused"

Make sure PostgreSQL is running:
```bash
# Check status
pg_isready

# Start if not running
# Ubuntu/Debian
sudo service postgresql start

# macOS
brew services start postgresql
```

### MySQL: "Access denied"

Make sure your credentials are correct:
```bash
# Test connection
mysql -u root -p

# If you need to reset password
sudo mysql_secure_installation
```

### Compilation Error: "cannot find -lmysqlclient"

Use the bundled feature:
```bash
cargo run --example main --features mysql-bundled --no-default-features
```

## Next Steps

1. **Read the README.md** for detailed documentation
2. **Explore the code** in `examples/models/` and `examples/repositories/`
3. **Modify the example** to add your own entities
4. **Check CHANGES.md** to understand the implementation patterns

## Project Structure

```
examples/
├── QUICKSTART.md          ← You are here
├── README.md              ← Full documentation
├── main.rs                ← Example application
├── models/                ← Data models
│   ├── tables.rs          ← Schema definitions
│   ├── image.rs           ← Image model
│   ├── tag.rs             ← Tag model
│   └── image_tag.rs       ← Junction table
├── repositories/          ← Repository pattern
│   ├── image_repository.rs
│   └── tag_repository.rs
└── migrations/            ← Database migrations
    ├── sqlite/
    │   ├── up.sql
    │   └── down.sql
    ├── postgres/
    │   ├── up.sql
    │   └── down.sql
    └── mysql/
        ├── up.sql
        └── down.sql
```

## Common Tasks

### Add a New Field to Image

1. Update the migration SQL file
2. Update `tables.rs` schema
3. Update `ImageModel` in `image.rs`
4. Update `NewImageModel` and `UpdateImageModel`
5. Run migration

### Create a New Entity

1. Add table definition in `tables.rs`
2. Create model file in `models/`
3. Create repository in `repositories/`
4. Update `mod.rs` files to export

### Use in Your Own Project

```toml
[dependencies]
emixdiesel = { path = "path/to/crates/db/diesel", features = ["sqlite-bundled"] }
diesel = { version = "2.2", features = ["chrono"] }
diesel-async = { version = "0.5" }
tokio = { version = "1", features = ["full"] }
```

```rust
use emixdiesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create connection pool
    let config = AsyncDieselConnectionManager::new("database.db");
    let pool = Pool::builder(config).build()?;
    
    // Create repository
    let repo = ImageRepository::new(pool);
    
    // Use repository...
    let images = repo.list(None).await?;
    
    Ok(())
}
```

## Need Help?

- Check the [full README](README.md) for detailed documentation
- Review [CHANGES.md](../CHANGES.md) for implementation details
- See [Diesel documentation](https://diesel.rs) for Diesel-specific questions
- Look at the [diesel-async documentation](https://docs.rs/diesel-async) for async patterns

Happy coding! 🚀

