# ProxyPal Deployment Files

Thư mục này chứa các file cần thiết để deploy ProxyPal như system service.

## Files

| File | Mô tả |
|------|-------|
| `proxypal.service` | Systemd unit file cho Ubuntu 20.04+ |
| `install-systemd.sh` | Script cài đặt tự động |

## Quick Start

```bash
# Build binary (từ thư mục gốc project)
cd src-headless
cargo build --release

# Cài đặt (cần sudo)
cd ../deployment
sudo ./install-systemd.sh
```

## Systemd Commands

```bash
# Quản lý service
systemctl start proxypal     # Khởi động
systemctl stop proxypal      # Dừng
systemctl restart proxypal   # Khởi động lại
systemctl status proxypal    # Kiểm tra trạng thái

# Xem logs
journalctl -u proxypal -f              # Logs realtime
journalctl -u proxypal --since "1h"    # Logs 1 giờ gần đây
journalctl -u proxypal -n 100          # 100 dòng log gần nhất
```

## Configuration

Config file mặc định: `/etc/proxypal/config.yaml`

Sau khi chỉnh sửa config:
```bash
sudo systemctl restart proxypal
```

## Uninstall

```bash
sudo ./install-systemd.sh --uninstall
```

## Security Features

Unit file bao gồm các hardening options:
- `NoNewPrivileges=true` - Ngăn privilege escalation
- `ProtectSystem=strict` - Filesystem read-only
- `ProtectHome=read-only` - Home directory read-only
- `PrivateTmp=true` - Isolated /tmp
- `RestrictAddressFamilies` - Chỉ cho phép network sockets cần thiết
