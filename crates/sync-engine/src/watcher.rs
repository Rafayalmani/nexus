//! Filesystem watcher implementation

use notify::{Watcher, RecursiveMode, watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use tracing::{info, warn};

/// File system events
#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

/// Watches filesystem changes
pub struct FilesystemWatcher {
    root: PathBuf,
}

impl FilesystemWatcher {
    /// Create a new filesystem watcher
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Start watching for filesystem changes
    pub fn watch(&self) -> Result<mpsc::Receiver<FileEvent>, Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel();
        let root = self.root.clone();

        let mut watcher = watcher(
            move |res| {
                match res {
                    Ok(notify::DebouncedEvent::Create(path)) => {
                        let _ = tx.send(FileEvent::Created(path));
                    }
                    Ok(notify::DebouncedEvent::Write(path)) => {
                        let _ = tx.send(FileEvent::Modified(path));
                    }
                    Ok(notify::DebouncedEvent::Remove(path)) => {
                        let _ = tx.send(FileEvent::Removed(path));
                    }
                    Ok(notify::DebouncedEvent::Rename(from, to)) => {
                        let _ = tx.send(FileEvent::Renamed { from, to });
                    }
                    Err(e) => warn!("Watch error: {:?}", e),
                    _ => {}
                }
            },
            std::time::Duration::from_millis(500),
        )?;

        watcher.watch(&root, RecursiveMode::Recursive)?;

        // Keep watcher alive
        std::mem::forget(watcher);

        info!("Watching: {:?}", root);
        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watcher_creation() {
        let watcher = FilesystemWatcher::new(PathBuf::from("."));
        assert_eq!(watcher.root, PathBuf::from("."));
    }
}
