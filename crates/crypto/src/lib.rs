//! Nexus Cryptography Module
//!
//! Provides X25519 key exchange, ChaCha20-Poly1305 AEAD encryption,
//! and Ed25519 signatures for device identity.

pub mod key_exchange;
pub mod encryption;
pub mod signatures;

pub use key_exchange::KeyPair;
pub use encryption::{encrypt, decrypt};
pub use signatures::{sign, verify};
