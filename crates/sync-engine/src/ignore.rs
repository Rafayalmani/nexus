//! Smart ignore rules for files and directories

use std::path::Path;

const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
    "node_modules",
    ".git",
    ".vscode",
    ".idea",
    "__pycache__",
    "*.pyc",
    "target",
    "build",
    ".DS_Store",
    "Thumbs.db",
    ".gradle",
    ".dart_tool",
];

/// Smart ignore rule matcher
pub struct SmartIgnore {
    patterns: Vec<String>,
}

impl SmartIgnore {
    /// Create with default patterns
    pub fn new() -> Self {
        Self {
            patterns: DEFAULT_IGNORE_PATTERNS.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Add custom pattern
    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    /// Check if path should be ignored
    pub fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern in &self.patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }
}

impl Default for SmartIgnore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore_node_modules() {
        let ignore = SmartIgnore::new();
        assert!(ignore.should_ignore(Path::new("/home/user/project/node_modules")));
    }

    #[test]
    fn test_ignore_git() {
        let ignore = SmartIgnore::new();
        assert!(ignore.should_ignore(Path::new(".git/config")));
    }

    #[test]
    fn test_dont_ignore() {
        let ignore = SmartIgnore::new();
        assert!(!ignore.should_ignore(Path::new("/home/user/myfile.txt")));
    }
}
