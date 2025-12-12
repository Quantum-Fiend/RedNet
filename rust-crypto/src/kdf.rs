use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use hkdf::Hkdf;
use sha2::Sha256;

/// Derive a key using HKDF-SHA256
pub fn hkdf_derive(
    ikm: &[u8],  // Input key material
    salt: &[u8],
    info: &[u8],
    output_len: usize,
) -> Vec<u8> {
    let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
    let mut okm = vec![0u8; output_len];
    hk.expand(info, &mut okm).expect("HKDF expand failed");
    okm
}

/// Hash a password using Argon2id
pub fn argon2_hash(password: &[u8]) -> Result<String, &'static str> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password, &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| "Password hashing failed")
}

/// Verify a password against an Argon2 hash
pub fn argon2_verify(password: &[u8], hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok()
}

/// Derive a key from a password using Argon2id
pub fn argon2_derive_key(password: &[u8], salt: &[u8], output_len: usize) -> Result<Vec<u8>, &'static str> {
    use argon2::Algorithm;
    use argon2::Params;
    use argon2::Version;
    
    let params = Params::new(65536, 3, 4, Some(output_len))
        .map_err(|_| "Invalid Argon2 parameters")?;
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    let mut output = vec![0u8; output_len];
    argon2
        .hash_password_into(password, salt, &mut output)
        .map_err(|_| "Key derivation failed")?;
    
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hkdf() {
        let ikm = b"input key material";
        let salt = b"salt";
        let info = b"application info";
        
        let key = hkdf_derive(ikm, salt, info, 32);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_argon2_hash_verify() {
        let password = b"super_secret_password";
        let hash = argon2_hash(password).unwrap();
        
        assert!(argon2_verify(password, &hash));
        assert!(!argon2_verify(b"wrong_password", &hash));
    }

    #[test]
    fn test_argon2_derive_key() {
        let password = b"my_password";
        let salt = b"random_salt_1234";
        
        let key = argon2_derive_key(password, salt, 32).unwrap();
        assert_eq!(key.len(), 32);
    }
}
