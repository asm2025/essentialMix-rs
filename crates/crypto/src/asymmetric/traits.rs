use crate::error::CryptoError;
use crate::Encrypt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RSAPadding {
    Pkcs1,
    Oaep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RSASignaturePadding {
    Pkcs1,
    Pss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

pub trait AsymmetricAlgorithm: Encrypt {
    fn key_size(&self) -> usize;
    fn set_key_size(&mut self, size: usize) -> Result<(), CryptoError>;

    fn from_xml_string(&mut self, xml: &str) -> Result<(), CryptoError>;
    fn to_xml_string(&self, include_private: bool) -> Result<String, CryptoError>;

    fn clear(&mut self);
}

pub trait RSAAlgorithm: AsymmetricAlgorithm {
    fn padding(&self) -> RSAPadding;
    fn set_padding(&mut self, padding: RSAPadding);

    fn signature_padding(&self) -> RSASignaturePadding;
    fn set_signature_padding(&mut self, padding: RSASignaturePadding);

    fn hash_algorithm(&self) -> HashAlgorithm;
    fn set_hash_algorithm(&mut self, algorithm: HashAlgorithm);
}

