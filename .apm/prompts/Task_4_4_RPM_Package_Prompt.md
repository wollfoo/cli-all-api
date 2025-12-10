---
task_ref: "Task 4.4 - Create RPM Package"
agent_assignment: "Agent_Packaging"
memory_log_path: ".apm/Memory/Phase_04_Packaging_Distribution/Task_4_4_RPM_Package.md"
execution_type: "single-step"
dependency_context: true
ad_hoc_delegation: false
---

# APM Task Assignment: Create RPM Package

## Task Reference
Implementation Plan: **Task 4.4 - Create RPM Package** assigned to **Agent_Packaging**

## Context from Dependencies
Nhiệm vụ này **phụ thuộc vào Task 4.1 Output và Task 2.1 Output**:

**Các bước Tích hợp (hoàn thành trong một response):**
1. Đọc `packaging/deb/` để tham khảo structure tương tự
2. Đọc `deployment/proxypal.service` để include systemd unit file
3. Kiểm tra `dist/linux-x86_64/proxypal` binary

**Tóm tắt Đầu ra từ Previous Tasks:**
- **Binary**: `dist/linux-x86_64/proxypal`
- **Systemd unit**: `deployment/proxypal.service`
- **DEB structure**: `packaging/deb/` (for reference)

## Objective
Tạo RPM package cho RHEL/CentOS deployment.

## Detailed Instructions
Hoàn thành tất cả mục trong một response:

1. **Create RPM Spec File** `packaging/rpm/proxypal.spec`:
   ```spec
   Name:           proxypal
   Version:        0.1.0
   Release:        1%{?dist}
   Summary:        AI Proxy Server for Claude, OpenAI, Gemini
   
   License:        MIT
   URL:            https://github.com/proxypal/proxypal
   
   Requires:       systemd
   
   %description
   ProxyPal is a unified proxy server that provides a single
   endpoint for multiple AI providers.
   
   %install
   mkdir -p %{buildroot}/usr/local/bin
   mkdir -p %{buildroot}/lib/systemd/system
   install -m 755 proxypal %{buildroot}/usr/local/bin/
   install -m 644 proxypal.service %{buildroot}/lib/systemd/system/
   
   %files
   /usr/local/bin/proxypal
   /lib/systemd/system/proxypal.service
   
   %post
   systemctl daemon-reload
   
   %preun
   systemctl stop proxypal || true
   systemctl disable proxypal || true
   ```

2. **Create Build Script** `scripts/build-rpm.sh`:
   - Setup rpmbuild directory structure
   - Copy binary and service file
   - Build with `rpmbuild -bb`
   - Output to `dist/proxypal-0.1.0-1.x86_64.rpm`

3. **Configure Scriptlets**:
   - `%post`: daemon-reload, enable service
   - `%preun`: stop and disable service
   - `%postun`: cleanup

4. **Include Man Pages**:
   - Copy man page to `/usr/share/man/man1/`

5. **Test RPM Build** (nếu có rpmbuild):
   - `./scripts/build-rpm.sh`
   - Verify với `rpm -qip dist/proxypal-*.rpm`

## Expected Output
- **Sản phẩm**: `.rpm` package file
- **Tiêu chí thành công**: `rpm -i proxypal-*.rpm` installs package
- **Vị trí file**: `/home/azureuser/cli-all-api/packaging/rpm/`, `dist/`

## Memory Logging
Khi hoàn thành, bạn **PHẢI** ghi log công việc trong: `.apm/Memory/Phase_04_Packaging_Distribution/Task_4_4_RPM_Package.md`
Tuân theo hướng dẫn `.apm/guides/Memory_Log_Guide.md`.

## Reporting Protocol
Sau khi ghi log, bạn **PHẢI** xuất code block **Final Task Report** để Người dùng copy-paste lại cho Manager Agent.
