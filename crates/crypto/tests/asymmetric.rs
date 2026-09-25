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

#[test]
fn test_rsa_der_export_import_round_trip() {
    let original = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    let public = RsaAlgorithm::from_public_key_bytes(&original.public_key_der().unwrap()).unwrap();
    let private = RsaAlgorithm::from_private_key_bytes(&original.private_key_der().unwrap()).unwrap();

    let encrypted = public.encrypt_bytes(b"der keys").unwrap();
    assert_eq!(private.decrypt_bytes(&encrypted).unwrap(), b"der keys");
    assert_eq!(private.public_key_der().unwrap(), original.public_key_der().unwrap());
}

#[test]
fn test_rsa_pem_export_import_round_trip() {
    let original = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    let public_pem = original.public_key_pem().unwrap();
    let private_pem = original.private_key_pem().unwrap();
    assert!(public_pem.starts_with("-----BEGIN PUBLIC KEY-----"));
    assert!(private_pem.starts_with("-----BEGIN PRIVATE KEY-----"));

    let public = RsaAlgorithm::from_public_key_bytes(public_pem.as_bytes()).unwrap();
    let private = RsaAlgorithm::from_private_key_bytes(private_pem.as_bytes()).unwrap();
    let encrypted = public.encrypt_bytes(b"pem keys").unwrap();
    assert_eq!(private.decrypt_bytes(&encrypted).unwrap(), b"pem keys");
}

#[test]
fn test_rsa_pkcs1_pem_import() {
    use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
    let key = rsa::RsaPrivateKey::new(&mut rand_core::OsRng, TEST_KEY_BITS).unwrap();
    let private_pem = key.to_pkcs1_pem(LineEnding::LF).unwrap();
    let public_pem = rsa::RsaPublicKey::from(&key).to_pkcs1_pem(LineEnding::LF).unwrap();

    let public = RsaAlgorithm::from_public_key_bytes(public_pem.as_bytes()).unwrap();
    let private = RsaAlgorithm::from_private_key_bytes(private_pem.as_bytes()).unwrap();
    let encrypted = public.encrypt_bytes(b"pkcs1").unwrap();
    assert_eq!(private.decrypt_bytes(&encrypted).unwrap(), b"pkcs1");
}

#[test]
fn test_rsa_invalid_key_bytes() {
    assert!(RsaAlgorithm::from_public_key_bytes(b"not a key").is_err());
    assert!(RsaAlgorithm::from_private_key_bytes(b"-----BEGIN PRIVATE KEY-----\ngarbage\n-----END PRIVATE KEY-----").is_err());
}

#[test]
fn test_rsa_public_only_cannot_export_private() {
    let original = RsaAlgorithm::new(TEST_KEY_BITS).unwrap();
    let public = RsaAlgorithm::from_public_key_bytes(&original.public_key_der().unwrap()).unwrap();
    assert!(public.private_key_der().is_err());
    assert!(public.private_key_pem().is_err());
}

mod quick_cipher {
    use super::TEST_KEY_BITS;
    use emixcrypto::QuickCipher;

    #[test]
    fn test_generate_asymmetric_keys_returns_der() {
        let (public, private) = QuickCipher::generate_asymmetric_keys(TEST_KEY_BITS).unwrap();
        assert!(!public.is_empty());
        assert!(!private.is_empty());
        assert_ne!(public, private);
    }

    #[test]
    fn test_asymmetric_round_trip() {
        let (public, private) = QuickCipher::generate_asymmetric_keys(TEST_KEY_BITS).unwrap();
        let encrypted = QuickCipher::asymmetric_encrypt("hello rsa", &public).unwrap();
        assert_eq!(QuickCipher::asymmetric_decrypt(&encrypted, &private).unwrap(), "hello rsa");
    }

    #[test]
    fn test_asymmetric_wrong_private_key_fails() {
        let (public, _) = QuickCipher::generate_asymmetric_keys(TEST_KEY_BITS).unwrap();
        let (_, other_private) = QuickCipher::generate_asymmetric_keys(TEST_KEY_BITS).unwrap();
        let encrypted = QuickCipher::asymmetric_encrypt("hello rsa", &public).unwrap();
        assert!(QuickCipher::asymmetric_decrypt(&encrypted, &other_private).is_err());
    }

    #[test]
    fn test_asymmetric_rejects_bad_key() {
        assert!(QuickCipher::asymmetric_encrypt("x", b"bad").is_err());
        assert!(QuickCipher::asymmetric_decrypt("AAAA", b"bad").is_err());
    }

    #[cfg(all(feature = "aes", feature = "cbc"))]
    #[test]
    fn test_hyper_round_trip_long_message() {
        let (public, private) = QuickCipher::generate_asymmetric_keys(TEST_KEY_BITS).unwrap();
        // Far longer than a single RSA block can hold
        let message = "hybrid ".repeat(500);
        let encrypted = QuickCipher::hyper_encrypt(&message, &public).unwrap();
        assert_eq!(encrypted.matches(':').count(), 1);
        assert_eq!(QuickCipher::hyper_decrypt(&encrypted, &private).unwrap(), message);
    }

    #[cfg(all(feature = "aes", feature = "cbc"))]
    #[test]
    fn test_hyper_wrong_key_and_bad_format() {
        let (public, _) = QuickCipher::generate_asymmetric_keys(TEST_KEY_BITS).unwrap();
        let (_, other_private) = QuickCipher::generate_asymmetric_keys(TEST_KEY_BITS).unwrap();
        let encrypted = QuickCipher::hyper_encrypt("secret", &public).unwrap();
        assert!(QuickCipher::hyper_decrypt(&encrypted, &other_private).is_err());
        assert!(QuickCipher::hyper_decrypt("no-separator", &other_private).is_err());
    }
}
