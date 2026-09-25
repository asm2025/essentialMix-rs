# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Cargo workspace (edition 2024) of utility library crates published to crates.io under the `emix*` names. The root package `essentialmix-rs` (`src/main.rs`) is only a placeholder binary and is never published.

## Commands

```bash
cargo build                                   # root package only
cargo build --workspace                       # all crates
cargo test --workspace                        # all tests (the --workspace flag is required; plain `cargo test` only tests the root)
cargo test -p emix                            # one crate (use package name, not dir name)
cargo test -p emix --test string              # one integration test file (crates/base/tests/string.rs)
cargo test -p emix --test string some_fn      # single test by name filter
cargo test -p emixai --features audio -- --ignored   # manual tests (need API keys / model downloads)
cargo test -p emixdiesel --features postgres  # feature-gated code needs its feature enabled
```

- Tests live in `crates/*/tests/*.rs` (integration tests), not inline modules.
- Tests marked `#[ignore]` require external resources (OpenAI key, model downloads, network, mail/VPN). Their comments give the exact run command.
- Non-ignored tests must not hit the network: `emixnet` HTTP tests use a local `httpmock` server (`crates/net/tests/web.rs`).
- Development happens in WSL (Ubuntu). Native features need system packages: `build-essential pkg-config perl cmake libssl-dev libpq-dev libmysqlclient-dev` (diesel backends, `*-bundled`), plus `libasound2-dev` for `emixai`'s `audio` feature. The Windows `x86_64-pc-windows-gnu` toolchain cannot build `ort-sys` (`emixai` audio).
- `.cargo/config.toml` sets `-Adead_code` globally, so unused-code warnings are suppressed.
- Publishing: `cargo publish --workspace --dry-run`, then `cargo publish --workspace`. Cargo orders crates by dependency, waits for each to reach the index, and the dry-run checks dependents against the local packages. The root package has `publish = false`. To resume after a partial publish, add `--exclude <crate>` for each crate already uploaded.

## Architecture

Package name vs directory (package names are used with `-p`):

| Package | Dir | Depends on |
|---|---|---|
| emixcore | crates/core | none (thiserror, num_cpus) |
| emix | crates/base | emixcore |
| emixcollections | crates/collections | emixcore |
| emixcrypto | crates/crypto | emixcore |
| emixthreading | crates/threading | emixcore |
| emixlog | crates/log | emixcore |
| emixdb | crates/db/common | emixcore |
| emixdiesel / emixseaorm | crates/db/diesel, crates/db/seaorm | emixcore, emixdb |
| emixnet | crates/net | emix, emixcore |
| emixai | crates/ai | emix |

Key conventions spanning crates:

- **Shared error type**: `emixcore::Error` (big `thiserror` enum) and `emixcore::Result<T>`. Most crates re-export them as `pub use emixcore::{Error, Result}`. Crates with domain errors (`net`, `log`, `crypto`) define their own enum (e.g. `NetError`, `LogError`) plus `impl From<XError> for emixcore::Error`.
- **Global debug flag**: `emixcore::set_debug` / `is_debug` (a `OnceLock`, so it can be set only once). With debug on, `emixcore::system::num_cpus()` returns 1, which makes threading code run single-threaded.
- **Feature-gated modules**: optional functionality is behind Cargo features and `#[cfg(feature = "...")]` modules. Most crates have a `full` feature. Notable defaults: `emixai` = `language` (kalosm + async-openai; `cuda`/`metal`/`mkl` for acceleration), `emixlog` = `log4rs` (alt `slog`), `emixcrypto` = aes/cbc/rsa/pbkdf2 (`sha2` is always on; the feature is a no-op kept for compatibility), `emixdiesel` = `sqlite-bundled` (`*-bundled` variants vendor native client libs), `emixseaorm` = `sqlite`. `emix` has `terminal` and `fake`; `emixnet` has `mail` (enables `emix/fake`) and `vpn`.
- **DB layer**: `emixdb` holds backend-agnostic DTOs/models. `emixdiesel` and `emixseaorm` each mirror that layout (`dto.rs`, `models.rs`) and add backend-specific repository/filter traits (e.g. `TFilterQuery`, `TFilterCondition` in SeaORM).

## Versioning

All crates inherit `version` from `[workspace.package]` in the root `Cargo.toml`. Internal dependencies are declared once in `[workspace.dependencies]` with both `path` and `version`. When bumping the version, update `[workspace.package].version` and every `version` in `[workspace.dependencies]` together. Crate dependencies use loose `"0"` / `"1"` version specs.
