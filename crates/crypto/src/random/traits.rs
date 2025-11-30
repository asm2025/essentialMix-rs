use crate::error::CryptoError;
use crate::Algorithm;

pub trait RandomNumberGenerator: Algorithm {
    fn get_bytes(&mut self, buffer: &mut [u8]) -> Result<(), CryptoError>;
    fn get_bytes_slice(&mut self, buffer: &mut [u8], offset: usize, count: usize) -> Result<(), CryptoError>;
    fn get_non_zero_bytes(&mut self, buffer: &mut [u8]) -> Result<(), CryptoError>;

    fn next_double(&mut self) -> Result<f64, CryptoError>;
    fn next(&mut self) -> Result<u32, CryptoError>;
    fn next_range(&mut self, min: u32, max: u32) -> Result<u32, CryptoError>;
    fn get_unique_values(&mut self, length: usize) -> Result<Vec<u8>, CryptoError>;
}

