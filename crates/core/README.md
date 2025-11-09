# emixcore

`emixcore` provides the shared error type, debug configuration toggle, and core
traits that the other EssentialMix crates build upon.

## Highlights

- `Error` wraps workspace-specific failures while implementing `std::error::Error`.
- `Result<T>` aliases `std::result::Result<T, Error>`.
- `set_debug` / `is_debug` flip a global once-initialized flag used by helper
  crates.
- `system::num_cpus()` respects the debug flag to make deterministic unit tests
  easier (returns `1` CPU when debug mode is active).
- `CallbackHandler<T>` is a lightweight observer trait for progress reporting.

## Quick Example

```rust
use emixcore::{set_debug, system, CallbackHandler};

struct Logger;

impl CallbackHandler<u64> for Logger {
    fn starting(&self) { println!("Starting"); }
    fn update(&self, progress: u64) { println!("Progress: {progress}%"); }
    fn completed(&self) { println!("Done"); }
}

fn main() {
    set_debug(true);
    assert_eq!(system::num_cpus(), 1);
}
```

Consult `src/errors.rs` for the detailed error taxonomy and `tests/` for
real-world integration coverage.


