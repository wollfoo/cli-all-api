---
task_ref: "Task 5.1 - Update README with Headless Instructions"
agent_assignment: "Agent_Docs"
memory_log_path: ".apm/Memory/Phase_05_Documentation/Task_5_1_Update_README.md"
execution_type: "single-step"
dependency_context: true
ad_hoc_delegation: false
---

# APM Task Assignment: Update README with Headless Instructions

## Task Reference
Implementation Plan: **Task 5.1 - Update README with Headless Instructions** assigned to **Agent_Docs**

## Context from Dependencies
Nhiệm vụ này **phụ thuộc vào Phase 1-4 completion**:

**Các bước Tích hợp (hoàn thành trong một response):**
1. Đọc existing `README.md` để hiểu current structure
2. Đọc `src-headless/src/main.rs` để hiểu CLI commands
3. Kiểm tra `deployment/` directory để hiểu installation options

**Tóm tắt Đầu ra từ All Previous Phases:**
- **CLI Commands:** serve, config, auth, stop, status, health
- **Installation:** DEB package, Docker, Windows service
- **Authentication:** Device code flow, API key, file import
- **Deployment:** systemd, Windows NSSM

## Objective
Cập nhật README.md với headless mode documentation đầy đủ.

## Detailed Instructions
Hoàn thành tất cả mục trong một response:

1. **Add Headless Mode Section**:
   - Overview của headless mode
   - Key features (daemon mode, multi-platform, packaging)
   - When to use headless vs GUI

2. **Document CLI Commands**:
   ```bash
   # Start proxy server
   proxypal serve [--port PORT] [--foreground]
   
   # Configuration
   proxypal config init
   proxypal config validate
   proxypal config show
   
   # Authentication
   proxypal auth add --provider gemini --api-key YOUR_KEY
   proxypal auth add --device-code
   proxypal auth import credentials.json
   proxypal auth list
   
   # Service management
   proxypal status
   proxypal stop
   proxypal health
   ```

3. **Add Quick Start Guide**:
   - Install (DEB/Docker/Binary)
   - Configure
   - Start
   - Verify

4. **Update Feature List**:
   - Add headless mode features
   - Add supported platforms
   - Add packaging options

5. **Add Installation Section**:
   - Ubuntu/Debian: `sudo dpkg -i proxypal_*.deb`
   - Docker: `docker-compose up -d`
   - Binary: Download and run

## Expected Output
- **Sản phẩm**: Updated `README.md` với headless documentation
- **Tiêu chí thành công**: README covers all CLI commands và installation methods
- **Vị trí file**: `/home/azureuser/cli-all-api/README.md`

## Memory Logging
Khi hoàn thành, bạn **PHẢI** ghi log công việc trong: `.apm/Memory/Phase_05_Documentation/Task_5_1_Update_README.md`
Tuân theo hướng dẫn `.apm/guides/Memory_Log_Guide.md`.

## Reporting Protocol
Sau khi ghi log, bạn **PHẢI** xuất code block **Final Task Report** để Người dùng copy-paste lại cho Manager Agent.
