use base64::{Engine as _, engine::general_purpose};
use crate::encoder::traits::Encoder;
use crate::traits::{Algorithm, EncodingConfig};
use crate::error::{CryptoError, Result};

/// Base64 encoder/decoder
pub struct Base64Encoder {
    encoding: String,
}

impl Base64Encoder {
    pub fn new() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
        }
    }
}

impl Default for Base64Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Algorithm for Base64Encoder {
    fn algorithm_name(&self) -> &str {
        "Base64"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(Base64Encoder {
            encoding: self.encoding.clone(),
        })
    }
}

impl EncodingConfig for Base64Encoder {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

impl Encoder for Base64Encoder {
    fn encode_string(&self, value: &str) -> Result<String> {
        let bytes = value.as_bytes();
        Ok(general_purpose::STANDARD.encode(bytes))
    }

    fn encode_bytes(&self, buffer: &[u8]) -> Result<String> {
        Ok(general_purpose::STANDARD.encode(buffer))
    }

    fn encode_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<String> {
        if start + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        Ok(general_purpose::STANDARD.encode(&buffer[start..start + count]))
    }

    fn decode_string(&self, value: &str) -> Result<String> {
        let bytes = general_purpose::STANDARD.decode(value)
            .map_err(|e| CryptoError::decoding(format!("Failed to decode base64: {}", e)))?;
        String::from_utf8(bytes)
            .map_err(|e| CryptoError::decoding(format!("Invalid UTF-8: {}", e)))
    }

    fn decode_to_bytes(&self, value: &str) -> Result<Vec<u8>> {
        general_purpose::STANDARD.decode(value)
            .map_err(|e| CryptoError::decoding(format!("Failed to decode base64: {}", e)))
    }
}

