# ProxyPal Deployment Guide

Hướng dẫn triển khai ProxyPal headless mode trên các nền tảng khác nhau.

## Table of Contents

- [Ubuntu/Debian Deployment (Systemd)](#ubuntudebian-deployment-systemd)
- [Windows Deployment (NSSM)](#windows-deployment-nssm)
- [Docker Deployment](#docker-deployment)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

---

## Ubuntu/Debian Deployment (Systemd)

### Prerequisites

- Ubuntu 20.04+ hoặc Debian 11+
- Root access (sudo)
- Port 8317 available

### Option 1: DEB Package (Recommended)

```bash
# Download và install DEB package
wget https://github.com/wollfoo/cli-all-api/releases/download/v0.1.0/proxypal_0.1.0_amd64.deb
sudo dpkg -i proxypal_0.1.0_amd64.deb

# Verify installation
proxypal --version

# Start service
sudo systemctl start proxypal
sudo systemctl enable proxypal
```

### Option 2: Install Script

```bash
# Clone repository
git clone https://github.com/wollfoo/cli-all-api.git
cd cli-all-api

# Build binary
cd src-headless
cargo build --release
cd ..

# Run install script
cd deployment
sudo ./install-systemd.sh
```

### Option 3: Manual Installation

```bash
# Copy binary
sudo cp src-headless/target/release/proxypal /usr/local/bin/
sudo chmod +x /usr/local/bin/proxypal

# Create service user
sudo useradd --system --no-create-home --shell /usr/sbin/nologin proxypal

# Create directories
sudo mkdir -p /etc/proxypal /var/log/proxypal
sudo chown proxypal:proxypal /etc/proxypal /var/log/proxypal

# Copy service file
sudo cp deployment/proxypal.service /etc/systemd/system/
sudo systemctl daemon-reload

# Initialize config
sudo -u proxypal proxypal config init --config /etc/proxypal/config.yaml
```

### Service Management

```bash
# Start/Stop/Restart
sudo systemctl start proxypal
sudo systemctl stop proxypal
sudo systemctl restart proxypal

# Check status
sudo systemctl status proxypal

# Enable auto-start on boot
sudo systemctl enable proxypal
```

### Viewing Logs

```bash
# Real-time logs
journalctl -u proxypal -f

# Logs from last hour
journalctl -u proxypal --since "1 hour ago"

# Last 100 lines
journalctl -u proxypal -n 100

# Filter by priority (error and above)
journalctl -u proxypal -p err
```

### Uninstall

```bash
# Using install script
sudo ./deployment/install-systemd.sh --uninstall

# Or manually
sudo systemctl stop proxypal
sudo systemctl disable proxypal
sudo rm /etc/systemd/system/proxypal.service
sudo rm /usr/local/bin/proxypal
sudo systemctl daemon-reload

# Optional: Remove config and logs
sudo rm -rf /etc/proxypal /var/log/proxypal
```

---

## Windows Deployment (NSSM)

### Prerequisites

- Windows 10/11 hoặc Windows Server 2016+
- PowerShell 5.1+ (Run as Administrator)
- Port 8317 available

### Quick Start

```powershell
# Mở PowerShell với quyền Administrator
cd C:\path\to\cli-all-api\deployment

# Run install script
.\install-windows.ps1
```

### Installation Options

```powershell
# Basic install (binary in current directory)
.\install-windows.ps1

# Specify binary path
.\install-windows.ps1 -BinaryPath "C:\Downloads\proxypal.exe"

# Custom port
.\install-windows.ps1 -Port 9000

# Full options
.\install-windows.ps1 -BinaryPath ".\proxypal.exe" -ServiceName "ProxyPal" -Port 8317
```

### Installation Paths

| Component | Location |
|-----------|----------|
| Binary | `C:\Program Files\ProxyPal\proxypal.exe` |
| Config | `%APPDATA%\ProxyPal\config.yaml` |
| Logs | `C:\Program Files\ProxyPal\logs\` |
| NSSM | `C:\Tools\nssm\nssm.exe` |

### Service Management

```powershell
# Check status
Get-Service ProxyPal

# Start/Stop/Restart
Start-Service ProxyPal
Stop-Service ProxyPal
Restart-Service ProxyPal

# View details
Get-Service ProxyPal | Format-List *
```

### Viewing Logs

```powershell
# View log output
Get-Content "C:\Program Files\ProxyPal\logs\proxypal.log" -Tail 50

# Follow logs (realtime)
Get-Content "C:\Program Files\ProxyPal\logs\proxypal.log" -Wait

# Error logs
Get-Content "C:\Program Files\ProxyPal\logs\proxypal-error.log" -Tail 50
```

### Uninstall

```powershell
# Keep config
.\uninstall-windows.ps1

# Remove everything
.\uninstall-windows.ps1 -RemoveAll
```

### Recovery Options

Service automatically restarts on failure:
- **First failure**: Restart immediately
- **Second failure**: Restart after 5 seconds
- **Subsequent failures**: Restart after 5 minutes
- **Reset fail count**: After 24 hours

---

## Docker Deployment

### Prerequisites

- Docker 20.10+
- Docker Compose 2.0+

### Quick Start

```bash
# Clone repository
git clone https://github.com/wollfoo/cli-all-api.git
cd cli-all-api

# Create config directory
mkdir -p config auth

# Create config file
cat > config/config.yaml << 'EOF'
server:
  port: 8317
  host: "0.0.0.0"
logging:
  level: info
EOF

# Start container
docker compose up -d

# View logs
docker compose logs -f
```

### Docker Compose Configuration

File: `docker-compose.yml`

```yaml
services:
  proxypal:
    image: proxypal:latest
    container_name: proxypal
    ports:
      - "8317:8317"
    volumes:
      - ./config:/etc/proxypal:ro
      - ./auth:/var/lib/proxypal/auth
      - proxypal-logs:/var/log/proxypal
    environment:
      - RUST_LOG=info
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "proxypal", "check", "--port", "8317"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  proxypal-logs:
```

### Container Management

```bash
# Start
docker compose up -d

# Stop
docker compose down

# Restart
docker compose restart

# View logs
docker compose logs -f proxypal

# Health check
docker compose exec proxypal proxypal check

# Shell access
docker compose exec proxypal sh
```

### Build Custom Image

```bash
# Build from source
docker build -t proxypal:custom .

# Build with specific binary
docker build \
  --build-arg CLIPROXYAPI_BINARY=path/to/cliproxyapi \
  -t proxypal:custom .
```

### Volume Mounts

| Mount | Purpose |
|-------|---------|
| `/etc/proxypal` | Configuration files (read-only) |
| `/var/lib/proxypal/auth` | Authentication credentials |
| `/var/log/proxypal` | Log files |

### Resource Limits

```yaml
deploy:
  resources:
    limits:
      cpus: '2'
      memory: 1G
    reservations:
      cpus: '0.5'
      memory: 256M
```

---

## Configuration

### Config File Locations

| Platform | Location |
|----------|----------|
| Linux (user) | `~/.config/proxypal/config.yaml` |
| Linux (system) | `/etc/proxypal/config.yaml` |
| Windows | `%APPDATA%\ProxyPal\config.yaml` |
| Docker | `/etc/proxypal/config.yaml` |

### Configuration Options

```yaml
# Server settings
server:
  port: 8317              # Port to listen on (1-65535)
  host: "127.0.0.1"       # Host to bind (use 0.0.0.0 for all interfaces)

# Logging
logging:
  level: info             # trace, debug, info, warn, error
  format: json            # json, text

# Provider settings
providers:
  gemini:
    enabled: true
    timeout: 30           # Request timeout in seconds
  claude:
    enabled: true
    timeout: 60
  openai:
    enabled: true
  copilot:
    enabled: false
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PROXYPAL_CONFIG` | Config file path | `~/.config/proxypal/config.yaml` |
| `PROXYPAL_OAUTH_CLIENT_ID` | OAuth client ID | - |
| `RUST_LOG` | Log level | `info` |

### Security Recommendations

1. **Bind to localhost**: Use `host: "127.0.0.1"` unless remote access needed
2. **Config permissions**: Restrict read access to config file (contains API keys)
3. **Firewall**: Only open port if remote access required
4. **Updates**: Keep ProxyPal updated for security patches

---

## Troubleshooting

### Common Issues

#### Port Already in Use

```bash
# Linux: Check what's using port 8317
sudo lsof -i :8317
sudo netstat -tlnp | grep 8317

# Windows
netstat -ano | findstr :8317

# Solution: Stop the conflicting service or use different port
proxypal serve --port 9000
```

#### Service Fails to Start

```bash
# Linux: Check systemd logs
journalctl -u proxypal -n 50

# Test manual start
proxypal serve --foreground -vv

# Validate config
proxypal config validate
```

```powershell
# Windows: Check error log
Get-Content "C:\Program Files\ProxyPal\logs\proxypal-error.log"

# Test manual start
& "C:\Program Files\ProxyPal\proxypal.exe" serve --foreground
```

#### Permission Denied

```bash
# Linux: Check file ownership
ls -la /usr/local/bin/proxypal
ls -la /etc/proxypal/

# Fix permissions
sudo chown proxypal:proxypal /etc/proxypal -R
sudo chmod 750 /etc/proxypal
```

#### Config File Not Found

```bash
# Initialize config
proxypal config init

# Or specify config path
proxypal serve --config /path/to/config.yaml
```

### Debug Mode

```bash
# Enable verbose logging
proxypal serve --foreground -vvv

# Or via environment
RUST_LOG=debug proxypal serve --foreground
```

### Health Check

```bash
# Basic check
proxypal check

# JSON output (for scripts)
proxypal check --json

# Check specific provider
proxypal check --provider gemini

# HTTP endpoint (when running)
curl http://localhost:8317/health
```

### Log Locations

| Platform | Location |
|----------|----------|
| Systemd | `journalctl -u proxypal` |
| Windows | `C:\Program Files\ProxyPal\logs\` |
| Docker | `docker compose logs proxypal` |

### Support Resources

- **GitHub Issues**: https://github.com/wollfoo/cli-all-api/issues
- **Documentation**: https://github.com/wollfoo/cli-all-api#readme
- **Man Pages**: `man proxypal` (after installation)

---

## Security Hardening (Systemd)

The systemd service includes these security features:

```ini
NoNewPrivileges=true       # Prevent privilege escalation
ProtectSystem=strict       # Mount filesystem read-only
ProtectHome=read-only      # Protect home directories
PrivateTmp=true            # Use private /tmp
ProtectKernelTunables=true # Protect kernel settings
ProtectKernelModules=true  # Prevent kernel module loading
ProtectControlGroups=true  # Protect cgroups
RestrictRealtime=true      # Prevent realtime scheduling
RestrictSUIDSGID=true      # Restrict SUID/SGID
RestrictAddressFamilies=AF_INET AF_INET6 AF_UNIX  # Network only
```

These settings ensure ProxyPal runs with minimal privileges.

---

## Quick Reference

### Ubuntu Systemd

```bash
sudo systemctl start proxypal    # Start
sudo systemctl stop proxypal     # Stop
sudo systemctl status proxypal   # Status
journalctl -u proxypal -f        # Logs
```

### Windows NSSM

```powershell
Start-Service ProxyPal           # Start
Stop-Service ProxyPal            # Stop
Get-Service ProxyPal             # Status
Get-Content "...\proxypal.log" -Wait  # Logs
```

### Docker

```bash
docker compose up -d             # Start
docker compose down              # Stop
docker compose ps                # Status
docker compose logs -f           # Logs
```
