mod app;
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "imaging")]
pub mod imaging;
#[cfg(feature = "language")]
pub mod language;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "vision")]
pub mod vision;
pub use self::app::*;
pub mod date;
pub mod error;
pub mod io;
pub mod random;
pub mod string;
pub mod threading;
pub mod vpn;
pub mod web;

use std::sync::OnceLock;

use crate::error::*;

pub type Result<T> = std::result::Result<T, RmxError>;

static DEBUG: OnceLock<bool> = OnceLock::new();

pub fn set_debug(value: bool) {
    DEBUG.set(value).expect("Debug flag mode was already initialized.");
}

pub fn is_debug() -> bool {
    *DEBUG.get().unwrap_or(&false)
}

pub trait CallbackHandler<T> {
    fn starting(&self);
    fn update(&self, data: T);
    fn completed(&self);
}

pub mod ai {
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
}

pub mod system {
    pub fn num_cpus() -> usize {
        if crate::is_debug() {
            1
        } else {
            num_cpus::get()
        }
    }
}
