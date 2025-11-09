# emixai

`emixai` bundles feature-gated helpers for audio, imaging, language, and vision
workloads on top of the EssentialMix core libraries. Each module stays optional
so you only pull in the tooling your project needs.

## Features

- `language` (default): OpenAI and Kalosm-backed chat helpers, prompt flows, and
  streaming responses.
- `audio`: Whisper-based transcription utilities and playback helpers.
- `imaging`: Image generation and manipulation helpers.
- `vision`: Shared computer-vision helpers.
- `full`: Convenience flag that enables every module.
- Hardware flags (`cuda`, `metal`, `mkl`) align with Kalosm’s acceleration
  backends.

Enable the crate with the features you need in your `Cargo.toml`:

```toml
[dependencies]
emixai = { path = "../../crates/ai", features = ["language"] }
tokio = { version = "1", features = ["full"] }
config = "0.14"
```

## Quick Example (ChatGPT Stream)

```rust
use config::{Config, Environment};
use emixai::language::openai::{ChatGpt, OpenAiSource};
use emixai::SourceSize;

#[tokio::main]
async fn main() -> emixai::Result<()> {
    // Expect OPENAI__API_KEY in the environment.
    let settings = Config::builder()
        .add_source(Environment::with_prefix("OPENAI"))
        .build()?;

    // Use the default stream buffer size and a small model footprint.
    let chat = ChatGpt::from_size(settings, SourceSize::Small, None);
    chat.prompt("Describe EssentialMix in one sentence.").await?;

    Ok(())
}
```

- Swap `SourceSize` at runtime to move between inference cost tiers.
- Prefer `ChatGpt::subscribe()` to stream deltas into your own channel.
- See `tests/` for more end-to-end usage covering audio transcription,
  Whisper, Wuerstchen imaging, and LLaMA language flows.


