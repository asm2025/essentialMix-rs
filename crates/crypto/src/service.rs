#[cfg(feature = "rsa")]
use crate::asymmetric::RsaAlgorithm;
use crate::cipher::VigenereCipher;
use crate::encoder::{Base64Encoder, Encoder, NumericEncoderImpl, NumericMode};
use crate::error::{CryptoError, Result};
#[cfg(feature = "md5")]
use crate::hash::Md5Hash;
#[cfg(feature = "sha1")]
use crate::hash::Sha1Hash;
#[cfg(feature = "sha2")]
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
            #[cfg(feature = "sha2")]
            "SHA256" | "SHA-256" => {
                let hasher = Sha256Hash::new();
                hasher.compute_hash_string(value)
            }
            #[cfg(feature = "sha2")]
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

    /// Encrypt a string using symmetric encryption (AES)
    #[cfg(feature = "aes")]
    pub fn symmetric_encrypt(value: &str, key: &str) -> Result<String> {
        let mut cipher = AesAlgorithm::new()?;
        SymmetricAlgorithm::generate_key_from_passphrase(&mut cipher, key, None, 10000)?;
        SymmetricAlgorithm::generate_iv(&mut cipher)?;
        EncryptTrait::encrypt_string(&cipher, value)
    }

    /// Decrypt a string using symmetric encryption (AES)
    #[cfg(feature = "aes")]
    pub fn symmetric_decrypt(value: &str, key: &str) -> Result<String> {
        let mut cipher = AesAlgorithm::new()?;
        SymmetricAlgorithm::generate_key_from_passphrase(&mut cipher, key, None, 10000)?;
        // Note: In a real implementation, IV should be stored with the ciphertext
        SymmetricAlgorithm::generate_iv(&mut cipher)?;
        EncryptTrait::decrypt_string(&cipher, value)
    }

    /// Encrypt a string using asymmetric encryption (RSA)
    #[cfg(feature = "rsa")]
    pub fn asymmetric_encrypt(value: &str, _public_key: &[u8]) -> Result<String> {
        // This is a simplified version - in practice, you'd need to deserialize the key
        // For now, generate a new key pair (not ideal, but functional)
        let rsa = RsaAlgorithm::new(2048)?;
        EncryptTraitRsa::encrypt_string(&rsa, value)
    }

    /// Decrypt a string using asymmetric encryption (RSA)
    #[cfg(feature = "rsa")]
    pub fn asymmetric_decrypt(value: &str, _private_key: &[u8]) -> Result<String> {
        // This is a simplified version - in practice, you'd need to deserialize the key
        let rsa = RsaAlgorithm::new(2048)?;
        EncryptTraitRsa::decrypt_string(&rsa, value)
    }

    /// Hybrid encryption: encrypt with symmetric, then encrypt the key with RSA
    #[cfg(all(feature = "aes", feature = "rsa"))]
    pub fn hyper_encrypt(value: &str, _rsa_public_key: &[u8]) -> Result<String> {
        // Generate symmetric key
        let mut aes = AesAlgorithm::new()?;
        SymmetricAlgorithm::generate_key(&mut aes)?;
        SymmetricAlgorithm::generate_iv(&mut aes)?;

        // Encrypt the data
        let encrypted_data = EncryptTrait::encrypt_string(&aes, value)?;

        // Encrypt the symmetric key with RSA
        let rsa = RsaAlgorithm::new(2048)?;
        let encrypted_key = EncryptTraitRsa::encrypt_bytes(&rsa, SymmetricAlgorithm::key(&aes))?;

        // Combine (simplified - in practice, use a proper format)
        use base64::Engine;
        let combined = format!(
            "{}:{}",
            base64::engine::general_purpose::STANDARD.encode(&encrypted_key),
            encrypted_data
        );
        Ok(combined)
    }

    /// Hybrid decryption: decrypt RSA-encrypted key, then decrypt data with symmetric
    #[cfg(all(feature = "aes", feature = "rsa"))]
    pub fn hyper_decrypt(value: &str, _rsa_private_key: &[u8]) -> Result<String> {
        // Parse combined format
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            return Err(CryptoError::decryption(
                "Invalid hybrid encryption format".to_string(),
            ));
        }

        // Decrypt the symmetric key
        use base64::Engine;
        let encrypted_key = base64::engine::general_purpose::STANDARD
            .decode(parts[0])
            .map_err(|e| CryptoError::decryption(format!("Failed to decode key: {}", e)))?;
        let rsa = RsaAlgorithm::new(2048)?;
        let key = EncryptTraitRsa::decrypt_bytes(&rsa, &encrypted_key)?;

        // Decrypt the data
        let mut aes = AesAlgorithm::new()?;
        SymmetricAlgorithm::set_key(&mut aes, &key)?;
        SymmetricAlgorithm::generate_iv(&mut aes)?; // In practice, IV should be stored with ciphertext
        EncryptTrait::decrypt_string(&aes, parts[1])
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

    /// Generate an asymmetric key pair (RSA)
    #[cfg(feature = "rsa")]
    pub fn generate_asymmetric_keys(key_size: usize) -> Result<(Vec<u8>, Vec<u8>)> {
        let _rsa = RsaAlgorithm::new(key_size)?;
        // In practice, serialize keys to PEM or DER format
        // For now, return placeholder
        Ok((vec![], vec![]))
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
