use crate::error::CryptoError;

/// Base trait for all cryptographic algorithms
pub trait Algorithm: Send + Sync {
    /// Returns the name of the algorithm
    fn algorithm_name(&self) -> &str;

    /// Clones the algorithm instance
    fn clone_algorithm(&self) -> Box<dyn Algorithm>;
}

/// Trait for algorithms that support encoding configuration
pub trait EncodingConfig {
    fn encoding(&self) -> &str;
    fn set_encoding(&mut self, encoding: &str);
}

/// Trait for encryption/decryption operations
pub trait Encrypt: Algorithm + EncodingConfig {
    /// Encrypt a string, returning base64-encoded result
    fn encrypt_string(&self, value: &str) -> Result<String, CryptoError>;

    /// Encrypt bytes
    fn encrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Encrypt a slice of bytes
    fn encrypt_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<Vec<u8>, CryptoError>;

    /// Decrypt a base64-encoded string
    fn decrypt_string(&self, value: &str) -> Result<String, CryptoError>;

    /// Decrypt bytes
    fn decrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Decrypt a slice of bytes
    fn decrypt_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<Vec<u8>, CryptoError>;

    /// Generate a random string of specified length
    fn random_string(&self, length: usize) -> Result<String, CryptoError>;
}

