//! X25519 key exchange for device pairing

use x25519_dalek::{StaticSecret, PublicKey};
use rand::rngs::OsRng;
use hex;

/// Device key pair for X25519 exchange
#[derive(Clone)]
pub struct KeyPair {
    pub secret: StaticSecret,
    pub public: PublicKey,
}

impl KeyPair {
    /// Generate new keypair
    pub fn generate() -> Self {
        let secret = StaticSecret::new(OsRng);
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }

    /// Get public key as hex string
    pub fn public_hex(&self) -> String {
        hex::encode(self.public.as_bytes())
    }

    /// Perform key exchange to derive shared secret
    pub fn exchange(&self, peer_public: &PublicKey) -> [u8; 32] {
        self.secret.diffie_hellman(peer_public).as_bytes().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = KeyPair::generate();
        assert_eq!(kp.public_hex().len(), 64);
    }

    #[test]
    fn test_key_exchange() {
        let alice = KeyPair::generate();
        let bob = KeyPair::generate();

        let shared1 = alice.exchange(&bob.public);
        let shared2 = bob.exchange(&alice.public);

        assert_eq!(shared1, shared2);
    }
}
