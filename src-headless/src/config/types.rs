//! Core Types - Shared Data Structures
//! 
//! **Core Types** (Các kiểu dữ liệu core - structs dùng chung cho GUI và headless)
//! 
//! These types are extracted from `src-tauri/src/lib.rs` and designed to be
//! reusable across both the Tauri GUI app and the headless CLI binary.
//! 
//! ## Categories:
//! - **Status types**: ProxyStatus, CopilotStatus, AuthStatus
//! - **Config types**: AppConfig, CopilotConfig, AmpModelMapping
//! - **API types**: GeminiApiKey, ClaudeApiKey, CodexApiKey
//! - **Usage types**: UsageStats, RequestLog, ModelUsage

use serde::{Deserialize, Serialize};

// ============================================================================
// Status Types
// ============================================================================

/// Proxy server status
/// **ProxyStatus** (Trạng thái Proxy - running, port, endpoint)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyStatus {
    pub running: bool,
    pub port: u16,
    pub endpoint: String,
}

impl Default for ProxyStatus {
    fn default() -> Self {
        Self {
            running: false,
            port: 8317,
            endpoint: "http://localhost:8317/v1".to_string(),
        }
    }
}

/// Authentication status for different providers (count of connected accounts)
/// **AuthStatus** (Trạng thái xác thực - số tài khoản đã kết nối cho mỗi provider)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    pub claude: u32,
    pub openai: u32,
    pub gemini: u32,
    pub qwen: u32,
    pub iflow: u32,
    pub vertex: u32,
    pub antigravity: u32,
}

/// Copilot proxy status
/// **CopilotStatus** (Trạng thái Copilot - running, authenticated)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopilotStatus {
    pub running: bool,
    pub port: u16,
    pub endpoint: String,
    pub authenticated: bool,
}

impl Default for CopilotStatus {
    fn default() -> Self {
        Self {
            running: false,
            port: 4141,
            endpoint: "http://localhost:4141".to_string(),
            authenticated: false,
        }
    }
}

/// Provider health status
/// **ProviderHealth** (Sức khỏe Provider - trạng thái của các AI providers)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderHealth {
    pub claude: HealthStatus,
    pub openai: HealthStatus,
    pub gemini: HealthStatus,
    pub qwen: HealthStatus,
    pub iflow: HealthStatus,
    pub vertex: HealthStatus,
    pub antigravity: HealthStatus,
}

/// Individual health status entry
/// **HealthStatus** (Trạng thái sức khỏe - healthy/degraded/offline/unconfigured)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthStatus {
    /// Status: "healthy", "degraded", "offline", "unconfigured"
    pub status: String,
    pub latency_ms: Option<u64>,
    pub last_checked: u64,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            status: "unconfigured".to_string(),
            latency_ms: None,
            last_checked: 0,
        }
    }
}

// ============================================================================
// Configuration Types
// ============================================================================

/// Main application configuration
/// **AppConfig** (Cấu hình ứng dụng - struct chính chứa tất cả settings)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Port for the proxy server (default: 8317)
    pub port: u16,
    /// Auto-start proxy on app launch
    #[serde(default)]
    pub auto_start: bool,
    /// Launch app at system login
    #[serde(default)]
    pub launch_at_login: bool,
    /// Enable debug logging
    #[serde(default)]
    pub debug: bool,
    /// HTTP proxy URL for outbound requests
    #[serde(default)]
    pub proxy_url: String,
    /// Request retry count on failure
    #[serde(default)]
    pub request_retry: u16,
    /// Switch project on quota exceeded
    #[serde(default)]
    pub quota_switch_project: bool,
    /// Switch to preview model on quota exceeded
    #[serde(default)]
    pub quota_switch_preview_model: bool,
    /// Enable usage statistics
    #[serde(default = "default_true")]
    pub usage_stats_enabled: bool,
    /// Enable request logging to file
    #[serde(default)]
    pub request_logging: bool,
    /// Enable logging to file
    #[serde(default)]
    pub logging_to_file: bool,
    /// Config file version for migrations
    #[serde(default = "default_config_version")]
    pub config_version: u8,
    /// Amp CLI API key
    #[serde(default)]
    pub amp_api_key: String,
    /// Amp CLI model mappings
    #[serde(default)]
    pub amp_model_mappings: Vec<AmpModelMapping>,
    /// DEPRECATED: Use amp_openai_providers
    #[serde(default)]
    pub amp_openai_provider: Option<AmpOpenAIProvider>,
    /// Multiple custom OpenAI-compatible providers
    #[serde(default)]
    pub amp_openai_providers: Vec<AmpOpenAIProvider>,
    /// Routing mode: "mappings" or "openai"
    #[serde(default = "default_routing_mode")]
    pub amp_routing_mode: String,
    /// Copilot integration settings
    #[serde(default)]
    pub copilot: CopilotConfig,
    /// Claude API keys
    #[serde(default)]
    pub claude_api_keys: Vec<ClaudeApiKey>,
    /// Gemini API keys  
    #[serde(default)]
    pub gemini_api_keys: Vec<GeminiApiKey>,
    /// Codex (OpenAI) API keys
    #[serde(default)]
    pub codex_api_keys: Vec<CodexApiKey>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 8317,
            auto_start: true,
            launch_at_login: false,
            debug: false,
            proxy_url: String::new(),
            request_retry: 0,
            quota_switch_project: false,
            quota_switch_preview_model: false,
            usage_stats_enabled: true,
            request_logging: false,
            logging_to_file: false,
            config_version: 1,
            amp_api_key: String::new(),
            amp_model_mappings: Vec::new(),
            amp_openai_provider: None,
            amp_openai_providers: Vec::new(),
            amp_routing_mode: "mappings".to_string(),
            copilot: CopilotConfig::default(),
            claude_api_keys: Vec::new(),
            gemini_api_keys: Vec::new(),
            codex_api_keys: Vec::new(),
        }
    }
}

/// GitHub Copilot proxy configuration
/// **CopilotConfig** (Config Copilot - cài đặt GitHub Copilot integration)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopilotConfig {
    pub enabled: bool,
    #[serde(default = "default_copilot_port")]
    pub port: u16,
    /// Account type: "individual", "business", "enterprise"
    #[serde(default = "default_individual")]
    pub account_type: String,
    /// Optional pre-authenticated GitHub token
    #[serde(default)]
    pub github_token: String,
    /// Seconds between requests for rate limiting
    #[serde(default)]
    pub rate_limit: Option<u16>,
    /// Wait instead of error on rate limit
    #[serde(default)]
    pub rate_limit_wait: bool,
}

impl Default for CopilotConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            port: 4141,
            account_type: "individual".to_string(),
            github_token: String::new(),
            rate_limit: None,
            rate_limit_wait: false,
        }
    }
}

/// Amp model mapping for routing requests to different models
/// **AmpModelMapping** (Ánh xạ model Amp - mapping model name -> target)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmpModelMapping {
    pub from: String,
    pub to: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// OpenAI-compatible provider model for Amp routing
/// **AmpOpenAIModel** (Model OpenAI Amp - config cho custom model)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmpOpenAIModel {
    pub name: String,
    #[serde(default)]
    pub alias: String,
}

/// OpenAI-compatible provider configuration
/// **AmpOpenAIProvider** (Provider OpenAI Amp - custom endpoint config)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmpOpenAIProvider {
    #[serde(default = "generate_uuid")]
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    #[serde(default)]
    pub models: Vec<AmpOpenAIModel>,
}

// ============================================================================
// API Key Types
// ============================================================================

/// Gemini API key entry
/// **GeminiApiKey** (Entry Gemini API key - key với optional config)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiApiKey {
    pub api_key: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub excluded_models: Option<Vec<String>>,
}

/// Claude API key entry
/// **ClaudeApiKey** (Entry Claude API key)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaudeApiKey {
    pub api_key: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub models: Option<Vec<ModelMapping>>,
    #[serde(default)]
    pub excluded_models: Option<Vec<String>>,
}

/// Codex (OpenAI) API key entry
/// **CodexApiKey** (Entry Codex API key)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexApiKey {
    pub api_key: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub proxy_url: Option<String>,
    #[serde(default)]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

/// Model mapping with alias
/// **ModelMapping** (Ánh xạ model - name và alias)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelMapping {
    pub name: String,
    #[serde(default)]
    pub alias: Option<String>,
}

/// OpenAI-compatible provider (for Management API)
/// **OpenAICompatibleProvider** (Provider tương thích OpenAI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAICompatibleProvider {
    pub name: String,
    pub base_url: String,
    pub api_key_entries: Vec<OpenAICompatibleApiKeyEntry>,
    #[serde(default)]
    pub models: Option<Vec<ModelMapping>>,
    #[serde(default)]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

/// OpenAI-compatible API key entry
/// **OpenAICompatibleApiKeyEntry** (Entry API key tương thích OpenAI)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAICompatibleApiKeyEntry {
    pub api_key: String,
    #[serde(default)]
    pub proxy_url: Option<String>,
}

// ============================================================================
// Usage & Logging Types
// ============================================================================

/// Usage statistics from Management API
/// **UsageStats** (Thống kê sử dụng - requests, tokens, costs)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageStats {
    pub total_requests: u64,
    pub success_count: u64,
    pub failure_count: u64,
    pub total_tokens: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub requests_today: u64,
    pub tokens_today: u64,
    #[serde(default)]
    pub models: Vec<ModelUsage>,
    #[serde(default)]
    pub requests_by_day: Vec<TimeSeriesPoint>,
    #[serde(default)]
    pub tokens_by_day: Vec<TimeSeriesPoint>,
    #[serde(default)]
    pub requests_by_hour: Vec<TimeSeriesPoint>,
    #[serde(default)]
    pub tokens_by_hour: Vec<TimeSeriesPoint>,
}

/// Time series data point for charts
/// **TimeSeriesPoint** (Điểm time series - label và value)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesPoint {
    pub label: String,
    pub value: u64,
}

/// Model usage statistics
/// **ModelUsage** (Thống kê sử dụng model - requests và tokens)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelUsage {
    pub model: String,
    pub requests: u64,
    pub tokens: u64,
}

/// Request log entry for live monitoring
/// **RequestLog** (Log request - entry cho monitoring)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestLog {
    pub id: String,
    pub timestamp: u64,
    pub provider: String,
    pub model: String,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub duration_ms: u64,
    pub tokens_in: Option<u32>,
    pub tokens_out: Option<u32>,
}

/// Request history with metadata
/// **RequestHistory** (Lịch sử request - danh sách requests và totals)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestHistory {
    pub requests: Vec<RequestLog>,
    pub total_tokens_in: u64,
    pub total_tokens_out: u64,
    pub total_cost_usd: f64,
}

// ============================================================================
// Detection Types
// ============================================================================

/// Detected AI coding tool
/// **DetectedTool** (Công cụ AI đã phát hiện - installed, configurable)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedTool {
    pub id: String,
    pub name: String,
    pub installed: bool,
    pub config_path: Option<String>,
    pub can_auto_configure: bool,
}

/// CLI Agent configuration status
/// **AgentStatus** (Trạng thái CLI Agent - installed, configured)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentStatus {
    pub id: String,
    pub name: String,
    pub description: String,
    pub installed: bool,
    pub configured: bool,
    /// Config type: "env", "file", "both", "config"
    pub config_type: String,
    pub config_path: Option<String>,
    pub logo: String,
    pub docs_url: String,
}

/// Available model from CLIProxyAPI
/// **AvailableModel** (Model có sẵn - id và owned_by)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableModel {
    pub id: String,
    pub owned_by: String,
}

/// Agent test result
/// **AgentTestResult** (Kết quả test agent - success, latency)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTestResult {
    pub success: bool,
    pub message: String,
    pub latency_ms: Option<u64>,
}

/// Provider test result
/// **ProviderTestResult** (Kết quả test provider - success, models found)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderTestResult {
    pub success: bool,
    pub message: String,
    pub latency_ms: Option<u64>,
    pub models_found: Option<u32>,
}

/// Copilot API detection result
/// **CopilotApiDetection** (Kết quả phát hiện Copilot API)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopilotApiDetection {
    pub found: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub node_found: bool,
    pub node_version: Option<String>,
    pub npm_found: bool,
    pub npm_path: Option<String>,
}

/// Auth file entry from Management API
/// **AuthFile** (Entry auth file - metadata về credential file)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthFile {
    pub id: String,
    pub name: String,
    pub provider: String,
    #[serde(default)]
    pub label: Option<String>,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default)]
    pub runtime_only: bool,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub modtime: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub account_type: Option<String>,
    #[serde(default)]
    pub account: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub last_refresh: Option<String>,
    #[serde(default)]
    pub success_count: Option<u64>,
    #[serde(default)]
    pub failure_count: Option<u64>,
}

/// Log entry structure
/// **LogEntry** (Entry log - timestamp, level, message)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn default_true() -> bool {
    true
}

fn default_config_version() -> u8 {
    1
}

fn default_copilot_port() -> u16 {
    4141
}

fn default_individual() -> String {
    "individual".to_string()
}

fn default_routing_mode() -> String {
    "mappings".to_string()
}

/// Generate a new UUID v4
/// **generate_uuid** (Tạo UUID - ID duy nhất)
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
