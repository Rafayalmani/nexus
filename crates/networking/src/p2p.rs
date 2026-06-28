//! P2P connection management

use std::net::SocketAddr;

/// Represents a P2P connection between devices
pub struct P2PConnection {
    pub device_id: String,
    pub remote_addr: SocketAddr,
    pub is_direct: bool,
}

impl P2PConnection {
    /// Create new P2P connection
    pub fn new(device_id: String, remote_addr: SocketAddr, is_direct: bool) -> Self {
        Self {
            device_id,
            remote_addr,
            is_direct,
        }
    }

    /// Send data to peer
    pub async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Receive data from peer
    pub async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}
