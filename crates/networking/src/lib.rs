//! Nexus Networking Layer
//!
//! Handles P2P connections, NAT traversal via STUN,
//! UDP hole punching, and QUIC transport.

pub mod stun_client;
pub mod p2p;

pub use stun_client::StunClient;
pub use p2p::P2PConnection;
