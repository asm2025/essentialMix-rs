use crate::asymmetric::traits::{
    AsymmetricAlgorithm, HashAlgorithm as HashAlg, RSAAlgorithm, RSAPadding, RSASignaturePadding,
};
use crate::error::{CryptoError, Result};
use crate::random::rng::RngCryptoServiceProvider;
use crate::random::traits::RandomNumberGenerator;
use crate::traits::{Algorithm, EncodingConfig, Encrypt};
#[cfg(feature = "rsa")]
use rsa::{Oaep, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
#[cfg(feature = "rsa")]
#[cfg(feature = "sha2")]
use sha2::Sha256;

/// RSA asymmetric encryption implementation
#[cfg(feature = "rsa")]
pub struct RsaAlgorithm {
    encoding: String,
    private_key: Option<RsaPrivateKey>,
    public_key: Option<RsaPublicKey>,
    key_size: usize,
    padding: RSAPadding,
    signature_padding: RSASignaturePadding,
    hash_algorithm: HashAlg,
}

#[cfg(feature = "rsa")]
impl RsaAlgorithm {
    pub fn new(key_size: usize) -> Result<Self> {
        use rand_core::OsRng;
        let private_key = RsaPrivateKey::new(&mut OsRng, key_size)
            .map_err(|e| CryptoError::key(format!("Failed to generate RSA key: {}", e)))?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            encoding: "UTF-8".to_string(),
            private_key: Some(private_key),
            public_key: Some(public_key),
            key_size,
            padding: RSAPadding::Pkcs1,
            signature_padding: RSASignaturePadding::Pkcs1,
            hash_algorithm: HashAlg::Sha1,
        })
    }

    pub fn from_private_key(private_key: RsaPrivateKey) -> Self {
        use rsa::traits::PublicKeyParts;
        let public_key = RsaPublicKey::from(&private_key);
        let key_size = private_key.size() * 8;
        Self {
            encoding: "UTF-8".to_string(),
            private_key: Some(private_key),
            public_key: Some(public_key),
            key_size,
            padding: RSAPadding::Pkcs1,
            signature_padding: RSASignaturePadding::Pkcs1,
            hash_algorithm: HashAlg::Sha1,
        }
    }

    pub fn from_public_key(public_key: RsaPublicKey) -> Self {
        use rsa::traits::PublicKeyParts;
        let key_size = public_key.size() * 8;
        Self {
            encoding: "UTF-8".to_string(),
            private_key: None,
            public_key: Some(public_key),
            key_size,
            padding: RSAPadding::Pkcs1,
            signature_padding: RSASignaturePadding::Pkcs1,
            hash_algorithm: HashAlg::Sha1,
        }
    }
}

#[cfg(feature = "rsa")]
impl Algorithm for RsaAlgorithm {
    fn algorithm_name(&self) -> &str {
        "RSA"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        // Cannot clone RSA keys easily, return a new instance
        Box::new(RsaAlgorithm {
            encoding: self.encoding.clone(),
            private_key: None,
            public_key: None,
            key_size: self.key_size,
            padding: self.padding,
            signature_padding: self.signature_padding,
            hash_algorithm: self.hash_algorithm,
        })
    }
}

#[cfg(feature = "rsa")]
impl EncodingConfig for RsaAlgorithm {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

#[cfg(feature = "rsa")]
impl Encrypt for RsaAlgorithm {
    fn encrypt_string(&self, value: &str) -> Result<String> {
        use base64::Engine;
        let bytes = value.as_bytes();
        let encrypted = self.encrypt_bytes(bytes)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&encrypted))
    }

    fn encrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let public_key = self
            .public_key
            .as_ref()
            .ok_or_else(|| CryptoError::NotInitialized("Public key not set".to_string()))?;

        use rand_core::OsRng;
        let result = match self.padding {
            RSAPadding::Pkcs1 => {
                let padding = Pkcs1v15Encrypt;
                public_key.encrypt(&mut OsRng, padding, buffer)
            }
            RSAPadding::Oaep => {
                #[cfg(feature = "sha2")]
                {
                    let padding = Oaep::new::<Sha256>();
                    public_key.encrypt(&mut OsRng, padding, buffer)
                }
                #[cfg(not(feature = "sha2"))]
                {
                    return Err(CryptoError::encryption(
                        "OAEP requires sha2 feature".to_string(),
                    ));
                }
            }
        };
        result.map_err(|e| CryptoError::encryption(format!("RSA encryption failed: {}", e)))
    }

    fn encrypt_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<Vec<u8>> {
        if start + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.encrypt_bytes(&buffer[start..start + count])
    }

    fn decrypt_string(&self, value: &str) -> Result<String> {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(value)
            .map_err(|e| CryptoError::decryption(format!("Failed to decode base64: {}", e)))?;
        let decrypted = self.decrypt_bytes(&bytes)?;
        String::from_utf8(decrypted)
            .map_err(|e| CryptoError::decryption(format!("Invalid UTF-8: {}", e)))
    }

    fn decrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let private_key = self
            .private_key
            .as_ref()
            .ok_or_else(|| CryptoError::NotInitialized("Private key not set".to_string()))?;

        let result = match self.padding {
            RSAPadding::Pkcs1 => {
                let padding = Pkcs1v15Encrypt;
                private_key.decrypt(padding, buffer)
            }
            RSAPadding::Oaep => {
                #[cfg(feature = "sha2")]
                {
                    let padding = Oaep::new::<Sha256>();
                    private_key.decrypt(padding, buffer)
                }
                #[cfg(not(feature = "sha2"))]
                {
                    return Err(CryptoError::decryption(
                        "OAEP requires sha2 feature".to_string(),
                    ));
                }
            }
        };
        result.map_err(|e| CryptoError::decryption(format!("RSA decryption failed: {}", e)))
    }

    fn decrypt_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<Vec<u8>> {
        if start + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.decrypt_bytes(&buffer[start..start + count])
    }

    fn random_string(&self, length: usize) -> Result<String> {
        use base64::Engine;
        let mut rng = RngCryptoServiceProvider::new()?;
        let mut bytes = vec![0u8; length];
        RandomNumberGenerator::get_bytes(&mut rng, &mut bytes)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }
}

#[cfg(feature = "rsa")]
impl AsymmetricAlgorithm for RsaAlgorithm {
    fn key_size(&self) -> usize {
        self.key_size
    }

    fn set_key_size(&mut self, size: usize) -> Result<()> {
        if size < 512 || size > 4096 {
            return Err(CryptoError::InvalidKeySize {
                expected: 2048,
                actual: size,
            });
        }
        self.key_size = size;
        // Generate new keys
        use rand_core::OsRng;
        let private_key = RsaPrivateKey::new(&mut OsRng, size)
            .map_err(|e| CryptoError::key(format!("Failed to generate RSA key: {}", e)))?;
        let public_key = RsaPublicKey::from(&private_key);
        self.private_key = Some(private_key);
        self.public_key = Some(public_key);
        Ok(())
    }

    fn from_xml_string(&mut self, _xml: &str) -> Result<()> {
        // XML key format parsing would go here
        // For now, return an error indicating it's not implemented
        Err(CryptoError::XmlKeyFormat(
            "XML key format parsing not yet implemented".to_string(),
        ))
    }

    fn to_xml_string(&self, _include_private: bool) -> Result<String> {
        // XML key format serialization would go here
        Err(CryptoError::XmlKeyFormat(
            "XML key format serialization not yet implemented".to_string(),
        ))
    }

    fn clear(&mut self) {
        // RSA keys don't implement Zeroize directly, but we can drop them
        self.private_key = None;
        self.public_key = None;
    }
}

#[cfg(feature = "rsa")]
impl RSAAlgorithm for RsaAlgorithm {
    fn padding(&self) -> RSAPadding {
        self.padding
    }

    fn set_padding(&mut self, padding: RSAPadding) {
        self.padding = padding;
    }

    fn signature_padding(&self) -> RSASignaturePadding {
        self.signature_padding
    }

    fn set_signature_padding(&mut self, padding: RSASignaturePadding) {
        self.signature_padding = padding;
    }

    fn hash_algorithm(&self) -> HashAlg {
        self.hash_algorithm
    }

    fn set_hash_algorithm(&mut self, algorithm: HashAlg) {
        self.hash_algorithm = algorithm;
    }
}
