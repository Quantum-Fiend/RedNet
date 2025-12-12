use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey};
use x25519_dalek::{PublicKey, StaticSecret};
use rand::rngs::OsRng;
use zeroize::Zeroize;

pub struct KeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl KeyPair {
    /// Generate a new Ed25519 key pair
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing_key.sign(message).to_bytes().to_vec()
    }

    /// Verify a signature
    pub fn verify(verifying_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        if verifying_key.len() != 32 || signature.len() != 64 {
            return false;
        }

        let vk = match VerifyingKey::from_bytes(verifying_key.try_into().unwrap()) {
            Ok(k) => k,
            Err(_) => return false,
        };

        let sig = match Signature::from_bytes(signature.try_into().unwrap()) {
            Ok(s) => s,
            Err(_) => return false,
        };

        vk.verify(message, &sig).is_ok()
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
}

pub struct DhKeyPair {
    secret: StaticSecret,
    public: PublicKey,
}

impl DhKeyPair {
    /// Generate a new X25519 key pair for Diffie-Hellman
    pub fn generate() -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);
        
        Self { secret, public }
    }

    /// Perform Diffie-Hellman key exchange
    pub fn exchange(&self, their_public: &[u8]) -> Result<[u8; 32], &'static str> {
        if their_public.len() != 32 {
            return Err("Invalid public key length");
        }

        let their_public_key = PublicKey::from(*array_ref![their_public, 0, 32]);
        let shared_secret = self.secret.diffie_hellman(&their_public_key);
        
        Ok(shared_secret.to_bytes())
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }
}

impl Drop for DhKeyPair {
    fn drop(&mut self) {
        // Zeroize is automatically called on StaticSecret
    }
}

#[macro_export]
macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            fn as_array<T>(slice: &[T]) -> &[T; $len] {
                unsafe { &*(slice.as_ptr() as *const [T; $len]) }
            }
            as_array(&$arr[$offset..$offset + $len])
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_sign_verify() {
        let keypair = KeyPair::generate();
        let message = b"Test message for signing";
        
        let signature = keypair.sign(message);
        let public_key = keypair.public_key_bytes();
        
        assert!(KeyPair::verify(&public_key, message, &signature));
        assert!(!KeyPair::verify(&public_key, b"wrong message", &signature));
    }

    #[test]
    fn test_x25519_key_exchange() {
        let alice = DhKeyPair::generate();
        let bob = DhKeyPair::generate();
        
        let alice_shared = alice.exchange(&bob.public_key_bytes()).unwrap();
        let bob_shared = bob.exchange(&alice.public_key_bytes()).unwrap();
        
        assert_eq!(alice_shared, bob_shared);
    }
}
