use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305, Key};
use zeroize::Zeroize;

pub struct AeadEngine {
    algorithm: AeadAlgorithm,
}

pub enum AeadAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
}

#[derive(Debug)]
pub enum CryptoError {
    EncryptionFailed,
    DecryptionFailed,
    InvalidKeyLength,
    InvalidNonceLength,
}

impl AeadEngine {
    pub fn new(algorithm: AeadAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Encrypt data with AES-256-GCM
    pub fn encrypt_aes_gcm(
        &self,
        plaintext: &[u8],
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyLength);
        }
        if nonce.len() != 12 {
            return Err(CryptoError::InvalidNonceLength);
        }

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        
        let nonce_array = Nonce::from_slice(nonce);
        
        let mut payload = plaintext.to_vec();
        payload.extend_from_slice(aad);
        
        cipher
            .encrypt(nonce_array, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)
    }

    /// Decrypt data with AES-256-GCM
    pub fn decrypt_aes_gcm(
        &self,
        ciphertext: &[u8],
        key: &[u8],
        nonce: &[u8],
        _aad: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyLength);
        }
        if nonce.len() != 12 {
            return Err(CryptoError::InvalidNonceLength);
        }

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        
        let nonce_array = Nonce::from_slice(nonce);
        
        cipher
            .decrypt(nonce_array, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }

    /// Encrypt data with ChaCha20-Poly1305
    pub fn encrypt_chacha20(
        &self,
        plaintext: &[u8],
        key: &[u8],
        nonce: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyLength);
        }
        if nonce.len() != 12 {
            return Err(CryptoError::InvalidNonceLength);
        }

        let key_array = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key_array);
        
        let nonce_array = chacha20poly1305::Nonce::from_slice(nonce);
        
        cipher
            .encrypt(nonce_array, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)
    }

    /// Decrypt data with ChaCha20-Poly1305
    pub fn decrypt_chacha20(
        &self,
        ciphertext: &[u8],
        key: &[u8],
        nonce: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyLength);
        }
        if nonce.len() != 12 {
            return Err(CryptoError::InvalidNonceLength);
        }

        let key_array = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key_array);
        
        let nonce_array = chacha20poly1305::Nonce::from_slice(nonce);
        
        cipher
            .decrypt(nonce_array, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

/// Generate a random 256-bit key
pub fn generate_key() -> [u8; 32] {
    use rand::RngCore;
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

/// Generate a random 96-bit nonce
pub fn generate_nonce() -> [u8; 12] {
    use rand::RngCore;
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm_roundtrip() {
        let engine = AeadEngine::new(AeadAlgorithm::Aes256Gcm);
        let key = generate_key();
        let nonce = generate_nonce();
        let plaintext = b"Hello, RedNet!";
        let aad = b"additional data";

        let ciphertext = engine.encrypt_aes_gcm(plaintext, &key, &nonce, aad).unwrap();
        let decrypted = engine.decrypt_aes_gcm(&ciphertext, &key, &nonce, aad).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_chacha20_roundtrip() {
        let engine = AeadEngine::new(AeadAlgorithm::ChaCha20Poly1305);
        let key = generate_key();
        let nonce = generate_nonce();
        let plaintext = b"Hello, RedNet with ChaCha20!";

        let ciphertext = engine.encrypt_chacha20(plaintext, &key, &nonce).unwrap();
        let decrypted = engine.decrypt_chacha20(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
