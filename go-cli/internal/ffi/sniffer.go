package ffi

// #cgo CFLAGS: -I../../c-sniffer/include
// #cgo LDFLAGS: -L../../c-sniffer/lib -lpacket_sniffer
// #include <stdlib.h>
// #include "packet_sniffer.h"
import "C"
import (
	"fmt"
	"unsafe"
)

type Packet struct {
	Data     []byte
	Length   uint32
	Protocol string
	SrcIP    string
	DstIP    string
	SrcPort  uint16
	DstPort  uint16
}

type SnifferStats struct {
	TotalPackets   uint64
	TotalBytes     uint64
	TcpPackets     uint64
	UdpPackets     uint64
	HttpPackets    uint64
	HttpsPackets   uint64
	DnsPackets     uint64
	DroppedPackets uint64
}

type Sniffer struct {
	handle *C.sniffer_t
}

func NewSniffer(iface string) *Sniffer {
	cIface := C.CString(iface)
	defer C.free(unsafe.Pointer(cIface))

	handle := C.sniffer_create(cIface)
	if handle == nil {
		return nil
	}

	return &Sniffer{handle: handle}
}

func (s *Sniffer) Destroy() {
	if s.handle != nil {
		C.sniffer_destroy(s.handle)
		s.handle = nil
	}
}

func (s *Sniffer) Start(callback func(*Packet)) error {
	// Note: Actual callback implementation would require more complex CGO
	// This is a simplified version
	return fmt.Errorf("not implemented - use Docker version with full FFI")
}

func (s *Sniffer) Stop() {
	if s.handle != nil {
		C.sniffer_stop(s.handle)
	}
}

func (s *Sniffer) SetFilter(filter string) error {
	cFilter := C.CString(filter)
	defer C.free(unsafe.Pointer(cFilter))

	ret := C.sniffer_set_filter(s.handle, cFilter)
	if ret != 0 {
		return fmt.Errorf("failed to set filter")
	}
	return nil
}

func (s *Sniffer) StartPcapDump(filename string) error {
	cFilename := C.CString(filename)
	defer C.free(unsafe.Pointer(cFilename))

	ret := C.pcap_start_dump(s.handle, cFilename)
	if ret != 0 {
		return fmt.Errorf("failed to start PCAP dump")
	}
	return nil
}

func (s *Sniffer) StopPcapDump() {
	if s.handle != nil {
		C.pcap_stop_dump(s.handle)
	}
}

func (s *Sniffer) GetStats() *SnifferStats {
	var cStats C.sniffer_stats_t
	C.sniffer_get_stats(s.handle, &cStats)

	return &SnifferStats{
		TotalPackets:   uint64(cStats.total_packets),
		TotalBytes:     uint64(cStats.total_bytes),
		TcpPackets:     uint64(cStats.tcp_packets),
		UdpPackets:     uint64(cStats.udp_packets),
		HttpPackets:    uint64(cStats.http_packets),
		HttpsPackets:   uint64(cStats.https_packets),
		DnsPackets:     uint64(cStats.dns_packets),
		DroppedPackets: uint64(cStats.dropped_packets),
	}
}
