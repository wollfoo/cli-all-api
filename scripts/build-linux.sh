#!/bin/bash
# =============================================================================
# ProxyPal Linux Build Script
# Build script cho Linux x86_64 binary
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
DIST_DIR="$PROJECT_ROOT/dist/linux-x86_64"
TARGET="x86_64-unknown-linux-gnu"

# Lấy version từ Cargo.toml
VERSION=$(grep '^version' "$SRC_DIR/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  ProxyPal Linux Build Script${NC}"
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

# Kiểm tra target (chỉ cần nếu cross-compile)
if [[ "$(rustc -vV | grep host | cut -d' ' -f2)" != "$TARGET" ]]; then
    if ! rustup target list --installed | grep -q "$TARGET"; then
        echo -e "${YELLOW}Thêm target ${TARGET}...${NC}"
        rustup target add "$TARGET"
    fi
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 2: Build Release Binary
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[2/5] Building release binary...${NC}"

cd "$SRC_DIR"

# Build với release profile (đã có strip, lto, opt-level=3 trong Cargo.toml)
cargo build --release --target "$TARGET"

echo -e "${GREEN}✓ Build completed${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 3: Tạo Distribution Directory
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[3/5] Tạo distribution directory...${NC}"

mkdir -p "$DIST_DIR"

echo -e "${GREEN}✓ Directory created: ${DIST_DIR}${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 4: Copy Binary và Strip
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[4/5] Copy và optimize binary...${NC}"

BINARY_SRC="$SRC_DIR/target/$TARGET/release/proxypal"
BINARY_DST="$DIST_DIR/proxypal"

if [[ ! -f "$BINARY_SRC" ]]; then
    echo -e "${RED}Error: Binary không tìm thấy tại ${BINARY_SRC}${NC}"
    exit 1
fi

cp "$BINARY_SRC" "$BINARY_DST"

# Strip symbols để giảm kích thước (dự phòng nếu Cargo.toml strip không hoạt động)
if command -v strip &> /dev/null; then
    strip "$BINARY_DST" 2>/dev/null || true
fi

# Đặt quyền thực thi
chmod +x "$BINARY_DST"

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
ProxyPal Headless - Linux Build
================================
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
echo -e "Binary:           ${GREEN}${DIST_DIR}/proxypal${NC}"
echo
echo -e "Để chạy: ${YELLOW}${DIST_DIR}/proxypal --help${NC}"
echo
