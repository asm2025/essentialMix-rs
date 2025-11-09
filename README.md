# EssentialMix-rs

A comprehensive Rust library workspace containing multiple utility crates for common development tasks.

## Project Structure

This is a Cargo workspace containing the following crates:

- **emixcore** (`crates/core`) – shared error types, debug flag, and traits
- **emix** (`crates/base`) – core utility functions and components
- **emixai** (`crates/ai`) – AI-focused helpers (audio, imaging, language, vision)
- **emixdb** (`crates/db/common`) – shared DTOs for database abstractions
- **emixdiesel** (`crates/db/diesel`) – async Diesel repositories and helpers
- **emixseaorm** (`crates/db/seaorm`) – SeaORM repositories and filters
- **emixlog** (`crates/log`) – logging utilities and adapters
- **emixnet** (`crates/net`) – networking utilities (VPN, web, mail)
- **emixthreading** (`crates/threading`) – threading and concurrency utilities

Each crate has a dedicated `README.md` under its directory with feature notes and
usage examples.

## Building

```bash
cargo build
```

## Testing

**Important:** This is a workspace project. To run all tests, you need to use the `--workspace` flag:

```bash
cargo test --workspace
```

To run tests for a specific crate:

```bash
cd crates/base
cargo test
```

Or from the root:

```bash
cargo test -p emix
```

## Running the Application

```bash
cargo run
```

## Features

Each crate supports optional features. Check individual crate `Cargo.toml` files for available features.

## License

MIT

