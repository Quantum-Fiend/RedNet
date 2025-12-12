use blake3::{Hash, Hasher};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Compute BLAKE3 hash of data
pub fn hash_data(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

/// Compute BLAKE3 hash of a file
pub fn hash_file<P: AsRef<Path>>(path: P) -> io::Result<[u8; 32]> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(hasher.finalize().into())
}

/// Verify file integrity
pub fn verify_file_integrity<P: AsRef<Path>>(path: P, expected_hash: &[u8; 32]) -> io::Result<bool> {
    let computed_hash = hash_file(path)?;
    Ok(&computed_hash == expected_hash)
}

/// Convert hash to hex string
pub fn hash_to_hex(hash: &[u8; 32]) -> String {
    hex::encode(hash)
}

/// Keyed hash (MAC)
pub fn keyed_hash(key: &[u8; 32], data: &[u8]) -> [u8; 32] {
    blake3::keyed_hash(key, data).into()
}

/// Derive key using BLAKE3 KDF
pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; 32] {
    blake3::derive_key(context, key_material).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_data() {
        let data = b"Hello, BLAKE3!";
        let hash = hash_data(data);
        assert_eq!(hash.len(), 32);
        
        // Same data should produce same hash
        let hash2 = hash_data(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_hash_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"File content for hashing").unwrap();
        
        let hash = hash_file(temp_file.path()).unwrap();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_verify_integrity() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = b"Integrity test content";
        temp_file.write_all(content).unwrap();
        
        let hash = hash_file(temp_file.path()).unwrap();
        assert!(verify_file_integrity(temp_file.path(), &hash).unwrap());
        
        // Wrong hash should fail
        let wrong_hash = [0u8; 32];
        assert!(!verify_file_integrity(temp_file.path(), &wrong_hash).unwrap());
    }

    #[test]
    fn test_keyed_hash() {
        let key = [42u8; 32];
        let data = b"Message to MAC";
        
        let mac = keyed_hash(&key, data);
        assert_eq!(mac.len(), 32);
    }

    #[test]
    fn test_derive_key() {
        let context = "RedNet Application Key";
        let material = b"source key material";
        
        let derived = derive_key(context, material);
        assert_eq!(derived.len(), 32);
    }
}
