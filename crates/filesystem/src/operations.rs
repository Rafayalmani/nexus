//! Filesystem operations

use std::path::Path;
use walkdir::WalkDir;

/// List directory contents
pub fn list_directory(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        entries.push(entry.path().to_string_lossy().to_string());
    }

    Ok(entries)
}
