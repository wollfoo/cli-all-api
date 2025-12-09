//! Proxy Management Module
//! 
//! **Proxy Module** (Module proxy - quản lý CLIProxyAPI process)
//! 
//! This module handles:
//! - Starting/stopping CLIProxyAPI sidecar
//! - Generating proxy-config.yaml
//! - Health checks and status monitoring
//! - Signal handling for graceful shutdown

pub mod config_gen;
pub mod process;

pub use config_gen::*;
pub use process::*;

use crate::config::load_config_yaml;
use anyhow::{Result, Context, bail};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, error, warn};

/// Static flag for shutdown signal
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
    let mut config = load_config_yaml(config_path)
        .context("Failed to load configuration")?;
    
    // Apply port override if specified
    if let Some(port) = port_override {
        config.port = port;
    }
    
    info!("Starting ProxyPal on port {}", config.port);
    
    // Setup signal handler
    setup_signal_handler()?;
    
    if !foreground {
        warn!("Daemon mode not yet implemented, running in foreground");
    }
    
    // Generate CLIProxyAPI config
    let proxy_config_path = generate_proxy_config(&config)?;
    info!("Generated proxy config: {}", proxy_config_path.display());
    
    // Start CLIProxyAPI process
    let mut handle = start_cliproxyapi(&proxy_config_path)?;
    
    // Write PID file
    let pid = handle.id();
    std::fs::write(pid_file, pid.to_string())
        .context("Failed to write PID file")?;
    info!("PID file written: {} (PID: {})", pid_file, pid);
    
    // Wait for shutdown signal
    info!("Proxy server running. Press Ctrl+C to stop.");
    
    while !SHUTDOWN_REQUESTED.load(Ordering::Relaxed) {
        match handle.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    info!("CLIProxyAPI exited normally");
                } else {
                    error!("CLIProxyAPI exited with status: {:?}", status);
                }
                break;
            }
            Ok(None) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
            Err(e) => {
                error!("Error checking process status: {}", e);
                break;
            }
        }
    }
    
    // Cleanup
    info!("Shutting down...");
    
    if let Err(e) = handle.kill() {
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
    
    kill_by_pid(pid)?;
    
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
fn setup_signal_handler() -> Result<()> {
    ctrlc::set_handler(move || {
        info!("Received shutdown signal");
        SHUTDOWN_REQUESTED.store(true, Ordering::Relaxed);
    }).context("Failed to set signal handler")?;
    
    Ok(())
}
