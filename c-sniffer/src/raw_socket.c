#include "../include/packet_sniffer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef _WIN32
    #include <winsock2.h>
    #include <ws2tcpip.h>
    #include <windows.h>
    #pragma comment(lib, "ws2_32.lib")
#else
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <netinet/ip.h>
    #include <netinet/tcp.h>
    #include <netinet/udp.h>
    #include <arpa/inet.h>
    #include <unistd.h>
    #include <sys/ioctl.h>
    #include <net/if.h>
    #include <linux/if_packet.h>
    #include <net/ethernet.h>
#endif

#define RING_BUFFER_SIZE 4096
#define MAX_PACKET_SIZE 65535

// Ring buffer for zero-copy packet handling
typedef struct {
    packet_t packets[RING_BUFFER_SIZE];
    uint8_t* data_buffers[RING_BUFFER_SIZE];
    uint32_t head;
    uint32_t tail;
    uint32_t count;
} ring_buffer_t;

struct sniffer_s {
    int socket_fd;
    char interface[256];
    bool running;
    ring_buffer_t ring_buffer;
    packet_callback_t callback;
    void* user_data;
    FILE* pcap_file;
    sniffer_stats_t stats;
    filter_t* filter;
};

// Initialize ring buffer
static void ring_buffer_init(ring_buffer_t* rb) {
    rb->head = 0;
    rb->tail = 0;
    rb->count = 0;
    for (int i = 0; i < RING_BUFFER_SIZE; i++) {
        rb->data_buffers[i] = (uint8_t*)malloc(MAX_PACKET_SIZE);
    }
}

// Cleanup ring buffer
static void ring_buffer_cleanup(ring_buffer_t* rb) {
    for (int i = 0; i < RING_BUFFER_SIZE; i++) {
        free(rb->data_buffers[i]);
    }
}

// Add packet to ring buffer
static bool ring_buffer_push(ring_buffer_t* rb, const uint8_t* data, uint32_t length) {
    if (rb->count >= RING_BUFFER_SIZE) {
        return false; // Buffer full
    }
    
    uint32_t index = rb->tail;
    memcpy(rb->data_buffers[index], data, length);
    rb->packets[index].data = rb->data_buffers[index];
    rb->packets[index].length = length;
    rb->packets[index].timestamp = (uint64_t)time(NULL);
    
    rb->tail = (rb->tail + 1) % RING_BUFFER_SIZE;
    rb->count++;
    return true;
}

// Get packet from ring buffer
static packet_t* ring_buffer_pop(ring_buffer_t* rb) {
    if (rb->count == 0) {
        return NULL;
    }
    
    uint32_t index = rb->head;
    rb->head = (rb->head + 1) % RING_BUFFER_SIZE;
    rb->count--;
    return &rb->packets[index];
}

sniffer_t* sniffer_create(const char* interface_name) {
    sniffer_t* sniffer = (sniffer_t*)calloc(1, sizeof(sniffer_t));
    if (!sniffer) return NULL;
    
    strncpy(sniffer->interface, interface_name ? interface_name : "any", sizeof(sniffer->interface) - 1);
    sniffer->running = false;
    sniffer->pcap_file = NULL;
    sniffer->filter = NULL;
    
    ring_buffer_init(&sniffer->ring_buffer);
    memset(&sniffer->stats, 0, sizeof(sniffer_stats_t));
    
#ifdef _WIN32
    WSADATA wsa;
    if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0) {
        free(sniffer);
        return NULL;
    }
    
    sniffer->socket_fd = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
    if (sniffer->socket_fd == INVALID_SOCKET) {
        WSACleanup();
        free(sniffer);
        return NULL;
    }
    
    // Enable promiscuous mode on Windows
    DWORD dwValue = 1;
    if (ioctlsocket(sniffer->socket_fd, SIO_RCVALL, &dwValue) == SOCKET_ERROR) {
        closesocket(sniffer->socket_fd);
        WSACleanup();
        free(sniffer);
        return NULL;
    }
#else
    // Linux raw socket
    sniffer->socket_fd = socket(AF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
    if (sniffer->socket_fd < 0) {
        free(sniffer);
        return NULL;
    }
    
    // Set promiscuous mode
    struct ifreq ifr;
    memset(&ifr, 0, sizeof(ifr));
    strncpy(ifr.ifr_name, sniffer->interface, IFNAMSIZ - 1);
    if (ioctl(sniffer->socket_fd, SIOCGIFFLAGS, &ifr) == 0) {
        ifr.ifr_flags |= IFF_PROMISC;
        ioctl(sniffer->socket_fd, SIOCSIFFLAGS, &ifr);
    }
#endif
    
    return sniffer;
}

void sniffer_destroy(sniffer_t* sniffer) {
    if (!sniffer) return;
    
    if (sniffer->running) {
        sniffer_stop(sniffer);
    }
    
    if (sniffer->pcap_file) {
        pcap_stop_dump(sniffer);
    }
    
#ifdef _WIN32
    closesocket(sniffer->socket_fd);
    WSACleanup();
#else
    close(sniffer->socket_fd);
#endif
    
    ring_buffer_cleanup(&sniffer->ring_buffer);
    free(sniffer);
}

int sniffer_start(sniffer_t* sniffer, packet_callback_t callback, void* user_data) {
    if (!sniffer || sniffer->running) return -1;
    
    sniffer->callback = callback;
    sniffer->user_data = user_data;
    sniffer->running = true;
    
    uint8_t buffer[MAX_PACKET_SIZE];
    
    while (sniffer->running) {
        int recv_len = recv(sniffer->socket_fd, (char*)buffer, MAX_PACKET_SIZE, 0);
        
        if (recv_len > 0) {
            sniffer->stats.total_packets++;
            sniffer->stats.total_bytes += recv_len;
            
            // Add to ring buffer
            if (ring_buffer_push(&sniffer->ring_buffer, buffer, recv_len)) {
                packet_t* pkt = ring_buffer_pop(&sniffer->ring_buffer);
                if (pkt) {
                    // Parse protocol
                    pkt->protocol = parse_packet_protocol(pkt->data, pkt->length);
                    
                    // Extract IP addresses and ports (simplified)
                    if (pkt->length >= 20) {
                        struct iphdr* ip = (struct iphdr*)(pkt->data + 14); // Skip Ethernet header
                        inet_ntop(AF_INET, &ip->saddr, pkt->src_ip, sizeof(pkt->src_ip));
                        inet_ntop(AF_INET, &ip->daddr, pkt->dst_ip, sizeof(pkt->dst_ip));
                        
                        if (ip->protocol == IPPROTO_TCP) {
                            sniffer->stats.tcp_packets++;
                            struct tcphdr* tcp = (struct tcphdr*)((uint8_t*)ip + (ip->ihl * 4));
                            pkt->src_port = ntohs(tcp->source);
                            pkt->dst_port = ntohs(tcp->dest);
                        } else if (ip->protocol == IPPROTO_UDP) {
                            sniffer->stats.udp_packets++;
                            struct udphdr* udp = (struct udphdr*)((uint8_t*)ip + (ip->ihl * 4));
                            pkt->src_port = ntohs(udp->source);
                            pkt->dst_port = ntohs(udp->dest);
                        }
                    }
                    
                    // Write to PCAP if enabled
                    if (sniffer->pcap_file) {
                        fwrite(pkt->data, 1, pkt->length, sniffer->pcap_file);
                    }
                    
                    // Callback
                    if (callback) {
                        callback(pkt, user_data);
                    }
                }
            } else {
                sniffer->stats.dropped_packets++;
            }
        }
    }
    
    return 0;
}

void sniffer_stop(sniffer_t* sniffer) {
    if (sniffer) {
        sniffer->running = false;
    }
}

const char* protocol_to_string(protocol_type_t proto) {
    switch (proto) {
        case PROTO_ETHERNET: return "Ethernet";
        case PROTO_IP: return "IP";
        case PROTO_TCP: return "TCP";
        case PROTO_UDP: return "UDP";
        case PROTO_ICMP: return "ICMP";
        case PROTO_HTTP: return "HTTP";
        case PROTO_HTTPS: return "HTTPS";
        case PROTO_DNS: return "DNS";
        case PROTO_TLS: return "TLS";
        case PROTO_QUIC: return "QUIC";
        default: return "Unknown";
    }
}

protocol_type_t parse_packet_protocol(const uint8_t* data, uint32_t length) {
    if (length < 14) return PROTO_UNKNOWN;
    
    // Check Ethernet type
    uint16_t eth_type = (data[12] << 8) | data[13];
    if (eth_type == 0x0800) { // IPv4
        if (length < 34) return PROTO_IP;
        
        uint8_t ip_proto = data[23];
        if (ip_proto == 6) return PROTO_TCP;
        if (ip_proto == 17) return PROTO_UDP;
        if (ip_proto == 1) return PROTO_ICMP;
    }
    
    return PROTO_UNKNOWN;
}

void sniffer_get_stats(sniffer_t* sniffer, sniffer_stats_t* stats) {
    if (sniffer && stats) {
        memcpy(stats, &sniffer->stats, sizeof(sniffer_stats_t));
    }
}

int pcap_start_dump(sniffer_t* sniffer, const char* filename) {
    if (!sniffer || sniffer->pcap_file) return -1;
    
    sniffer->pcap_file = fopen(filename, "wb");
    if (!sniffer->pcap_file) return -1;
    
    // Write PCAP global header
    uint32_t magic = 0xa1b2c3d4;
    uint16_t version_major = 2;
    uint16_t version_minor = 4;
    uint32_t thiszone = 0;
    uint32_t sigfigs = 0;
    uint32_t snaplen = 65535;
    uint32_t network = 1; // Ethernet
    
    fwrite(&magic, 4, 1, sniffer->pcap_file);
    fwrite(&version_major, 2, 1, sniffer->pcap_file);
    fwrite(&version_minor, 2, 1, sniffer->pcap_file);
    fwrite(&thiszone, 4, 1, sniffer->pcap_file);
    fwrite(&sigfigs, 4, 1, sniffer->pcap_file);
    fwrite(&snaplen, 4, 1, sniffer->pcap_file);
    fwrite(&network, 4, 1, sniffer->pcap_file);
    
    return 0;
}

void pcap_stop_dump(sniffer_t* sniffer) {
    if (sniffer && sniffer->pcap_file) {
        fclose(sniffer->pcap_file);
        sniffer->pcap_file = NULL;
    }
}
