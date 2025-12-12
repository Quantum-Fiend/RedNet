package commands

import (
	"fmt"
	"os"

	"github.com/rednet/cli/internal/ffi"
	"github.com/spf13/cobra"
)

var (
	decryptInput     string
	decryptOutput    string
	decryptKeyFile   string
	decryptNonceFile string
	decryptAlgorithm string
)

var DecryptCmd = &cobra.Command{
	Use:   "decrypt",
	Short: "Decrypt files or data",
	Long:  "Decrypt files encrypted with AES-256-GCM or ChaCha20-Poly1305",
	RunE:  runDecrypt,
}

func init() {
	DecryptCmd.Flags().StringVarP(&decryptInput, "input", "i", "", "Input file to decrypt (required)")
	DecryptCmd.Flags().StringVarP(&decryptOutput, "output", "o", "", "Output file")
	DecryptCmd.Flags().StringVarP(&decryptKeyFile, "key", "k", "", "Key file (default: input.key)")
	DecryptCmd.Flags().StringVarP(&decryptNonceFile, "nonce", "n", "", "Nonce file (default: input.nonce)")
	DecryptCmd.Flags().StringVarP(&decryptAlgorithm, "algorithm", "a", "aes-gcm", "Decryption algorithm")
	DecryptCmd.MarkFlagRequired("input")
}

func runDecrypt(cmd *cobra.Command, args []string) error {
	if decryptKeyFile == "" {
		decryptKeyFile = decryptInput + ".key"
	}
	if decryptNonceFile == "" {
		decryptNonceFile = decryptInput + ".nonce"
	}

	fmt.Printf("🔓 Decrypting: %s\n", decryptInput)

	// Read files
	ciphertext, err := os.ReadFile(decryptInput)
	if err != nil {
		return fmt.Errorf("failed to read input file: %w", err)
	}

	key, err := os.ReadFile(decryptKeyFile)
	if err != nil {
		return fmt.Errorf("failed to read key file: %w", err)
	}

	nonce, err := os.ReadFile(decryptNonceFile)
	if err != nil {
		return fmt.Errorf("failed to read nonce file: %w", err)
	}

	// Decrypt
	var plaintext []byte
	if decryptAlgorithm == "aes-gcm" {
		plaintext, err = ffi.DecryptAesGcm(ciphertext, key, nonce)
	} else if decryptAlgorithm == "chacha20" {
		plaintext, err = ffi.DecryptChaCha20(ciphertext, key, nonce)
	} else {
		return fmt.Errorf("unsupported algorithm: %s", decryptAlgorithm)
	}

	if err != nil {
		return fmt.Errorf("decryption failed: %w", err)
	}

	// Write decrypted data
	if decryptOutput == "" {
		decryptOutput = decryptInput + ".dec"
	}

	if err := os.WriteFile(decryptOutput, plaintext, 0644); err != nil {
		return fmt.Errorf("failed to write output file: %w", err)
	}

	fmt.Printf("✅ Decrypted successfully!\n")
	fmt.Printf("   Output: %s\n", decryptOutput)
	fmt.Printf("   Size:   %d bytes → %d bytes\n", len(ciphertext), len(plaintext))

	return nil
}
