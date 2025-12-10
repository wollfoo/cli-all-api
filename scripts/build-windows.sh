#!/bin/bash
# =============================================================================
# ProxyPal Windows Cross-Compilation Build Script
# Build script cho Windows x86_64 binary (cross-compile từ Linux)
# =============================================================================

set -e  # Dừng nếu có lỗi

# Màu cho output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Đường dẫn
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SRC_DIR="$PROJECT_ROOT/src-headless"
DIST_DIR="$PROJECT_ROOT/dist/windows-x86_64"
TARGET="x86_64-pc-windows-gnu"

# Lấy version từ Cargo.toml
VERSION=$(grep '^version' "$SRC_DIR/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  ProxyPal Windows Cross-Compile Script${NC}"
echo -e "${BLUE}  Version: ${VERSION}${NC}"
echo -e "${BLUE}  Target: ${TARGET}${NC}"
echo -e "${BLUE}============================================${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 1: Kiểm tra Prerequisites
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[1/5] Kiểm tra prerequisites...${NC}"

# Kiểm tra Rust toolchain
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo không được cài đặt. Vui lòng cài đặt Rust toolchain.${NC}"
    exit 1
fi

# Kiểm tra mingw-w64 toolchain
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo -e "${RED}Error: mingw-w64 không được cài đặt.${NC}"
    echo -e "${YELLOW}Cài đặt trên Ubuntu/Debian:${NC}"
    echo -e "  sudo apt-get install mingw-w64"
    echo -e "${YELLOW}Cài đặt trên Fedora:${NC}"
    echo -e "  sudo dnf install mingw64-gcc"
    echo -e "${YELLOW}Cài đặt trên Arch:${NC}"
    echo -e "  sudo pacman -S mingw-w64-gcc"
    exit 1
fi

# Kiểm tra và thêm Windows target
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo -e "${YELLOW}Thêm target ${TARGET}...${NC}"
    rustup target add "$TARGET"
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 2: Configure Cross-Compilation
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[2/5] Configuring cross-compilation...${NC}"

# Tạo hoặc cập nhật .cargo/config.toml cho cross-compilation
CARGO_CONFIG_DIR="$SRC_DIR/.cargo"
CARGO_CONFIG_FILE="$CARGO_CONFIG_DIR/config.toml"

mkdir -p "$CARGO_CONFIG_DIR"

# Kiểm tra và thêm linker config nếu chưa có
if [[ ! -f "$CARGO_CONFIG_FILE" ]] || ! grep -q "$TARGET" "$CARGO_CONFIG_FILE"; then
    cat >> "$CARGO_CONFIG_FILE" << 'EOF'

# Windows cross-compilation configuration
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"
EOF
    echo -e "${GREEN}✓ Cross-compilation config added${NC}"
else
    echo -e "${GREEN}✓ Cross-compilation config exists${NC}"
fi
echo

# -----------------------------------------------------------------------------
# Bước 3: Build Release Binary
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[3/5] Building Windows release binary...${NC}"

cd "$SRC_DIR"

# Build với release profile
# Note: Một số features như daemonize có thể không hỗ trợ Windows
# cargo build sẽ bỏ qua các dependencies unix-only theo target cfg
cargo build --release --target "$TARGET"

echo -e "${GREEN}✓ Build completed${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 4: Tạo Distribution Directory
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[4/5] Tạo distribution directory...${NC}"

mkdir -p "$DIST_DIR"

BINARY_SRC="$SRC_DIR/target/$TARGET/release/proxypal.exe"
BINARY_DST="$DIST_DIR/proxypal.exe"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo -e "${RED}Error: Binary không tìm thấy tại ${BINARY_SRC}${NC}"
    exit 1
fi

cp "$BINARY_SRC" "$BINARY_DST"

# Strip symbols để giảm kích thước
if command -v x86_64-w64-mingw32-strip &> /dev/null; then
    x86_64-w64-mingw32-strip "$BINARY_DST" 2>/dev/null || true
fi

# Hiển thị kích thước
BINARY_SIZE=$(du -h "$BINARY_DST" | cut -f1)
echo -e "${GREEN}✓ Binary copied: ${BINARY_DST}${NC}"
echo -e "${GREEN}  Size: ${BINARY_SIZE}${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 5: Tạo Version Info
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[5/5] Tạo version info...${NC}"

BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
BUILD_HOST=$(hostname)
RUST_VERSION=$(rustc --version)
GIT_COMMIT=$(git -C "$PROJECT_ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown")
GIT_BRANCH=$(git -C "$PROJECT_ROOT" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

cat > "$DIST_DIR/VERSION.txt" << EOF
ProxyPal Headless - Windows Build
==================================
Version:      ${VERSION}
Target:       ${TARGET}
Build Date:   ${BUILD_DATE}
Build Host:   ${BUILD_HOST}
Rust Version: ${RUST_VERSION}
Git Commit:   ${GIT_COMMIT}
Git Branch:   ${GIT_BRANCH}
Binary Size:  ${BINARY_SIZE}
EOF

echo -e "${GREEN}✓ Version info created${NC}"
echo

# -----------------------------------------------------------------------------
# Hoàn thành
# -----------------------------------------------------------------------------
echo -e "${BLUE}============================================${NC}"
echo -e "${GREEN}  BUILD SUCCESSFUL!${NC}"
echo -e "${BLUE}============================================${NC}"
echo
echo -e "Output directory: ${GREEN}${DIST_DIR}${NC}"
echo -e "Binary:           ${GREEN}${DIST_DIR}/proxypal.exe${NC}"
echo
echo -e "${YELLOW}Lưu ý: Windows binary này cần được test trên Windows system.${NC}"
echo
