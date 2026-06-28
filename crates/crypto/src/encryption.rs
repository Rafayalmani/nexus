//! ChaCha20-Poly1305 AEAD encryption

use chapoly::ChaCha20Poly1305;
use rand::rngs::OsRng;
use rand::RngCore;

/// Encrypt data with ChaCha20-Poly1305
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let cipher = ChaCha20Poly1305::new(key.into());
    let ciphertext = cipher.encrypt(nonce.as_ref(), plaintext)?;

    let mut result = Vec::new();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt data with ChaCha20-Poly1305
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if ciphertext.len() < 12 {
        return Err("Ciphertext too short".into());
    }

    let (nonce, encrypted) = ciphertext.split_at(12);
    let cipher = ChaCha20Poly1305::new(key.into());
    let plaintext = cipher.decrypt(nonce, encrypted)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let plaintext = b"Hello, Nexus!";

        let encrypted = encrypt(plaintext, &key).expect("Encryption failed");
        let decrypted = decrypt(&encrypted, &key).expect("Decryption failed");

        assert_eq!(plaintext, decrypted.as_slice());
    }
}
