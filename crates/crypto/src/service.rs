#[cfg(feature = "rsa")]
use crate::asymmetric::{RSAAlgorithm, RSAPadding, RsaAlgorithm};
use crate::cipher::VigenereCipher;
use crate::encoder::{Base64Encoder, Encoder, NumericEncoderImpl, NumericMode};
use crate::error::{CryptoError, Result};
#[cfg(feature = "md5")]
use crate::hash::Md5Hash;
#[cfg(feature = "sha1")]
use crate::hash::Sha1Hash;
use crate::hash::{HashAlgorithm, Sha256Hash, Sha512Hash};
use crate::random::RngCryptoServiceProvider;
use crate::random::traits::RandomNumberGenerator;
#[cfg(feature = "aes")]
use crate::symmetric::{AesAlgorithm, SymmetricAlgorithm};
#[cfg(feature = "aes")]
use crate::traits::Encrypt as EncryptTrait;
#[cfg(feature = "rsa")]
use crate::traits::Encrypt as EncryptTraitRsa;
use crate::traits::Encrypt as EncryptTraitVigenere;

/// High-level cryptographic service providing convenient methods for common operations
pub struct QuickCipher;

impl QuickCipher {
    /// Compute a hash of the input string using the specified algorithm
    pub fn hash(value: &str, algorithm: &str) -> Result<String> {
        match algorithm.to_uppercase().as_str() {
            "SHA256" | "SHA-256" => {
                let hasher = Sha256Hash::new();
                hasher.compute_hash_string(value)
            }
            "SHA512" | "SHA-512" => {
                let hasher = Sha512Hash::new();
                hasher.compute_hash_string(value)
            }
            #[cfg(feature = "sha1")]
            "SHA1" | "SHA-1" => {
                let hasher = Sha1Hash::new();
                hasher.compute_hash_string(value)
            }
            #[cfg(feature = "md5")]
            "MD5" => {
                let hasher = Md5Hash::new();
                hasher.compute_hash_string(value)
            }
            _ => Err(CryptoError::hash(format!(
                "Unsupported hash algorithm: {}",
                algorithm
            ))),
        }
    }

    /// Base64 encode a string
    pub fn base64_encode(value: &str) -> Result<String> {
        let encoder = Base64Encoder::new();
        encoder.encode_string(value)
    }

    /// Base64 decode a string
    pub fn base64_decode(value: &str) -> Result<String> {
        let encoder = Base64Encoder::new();
        encoder.decode_string(value)
    }

    /// Numeric encode a string
    pub fn numeric_encode(value: &str, mode: NumericMode) -> Result<String> {
        let encoder = NumericEncoderImpl::new(mode);
        encoder.encode_string(value)
    }

    /// Numeric decode a string
    pub fn numeric_decode(value: &str, mode: NumericMode) -> Result<String> {
        let encoder = NumericEncoderImpl::new(mode);
        encoder.decode_string(value)
    }

    /// Encrypt a string using symmetric encryption (AES).
    ///
    /// Returns base64 of `IV || ciphertext` so [`QuickCipher::symmetric_decrypt`] can recover the IV.
    #[cfg(feature = "aes")]
    pub fn symmetric_encrypt(value: &str, key: &str) -> Result<String> {
        use base64::Engine;
        let mut cipher = AesAlgorithm::new()?;
        SymmetricAlgorithm::generate_key_from_passphrase(&mut cipher, key, None, 10000)?;
        let output = Self::aes_encrypt_with_iv(&mut cipher, value.as_bytes())?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&output))
    }

    /// Decrypt a string produced by [`QuickCipher::symmetric_encrypt`].
    #[cfg(feature = "aes")]
    pub fn symmetric_decrypt(value: &str, key: &str) -> Result<String> {
        let bytes = decode_base64(value)?;
        let mut cipher = AesAlgorithm::new()?;
        SymmetricAlgorithm::generate_key_from_passphrase(&mut cipher, key, None, 10000)?;
        let decrypted = Self::aes_decrypt_with_iv(&mut cipher, &bytes)?;
        into_utf8(decrypted)
    }

    /// Encrypt a string with an RSA public key (DER or PEM, SPKI or PKCS#1), using OAEP (SHA-256) padding.
    #[cfg(feature = "rsa")]
    pub fn asymmetric_encrypt(value: &str, public_key: &[u8]) -> Result<String> {
        let rsa = Self::rsa_oaep(RsaAlgorithm::from_public_key_bytes(public_key)?);
        EncryptTraitRsa::encrypt_string(&rsa, value)
    }

    /// Decrypt a string produced by [`QuickCipher::asymmetric_encrypt`] with the matching
    /// RSA private key (DER or PEM, PKCS#8 or PKCS#1).
    #[cfg(feature = "rsa")]
    pub fn asymmetric_decrypt(value: &str, private_key: &[u8]) -> Result<String> {
        let rsa = Self::rsa_oaep(RsaAlgorithm::from_private_key_bytes(private_key)?);
        EncryptTraitRsa::decrypt_string(&rsa, value)
    }

    /// Hybrid encryption: encrypt the data with a random AES-256 key, then encrypt that key with RSA.
    ///
    /// Output format: `base64(RSA-OAEP(aes_key)):base64(IV || AES-CBC ciphertext)`.
    /// Unlike [`QuickCipher::asymmetric_encrypt`], the message length is not limited by the RSA key size.
    #[cfg(all(feature = "aes", feature = "rsa"))]
    pub fn hyper_encrypt(value: &str, rsa_public_key: &[u8]) -> Result<String> {
        use base64::Engine;
        let rsa = Self::rsa_oaep(RsaAlgorithm::from_public_key_bytes(rsa_public_key)?);

        let mut aes = AesAlgorithm::new()?;
        SymmetricAlgorithm::generate_key(&mut aes)?;
        let encrypted_data = Self::aes_encrypt_with_iv(&mut aes, value.as_bytes())?;
        let encrypted_key = EncryptTraitRsa::encrypt_bytes(&rsa, SymmetricAlgorithm::key(&aes))?;
        SymmetricAlgorithm::clear(&mut aes);

        let engine = base64::engine::general_purpose::STANDARD;
        Ok(format!("{}:{}", engine.encode(&encrypted_key), engine.encode(&encrypted_data)))
    }

    /// Hybrid decryption of a value produced by [`QuickCipher::hyper_encrypt`].
    #[cfg(all(feature = "aes", feature = "rsa"))]
    pub fn hyper_decrypt(value: &str, rsa_private_key: &[u8]) -> Result<String> {
        let (key_part, data_part) = value
            .split_once(':')
            .ok_or_else(|| CryptoError::decryption("Invalid hybrid encryption format".to_string()))?;
        let rsa = Self::rsa_oaep(RsaAlgorithm::from_private_key_bytes(rsa_private_key)?);
        let key = EncryptTraitRsa::decrypt_bytes(&rsa, &decode_base64(key_part)?)?;

        let mut aes = AesAlgorithm::new()?;
        SymmetricAlgorithm::set_key(&mut aes, &key)?;
        let decrypted = Self::aes_decrypt_with_iv(&mut aes, &decode_base64(data_part)?);
        SymmetricAlgorithm::clear(&mut aes);
        into_utf8(decrypted?)
    }

    /// Generates a fresh IV, encrypts `data` and returns `IV || ciphertext`.
    #[cfg(feature = "aes")]
    fn aes_encrypt_with_iv(cipher: &mut AesAlgorithm, data: &[u8]) -> Result<Vec<u8>> {
        SymmetricAlgorithm::generate_iv(cipher)?;
        let mut output = SymmetricAlgorithm::iv(cipher).unwrap_or_default().to_vec();
        output.extend(EncryptTrait::encrypt_bytes(cipher, data)?);
        Ok(output)
    }

    /// Splits `IV || ciphertext` and decrypts it.
    #[cfg(feature = "aes")]
    fn aes_decrypt_with_iv(cipher: &mut AesAlgorithm, data: &[u8]) -> Result<Vec<u8>> {
        const IV_LEN: usize = 16;
        if data.len() < IV_LEN {
            return Err(CryptoError::decryption("Ciphertext is missing the IV".to_string()));
        }
        let (iv, ciphertext) = data.split_at(IV_LEN);
        SymmetricAlgorithm::set_iv(cipher, iv)?;
        EncryptTrait::decrypt_bytes(cipher, ciphertext)
    }

    #[cfg(feature = "rsa")]
    fn rsa_oaep(mut rsa: RsaAlgorithm) -> RsaAlgorithm {
        RSAAlgorithm::set_padding(&mut rsa, RSAPadding::Oaep);
        rsa
    }

    /// Generate a random string
    pub fn random_string(length: usize) -> Result<String> {
        use base64::Engine;
        let mut rng = RngCryptoServiceProvider::new()?;
        let mut bytes = vec![0u8; length];
        RandomNumberGenerator::get_bytes(&mut rng, &mut bytes)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }

    /// Generate a symmetric encryption key
    #[cfg(feature = "aes")]
    pub fn generate_symmetric_key(key_size: usize) -> Result<Vec<u8>> {
        let mut cipher = AesAlgorithm::new()?;
        SymmetricAlgorithm::set_key_size(&mut cipher, key_size)?;
        SymmetricAlgorithm::generate_key(&mut cipher)?;
        Ok(SymmetricAlgorithm::key(&cipher).to_vec())
    }

    /// Generate an RSA key pair, returned as `(public_key, private_key)` DER bytes
    /// (SPKI and PKCS#8 respectively), ready for the `asymmetric_*` and `hyper_*` methods.
    #[cfg(feature = "rsa")]
    pub fn generate_asymmetric_keys(key_size: usize) -> Result<(Vec<u8>, Vec<u8>)> {
        let rsa = RsaAlgorithm::new(key_size)?;
        Ok((rsa.public_key_der()?, rsa.private_key_der()?))
    }

    /// Encrypt using Vigenère cipher
    pub fn vigenere_encrypt(value: &str, key: &str) -> Result<String> {
        let cipher = VigenereCipher::new(key);
        EncryptTraitVigenere::encrypt_string(&cipher, value)
    }

    /// Decrypt using Vigenère cipher
    pub fn vigenere_decrypt(value: &str, key: &str) -> Result<String> {
        let cipher = VigenereCipher::new(key);
        EncryptTraitVigenere::decrypt_string(&cipher, value)
    }
}

#[cfg(any(feature = "aes", feature = "rsa"))]
fn decode_base64(value: &str) -> Result<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|e| CryptoError::decryption(format!("Failed to decode base64: {}", e)))
}

#[cfg(feature = "aes")]
fn into_utf8(bytes: Vec<u8>) -> Result<String> {
    String::from_utf8(bytes).map_err(|e| CryptoError::decryption(format!("Invalid UTF-8: {}", e)))
}
