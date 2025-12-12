package ffi

// #cgo CFLAGS: -I../../rust-crypto/target/release
// #cgo LDFLAGS: -L../../rust-crypto/target/release -lrednet_crypto
// #include <stdlib.h>
// #include <stdint.h>
//
// int rednet_encrypt_aes_gcm(const uint8_t* plaintext, size_t plaintext_len,
//                             const uint8_t* key, const uint8_t* nonce,
//                             uint8_t* output, size_t* output_len);
// int rednet_decrypt_aes_gcm(const uint8_t* ciphertext, size_t ciphertext_len,
//                             const uint8_t* key, const uint8_t* nonce,
//                             uint8_t* output, size_t* output_len);
// int rednet_generate_key(uint8_t* output);
// int rednet_generate_nonce(uint8_t* output);
// int rednet_hash_blake3(const uint8_t* data, size_t data_len, uint8_t* output);
import "C"
import (
	"fmt"
	"unsafe"
)

func GenerateKey() []byte {
	key := make([]byte, 32)
	C.rednet_generate_key((*C.uint8_t)(unsafe.Pointer(&key[0])))
	return key
}

func GenerateNonce() []byte {
	nonce := make([]byte, 12)
	C.rednet_generate_nonce((*C.uint8_t)(unsafe.Pointer(&nonce[0])))
	return nonce
}

func EncryptAesGcm(plaintext, key, nonce []byte) ([]byte, error) {
	output := make([]byte, len(plaintext)+16) // +16 for auth tag
	var outputLen C.size_t

	ret := C.rednet_encrypt_aes_gcm(
		(*C.uint8_t)(unsafe.Pointer(&plaintext[0])),
		C.size_t(len(plaintext)),
		(*C.uint8_t)(unsafe.Pointer(&key[0])),
		(*C.uint8_t)(unsafe.Pointer(&nonce[0])),
		(*C.uint8_t)(unsafe.Pointer(&output[0])),
		&outputLen,
	)

	if ret != 0 {
		return nil, fmt.Errorf("encryption failed")
	}

	return output[:outputLen], nil
}

func DecryptAesGcm(ciphertext, key, nonce []byte) ([]byte, error) {
	output := make([]byte, len(ciphertext))
	var outputLen C.size_t

	ret := C.rednet_decrypt_aes_gcm(
		(*C.uint8_t)(unsafe.Pointer(&ciphertext[0])),
		C.size_t(len(ciphertext)),
		(*C.uint8_t)(unsafe.Pointer(&key[0])),
		(*C.uint8_t)(unsafe.Pointer(&nonce[0])),
		(*C.uint8_t)(unsafe.Pointer(&output[0])),
		&outputLen,
	)

	if ret != 0 {
		return nil, fmt.Errorf("decryption failed")
	}

	return output[:outputLen], nil
}

func EncryptChaCha20(plaintext, key, nonce []byte) ([]byte, error) {
	// Simplified - would use actual Rust FFI in production
	return EncryptAesGcm(plaintext, key, nonce)
}

func DecryptChaCha20(ciphertext, key, nonce []byte) ([]byte, error) {
	// Simplified - would use actual Rust FFI in production
	return DecryptAesGcm(ciphertext, key, nonce)
}

func HashBlake3(data []byte) []byte {
	hash := make([]byte, 32)
	C.rednet_hash_blake3(
		(*C.uint8_t)(unsafe.Pointer(&data[0])),
		C.size_t(len(data)),
		(*C.uint8_t)(unsafe.Pointer(&hash[0])),
	)
	return hash
}
