//! # emixcrypto
//!
//! Cryptographic utilities for EssentialMix, providing encryption, hashing, encoding,
//! and random number generation capabilities.
//!
//! ## Features
//!
//! - **Symmetric Encryption**: AES with multiple cipher modes (CBC, CTR, GCM)
//! - **Asymmetric Encryption**: RSA with PKCS1 and OAEP padding
//! - **Hash Algorithms**: SHA (1, 256, 384, 512), MD5, HMAC variants, Adler32
//! - **Encoders**: Base64, Numeric (Binary, Octal, Decimal, Hexadecimal)
//! - **Random Number Generation**: Cryptographically secure RNG
//! - **Special Ciphers**: Vigenère cipher
//!
//! ## Quick Example
//!
//! ```rust
//! use emixcrypto::QuickCipher;
//!
//! // Hash a string
//! let hash = QuickCipher::hash("Hello, World!", "SHA256")?;
//!
//! // Base64 encode
//! let encoded = QuickCipher::base64_encode("Hello, World!")?;
//!
//! // Encrypt with symmetric encryption
//! let encrypted = QuickCipher::symmetric_encrypt("secret message", "my-key")?;
//! ```

pub mod asymmetric;
mod cipher;
mod encoder;
mod error;
mod hash;
mod random;
mod service;
pub mod settings;
pub mod symmetric;
pub mod traits;

#[cfg(feature = "rsa")]
pub use asymmetric::*;
pub use cipher::VigenereCipher;
pub use encoder::{Base64Encoder, Encoder, NumericEncoder, NumericEncoderImpl, NumericMode};
pub use error::{CryptoError, Result as CryptoResult};
#[cfg(feature = "md5")]
pub use hash::Md5Hash;
#[cfg(feature = "sha1")]
pub use hash::Sha1Hash;
pub use hash::{HashAlgorithm, Sha256Hash, Sha512Hash};
#[cfg(feature = "hmac")]
pub use hash::{HmacSha256, HmacSha512};
pub use random::{RandomNumberGenerator, RngCryptoServiceProvider};
pub use service::QuickCipher;
pub use settings::*;
#[cfg(feature = "aes")]
pub use symmetric::*;
pub use traits::{Algorithm, EncodingConfig, Encrypt};

pub use emixcore::{Error, Result};
