# emixdiesel

`emixdiesel` offers repository traits, DTO helpers, and opinionated filters for
building async Diesel-based data access layers that stay compatible with the
rest of EssentialMix.

## Feature Flags

-   `sqlite` / `postgres` / `mysql`: Enable the respective Diesel backends.
-   `sqlite-bundled`, `postgres-bundled`, `mysql-bundled`: Pull in vendored client
    libraries so you can build without system dependencies.
-   `full`: Turn on every backend (non-bundled).
-   `full-bundled`: Turn on every backend with vendored client libraries.

```toml
[dependencies]
emixdiesel = { path = "../../crates/db/diesel", features = ["postgres"] }
```

## Quick Example

```rust
use emixdiesel::prelude::*;
use emixdiesel::{ClosureFilter, TFilterQuery};
use crate::schema::posts::dsl::*;

// Start from any boxed Diesel query (useful inside repositories).
let query = posts.into_boxed();

// Wrap your filtering logic so it composes with the repository traits.
let only_published = ClosureFilter::new(|q| q.filter(published.eq(true)));

let filtered = only_published.apply(query);
```
