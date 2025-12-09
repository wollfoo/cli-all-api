//! Process Management
//! 
//! **Process Module** (Module process - quản lý CLIProxyAPI process)
//! 
//! This module handles spawning and managing the CLIProxyAPI process.
//! Uses std::process instead of Tauri's shell plugin.

use anyhow::{Result, Context, bail};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use tracing::{info, debug};

/// Start CLIProxyAPI sidecar process
/// **start_cliproxyapi** (Khởi động CLIProxyAPI - spawn process)
pub fn start_cliproxyapi(config_path: &Path) -> Result<Child> {
    // Find CLIProxyAPI binary
    // **Find Binary** (Tìm binary - xác định đường dẫn CLIProxyAPI)
    let binary_path = find_cliproxyapi_binary()?;
    
    debug!("Starting CLIProxyAPI from: {}", binary_path.display());
    debug!("Config path: {}", config_path.display());
    
    // Get config directory for WRITABLE_PATH
    // **Writable Path** (Đường dẫn ghi - thư mục CLIProxyAPI có thể ghi)
    let config_dir = config_path.parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid config path"))?;
    
    // Spawn process
    // **Spawn** (Tạo process - khởi động CLIProxyAPI)
    let child = Command::new(&binary_path)
        .args(["--config", config_path.to_str().unwrap()])
        .env("WRITABLE_PATH", config_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn CLIProxyAPI: {}", binary_path.display()))?;
    
    info!("CLIProxyAPI started (PID: {:?})", child.id());
    
    Ok(child)
}

/// Find CLIProxyAPI binary
/// **find_cliproxyapi_binary** (Tìm binary CLIProxyAPI)
fn find_cliproxyapi_binary() -> Result<std::path::PathBuf> {
    // Check common locations
    // **Locations** (Vị trí - các đường dẫn phổ biến)
    let locations = vec![
        // Same directory as this binary
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("cliproxyapi"))),
        // ~/.local/bin (Linux)
        dirs::home_dir().map(|h| h.join(".local/bin/cliproxyapi")),
        // /usr/local/bin (macOS/Linux)
        Some(std::path::PathBuf::from("/usr/local/bin/cliproxyapi")),
        // /opt/homebrew/bin (macOS ARM)
        Some(std::path::PathBuf::from("/opt/homebrew/bin/cliproxyapi")),
        // Current directory
        Some(std::path::PathBuf::from("./cliproxyapi")),
    ];
    
    for location in locations.into_iter().flatten() {
        debug!("Checking for CLIProxyAPI at: {}", location.display());
        
        if location.exists() {
            return Ok(location);
        }
        
        // On Windows, also check .exe extension
        #[cfg(windows)]
        {
            let exe_path = location.with_extension("exe");
            if exe_path.exists() {
                return Ok(exe_path);
            }
        }
    }
    
    // Try PATH
    // **PATH** (Biến PATH - tìm trong system PATH)
    if let Ok(path) = which::which("cliproxyapi") {
        return Ok(path);
    }
    
    bail!(
        "CLIProxyAPI binary not found.\n\
         Please ensure cliproxyapi is installed and in your PATH.\n\
         Download from: https://github.com/nicezic/CLIProxyAPI/releases"
    )
}
