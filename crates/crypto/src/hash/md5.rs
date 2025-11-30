#[cfg(feature = "md5")]
use md5::{Md5, Digest};
#[cfg(feature = "md5")]
use hex;
#[cfg(feature = "md5")]
use crate::hash::traits::HashAlgorithm;
#[cfg(feature = "md5")]
use crate::traits::{Algorithm, EncodingConfig};
#[cfg(feature = "md5")]
use crate::error::{CryptoError, Result};

/// MD5 hash algorithm (legacy, use SHA-256 or SHA-512 for new code)
#[cfg(feature = "md5")]
pub struct Md5Hash {
    encoding: String,
}

#[cfg(feature = "md5")]
impl Md5Hash {
    pub fn new() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
        }
    }
}

#[cfg(feature = "md5")]
impl Default for Md5Hash {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "md5")]
impl Algorithm for Md5Hash {
    fn algorithm_name(&self) -> &str {
        "MD5"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(Md5Hash {
            encoding: self.encoding.clone(),
        })
    }
}

#[cfg(feature = "md5")]
impl EncodingConfig for Md5Hash {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

#[cfg(feature = "md5")]
impl HashAlgorithm for Md5Hash {
    fn hash_size(&self) -> usize {
        128 / 8 // 16 bytes
    }

    fn input_block_size(&self) -> usize {
        512 / 8 // 64 bytes
    }

    fn output_block_size(&self) -> usize {
        128 / 8 // 16 bytes
    }

    fn compute_hash_string(&self, value: &str) -> Result<String> {
        let bytes = value.as_bytes();
        let hash = self.compute_hash_bytes(bytes)?;
        Ok(hex::encode(hash))
    }

    fn compute_hash_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Md5::new();
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

