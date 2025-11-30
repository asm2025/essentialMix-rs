#[cfg(feature = "hmac")]
use hmac::{Hmac, Mac};
#[cfg(feature = "hmac")]
use sha2::{Sha256, Sha512};
#[cfg(feature = "hmac")]
use zeroize::Zeroize;
#[cfg(feature = "hmac")]
use hex;
#[cfg(feature = "hmac")]
use crate::hash::traits::HashAlgorithm;
#[cfg(feature = "hmac")]
use crate::traits::{Algorithm, EncodingConfig};
#[cfg(feature = "hmac")]
use crate::error::{CryptoError, Result};

/// HMAC-SHA256
#[cfg(feature = "hmac")]
pub struct HmacSha256 {
    encoding: String,
    key: Vec<u8>,
}

#[cfg(feature = "hmac")]
impl HmacSha256 {
    pub fn new(key: &[u8]) -> Self {
        Self {
            encoding: "UTF-8".to_string(),
            key: key.to_vec(),
        }
    }
}

#[cfg(feature = "hmac")]
impl Algorithm for HmacSha256 {
    fn algorithm_name(&self) -> &str {
        "HMAC-SHA256"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(HmacSha256 {
            encoding: self.encoding.clone(),
            key: self.key.clone(),
        })
    }
}

#[cfg(feature = "hmac")]
impl EncodingConfig for HmacSha256 {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

#[cfg(feature = "hmac")]
impl HashAlgorithm for HmacSha256 {
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
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .map_err(|e| CryptoError::key(format!("Invalid key: {}", e)))?;
        mac.update(buffer);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn compute_hash_slice(&self, buffer: &[u8], offset: usize, count: usize) -> Result<Vec<u8>> {
        if offset + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.compute_hash_bytes(&buffer[offset..offset + count])
    }

    fn clear(&mut self) {
        self.key.zeroize();
    }
}

/// HMAC-SHA512
#[cfg(feature = "hmac")]
pub struct HmacSha512 {
    encoding: String,
    key: Vec<u8>,
}

#[cfg(feature = "hmac")]
impl HmacSha512 {
    pub fn new(key: &[u8]) -> Self {
        Self {
            encoding: "UTF-8".to_string(),
            key: key.to_vec(),
        }
    }
}

#[cfg(feature = "hmac")]
impl Algorithm for HmacSha512 {
    fn algorithm_name(&self) -> &str {
        "HMAC-SHA512"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(HmacSha512 {
            encoding: self.encoding.clone(),
            key: self.key.clone(),
        })
    }
}

#[cfg(feature = "hmac")]
impl EncodingConfig for HmacSha512 {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

#[cfg(feature = "hmac")]
impl HashAlgorithm for HmacSha512 {
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
        let mut mac = Hmac::<Sha512>::new_from_slice(&self.key)
            .map_err(|e| CryptoError::key(format!("Invalid key: {}", e)))?;
        mac.update(buffer);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn compute_hash_slice(&self, buffer: &[u8], offset: usize, count: usize) -> Result<Vec<u8>> {
        if offset + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.compute_hash_bytes(&buffer[offset..offset + count])
    }

    fn clear(&mut self) {
        self.key.zeroize();
    }
}

