//! Process Management - Cross-platform Process Manager Trait
//! 
//! **Process Module** (Module process - quản lý CLIProxyAPI process)
//! 
//! This module provides a trait-based abstraction for process management,
//! allowing both Tauri (via tauri_plugin_shell) and headless (via std::process)
//! implementations to share the same interface.

use anyhow::{Result, Context, bail};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use tracing::{info, debug};

// ============================================================================
// Process Manager Trait
// ============================================================================

/// Handle representing a spawned process
/// **ProcessHandle** (Handle process - đại diện cho process đã spawn)
pub struct ProcessHandle {
    pub pid: u32,
    pub child: Option<Child>,
}

impl ProcessHandle {
    /// Get the process ID
    pub fn id(&self) -> u32 {
        self.pid
    }
    
    /// Check if process is still running
    pub fn is_running(&self) -> bool {
        is_process_running(self.pid)
    }
    
    /// Wait for process to exit (blocking)
    pub fn wait(&mut self) -> Result<std::process::ExitStatus> {
        if let Some(ref mut child) = self.child {
            child.wait().context("Failed to wait for process")
        } else {
            bail!("No child process to wait for")
        }
    }
    
    /// Try to get exit status without blocking
    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        if let Some(ref mut child) = self.child {
            child.try_wait().context("Failed to check process status")
        } else {
            Ok(None)
        }
    }
    
    /// Kill the process
    pub fn kill(&mut self) -> Result<()> {
        if let Some(ref mut child) = self.child {
            child.kill().context("Failed to kill process")?;
        } else {
            // Kill by PID
            kill_by_pid(self.pid)?;
        }
        Ok(())
    }
}

/// Process Manager trait for abstracting process spawning
/// **ProcessManager** (Quản lý process - trait cho spawning/killing processes)
pub trait ProcessManager: Send + Sync {
    /// Spawn a new process
    fn spawn_process(
        &self,
        cmd: &str,
        args: &[&str],
        env_vars: &[(&str, &str)],
    ) -> Result<ProcessHandle>;
    
    /// Kill a process by handle
    fn kill_process(&self, handle: &mut ProcessHandle) -> Result<()>;
    
    /// Check if a process is running
    fn is_running(&self, handle: &ProcessHandle) -> bool;
}

// ============================================================================
// Standard Library Implementation
// ============================================================================

/// Standard library process manager using std::process::Command
/// **StdProcessManager** (Process manager dùng std::process)
pub struct StdProcessManager;

impl Default for StdProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StdProcessManager {
    pub fn new() -> Self {
        Self
    }
}

impl ProcessManager for StdProcessManager {
    fn spawn_process(
        &self,
        cmd: &str,
        args: &[&str],
        env_vars: &[(&str, &str)],
    ) -> Result<ProcessHandle> {
        debug!("Spawning process: {} {:?}", cmd, args);
        
        let mut command = Command::new(cmd);
        command
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        for (key, value) in env_vars {
            command.env(key, value);
        }
        
        let child = command.spawn()
            .with_context(|| format!("Failed to spawn: {}", cmd))?;
        
        let pid = child.id();
        info!("Process spawned (PID: {})", pid);
        
        Ok(ProcessHandle {
            pid,
            child: Some(child),
        })
    }
    
    fn kill_process(&self, handle: &mut ProcessHandle) -> Result<()> {
        handle.kill()
    }
    
    fn is_running(&self, handle: &ProcessHandle) -> bool {
        handle.is_running()
    }
}

// ============================================================================
// CLIProxyAPI Specific Functions
// ============================================================================

/// Start CLIProxyAPI sidecar process
/// **start_cliproxyapi** (Khởi động CLIProxyAPI - spawn process)
pub fn start_cliproxyapi(config_path: &Path) -> Result<ProcessHandle> {
    let binary_path = find_cliproxyapi_binary()?;
    
    debug!("Starting CLIProxyAPI from: {}", binary_path.display());
    debug!("Config path: {}", config_path.display());
    
    let config_dir = config_path.parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid config path"))?;
    
    let manager = StdProcessManager::new();
    
    manager.spawn_process(
        binary_path.to_str().unwrap(),
        &["--config", config_path.to_str().unwrap()],
        &[("WRITABLE_PATH", config_dir.to_str().unwrap())],
    )
}

/// Find CLIProxyAPI binary in common locations
/// **find_cliproxyapi_binary** (Tìm binary CLIProxyAPI)
pub fn find_cliproxyapi_binary() -> Result<std::path::PathBuf> {
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
        
        #[cfg(windows)]
        {
            let exe_path = location.with_extension("exe");
            if exe_path.exists() {
                return Ok(exe_path);
            }
        }
    }
    
    // Try PATH
    if let Ok(path) = which::which("cliproxyapi") {
        return Ok(path);
    }
    
    bail!(
        "CLIProxyAPI binary not found.\n\
         Please ensure cliproxyapi is installed and in your PATH.\n\
         Download from: https://github.com/nicezic/CLIProxyAPI/releases"
    )
}

// ============================================================================
// Platform-specific Helper Functions
// ============================================================================

/// Check if a process is running by PID
/// **is_process_running** (Kiểm tra process chạy - xác nhận PID còn active)
pub fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
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

/// Kill a process by PID
/// **kill_by_pid** (Kill process bằng PID)
pub fn kill_by_pid(pid: u32) -> Result<()> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::kill(pid as i32, libc::SIGTERM) };
        if result != 0 {
            bail!("Failed to kill process {}", pid);
        }
        Ok(())
    }
    
    #[cfg(windows)]
    {
        std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output()
            .context("Failed to kill process")?;
        Ok(())
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        bail!("kill_by_pid not supported on this platform")
    }
}
