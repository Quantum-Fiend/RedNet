# RedNet - Quick Start Guide

## Prerequisites

### Option 1: Docker (Recommended)
- Docker Desktop 20.10+
- Docker Compose 2.0+
- 8GB RAM minimum
- 20GB free disk space

### Option 2: Manual Installation
- **C Compiler**: GCC 9+ or Clang 10+
- **Rust**: 1.75+
- **Go**: 1.21+
- **Python**: 3.11+
- **Node.js**: 20+
- **Make**: GNU Make 4.0+

---

## Docker Installation (Easiest)

### 1. Clone and Start

```bash
git clone https://github.com/yourusername/RedNet.git
cd RedNet
docker-compose up -d
```

### 2. Verify Services

```bash
docker-compose ps
```

You should see all services running:
- `c-sniffer` - C packet sniffer library
- `rust-crypto` - Rust encryption engine
- `go-cli` - Go CLI orchestrator
- `python-payload` - Python payload generator
- `web-dashboard` - Next.js frontend
- `backend` - Express + Socket.IO backend
- `postgres` - PostgreSQL database
- `redis` - Redis cache

### 3. Access the Dashboard

Open your browser to: **http://localhost:3000**

Default credentials:
- Username: `admin`
- Password: `admin123`

### 4. Use the CLI

```bash
# Enter the Go CLI container
docker-compose exec go-cli sh

# Run commands
rednet --help
rednet capture -i eth0 -c 10
rednet encrypt -i /tmp/test.txt -o /tmp/test.enc
```

---

## Manual Installation

### 1. Build C Sniffer

```bash
cd c-sniffer
make all
cd ..
```

### 2. Build Rust Crypto Engine

```bash
cd rust-crypto
cargo build --release
cd ..
```

### 3. Build Go CLI

```bash
cd go-cli
go mod download
go build -o rednet ./cmd/rednet
cd ..
```

### 4. Install Python Payload Generator

```bash
cd python-payload
pip install -e .
cd ..
```

### 5. Run Web Dashboard

Terminal 1 (Backend):
```bash
cd web-dashboard
npm install
npm run backend
```

Terminal 2 (Frontend):
```bash
cd web-dashboard
npm run dev
```

Access: **http://localhost:3000**

---

## Common Commands

### Packet Capture
```bash
# Capture 100 packets
rednet capture -i eth0 -c 100 -o capture.pcap

# With filter
rednet capture -i eth0 -f "tcp port 443"
```

### Encryption
```bash
# Encrypt file
rednet encrypt -i secret.txt -o secret.enc -a aes-gcm

# Decrypt file
rednet decrypt -i secret.enc -o decrypted.txt
```

### Payload Generation
```bash
# Generate test payload
rednet-payload generate -t benign_test -o test.bin

# Encode payload
rednet-payload encode input.bin -o encoded.bin -e xor -e base64

# Sandbox detection
rednet-payload detect
```

---

## Troubleshooting

### Docker Issues

**Services won't start:**
```bash
docker-compose down
docker-compose up -d --build
```

**Permission denied (packet capture):**
```bash
# Run with host network mode (Linux only)
docker-compose up -d
```

### Manual Build Issues

**C Sniffer compilation fails:**
- Install libpcap: `sudo apt-get install libpcap-dev` (Linux) or `brew install libpcap` (macOS)

**Rust build fails:**
- Update Rust: `rustup update`
- Clear cache: `cargo clean`

**Go build fails:**
- Ensure CGO is enabled: `export CGO_ENABLED=1`
- Install build tools: `sudo apt-get install build-essential`

**Web dashboard errors:**
- Clear node_modules: `rm -rf node_modules && npm install`
- Check Node version: `node --version` (should be 20+)

---

## Next Steps

1. ✅ Explore the web dashboard
2. ✅ Try packet capture commands
3. ✅ Test encryption/decryption
4. ✅ Generate test payloads
5. ✅ Review the [Architecture Guide](ARCHITECTURE.md)
6. ✅ Check out the [API Documentation](API.md)

---

## Support

- 📖 [Full Documentation](docs/)
- 🐛 [Report Issues](https://github.com/yourusername/RedNet/issues)
- 💬 [Discussions](https://github.com/yourusername/RedNet/discussions)
