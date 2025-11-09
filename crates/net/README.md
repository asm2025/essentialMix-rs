# emixnet

`emixnet` layers higher-level networking helpers on top of `reqwest`, `lettre`,
and supporting crates. It includes HTTP client builders, VPN automation hooks,
and mail utilities that reuse EssentialsMix error handling.

## Feature Flags

- `mail`: Enable email helpers (pulls in `lettre`, `html-entities`, and `once_cell`).
- `vpn`: Enable shell-backed VPN automation utilities.
- `full`: Convenience flag for both.

```toml
[dependencies]
emixnet = { path = "../../crates/net", features = ["mail"] }
```

## Quick Example

```rust
use emixnet::web::reqwestx;

#[tokio::main]
async fn main() -> emixnet::Result<()> {
    let client = reqwestx::build_client_for_api()
        .user_agent("essentialmix/1.0")
        .build()?;

    let response = client
        .get("https://httpbin.org/json")
        .send()
        .await?
        .error_for_status()?;

    println!("status = {}", response.status());
    Ok(())
}
```

- Prefer `build_blocking_client*` when working in synchronous contexts.
- Combine with `emix::env` helpers to source credentials and timeouts.
- Explore the `web::mail` and `vpn` modules (feature-gated) for higher-level
  workflows.


