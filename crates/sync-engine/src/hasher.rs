//! SHA-256 hashing for chunks

use sha2::{Sha256, Digest};
use hex;

/// Represents a chunk hash (SHA-256)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkHash(pub String);

impl ChunkHash {
    /// Compute SHA-256 hash of data
    pub fn compute(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Self(hex::encode(result))
    }

    /// Get hash as hex string
    pub fn as_hex(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ChunkHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_compute() {
        let data = b"hello world";
        let hash = ChunkHash::compute(data);
        assert_eq!(hash.as_hex().len(), 64); // SHA-256 = 64 hex chars
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = ChunkHash::compute(data);
        let hash2 = ChunkHash::compute(data);
        assert_eq!(hash1, hash2);
    }
}
