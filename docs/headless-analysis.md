# Headless Binary Analysis: lib.rs Tauri Dependencies

## Tổng quan

Tài liệu này phân tích file `src-tauri/src/lib.rs` (5339 dòng) để xác định và phân loại tất cả **Tauri-specific code** nhằm chuẩn bị refactoring cho **headless binary** (CLI daemon).

### Mục tiêu Refactoring
- Tách logic có thể tái sử dụng từ Tauri GUI app
- Tạo headless binary chạy như daemon service (systemd/Windows Service)
- Giữ nguyên tính năng proxy core mà không cần GUI

---

## 1. Tauri Imports và API Calls

### 1.1 Direct Tauri Imports (Lines 5-12)

| Line | Import | Category | Reasoning |
|------|--------|----------|-----------|
| 5-9 | `tauri::{menu::*, tray::*, Emitter, Manager, State}` | `tauri-only` | GUI tray menu, event emitting, state management |
| 10 | `tauri_plugin_opener::OpenerExt` | `tauri-only` | Browser opener cho OAuth flows |
| 11 | `tauri_plugin_shell::process::CommandChild` | `needs-refactor` | Process spawning - có thể thay bằng `std::process` |
| 12 | `tauri_plugin_shell::ShellExt` | `needs-refactor` | Shell commands - có thể thay bằng `std::process::Command` |

### 1.2 Tauri Plugin Dependencies

| Plugin | Usage | Headless Alternative |
|--------|-------|---------------------|
| `tauri-plugin-shell` | Spawn CLIProxyAPI sidecar | `std::process::Command` |
| `tauri-plugin-opener` | Open OAuth URLs in browser | `webbrowser` crate hoặc CLI prompt |
| `tauri-plugin-deep-link` | OAuth callback handling | Not needed - dùng device code flow |
| `tauri-plugin-notification` | Desktop notifications | `notify-rust` crate hoặc logging |
| `tauri-plugin-dialog` | File dialogs | CLI file path arguments |
| `tauri-plugin-single-instance` | Single app instance | PID file lock pattern |

---

## 2. Dependency Matrix: Structs

### 2.1 Data Structures

| Struct | Lines | Category | Reasoning |
|--------|-------|----------|-----------|
| `ProxyStatus` | 17-22 | `reusable` | Pure data, no Tauri deps |
| `RequestLog` | 25-38 | `reusable` | Pure data for logging |
| `AuthStatus` | 51-60 | `reusable` | Provider auth counts |
| `AppConfig` | 77-120 | `reusable` | Configuration structure |
| `AmpModelMapping` | 131-137 | `reusable` | Model routing config |
| `AmpOpenAIModel` | 144-150 | `reusable` | OpenAI model config |
| `AmpOpenAIProvider` | 153-163 | `reusable` | Custom provider config |
| `CopilotConfig` | 170-185 | `reusable` | Copilot settings |
| `CopilotStatus` | 205-223 | `reusable` | Copilot status |
| `OAuthState` | 255-259 | `tauri-only` | OAuth flow tracking (browser-based) |
| `UsageStats` | 262-284 | `reusable` | Usage statistics |
| `TimeSeriesPoint` | 286-291 | `reusable` | Chart data |
| `ModelUsage` | 293-299 | `reusable` | Model usage stats |
| `AppState` | 302-312 | `needs-refactor` | Contains `CommandChild` (Tauri), needs abstraction |
| `RequestHistory` | 356-363 | `reusable` | Request history data |
| `CopilotApiDetection` | 1514-1526 | `reusable` | Detection result |
| `CopilotApiInstallResult` | 1803-1809 | `needs-refactor` | Uses Tauri shell |
| `ProviderHealth` | 2500-2509 | `reusable` | Health check data |
| `HealthStatus` | 2511-2526 | `reusable` | Health status |
| `AgentTestResult` | 2623-2629 | `reusable` | Connection test result |
| `AvailableModel` | 2686-2691 | `reusable` | Model info |
| `ProviderTestResult` | 2758-2765 | `reusable` | Provider test result |
| `DetectedTool` | 2953-2961 | `reusable` | Detected AI tool |
| `AgentStatus` | 2964-2976 | `reusable` | CLI agent status |
| `GeminiApiKey` | 3908-3920 | `reusable` | API key structure |
| `ClaudeApiKey` | 3931-3945 | `reusable` | API key structure |
| `CodexApiKey` | 3947-3957 | `reusable` | API key structure |
| `OpenAICompatibleProvider` | 3967-3977 | `reusable` | Provider config |
| `AuthFile` | 4316-4357 | `reusable` | Auth file metadata |
| `LogEntry` | 4906-4912 | `reusable` | Log parsing |

### 2.2 Summary by Category

| Category | Count | Percentage |
|----------|-------|------------|
| `reusable` | 26 | 84% |
| `needs-refactor` | 2 | 6% |
| `tauri-only` | 3 | 10% |

---

## 3. Dependency Matrix: Functions

### 3.1 Pure Utility Functions (Reusable)

| Function | Lines | Purpose |
|----------|-------|---------|
| `get_config_path()` | 331-337 | Config file path |
| `get_auth_path()` | 339-345 | Auth file path |
| `get_history_path()` | 347-353 | History file path |
| `load_request_history()` | 366-376 | Load history from file |
| `save_request_history()` | 378-388 | Save history to file |
| `estimate_request_cost()` | 391-415 | Cost estimation |
| `load_config()` | 418-446 | Load config from file |
| `save_config_to_file()` | 449-453 | Save config to file |
| `load_auth_status()` | 456-466 | Load auth status |
| `save_auth_to_file()` | 469-473 | Save auth to file |
| `detect_provider_from_model()` | 476-505 | Detect provider from model name |
| `detect_provider_from_path()` | 511-541 | Detect provider from API path |
| `extract_model_from_path()` | 546-568 | Extract model from URL path |
| `parse_gin_log_line()` | 572-665 | Parse GIN log format |
| `generate_uuid()` | 165-167 | Generate UUID |
| `default_*()` | various | Default value functions |
| `which_exists()` | 3122-3250 | Check command existence |
| `check_env_configured()` | 3253-3257 | Check env var |
| `get_model_limits()` | 3261-3325 | Get model context limits |
| `get_model_display_name()` | 3328-3352 | Format model name |
| `build_management_client()` | 3980-3985 | Build HTTP client |
| `get_management_url()` | 3988-3990 | Build Management API URL |
| `convert_api_key_response()` | 3995-4020 | Convert API response |
| `convert_to_management_format()` | 4023-4032 | Convert to API format |
| `parse_log_line()` | 4971-5045 | Parse log line |
| `normalize_log_level()` | 5048-5057 | Normalize log level |

### 3.2 Tauri Commands (Need Refactoring)

| Function | Lines | Category | Headless Approach |
|----------|-------|----------|-------------------|
| `get_proxy_status()` | 768-771 | `needs-refactor` | Return from shared state |
| `start_proxy()` | 773-1174 | `needs-refactor` | Replace Tauri sidecar with `std::process` |
| `stop_proxy()` | 1176-1211 | `needs-refactor` | Kill process via PID |
| `start_copilot()` | 1222-1443 | `needs-refactor` | Replace Tauri shell with `std::process` |
| `stop_copilot()` | 1445-1478 | `needs-refactor` | Kill process via PID |
| `check_copilot_health()` | 1480-1511 | `reusable` | HTTP health check |
| `detect_copilot_api()` | 1528-1801 | `needs-refactor` | Replace app.shell() with std::process |
| `install_copilot_api()` | 1811-1922 | `needs-refactor` | Replace app.shell() |
| `get_auth_status()` | 1924-1927 | `needs-refactor` | State accessor |
| `get_usage_stats()` | 1931-2059 | `reusable` | Pure computation |
| `get_request_history()` | 2062-2065 | `reusable` | File read |
| `add_request_to_history()` | 2068-2092 | `reusable` | File write |
| `clear_request_history()` | 2095-2099 | `reusable` | File delete |
| `sync_usage_from_proxy()` | 2103-2177 | `reusable` | HTTP call to proxy |
| `open_oauth()` | 2179-2245 | `tauri-only` | Replace with device code flow |
| `poll_oauth_status()` | 2247-2279 | `reusable` | HTTP polling |
| `refresh_auth_status()` | 2281-2337 | `needs-refactor` | Remove app.emit() |
| `complete_oauth()` | 2339-2378 | `tauri-only` | Not needed for headless |
| `disconnect_provider()` | 2380-2437 | `needs-refactor` | Remove app.emit() |
| `import_vertex_credential()` | 2440-2485 | `needs-refactor` | Remove app.emit() |
| `get_config()` | 2487-2490 | `needs-refactor` | State accessor |
| `save_config()` | 2492-2497 | `needs-refactor` | State mutator |
| `check_provider_health()` | 2528-2620 | `reusable` | HTTP call |
| `test_agent_connection()` | 2631-2683 | `reusable` | HTTP call |
| `get_available_models()` | 2704-2755 | `reusable` | HTTP call |
| `test_openai_provider()` | 2767-2861 | `reusable` | HTTP call |
| `detect_cli_agents()` | 2979-3117 | `reusable` | File system checks |
| `configure_cli_agent()` | 3355-3698 | `reusable` | File write |
| `get_shell_profile_path()` | 3701-3725 | `reusable` | Path resolution |
| `append_to_shell_profile()` | 3728-3746 | `reusable` | File append |
| `detect_ai_tools()` | 3749-3840 | `reusable` | File system checks |
| `configure_continue()` | 3843-3901 | `reusable` | File write |
| All API Key CRUD functions | 4034-4309 | `reusable` | HTTP calls to Management API |
| All Auth File functions | 4359-4551 | `reusable` | HTTP calls + file ops |
| All Management Settings | 4557-4794 | `reusable` | HTTP calls |
| `get_config_yaml()` | 4797-4817 | `reusable` | HTTP call |
| `set_config_yaml()` | 4820-4842 | `reusable` | HTTP call |
| Log functions | 4929-5080 | `reusable` | HTTP calls |
| `get_tool_setup_info()` | 5083-5173 | `reusable` | Static data |

### 3.3 Tauri-Only Functions (GUI-specific)

| Function | Lines | Purpose | Headless Action |
|----------|-------|---------|-----------------|
| `start_log_watcher()` | 668-765 | Emit events to frontend | Replace with file logging |
| `handle_deep_link()` | 2864-2897 | OAuth deep link handling | Not needed |
| `setup_tray()` | 2900-2950 | System tray setup | Not needed |
| `run()` | 5176-5338 | Tauri app entry point | Create new CLI entry point |

### 3.4 Summary by Category

| Category | Count | Notes |
|----------|-------|-------|
| `reusable` | ~45 | Pure logic, HTTP calls, file I/O |
| `needs-refactor` | ~20 | Remove State/app.emit/shell deps |
| `tauri-only` | ~5 | GUI/tray/deep-link specific |

---

## 4. Config Generation Logic (proxy-config.yaml)

### 4.1 Location
- **Generation Function**: `start_proxy()` (lines 816-1084)
- **File Path**: `~/.config/proxypal/proxy-config.yaml`

### 4.2 Config Structure

```yaml
# ProxyPal generated config
port: {config.port}
auth-dir: "~/.cli-proxy-api"
api-keys:
  - "proxypal-local"
debug: {config.debug}
usage-statistics-enabled: {config.usage_stats_enabled}
logging-to-file: {config.logging_to_file}
request-retry: {config.request_retry}
{proxy_url_line}  # Optional: proxy-url: "..."

# Quota exceeded behavior
quota-exceeded:
  switch-project: {config.quota_switch_project}
  switch-preview-model: {config.quota_switch_preview_model}

# Enable Management API for OAuth flows
remote-management:
  allow-remote: false
  secret-key: "proxypal-mgmt-key"
  disable-control-panel: true

{openai_compat_section}   # OpenAI-compatible providers
{claude_api_key_section}  # Claude API keys  
{gemini_api_key_section}  # Gemini API keys
{codex_api_key_section}   # Codex API keys

# Amp CLI Integration
ampcode:
  upstream-url: "https://ampcode.com"
{amp_api_key_line}
{amp_model_mappings_section}
  restrict-management-to-localhost: true
```

### 4.3 Dynamic Sections

| Section | Source | Lines |
|---------|--------|-------|
| `proxy_url_line` | `config.proxy_url` | 825-829 |
| `amp_api_key_line` | `config.amp_api_key` | 832-836 |
| `amp_model_mappings_section` | `config.amp_model_mappings` | 838-854 |
| `openai_compat_section` | `config.amp_openai_providers` + `config.copilot` | 857-939 |
| `claude_api_key_section` | `config.copilot` + `config.claude_api_keys` | 942-993 |
| `gemini_api_key_section` | `config.gemini_api_keys` | 996-1013 |
| `codex_api_key_section` | `config.codex_api_keys` | 1016-1033 |

### 4.4 Headless Refactoring Approach
1. Extract config generation to standalone function `generate_proxy_config(config: &AppConfig) -> String`
2. Support reading config from YAML file (e.g., `config.yaml`) instead of interactive UI
3. Support environment variables for sensitive values (API keys)
4. Validate config before writing

---

## 5. Refactoring Recommendations

### 5.1 Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Shared Library                            │
│  (proxypal-core)                                                 │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐   │
│  │ Data Structs │  │ Config I/O   │  │ CLIProxyAPI Manager  │   │
│  │ ProxyStatus  │  │ load_config  │  │ start/stop process   │   │
│  │ AuthStatus   │  │ save_config  │  │ generate config YAML │   │
│  │ AppConfig    │  │ load_auth    │  │ health checks        │   │
│  └──────────────┘  └──────────────┘  └──────────────────────┘   │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐   │
│  │ HTTP Utils   │  │ Log Parser   │  │ Provider Detection   │   │
│  │ mgmt client  │  │ parse_gin    │  │ detect_provider      │   │
│  │ API calls    │  │ parse_log    │  │ detect_cli_agents    │   │
│  └──────────────┘  └──────────────┘  └──────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                    ▲                        ▲
                    │                        │
        ┌───────────┴───────────┐ ┌─────────┴───────────┐
        │   Tauri GUI App       │ │   Headless CLI      │
        │   (proxypal-desktop)  │ │   (proxypal-daemon) │
        │                       │ │                     │
        │ - System Tray         │ │ - CLI commands      │
        │ - OAuth browser flow  │ │ - Device code auth  │
        │ - Event emitting      │ │ - Signal handling   │
        │ - Deep links          │ │ - systemd support   │
        └───────────────────────┘ └─────────────────────┘
```

### 5.2 Recommended File Structure

```
src/
├── lib.rs              # Shared library exports
├── core/
│   ├── mod.rs
│   ├── config.rs       # AppConfig, load/save
│   ├── types.rs        # All data structs
│   └── utils.rs        # Utility functions
├── proxy/
│   ├── mod.rs
│   ├── manager.rs      # Process management
│   ├── config_gen.rs   # YAML generation
│   └── health.rs       # Health checks
├── api/
│   ├── mod.rs
│   ├── management.rs   # Management API calls
│   └── auth.rs         # Auth file management
└── detection/
    ├── mod.rs
    ├── agents.rs       # CLI agent detection
    └── tools.rs        # AI tool detection

src-tauri/
└── src/
    ├── main.rs         # Tauri entry point
    └── commands.rs     # Tauri commands (thin wrappers)

src-cli/
└── src/
    ├── main.rs         # CLI entry point
    ├── commands/
    │   ├── serve.rs    # `proxypal serve`
    │   ├── config.rs   # `proxypal config validate`
    │   └── auth.rs     # `proxypal auth add`
    └── daemon.rs       # Daemon mode logic
```

### 5.3 Refactoring Steps

#### Phase 1: Extract Core Library (Effort: Medium)
1. Create `proxypal-core` crate with shared types and utilities
2. Move all `reusable` structs and functions
3. Abstract `AppState` to remove Tauri `CommandChild` dependency
4. Create trait for process management

#### Phase 2: Refactor Process Management (Effort: High)
1. Replace `tauri_plugin_shell` with `std::process::Command`
2. Create `ProcessManager` trait with Tauri and std implementations
3. Handle async process spawning and monitoring

#### Phase 3: Create CLI Binary (Effort: Medium)
1. Implement CLI with `clap` crate
2. Add `serve`, `config validate`, `auth add` commands
3. Implement signal handling (SIGTERM, SIGINT)
4. Add daemon mode with PID file

#### Phase 4: Authentication Refactoring (Effort: Low)
1. Replace browser OAuth with device code flow
2. Support API key auth via config file
3. Support auth file import from CLI

---

## 6. Estimated Effort

| Component | Effort | Hours | Priority |
|-----------|--------|-------|----------|
| Core library extraction | Medium | 8-12 | P0 |
| Process management refactor | High | 12-16 | P0 |
| Config generation refactor | Low | 2-4 | P0 |
| CLI binary skeleton | Medium | 4-6 | P1 |
| `serve` command | Medium | 6-8 | P1 |
| `config validate` command | Low | 2-3 | P2 |
| `auth add` command | Medium | 4-6 | P2 |
| Signal handling | Low | 2-3 | P1 |
| Daemon mode | Medium | 4-6 | P1 |
| systemd integration | Low | 2-3 | P2 |
| Windows Service | Medium | 6-8 | P3 |
| **Total** | | **52-75 hours** | |

---

## 7. Key Findings

### 7.1 Good News
- **84% of structs are reusable** without modification
- Core business logic (config I/O, API calls, detection) is Tauri-agnostic
- HTTP calls use `reqwest` - already async and portable
- Config generation is string-based, easy to extract

### 7.2 Challenges
- `AppState` contains `tauri_plugin_shell::CommandChild` - needs abstraction
- ~20 Tauri commands use `State<AppState>` - need state management refactor
- Event emitting (`app.emit()`) used for real-time updates - needs alternative
- OAuth flow requires browser - need device code flow for headless

### 7.3 Critical Path
1. Extract core library with types and utilities
2. Abstract process management to support both Tauri and std
3. Create CLI binary with `serve` command
4. Add signal handling and graceful shutdown

---

## Appendix: Complete Import Analysis

```rust
// Tauri-specific (lines 5-12)
use tauri::{
    menu::{Menu, MenuItem},                    // tauri-only
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, // tauri-only
    Emitter, Manager, State,                   // needs-refactor (State)
};
use tauri_plugin_opener::OpenerExt;            // tauri-only
use tauri_plugin_shell::process::CommandChild; // needs-refactor
use tauri_plugin_shell::ShellExt;              // needs-refactor

// Standard library (reusable)
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use regex::Regex;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

// External crates used (all reusable)
// - serde, serde_json   (serialization)
// - reqwest             (HTTP client)
// - chrono              (datetime)
// - dirs                (system directories)
// - regex               (log parsing)
// - uuid                (ID generation)
// - lazy_static         (static regex)
// - tokio               (async runtime)
```
