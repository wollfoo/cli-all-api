#!/bin/bash
# ProxyPal Systemd Installation Script
# Script cài đặt ProxyPal như systemd service trên Ubuntu 20.04+
#
# Usage: sudo ./install-systemd.sh [OPTIONS]
#
# Options:
#   --uninstall    Gỡ cài đặt service
#   --status       Kiểm tra trạng thái service
#   --help         Hiển thị hướng dẫn
#
# Sau khi cài đặt, sử dụng các lệnh sau:
#   systemctl start proxypal    - Khởi động service
#   systemctl stop proxypal     - Dừng service  
#   systemctl restart proxypal  - Khởi động lại service
#   systemctl status proxypal   - Kiểm tra trạng thái
#   journalctl -u proxypal -f   - Xem logs theo thời gian thực
#   journalctl -u proxypal --since "1 hour ago"  - Xem logs 1 giờ gần đây

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
BINARY_NAME="proxypal"
BINARY_SOURCE="./target/release/${BINARY_NAME}"
BINARY_DEST="/usr/local/bin/${BINARY_NAME}"
SERVICE_FILE="proxypal.service"
SERVICE_DEST="/etc/systemd/system/${SERVICE_FILE}"
CONFIG_DIR="/etc/proxypal"
LOG_DIR="/var/log/proxypal"
SERVICE_USER="proxypal"
SERVICE_GROUP="proxypal"

# Functions
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        print_error "Script này cần chạy với quyền root (sudo)"
        exit 1
    fi
}

show_help() {
    echo "ProxyPal Systemd Installation Script"
    echo ""
    echo "Usage: sudo $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --uninstall    Gỡ cài đặt ProxyPal service"
    echo "  --status       Kiểm tra trạng thái service"
    echo "  --help         Hiển thị hướng dẫn này"
    echo ""
    echo "Examples:"
    echo "  sudo $0                # Cài đặt service"
    echo "  sudo $0 --uninstall    # Gỡ cài đặt service"
    echo "  sudo $0 --status       # Kiểm tra trạng thái"
}

create_user() {
    if ! id "${SERVICE_USER}" &>/dev/null; then
        print_info "Tạo user hệ thống: ${SERVICE_USER}"
        useradd --system --no-create-home --shell /usr/sbin/nologin "${SERVICE_USER}"
    else
        print_info "User ${SERVICE_USER} đã tồn tại"
    fi
}

create_directories() {
    print_info "Tạo các thư mục cần thiết..."
    
    # Config directory
    mkdir -p "${CONFIG_DIR}"
    chown "${SERVICE_USER}:${SERVICE_GROUP}" "${CONFIG_DIR}"
    chmod 750 "${CONFIG_DIR}"
    
    # Log directory
    mkdir -p "${LOG_DIR}"
    chown "${SERVICE_USER}:${SERVICE_GROUP}" "${LOG_DIR}"
    chmod 750 "${LOG_DIR}"
}

install_binary() {
    print_info "Cài đặt binary..."
    
    # Check if binary exists
    if [[ -f "${BINARY_SOURCE}" ]]; then
        cp "${BINARY_SOURCE}" "${BINARY_DEST}"
    elif [[ -f "./${BINARY_NAME}" ]]; then
        cp "./${BINARY_NAME}" "${BINARY_DEST}"
    else
        print_error "Không tìm thấy binary ${BINARY_NAME}"
        print_error "Vui lòng build trước: cd src-headless && cargo build --release"
        exit 1
    fi
    
    chmod 755 "${BINARY_DEST}"
    print_info "Binary đã được cài đặt tại: ${BINARY_DEST}"
}

install_service() {
    print_info "Cài đặt systemd service..."
    
    # Get script directory
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    
    if [[ -f "${SCRIPT_DIR}/${SERVICE_FILE}" ]]; then
        cp "${SCRIPT_DIR}/${SERVICE_FILE}" "${SERVICE_DEST}"
    else
        print_error "Không tìm thấy file ${SERVICE_FILE}"
        exit 1
    fi
    
    chmod 644 "${SERVICE_DEST}"
    
    # Reload systemd
    systemctl daemon-reload
    print_info "Service file đã được cài đặt tại: ${SERVICE_DEST}"
}

create_default_config() {
    CONFIG_FILE="${CONFIG_DIR}/config.yaml"
    
    if [[ ! -f "${CONFIG_FILE}" ]]; then
        print_info "Tạo file config mặc định..."
        cat > "${CONFIG_FILE}" << 'EOF'
# ProxyPal Configuration File
# Được tạo tự động bởi install-systemd.sh

# Server port
port: 8080

# Debug mode
debug: false

# Logging level (trace, debug, info, warn, error)
log_level: info

# Provider configurations (thêm API keys của bạn)
# providers:
#   gemini:
#     api_key: "YOUR_GEMINI_API_KEY"
#   claude:
#     api_key: "YOUR_CLAUDE_API_KEY"
EOF
        chown "${SERVICE_USER}:${SERVICE_GROUP}" "${CONFIG_FILE}"
        chmod 640 "${CONFIG_FILE}"
        print_info "Config mặc định đã được tạo tại: ${CONFIG_FILE}"
    else
        print_warn "File config đã tồn tại, giữ nguyên"
    fi
}

enable_and_start() {
    print_info "Kích hoạt và khởi động service..."
    systemctl enable "${BINARY_NAME}"
    systemctl start "${BINARY_NAME}"
    
    # Wait a moment for service to start
    sleep 2
    
    # Check status
    if systemctl is-active --quiet "${BINARY_NAME}"; then
        print_info "Service đang chạy thành công!"
    else
        print_warn "Service có thể gặp vấn đề. Kiểm tra logs:"
        print_warn "  journalctl -u ${BINARY_NAME} -n 50"
    fi
}

show_status() {
    echo ""
    echo "=== ProxyPal Service Status ==="
    systemctl status "${BINARY_NAME}" --no-pager || true
    echo ""
    echo "=== Recent Logs ==="
    journalctl -u "${BINARY_NAME}" -n 10 --no-pager || true
}

uninstall_service() {
    print_info "Gỡ cài đặt ProxyPal service..."
    
    # Stop and disable service
    systemctl stop "${BINARY_NAME}" 2>/dev/null || true
    systemctl disable "${BINARY_NAME}" 2>/dev/null || true
    
    # Remove files
    rm -f "${SERVICE_DEST}"
    rm -f "${BINARY_DEST}"
    
    # Reload systemd
    systemctl daemon-reload
    
    print_info "Service đã được gỡ cài đặt"
    print_warn "Config và logs được giữ lại tại:"
    print_warn "  - ${CONFIG_DIR}"
    print_warn "  - ${LOG_DIR}"
    print_warn "Để xóa hoàn toàn: rm -rf ${CONFIG_DIR} ${LOG_DIR}"
}

# Main script
main() {
    case "${1:-}" in
        --help|-h)
            show_help
            exit 0
            ;;
        --uninstall)
            check_root
            uninstall_service
            exit 0
            ;;
        --status)
            show_status
            exit 0
            ;;
        "")
            # Default: install
            check_root
            print_info "=== Bắt đầu cài đặt ProxyPal ==="
            create_user
            create_directories
            install_binary
            install_service
            create_default_config
            enable_and_start
            echo ""
            print_info "=== Cài đặt hoàn tất ==="
            echo ""
            echo "Các lệnh hữu ích:"
            echo "  systemctl status proxypal   - Kiểm tra trạng thái"
            echo "  systemctl restart proxypal  - Khởi động lại"
            echo "  journalctl -u proxypal -f   - Xem logs"
            echo ""
            echo "Config file: ${CONFIG_DIR}/config.yaml"
            echo "Chỉnh sửa config và restart service để áp dụng thay đổi."
            exit 0
            ;;
        *)
            print_error "Option không hợp lệ: $1"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
