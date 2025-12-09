//! Configuration Types
//! 
//! **Config Types** (Các kiểu cấu hình - định nghĩa struct cho config)
//! 
//! These types are designed to be compatible with the Tauri GUI app
//! and can be shared via a future proxypal-core crate.

use serde::{Deserialize, Serialize};

/// Main application configuration
/// **AppConfig** (Cấu hình ứng dụng - struct chính chứa tất cả settings)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    /// Port for the proxy server (default: 8317)
    /// **port** (Cổng - cổng HTTP proxy lắng nghe)
    #[serde(default = "default_port")]
    pub port: u16,
    
    /// Enable debug logging
    /// **debug** (Gỡ lỗi - bật log chi tiết)
    #[serde(default)]
    pub debug: bool,
    
    /// Enable usage statistics
    /// **usage_stats_enabled** (Thống kê sử dụng - theo dõi request counts)
    #[serde(default = "default_true")]
    pub usage_stats_enabled: bool,
    
    /// Enable logging to file
    /// **logging_to_file** (Ghi log file - lưu log vào file)
    #[serde(default = "default_true")]
    pub logging_to_file: bool,
    
    /// Enable request retry on failure
    /// **request_retry** (Thử lại request - tự động retry khi lỗi)
    #[serde(default = "default_true")]
    pub request_retry: bool,
    
    /// HTTP proxy URL for outbound requests
    /// **proxy_url** (Proxy URL - URL proxy cho requests đi ra)
    #[serde(default)]
    pub proxy_url: Option<String>,
    
    /// Switch project on quota exceeded
    /// **quota_switch_project** (Chuyển project - khi hết quota)
    #[serde(default = "default_true")]
    pub quota_switch_project: bool,
    
    /// Switch to preview model on quota exceeded
    /// **quota_switch_preview_model** (Chuyển model preview - khi hết quota)
    #[serde(default)]
    pub quota_switch_preview_model: bool,
    
    /// Amp CLI API key
    /// **amp_api_key** (Amp API Key - key cho Amp CLI integration)
    #[serde(default)]
    pub amp_api_key: Option<String>,
    
    /// Amp CLI model mappings
    /// **amp_model_mappings** (Amp Model Mappings - ánh xạ model cho Amp)
    #[serde(default)]
    pub amp_model_mappings: Vec<AmpModelMapping>,
    
    /// Amp OpenAI-compatible providers
    /// **amp_openai_providers** (Amp OpenAI Providers - custom OpenAI endpoints)
    #[serde(default)]
    pub amp_openai_providers: Vec<AmpOpenAIProvider>,
    
    /// Gemini API keys
    /// **gemini_api_keys** (Gemini API Keys - keys cho Google Gemini)
    #[serde(default)]
    pub gemini_api_keys: Vec<ApiKeyEntry>,
    
    /// Claude API keys
    /// **claude_api_keys** (Claude API Keys - keys cho Anthropic Claude)
    #[serde(default)]
    pub claude_api_keys: Vec<ApiKeyEntry>,
    
    /// Codex API keys (OpenAI)
    /// **codex_api_keys** (Codex API Keys - keys cho OpenAI Codex)
    #[serde(default)]
    pub codex_api_keys: Vec<ApiKeyEntry>,
    
    /// Copilot integration settings
    /// **copilot** (Copilot - cài đặt GitHub Copilot integration)
    #[serde(default)]
    pub copilot: Option<CopilotConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            debug: false,
            usage_stats_enabled: true,
            logging_to_file: true,
            request_retry: true,
            proxy_url: None,
            quota_switch_project: true,
            quota_switch_preview_model: false,
            amp_api_key: None,
            amp_model_mappings: Vec::new(),
            amp_openai_providers: Vec::new(),
            gemini_api_keys: Vec::new(),
            claude_api_keys: Vec::new(),
            codex_api_keys: Vec::new(),
            copilot: None,
        }
    }
}

/// Amp CLI model mapping
/// **AmpModelMapping** (Ánh xạ model Amp - mapping model name -> target)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AmpModelMapping {
    /// Original model name from Amp CLI
    pub from: String,
    /// Target model to route to
    pub to: String,
}

/// Amp OpenAI-compatible provider
/// **AmpOpenAIProvider** (Provider OpenAI Amp - custom OpenAI endpoint)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AmpOpenAIProvider {
    /// Provider name
    pub name: String,
    /// Base URL for the provider
    pub base_url: String,
    /// API key
    pub api_key: String,
    /// Model configurations
    #[serde(default)]
    pub models: Vec<AmpOpenAIModel>,
}

/// Amp OpenAI model configuration
/// **AmpOpenAIModel** (Config model OpenAI Amp)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AmpOpenAIModel {
    /// Model name
    pub name: String,
    /// Optional alias
    #[serde(default)]
    pub alias: Option<String>,
}

/// API key entry with optional configuration
/// **ApiKeyEntry** (Entry API key - key với optional config)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApiKeyEntry {
    /// The API key
    pub api_key: String,
    /// Optional base URL override
    #[serde(default)]
    pub base_url: Option<String>,
    /// Optional proxy URL for this key
    #[serde(default)]
    pub proxy_url: Option<String>,
}

/// Copilot integration configuration
/// **CopilotConfig** (Config Copilot - cài đặt GitHub Copilot)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CopilotConfig {
    /// Enable Copilot integration
    pub enabled: bool,
    /// Copilot API port
    #[serde(default = "default_copilot_port")]
    pub port: u16,
}

// Default value functions
// **Default functions** (Hàm mặc định - trả về giá trị default)

fn default_port() -> u16 {
    8317
}

fn default_copilot_port() -> u16 {
    8318
}

fn default_true() -> bool {
    true
}
