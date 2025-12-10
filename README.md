# ProxyPal

Use your AI subscriptions (Claude, ChatGPT, Gemini, GitHub Copilot) with any coding tool. Native desktop app wrapping [CLIProxyAPI](https://github.com/router-for-me/CLIProxyAPI).

![ProxyPal Dashboard](src/assets/dashboard.png)

## Features

### GUI Mode (Desktop App)
- **Multiple AI Providers** - Connect Claude, ChatGPT, Gemini, Qwen, iFlow, Vertex AI, and custom OpenAI-compatible endpoints
- **GitHub Copilot Bridge** - Use Copilot models via OpenAI-compatible API
- **Antigravity Support** - Access thinking models through Antigravity proxy
- **Works Everywhere** - Cursor, Cline, Continue, Claude Code, OpenCode, and any OpenAI-compatible client
- **Usage Analytics** - Track requests, tokens, success rates, and estimated savings
- **Request Monitoring** - View all API requests with response times and status codes
- **Auto-Configure** - Detects installed CLI agents and sets them up automatically

### Headless Mode (CLI Daemon)
- **Daemon Service** - Run as background service on Ubuntu (systemd) and Windows (NSSM)
- **Multi-Platform** - Cross-compiled binaries for Linux (x64, ARM64) and Windows (x64)
- **Easy Packaging** - DEB package, Docker image, or standalone binary
- **CLI Management** - Full control via command-line interface
- **Device Code Auth** - OAuth device code flow for Gemini and Copilot
- **Health Monitoring** - Systemd integration with health checks

## Supported Platforms

| Platform | Architecture | GUI | Headless |
|----------|-------------|-----|----------|
| macOS    | Apple Silicon (ARM64) | ✅ | ✅ |
| macOS    | Intel (x64) | ✅ | ✅ |
| Windows  | x64 | ✅ | ✅ |
| Linux    | x64 (.deb) | ✅ | ✅ |
| Linux    | ARM64 | - | ✅ |

## Quick Start (GUI)

1. Download from [Releases](https://github.com/wollfoo/cli-all-api/releases)
2. Launch ProxyPal and start the proxy
3. Connect your AI accounts (OAuth or auth files)
4. Point your coding tool to `http://localhost:8317/v1`

## Quick Start (Headless)

### Install

**Ubuntu/Debian (DEB Package):**
```bash
sudo dpkg -i proxypal_0.1.0_amd64.deb
```

**Docker:**
```bash
docker-compose up -d
```

**Binary:**
```bash
# Download and extract
wget https://github.com/wollfoo/cli-all-api/releases/download/v0.1.0/proxypal-linux-x64.tar.gz
tar -xzf proxypal-linux-x64.tar.gz
sudo mv proxypal /usr/local/bin/
```

### Configure

```bash
# Initialize config file
proxypal config init

# Edit configuration (opens in $EDITOR)
proxypal config edit

# Validate configuration
proxypal config validate
```

### Authenticate

```bash
# Add Gemini via device code flow (recommended)
proxypal auth add --provider gemini --device-code

# Add Gemini via API key
proxypal auth add --provider gemini --api-key YOUR_API_KEY

# Add Claude API key
proxypal auth add --provider claude --api-key sk-ant-...

# Import credentials from file
proxypal auth import credentials.json

# Import all credentials from directory
proxypal auth import ./credentials/

# List configured authentications
proxypal auth list
```

### Start

```bash
# Start as daemon (background)
proxypal serve

# Start in foreground with verbose logging
proxypal serve --foreground -vv

# Start on custom port
proxypal serve --port 9000
```

### Verify

```bash
# Check service status
proxypal status

# Health check (exit code 0 = healthy, 1 = unhealthy)
proxypal check

# Health check with JSON output
proxypal check --json

# Check specific provider
proxypal check --provider gemini

# Stop daemon
proxypal stop
```

## CLI Commands Reference

```bash
# Proxy Server
proxypal serve [--port PORT] [--foreground] [--pid-file PATH]
proxypal stop [--pid-file PATH]
proxypal status [--pid-file PATH]
proxypal check [--port PORT] [--json] [--provider NAME]

# Configuration
proxypal config init [--force]
proxypal config validate
proxypal config show
proxypal config edit

# Authentication
proxypal auth add --provider PROVIDER [--api-key KEY | --device-code | --file PATH]
proxypal auth remove --provider PROVIDER
proxypal auth list
proxypal auth test --provider PROVIDER
proxypal auth import PATH [--provider PROVIDER]
```

### Supported Providers

| Provider | Auth Methods |
|----------|-------------|
| `gemini` | API key, device code flow |
| `copilot` | Device code flow |
| `claude` | API key |
| `openai` | API key |
| `vertex` | Service account JSON |
| `qwen` | API key |
| `codex` | API key |

## Systemd (Ubuntu)

```bash
# Install service (from source)
cd deployment
sudo ./install-systemd.sh

# Manage service
sudo systemctl start proxypal
sudo systemctl stop proxypal
sudo systemctl restart proxypal
sudo systemctl status proxypal

# View logs
journalctl -u proxypal -f

# Uninstall
sudo ./install-systemd.sh --uninstall
```

## Windows Service

```powershell
# Install service (requires NSSM)
.\deployment\install-windows.ps1

# Manage service
sc start proxypal
sc stop proxypal
sc query proxypal

# Uninstall
.\deployment\uninstall-windows.ps1
```

## Docker

```bash
# Start with docker-compose
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

## Configuration File

Default location: `~/.config/proxypal/config.yaml`

```yaml
# Server settings
server:
  port: 8317
  host: "127.0.0.1"

# Logging
logging:
  level: info
  format: json

# Provider settings
providers:
  gemini:
    enabled: true
  claude:
    enabled: true
  openai:
    enabled: true
```

## Development

### Desktop App (GUI)
```bash
pnpm install
pnpm tauri dev
```

### Headless Binary
```bash
cd src-headless
cargo build --release
```

### Build DEB Package
```bash
./scripts/build-deb.sh
```

## Tech Stack

- **Frontend**: SolidJS + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri v2 (GUI) / Tokio (Headless)
- **Proxy**: CLIProxyAPI (bundled)
- **CLI**: clap v4 with subcommands

## Support

If you find ProxyPal useful, consider [buying me a coffee](https://buymeacoffee.com/heyhuynhgiabuu).

## License

MIT
