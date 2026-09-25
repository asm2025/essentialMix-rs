#![cfg(all(feature = "aes", feature = "cbc"))]

use emixcrypto::symmetric::{AesAlgorithm, CipherMode, PaddingMode, SymmetricAlgorithm};
use emixcrypto::{Encrypt, QuickCipher};

fn aes(key: &[u8], iv: &[u8]) -> AesAlgorithm {
    let mut cipher = AesAlgorithm::new().unwrap();
    cipher.set_key_size(key.len() * 8).unwrap();
    cipher.set_key(key).unwrap();
    cipher.set_iv(iv).unwrap();
    cipher
}

// NIST SP 800-38A, F.2.1 CBC-AES128.Encrypt (first block)
#[test]
fn test_aes128_cbc_nist_vector() {
    let key = hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap();
    let iv = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
    let plaintext = hex::decode("6bc1bee22e409f96e93d7e117393172a").unwrap();

    let mut cipher = aes(&key, &iv);
    cipher.set_padding(PaddingMode::NoPadding);

    let ciphertext = cipher.encrypt_bytes(&plaintext).unwrap();
    assert_eq!(hex::encode(&ciphertext), "7649abac8119b246cee98e9b12e9197d");
    assert_eq!(cipher.decrypt_bytes(&ciphertext).unwrap(), plaintext);
}

// NIST SP 800-38A, F.2.5 CBC-AES256.Encrypt (first block)
#[test]
fn test_aes256_cbc_nist_vector() {
    let key = hex::decode("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4").unwrap();
    let iv = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
    let plaintext = hex::decode("6bc1bee22e409f96e93d7e117393172a").unwrap();

    let mut cipher = aes(&key, &iv);
    cipher.set_padding(PaddingMode::NoPadding);

    let ciphertext = cipher.encrypt_bytes(&plaintext).unwrap();
    assert_eq!(hex::encode(&ciphertext), "f58c4c04d6e5f1ba779eabfb5f7bfbd6");
}

#[test]
fn test_round_trip_all_key_sizes() {
    let message = "The quick brown fox jumps over the lazy dog";
    for key_len in [16usize, 24, 32] {
        let cipher = aes(&vec![0x42; key_len], &[0x24; 16]);
        let encrypted = cipher.encrypt_string(message).unwrap();
        assert_eq!(cipher.decrypt_string(&encrypted).unwrap(), message, "key_len={key_len}");
    }
}

#[test]
fn test_pkcs7_always_adds_padding() {
    let cipher = aes(&[1u8; 32], &[2u8; 16]);
    assert_eq!(cipher.encrypt_bytes(&[]).unwrap().len(), 16);
    assert_eq!(cipher.encrypt_bytes(&[0u8; 15]).unwrap().len(), 16);
    assert_eq!(cipher.encrypt_bytes(&[0u8; 16]).unwrap().len(), 32);
}

#[test]
fn test_zero_padding_round_trip() {
    let mut cipher = aes(&[1u8; 32], &[2u8; 16]);
    cipher.set_padding(PaddingMode::ZeroPadding);
    let encrypted = cipher.encrypt_bytes(b"hello").unwrap();
    assert_eq!(encrypted.len(), 16);
    assert_eq!(cipher.decrypt_bytes(&encrypted).unwrap(), b"hello");
}

#[test]
fn test_no_padding_rejects_partial_block() {
    let mut cipher = aes(&[1u8; 32], &[2u8; 16]);
    cipher.set_padding(PaddingMode::NoPadding);
    assert!(cipher.encrypt_bytes(b"not a full block").is_ok()); // exactly 16 bytes
    assert!(cipher.encrypt_bytes(b"short").is_err());
}

#[test]
fn test_decrypt_rejects_bad_length() {
    let cipher = aes(&[1u8; 32], &[2u8; 16]);
    assert!(cipher.decrypt_bytes(&[0u8; 15]).is_err());
}

#[test]
fn test_decrypt_with_wrong_key_does_not_recover_plaintext() {
    let encrypted = aes(&[1u8; 32], &[2u8; 16]).encrypt_bytes(b"secret").unwrap();
    let result = aes(&[9u8; 32], &[2u8; 16]).decrypt_bytes(&encrypted);
    assert!(!matches!(result, Ok(ref p) if p == b"secret"));
}

#[test]
fn test_unsupported_mode() {
    let mut cipher = aes(&[1u8; 32], &[2u8; 16]);
    cipher.set_mode(CipherMode::Ecb);
    assert!(cipher.encrypt_bytes(b"data").is_err());
    assert!(cipher.decrypt_bytes(&[0u8; 16]).is_err());
}

#[test]
fn test_invalid_key_and_iv_sizes() {
    let mut cipher = AesAlgorithm::new().unwrap();
    assert!(cipher.set_key(&[0u8; 10]).is_err());
    assert!(cipher.set_iv(&[0u8; 8]).is_err());
    assert!(cipher.set_key_size(100).is_err());
    assert!(cipher.set_block_size(256).is_err());
}

#[test]
fn test_set_key_size_regenerates_key() {
    let mut cipher = AesAlgorithm::new().unwrap();
    assert_eq!(cipher.key().len(), 32);
    cipher.set_key_size(128).unwrap();
    assert_eq!(cipher.key_size(), 128);
    assert_eq!(cipher.key().len(), 16);
    let encrypted = cipher.encrypt_string("abc").unwrap();
    assert_eq!(cipher.decrypt_string(&encrypted).unwrap(), "abc");
}

#[cfg(feature = "pbkdf2")]
#[test]
fn test_passphrase_key_is_deterministic() {
    let mut a = AesAlgorithm::new().unwrap();
    let mut b = AesAlgorithm::new().unwrap();
    a.generate_key_from_passphrase("password", Some(b"salt1234"), 1000).unwrap();
    b.generate_key_from_passphrase("password", Some(b"salt1234"), 1000).unwrap();
    assert_eq!(a.key(), b.key());

    b.generate_key_from_passphrase("password", Some(b"other"), 1000).unwrap();
    assert_ne!(a.key(), b.key());
}

#[test]
fn test_clear_zeroizes_key() {
    let mut cipher = aes(&[5u8; 32], &[6u8; 16]);
    SymmetricAlgorithm::clear(&mut cipher);
    assert!(cipher.key().iter().all(|&b| b == 0));
    assert!(cipher.iv().unwrap().iter().all(|&b| b == 0));
}

#[cfg(feature = "pbkdf2")]
#[test]
fn test_quick_cipher_symmetric_round_trip() {
    let encrypted = QuickCipher::symmetric_encrypt("secret message", "my-key").unwrap();
    assert_eq!(QuickCipher::symmetric_decrypt(&encrypted, "my-key").unwrap(), "secret message");
}

#[cfg(feature = "pbkdf2")]
#[test]
fn test_quick_cipher_symmetric_uses_random_iv() {
    let a = QuickCipher::symmetric_encrypt("same", "key").unwrap();
    let b = QuickCipher::symmetric_encrypt("same", "key").unwrap();
    assert_ne!(a, b);
}

#[cfg(feature = "pbkdf2")]
#[test]
fn test_quick_cipher_symmetric_wrong_key() {
    let encrypted = QuickCipher::symmetric_encrypt("secret message", "right").unwrap();
    let result = QuickCipher::symmetric_decrypt(&encrypted, "wrong");
    assert!(!matches!(result, Ok(ref s) if s == "secret message"));
}

#[cfg(feature = "pbkdf2")]
#[test]
fn test_quick_cipher_symmetric_malformed_input() {
    assert!(QuickCipher::symmetric_decrypt("!!!", "key").is_err());
    assert!(QuickCipher::symmetric_decrypt("AAAA", "key").is_err()); // shorter than IV
}

#[test]
fn test_generate_symmetric_key() {
    assert_eq!(QuickCipher::generate_symmetric_key(128).unwrap().len(), 16);
    assert_eq!(QuickCipher::generate_symmetric_key(256).unwrap().len(), 32);
    assert!(QuickCipher::generate_symmetric_key(64).is_err());
}
