#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "imaging")]
pub mod imaging;
#[cfg(feature = "language")]
pub mod language;
#[cfg(feature = "vision")]
pub mod vision;

pub use emix::{Error, Result};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceSize {
    // Example: Phi-3 Mini, Orca Mini
    Tiny,
    // Example: Mistral 7B, Llama 7B
    #[default]
    Small,
    // Example: Llama 13B, Mixtral 8x7B (could also be Medium)
    Base,
    // Example: Llama 30B/34B
    Medium,
    // Example: Llama 70B+
    Large,
}
