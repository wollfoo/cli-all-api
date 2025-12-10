---
task_ref: "Task 4.3 - Create DEB Package"
agent_assignment: "Agent_Packaging"
memory_log_path: ".apm/Memory/Phase_04_Packaging_Distribution/Task_4_3_DEB_Package.md"
execution_type: "single-step"
dependency_context: true
ad_hoc_delegation: false
---

# APM Task Assignment: Create DEB Package

## Task Reference
Implementation Plan: **Task 4.3 - Create DEB Package** assigned to **Agent_Packaging**

## Context from Dependencies
Nhiệm vụ này **phụ thuộc vào Task 4.1 Output và Task 2.1 Output**:

**Các bước Tích hợp (hoàn thành trong một response):**
1. Đọc `scripts/build-linux.sh` để hiểu build process
2. Đọc `deployment/proxypal.service` để include systemd unit file
3. Kiểm tra `dist/linux-x86_64/proxypal` binary đã build

**Tóm tắt Đầu ra từ Previous Tasks:**
- **Binary**: `dist/linux-x86_64/proxypal` (5.6M)
- **Systemd unit**: `deployment/proxypal.service`
- **Install script**: `deployment/install-systemd.sh`

**Yêu cầu Tích hợp:**
- DEB package phải include binary, systemd unit, man pages
- postinst script để enable service
- prerm script để stop service before uninstall

## Objective
Tạo Debian package cho Ubuntu deployment.

## Detailed Instructions
Hoàn thành tất cả mục trong một response:

1. **Create Debian Package Structure** trong `packaging/deb/`:
   ```
   packaging/deb/
   ├── DEBIAN/
   │   ├── control         # Package metadata
   │   ├── postinst        # Post-install script
   │   ├── prerm           # Pre-remove script
   │   └── conffiles       # Config files list
   ├── usr/
   │   ├── local/bin/
   │   │   └── proxypal    # Binary
   │   └── share/man/man1/
   │       └── proxypal.1  # Man page
   └── lib/systemd/system/
       └── proxypal.service
   ```

2. **Create control file**:
   ```
   Package: proxypal
   Version: 0.1.0
   Section: net
   Priority: optional
   Architecture: amd64
   Depends: libc6 (>= 2.31)
   Maintainer: ProxyPal Team <team@proxypal.dev>
   Description: AI Proxy Server for Claude, OpenAI, Gemini
    ProxyPal is a unified proxy server that provides a single
    endpoint for multiple AI providers with automatic auth management.
   ```

3. **Create postinst script**:
   - Create proxypal user if not exists
   - Create config directories
   - Enable systemd service
   - Print post-install message

4. **Create prerm script**:
   - Stop service before removal
   - Disable service

5. **Create Build Script** `scripts/build-deb.sh`:
   - Copy files to package structure
   - Set correct permissions (755 for binary, 644 for configs)
   - Build with `dpkg-deb --build`
   - Output to `dist/proxypal_0.1.0_amd64.deb`

6. **Test DEB Build** (nếu có dpkg-deb):
   - `./scripts/build-deb.sh`
   - Verify với `dpkg -I dist/proxypal_*.deb`

## Expected Output
- **Sản phẩm**: `.deb` package file
- **Tiêu chí thành công**: `dpkg -i proxypal_*.deb` installs package
- **Vị trí file**: `/home/azureuser/cli-all-api/packaging/deb/`, `dist/`

## Memory Logging
Khi hoàn thành, bạn **PHẢI** ghi log công việc trong: `.apm/Memory/Phase_04_Packaging_Distribution/Task_4_3_DEB_Package.md`
Tuân theo hướng dẫn `.apm/guides/Memory_Log_Guide.md`.

## Reporting Protocol
Sau khi ghi log, bạn **PHẢI** xuất code block **Final Task Report** để Người dùng copy-paste lại cho Manager Agent.
