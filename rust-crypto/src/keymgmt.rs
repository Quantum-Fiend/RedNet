use zeroize::{Zeroize, ZeroizeOnDrop};
use std::fmt;

/// Secure key storage with automatic zeroization
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureKey {
    #[zeroize(skip)]
    key_type: KeyType,
    data: Vec<u8>,
}

#[derive(Clone, Copy)]
pub enum KeyType {
    Aes256,
    ChaCha20,
    Ed25519Secret,
    X25519Secret,
}

impl SecureKey {
    /// Create a new secure key
    pub fn new(key_type: KeyType, data: Vec<u8>) -> Self {
        Self { key_type, data }
    }

    /// Get key data (use carefully)
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Get key type
    pub fn key_type(&self) -> KeyType {
        self.key_type
    }

    /// Get key length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if key is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl fmt::Debug for SecureKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecureKey")
            .field("key_type", &self.key_type)
            .field("length", &self.data.len())
            .field("data", &"<redacted>")
            .finish()
    }
}

impl fmt::Debug for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Aes256 => write!(f, "AES-256"),
            KeyType::ChaCha20 => write!(f, "ChaCha20"),
            KeyType::Ed25519Secret => write!(f, "Ed25519-Secret"),
            KeyType::X25519Secret => write!(f, "X25519-Secret"),
        }
    }
}

/// Key manager for lifecycle management
pub struct KeyManager {
    keys: Vec<SecureKey>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    /// Add a key to the manager
    pub fn add_key(&mut self, key: SecureKey) -> usize {
        self.keys.push(key);
        self.keys.len() - 1
    }

    /// Get a key by index
    pub fn get_key(&self, index: usize) -> Option<&SecureKey> {
        self.keys.get(index)
    }

    /// Remove and zeroize a key
    pub fn remove_key(&mut self, index: usize) -> Option<SecureKey> {
        if index < self.keys.len() {
            Some(self.keys.remove(index))
        } else {
            None
        }
    }

    /// Get number of managed keys
    pub fn key_count(&self) -> usize {
        self.keys.len()
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_key_zeroize() {
        let key_data = vec![1, 2, 3, 4, 5];
        let key = SecureKey::new(KeyType::Aes256, key_data.clone());
        
        assert_eq!(key.as_bytes(), &key_data[..]);
        assert_eq!(key.len(), 5);
        
        drop(key);
        // Key data is automatically zeroized on drop
    }

    #[test]
    fn test_key_manager() {
        let mut manager = KeyManager::new();
        
        let key1 = SecureKey::new(KeyType::Aes256, vec![1; 32]);
        let key2 = SecureKey::new(KeyType::ChaCha20, vec![2; 32]);
        
        let idx1 = manager.add_key(key1);
        let idx2 = manager.add_key(key2);
        
        assert_eq!(manager.key_count(), 2);
        assert!(manager.get_key(idx1).is_some());
        assert!(manager.get_key(idx2).is_some());
        
        manager.remove_key(idx1);
        assert_eq!(manager.key_count(), 1);
    }
}
