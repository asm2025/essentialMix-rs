use emixcrypto::{HashAlgorithm, QuickCipher, Sha256Hash, Sha512Hash};

const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
const ABC_SHA512: &str = "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f";

#[test]
fn test_sha256_known_vector() {
    let hash = Sha256Hash::new().compute_hash_string("abc").unwrap();
    assert_eq!(hash, ABC_SHA256);
}

#[test]
fn test_sha512_known_vector() {
    let hash = Sha512Hash::new().compute_hash_string("abc").unwrap();
    assert_eq!(hash, ABC_SHA512);
}

#[test]
fn test_sha_sizes() {
    let sha256 = Sha256Hash::new();
    assert_eq!(sha256.hash_size(), 32);
    assert_eq!(sha256.compute_hash_bytes(b"abc").unwrap().len(), 32);

    let sha512 = Sha512Hash::new();
    assert_eq!(sha512.hash_size(), 64);
    assert_eq!(sha512.compute_hash_bytes(b"abc").unwrap().len(), 64);
}

#[test]
fn test_hash_slice_matches_hash_of_subslice() {
    let hasher = Sha256Hash::new();
    let data = b"xxabcxx";
    let slice = hasher.compute_hash_slice(data, 2, 3).unwrap();
    assert_eq!(hex::encode(slice), ABC_SHA256);
}

#[test]
fn test_hash_slice_out_of_bounds() {
    let hasher = Sha256Hash::new();
    assert!(hasher.compute_hash_slice(b"abc", 2, 5).is_err());
}

#[test]
fn test_quick_cipher_hash() {
    assert_eq!(QuickCipher::hash("abc", "SHA256").unwrap(), ABC_SHA256);
    assert_eq!(QuickCipher::hash("abc", "sha-512").unwrap(), ABC_SHA512);
    assert!(QuickCipher::hash("abc", "NOPE").is_err());
}

#[cfg(feature = "sha1")]
#[test]
fn test_sha1_known_vector() {
    use emixcrypto::Sha1Hash;
    let hash = Sha1Hash::new().compute_hash_string("abc").unwrap();
    assert_eq!(hash, "a9993e364706816aba3e25717850c26c9cd0d89d");
    assert_eq!(QuickCipher::hash("abc", "SHA1").unwrap(), hash);
}

#[cfg(feature = "md5")]
#[test]
fn test_md5_known_vector() {
    use emixcrypto::Md5Hash;
    let hash = Md5Hash::new().compute_hash_string("abc").unwrap();
    assert_eq!(hash, "900150983cd24fb0d6963f7d28e17f72");
    assert_eq!(QuickCipher::hash("abc", "MD5").unwrap(), hash);
}

// RFC 4231, test case 2
#[cfg(feature = "hmac")]
#[test]
fn test_hmac_sha256_rfc4231() {
    use emixcrypto::HmacSha256;
    let mac = HmacSha256::new(b"Jefe")
        .compute_hash_string("what do ya want for nothing?")
        .unwrap();
    assert_eq!(mac, "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843");
}

#[cfg(feature = "hmac")]
#[test]
fn test_hmac_sha512_rfc4231() {
    use emixcrypto::HmacSha512;
    let mac = HmacSha512::new(b"Jefe")
        .compute_hash_string("what do ya want for nothing?")
        .unwrap();
    assert_eq!(
        mac,
        "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"
    );
}
