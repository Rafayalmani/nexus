//! Nexus Sync Engine
//!
//! Handles filesystem watching, delta detection, and chunk management.

pub mod watcher;
pub mod chunker;
pub mod hasher;
pub mod ignore;

pub use watcher::FilesystemWatcher;
pub use chunker::Chunker;
pub use hasher::ChunkHash;
pub use ignore::SmartIgnore;

use std::path::PathBuf;

/// Configuration for the sync engine
#[derive(Debug, Clone)]
pub struct SyncConfig {
    /// Root directory to watch
    pub root_dir: PathBuf,
    /// Chunk size in bytes (default 4MB)
    pub chunk_size: usize,
    /// Path to ignore rules file
    pub ignore_file: Option<PathBuf>,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("."),
            chunk_size: 4 * 1024 * 1024, // 4MB
            ignore_file: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SyncConfig::default();
        assert_eq!(config.chunk_size, 4 * 1024 * 1024);
    }
}
