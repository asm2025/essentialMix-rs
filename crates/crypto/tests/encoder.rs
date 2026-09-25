use emixcrypto::{Base64Encoder, Encoder, Encrypt, NumericEncoderImpl, NumericMode, QuickCipher, VigenereCipher};

#[test]
fn test_base64_known_vector() {
    let encoder = Base64Encoder::new();
    assert_eq!(encoder.encode_string("Hello").unwrap(), "SGVsbG8=");
    assert_eq!(encoder.decode_string("SGVsbG8=").unwrap(), "Hello");
}

#[test]
fn test_base64_invalid_input() {
    let encoder = Base64Encoder::new();
    assert!(encoder.decode_string("not base64!").is_err());
}

#[test]
fn test_quick_cipher_base64_round_trip() {
    let encoded = QuickCipher::base64_encode("Hello, World!").unwrap();
    assert_eq!(QuickCipher::base64_decode(&encoded).unwrap(), "Hello, World!");
}

#[test]
fn test_numeric_encode_known_values() {
    let cases = [
        (NumericMode::Binary, "01001000 01101001"),
        (NumericMode::Octal, "110 151"),
        (NumericMode::Decimal, "72 105"),
        (NumericMode::Hexadecimal, "48 69"),
    ];
    for (mode, expected) in cases {
        let encoder = NumericEncoderImpl::new(mode);
        assert_eq!(encoder.encode_string("Hi").unwrap(), expected, "{mode:?}");
        assert_eq!(encoder.decode_string(expected).unwrap(), "Hi", "{mode:?}");
    }
}

#[test]
fn test_numeric_round_trip_all_bytes() {
    let bytes: Vec<u8> = (0..=255).collect();
    for mode in [NumericMode::Binary, NumericMode::Octal, NumericMode::Decimal, NumericMode::Hexadecimal] {
        let encoder = NumericEncoderImpl::new(mode);
        let encoded = encoder.encode_bytes(&bytes).unwrap();
        assert_eq!(encoder.decode_to_bytes(&encoded).unwrap(), bytes, "{mode:?}");
    }
}

#[test]
fn test_numeric_decode_invalid() {
    assert!(NumericEncoderImpl::new(NumericMode::Binary).decode_string("102").is_err());
    assert!(NumericEncoderImpl::new(NumericMode::Decimal).decode_string("256").is_err());
    assert!(NumericEncoderImpl::new(NumericMode::Hexadecimal).decode_string("zz").is_err());
}

#[test]
fn test_quick_cipher_numeric_round_trip() {
    let encoded = QuickCipher::numeric_encode("abc", NumericMode::Hexadecimal).unwrap();
    assert_eq!(encoded, "61 62 63");
    assert_eq!(QuickCipher::numeric_decode(&encoded, NumericMode::Hexadecimal).unwrap(), "abc");
}

#[test]
fn test_vigenere_known_vector() {
    let cipher = VigenereCipher::new("LEMON");
    assert_eq!(cipher.encrypt_string("ATTACKATDAWN").unwrap(), "LXFOPVEFRNHR");
    assert_eq!(cipher.decrypt_string("LXFOPVEFRNHR").unwrap(), "ATTACKATDAWN");
}

#[test]
fn test_vigenere_preserves_case_and_non_letters() {
    let encrypted = QuickCipher::vigenere_encrypt("Attack at dawn!", "lemon").unwrap();
    assert_eq!(encrypted, "Lxfopv ef rnhr!");
    assert_eq!(QuickCipher::vigenere_decrypt(&encrypted, "lemon").unwrap(), "Attack at dawn!");
}

#[test]
fn test_vigenere_empty_key() {
    let cipher = VigenereCipher::new("");
    assert!(cipher.encrypt_string("abc").is_err());
    assert!(cipher.decrypt_string("abc").is_err());
}
