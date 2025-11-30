use sha2::{Sha256, Sha512, Digest};
#[cfg(feature = "sha1")]
use sha1::{Sha1, Digest as Sha1Digest};
use crate::hash::traits::HashAlgorithm;
use crate::traits::{Algorithm, EncodingConfig};
use crate::error::{CryptoError, Result};
use hex;

/// SHA-256 hash algorithm
pub struct Sha256Hash {
    encoding: String,
}

impl Sha256Hash {
    pub fn new() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
        }
    }
}

impl Default for Sha256Hash {
    fn default() -> Self {
        Self::new()
    }
}

impl Algorithm for Sha256Hash {
    fn algorithm_name(&self) -> &str {
        "SHA-256"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(Sha256Hash {
            encoding: self.encoding.clone(),
        })
    }
}

impl EncodingConfig for Sha256Hash {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

impl HashAlgorithm for Sha256Hash {
    fn hash_size(&self) -> usize {
        256 / 8 // 32 bytes
    }

    fn input_block_size(&self) -> usize {
        512 / 8 // 64 bytes
    }

    fn output_block_size(&self) -> usize {
        256 / 8 // 32 bytes
    }

    fn compute_hash_string(&self, value: &str) -> Result<String> {
        let bytes = value.as_bytes();
        let hash = self.compute_hash_bytes(bytes)?;
        Ok(hex::encode(hash))
    }

    fn compute_hash_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(buffer);
        Ok(hasher.finalize().to_vec())
    }

    fn compute_hash_slice(&self, buffer: &[u8], offset: usize, count: usize) -> Result<Vec<u8>> {
        if offset + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.compute_hash_bytes(&buffer[offset..offset + count])
    }

    fn clear(&mut self) {
        // No state to clear for stateless hash
    }
}

/// SHA-512 hash algorithm
pub struct Sha512Hash {
    encoding: String,
}

impl Sha512Hash {
    pub fn new() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
        }
    }
}

impl Default for Sha512Hash {
    fn default() -> Self {
        Self::new()
    }
}

impl Algorithm for Sha512Hash {
    fn algorithm_name(&self) -> &str {
        "SHA-512"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(Sha512Hash {
            encoding: self.encoding.clone(),
        })
    }
}

impl EncodingConfig for Sha512Hash {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

impl HashAlgorithm for Sha512Hash {
    fn hash_size(&self) -> usize {
        512 / 8 // 64 bytes
    }

    fn input_block_size(&self) -> usize {
        1024 / 8 // 128 bytes
    }

    fn output_block_size(&self) -> usize {
        512 / 8 // 64 bytes
    }

    fn compute_hash_string(&self, value: &str) -> Result<String> {
        let bytes = value.as_bytes();
        let hash = self.compute_hash_bytes(bytes)?;
        Ok(hex::encode(hash))
    }

    fn compute_hash_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Sha512::new();
        hasher.update(buffer);
        Ok(hasher.finalize().to_vec())
    }

    fn compute_hash_slice(&self, buffer: &[u8], offset: usize, count: usize) -> Result<Vec<u8>> {
        if offset + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.compute_hash_bytes(&buffer[offset..offset + count])
    }

    fn clear(&mut self) {
        // No state to clear for stateless hash
    }
}

/// SHA-1 hash algorithm (legacy)
#[cfg(feature = "sha1")]
pub struct Sha1Hash {
    encoding: String,
}

#[cfg(feature = "sha1")]
impl Sha1Hash {
    pub fn new() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
        }
    }
}

#[cfg(feature = "sha1")]
impl Default for Sha1Hash {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "sha1")]
impl Algorithm for Sha1Hash {
    fn algorithm_name(&self) -> &str {
        "SHA-1"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(Sha1Hash {
            encoding: self.encoding.clone(),
        })
    }
}

#[cfg(feature = "sha1")]
impl EncodingConfig for Sha1Hash {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

#[cfg(feature = "sha1")]
impl HashAlgorithm for Sha1Hash {
    fn hash_size(&self) -> usize {
        160 / 8 // 20 bytes
    }

    fn input_block_size(&self) -> usize {
        512 / 8 // 64 bytes
    }

    fn output_block_size(&self) -> usize {
        160 / 8 // 20 bytes
    }

    fn compute_hash_string(&self, value: &str) -> Result<String> {
        let bytes = value.as_bytes();
        let hash = self.compute_hash_bytes(bytes)?;
        Ok(hex::encode(hash))
    }

    fn compute_hash_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Sha1::new();
        hasher.update(buffer);
        Ok(hasher.finalize().to_vec())
    }

    fn compute_hash_slice(&self, buffer: &[u8], offset: usize, count: usize) -> Result<Vec<u8>> {
        if offset + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.compute_hash_bytes(&buffer[offset..offset + count])
    }

    fn clear(&mut self) {
        // No state to clear for stateless hash
    }
}

