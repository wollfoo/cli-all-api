#!/bin/bash
# =============================================================================
# ProxyPal RPM Package Build Script
# Build RPM package cho RHEL/CentOS/Fedora deployment
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
SPEC_FILE="$PROJECT_ROOT/packaging/rpm/proxypal.spec"
DIST_DIR="$PROJECT_ROOT/dist"
BINARY_SRC="$DIST_DIR/linux-x86_64/proxypal"
CLIPROXYAPI_SRC="$PROJECT_ROOT/src-tauri/binaries/cliproxyapi-x86_64-unknown-linux-gnu"
SYSTEMD_SRC="$PROJECT_ROOT/deployment/proxypal.service"
MANPAGE_SRC="$PROJECT_ROOT/packaging/deb/usr/share/man/man1/proxypal.1.gz"

# Version từ Cargo.toml
VERSION=$(grep '^version' "$PROJECT_ROOT/src-headless/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

# RPM build directories
RPMBUILD_DIR="$HOME/rpmbuild"

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  ProxyPal RPM Package Builder${NC}"
echo -e "${BLUE}  Version: ${VERSION}${NC}"
echo -e "${BLUE}============================================${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 1: Kiểm tra Prerequisites
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[1/5] Kiểm tra prerequisites...${NC}"

# Kiểm tra rpmbuild
if ! command -v rpmbuild &> /dev/null; then
    echo -e "${RED}Error: rpmbuild không được cài đặt.${NC}"
    echo -e "${YELLOW}Cài đặt:${NC}"
    echo -e "  RHEL/CentOS: sudo yum install rpm-build"
    echo -e "  Fedora:      sudo dnf install rpm-build"
    echo -e "  Ubuntu:      sudo apt-get install rpm"
    exit 1
fi

# Kiểm tra binary đã build
if [[ ! -f "$BINARY_SRC" ]]; then
    echo -e "${RED}Error: Binary không tìm thấy tại ${BINARY_SRC}${NC}"
    echo -e "${YELLOW}Chạy trước: ./scripts/build-linux.sh${NC}"
    exit 1
fi

# Kiểm tra spec file
if [[ ! -f "$SPEC_FILE" ]]; then
    echo -e "${RED}Error: Spec file không tìm thấy tại ${SPEC_FILE}${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 2: Setup RPM Build Directory Structure
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[2/5] Setting up rpmbuild directory...${NC}"

# Tạo cấu trúc thư mục rpmbuild chuẩn
mkdir -p "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

echo -e "${GREEN}✓ RPM build directories created${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 3: Copy Source Files
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[3/5] Copying source files...${NC}"

# Copy binary
cp "$BINARY_SRC" "$RPMBUILD_DIR/SOURCES/proxypal"
echo "  - proxypal binary"

# Copy CLIProxyAPI nếu tồn tại
if [[ -f "$CLIPROXYAPI_SRC" ]]; then
    cp "$CLIPROXYAPI_SRC" "$RPMBUILD_DIR/SOURCES/cliproxyapi"
    echo "  - cliproxyapi binary"
else
    # Tạo placeholder để spec file không fail
    touch "$RPMBUILD_DIR/SOURCES/cliproxyapi"
    echo "  - cliproxyapi placeholder (binary not found)"
fi

# Copy systemd service
cp "$SYSTEMD_SRC" "$RPMBUILD_DIR/SOURCES/proxypal.service"
echo "  - proxypal.service"

# Copy man page (compressed)
if [[ -f "$MANPAGE_SRC" ]]; then
    cp "$MANPAGE_SRC" "$RPMBUILD_DIR/SOURCES/proxypal.1.gz"
    echo "  - proxypal.1.gz (man page)"
elif [[ -f "$PROJECT_ROOT/packaging/deb/usr/share/man/man1/proxypal.1" ]]; then
    gzip -c "$PROJECT_ROOT/packaging/deb/usr/share/man/man1/proxypal.1" > "$RPMBUILD_DIR/SOURCES/proxypal.1.gz"
    echo "  - proxypal.1.gz (compressed from source)"
else
    touch "$RPMBUILD_DIR/SOURCES/proxypal.1.gz"
    echo "  - proxypal.1.gz placeholder"
fi

# Copy spec file
cp "$SPEC_FILE" "$RPMBUILD_DIR/SPECS/proxypal.spec"
echo "  - proxypal.spec"

echo -e "${GREEN}✓ Source files copied${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 4: Update Spec File Version
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[4/5] Updating spec file version...${NC}"

# Update version trong spec file
sed -i "s/^Version:.*/Version:        ${VERSION}/" "$RPMBUILD_DIR/SPECS/proxypal.spec"

echo -e "${GREEN}✓ Version updated to ${VERSION}${NC}"
echo

# -----------------------------------------------------------------------------
# Bước 5: Build RPM Package
# -----------------------------------------------------------------------------
echo -e "${YELLOW}[5/5] Building RPM package...${NC}"

# Build RPM (binary only, no source)
rpmbuild -bb "$RPMBUILD_DIR/SPECS/proxypal.spec" \
    --define "_topdir $RPMBUILD_DIR" \
    --define "_sourcedir $RPMBUILD_DIR/SOURCES"

# Find built RPM
RPM_OUTPUT=$(find "$RPMBUILD_DIR/RPMS" -name "proxypal-*.rpm" -type f | head -1)

if [[ -z "$RPM_OUTPUT" ]]; then
    echo -e "${RED}Error: RPM package not found after build${NC}"
    exit 1
fi

# Copy to dist directory
mkdir -p "$DIST_DIR"
cp "$RPM_OUTPUT" "$DIST_DIR/"
RPM_FINAL="$DIST_DIR/$(basename "$RPM_OUTPUT")"

# Show package info
echo ""
echo -e "${GREEN}Package info:${NC}"
rpm -qip "$RPM_FINAL"

# Package size
PACKAGE_SIZE=$(du -h "$RPM_FINAL" | cut -f1)

echo ""
echo -e "${BLUE}============================================${NC}"
echo -e "${GREEN}  BUILD SUCCESSFUL!${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""
echo -e "Package: ${GREEN}${RPM_FINAL}${NC}"
echo -e "Size:    ${GREEN}${PACKAGE_SIZE}${NC}"
echo ""
echo -e "Install: ${YELLOW}sudo rpm -i ${RPM_FINAL}${NC}"
echo -e "Or:      ${YELLOW}sudo dnf install ${RPM_FINAL}${NC}"
echo ""
