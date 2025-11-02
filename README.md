# EssentialMix-rs

A comprehensive Rust library workspace containing multiple utility crates for common development tasks.

## Project Structure

This is a Cargo workspace containing the following crates:

- **emix** (`crates/base`) - Core utility functions and components
- **emixai** (`crates/ai`) - AI-related utilities (audio, imaging, language, vision)
- **emixcore** (`crates/core`) - Core error handling and system utilities
- **emixlog** (`crates/log`) - Logging utilities
- **emixnet** (`crates/net`) - Networking utilities (VPN, web, mail)
- **emixthreading** (`crates/threading`) - Threading and concurrency utilities

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

