use crate::symmetric::CipherMode;
use crate::symmetric::PaddingMode;
use crate::asymmetric::RSAPadding;
use crate::asymmetric::RSASignaturePadding;
use crate::asymmetric::HashAlgorithm;
use crate::encoder::NumericMode;

/// Common settings for cryptographic operations
#[derive(Debug, Clone)]
pub struct Settings {
    pub encoding: String,
    pub salt_size: u16,
    pub rfc2898_iterations: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
            salt_size: 0,
            rfc2898_iterations: 0,
        }
    }
}

/// Settings for symmetric encryption algorithms
#[derive(Debug, Clone)]
pub struct SymmetricSettings {
    pub settings: Settings,
    pub key_size: usize,        // In bits
    pub block_size: usize,       // In bits
    pub mode: CipherMode,
    pub padding: PaddingMode,
    pub use_expiration: bool,
    pub expiration: Option<std::time::SystemTime>,
}

impl Default for SymmetricSettings {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            key_size: 256,
            block_size: 128,
            mode: CipherMode::Cbc,
            padding: PaddingMode::Pkcs7,
            use_expiration: false,
            expiration: None,
        }
    }
}

/// Settings for RSA asymmetric encryption
#[derive(Debug, Clone)]
pub struct RSASettings {
    pub settings: Settings,
    pub key_size: usize,        // In bits
    pub padding: RSAPadding,
    pub signature_padding: RSASignaturePadding,
    pub hash_algorithm: HashAlgorithm,
    pub use_expiration: bool,
    pub expiration: Option<std::time::SystemTime>,
}

impl Default for RSASettings {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            key_size: 2048,
            padding: RSAPadding::Pkcs1,
            signature_padding: RSASignaturePadding::Pkcs1,
            hash_algorithm: HashAlgorithm::Sha1,
            use_expiration: false,
            expiration: None,
        }
    }
}

/// Settings for hybrid encryption (symmetric + asymmetric)
#[derive(Debug, Clone)]
pub struct HyperSettings {
    pub symmetric: SymmetricSettings,
    pub rsa: RSASettings,
}

impl Default for HyperSettings {
    fn default() -> Self {
        Self {
            symmetric: SymmetricSettings::default(),
            rsa: RSASettings::default(),
        }
    }
}

/// Settings for numeric encoding
#[derive(Debug, Clone)]
pub struct NumericSettings {
    pub settings: Settings,
    pub mode: NumericMode,
}

impl Default for NumericSettings {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            mode: NumericMode::Hexadecimal,
        }
    }
}

