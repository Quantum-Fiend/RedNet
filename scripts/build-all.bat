@echo off
REM RedNet Build Script for Windows

echo ========================================
echo RedNet Multi-Language Build Script
echo ========================================

REM Build C Sniffer
echo.
echo [1/5] Building C Packet Sniffer...
cd c-sniffer
where make >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    make clean
    make all
    echo [OK] C Sniffer built successfully
) else (
    echo [SKIP] Make not found, skipping C build
)
cd ..

REM Build Rust Crypto Engine
echo.
echo [2/5] Building Rust Encryption Engine...
cd rust-crypto
where cargo >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    cargo clean
    cargo build --release
    cargo test --release
    echo [OK] Rust Crypto built successfully
) else (
    echo [SKIP] Cargo not found, skipping Rust build
)
cd ..

REM Build Go CLI
echo.
echo [3/5] Building Go CLI...
cd go-cli
where go >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    go mod tidy
    go build -o rednet.exe ./cmd/rednet
    echo [OK] Go CLI built successfully
) else (
    echo [SKIP] Go not found, skipping Go build
)
cd ..

REM Install Python Payload Generator
echo.
echo [4/5] Installing Python Payload Generator...
cd python-payload
where pip >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    pip install -e . --quiet
    echo [OK] Python Payload installed successfully
) else (
    echo [SKIP] Pip not found, skipping Python install
)
cd ..

REM Build Web Dashboard
echo.
echo [5/5] Building Web Dashboard...
cd web-dashboard
where npm >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    npm install --silent
    echo [OK] Web Dashboard dependencies installed
) else (
    echo [SKIP] npm not found, skipping Web Dashboard
)
cd ..

echo.
echo ========================================
echo Build Complete!
echo ========================================
echo.
echo Next steps:
echo   1. Run with Docker: docker-compose up -d
echo   2. Access dashboard: http://localhost:3000
echo   3. Use CLI: go-cli\rednet.exe --help
echo.
pause
