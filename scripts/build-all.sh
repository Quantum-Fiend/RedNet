#!/bin/bash
# RedNet Build Script - Builds all components

set -e

echo "🚀 RedNet Multi-Language Build Script"
echo "======================================"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Build C Sniffer
echo -e "\n${BLUE}[1/5] Building C Packet Sniffer...${NC}"
cd c-sniffer
if command -v make &> /dev/null; then
    make clean
    make all
    echo -e "${GREEN}✓ C Sniffer built successfully${NC}"
else
    echo -e "${RED}✗ Make not found, skipping C build${NC}"
fi
cd ..

# Build Rust Crypto Engine
echo -e "\n${BLUE}[2/5] Building Rust Encryption Engine...${NC}"
cd rust-crypto
if command -v cargo &> /dev/null; then
    cargo clean
    cargo build --release
    cargo test --release
    echo -e "${GREEN}✓ Rust Crypto built successfully${NC}"
else
    echo -e "${RED}✗ Cargo not found, skipping Rust build${NC}"
fi
cd ..

# Build Go CLI
echo -e "\n${BLUE}[3/5] Building Go CLI...${NC}"
cd go-cli
if command -v go &> /dev/null; then
    go mod tidy
    go build -o rednet ./cmd/rednet
    echo -e "${GREEN}✓ Go CLI built successfully${NC}"
else
    echo -e "${RED}✗ Go not found, skipping Go build${NC}"
fi
cd ..

# Install Python Payload Generator
echo -e "\n${BLUE}[4/5] Installing Python Payload Generator...${NC}"
cd python-payload
if command -v pip &> /dev/null; then
    pip install -e . --quiet
    echo -e "${GREEN}✓ Python Payload installed successfully${NC}"
else
    echo -e "${RED}✗ Pip not found, skipping Python install${NC}"
fi
cd ..

# Build Web Dashboard
echo -e "\n${BLUE}[5/5] Building Web Dashboard...${NC}"
cd web-dashboard
if command -v npm &> /dev/null; then
    npm install --silent
    echo -e "${GREEN}✓ Web Dashboard dependencies installed${NC}"
else
    echo -e "${RED}✗ npm not found, skipping Web Dashboard${NC}"
fi
cd ..

echo -e "\n${GREEN}======================================"
echo -e "✅ Build Complete!"
echo -e "======================================${NC}"
echo ""
echo "Next steps:"
echo "  1. Run with Docker: docker-compose up -d"
echo "  2. Access dashboard: http://localhost:3000"
echo "  3. Use CLI: ./go-cli/rednet --help"
echo ""
