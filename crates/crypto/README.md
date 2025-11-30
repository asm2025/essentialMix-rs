# emixcrypto

`emixcrypto` provides comprehensive cryptographic utilities for the EssentialMix project, including encryption, hashing, encoding, and random number generation capabilities.

## Overview

This crate implements a Rust-native cryptographic library based on the C# `essentialMix.Cryptography` project. It follows Rust best practices and idioms, providing a safe and efficient API for cryptographic operations.

## Features

### Symmetric Encryption
- **AES** (Advanced Encryption Standard) with support for:
  - Key sizes: 128, 192, 256 bits
  - Cipher modes: CBC (with more modes planned)
  - Padding: PKCS7, NoPadding, ZeroPadding
  - Key derivation from passphrases using PBKDF2

### Asymmetric Encryption
- **RSA** encryption with:
  - Configurable key sizes (512-4096 bits)
  - Padding schemes: PKCS1, OAEP
  - Signature padding: PKCS1, PSS

### Hash Algorithms
- **SHA-256** and **SHA-512** (default features)
- **SHA-1** (legacy feature)
- **MD5** (legacy feature)
- **HMAC** variants (HMAC-SHA256, HMAC-SHA512)

### Encoders
- **Base64** encoding/decoding
- **Numeric** encoders supporting:
  - Binary
  - Octal
  - Decimal
  - Hexadecimal

### Random Number Generation
- Cryptographically secure RNG using ChaCha20
- Support for generating random bytes, integers, and doubles

### Special Ciphers
- **Vigenère cipher** for educational/historical use

## Quick Start

### Basic Usage

```rust
use emixcrypto::QuickCipher;

// Hash a string
let hash = QuickCipher::hash("Hello, World!", "SHA256")?;
println!("Hash: {}", hash);

// Base64 encode
let encoded = QuickCipher::base64_encode("Hello, World!")?;
println!("Encoded: {}", encoded);

// Decode
let decoded = QuickCipher::base64_decode(&encoded)?;
println!("Decoded: {}", decoded);
```

### Symmetric Encryption

```rust
use emixcrypto::{AesAlgorithm, SymmetricAlgorithm};

// Create an AES cipher
let mut cipher = AesAlgorithm::new()?;

// Generate key from passphrase
cipher.generate_key_from_passphrase("my-secret-passphrase", None, 10000)?;
cipher.generate_iv()?;

// Encrypt
let encrypted = cipher.encrypt_string("Secret message")?;
println!("Encrypted: {}", encrypted);

// Decrypt
let decrypted = cipher.decrypt_string(&encrypted)?;
println!("Decrypted: {}", decrypted);
```

### Asymmetric Encryption

```rust
use emixcrypto::RsaAlgorithm;

// Create RSA key pair
let mut rsa = RsaAlgorithm::new(2048)?;

// Encrypt
let encrypted = rsa.encrypt_string("Secret message")?;

// Decrypt
let decrypted = rsa.decrypt_string(&encrypted)?;
```

### Hash Algorithms

```rust
use emixcrypto::{Sha256Hash, HashAlgorithm};

let hasher = Sha256Hash::new();
let hash = hasher.compute_hash_string("Hello, World!")?;
println!("SHA-256: {}", hash);
```

## Feature Flags

The crate uses feature flags to allow you to include only the algorithms you need:

- **default**: Includes AES, RSA, and SHA2 (SHA-256, SHA-512)
- **full**: Includes all algorithms (MD5, SHA-1, HMAC, Adler32)
- **legacy**: Includes legacy algorithms (MD5, SHA-1)

### Example: Using specific features

```toml
[dependencies]
emixcrypto = { path = "../crates/crypto", features = ["default"] }
```

Or for full functionality:

```toml
[dependencies]
emixcrypto = { path = "../crates/crypto", features = ["full"] }
```

## Error Handling

The crate uses a custom `CryptoError` type that can be converted to `emixcore::Error`:

```rust
use emixcrypto::{CryptoError, Result};

fn my_function() -> Result<String> {
    // Operations that may fail
    Ok("success".to_string())
}
```

## Security Considerations

1. **Key Management**: Always use secure key storage. Keys are automatically zeroized when dropped (using the `zeroize` crate).

2. **Random Number Generation**: The crate uses ChaCha20 RNG, which is cryptographically secure.

3. **Key Derivation**: PBKDF2 is used for deriving keys from passphrases. Use a sufficient number of iterations (recommended: 100,000+).

4. **IV Generation**: Always use cryptographically secure random IVs. Never reuse IVs with the same key.

5. **Legacy Algorithms**: MD5 and SHA-1 are provided for legacy compatibility only. Use SHA-256 or SHA-512 for new code.

## Module Structure

```
src/
├── lib.rs              # Public API and re-exports
├── error.rs            # Error types
├── traits.rs           # Core traits (Algorithm, Encrypt, etc.)
├── settings.rs         # Configuration types
├── symmetric/          # Symmetric encryption
│   ├── aes.rs
│   └── traits.rs
├── asymmetric/         # Asymmetric encryption
│   ├── rsa.rs
│   └── traits.rs
├── hash/               # Hash algorithms
│   ├── sha.rs
│   ├── md5.rs
│   └── hmac.rs
├── encoder/            # Encoding utilities
│   ├── base64.rs
│   └── numeric.rs
├── random/             # Random number generation
│   └── rng.rs
├── cipher/             # Special ciphers
│   └── vigenere.rs
└── service.rs          # QuickCipher high-level service
```

## Dependencies

- **emixcore**: Core error types and utilities
- **aes**: AES encryption (feature-gated)
- **rsa**: RSA encryption (feature-gated)
- **sha2**: SHA-256 and SHA-512 hashing (feature-gated)
- **base64**: Base64 encoding
- **rand**, **rand_chacha**: Random number generation
- **zeroize**: Secure memory zeroing
- **pbkdf2**: Key derivation (feature-gated)

## Examples

See the `examples/` directory (when available) for more comprehensive examples.

## License

MIT (inherited from workspace)

## Contributing

This crate is part of the EssentialMix project. Please follow the project's contribution guidelines.
