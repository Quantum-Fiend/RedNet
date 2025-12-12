package commands

import (
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/rednet/cli/internal/ffi"
	"github.com/spf13/cobra"
)

var (
	captureInterface string
	captureFilter    string
	captureOutput    string
	captureCount     int
)

var CaptureCmd = &cobra.Command{
	Use:   "capture",
	Short: "Capture network packets",
	Long:  "Capture and analyze network traffic using the C packet sniffer engine",
	RunE:  runCapture,
}

func init() {
	CaptureCmd.Flags().StringVarP(&captureInterface, "interface", "i", "any", "Network interface to capture from")
	CaptureCmd.Flags().StringVarP(&captureFilter, "filter", "f", "", "BPF-style filter expression")
	CaptureCmd.Flags().StringVarP(&captureOutput, "output", "o", "", "PCAP output file")
	CaptureCmd.Flags().IntVarP(&captureCount, "count", "c", 0, "Number of packets to capture (0 = unlimited)")
}

func runCapture(cmd *cobra.Command, args []string) error {
	fmt.Printf("🔍 Starting packet capture on interface: %s\n", captureInterface)

	sniffer := ffi.NewSniffer(captureInterface)
	if sniffer == nil {
		return fmt.Errorf("failed to create sniffer")
	}
	defer sniffer.Destroy()

	if captureFilter != "" {
		fmt.Printf("📝 Applying filter: %s\n", captureFilter)
		if err := sniffer.SetFilter(captureFilter); err != nil {
			return fmt.Errorf("failed to set filter: %w", err)
		}
	}

	if captureOutput != "" {
		fmt.Printf("💾 Writing packets to: %s\n", captureOutput)
		if err := sniffer.StartPcapDump(captureOutput); err != nil {
			return fmt.Errorf("failed to start PCAP dump: %w", err)
		}
		defer sniffer.StopPcapDump()
	}

	// Setup signal handling
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)

	packetCount := 0
	packetChan := make(chan *ffi.Packet, 100)

	// Start capture in goroutine
	go func() {
		sniffer.Start(func(pkt *ffi.Packet) {
			packetChan <- pkt
		})
	}()

	fmt.Println("📡 Capturing packets... (Press Ctrl+C to stop)")
	fmt.Println("─────────────────────────────────────────────────────────")

	for {
		select {
		case <-sigChan:
			fmt.Println("\n🛑 Stopping capture...")
			sniffer.Stop()
			stats := sniffer.GetStats()
			printStats(stats)
			return nil

		case pkt := <-packetChan:
			packetCount++
			printPacket(pkt, packetCount)

			if captureCount > 0 && packetCount >= captureCount {
				fmt.Printf("\n✅ Captured %d packets\n", packetCount)
				sniffer.Stop()
				stats := sniffer.GetStats()
				printStats(stats)
				return nil
			}
		}
	}
}

func printPacket(pkt *ffi.Packet, count int) {
	fmt.Printf("[%d] %s | %s:%d → %s:%d | %d bytes\n",
		count,
		pkt.Protocol,
		pkt.SrcIP,
		pkt.SrcPort,
		pkt.DstIP,
		pkt.DstPort,
		pkt.Length,
	)
}

func printStats(stats *ffi.SnifferStats) {
	fmt.Println("\n📊 Capture Statistics:")
	fmt.Printf("  Total Packets: %d\n", stats.TotalPackets)
	fmt.Printf("  Total Bytes:   %d\n", stats.TotalBytes)
	fmt.Printf("  TCP Packets:   %d\n", stats.TcpPackets)
	fmt.Printf("  UDP Packets:   %d\n", stats.UdpPackets)
	fmt.Printf("  HTTP Packets:  %d\n", stats.HttpPackets)
	fmt.Printf("  HTTPS Packets: %d\n", stats.HttpsPackets)
	fmt.Printf("  DNS Packets:   %d\n", stats.DnsPackets)
	fmt.Printf("  Dropped:       %d\n", stats.DroppedPackets)
}
