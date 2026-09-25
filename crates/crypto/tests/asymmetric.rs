#![cfg(feature = "rsa")]

use emixcrypto::asymmetric::{AsymmetricAlgorithm, RSAAlgorithm, RSAPadding, RsaAlgorithm};
use emixcrypto::Encrypt;

// Small keys keep debug-build key generation fast; not for production use
const TEST_KEY_BITS: usize = 1024;

#[test]
fn test_rsa_pkcs1_round_trip() {
    let rsa = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    assert_eq!(rsa.key_size(), TEST_KEY_BITS);
    let encrypted = rsa.encrypt_string("hello rsa").unwrap();
    assert_eq!(rsa.decrypt_string(&encrypted).unwrap(), "hello rsa");
}

#[cfg(feature = "sha2")]
#[test]
fn test_rsa_oaep_round_trip() {
    let mut rsa = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    rsa.set_padding(RSAPadding::Oaep);
    let encrypted = rsa.encrypt_bytes(b"hello oaep").unwrap();
    assert_eq!(encrypted.len(), TEST_KEY_BITS / 8);
    assert_eq!(rsa.decrypt_bytes(&encrypted).unwrap(), b"hello oaep");
}

#[test]
fn test_rsa_encryption_is_randomized() {
    let rsa = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    let a = rsa.encrypt_bytes(b"same").unwrap();
    let b = rsa.encrypt_bytes(b"same").unwrap();
    assert_ne!(a, b);
}

#[test]
fn test_rsa_public_key_only_cannot_decrypt() {
    let full = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    let encrypted = full.encrypt_bytes(b"data").unwrap();

    // Build a public-only instance from a fresh private key's public half
    let private = rsa::RsaPrivateKey::new(&mut rand_core::OsRng, TEST_KEY_BITS).unwrap();
    let public_only = RsaAlgorithm::from_public_key(rsa::RsaPublicKey::from(&private));
    assert!(public_only.encrypt_bytes(b"data").is_ok());
    assert!(public_only.decrypt_bytes(&encrypted).is_err());
}

#[test]
fn test_rsa_from_private_key() {
    let private = rsa::RsaPrivateKey::new(&mut rand_core::OsRng, TEST_KEY_BITS).unwrap();
    let rsa = RsaAlgorithm::from_private_key(private);
    assert_eq!(rsa.key_size(), TEST_KEY_BITS);
    let encrypted = rsa.encrypt_bytes(b"abc").unwrap();
    assert_eq!(rsa.decrypt_bytes(&encrypted).unwrap(), b"abc");
}

#[test]
fn test_rsa_message_too_long() {
    let rsa = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    assert!(rsa.encrypt_bytes(&[0u8; TEST_KEY_BITS / 8]).is_err());
}

#[test]
fn test_rsa_set_key_size_bounds() {
    let mut rsa = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    assert!(rsa.set_key_size(256).is_err());
    assert!(rsa.set_key_size(8192).is_err());
}

#[test]
fn test_rsa_clear_removes_keys() {
    let mut rsa = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    AsymmetricAlgorithm::clear(&mut rsa);
    assert!(rsa.encrypt_bytes(b"x").is_err());
    assert!(rsa.decrypt_bytes(&[0u8; TEST_KEY_BITS / 8]).is_err());
}
