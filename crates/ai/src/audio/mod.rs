mod whisper;
pub use whisper::*;

#[cfg(all(feature = "audio", feature = "language"))]
mod openai;
#[cfg(all(feature = "audio", feature = "language"))]
pub use openai::*;
