# ProxyPal Headless Mode Integration – Kế hoạch Triển khai APM
**Chiến lược Memory:** Dynamic-MD
**Sửa đổi Cuối cùng:** Plan creation by the Setup Agent.
**Tổng quan Dự án:** Mở rộng ProxyPal để chạy như headless daemon service trên Ubuntu server và Windows, không cần GUI. Sử dụng Rust CLI binary tái sử dụng logic từ `lib.rs`, tích hợp CLIProxyAPI (Go binary), hỗ trợ unified API endpoint (OpenAI + Claude format), device code auth flow, và đa dạng packaging (binary, Docker, DEB/RPM).

---

## Phase 1: CLI Core Development

### Task 1.1 – Analyze lib.rs for Tauri Dependencies - Agent_CLI_Core
**Objective:** Xác định và phân loại tất cả Tauri-specific code trong `lib.rs` để chuẩn bị refactoring.
**Output:** `docs/headless-analysis.md` với dependency matrix.
**Guidance:** File `lib.rs` có 5339 dòng, cần phân tích kỹ các `tauri::` imports và `#[tauri::command]` macros.

- Scan `lib.rs` để xác định tất cả `tauri::` imports và API calls
- Phân loại structs/functions: reusable, needs-refactor, tauri-only
- Document config generation logic (proxy-config.yaml)
- Output: dependency matrix với recommendations

### Task 1.2 – Create Headless Rust Binary Project Structure - Agent_CLI_Core
**Objective:** Tạo project structure cho headless binary trong thư mục `src-headless/`.
**Output:** Cargo.toml và folder structure sẵn sàng cho development.
**Guidance:** **Depends on: Task 1.1 Output** - cần dependency list từ analysis.

- Tạo `src-headless/` với `cargo init --name proxypal-headless`
- Configure Cargo.toml với shared dependencies từ src-tauri
- Setup module structure: `cli/`, `config/`, `proxy/`, `auth/`
- Add clap dependency cho CLI argument parsing

### Task 1.3 – Extract Core Proxy Logic - Agent_CLI_Core
**Objective:** Tách core proxy logic từ `lib.rs` thành reusable modules.
**Output:** Shared library code không phụ thuộc Tauri.
**Guidance:** **Depends on: Task 1.2 Output** - cần project structure.

1. Ad-Hoc Delegation – Code structure analysis
2. Extract `ProxyStatus`, `AppConfig`, `AuthStatus` structs to shared module
3. Extract `start_proxy`, config generation logic
4. Remove Tauri-specific wrappers (`#[tauri::command]`)
5. Create `lib.rs` với public API cho cả Tauri app và headless binary

### Task 1.4 – Implement CLI Command Framework - Agent_CLI_Core
**Objective:** Setup CLI framework với clap, implement subcommand structure.
**Output:** CLI skeleton với help, version, và subcommand routing.
**Guidance:** **Depends on: Task 1.2 Output** - cần project structure.

- Implement main.rs với clap derive macros
- Define subcommands: `serve`, `config`, `auth`
- Add global flags: `--config`, `--verbose`, `--version`
- Implement help text và usage examples

### Task 1.5 – Implement `serve` Command - Agent_CLI_Core
**Objective:** Implement `proxypal serve` command để start proxy daemon.
**Output:** Working serve command với CLIProxyAPI integration.
**Guidance:** **Depends on: Task 1.3 Output, Task 1.4 Output** - cần core logic và CLI framework.

1. Parse config file (YAML) và validate
2. Generate runtime config cho CLIProxyAPI
3. Spawn CLIProxyAPI binary as subprocess
4. Implement signal handling (SIGTERM, SIGINT)
5. Implement foreground vs daemon mode (--daemon flag)

### Task 1.6 – Implement `config validate` Command - Agent_CLI_Core
**Objective:** Implement `proxypal config validate` để kiểm tra config file.
**Output:** Validation command với error reporting.
**Guidance:** **Depends on: Task 1.4 Output** - cần CLI framework.

- Parse YAML file với serde_yaml
- Validate required fields và types
- Output validation result với clear error messages

---

## Phase 2: Service Integration

### Task 2.1 – Create Systemd Unit File - Agent_Service
**Objective:** Tạo systemd unit file cho Ubuntu 20.04+ với auto-restart và logging.
**Output:** `deployment/proxypal.service` file.
**Guidance:** **Depends on: Task 1.5 Output by Agent_CLI_Core** - cần working serve command.

- Create systemd unit file với Type=simple
- Configure auto-restart on failure (Restart=on-failure)
- Setup logging to journald (StandardOutput=journal)
- Add hardening options (NoNewPrivileges, ProtectSystem)

### Task 2.2 – Implement Health Check Endpoint - Agent_Service
**Objective:** Implement health check endpoint cho service monitoring.
**Output:** `/health` endpoint trong proxy hoặc separate check command.
**Guidance:** **Depends on: Task 1.5 Output by Agent_CLI_Core** - cần serve command.

- Add `proxypal health` subcommand
- Check CLIProxyAPI process status
- Check port availability và response
- Return exit code 0/1 cho systemd health checks

### Task 2.3 – Create Windows Service Wrapper - Agent_Service
**Objective:** Tạo Windows service wrapper sử dụng nssm hoặc native Windows Service API.
**Output:** Installation script và service wrapper.
**Guidance:** **Depends on: Task 1.5 Output by Agent_CLI_Core** - cần working serve command.

1. Evaluate nssm vs native Windows Service
2. Create install/uninstall PowerShell scripts
3. Configure service recovery options
4. Document Windows-specific setup

---

## Phase 3: Authentication Headless

### Task 3.1 – Implement Device Code Flow - Agent_Auth
**Objective:** Implement OAuth device code flow cho headless authentication.
**Output:** Device code auth module với CLI output.
**Guidance:** **Depends on: Task 1.4 Output by Agent_CLI_Core** - cần CLI framework. CLIProxyAPI đã có OAuth endpoints.

1. Research OAuth device code flow cho each provider (Claude, OpenAI, Gemini, Copilot)
2. Implement device code request và polling
3. Display user code và verification URL
4. Poll for token completion
5. Store credentials to auth directory

### Task 3.2 – Implement `auth add` Command - Agent_Auth
**Objective:** Implement `proxypal auth add` command cho interactive auth.
**Output:** Auth add command với provider selection.
**Guidance:** **Depends on: Task 3.1 Output** - cần device code flow.

- Implement provider selection menu
- Integrate device code flow
- Validate và store credentials
- Display success/failure status

### Task 3.3 – Implement Auth File Import - Agent_Auth
**Objective:** Implement auth file import từ existing credentials.
**Output:** Auth import command.
**Guidance:** **Depends on: Task 1.4 Output by Agent_CLI_Core** - cần CLI framework.

- Implement `proxypal auth import <file>` command
- Validate credential file format
- Copy credentials to auth directory
- Support bulk import từ directory

---

## Phase 4: Packaging & Distribution

### Task 4.1 – Setup Cross-compilation Build Scripts - Agent_Packaging
**Objective:** Tạo build scripts cho Linux và Windows binaries.
**Output:** Build scripts trong `scripts/` directory.
**Guidance:** **Depends on: Phase 1-3 completion** - cần complete binary.

- Setup cargo build với target triples
- Create build script (bash/Makefile)
- Configure release optimization flags
- Output binaries vào `dist/` folder

### Task 4.2 – Create Dockerfile - Agent_Packaging
**Objective:** Tạo Dockerfile cho container deployment.
**Output:** `Dockerfile` và `docker-compose.yml`.
**Guidance:** **Depends on: Task 4.1 Output** - cần build process.

- Create multi-stage Dockerfile (build + runtime)
- Include CLIProxyAPI binary
- Configure volume mounts cho config và auth
- Create docker-compose.yml với health checks

### Task 4.3 – Create DEB Package - Agent_Packaging
**Objective:** Tạo Debian package cho Ubuntu deployment.
**Output:** `.deb` package file.
**Guidance:** **Depends on: Task 4.1 Output, Task 2.1 Output by Agent_Service** - cần binary và systemd file.

- Create debian package structure
- Include binary, systemd unit, man pages
- Configure postinst/prerm scripts
- Build với dpkg-deb

### Task 4.4 – Create RPM Package - Agent_Packaging
**Objective:** Tạo RPM package cho RHEL/CentOS deployment.
**Output:** `.rpm` package file.
**Guidance:** **Depends on: Task 4.1 Output, Task 2.1 Output by Agent_Service** - cần binary và systemd file.

- Create RPM spec file
- Include binary, systemd unit, man pages
- Configure scriptlets
- Build với rpmbuild

---

## Phase 5: Documentation

### Task 5.1 – Update README with Headless Instructions - Agent_Docs
**Objective:** Cập nhật README.md với headless mode documentation.
**Output:** Updated `README.md`.
**Guidance:** **Depends on: Phase 1-4 completion** - cần complete features.

- Add Headless Mode section
- Document CLI commands và options
- Add quick start guide cho server deployment
- Update feature list

### Task 5.2 – Create Man Pages - Agent_Docs
**Objective:** Tạo man pages cho CLI commands.
**Output:** Man pages trong `docs/man/` directory.
**Guidance:** **Depends on: Phase 1, 3 completion** - cần CLI commands spec.

- Create proxypal.1 (main command)
- Create proxypal-serve.1 (serve subcommand)
- Create proxypal-auth.1 (auth subcommand)
- Create proxypal-config.1 (config subcommand)

### Task 5.3 – Create Deployment Guide - Agent_Docs
**Objective:** Tạo comprehensive deployment guide.
**Output:** `docs/DEPLOYMENT.md`.
**Guidance:** **Depends on: Phase 1-4 completion** - cần complete features.

1. Document Ubuntu deployment với systemd
2. Document Windows deployment với service
3. Document Docker deployment
4. Document configuration options
5. Add troubleshooting section

