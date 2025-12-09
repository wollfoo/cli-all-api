//! Configuration Loader - Config I/O Functions
//! 
//! **Config Loader** (Bộ tải cấu hình - đọc/ghi config từ file)
//! 
//! These functions are extracted from `src-tauri/src/lib.rs` and handle
//! configuration file I/O operations.

use super::types::*;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tracing::debug;

// ============================================================================
// Path Functions
// ============================================================================

/// Get the ProxyPal configuration directory
/// **get_config_dir** (Lấy thư mục config - ~/.config/proxypal)
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("proxypal");
    
    // Create if not exists
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
    }
    
    Ok(config_dir)
}

/// Get config file path
/// **get_config_path** (Lấy đường dẫn config - config.json)
pub fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("proxypal");
    std::fs::create_dir_all(&config_dir).ok();
    config_dir.join("config.json")
}

/// Get auth status file path
/// **get_auth_path** (Lấy đường dẫn auth - auth.json)
pub fn get_auth_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("proxypal");
    std::fs::create_dir_all(&config_dir).ok();
    config_dir.join("auth.json")
}

/// Get request history file path
/// **get_history_path** (Lấy đường dẫn history - history.json)
pub fn get_history_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("proxypal");
    std::fs::create_dir_all(&config_dir).ok();
    config_dir.join("history.json")
}

/// Get default configuration file path (for headless mode)
/// **get_default_config_path** (Lấy đường dẫn config mặc định)
pub fn get_default_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.yaml"))
}

// ============================================================================
// Config Load/Save Functions
// ============================================================================

/// Load configuration from JSON file
/// **load_config** (Tải config - đọc từ config.json)
pub fn load_config() -> AppConfig {
    let path = get_config_path();
    if path.exists() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(mut config) = serde_json::from_str::<AppConfig>(&data) {
                // Migration: Convert deprecated amp_openai_provider to amp_openai_providers array
                if let Some(old_provider) = config.amp_openai_provider.take() {
                    if config.amp_openai_providers.is_empty() {
                        let provider_with_id = if old_provider.id.is_empty() {
                            AmpOpenAIProvider {
                                id: generate_uuid(),
                                ..old_provider
                            }
                        } else {
                            old_provider
                        };
                        config.amp_openai_providers.push(provider_with_id);
                        let _ = save_config_to_file(&config);
                    }
                }
                return config;
            }
        }
    }
    AppConfig::default()
}

/// Load configuration from YAML file (for headless mode)
/// **load_config_yaml** (Tải config YAML - đọc từ file .yaml)
pub fn load_config_yaml(path: &str) -> Result<AppConfig> {
    let path = Path::new(path);
    
    debug!("Loading configuration from: {}", path.display());
    
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    
    let config: AppConfig = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    
    debug!("Configuration loaded successfully");
    
    Ok(config)
}

/// Save configuration to JSON file
/// **save_config_to_file** (Lưu config - ghi vào config.json)
pub fn save_config_to_file(config: &AppConfig) -> Result<(), String> {
    let path = get_config_path();
    let data = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(path, data).map_err(|e| e.to_string())
}

/// Save configuration to YAML file
/// **save_config_yaml** (Lưu config YAML - ghi vào file .yaml)
pub fn save_config_yaml(config: &AppConfig, path: &str) -> Result<()> {
    let yaml = serde_yaml::to_string(config)
        .context("Failed to serialize configuration")?;
    
    std::fs::write(path, yaml)
        .with_context(|| format!("Failed to write config file: {}", path))?;
    
    debug!("Configuration saved to: {}", path);
    
    Ok(())
}

// ============================================================================
// Auth Status Load/Save Functions  
// ============================================================================

/// Load auth status from file
/// **load_auth_status** (Tải auth status - đọc từ auth.json)
pub fn load_auth_status() -> AuthStatus {
    let path = get_auth_path();
    if path.exists() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(auth) = serde_json::from_str(&data) {
                return auth;
            }
        }
    }
    AuthStatus::default()
}

/// Save auth status to file
/// **save_auth_to_file** (Lưu auth status - ghi vào auth.json)
pub fn save_auth_to_file(auth: &AuthStatus) -> Result<(), String> {
    let path = get_auth_path();
    let data = serde_json::to_string_pretty(auth).map_err(|e| e.to_string())?;
    std::fs::write(path, data).map_err(|e| e.to_string())
}

// ============================================================================
// Request History Load/Save Functions
// ============================================================================

/// Load request history from file
/// **load_request_history** (Tải lịch sử request - đọc từ history.json)
pub fn load_request_history() -> RequestHistory {
    let path = get_history_path();
    if path.exists() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(history) = serde_json::from_str(&data) {
                return history;
            }
        }
    }
    RequestHistory::default()
}

/// Save request history to file (keep last 500 requests)
/// **save_request_history** (Lưu lịch sử request - giữ 500 requests gần nhất)
pub fn save_request_history(history: &RequestHistory) -> Result<(), String> {
    let path = get_history_path();
    let mut trimmed = history.clone();
    // Keep only last 500 requests
    if trimmed.requests.len() > 500 {
        trimmed.requests = trimmed.requests.split_off(trimmed.requests.len() - 500);
    }
    let data = serde_json::to_string_pretty(&trimmed).map_err(|e| e.to_string())?;
    std::fs::write(path, data).map_err(|e| e.to_string())
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Estimate cost based on model and tokens
/// **estimate_request_cost** (Ước tính chi phí - dựa trên model và token count)
pub fn estimate_request_cost(model: &str, tokens_in: u32, tokens_out: u32) -> f64 {
    // Pricing per 1M tokens (input, output) - approximate as of 2024
    let (input_rate, output_rate) = match model.to_lowercase().as_str() {
        // Claude models
        m if m.contains("claude") && m.contains("opus") => (15.0, 75.0),
        m if m.contains("claude") && m.contains("sonnet") => (3.0, 15.0),
        m if m.contains("claude") && m.contains("haiku") => (0.25, 1.25),
        // GPT models
        m if m.contains("gpt-5") => (15.0, 45.0),
        m if m.contains("gpt-4o") => (2.5, 10.0),
        m if m.contains("gpt-4-turbo") || m.contains("gpt-4") => (10.0, 30.0),
        m if m.contains("gpt-3.5") => (0.5, 1.5),
        // Gemini models
        m if m.contains("gemini") && m.contains("pro") => (1.25, 5.0),
        m if m.contains("gemini") && m.contains("flash") => (0.075, 0.30),
        m if m.contains("gemini-2") => (0.10, 0.40),
        m if m.contains("qwen") => (0.50, 2.0),
        _ => (1.0, 3.0), // Default conservative estimate
    };
    
    let input_cost = (tokens_in as f64 / 1_000_000.0) * input_rate;
    let output_cost = (tokens_out as f64 / 1_000_000.0) * output_rate;
    input_cost + output_cost
}

/// Detect provider from model name
/// **detect_provider_from_model** (Phát hiện provider từ tên model)
pub fn detect_provider_from_model(model: &str) -> String {
    let model_lower = model.to_lowercase();
    
    if model_lower.contains("claude") || model_lower.contains("sonnet") || 
       model_lower.contains("opus") || model_lower.contains("haiku") {
        return "claude".to_string();
    }
    if model_lower.contains("gpt") || model_lower.contains("codex") || 
       model_lower.starts_with("o3") || model_lower.starts_with("o1") {
        return "openai".to_string();
    }
    if model_lower.contains("gemini") {
        return "gemini".to_string();
    }
    if model_lower.contains("qwen") {
        return "qwen".to_string();
    }
    if model_lower.contains("deepseek") {
        return "deepseek".to_string();
    }
    if model_lower.contains("glm") {
        return "zhipu".to_string();
    }
    if model_lower.contains("antigravity") {
        return "antigravity".to_string();
    }
    
    "unknown".to_string()
}

/// Detect provider from API path
/// **detect_provider_from_path** (Phát hiện provider từ API path)
pub fn detect_provider_from_path(path: &str) -> Option<String> {
    // First try Amp-style path
    if path.contains("/api/provider/") {
        let parts: Vec<&str> = path.split('/').collect();
        if let Some(idx) = parts.iter().position(|&p| p == "provider") {
            if let Some(provider) = parts.get(idx + 1) {
                return Some(match *provider {
                    "anthropic" => "claude".to_string(),
                    "openai" => "openai".to_string(),
                    "google" => "gemini".to_string(),
                    p => p.to_string(),
                });
            }
        }
    }
    
    // Fallback: infer from standard endpoint paths
    if path.contains("/v1/messages") || path.contains("/messages") {
        return Some("claude".to_string());
    }
    if path.contains("/v1/chat/completions") || path.contains("/chat/completions") {
        return Some("openai-compat".to_string());
    }
    if path.contains("/v1beta") || path.contains(":generateContent") || path.contains(":streamGenerateContent") {
        return Some("gemini".to_string());
    }
    
    None
}
