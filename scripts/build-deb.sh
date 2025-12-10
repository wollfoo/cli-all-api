#!/bin/bash
# =============================================================================
# ProxyPal DEB Package Build Script
# Build Debian package cho Ubuntu deployment
# =============================================================================

set -e

# Màu cho output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Đường dẫn
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PACKAGE_DIR="$PROJECT_ROOT/packaging/deb"
DIST_DIR="$PROJECT_ROOT/dist"
BINARY_SRC="$DIST_DIR/linux-x86_64/proxypal"
SYSTEMD_SRC="$PROJECT_ROOT/deployment/proxypal.service"
CLIPROXYAPI_SRC="$PROJECT_ROOT/src-tauri/binaries/cliproxyapi-x86_64-unknown-linux-gnu"

# Version từ Cargo.toml
VERSION=$(grep '^version' "$PROJECT_ROOT/src-headless/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
PACKAGE_NAME="proxypal_${VERSION}_amd64"

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  ProxyPal DEB Package Builder${NC}"
echo -e "${BLUE}  Version: ${VERSION}${NC}"
echo -e "${BLUE}============================================${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 1: Kiểm tra Prerequisites
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[1/6] Kiểm tra prerequisites...${NC}"

# Kiểm tra dpkg-deb
if ! command -v dpkg-deb &> /dev/null; then
    echo -e "${RED}Error: dpkg-deb không được cài đặt.${NC}"
    echo -e "${YELLOW}Cài đặt: sudo apt-get install dpkg${NC}"
    exit 1
fi

# Kiểm tra binary đã build
if [[ ! -f "$BINARY_SRC" ]]; then
    echo -e "${RED}Error: Binary không tìm thấy tại ${BINARY_SRC}${NC}"
    echo -e "${YELLOW}Chạy trước: ./scripts/build-linux.sh${NC}"
    exit 1
fi

# Kiểm tra systemd service file
if [[ ! -f "$SYSTEMD_SRC" ]]; then
    echo -e "${RED}Error: Systemd service file không tìm thấy tại ${SYSTEMD_SRC}${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 2: Chuẩn bị Package Directory
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[2/6] Chuẩn bị package directory...${NC}"

# Tạo cấu trúc thư mục
mkdir -p "$PACKAGE_DIR/DEBIAN"
mkdir -p "$PACKAGE_DIR/usr/local/bin"
mkdir -p "$PACKAGE_DIR/usr/share/man/man1"
mkdir -p "$PACKAGE_DIR/lib/systemd/system"
mkdir -p "$PACKAGE_DIR/etc/proxypal"

echo -e "${GREEN}✓ Directory structure created${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 3: Copy Files
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[3/6] Copying files...${NC}"

# Copy ProxyPal binary
cp "$BINARY_SRC" "$PACKAGE_DIR/usr/local/bin/proxypal"
echo "  - proxypal binary"

# Copy CLIProxyAPI nếu tồn tại
if [[ -f "$CLIPROXYAPI_SRC" ]]; then
    cp "$CLIPROXYAPI_SRC" "$PACKAGE_DIR/usr/local/bin/cliproxyapi"
    echo "  - cliproxyapi binary"
fi

# Copy systemd service
cp "$SYSTEMD_SRC" "$PACKAGE_DIR/lib/systemd/system/proxypal.service"
echo "  - proxypal.service"

# Man page đã được tạo sẵn trong packaging/deb/usr/share/man/man1/
# Compress man page
if [[ -f "$PACKAGE_DIR/usr/share/man/man1/proxypal.1" ]]; then
    gzip -f "$PACKAGE_DIR/usr/share/man/man1/proxypal.1"
    echo "  - proxypal.1.gz (man page)"
fi

echo -e "${GREEN}✓ Files copied${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 4: Set Permissions
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[4/6] Setting permissions...${NC}"

# Binaries: 755
chmod 755 "$PACKAGE_DIR/usr/local/bin/proxypal"
[[ -f "$PACKAGE_DIR/usr/local/bin/cliproxyapi" ]] && chmod 755 "$PACKAGE_DIR/usr/local/bin/cliproxyapi"

# Service file: 644
chmod 644 "$PACKAGE_DIR/lib/systemd/system/proxypal.service"

# Man page: 644
[[ -f "$PACKAGE_DIR/usr/share/man/man1/proxypal.1.gz" ]] && chmod 644 "$PACKAGE_DIR/usr/share/man/man1/proxypal.1.gz"

# DEBIAN scripts: 755
chmod 755 "$PACKAGE_DIR/DEBIAN/postinst"
chmod 755 "$PACKAGE_DIR/DEBIAN/prerm"
chmod 644 "$PACKAGE_DIR/DEBIAN/control"
chmod 644 "$PACKAGE_DIR/DEBIAN/conffiles"

# Directory permissions
find "$PACKAGE_DIR" -type d -exec chmod 755 {} \;

echo -e "${GREEN}✓ Permissions set${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 5: Update control file với Installed-Size
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[5/6] Updating control file...${NC}"

# Tính Installed-Size (KB)
INSTALLED_SIZE=$(du -sk "$PACKAGE_DIR" | cut -f1)

# Update Installed-Size trong control file
sed -i "s/^Installed-Size:.*/Installed-Size: ${INSTALLED_SIZE}/" "$PACKAGE_DIR/DEBIAN/control"

# Update Version
sed -i "s/^Version:.*/Version: ${VERSION}/" "$PACKAGE_DIR/DEBIAN/control"

echo -e "${GREEN}✓ Control file updated (Installed-Size: ${INSTALLED_SIZE}K)${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 6: Build DEB Package
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[6/6] Building DEB package...${NC}"

# Build package
DEB_OUTPUT="$DIST_DIR/${PACKAGE_NAME}.deb"
dpkg-deb --build --root-owner-group "$PACKAGE_DIR" "$DEB_OUTPUT"

# Verify package
echo ""
echo -e "${GREEN}Package info:${NC}"
dpkg-deb --info "$DEB_OUTPUT"

# Package size
PACKAGE_SIZE=$(du -h "$DEB_OUTPUT" | cut -f1)

echo ""
echo -e "${BLUE}============================================${NC}"
echo -e "${GREEN}  BUILD SUCCESSFUL!${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""
echo -e "Package: ${GREEN}${DEB_OUTPUT}${NC}"
echo -e "Size:    ${GREEN}${PACKAGE_SIZE}${NC}"
echo ""
echo -e "Install:     ${YELLOW}sudo dpkg -i ${DEB_OUTPUT}${NC}"
echo -e "Or:          ${YELLOW}sudo apt install ./${PACKAGE_NAME}.deb${NC}"
echo ""
