# emixthreading

`emixthreading` provides synchronization primitives, producer/consumer helpers,
spinners, and async blocking utilities that build on `emixcore::Error`.

## Modules at a Glance

- `cond`: Manual reset conditions and cross-thread notifications.
- `consumer`: Awaitable producer/consumer abstractions.
- `signal`: Cancellation-aware signals.
- `spinner`: Terminal spinners built on `indicatif`.
- `constants`: Shared timing constants for queues and waits.

```toml
[dependencies]
emixthreading = { path = "../../crates/threading" }
```

## Quick Examples

Manual spinner:

```rust
use emixthreading::Spinner;

let spinner = Spinner::with_prefix("sync ".into());
spinner.tick();
spinner.finish_with_message("done");
```

Timeout waiting for an async worker:

```rust
use emixthreading::{wait_for_async, TaskResult};
use std::sync::Arc;
use tokio::sync::Notify;

struct Worker {
    cancelled: bool,
    finished: bool,
}

impl emixthreading::AwaitableConsumer<TaskResult> for Worker {
    fn is_cancelled(&self) -> bool { self.cancelled }
    fn is_finished(&self) -> bool { self.finished }
}

// Wait up to 5 seconds for a completion signal.
wait_for_async(
    &Worker { cancelled: false, finished: true },
    std::time::Duration::from_secs(5),
    &Arc::new(Notify::new()),
).await.ok();
```

Browse the `tests/` directory for more advanced queue coordination patterns and
signal-handling examples.


