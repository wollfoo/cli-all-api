# ProxyPal Windows Service Guide

Hướng dẫn cài đặt và quản lý ProxyPal như Windows Service.

## Prerequisites

1. **Windows 10/11 hoặc Windows Server 2016+**
2. **PowerShell 5.1+** (có sẵn trong Windows)
3. **Administrator rights**

## Quick Start

```powershell
# Mở PowerShell với quyền Administrator
# Chuyển đến thư mục chứa proxypal.exe và scripts

.\install-windows.ps1
```

## Installation Options

```powershell
# Cài đặt cơ bản (binary trong thư mục hiện tại)
.\install-windows.ps1

# Chỉ định đường dẫn binary
.\install-windows.ps1 -BinaryPath "C:\Downloads\proxypal.exe"

# Cài đặt với port tùy chỉnh
.\install-windows.ps1 -Port 9000

# Full options
.\install-windows.ps1 -BinaryPath ".\proxypal.exe" -ServiceName "ProxyPal" -Port 8317
```

## Installation Paths

| Component | Location |
|-----------|----------|
| Binary | `C:\Program Files\ProxyPal\proxypal.exe` |
| Config | `%APPDATA%\ProxyPal\config.yaml` |
| Logs | `C:\Program Files\ProxyPal\logs\` |
| NSSM | `C:\Tools\nssm\nssm.exe` |

## Service Management

```powershell
# Kiểm tra trạng thái
Get-Service ProxyPal

# Khởi động
Start-Service ProxyPal

# Dừng
Stop-Service ProxyPal

# Khởi động lại
Restart-Service ProxyPal

# Xem chi tiết
Get-Service ProxyPal | Format-List *
```

## Viewing Logs

```powershell
# Xem log output
Get-Content "C:\Program Files\ProxyPal\logs\proxypal.log" -Tail 50

# Follow logs (realtime)
Get-Content "C:\Program Files\ProxyPal\logs\proxypal.log" -Wait

# Xem error logs
Get-Content "C:\Program Files\ProxyPal\logs\proxypal-error.log" -Tail 50

# Windows Event Viewer (services events)
Get-EventLog -LogName Application -Source "ProxyPal" -Newest 20
```

## Configuration

Edit config file:
```powershell
notepad "$env:APPDATA\ProxyPal\config.yaml"
```

After editing, restart service:
```powershell
Restart-Service ProxyPal
```

## Recovery Options

Service tự động restart khi fail:
- **First failure**: Restart immediately
- **Second failure**: Restart after 5 seconds
- **Subsequent failures**: Restart after 5 minutes
- **Reset fail count**: After 24 hours

## Uninstallation

```powershell
# Gỡ cài đặt (giữ config)
.\uninstall-windows.ps1

# Gỡ hoàn toàn (xóa tất cả)
.\uninstall-windows.ps1 -RemoveAll
```

## Troubleshooting

### Service fails to start

1. Kiểm tra logs:
   ```powershell
   Get-Content "C:\Program Files\ProxyPal\logs\proxypal-error.log"
   ```

2. Kiểm tra port đã bị chiếm:
   ```powershell
   netstat -ano | findstr :8317
   ```

3. Chạy manual test:
   ```powershell
   & "C:\Program Files\ProxyPal\proxypal.exe" serve --foreground
   ```

### Permission issues

1. Đảm bảo chạy PowerShell với Administrator
2. Kiểm tra antivirus không block proxypal.exe

### NSSM issues

1. Kiểm tra NSSM đã cài:
   ```powershell
   C:\Tools\nssm\nssm.exe version
   ```

2. Xem service config:
   ```powershell
   C:\Tools\nssm\nssm.exe dump ProxyPal
   ```

3. Edit service tương tác:
   ```powershell
   C:\Tools\nssm\nssm.exe edit ProxyPal
   ```

## Manual Installation (Without Scripts)

Nếu không muốn dùng scripts:

1. Download NSSM từ https://nssm.cc/download

2. Cài đặt service:
   ```cmd
   nssm install ProxyPal "C:\path\to\proxypal.exe" serve --foreground
   nssm set ProxyPal AppDirectory "C:\path\to"
   nssm set ProxyPal DisplayName "ProxyPal - AI Proxy Server"
   nssm set ProxyPal Start SERVICE_AUTO_START
   ```

3. Start service:
   ```cmd
   nssm start ProxyPal
   ```

## Firewall Configuration

Nếu cần truy cập từ máy khác:

```powershell
# Mở port 8317 cho inbound
New-NetFirewallRule -DisplayName "ProxyPal" -Direction Inbound -Port 8317 -Protocol TCP -Action Allow
```

## Security Recommendations

1. **Config file**: Chứa API keys, đảm bảo permissions phù hợp
2. **Network**: Chỉ bind localhost nếu không cần remote access
3. **Updates**: Cập nhật proxypal.exe thường xuyên
