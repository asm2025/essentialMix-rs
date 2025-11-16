# emixdb

`emixdb` provides DTOs and traits that are shared between the Diesel and SeaORM
adapters in EssentialMix. Use it to keep pagination, result metadata, and model
merge semantics consistent across database backends.

## Building Blocks

- `Pagination`: Captures page number and page size with sensible defaults.
- `ResultSet<T>`: Couples result data with totals and pagination metadata.
- `ModelWithRelated<M, R>`: Wraps a primary model and its related entities.
- `models::TMerge`: Trait for applying update DTOs onto existing models.

## Quick Example

```rust
use emixdb::dto::{Pagination, ResultSet};

fn first_page<T>() -> Pagination {
    Pagination::default() // page 1, page_size 10
}

fn empty<T>() -> ResultSet<T> {
    ResultSet {
        data: Vec::new(),
        total: 0,
        pagination: Some(first_page()),
    }
}
```

Implement `TMerge` on your update DTOs to reuse the repository logic across the
Diesel and SeaORM crates.


