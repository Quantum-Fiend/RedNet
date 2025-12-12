# RedNet Demo Script

This script guides you through a complete demonstration of RedNet's capabilities.

## Prerequisites

- Docker and Docker Compose installed
- Terminal access
- Web browser

## Demo Flow (15 minutes)

### Part 1: Setup (2 minutes)

```bash
# Clone and start services
cd RedNet
docker-compose up -d

# Verify all services are running
docker-compose ps
```

**Expected Output**: 8 services running (c-sniffer, rust-crypto, go-cli, python-payload, web-dashboard, backend, postgres, redis)

---

### Part 2: Web Dashboard (3 minutes)

1. **Open Dashboard**
   - Navigate to: http://localhost:3000
   - Login: `admin` / `admin123`

2. **Explore Features**
   - View real-time statistics (packets, agents, threats, bandwidth)
   - Check network telemetry charts
   - Observe protocol distribution pie chart
   - Monitor bandwidth utilization
   - Review agent status cards

3. **Highlight Points**
   - Glassmorphism UI with neon accents
   - Real-time WebSocket updates (every 3 seconds)
   - Responsive design
   - Cybersecurity-themed dark mode

---

### Part 3: Go CLI - Encryption (3 minutes)

```bash
# Enter Go CLI container
docker-compose exec go-cli sh

# Create test file
echo "RedNet Cybersecurity Toolkit - Secret Data" > /tmp/secret.txt

# Encrypt with AES-256-GCM
rednet encrypt -i /tmp/secret.txt -o /tmp/secret.enc -a aes-gcm

# View generated files
ls -lh /tmp/secret.*

# Decrypt
rednet decrypt -i /tmp/secret.enc -o /tmp/decrypted.txt

# Verify
cat /tmp/decrypted.txt
```

**Highlight Points**:
- Automatic key and nonce generation
- Secure key storage
- File size comparison (plaintext vs ciphertext)
- Round-trip encryption/decryption

---

### Part 4: Python Payload Generator (3 minutes)

```bash
# Enter Python container
docker-compose exec python-payload sh

# Generate benign test payload
rednet-payload generate -t benign_test -o /tmp/test.bin -m "RedNet Test"

# View payload
hexdump -C /tmp/test.bin | head

# Encode with XOR + Base64
rednet-payload encode /tmp/test.bin -o /tmp/encoded.bin -e xor -e base64

# Run sandbox detection
rednet-payload detect
```

**Highlight Points**:
- Multiple payload types (benign, shellcode, script, binary)
- Polymorphic encoding chains
- VM and debugger detection
- Sandbox evasion simulation

---

### Part 5: Architecture Overview (2 minutes)

**Show the multi-language integration**:

```
Frontend (TypeScript/Next.js)
    ↓ WebSocket
Backend (Node.js/Express)
    ↓ REST API
Go CLI (Orchestrator)
    ↓ CGO FFI
C Sniffer ← → Rust Crypto
```

**Key Points**:
- 6 programming languages working together
- FFI bindings between C, Rust, and Go
- Real-time communication via WebSockets
- Docker orchestration of 8 services

---

### Part 6: Code Walkthrough (2 minutes)

**Show key files**:

1. **C Packet Sniffer** - `c-sniffer/src/raw_socket.c`
   - Ring buffer implementation
   - Raw socket capture
   - Protocol parsing

2. **Rust Crypto** - `rust-crypto/src/aead.rs`
   - AES-GCM encryption
   - Memory-safe key handling
   - C FFI exports

3. **Go CLI** - `go-cli/internal/commands/encrypt.go`
   - Command implementation
   - FFI integration
   - Error handling

4. **Web Dashboard** - `web-dashboard/src/components/NetworkTelemetry.tsx`
   - Real-time charts
   - Glassmorphism design
   - WebSocket integration

---

## Key Talking Points

### Technical Excellence
- ✅ Systems programming in C (raw sockets, DPI)
- ✅ Memory-safe cryptography in Rust
- ✅ Cross-platform CLI in Go
- ✅ Security automation in Python
- ✅ Modern web stack in TypeScript

### Security Features
- ✅ Modern encryption (AES-GCM, ChaCha20-Poly1305)
- ✅ Digital signatures (Ed25519)
- ✅ Key exchange (X25519)
- ✅ Secure key management (zeroization)
- ✅ JWT authentication

### DevOps
- ✅ Complete Docker orchestration
- ✅ Multi-stage builds
- ✅ Service isolation
- ✅ Database integration (PostgreSQL, Redis)

### Use Cases
- Security Operations Center (SOC) tooling
- Red/blue team exercises
- Network forensics
- Cryptographic operations
- Security training and education

---

## Cleanup

```bash
# Stop all services
docker-compose down

# Remove volumes (optional)
docker-compose down -v
```

---

## Q&A Preparation

**Q: Why six languages?**
A: Each language is chosen for its strengths - C for performance, Rust for safety, Go for concurrency, Python for rapid development, TypeScript for modern web, demonstrating cross-language integration.

**Q: Is this production-ready?**
A: The core implementation is solid, but would need additional testing, security audits, and hardening for production deployment.

**Q: What's the performance like?**
A: The C sniffer can handle high packet rates with zero-copy buffers, Rust crypto is optimized for speed, and the web dashboard uses efficient WebSocket streaming.

**Q: Can this be extended?**
A: Absolutely! The modular architecture allows adding new features, protocols, encryption algorithms, or analysis capabilities.
