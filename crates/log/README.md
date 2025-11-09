# emixlog

`emixlog` adds multi-sink logging utilities and adapters for popular Rust
logging ecosystems so you can fan out structured logs across multiple targets.

## Feature Flags

- `log4rs`: Enable integration helpers for the `log4rs` configuration-driven
  backend.
- `slog`: Enable adapters for `slog`.

```toml
[dependencies]
emixlog = { path = "../../crates/log", features = ["log4rs"] }
```

## Quick Example

```rust
use emixlog::{MultiLogger, LogLevel};
use log::{Log, Metadata, Record};

struct StdoutLogger;

impl Log for StdoutLogger {
    fn enabled(&self, _: &Metadata) -> bool { true }
    fn log(&self, record: &Record) { println!("{}", record.args()); }
    fn flush(&self) {}
}

MultiLogger::new()
    .add_logger(Box::new(StdoutLogger))
    .init_with_level(LogLevel::Info)
    .expect("logger already initialized");
```

- Add as many loggers as you need (file, stdout, log4rs appenders, slog drains).
- Use `try_init` variants when you want to ignore duplicate registration errors.
- `LOG_DATE_FORMAT`, `LOG_SIZE_MIN`, and `LOG_SIZE_MAX` centralize shared
  formatting constants for downstream crates.


