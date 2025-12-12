#ifndef PACKET_SNIFFER_H
#define PACKET_SNIFFER_H

#include <stdint.h>
#include <stdbool.h>

#ifdef _WIN32
    #ifdef BUILDING_DLL
        #define DLL_EXPORT __declspec(dllexport)
    #else
        #define DLL_EXPORT __declspec(dllimport)
    #endif
#else
    #define DLL_EXPORT
#endif

// Protocol types
typedef enum {
    PROTO_UNKNOWN = 0,
    PROTO_ETHERNET,
    PROTO_IP,
    PROTO_TCP,
    PROTO_UDP,
    PROTO_ICMP,
    PROTO_HTTP,
    PROTO_HTTPS,
    PROTO_DNS,
    PROTO_TLS,
    PROTO_QUIC
} protocol_type_t;

// Packet structure
typedef struct {
    uint8_t* data;
    uint32_t length;
    uint64_t timestamp;
    protocol_type_t protocol;
    char src_ip[46];  // IPv6 compatible
    char dst_ip[46];
    uint16_t src_port;
    uint16_t dst_port;
} packet_t;

// Filter structure
typedef struct filter_s filter_t;

// Callback for packet processing
typedef void (*packet_callback_t)(const packet_t* packet, void* user_data);

// Sniffer handle
typedef struct sniffer_s sniffer_t;

// Initialize sniffer
DLL_EXPORT sniffer_t* sniffer_create(const char* interface_name);

// Destroy sniffer
DLL_EXPORT void sniffer_destroy(sniffer_t* sniffer);

// Start capture
DLL_EXPORT int sniffer_start(sniffer_t* sniffer, packet_callback_t callback, void* user_data);

// Stop capture
DLL_EXPORT void sniffer_stop(sniffer_t* sniffer);

// Apply filter
DLL_EXPORT int sniffer_set_filter(sniffer_t* sniffer, const char* filter_expr);

// PCAP operations
DLL_EXPORT int pcap_start_dump(sniffer_t* sniffer, const char* filename);
DLL_EXPORT void pcap_stop_dump(sniffer_t* sniffer);

// Protocol parsing
DLL_EXPORT const char* protocol_to_string(protocol_type_t proto);
DLL_EXPORT protocol_type_t parse_packet_protocol(const uint8_t* data, uint32_t length);

// Statistics
typedef struct {
    uint64_t total_packets;
    uint64_t total_bytes;
    uint64_t tcp_packets;
    uint64_t udp_packets;
    uint64_t http_packets;
    uint64_t https_packets;
    uint64_t dns_packets;
    uint64_t dropped_packets;
} sniffer_stats_t;

DLL_EXPORT void sniffer_get_stats(sniffer_t* sniffer, sniffer_stats_t* stats);

#endif // PACKET_SNIFFER_H
