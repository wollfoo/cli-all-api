//! Proxy Management Module
//! 
//! **Proxy Module** (Module proxy - quản lý CLIProxyAPI process)
//! 
//! This module handles:
//! - Starting/stopping CLIProxyAPI sidecar
//! - Generating proxy-config.yaml
//! - Health checks and status monitoring
//! - Signal handling for graceful shutdown
//! - Daemon mode with PID file management

pub mod config_gen;
pub mod process;

pub use config_gen::*;
pub use process::*;

use crate::config::{load_config_yaml, AppConfig};
use anyhow::{Result, Context, bail};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc; // Reserved for future use
use tracing::{info, error, warn, debug};

/// Static flag for shutdown signal
static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

// ============================================================================
// Serve Command Implementation
// ============================================================================

/// Start the proxy server
/// **serve** (Phục vụ - khởi động proxy server)
/// 
/// This function:
/// 1. Loads and validates configuration
/// 2. Generates proxy-config.yaml for CLIProxyAPI
/// 3. Starts CLIProxyAPI as a subprocess
/// 4. Manages lifecycle (PID file, signal handling)
pub async fn serve(
    config_path: &str,
    port_override: Option<u16>,
    foreground: bool,
    pid_file: &str,
) -> Result<()> {
    // Check if CLIProxyAPI binary exists first
    let cliproxyapi_path = match find_cliproxyapi_binary() {
        Ok(path) => {
            info!("Found CLIProxyAPI at: {}", path.display());
            path
        }
        Err(e) => {
            error!("CLIProxyAPI binary not found!");
            eprintln!("\n{}", "=".repeat(60));
            eprintln!("ERROR: CLIProxyAPI binary not found");
            eprintln!("{}", "=".repeat(60));
            eprintln!("\nProxyPal requires CLIProxyAPI to function.");
            eprintln!("\nTo install CLIProxyAPI:");
            eprintln!("  1. Download from: https://github.com/nicezic/CLIProxyAPI/releases");
            eprintln!("  2. Extract and place in one of:");
            eprintln!("     - ~/.local/bin/cliproxyapi");
            eprintln!("     - /usr/local/bin/cliproxyapi");
            eprintln!("     - Same directory as proxypal");
            eprintln!("  3. Ensure it's executable: chmod +x cliproxyapi");
            eprintln!("\nAlternatively, add cliproxyapi to your PATH.");
            eprintln!("{}\n", "=".repeat(60));
            return Err(e);
        }
    };
    
    // Load configuration
    let config = load_or_create_config(config_path, port_override).await?;
    
    // Validate configuration
    validate_config(&config)?;
    
    info!("Starting ProxyPal on port {}", config.port);
    info!("Endpoint: http://localhost:{}/v1", config.port);
    
    // Check if already running
    if Path::new(pid_file).exists() {
        let existing_pid = std::fs::read_to_string(pid_file)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok());
        
        if let Some(pid) = existing_pid {
            if is_process_running(pid) {
                bail!(
                    "ProxyPal is already running (PID: {})\n\
                     Use 'proxypal stop' to stop the existing instance, or\n\
                     Use 'proxypal status' to check the current status.",
                    pid
                );
            } else {
                warn!("Found stale PID file, removing...");
                std::fs::remove_file(pid_file).ok();
            }
        }
    }
    
    // Setup signal handler (do this before starting process)
    setup_signal_handler()?;
    
    // Handle daemon mode
    if !foreground {
        #[cfg(unix)]
        {
            info!("Starting in daemon mode...");
            // For now, just warn - full daemonization requires more work
            // The daemonize crate would detach from terminal
            warn!("Full daemon mode not yet implemented, running in background-like mode");
        }
        
        #[cfg(windows)]
        {
            warn!("Daemon mode on Windows not yet implemented, running in foreground");
        }
    }
    
    // Generate CLIProxyAPI config
    let proxy_config_path = generate_proxy_config(&config)?;
    info!("Generated proxy config: {}", proxy_config_path.display());
    debug!("Config contents written to: {}", proxy_config_path.display());
    
    // Start CLIProxyAPI process
    info!("Starting CLIProxyAPI...");
    let mut handle = start_cliproxyapi(&proxy_config_path)
        .context("Failed to start CLIProxyAPI")?;
    
    let pid = handle.id();
    info!("CLIProxyAPI started (PID: {})", pid);
    
    // Write PID file
    let pid_file_path = expand_pid_path(pid_file)?;
    std::fs::write(&pid_file_path, pid.to_string())
        .with_context(|| format!("Failed to write PID file: {}", pid_file_path.display()))?;
    info!("PID file: {}", pid_file_path.display());
    
    // Print startup banner
    print_startup_banner(&config);
    
    // Wait for shutdown signal or process exit
    info!("Proxy server running. Press Ctrl+C to stop.");
    
    let exit_status = run_main_loop(&mut handle).await;
    
    // Cleanup
    cleanup(pid_file, &mut handle);
    
    match exit_status {
        Ok(()) => {
            info!("Proxy server stopped gracefully");
            Ok(())
        }
        Err(e) => {
            error!("Proxy server stopped with error: {}", e);
            Err(e)
        }
    }
}

/// Load config from file, or create default if not exists
async fn load_or_create_config(config_path: &str, port_override: Option<u16>) -> Result<AppConfig> {
    let path = Path::new(config_path);
    
    let mut config = if path.exists() {
        debug!("Loading configuration from: {}", config_path);
        load_config_yaml(config_path)
            .with_context(|| format!("Failed to load configuration from: {}", config_path))?
    } else {
        warn!("Configuration file not found: {}", config_path);
        info!("Using default configuration");
        
        // Create parent directories
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .context("Failed to create config directory")?;
            }
        }
        
        let default_config = AppConfig::default();
        
        // Optionally save default config
        debug!("Using default configuration (no file created)");
        
        default_config
    };
    
    // Apply port override if specified
    if let Some(port) = port_override {
        info!("Overriding port to: {}", port);
        config.port = port;
    }
    
    Ok(config)
}

/// Validate configuration has required fields
fn validate_config(config: &AppConfig) -> Result<()> {
    // Port validation
    if config.port == 0 {
        bail!("Invalid port: 0. Port must be between 1 and 65535");
    }
    
    // Warn about common issues
    if config.port < 1024 {
        warn!("Port {} requires root/admin privileges", config.port);
    }
    
    // Check for common port conflicts
    let common_ports = [(8080, "HTTP proxy"), (3000, "Dev server"), (5000, "Flask")];
    for (port, name) in common_ports {
        if config.port == port {
            debug!("Using port {} commonly used by {}", port, name);
        }
    }
    
    Ok(())
}

/// Expand PID file path (handle relative paths)
fn expand_pid_path(pid_file: &str) -> Result<PathBuf> {
    let path = Path::new(pid_file);
    
    // If absolute, use as-is
    if path.is_absolute() {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create PID directory: {}", parent.display()))?;
            }
        }
        return Ok(path.to_path_buf());
    }
    
    // For relative paths, use /tmp or config directory
    let expanded = if pid_file.starts_with("./") {
        std::env::current_dir()?.join(&pid_file[2..])
    } else {
        PathBuf::from(pid_file)
    };
    
    Ok(expanded)
}

/// Print startup banner
fn print_startup_banner(config: &AppConfig) {
    eprintln!();
    eprintln!("╔══════════════════════════════════════════════════════════╗");
    eprintln!("║                    ProxyPal Headless                     ║");
    eprintln!("╠══════════════════════════════════════════════════════════╣");
    eprintln!("║  Status:    RUNNING                                      ║");
    eprintln!("║  Endpoint:  http://localhost:{:<5}/v1                   ║", config.port);
    eprintln!("║  Debug:     {:<46}║", if config.debug { "enabled" } else { "disabled" });
    eprintln!("╠══════════════════════════════════════════════════════════╣");
    eprintln!("║  Configure your AI tools to use:                         ║");
    eprintln!("║    OPENAI_API_BASE=http://localhost:{}/v1              ║", config.port);
    eprintln!("║    OPENAI_API_KEY=proxypal-local                         ║");
    eprintln!("╚══════════════════════════════════════════════════════════╝");
    eprintln!();
}

/// Main event loop - wait for shutdown or process exit
async fn run_main_loop(handle: &mut ProcessHandle) -> Result<()> {
    loop {
        // Check for shutdown signal
        if SHUTDOWN_REQUESTED.load(Ordering::Relaxed) {
            info!("Shutdown signal received");
            return Ok(());
        }
        
        // Check if process is still running
        match handle.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    info!("CLIProxyAPI exited normally");
                    return Ok(());
                } else {
                    let code = status.code().unwrap_or(-1);
                    error!("CLIProxyAPI exited with code: {}", code);
                    bail!("CLIProxyAPI exited unexpectedly (exit code: {})", code);
                }
            }
            Ok(None) => {
                // Still running, continue waiting
            }
            Err(e) => {
                error!("Error checking process status: {}", e);
                bail!("Failed to check CLIProxyAPI status: {}", e);
            }
        }
        
        // Sleep before next check
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}

/// Cleanup on shutdown
fn cleanup(pid_file: &str, handle: &mut ProcessHandle) {
    info!("Cleaning up...");
    
    // Kill process if still running
    if handle.is_running() {
        info!("Stopping CLIProxyAPI...");
        if let Err(e) = handle.kill() {
            warn!("Failed to kill CLIProxyAPI: {}", e);
        } else {
            info!("CLIProxyAPI stopped");
        }
    }
    
    // Remove PID file
    if Path::new(pid_file).exists() {
        if let Err(e) = std::fs::remove_file(pid_file) {
            warn!("Failed to remove PID file: {}", e);
        } else {
            debug!("PID file removed");
        }
    }
}

// ============================================================================
// Stop Command Implementation
// ============================================================================

/// Stop a running daemon
/// **stop** (Dừng - dừng daemon đang chạy)
pub fn stop(pid_file: &str) -> Result<()> {
    let pid_path = Path::new(pid_file);
    
    if !pid_path.exists() {
        eprintln!("ProxyPal is not running (no PID file found)");
        eprintln!("PID file path: {}", pid_file);
        return Ok(());
    }
    
    let pid_str = std::fs::read_to_string(pid_path)
        .context("Failed to read PID file")?;
    
    let pid: u32 = pid_str.trim().parse()
        .context("Invalid PID in file")?;
    
    // Check if process is actually running
    if !is_process_running(pid) {
        warn!("Process {} is not running (stale PID file)", pid);
        std::fs::remove_file(pid_path)?;
        println!("Removed stale PID file");
        return Ok(());
    }
    
    info!("Stopping ProxyPal (PID: {})...", pid);
    
    // Send SIGTERM
    kill_by_pid(pid)?;
    
    // Wait briefly for process to exit
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if !is_process_running(pid) {
            break;
        }
    }
    
    // Check if stopped
    if is_process_running(pid) {
        warn!("Process did not exit gracefully, sending SIGKILL...");
        #[cfg(unix)]
        unsafe {
            libc::kill(pid as i32, libc::SIGKILL);
        }
    }
    
    // Remove PID file
    std::fs::remove_file(pid_path)?;
    
    println!("ProxyPal stopped (PID: {})", pid);
    
    Ok(())
}

// ============================================================================
// Status Command Implementation  
// ============================================================================

/// Show daemon status
/// **status** (Trạng thái - hiển thị trạng thái daemon)
pub fn status(pid_file: &str) -> Result<()> {
    let pid_path = Path::new(pid_file);
    
    println!();
    println!("ProxyPal Status");
    println!("{}", "=".repeat(40));
    
    if !pid_path.exists() {
        println!("Status:   STOPPED");
        println!("PID file: {} (not found)", pid_file);
        println!();
        println!("Start with: proxypal serve");
        return Ok(());
    }
    
    let pid_str = std::fs::read_to_string(pid_path)
        .context("Failed to read PID file")?;
    
    let pid: u32 = pid_str.trim().parse()
        .context("Invalid PID in file")?;
    
    if is_process_running(pid) {
        println!("Status:   RUNNING");
        println!("PID:      {}", pid);
        println!("PID file: {}", pid_file);
        
        // Try to get more info
        #[cfg(unix)]
        {
            // Check process uptime (rough estimate)
            if let Ok(stat) = std::fs::metadata(pid_path) {
                if let Ok(created) = stat.created() {
                    if let Ok(duration) = created.elapsed() {
                        let hours = duration.as_secs() / 3600;
                        let mins = (duration.as_secs() % 3600) / 60;
                        println!("Uptime:   {}h {}m", hours, mins);
                    }
                }
            }
        }
        
        println!();
        println!("Stop with: proxypal stop");
    } else {
        println!("Status:   STOPPED (stale PID file)");
        println!("PID:      {} (not running)", pid);
        println!();
        println!("Cleaning up stale PID file...");
        std::fs::remove_file(pid_path)?;
        println!("Start with: proxypal serve");
    }
    
    println!("{}", "=".repeat(40));
    
    Ok(())
}

// ============================================================================
// Signal Handling
// ============================================================================

/// Setup signal handler for graceful shutdown
fn setup_signal_handler() -> Result<()> {
    // Reset flag in case of previous run
    SHUTDOWN_REQUESTED.store(false, Ordering::Relaxed);
    
    ctrlc::set_handler(move || {
        eprintln!("\nReceived shutdown signal (Ctrl+C)");
        SHUTDOWN_REQUESTED.store(true, Ordering::Relaxed);
    }).context("Failed to set signal handler")?;
    
    debug!("Signal handler installed (SIGINT/SIGTERM)");
    
    Ok(())
}
