//! Permission management for folders

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Device permissions for a folder
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    None,
    Read,
    ReadWrite,
}

/// Folder permissions mapping device IDs to permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderPermissions {
    pub path: String,
    pub permissions: HashMap<String, Permissions>,
}

impl FolderPermissions {
    /// Create new folder permissions
    pub fn new(path: String) -> Self {
        Self {
            path,
            permissions: HashMap::new(),
        }
    }

    /// Grant permission to a device
    pub fn grant(&mut self, device_id: String, perm: Permissions) {
        self.permissions.insert(device_id, perm);
    }

    /// Check if device has permission
    pub fn has_permission(&self, device_id: &str, required: Permissions) -> bool {
        matches!(
            self.permissions.get(device_id),
            Some(p) if *p as u8 >= required as u8
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_permissions() {
        let mut perms = FolderPermissions::new("/data".to_string());
        perms.grant("device1".to_string(), Permissions::Read);
        assert!(perms.has_permission("device1", Permissions::Read));
    }
}
