use crate::error::CryptoError;
use crate::Algorithm;
use crate::EncodingConfig;

pub trait HashAlgorithm: Algorithm + EncodingConfig {
    fn hash_size(&self) -> usize;
    fn input_block_size(&self) -> usize;
    fn output_block_size(&self) -> usize;

    fn compute_hash_string(&self, value: &str) -> Result<String, CryptoError>;
    fn compute_hash_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn compute_hash_slice(&self, buffer: &[u8], offset: usize, count: usize) -> Result<Vec<u8>, CryptoError>;

    fn clear(&mut self);
}

