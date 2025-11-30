use crate::error::CryptoError;
use crate::Algorithm;
use crate::EncodingConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericMode {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

pub trait Encoder: Algorithm + EncodingConfig {
    fn encode_string(&self, value: &str) -> Result<String, CryptoError>;
    fn encode_bytes(&self, buffer: &[u8]) -> Result<String, CryptoError>;
    fn encode_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<String, CryptoError>;

    fn decode_string(&self, value: &str) -> Result<String, CryptoError>;
    fn decode_to_bytes(&self, value: &str) -> Result<Vec<u8>, CryptoError>;
}

pub trait NumericEncoder: Encoder {
    fn mode(&self) -> NumericMode;
    fn set_mode(&mut self, mode: NumericMode);
    fn can_change(&self) -> bool;
}

