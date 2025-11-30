use crate::traits::{Algorithm, EncodingConfig, Encrypt};
use crate::error::{CryptoError, Result};

/// Vigenère cipher implementation
pub struct VigenereCipher {
    encoding: String,
    key: String,
}

impl VigenereCipher {
    pub fn new(key: &str) -> Self {
        Self {
            encoding: "UTF-8".to_string(),
            key: key.to_string(),
        }
    }

    fn encrypt_char(c: char, key_char: char, is_uppercase: bool) -> char {
        let base = if is_uppercase { b'A' } else { b'a' } as u32;
        let c_val = c as u32 - base;
        let k_val = (key_char.to_ascii_uppercase() as u32 - b'A' as u32) % 26;
        let encrypted = (c_val + k_val) % 26;
        char::from_u32(base + encrypted).unwrap_or(c)
    }

    fn decrypt_char(c: char, key_char: char, is_uppercase: bool) -> char {
        let base = if is_uppercase { b'A' } else { b'a' } as u32;
        let c_val = c as u32 - base;
        let k_val = (key_char.to_ascii_uppercase() as u32 - b'A' as u32) % 26;
        let decrypted = (c_val + 26 - k_val) % 26;
        char::from_u32(base + decrypted).unwrap_or(c)
    }
}

impl Algorithm for VigenereCipher {
    fn algorithm_name(&self) -> &str {
        "Vigenère"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(VigenereCipher {
            encoding: self.encoding.clone(),
            key: self.key.clone(),
        })
    }
}

impl EncodingConfig for VigenereCipher {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

impl Encrypt for VigenereCipher {
    fn encrypt_string(&self, value: &str) -> Result<String> {
        if self.key.is_empty() {
            return Err(CryptoError::key("Key cannot be empty".to_string()));
        }

        let key_chars: Vec<char> = self.key.chars().collect();
        let mut key_index = 0;
        let mut result = String::new();

        for c in value.chars() {
            if c.is_alphabetic() {
                let is_uppercase = c.is_uppercase();
                let key_char = key_chars[key_index % key_chars.len()];
                result.push(Self::encrypt_char(c, key_char, is_uppercase));
                key_index += 1;
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }

    fn encrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let text = String::from_utf8(buffer.to_vec())
            .map_err(|e| CryptoError::encryption(format!("Invalid UTF-8: {}", e)))?;
        let encrypted = self.encrypt_string(&text)?;
        Ok(encrypted.into_bytes())
    }

    fn encrypt_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<Vec<u8>> {
        if start + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.encrypt_bytes(&buffer[start..start + count])
    }

    fn decrypt_string(&self, value: &str) -> Result<String> {
        if self.key.is_empty() {
            return Err(CryptoError::key("Key cannot be empty".to_string()));
        }

        let key_chars: Vec<char> = self.key.chars().collect();
        let mut key_index = 0;
        let mut result = String::new();

        for c in value.chars() {
            if c.is_alphabetic() {
                let is_uppercase = c.is_uppercase();
                let key_char = key_chars[key_index % key_chars.len()];
                result.push(Self::decrypt_char(c, key_char, is_uppercase));
                key_index += 1;
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }

    fn decrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let text = String::from_utf8(buffer.to_vec())
            .map_err(|e| CryptoError::decryption(format!("Invalid UTF-8: {}", e)))?;
        let decrypted = self.decrypt_string(&text)?;
        Ok(decrypted.into_bytes())
    }

    fn decrypt_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<Vec<u8>> {
        if start + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.decrypt_bytes(&buffer[start..start + count])
    }

    fn random_string(&self, length: usize) -> Result<String> {
        use base64::Engine;
        use crate::random::traits::RandomNumberGenerator;
        use crate::random::rng::RngCryptoServiceProvider;
        let mut rng = RngCryptoServiceProvider::new()?;
        let mut bytes = vec![0u8; length];
        RandomNumberGenerator::get_bytes(&mut rng, &mut bytes)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }
}

