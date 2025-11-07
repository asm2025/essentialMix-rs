# emixdb

`emixdb` is a shared Rust library providing common types and traits for database ORM abstractions used across the essentialMix-rs ecosystem.

## Overview

This crate contains common types and traits that are shared between different ORM implementations (like Diesel and SeaORM) to promote code reuse and consistency.

## Common Types

### Repository Types

- **`ModelWithRelated<M, R>`**: A wrapper type for models with their related entities
  - `item: M` - The main model
  - `related: Vec<R>` - Vector of related models

- **`Pagination`**: Pagination parameters for query results
  - `page: u64` - Page number (default: 1)
  - `page_size: u64` - Number of items per page (default: 10)

- **`ResultSet<T>`**: A result set with pagination information
  - `data: Vec<T>` - The actual data
  - `total: u64` - Total count of items
  - `pagination: Option<Pagination>` - Optional pagination info

### Schema Types

- **`Merge<T>`**: Trait for merging update models into existing models
  - `fn merge(&self, model: &mut T) -> bool` - Merges changes into the target model

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
emixdb = { workspace = true }
```

Then import the types:

```rust
use emixdb::prelude::*;
```

## Integration

This crate is used by:
- `emix-diesel` - Diesel ORM integration
- `emix-sea-orm` - SeaORM integration

Each ORM crate extends these common types with ORM-specific functionality while maintaining a consistent interface.

## License

MIT

