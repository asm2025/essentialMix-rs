use emixcore::Error as CoreError;
use thiserror::Error;

/// Cryptographic-specific error types
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Key error: {0}")]
    Key(String),

    #[error("Invalid key size: expected {expected} bits, got {actual} bits")]
    InvalidKeySize { expected: usize, actual: usize },

    #[error("Invalid block size: {0}")]
    InvalidBlockSize(String),

    #[error("Invalid IV size: expected {expected} bytes, got {actual} bytes")]
    InvalidIvSize { expected: usize, actual: usize },

    #[error("Encoding error: {0}")]
    Encoding(String),

    #[error("Decoding error: {0}")]
    Decoding(String),

    #[error("Hash error: {0}")]
    Hash(String),

    #[error("Random number generation error: {0}")]
    Random(String),

    #[error("Padding error: {0}")]
    Padding(String),

    #[error("Cipher mode not supported: {0}")]
    UnsupportedCipherMode(String),

    #[error("Padding mode not supported: {0}")]
    UnsupportedPaddingMode(String),

    #[error("RSA padding not supported: {0}")]
    UnsupportedRsaPadding(String),

    #[error("XML key format error: {0}")]
    XmlKeyFormat(String),

    #[error("Algorithm not initialized: {0}")]
    NotInitialized(String),

    #[error("Invalid input data: {0}")]
    InvalidInput(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

impl From<CryptoError> for CoreError {
    fn from(err: CryptoError) -> Self {
        match err {
            CryptoError::Encryption(msg) => CoreError::Other(format!("Encryption error: {}", msg)),
            CryptoError::Decryption(msg) => CoreError::Other(format!("Decryption error: {}", msg)),
            CryptoError::Key(msg) => CoreError::InvalidInput(format!("Key error: {}", msg)),
            CryptoError::InvalidKeySize { expected, actual } => CoreError::InvalidInput(format!(
                "Invalid key size: expected {} bits, got {} bits",
                expected, actual
            )),
            CryptoError::InvalidBlockSize(msg) => {
                CoreError::InvalidInput(format!("Invalid block size: {}", msg))
            }
            CryptoError::InvalidIvSize { expected, actual } => CoreError::InvalidInput(format!(
                "Invalid IV size: expected {} bytes, got {} bytes",
                expected, actual
            )),
            CryptoError::Encoding(msg) => CoreError::Other(format!("Encoding error: {}", msg)),
            CryptoError::Decoding(msg) => CoreError::Other(format!("Decoding error: {}", msg)),
            CryptoError::Hash(msg) => CoreError::Other(format!("Hash error: {}", msg)),
            CryptoError::Random(msg) => {
                CoreError::Other(format!("Random number generation error: {}", msg))
            }
            CryptoError::Padding(msg) => CoreError::Other(format!("Padding error: {}", msg)),
            CryptoError::UnsupportedCipherMode(_msg) => CoreError::NotSupported,
            CryptoError::UnsupportedPaddingMode(_msg) => CoreError::NotSupported,
            CryptoError::UnsupportedRsaPadding(_msg) => CoreError::NotSupported,
            CryptoError::XmlKeyFormat(msg) => {
                CoreError::Parse(format!("XML key format error: {}", msg))
            }
            CryptoError::NotInitialized(msg) => {
                CoreError::InvalidOperation(format!("Algorithm not initialized: {}", msg))
            }
            CryptoError::InvalidInput(msg) => CoreError::InvalidInput(msg),
            CryptoError::OperationFailed(msg) => {
                CoreError::Other(format!("Operation failed: {}", msg))
            }
        }
    }
}

/// Result type alias for cryptographic operations
pub type Result<T> = std::result::Result<T, CryptoError>;

/// Helper functions for creating crypto-specific errors
impl CryptoError {
    pub fn encryption(msg: impl Into<String>) -> Self {
        CryptoError::Encryption(msg.into())
    }

    pub fn decryption(msg: impl Into<String>) -> Self {
        CryptoError::Decryption(msg.into())
    }

    pub fn key(msg: impl Into<String>) -> Self {
        CryptoError::Key(msg.into())
    }

    pub fn encoding(msg: impl Into<String>) -> Self {
        CryptoError::Encoding(msg.into())
    }

    pub fn decoding(msg: impl Into<String>) -> Self {
        CryptoError::Decoding(msg.into())
    }

    pub fn hash(msg: impl Into<String>) -> Self {
        CryptoError::Hash(msg.into())
    }

    pub fn random(msg: impl Into<String>) -> Self {
        CryptoError::Random(msg.into())
    }
}
