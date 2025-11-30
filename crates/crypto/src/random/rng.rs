use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use crate::random::traits::RandomNumberGenerator;
use crate::traits::Algorithm;
use crate::error::{CryptoError, Result};

/// Cryptographically secure random number generator using ChaCha20
pub struct RngCryptoServiceProvider {
    rng: ChaCha20Rng,
}

impl RngCryptoServiceProvider {
    pub fn new() -> Result<Self> {
        let mut rng = ChaCha20Rng::from_entropy();
        Ok(Self { rng })
    }

    pub fn from_seed(seed: &[u8]) -> Result<Self> {
        if seed.len() < 32 {
            return Err(CryptoError::InvalidInput("Seed must be at least 32 bytes".to_string()));
        }
        let mut seed_array = [0u8; 32];
        seed_array[..seed.len().min(32)].copy_from_slice(&seed[..seed.len().min(32)]);
        let rng = ChaCha20Rng::from_seed(seed_array);
        Ok(Self { rng })
    }
}

impl Algorithm for RngCryptoServiceProvider {
    fn algorithm_name(&self) -> &str {
        "ChaCha20RNG"
    }

    fn clone_algorithm(&self) -> Box<dyn Algorithm> {
        Box::new(RngCryptoServiceProvider::new().unwrap())
    }
}

impl RandomNumberGenerator for RngCryptoServiceProvider {
    fn get_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        self.rng.fill_bytes(buffer);
        Ok(())
    }

    fn get_bytes_slice(&mut self, buffer: &mut [u8], offset: usize, count: usize) -> Result<()> {
        if offset + count > buffer.len() {
            return Err(CryptoError::InvalidInput("Slice out of bounds".to_string()));
        }
        self.rng.fill_bytes(&mut buffer[offset..offset + count]);
        Ok(())
    }

    fn get_non_zero_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        loop {
            self.rng.fill_bytes(buffer);
            if buffer.iter().all(|&b| b != 0) {
                break;
            }
        }
        Ok(())
    }

    fn next_double(&mut self) -> Result<f64> {
        Ok(self.rng.next_u64() as f64 / u64::MAX as f64)
    }

    fn next(&mut self) -> Result<u32> {
        Ok(self.rng.next_u32())
    }

    fn next_range(&mut self, min: u32, max: u32) -> Result<u32> {
        if min >= max {
            return Err(CryptoError::InvalidInput("min must be less than max".to_string()));
        }
        Ok(self.rng.next_u32() % (max - min) + min)
    }

    fn get_unique_values(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; length];
        self.get_bytes(&mut buffer)?;
        Ok(buffer)
    }
}

