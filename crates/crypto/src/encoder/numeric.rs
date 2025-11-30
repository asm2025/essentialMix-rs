use crate::encoder::traits::{Encoder, NumericEncoder, NumericMode};
use crate::traits::{Algorithm, EncodingConfig};
use crate::error::{CryptoError, Result};

/// Numeric encoder supporting Binary, Octal, Decimal, and Hexadecimal
pub struct NumericEncoderImpl {
    encoding: String,
    mode: NumericMode,
}

impl NumericEncoderImpl {
    pub fn new(mode: NumericMode) -> Self {
        Self {
            encoding: "UTF-8".to_string(),
            mode,
        }
    }
}

impl Algorithm for NumericEncoderImpl {
    fn algorithm_name(&self) -> &str {
        match self.mode {
            NumericMode::Binary => "Numeric-Binary",
            NumericMode::Octal => "Numeric-Octal",
            NumericMode::Decimal => "Numeric-Decimal",
            NumericMode::Hexadecimal => "Numeric-Hexadecimal",
        }
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(NumericEncoderImpl {
            encoding: self.encoding.clone(),
            mode: self.mode,
        })
    }
}

impl EncodingConfig for NumericEncoderImpl {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

impl Encoder for NumericEncoderImpl {
    fn encode_string(&self, value: &str) -> Result<String> {
        let bytes = value.as_bytes();
        self.encode_bytes(bytes)
    }

    fn encode_bytes(&self, buffer: &[u8]) -> Result<String> {
        match self.mode {
            NumericMode::Binary => {
                Ok(buffer.iter()
                    .map(|b| format!("{:08b}", b))
                    .collect::<Vec<_>>()
                    .join(" "))
            }
            NumericMode::Octal => {
                Ok(buffer.iter()
                    .map(|b| format!("{:03o}", b))
                    .collect::<Vec<_>>()
                    .join(" "))
            }
            NumericMode::Decimal => {
                Ok(buffer.iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(" "))
            }
            NumericMode::Hexadecimal => {
                Ok(buffer.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" "))
            }
        }
    }

    fn encode_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<String> {
        if start + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.encode_bytes(&buffer[start..start + count])
    }

    fn decode_string(&self, value: &str) -> Result<String> {
        let bytes = self.decode_to_bytes(value)?;
        String::from_utf8(bytes)
            .map_err(|e| CryptoError::decoding(format!("Invalid UTF-8: {}", e)))
    }

    fn decode_to_bytes(&self, value: &str) -> Result<Vec<u8>> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let mut bytes = Vec::new();

        for part in parts {
            let byte = match self.mode {
                NumericMode::Binary => {
                    u8::from_str_radix(part, 2)
                        .map_err(|_| CryptoError::decoding(format!("Invalid binary: {}", part)))?
                }
                NumericMode::Octal => {
                    u8::from_str_radix(part, 8)
                        .map_err(|_| CryptoError::decoding(format!("Invalid octal: {}", part)))?
                }
                NumericMode::Decimal => {
                    part.parse::<u8>()
                        .map_err(|_| CryptoError::decoding(format!("Invalid decimal: {}", part)))?
                }
                NumericMode::Hexadecimal => {
                    u8::from_str_radix(part, 16)
                        .map_err(|_| CryptoError::decoding(format!("Invalid hexadecimal: {}", part)))?
                }
            };
            bytes.push(byte);
        }

        Ok(bytes)
    }
}

impl NumericEncoder for NumericEncoderImpl {
    fn mode(&self) -> NumericMode {
        self.mode
    }

    fn set_mode(&mut self, mode: NumericMode) {
        self.mode = mode;
    }

    fn can_change(&self) -> bool {
        true
    }
}

