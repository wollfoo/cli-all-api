//! Interactive Authentication Module
//!
//! **Interactive Module** (Module tương tác - giao diện chọn provider và auth method)
//!
//! Provides interactive prompts for:
//! - Provider selection menu
//! - Auth method selection (device code vs API key)
//! - API key input with validation

use anyhow::{Result, bail};
use dialoguer::{Select, Input, Confirm, theme::ColorfulTheme};
use tracing::{info, debug};

// ============================================================================
// Provider Information
// ============================================================================

/// Provider with description and auth methods
/// **ProviderInfo** (Thông tin Provider - tên, mô tả, và các auth methods hỗ trợ)
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub supports_device_code: bool,
    pub supports_api_key: bool,
    pub supports_file: bool,
}

/// Available providers with their capabilities
/// **PROVIDERS** (Danh sách providers - tất cả providers hỗ trợ)
pub const PROVIDERS: &[ProviderInfo] = &[
    ProviderInfo {
        id: "gemini",
        name: "Google Gemini",
        description: "Google AI Studio / Vertex AI",
        supports_device_code: true,
        supports_api_key: true,
        supports_file: false,
    },
    ProviderInfo {
        id: "claude",
        name: "Anthropic Claude",
        description: "Claude API (console.anthropic.com)",
        supports_device_code: false,
        supports_api_key: true,
        supports_file: false,
    },
    ProviderInfo {
        id: "openai",
        name: "OpenAI",
        description: "GPT-4, GPT-3.5 (platform.openai.com)",
        supports_device_code: false,
        supports_api_key: true,
        supports_file: false,
    },
    ProviderInfo {
        id: "copilot",
        name: "GitHub Copilot",
        description: "GitHub Copilot (requires OAuth)",
        supports_device_code: true,
        supports_api_key: false,
        supports_file: false,
    },
    ProviderInfo {
        id: "codex",
        name: "OpenAI Codex",
        description: "Codex API (platform.openai.com)",
        supports_device_code: false,
        supports_api_key: true,
        supports_file: false,
    },
    ProviderInfo {
        id: "qwen",
        name: "Alibaba Qwen",
        description: "Qwen API (dashscope.aliyun.com)",
        supports_device_code: false,
        supports_api_key: true,
        supports_file: false,
    },
    ProviderInfo {
        id: "vertex",
        name: "Google Vertex AI",
        description: "Vertex AI (service account JSON)",
        supports_device_code: false,
        supports_api_key: false,
        supports_file: true,
    },
];

// ============================================================================
// Interactive Functions
// ============================================================================

/// Get provider info by ID
/// **get_provider_info** (Lấy thông tin provider - từ ID)
pub fn get_provider_info(provider_id: &str) -> Option<&'static ProviderInfo> {
    PROVIDERS.iter().find(|p| p.id == provider_id.to_lowercase())
}

/// Interactive provider selection menu
/// **select_provider** (Chọn provider - hiển thị menu interactive)
pub fn select_provider() -> Result<&'static ProviderInfo> {
    let theme = ColorfulTheme::default();
    
    // Build selection items
    // **Build Items** (Xây dựng items - format provider list)
    let items: Vec<String> = PROVIDERS
        .iter()
        .map(|p| {
            let auth_methods = format_auth_methods(p);
            format!("{} - {} [{}]", p.name, p.description, auth_methods)
        })
        .collect();
    
    println!();
    println!("Select AI Provider:");
    println!();
    
    let selection = Select::with_theme(&theme)
        .items(&items)
        .default(0)
        .interact()?;
    
    debug!("User selected provider: {}", PROVIDERS[selection].id);
    
    Ok(&PROVIDERS[selection])
}

/// Format auth methods string for display
/// **format_auth_methods** (Format auth methods - hiển thị các methods hỗ trợ)
fn format_auth_methods(provider: &ProviderInfo) -> String {
    let mut methods = Vec::new();
    
    if provider.supports_device_code {
        methods.push("OAuth");
    }
    if provider.supports_api_key {
        methods.push("API Key");
    }
    if provider.supports_file {
        methods.push("File Import");
    }
    
    methods.join(", ")
}

/// Auth method selection
/// **AuthMethod** (Phương thức Auth - device code, API key, hoặc file)
#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    DeviceCode,
    ApiKey,
    FileImport,
}

/// Interactive auth method selection
/// **select_auth_method** (Chọn auth method - hiển thị menu)
pub fn select_auth_method(provider: &ProviderInfo) -> Result<AuthMethod> {
    let theme = ColorfulTheme::default();
    
    // Build available methods
    // **Build Methods** (Xây dựng methods - chỉ hiển thị methods hỗ trợ)
    let mut items = Vec::new();
    let mut methods = Vec::new();
    
    if provider.supports_device_code {
        items.push("🔐 OAuth Device Code (recommended - login via browser)");
        methods.push(AuthMethod::DeviceCode);
    }
    if provider.supports_api_key {
        items.push("🔑 API Key (paste key from provider console)");
        methods.push(AuthMethod::ApiKey);
    }
    if provider.supports_file {
        items.push("📁 File Import (service account JSON)");
        methods.push(AuthMethod::FileImport);
    }
    
    if items.is_empty() {
        bail!("Provider {} has no supported auth methods", provider.id);
    }
    
    // If only one method, use it directly
    // **Single Method** (Một method - tự động chọn)
    if items.len() == 1 {
        info!("Only one auth method available, using: {:?}", methods[0]);
        return Ok(methods[0].clone());
    }
    
    println!();
    println!("Select authentication method for {}:", provider.name);
    println!();
    
    let selection = Select::with_theme(&theme)
        .items(&items)
        .default(0)
        .interact()?;
    
    debug!("User selected auth method: {:?}", methods[selection]);
    
    Ok(methods[selection].clone())
}

/// Interactive API key input with validation
/// **prompt_api_key** (Nhập API key - với validation)
pub fn prompt_api_key(provider: &ProviderInfo) -> Result<String> {
    let theme = ColorfulTheme::default();
    
    println!();
    println!("Enter API key for {}:", provider.name);
    println!("(Get your key from: {})", get_provider_console_url(provider.id));
    println!();
    
    let api_key: String = Input::with_theme(&theme)
        .with_prompt("API Key")
        .validate_with(|input: &String| -> Result<(), String> {
            validate_api_key_format(provider.id, input)
        })
        .interact_text()?;
    
    Ok(api_key)
}

/// Interactive file path input
/// **prompt_file_path** (Nhập đường dẫn file - với validation)
pub fn prompt_file_path(provider: &ProviderInfo) -> Result<String> {
    let theme = ColorfulTheme::default();
    
    println!();
    println!("Enter path to {} credential file:", provider.name);
    println!();
    
    let file_path: String = Input::with_theme(&theme)
        .with_prompt("File path")
        .validate_with(|input: &String| -> Result<(), String> {
            if std::path::Path::new(input).exists() {
                Ok(())
            } else {
                Err(format!("File not found: {}", input))
            }
        })
        .interact_text()?;
    
    Ok(file_path)
}

/// Confirm action
/// **confirm_action** (Xác nhận hành động - yes/no prompt)
pub fn confirm_action(message: &str) -> Result<bool> {
    let theme = ColorfulTheme::default();
    
    let confirmed = Confirm::with_theme(&theme)
        .with_prompt(message)
        .default(true)
        .interact()?;
    
    Ok(confirmed)
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Get provider console URL
/// **get_provider_console_url** (Lấy URL console - để user lấy API key)
fn get_provider_console_url(provider_id: &str) -> &'static str {
    match provider_id {
        "gemini" => "https://aistudio.google.com/apikey",
        "claude" => "https://console.anthropic.com/settings/keys",
        "openai" | "codex" => "https://platform.openai.com/api-keys",
        "qwen" => "https://dashscope.console.aliyun.com/apiKey",
        "copilot" => "https://github.com/settings/tokens",
        "vertex" => "https://console.cloud.google.com/iam-admin/serviceaccounts",
        _ => "the provider's console",
    }
}

/// Validate API key format based on provider
/// **validate_api_key_format** (Kiểm tra format API key - theo provider)
pub fn validate_api_key_format(provider_id: &str, api_key: &str) -> Result<(), String> {
    let trimmed = api_key.trim();
    
    if trimmed.is_empty() {
        return Err("API key cannot be empty".to_string());
    }
    
    match provider_id {
        "gemini" => {
            // Google API keys typically start with AIza
            if !trimmed.starts_with("AIza") && trimmed.len() < 30 {
                return Err("Gemini API keys typically start with 'AIza' and are 39+ characters".to_string());
            }
        }
        "claude" => {
            // Anthropic keys start with sk-ant-
            if !trimmed.starts_with("sk-ant-") {
                return Err("Claude API keys should start with 'sk-ant-'".to_string());
            }
        }
        "openai" | "codex" => {
            // OpenAI keys start with sk-
            if !trimmed.starts_with("sk-") {
                return Err("OpenAI API keys should start with 'sk-'".to_string());
            }
        }
        "qwen" => {
            // Qwen keys are typically 32+ characters
            if trimmed.len() < 20 {
                return Err("Qwen API keys are typically 32+ characters".to_string());
            }
        }
        _ => {
            // Generic validation - just check length
            if trimmed.len() < 10 {
                return Err("API key seems too short".to_string());
            }
        }
    }
    
    Ok(())
}

/// Display success message
/// **display_success** (Hiển thị thành công - thông báo sau khi auth thành công)
pub fn display_success(provider: &ProviderInfo, auth_method: &AuthMethod) {
    println!();
    println!("════════════════════════════════════════════════════════════");
    println!("  ✓ Successfully added {} authentication!", provider.name);
    println!();
    match auth_method {
        AuthMethod::DeviceCode => println!("  Method: OAuth Device Code"),
        AuthMethod::ApiKey => println!("  Method: API Key"),
        AuthMethod::FileImport => println!("  Method: File Import"),
    }
    println!();
    println!("  Your credentials are stored securely.");
    println!("  Run 'proxypal auth list' to view all credentials.");
    println!("════════════════════════════════════════════════════════════");
    println!();
}

/// Display failure message
/// **display_failure** (Hiển thị thất bại - thông báo lỗi với suggestions)
pub fn display_failure(provider: &ProviderInfo, error: &str) {
    eprintln!();
    eprintln!("════════════════════════════════════════════════════════════");
    eprintln!("  ✗ Failed to add {} authentication", provider.name);
    eprintln!();
    eprintln!("  Error: {}", error);
    eprintln!();
    eprintln!("  Suggestions:");
    eprintln!("  • Check that your API key is correct and active");
    eprintln!("  • Verify you have the necessary permissions");
    eprintln!("  • Get a new key from: {}", get_provider_console_url(provider.id));
    eprintln!("════════════════════════════════════════════════════════════");
    eprintln!();
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_gemini_key() {
        assert!(validate_api_key_format("gemini", "AIzaSyBcdefghijklmnopqrstuvwxyz123456").is_ok());
        assert!(validate_api_key_format("gemini", "short").is_err());
    }
    
    #[test]
    fn test_validate_claude_key() {
        assert!(validate_api_key_format("claude", "sk-ant-api03-abc123").is_ok());
        assert!(validate_api_key_format("claude", "sk-abc123").is_err());
    }
    
    #[test]
    fn test_validate_openai_key() {
        assert!(validate_api_key_format("openai", "sk-proj-abc123456789").is_ok());
        assert!(validate_api_key_format("openai", "api-key-123").is_err());
    }
    
    #[test]
    fn test_get_provider_info() {
        assert!(get_provider_info("gemini").is_some());
        assert!(get_provider_info("claude").is_some());
        assert!(get_provider_info("unknown").is_none());
    }
    
    #[test]
    fn test_providers_have_at_least_one_method() {
        for provider in PROVIDERS {
            assert!(
                provider.supports_device_code || provider.supports_api_key || provider.supports_file,
                "Provider {} has no auth methods",
                provider.id
            );
        }
    }
}
