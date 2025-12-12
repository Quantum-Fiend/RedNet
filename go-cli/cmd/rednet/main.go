package main

import (
	"fmt"
	"os"

	"github.com/rednet/cli/internal/commands"
	"github.com/spf13/cobra"
)

var (
	version = "1.0.0"
	rootCmd = &cobra.Command{
		Use:   "rednet",
		Short: "RedNet - Multi-Language Cybersecurity Toolkit",
		Long: `RedNet is a comprehensive cybersecurity toolkit featuring:
- High-performance packet capture and DPI
- Modern encryption (AES-GCM, ChaCha20-Poly1305)
- Payload generation and testing
- Real-time monitoring and analytics`,
		Version: version,
	}
)

func init() {
	rootCmd.AddCommand(commands.CaptureCmd)
	rootCmd.AddCommand(commands.EncryptCmd)
	rootCmd.AddCommand(commands.DecryptCmd)
	rootCmd.AddCommand(commands.AnalyzeCmd)
	rootCmd.AddCommand(commands.PayloadCmd)
	rootCmd.AddCommand(commands.MonitorCmd)
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}
