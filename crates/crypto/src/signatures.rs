//! Ed25519 device signatures

use ed25519_dalek::{SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use hex;

/// Sign data with Ed25519
pub fn sign(data: &[u8], secret: &[u8; 32]) -> Signature {
    let signing_key = SigningKey::from_bytes(secret);
    signing_key.sign(data)
}

/// Verify Ed25519 signature
pub fn verify(data: &[u8], public_hex: &str, signature: &Signature) -> Result<(), Box<dyn std::error::Error>> {
    let public_bytes = hex::decode(public_hex)?;
    let verifying_key = VerifyingKey::from_bytes(public_bytes.as_slice().try_into()?)?;
    verifying_key.verify(data, signature)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let secret = [0u8; 32];
        let data = b"Device identity";

        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key = signing_key.verifying_key();

        let signature = sign(data, &secret);
        let public_hex = hex::encode(verifying_key.to_bytes());

        assert!(verify(data, &public_hex, &signature).is_ok());
    }
}
