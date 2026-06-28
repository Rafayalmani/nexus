//! File chunking logic

use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader};
use crate::hasher::ChunkHash;

const DEFAULT_CHUNK_SIZE: usize = 4 * 1024 * 1024; // 4MB

/// Represents a file chunk
#[derive(Debug, Clone)]
pub struct Chunk {
    pub index: usize,
    pub hash: ChunkHash,
    pub size: usize,
}

/// Chunker for breaking files into pieces
pub struct Chunker {
    chunk_size: usize,
}

impl Chunker {
    /// Create a new chunker with default size
    pub fn new() -> Self {
        Self {
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }

    /// Create a new chunker with custom size
    pub fn with_size(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// Get chunks for a file
    pub fn get_chunks(&self, path: &Path) -> std::io::Result<Vec<Chunk>> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let file_size = metadata.len() as usize;

        let mut chunks = Vec::new();
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; self.chunk_size];
        let mut index = 0;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            let chunk_data = &buffer[..bytes_read];
            let hash = ChunkHash::compute(chunk_data);

            chunks.push(Chunk {
                index,
                hash,
                size: bytes_read,
            });

            index += 1;
        }

        Ok(chunks)
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunker_creation() {
        let chunker = Chunker::new();
        assert_eq!(chunker.chunk_size, DEFAULT_CHUNK_SIZE);
    }

    #[test]
    fn test_custom_chunk_size() {
        let chunker = Chunker::with_size(1024);
        assert_eq!(chunker.chunk_size, 1024);
    }
}
