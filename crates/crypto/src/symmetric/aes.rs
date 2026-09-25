#[cfg(feature = "aes")]
use zeroize::Zeroize;
use crate::symmetric::traits::{SymmetricAlgorithm, CipherMode, PaddingMode};
use crate::traits::{Algorithm, EncodingConfig, Encrypt};
use crate::error::{CryptoError, Result};
use crate::random::traits::RandomNumberGenerator;
use crate::random::rng::RngCryptoServiceProvider;
#[cfg(feature = "pbkdf2")]
use pbkdf2::pbkdf2_hmac;
#[cfg(feature = "sha2")]
use sha2::Sha256;
#[cfg(all(feature = "aes", feature = "cbc"))]
use cbc::cipher::{
    BlockModeDecrypt, BlockModeEncrypt, KeyIvInit,
    block_padding::{NoPadding, Pkcs7, ZeroPadding},
};

/// Encrypts `$data` with AES-CBC using the given AES variant and padding mode.
#[cfg(all(feature = "aes", feature = "cbc"))]
macro_rules! cbc_encrypt {
    ($cipher:ty, $key:expr, $iv:expr, $data:expr, $padding:expr) => {{
        let enc = cbc::Encryptor::<$cipher>::new_from_slices($key, $iv)
            .map_err(|e| CryptoError::key(format!("Invalid key or IV length: {}", e)))?;
        match $padding {
            PaddingMode::Pkcs7 => enc.encrypt_padded_vec::<Pkcs7>($data),
            PaddingMode::ZeroPadding => enc.encrypt_padded_vec::<ZeroPadding>($data),
            PaddingMode::NoPadding => enc.encrypt_padded_vec::<NoPadding>($data),
        }
    }};
}

/// Decrypts `$data` with AES-CBC using the given AES variant and padding mode.
#[cfg(all(feature = "aes", feature = "cbc"))]
macro_rules! cbc_decrypt {
    ($cipher:ty, $key:expr, $iv:expr, $data:expr, $padding:expr) => {{
        let dec = cbc::Decryptor::<$cipher>::new_from_slices($key, $iv)
            .map_err(|e| CryptoError::key(format!("Invalid key or IV length: {}", e)))?;
        match $padding {
            PaddingMode::Pkcs7 => dec.decrypt_padded_vec::<Pkcs7>($data),
            PaddingMode::ZeroPadding => dec.decrypt_padded_vec::<ZeroPadding>($data),
            PaddingMode::NoPadding => dec.decrypt_padded_vec::<NoPadding>($data),
        }
        .map_err(|_| CryptoError::Padding("Invalid padding".to_string()))?
    }};
}

/// AES symmetric encryption implementation
#[cfg(feature = "aes")]
pub struct AesAlgorithm {
    encoding: String,
    key: Vec<u8>,
    iv: Option<Vec<u8>>,
    key_size: usize,  // In bits
    block_size: usize, // In bits
    mode: CipherMode,
    padding: PaddingMode,
}

#[cfg(feature = "aes")]
impl AesAlgorithm {
    pub fn new() -> Result<Self> {
        let mut rng = RngCryptoServiceProvider::new()?;
        let mut key = vec![0u8; 32]; // 256 bits
        RandomNumberGenerator::get_bytes(&mut rng, &mut key)?;
        let mut iv = vec![0u8; 16]; // 128 bits
        RandomNumberGenerator::get_bytes(&mut rng, &mut iv)?;

        Ok(Self {
            encoding: "UTF-8".to_string(),
            key,
            iv: Some(iv),
            key_size: 256,
            block_size: 128,
            mode: CipherMode::Cbc,
            padding: PaddingMode::Pkcs7,
        })
    }

    /// Validates the key against the configured key size and returns the IV.
    fn key_and_iv(&self) -> Result<&[u8]> {
        if !self.valid_key_size(self.key.len() * 8) || self.key.len() * 8 != self.key_size {
            return Err(CryptoError::InvalidKeySize { expected: self.key_size, actual: self.key.len() * 8 });
        }

        let iv = self.iv.as_deref().ok_or_else(|| CryptoError::NotInitialized("IV not set".to_string()))?;
        if iv.len() != 16 {
            return Err(CryptoError::InvalidIvSize { expected: 16, actual: iv.len() });
        }

        Ok(iv)
    }
}

#[cfg(feature = "aes")]
impl Algorithm for AesAlgorithm {
    fn algorithm_name(&self) -> &str {
        "AES"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(AesAlgorithm {
            encoding: self.encoding.clone(),
            key: self.key.clone(),
            iv: self.iv.clone(),
            key_size: self.key_size,
            block_size: self.block_size,
            mode: self.mode,
            padding: self.padding,
        })
    }
}

#[cfg(feature = "aes")]
impl EncodingConfig for AesAlgorithm {
    fn encoding(&self) -> &str {
        &self.encoding
    }

    fn set_encoding(&mut self, encoding: &str) {
        self.encoding = encoding.to_string();
    }
}

#[cfg(feature = "aes")]
impl Encrypt for AesAlgorithm {
    fn encrypt_string(&self, value: &str) -> Result<String> {
        use base64::Engine;
        let bytes = value.as_bytes();
        let encrypted = self.encrypt_bytes(bytes)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&encrypted))
    }

    fn encrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let iv = self.key_and_iv()?;
        // NoPadding panics inside the cipher on partial blocks, so reject them up front
        if self.padding == PaddingMode::NoPadding && buffer.len() % 16 != 0 {
            return Err(CryptoError::Padding("Data length must be multiple of block size".to_string()));
        }

        match self.mode {
            #[cfg(feature = "cbc")]
            CipherMode::Cbc => Ok(match self.key.len() {
                16 => cbc_encrypt!(aes::Aes128, &self.key, iv, buffer, self.padding),
                24 => cbc_encrypt!(aes::Aes192, &self.key, iv, buffer, self.padding),
                _ => cbc_encrypt!(aes::Aes256, &self.key, iv, buffer, self.padding),
            }),
            _ => Err(CryptoError::UnsupportedCipherMode(format!("{:?}", self.mode))),
        }
    }

    fn encrypt_slice(&self, buffer: &[u8], start: usize, count: usize) -> Result<Vec<u8>> {
        if start + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.encrypt_bytes(&buffer[start..start + count])
    }

    fn decrypt_string(&self, value: &str) -> Result<String> {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD.decode(value)
            .map_err(|e| CryptoError::decryption(format!("Failed to decode base64: {}", e)))?;
        let decrypted = self.decrypt_bytes(&bytes)?;
        String::from_utf8(decrypted)
            .map_err(|e| CryptoError::decryption(format!("Invalid UTF-8: {}", e)))
    }

    fn decrypt_bytes(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let iv = self.key_and_iv()?;
        if buffer.len() % 16 != 0 {
            return Err(CryptoError::decryption("Ciphertext length must be multiple of block size".to_string()));
        }

        match self.mode {
            #[cfg(feature = "cbc")]
            CipherMode::Cbc => Ok(match self.key.len() {
                16 => cbc_decrypt!(aes::Aes128, &self.key, iv, buffer, self.padding),
                24 => cbc_decrypt!(aes::Aes192, &self.key, iv, buffer, self.padding),
                _ => cbc_decrypt!(aes::Aes256, &self.key, iv, buffer, self.padding),
            }),
            _ => Err(CryptoError::UnsupportedCipherMode(format!("{:?}", self.mode))),
        }
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

#[cfg(feature = "aes")]
impl SymmetricAlgorithm for AesAlgorithm {
    fn block_size(&self) -> usize {
        self.block_size
    }

    fn set_block_size(&mut self, size: usize) -> Result<()> {
        if size != 128 {
            return Err(CryptoError::InvalidBlockSize("AES only supports 128-bit blocks".to_string()));
        }
        self.block_size = size;
        Ok(())
    }

    fn key_size(&self) -> usize {
        self.key_size
    }

    fn set_key_size(&mut self, size: usize) -> Result<()> {
        if !self.valid_key_size(size) {
            return Err(CryptoError::InvalidKeySize { expected: 128, actual: size });
        }
        self.key_size = size;
        // Adjust key length
        let key_len = size / 8;
        if self.key.len() != key_len {
            let mut rng = RngCryptoServiceProvider::new()?;
            self.key.resize(key_len, 0);
            rng.get_bytes(&mut self.key)?;
        }
        Ok(())
    }

    fn key(&self) -> &[u8] {
        &self.key
    }

    fn set_key(&mut self, key: &[u8]) -> Result<()> {
        let key_len = self.key_size / 8;
        if key.len() != key_len {
            return Err(CryptoError::InvalidKeySize { expected: self.key_size, actual: key.len() * 8 });
        }
        self.key = key.to_vec();
        Ok(())
    }

    fn iv(&self) -> Option<&[u8]> {
        self.iv.as_deref()
    }

    fn set_iv(&mut self, iv: &[u8]) -> Result<()> {
        if iv.len() != 16 {
            return Err(CryptoError::InvalidIvSize { expected: 16, actual: iv.len() });
        }
        self.iv = Some(iv.to_vec());
        Ok(())
    }

    fn mode(&self) -> CipherMode {
        self.mode
    }

    fn set_mode(&mut self, mode: CipherMode) {
        self.mode = mode;
    }

    fn padding(&self) -> PaddingMode {
        self.padding
    }

    fn set_padding(&mut self, padding: PaddingMode) {
        self.padding = padding;
    }

    fn generate_key(&mut self) -> Result<()> {
        let mut rng = RngCryptoServiceProvider::new()?;
        let key_len = self.key_size / 8;
        self.key.resize(key_len, 0);
        RandomNumberGenerator::get_bytes(&mut rng, &mut self.key)?;
        Ok(())
    }

    fn generate_iv(&mut self) -> Result<()> {
        let mut rng = RngCryptoServiceProvider::new()?;
        let mut iv = vec![0u8; 16];
        RandomNumberGenerator::get_bytes(&mut rng, &mut iv)?;
        self.iv = Some(iv);
        Ok(())
    }

    fn generate_key_from_passphrase(&mut self, passphrase: &str, salt: Option<&[u8]>, iterations: u32) -> Result<()> {
        #[cfg(feature = "pbkdf2")]
        {
            let salt = salt.unwrap_or(&[0u8; 8]);
            let key_len = self.key_size / 8;
            let mut key = vec![0u8; key_len];
            #[cfg(feature = "sha2")]
            pbkdf2_hmac::<Sha256>(passphrase.as_bytes(), salt, iterations, &mut key);
            self.key = key;
            Ok(())
        }
        #[cfg(not(feature = "pbkdf2"))]
        {
            Err(CryptoError::key("PBKDF2 feature not enabled".to_string()))
        }
    }

    fn clear(&mut self) {
        self.key.zeroize();
        if let Some(ref mut iv) = self.iv {
            iv.zeroize();
        }
    }

    fn valid_key_size(&self, bit_length: usize) -> bool {
        matches!(bit_length, 128 | 192 | 256)
    }
}

