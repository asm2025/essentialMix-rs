# emix

`emix` collects everyday utilities that sit on top of `emixcore`, ranging from
filesystem helpers to string manipulation, random data generators, and terminal
tooling. Pull in the features you need and keep the rest out of your build.

## Feature Flags

- `terminal`: Enables terminal UI helpers backed by `crossterm`, `dialoguer`,
  and `rpassword`.
- `fake`: Unlocks rich fake-data generation via the `fake` crate.
- `full`: Convenience flag for `["terminal", "fake"]`.

```toml
[dependencies]
emix = { path = "../../crates/base", features = ["terminal"] }
```

## Quick Examples

Path utilities:

```rust
use emix::io::path::PathEx;

let manifest = ("crates", "base", "Cargo.toml");
assert!(manifest.as_path().ends_with("crates\\base\\Cargo.toml") || manifest.as_path().ends_with("crates/base/Cargo.toml"));
```

String helpers:

```rust
use emix::string::StringEx;

let slug = "--EssentialMix--".trim_char(&'-').to_lowercase();
assert_eq!(slug, "essentialmix");
```

Fake data (behind the `fake` feature):

```rust
use emix::random::person::FullName;

let name = FullName().fake();
println!("Hello, {name}!");
```

Explore additional modules in `src/` and the corresponding tests under
`tests/` for more patterns.


