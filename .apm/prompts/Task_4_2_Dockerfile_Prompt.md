---
task_ref: "Task 4.2 - Create Dockerfile"
agent_assignment: "Agent_Packaging"
memory_log_path: ".apm/Memory/Phase_04_Packaging_Distribution/Task_4_2_Dockerfile.md"
execution_type: "single-step"
dependency_context: true
ad_hoc_delegation: false
---

# APM Task Assignment: Create Dockerfile

## Task Reference
Implementation Plan: **Task 4.2 - Create Dockerfile** assigned to **Agent_Packaging**

## Context from Dependencies
Dựa trên công việc Task 4.1 của bạn:

**Đầu ra Chính để Sử dụng:**
- `scripts/build-linux.sh` - Linux build script
- `dist/linux-x86_64/proxypal` - Built binary (5.6M)
- Release profile trong Cargo.toml với LTO và strip

**Cách tiếp cận Tích hợp:**
Use multi-stage build: build stage với Rust toolchain, runtime stage với minimal base image.

## Objective
Tạo Dockerfile cho container deployment.

## Detailed Instructions
Hoàn thành tất cả mục trong một response:

1. **Create Multi-stage Dockerfile**:
   ```dockerfile
   # Build stage
   FROM rust:1.75-bookworm AS builder
   WORKDIR /app
   COPY src-headless/ ./
   RUN cargo build --release
   
   # Runtime stage
   FROM debian:bookworm-slim
   COPY --from=builder /app/target/release/proxypal /usr/local/bin/
   # Add CLIProxyAPI binary
   ENTRYPOINT ["proxypal"]
   CMD ["serve", "--foreground"]
   ```

2. **Include CLIProxyAPI Binary**:
   - Download hoặc copy CLIProxyAPI vào image
   - Place in `/usr/local/bin/cliproxyapi`
   - Ensure executable permissions

3. **Configure Volume Mounts**:
   - `/etc/proxypal/` - Config directory
   - `/var/lib/proxypal/auth/` - Auth credentials
   - `/var/log/proxypal/` - Logs (optional)

4. **Create docker-compose.yml**:
   ```yaml
   version: '3.8'
   services:
     proxypal:
       image: proxypal:latest
       ports:
         - "8317:8317"
       volumes:
         - ./config:/etc/proxypal
         - ./auth:/var/lib/proxypal/auth
       restart: unless-stopped
       healthcheck:
         test: ["CMD", "proxypal", "health"]
         interval: 30s
         timeout: 10s
         retries: 3
   ```

5. **Add .dockerignore**:
   - Exclude `target/`, `dist/`, `.git/`
   - Keep only necessary files for build

6. **Test Docker Build**:
   - `docker build -t proxypal:latest .`
   - `docker run --rm proxypal:latest --version`

## Expected Output
- **Sản phẩm**: `Dockerfile` và `docker-compose.yml`
- **Tiêu chí thành công**: `docker build` produces working image
- **Vị trí file**: `/home/azureuser/cli-all-api/`

## Memory Logging
Khi hoàn thành, bạn **PHẢI** ghi log công việc trong: `.apm/Memory/Phase_04_Packaging_Distribution/Task_4_2_Dockerfile.md`
Tuân theo hướng dẫn `.apm/guides/Memory_Log_Guide.md`.

## Reporting Protocol
Sau khi ghi log, bạn **PHẢI** xuất code block **Final Task Report** để Người dùng copy-paste lại cho Manager Agent.
