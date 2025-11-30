use crate::error::CryptoError;
use crate::Encrypt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherMode {
    Cbc,
    Ecb,
    Cfb,
    Ofb,
    Ctr,
    Gcm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddingMode {
    Pkcs7,
    NoPadding,
    ZeroPadding,
}

pub trait SymmetricAlgorithm: Encrypt {
    fn block_size(&self) -> usize;
    fn set_block_size(&mut self, size: usize) -> Result<(), CryptoError>;

    fn key_size(&self) -> usize;
    fn set_key_size(&mut self, size: usize) -> Result<(), CryptoError>;

    fn key(&self) -> &[u8];
    fn set_key(&mut self, key: &[u8]) -> Result<(), CryptoError>;

    fn iv(&self) -> Option<&[u8]>;
    fn set_iv(&mut self, iv: &[u8]) -> Result<(), CryptoError>;

    fn mode(&self) -> CipherMode;
    fn set_mode(&mut self, mode: CipherMode);

    fn padding(&self) -> PaddingMode;
    fn set_padding(&mut self, padding: PaddingMode);

    fn generate_key(&mut self) -> Result<(), CryptoError>;
    fn generate_iv(&mut self) -> Result<(), CryptoError>;
    fn generate_key_from_passphrase(&mut self, passphrase: &str, salt: Option<&[u8]>, iterations: u32) -> Result<(), CryptoError>;

    fn clear(&mut self);
    fn valid_key_size(&self, bit_length: usize) -> bool;
}

