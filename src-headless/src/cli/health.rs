//! Health Check Module
//! 
//! **Health Module** (Module kiểm tra sức khỏe - kiểm tra trạng thái service)
//! 
//! This module provides health check functionality for:
//! - Process status verification (PID file check)
//! - Port connectivity (TCP connect test)
//! - HTTP endpoint availability

use anyhow::{Result, Context};
use std::path::Path;
use std::time::Duration;
use tracing::{info, debug, warn};

use crate::proxy::is_process_running;

// ============================================================================
// Health Status Types
// ============================================================================

/// Overall health status
/// **HealthStatus** (Trạng thái sức khỏe - kết quả health check tổng hợp)
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    /// All checks passed
    Healthy,
    /// Some checks failed but service may be partially operational
    Degraded,
    /// Service is not running or critical checks failed
    Unhealthy,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "HEALTHY"),
            HealthStatus::Degraded => write!(f, "DEGRADED"),
            HealthStatus::Unhealthy => write!(f, "UNHEALTHY"),
        }
    }
}

/// Individual health check result
/// **HealthCheck** (Kiểm tra đơn lẻ - kết quả của một check cụ thể)
#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
}

/// Comprehensive health check result
/// **HealthResult** (Kết quả health check - tổng hợp tất cả checks)
#[derive(Debug)]
pub struct HealthResult {
    pub status: HealthStatus,
    pub checks: Vec<HealthCheck>,
    pub pid: Option<u32>,
    pub port: u16,
    pub uptime_secs: Option<u64>,
}

impl HealthResult {
    /// Get exit code for systemd/CLI integration
    /// 0 = healthy, 1 = unhealthy
    pub fn exit_code(&self) -> i32 {
        match self.status {
            HealthStatus::Healthy => 0,
            HealthStatus::Degraded => 0, // Still considered OK for systemd
            HealthStatus::Unhealthy => 1,
        }
    }
}

// ============================================================================
// Health Check Implementation
// ============================================================================

/// Perform comprehensive health check
/// **check_health** (Kiểm tra sức khỏe - thực hiện tất cả health checks)
/// 
/// Checks:
/// 1. PID file exists and process is running
/// 2. Port is listening (TCP connect)
/// 3. HTTP endpoint responds (optional)
pub async fn check_health(pid_file: &str, port: u16) -> Result<HealthResult> {
    let mut checks = Vec::new();
    let mut pid = None;
    let mut uptime_secs = None;
    
    // === Check 1: PID File & Process ===
    let (process_check, process_pid, process_uptime) = check_process(pid_file);
    checks.push(process_check);
    pid = process_pid;
    uptime_secs = process_uptime;
    
    // === Check 2: Port Connectivity ===
    let port_check = check_port(port).await;
    checks.push(port_check);
    
    // === Check 3: HTTP Endpoint (only if port is open) ===
    if checks.last().map(|c| c.passed).unwrap_or(false) {
        let http_check = check_http_endpoint(port).await;
        checks.push(http_check);
    }
    
    // === Determine Overall Status ===
    let failed_count = checks.iter().filter(|c| !c.passed).count();
    let status = match failed_count {
        0 => HealthStatus::Healthy,
        1 => HealthStatus::Degraded,
        _ => HealthStatus::Unhealthy,
    };
    
    Ok(HealthResult {
        status,
        checks,
        pid,
        port,
        uptime_secs,
    })
}

/// Check PID file and process status
/// **check_process** (Kiểm tra process - xác nhận daemon đang chạy)
fn check_process(pid_file: &str) -> (HealthCheck, Option<u32>, Option<u64>) {
    let start = std::time::Instant::now();
    let path = Path::new(pid_file);
    
    // Check PID file exists
    if !path.exists() {
        return (
            HealthCheck {
                name: "process".to_string(),
                passed: false,
                message: format!("PID file not found: {}", pid_file),
                duration_ms: start.elapsed().as_millis() as u64,
            },
            None,
            None,
        );
    }
    
    // Read PID from file
    let pid_str = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            return (
                HealthCheck {
                    name: "process".to_string(),
                    passed: false,
                    message: format!("Failed to read PID file: {}", e),
                    duration_ms: start.elapsed().as_millis() as u64,
                },
                None,
                None,
            );
        }
    };
    
    let pid: u32 = match pid_str.trim().parse() {
        Ok(p) => p,
        Err(_) => {
            return (
                HealthCheck {
                    name: "process".to_string(),
                    passed: false,
                    message: "Invalid PID in file".to_string(),
                    duration_ms: start.elapsed().as_millis() as u64,
                },
                None,
                None,
            );
        }
    };
    
    // Check if process is running
    if !is_process_running(pid) {
        return (
            HealthCheck {
                name: "process".to_string(),
                passed: false,
                message: format!("Process {} is not running (stale PID file)", pid),
                duration_ms: start.elapsed().as_millis() as u64,
            },
            Some(pid),
            None,
        );
    }
    
    // Calculate uptime from PID file modification time
    let uptime = path.metadata()
        .ok()
        .and_then(|m| m.created().ok())
        .and_then(|t| t.elapsed().ok())
        .map(|d| d.as_secs());
    
    (
        HealthCheck {
            name: "process".to_string(),
            passed: true,
            message: format!("Process {} is running", pid),
            duration_ms: start.elapsed().as_millis() as u64,
        },
        Some(pid),
        uptime,
    )
}

/// Check if port is open (TCP connect test)
/// **check_port** (Kiểm tra port - xác nhận port đang listening)
async fn check_port(port: u16) -> HealthCheck {
    let start = std::time::Instant::now();
    let addr = format!("127.0.0.1:{}", port);
    
    debug!("Checking port connectivity: {}", addr);
    
    // Use tokio timeout for async TCP connect
    let connect_result = tokio::time::timeout(
        Duration::from_secs(2),
        tokio::net::TcpStream::connect(&addr),
    ).await;
    
    match connect_result {
        Ok(Ok(_stream)) => {
            HealthCheck {
                name: "port".to_string(),
                passed: true,
                message: format!("Port {} is listening", port),
                duration_ms: start.elapsed().as_millis() as u64,
            }
        }
        Ok(Err(e)) => {
            HealthCheck {
                name: "port".to_string(),
                passed: false,
                message: format!("Port {} connection failed: {}", port, e),
                duration_ms: start.elapsed().as_millis() as u64,
            }
        }
        Err(_) => {
            HealthCheck {
                name: "port".to_string(),
                passed: false,
                message: format!("Port {} connection timeout", port),
                duration_ms: start.elapsed().as_millis() as u64,
            }
        }
    }
}

/// Check HTTP endpoint availability
/// **check_http_endpoint** (Kiểm tra HTTP - xác nhận API endpoint hoạt động)
async fn check_http_endpoint(port: u16) -> HealthCheck {
    let start = std::time::Instant::now();
    let url = format!("http://127.0.0.1:{}/", port);
    
    debug!("Checking HTTP endpoint: {}", url);
    
    // Create HTTP client with timeout
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build() 
    {
        Ok(c) => c,
        Err(e) => {
            return HealthCheck {
                name: "http".to_string(),
                passed: false,
                message: format!("Failed to create HTTP client: {}", e),
                duration_ms: start.elapsed().as_millis() as u64,
            };
        }
    };
    
    // Try GET request
    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() || status.as_u16() == 404 {
                // 404 is OK - just means no root handler, but server is running
                HealthCheck {
                    name: "http".to_string(),
                    passed: true,
                    message: format!("HTTP endpoint responding (status: {})", status),
                    duration_ms: start.elapsed().as_millis() as u64,
                }
            } else if status.is_server_error() {
                HealthCheck {
                    name: "http".to_string(),
                    passed: false,
                    message: format!("HTTP endpoint error (status: {})", status),
                    duration_ms: start.elapsed().as_millis() as u64,
                }
            } else {
                // Other status codes (3xx, 4xx) - still means server is running
                HealthCheck {
                    name: "http".to_string(),
                    passed: true,
                    message: format!("HTTP endpoint responding (status: {})", status),
                    duration_ms: start.elapsed().as_millis() as u64,
                }
            }
        }
        Err(e) => {
            HealthCheck {
                name: "http".to_string(),
                passed: false,
                message: format!("HTTP request failed: {}", e),
                duration_ms: start.elapsed().as_millis() as u64,
            }
        }
    }
}

// ============================================================================
// Health Check Command (CLI Entry Point)
// ============================================================================

/// Run health check and display results
/// **run_health_check** (Chạy health check - CLI command entry point)
pub async fn run_health_check(
    pid_file: &str, 
    port: u16,
    json_output: bool,
) -> Result<i32> {
    info!("Running health check...");
    
    let result = check_health(pid_file, port).await?;
    
    if json_output {
        print_json_result(&result)?;
    } else {
        print_result(&result);
    }
    
    Ok(result.exit_code())
}

/// Print health check result as JSON
fn print_json_result(result: &HealthResult) -> Result<()> {
    use serde_json::json;
    
    let checks_json: Vec<_> = result.checks.iter().map(|c| {
        json!({
            "name": c.name,
            "passed": c.passed,
            "message": c.message,
            "duration_ms": c.duration_ms
        })
    }).collect();
    
    let output = json!({
        "status": result.status.to_string(),
        "healthy": result.status == HealthStatus::Healthy,
        "pid": result.pid,
        "port": result.port,
        "uptime_secs": result.uptime_secs,
        "checks": checks_json
    });
    
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Print health check result in human-readable format
fn print_result(result: &HealthResult) {
    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                   ProxyPal Health Check                  ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    
    // Status line
    let status_str = match result.status {
        HealthStatus::Healthy => "✅ HEALTHY",
        HealthStatus::Degraded => "⚠️  DEGRADED",
        HealthStatus::Unhealthy => "❌ UNHEALTHY",
    };
    println!("║  Status:  {:<47}║", status_str);
    
    // PID and Port
    if let Some(pid) = result.pid {
        println!("║  PID:     {:<47}║", pid);
    }
    println!("║  Port:    {:<47}║", result.port);
    
    // Uptime
    if let Some(uptime) = result.uptime_secs {
        let hours = uptime / 3600;
        let mins = (uptime % 3600) / 60;
        let secs = uptime % 60;
        println!("║  Uptime:  {:02}h {:02}m {:02}s{:<36}║", hours, mins, secs, "");
    }
    
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  Checks:                                                 ║");
    
    for check in &result.checks {
        let icon = if check.passed { "✓" } else { "✗" };
        let msg = if check.message.len() > 45 {
            format!("{}...", &check.message[..42])
        } else {
            check.message.clone()
        };
        println!("║    {} {}: {:<43}║", icon, check.name, msg);
    }
    
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();
}

// ============================================================================
// Provider-Specific Health Check
// ============================================================================

/// Check health of a specific provider
/// **check_provider_health** (Kiểm tra provider - xác nhận API provider hoạt động)
pub async fn check_provider_health(provider: &str) -> Result<HealthCheck> {
    let start = std::time::Instant::now();
    
    // TODO: Implement actual provider checks
    // For now, return a placeholder
    warn!("Provider health check not yet fully implemented for: {}", provider);
    
    Ok(HealthCheck {
        name: format!("provider:{}", provider),
        passed: true,
        message: format!("Provider '{}' check passed (placeholder)", provider),
        duration_ms: start.elapsed().as_millis() as u64,
    })
}
