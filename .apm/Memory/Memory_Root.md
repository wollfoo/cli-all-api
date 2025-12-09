# ProxyPal Headless Mode Integration – APM Memory Root
**Chiến lược Memory:** Dynamic-MD
**Tổng quan Dự án:** Mở rộng ProxyPal để chạy như headless daemon service trên Ubuntu server và Windows, không cần GUI. Sử dụng Rust CLI binary tái sử dụng logic từ `lib.rs`, tích hợp CLIProxyAPI (Go binary), hỗ trợ unified API endpoint (OpenAI + Claude format), device code auth flow, và đa dạng packaging (binary, Docker, DEB/RPM).
