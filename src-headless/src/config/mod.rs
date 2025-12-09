//! Configuration Handling Module
//! 
//! **Config Module** (Module cấu hình - load/save/validate config)
//! 
//! This module handles configuration file operations including:
//! - Loading config from JSON/YAML files  
//! - Validating configuration with comprehensive checks
//! - Generating proxy-config.yaml for CLIProxyAPI

pub mod types;
pub mod loader;

pub use types::*;
pub use loader::*;

use anyhow::{Result, Context, bail};
use std::path::Path;
use tracing::{info, warn, debug};

// ============================================================================
// Validation Types
// ============================================================================

/// Validation result for a single check
#[derive(Debug)]
pub struct ValidationCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub severity: Severity,
}

/// Severity level for validation issues
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Error,    // Must fix
    Warning,  // Should fix
    Info,     // FYI
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "ERROR"),
            Severity::Warning => write!(f, "WARN"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

/// Overall validation result
pub struct ValidationResult {
    pub valid: bool,
    pub checks: Vec<ValidationCheck>,
    pub error_count: usize,
    pub warning_count: usize,
}

impl ValidationResult {
    fn new() -> Self {
        Self {
            valid: true,
            checks: Vec::new(),
            error_count: 0,
            warning_count: 0,
        }
    }
    
    fn add_check(&mut self, name: &str, passed: bool, message: &str, severity: Severity) {
        if !passed && severity == Severity::Error {
            self.valid = false;
            self.error_count += 1;
        }
        if !passed && severity == Severity::Warning {
            self.warning_count += 1;
        }
        
        self.checks.push(ValidationCheck {
            name: name.to_string(),
            passed,
            message: message.to_string(),
            severity,
        });
    }
    
    fn add_error(&mut self, name: &str, message: &str) {
        self.add_check(name, false, message, Severity::Error);
    }
    
    fn add_warning(&mut self, name: &str, message: &str) {
        self.add_check(name, false, message, Severity::Warning);
    }
    
    fn add_info(&mut self, name: &str, message: &str) {
        self.add_check(name, true, message, Severity::Info);
    }
    
    fn add_pass(&mut self, name: &str, message: &str) {
        self.add_check(name, true, message, Severity::Info);
    }
}

// ============================================================================
// Validate Command
// ============================================================================

/// Validate a configuration file
/// **validate** (Xác thực - kiểm tra file config hợp lệ)
/// 
/// Returns Ok(()) if valid, Err if invalid or file issues
pub fn validate(config_path: &str) -> Result<()> {
    let path = Path::new(config_path);
    
    println!();
    println!("Validating configuration: {}", config_path);
    println!("{}", "=".repeat(60));
    
    // Check file exists
    if !path.exists() {
        eprintln!();
        eprintln!("❌ Configuration file not found: {}", config_path);
        eprintln!();
        eprintln!("To create a default configuration:");
        eprintln!("  proxypal config init");
        eprintln!();
        bail!("Configuration file not found");
    }
    
    // Try to read file
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            bail!("Failed to read configuration file: {}", e);
        }
    };
    
    // Check file is not empty
    if content.trim().is_empty() {
        eprintln!("❌ Configuration file is empty");
        bail!("Configuration file is empty");
    }
    
    // Try to parse YAML
    let config: AppConfig = match serde_yaml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!();
            eprintln!("❌ YAML Syntax Error");
            eprintln!("{}", "-".repeat(40));
            eprintln!("{}", e);
            eprintln!();
            eprintln!("Common issues:");
            eprintln!("  - Incorrect indentation (use spaces, not tabs)");
            eprintln!("  - Missing colons after keys");
            eprintln!("  - Invalid boolean values (use true/false)");
            eprintln!();
            bail!("Failed to parse YAML: {}", e);
        }
    };
    
    // Run comprehensive validation
    let result = validate_config(&config);
    
    // Print results
    print_validation_results(&result);
    
    if result.valid {
        println!();
        println!("✅ Configuration is valid");
        println!();
        Ok(())
    } else {
        println!();
        println!("❌ Configuration has {} error(s)", result.error_count);
        if result.warning_count > 0 {
            println!("   (and {} warning(s))", result.warning_count);
        }
        println!();
        bail!("Configuration validation failed");
    }
}

/// Validate AppConfig and return detailed results
fn validate_config(config: &AppConfig) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    // === Port Validation ===
    if config.port == 0 {
        result.add_error("port", "Port cannot be 0");
    } else if config.port < 1024 {
        result.add_warning(
            "port", 
            &format!("Port {} requires root/admin privileges", config.port)
        );
    } else {
        result.add_pass("port", &format!("Port {} is valid", config.port));
    }
    
    // Check common port conflicts
    let conflicts = [
        (3000, "Common dev server port"),
        (5000, "Flask/Python default"),
        (8080, "Common HTTP proxy"),
        (8000, "Django/Python default"),
    ];
    for (port, name) in conflicts {
        if config.port == port {
            result.add_info(
                "port_conflict", 
                &format!("Port {} is commonly used by: {}", port, name)
            );
        }
    }
    
    // === Proxy URL Validation ===
    if !config.proxy_url.is_empty() {
        if !config.proxy_url.starts_with("http://") && !config.proxy_url.starts_with("https://") {
            result.add_error(
                "proxy_url", 
                "Proxy URL must start with http:// or https://"
            );
        } else {
            result.add_pass("proxy_url", &format!("Proxy URL: {}", config.proxy_url));
        }
    }
    
    // === Retry Validation ===
    if config.request_retry > 10 {
        result.add_warning(
            "request_retry", 
            &format!("High retry count ({}) may cause slow failures", config.request_retry)
        );
    }
    
    // === API Keys Validation ===
    let api_key_count = config.gemini_api_keys.len() 
        + config.claude_api_keys.len() 
        + config.codex_api_keys.len();
    
    if api_key_count == 0 {
        result.add_warning(
            "api_keys", 
            "No API keys configured. Add keys with: proxypal auth add"
        );
    } else {
        result.add_pass(
            "api_keys", 
            &format!("{} API key(s) configured", api_key_count)
        );
        
        // Validate individual API keys
        for (i, key) in config.gemini_api_keys.iter().enumerate() {
            if key.api_key.is_empty() {
                result.add_error(
                    &format!("gemini_key_{}", i), 
                    "Empty API key"
                );
            }
        }
        for (i, key) in config.claude_api_keys.iter().enumerate() {
            if key.api_key.is_empty() {
                result.add_error(
                    &format!("claude_key_{}", i), 
                    "Empty API key"
                );
            }
        }
        for (i, key) in config.codex_api_keys.iter().enumerate() {
            if key.api_key.is_empty() {
                result.add_error(
                    &format!("codex_key_{}", i), 
                    "Empty API key"
                );
            }
        }
    }
    
    // === Amp Configuration ===
    if !config.amp_api_key.is_empty() {
        result.add_pass("amp_api_key", "Amp API key configured");
    }
    
    if !config.amp_model_mappings.is_empty() {
        result.add_pass(
            "amp_mappings", 
            &format!("{} model mapping(s) configured", config.amp_model_mappings.len())
        );
    }
    
    if !config.amp_openai_providers.is_empty() {
        result.add_pass(
            "amp_providers", 
            &format!("{} OpenAI-compatible provider(s) configured", config.amp_openai_providers.len())
        );
        
        for (i, provider) in config.amp_openai_providers.iter().enumerate() {
            if provider.base_url.is_empty() {
                result.add_error(
                    &format!("provider_{}_url", i), 
                    &format!("Provider '{}' has empty base_url", provider.name)
                );
            }
            if provider.api_key.is_empty() {
                result.add_error(
                    &format!("provider_{}_key", i), 
                    &format!("Provider '{}' has empty api_key", provider.name)
                );
            }
        }
    }
    
    // === Copilot Configuration ===
    if config.copilot.enabled {
        result.add_pass("copilot", &format!("Copilot enabled on port {}", config.copilot.port));
        
        if config.copilot.port == config.port {
            result.add_error(
                "copilot_port", 
                "Copilot port cannot be the same as proxy port"
            );
        }
    }
    
    // === Summary ===
    debug!("Validation complete: {} checks, {} errors, {} warnings", 
        result.checks.len(), result.error_count, result.warning_count);
    
    result
}

/// Print validation results in a nice format
fn print_validation_results(result: &ValidationResult) {
    println!();
    
    for check in &result.checks {
        let icon = if check.passed { "✓" } else { "✗" };
        let color_prefix = match (check.passed, check.severity) {
            (true, _) => "",
            (false, Severity::Error) => "",
            (false, Severity::Warning) => "",
            (false, Severity::Info) => "",
        };
        
        if check.passed {
            println!("  {} {}: {}", icon, check.name, check.message);
        } else {
            println!("  {} [{}] {}: {}", icon, check.severity, check.name, check.message);
        }
    }
    
    println!();
    println!("{}", "-".repeat(40));
    println!("  Checks:   {}", result.checks.len());
    println!("  Errors:   {}", result.error_count);
    println!("  Warnings: {}", result.warning_count);
}

// ============================================================================
// Show Command
// ============================================================================

/// Show current configuration
/// **show** (Hiển thị - in config hiện tại)
pub fn show(config_path: &str) -> Result<()> {
    let path = Path::new(config_path);
    
    if !path.exists() {
        println!();
        println!("Configuration file not found: {}", config_path);
        println!();
        println!("Using default configuration:");
        println!("{}", "-".repeat(40));
        
        let default_config = AppConfig::default();
        let yaml = serde_yaml::to_string(&default_config)
            .context("Failed to serialize default config")?;
        println!("{}", yaml);
        
        println!();
        println!("To create a configuration file:");
        println!("  proxypal config init");
        
        return Ok(());
    }
    
    // Try to load and re-serialize for consistent formatting
    match load_config_yaml(config_path) {
        Ok(config) => {
            let yaml = serde_yaml::to_string(&config)
                .context("Failed to serialize config")?;
            
            println!("# Configuration from: {}", config_path);
            println!("{}", yaml);
        }
        Err(_) => {
            // If parsing fails, just show raw content
            let content = std::fs::read_to_string(path)
                .context("Failed to read configuration file")?;
            println!("# Raw content from: {}", config_path);
            println!("# (Note: This file may have syntax errors)");
            println!("{}", content);
        }
    }
    
    Ok(())
}

// ============================================================================
// Init Command
// ============================================================================

/// Initialize a new configuration file
/// **init** (Khởi tạo - tạo file config mới)
pub fn init(config_path: &str, force: bool) -> Result<()> {
    let path = Path::new(config_path);
    
    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            info!("Created directory: {}", parent.display());
        }
    }
    
    // Check if file exists
    if path.exists() && !force {
        eprintln!();
        eprintln!("Configuration file already exists: {}", config_path);
        eprintln!();
        eprintln!("Options:");
        eprintln!("  proxypal config init --force   # Overwrite existing");
        eprintln!("  proxypal config show           # View current config");
        eprintln!("  proxypal config validate       # Validate current config");
        eprintln!();
        bail!("File already exists. Use --force to overwrite.");
    }
    
    let default_config = AppConfig::default();
    let yaml = serde_yaml::to_string(&default_config)
        .context("Failed to serialize default configuration")?;
    
    let content = format!(
r#"# ProxyPal Headless Configuration
# Generated by: proxypal config init
# Documentation: https://github.com/wollfoo/cli-all-api
#
# Quick Start:
#   1. Add your API keys:
#      proxypal auth add --provider gemini --api-key YOUR_KEY
#
#   2. Start the proxy:
#      proxypal serve
#
#   3. Configure your AI tools to use:
#      OPENAI_API_BASE=http://localhost:8317/v1
#      OPENAI_API_KEY=proxypal-local

{}"#, 
        yaml
    );
    
    std::fs::write(path, &content)
        .with_context(|| format!("Failed to write configuration file: {}", config_path))?;
    
    println!();
    println!("✅ Configuration file created: {}", config_path);
    println!();
    println!("Next steps:");
    println!("  1. Edit the configuration:");
    println!("     proxypal config edit");
    println!("     # or: {} {}", std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string()), config_path);
    println!();
    println!("  2. Add authentication:");
    println!("     proxypal auth add --provider gemini --api-key YOUR_KEY");
    println!();
    println!("  3. Start the proxy:");
    println!("     proxypal serve");
    println!();
    
    Ok(())
}
