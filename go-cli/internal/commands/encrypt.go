package commands

import (
	"fmt"
	"os"

	"github.com/rednet/cli/internal/ffi"
	"github.com/spf13/cobra"
)

var (
	encryptInput     string
	encryptOutput    string
	encryptAlgorithm string
)

var EncryptCmd = &cobra.Command{
	Use:   "encrypt",
	Short: "Encrypt files or data",
	Long:  "Encrypt files using AES-256-GCM or ChaCha20-Poly1305",
	RunE:  runEncrypt,
}

func init() {
	EncryptCmd.Flags().StringVarP(&encryptInput, "input", "i", "", "Input file to encrypt (required)")
	EncryptCmd.Flags().StringVarP(&encryptOutput, "output", "o", "", "Output file (default: input.enc)")
	EncryptCmd.Flags().StringVarP(&encryptAlgorithm, "algorithm", "a", "aes-gcm", "Encryption algorithm (aes-gcm, chacha20)")
	EncryptCmd.MarkFlagRequired("input")
}

func runEncrypt(cmd *cobra.Command, args []string) error {
	if encryptOutput == "" {
		encryptOutput = encryptInput + ".enc"
	}

	fmt.Printf("🔐 Encrypting: %s\n", encryptInput)
	fmt.Printf("📝 Algorithm: %s\n", encryptAlgorithm)

	// Read input file
	plaintext, err := os.ReadFile(encryptInput)
	if err != nil {
		return fmt.Errorf("failed to read input file: %w", err)
	}

	// Generate key and nonce
	key := ffi.GenerateKey()
	nonce := ffi.GenerateNonce()

	// Encrypt
	var ciphertext []byte
	if encryptAlgorithm == "aes-gcm" {
		ciphertext, err = ffi.EncryptAesGcm(plaintext, key, nonce)
	} else if encryptAlgorithm == "chacha20" {
		ciphertext, err = ffi.EncryptChaCha20(plaintext, key, nonce)
	} else {
		return fmt.Errorf("unsupported algorithm: %s", encryptAlgorithm)
	}

	if err != nil {
		return fmt.Errorf("encryption failed: %w", err)
	}

	// Write encrypted data
	if err := os.WriteFile(encryptOutput, ciphertext, 0644); err != nil {
		return fmt.Errorf("failed to write output file: %w", err)
	}

	// Save key and nonce
	keyFile := encryptOutput + ".key"
	nonceFile := encryptOutput + ".nonce"

	if err := os.WriteFile(keyFile, key, 0600); err != nil {
		return fmt.Errorf("failed to write key file: %w", err)
	}

	if err := os.WriteFile(nonceFile, nonce, 0600); err != nil {
		return fmt.Errorf("failed to write nonce file: %w", err)
	}

	fmt.Printf("✅ Encrypted successfully!\n")
	fmt.Printf("   Output: %s\n", encryptOutput)
	fmt.Printf("   Key:    %s\n", keyFile)
	fmt.Printf("   Nonce:  %s\n", nonceFile)
	fmt.Printf("   Size:   %d bytes → %d bytes\n", len(plaintext), len(ciphertext))

	return nil
}
