use emixcrypto::{QuickCipher, RandomNumberGenerator, RngCryptoServiceProvider};

#[test]
fn test_same_seed_is_deterministic() {
    let seed = [7u8; 32];
    let mut a = RngCryptoServiceProvider::from_seed(&seed).unwrap();
    let mut b = RngCryptoServiceProvider::from_seed(&seed).unwrap();

    let mut buf_a = [0u8; 64];
    let mut buf_b = [0u8; 64];
    a.get_bytes(&mut buf_a).unwrap();
    b.get_bytes(&mut buf_b).unwrap();
    assert_eq!(buf_a, buf_b);
    assert_eq!(a.next().unwrap(), b.next().unwrap());
}

#[test]
fn test_different_seeds_differ() {
    let mut a = RngCryptoServiceProvider::from_seed(&[1u8; 32]).unwrap();
    let mut b = RngCryptoServiceProvider::from_seed(&[2u8; 32]).unwrap();
    let mut buf_a = [0u8; 32];
    let mut buf_b = [0u8; 32];
    a.get_bytes(&mut buf_a).unwrap();
    b.get_bytes(&mut buf_b).unwrap();
    assert_ne!(buf_a, buf_b);
}

#[test]
fn test_short_seed_rejected() {
    assert!(RngCryptoServiceProvider::from_seed(&[0u8; 16]).is_err());
}

#[test]
fn test_get_bytes_fills_buffer() {
    let mut rng = RngCryptoServiceProvider::new().unwrap();
    let mut buf = [0u8; 256];
    rng.get_bytes(&mut buf).unwrap();
    // 256 zero bytes from a CSPRNG is practically impossible
    assert!(buf.iter().any(|&b| b != 0));
}

#[test]
fn test_get_bytes_slice_only_touches_range() {
    let mut rng = RngCryptoServiceProvider::from_seed(&[3u8; 32]).unwrap();
    let mut buf = [0u8; 32];
    rng.get_bytes_slice(&mut buf, 8, 16).unwrap();
    assert!(buf[..8].iter().all(|&b| b == 0));
    assert!(buf[24..].iter().all(|&b| b == 0));
    assert!(rng.get_bytes_slice(&mut buf, 20, 16).is_err());
}

#[test]
fn test_get_non_zero_bytes() {
    let mut rng = RngCryptoServiceProvider::new().unwrap();
    let mut buf = [0u8; 16];
    rng.get_non_zero_bytes(&mut buf).unwrap();
    assert!(buf.iter().all(|&b| b != 0));
}

#[test]
fn test_next_double_in_unit_interval() {
    let mut rng = RngCryptoServiceProvider::new().unwrap();
    for _ in 0..1000 {
        let v = rng.next_double().unwrap();
        assert!((0.0..=1.0).contains(&v));
    }
}

#[test]
fn test_next_range_bounds() {
    let mut rng = RngCryptoServiceProvider::new().unwrap();
    for _ in 0..1000 {
        let v = rng.next_range(10, 20).unwrap();
        assert!((10..20).contains(&v));
    }
    assert!(rng.next_range(5, 5).is_err());
    assert!(rng.next_range(6, 5).is_err());
}

#[test]
fn test_get_unique_values_length() {
    let mut rng = RngCryptoServiceProvider::new().unwrap();
    assert_eq!(rng.get_unique_values(24).unwrap().len(), 24);
}

#[test]
fn test_quick_cipher_random_string() {
    use base64::Engine;
    let s = QuickCipher::random_string(12).unwrap();
    let decoded = base64::engine::general_purpose::STANDARD.decode(&s).unwrap();
    assert_eq!(decoded.len(), 12);
}
