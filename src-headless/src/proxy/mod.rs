//! Proxy Management Module
//! 
//! **Proxy Module** (Module proxy - quản lý CLIProxyAPI process)
//! 
//! This module handles:
//! - Starting/stopping CLIProxyAPI sidecar
//! - Generating proxy-config.yaml
//! - Health checks and status monitoring
//! - Signal handling for graceful shutdown

mod config_gen;
mod process;

pub use config_gen::*;
pub use process::*;

use crate::config::load_config;
use anyhow::{Result, Context, bail};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, error, warn};

/// Static flag for shutdown signal
/// **SHUTDOWN_FLAG** (Cờ tắt - flag shutdown toàn cục)
static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Start the proxy server
/// **serve** (Phục vụ - khởi động proxy server)
pub async fn serve(
    config_path: &str,
    port_override: Option<u16>,
    foreground: bool,
    pid_file: &str,
) -> Result<()> {
    // Load configuration
    // **Load** (Tải - đọc config từ file)
    let mut config = load_config(config_path)
        .context("Failed to load configuration")?;
    
    // Apply port override if specified
    // **Override** (Ghi đè - thay đổi port nếu có tham số)
    if let Some(port) = port_override {
        config.port = port;
    }
    
    info!("Starting ProxyPal on port {}", config.port);
    
    // Setup signal handler
    // **Signal Handler** (Xử lý tín hiệu - bắt SIGINT/SIGTERM)
    setup_signal_handler()?;
    
    if !foreground {
        // TODO: Implement daemonization
        // **Daemonize** (Chạy nền - chuyển sang daemon mode)
        warn!("Daemon mode not yet implemented, running in foreground");
    }
    
    // Generate CLIProxyAPI config
    // **Generate Config** (Tạo config - sinh proxy-config.yaml)
    let proxy_config_path = generate_proxy_config(&config)?;
    info!("Generated proxy config: {}", proxy_config_path.display());
    
    // Start CLIProxyAPI process
    // **Start Process** (Khởi động process - chạy CLIProxyAPI)
    let mut child = start_cliproxyapi(&proxy_config_path)?;
    
    // Write PID file
    // **PID File** (File PID - lưu process ID)
    let pid = child.id();
    std::fs::write(pid_file, pid.to_string())
        .context("Failed to write PID file")?;
    info!("PID file written: {} (PID: {})", pid_file, pid);
    
    // Wait for shutdown signal
    // **Wait** (Chờ - chờ tín hiệu shutdown)
    info!("Proxy server running. Press Ctrl+C to stop.");
    
    while !SHUTDOWN_REQUESTED.load(Ordering::Relaxed) {
        // Check if process is still running
        // **Health Check** (Kiểm tra sức khỏe - xác nhận process còn chạy)
        match child.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    info!("CLIProxyAPI exited normally");
                } else {
                    error!("CLIProxyAPI exited with status: {:?}", status);
                }
                break;
            }
            Ok(None) => {
                // Still running, sleep a bit
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
            Err(e) => {
                error!("Error checking process status: {}", e);
                break;
            }
        }
    }
    
    // Cleanup
    // **Cleanup** (Dọn dẹp - dừng process và xóa PID file)
    info!("Shutting down...");
    
    if let Err(e) = child.kill() {
        warn!("Failed to kill process: {}", e);
    }
    
    if Path::new(pid_file).exists() {
        let _ = std::fs::remove_file(pid_file);
    }
    
    info!("Proxy server stopped");
    
    Ok(())
}

/// Stop a running daemon
/// **stop** (Dừng - dừng daemon đang chạy)
pub fn stop(pid_file: &str) -> Result<()> {
    let pid_path = Path::new(pid_file);
    
    if !pid_path.exists() {
        bail!("PID file not found: {}\nIs the daemon running?", pid_file);
    }
    
    let pid_str = std::fs::read_to_string(pid_path)
        .context("Failed to read PID file")?;
    
    let pid: u32 = pid_str.trim().parse()
        .context("Invalid PID in file")?;
    
    info!("Sending SIGTERM to process {}", pid);
    
    // Send SIGTERM
    // **SIGTERM** (Tín hiệu dừng - gửi signal để dừng process)
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, use taskkill
        std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output()
            .context("Failed to kill process")?;
    }
    
    // Remove PID file
    std::fs::remove_file(pid_path)?;
    
    info!("Daemon stopped (PID: {})", pid);
    
    Ok(())
}

/// Show daemon status
/// **status** (Trạng thái - hiển thị trạng thái daemon)
pub fn status(pid_file: &str) -> Result<()> {
    let pid_path = Path::new(pid_file);
    
    if !pid_path.exists() {
        println!("Status: STOPPED");
        println!("PID file not found: {}", pid_file);
        return Ok(());
    }
    
    let pid_str = std::fs::read_to_string(pid_path)
        .context("Failed to read PID file")?;
    
    let pid: u32 = pid_str.trim().parse()
        .context("Invalid PID in file")?;
    
    // Check if process is running
    // **Check Process** (Kiểm tra process - xác nhận còn chạy không)
    let running = is_process_running(pid);
    
    if running {
        println!("Status: RUNNING");
        println!("PID: {}", pid);
        println!("PID file: {}", pid_file);
    } else {
        println!("Status: STOPPED (stale PID file)");
        println!("Removing stale PID file...");
        std::fs::remove_file(pid_path)?;
    }
    
    Ok(())
}

/// Setup signal handler for graceful shutdown
/// **setup_signal_handler** (Cài đặt signal handler - bắt SIGINT/SIGTERM)
fn setup_signal_handler() -> Result<()> {
    ctrlc::set_handler(move || {
        info!("Received shutdown signal");
        SHUTDOWN_REQUESTED.store(true, Ordering::Relaxed);
    }).context("Failed to set signal handler")?;
    
    Ok(())
}

/// Check if a process is running
/// **is_process_running** (Kiểm tra process chạy - xác nhận PID còn active)
fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        // Send signal 0 to check if process exists
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        Command::new("tasklist")
            .args(["/FI", &format!("PID eq {}", pid)])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}
