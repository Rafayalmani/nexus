//! STUN client for NAT traversal

use std::net::SocketAddr;

/// STUN client for public IP discovery
pub struct StunClient {
    stun_server: String,
}

impl StunClient {
    /// Create new STUN client
    pub fn new(stun_server: String) -> Self {
        Self { stun_server }
    }

    /// Get public IP and port
    pub async fn get_public_address(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        Ok("0.0.0.0:0".parse()?)
    }
}

impl Default for StunClient {
    fn default() -> Self {
        Self::new("stun.l.google.com:19302".to_string())
    }
}
