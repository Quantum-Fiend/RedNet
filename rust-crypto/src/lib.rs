pub mod aead;
pub mod keyexchange;
pub mod kdf;
pub mod hash;
pub mod keymgmt;

pub use aead::{AeadEngine, AeadAlgorithm, CryptoError, generate_key, generate_nonce};
pub use keyexchange::{KeyPair, DhKeyPair};
pub use kdf::{hkdf_derive, argon2_hash, argon2_verify, argon2_derive_key};
pub use hash::{hash_data, hash_file, verify_file_integrity, hash_to_hex, keyed_hash, derive_key};
pub use keymgmt::{SecureKey, KeyType, KeyManager};

// C FFI exports
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar};
use std::slice;

/// Encrypt data using AES-256-GCM (C FFI)
#[no_mangle]
pub extern "C" fn rednet_encrypt_aes_gcm(
    plaintext: *const c_uchar,
    plaintext_len: usize,
    key: *const c_uchar,
    nonce: *const c_uchar,
    output: *mut c_uchar,
    output_len: *mut usize,
) -> c_int {
    if plaintext.is_null() || key.is_null() || nonce.is_null() || output.is_null() {
        return -1;
    }

    let plaintext_slice = unsafe { slice::from_raw_parts(plaintext, plaintext_len) };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let nonce_slice = unsafe { slice::from_raw_parts(nonce, 12) };

    let engine = AeadEngine::new(AeadAlgorithm::Aes256Gcm);
    
    match engine.encrypt_aes_gcm(plaintext_slice, key_slice, nonce_slice, &[]) {
        Ok(ciphertext) => {
            unsafe {
                std::ptr::copy_nonoverlapping(ciphertext.as_ptr(), output, ciphertext.len());
                *output_len = ciphertext.len();
            }
            0
        }
        Err(_) => -1,
    }
}

/// Decrypt data using AES-256-GCM (C FFI)
#[no_mangle]
pub extern "C" fn rednet_decrypt_aes_gcm(
    ciphertext: *const c_uchar,
    ciphertext_len: usize,
    key: *const c_uchar,
    nonce: *const c_uchar,
    output: *mut c_uchar,
    output_len: *mut usize,
) -> c_int {
    if ciphertext.is_null() || key.is_null() || nonce.is_null() || output.is_null() {
        return -1;
    }

    let ciphertext_slice = unsafe { slice::from_raw_parts(ciphertext, ciphertext_len) };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let nonce_slice = unsafe { slice::from_raw_parts(nonce, 12) };

    let engine = AeadEngine::new(AeadAlgorithm::Aes256Gcm);
    
    match engine.decrypt_aes_gcm(ciphertext_slice, key_slice, nonce_slice, &[]) {
        Ok(plaintext) => {
            unsafe {
                std::ptr::copy_nonoverlapping(plaintext.as_ptr(), output, plaintext.len());
                *output_len = plaintext.len();
            }
            0
        }
        Err(_) => -1,
    }
}

/// Generate a random 256-bit key (C FFI)
#[no_mangle]
pub extern "C" fn rednet_generate_key(output: *mut c_uchar) -> c_int {
    if output.is_null() {
        return -1;
    }

    let key = generate_key();
    unsafe {
        std::ptr::copy_nonoverlapping(key.as_ptr(), output, 32);
    }
    0
}

/// Generate a random 96-bit nonce (C FFI)
#[no_mangle]
pub extern "C" fn rednet_generate_nonce(output: *mut c_uchar) -> c_int {
    if output.is_null() {
        return -1;
    }

    let nonce = generate_nonce();
    unsafe {
        std::ptr::copy_nonoverlapping(nonce.as_ptr(), output, 12);
    }
    0
}

/// Hash data using BLAKE3 (C FFI)
#[no_mangle]
pub extern "C" fn rednet_hash_blake3(
    data: *const c_uchar,
    data_len: usize,
    output: *mut c_uchar,
) -> c_int {
    if data.is_null() || output.is_null() {
        return -1;
    }

    let data_slice = unsafe { slice::from_raw_parts(data, data_len) };
    let hash = hash_data(data_slice);
    
    unsafe {
        std::ptr::copy_nonoverlapping(hash.as_ptr(), output, 32);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_encryption_workflow() {
        let plaintext = b"RedNet Cybersecurity Toolkit";
        let key = generate_key();
        let nonce = generate_nonce();

        let engine = AeadEngine::new(AeadAlgorithm::Aes256Gcm);
        let ciphertext = engine.encrypt_aes_gcm(plaintext, &key, &nonce, &[]).unwrap();
        let decrypted = engine.decrypt_aes_gcm(&ciphertext, &key, &nonce, &[]).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
