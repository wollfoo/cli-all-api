//! Device Code Flow Authentication
//!
//! **Device Code Flow** (Luồng Device Code – OAuth cho CLI headless)
//!
//! Implements RFC 8628 OAuth 2.0 Device Authorization Grant for headless
//! authentication. Used for providers that support device code flow:
//! - Google (Gemini) via oauth2.googleapis.com
//! - GitHub (Copilot) via github.com/login/device
//!
//! API-key only providers (Claude, OpenAI) bypass this module.

use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};
use std::time::Duration;

// ============================================================================
// Constants - OAuth Endpoints
// ============================================================================

/// Google OAuth device code endpoint
/// **GOOGLE_DEVICE_CODE_URL** (URL device code Google – endpoint yêu cầu code)
const GOOGLE_DEVICE_CODE_URL: &str = "https://oauth2.googleapis.com/device/code";

/// Google OAuth token endpoint
/// **GOOGLE_TOKEN_URL** (URL token Google – endpoint polling token)
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

/// GitHub OAuth device code endpoint
/// **GITHUB_DEVICE_CODE_URL** (URL device code GitHub – endpoint yêu cầu code)
const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";

/// GitHub OAuth token endpoint
/// **GITHUB_TOKEN_URL** (URL token GitHub – endpoint polling token)
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

/// Default polling interval in seconds
/// **DEFAULT_POLL_INTERVAL** (Khoảng thời gian poll mặc định – 5 giây)
const DEFAULT_POLL_INTERVAL: u64 = 5;

/// Maximum polling duration in seconds (15 minutes)
/// **MAX_POLL_DURATION** (Thời gian poll tối đa – 15 phút)
const MAX_POLL_DURATION: u64 = 900;

// ============================================================================
// Structs
// ============================================================================

/// Supported OAuth providers for device code flow
/// **DeviceCodeProvider** (Provider OAuth – các provider hỗ trợ device code)
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceCodeProvider {
    /// Google OAuth for Gemini API
    Google,
    /// GitHub OAuth for Copilot
    GitHub,
}

impl DeviceCodeProvider {
    /// Parse provider from string
    /// **from_str** (Phân tích chuỗi – convert string thành enum)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "gemini" | "google" => Some(Self::Google),
            "copilot" | "github" => Some(Self::GitHub),
            _ => None,
        }
    }
    
    /// Check if provider supports device code flow
    /// **supports_device_code** (Kiểm tra hỗ trợ – provider có dùng device code không)
    pub fn supports_device_code(provider: &str) -> bool {
        Self::from_str(provider).is_some()
    }
    
    /// Get device code endpoint URL
    fn device_code_url(&self) -> &'static str {
        match self {
            Self::Google => GOOGLE_DEVICE_CODE_URL,
            Self::GitHub => GITHUB_DEVICE_CODE_URL,
        }
    }
    
    /// Get token endpoint URL
    fn token_url(&self) -> &'static str {
        match self {
            Self::Google => GOOGLE_TOKEN_URL,
            Self::GitHub => GITHUB_TOKEN_URL,
        }
    }
}

/// Request to initiate device code flow
/// **DeviceCodeRequest** (Yêu cầu Device Code – thông tin gửi đến OAuth server)
#[derive(Debug, Clone, Serialize)]
pub struct DeviceCodeRequest {
    /// OAuth client ID
    pub client_id: String,
    /// OAuth scopes (space-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// Response from device code request
/// **DeviceCodeResponse** (Phản hồi Device Code – thông tin từ OAuth server)
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeResponse {
    /// Code for the device to use when polling
    /// **device_code** (Mã thiết bị – dùng để poll token)
    pub device_code: String,
    
    /// Code for user to enter on verification page
    /// **user_code** (Mã người dùng – hiển thị cho user nhập)
    pub user_code: String,
    
    /// URL where user should enter the code
    /// **verification_uri** (URL xác minh – trang user mở để nhập code)
    pub verification_uri: String,
    
    /// Optional pre-filled URL with code
    /// **verification_uri_complete** (URL đầy đủ – URL có sẵn code)
    #[serde(default)]
    pub verification_uri_complete: Option<String>,
    
    /// Lifetime of device_code and user_code in seconds
    /// **expires_in** (Thời gian hết hạn – giây)
    pub expires_in: u64,
    
    /// Minimum polling interval in seconds
    /// **interval** (Khoảng thời gian poll – giây giữa các lần poll)
    #[serde(default = "default_interval")]
    pub interval: u64,
}

fn default_interval() -> u64 {
    DEFAULT_POLL_INTERVAL
}

/// Token response from successful authorization
/// **TokenResponse** (Phản hồi Token – access token và refresh token)
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    /// Access token for API calls
    pub access_token: String,
    
    /// Token type (usually "Bearer")
    #[serde(default)]
    pub token_type: Option<String>,
    
    /// Refresh token for getting new access tokens
    #[serde(default)]
    pub refresh_token: Option<String>,
    
    /// Token lifetime in seconds
    #[serde(default)]
    pub expires_in: Option<u64>,
    
    /// Granted scopes
    #[serde(default)]
    pub scope: Option<String>,
}

/// Error response during polling
/// **TokenError** (Lỗi Token – các lỗi khi poll)
#[derive(Debug, Clone, Deserialize)]
pub struct TokenError {
    /// Error code
    pub error: String,
    
    /// Error description
    #[serde(default)]
    pub error_description: Option<String>,
}

/// Polling result enum
/// **PollResult** (Kết quả Poll – pending, success, hoặc error)
#[derive(Debug)]
pub enum PollResult {
    /// User hasn't authorized yet, continue polling
    Pending,
    /// User authorized, token received
    Success(TokenResponse),
    /// Polling should slow down
    SlowDown,
    /// Error occurred, stop polling
    Error(String),
    /// Code expired
    Expired,
}

// ============================================================================
// Functions
// ============================================================================

/// Request device code from OAuth provider
/// **request_device_code** (Yêu cầu device code – gửi request đến OAuth server)
///
/// # Arguments
/// * `provider` - Provider name (gemini, copilot)
/// * `client_id` - OAuth client ID
/// * `scope` - Optional OAuth scopes
///
/// # Returns
/// DeviceCodeResponse with user_code and verification_uri
pub async fn request_device_code(
    provider: &str,
    client_id: &str,
    scope: Option<&str>,
) -> Result<DeviceCodeResponse> {
    let provider_enum = DeviceCodeProvider::from_str(provider)
        .ok_or_else(|| anyhow::anyhow!(
            "Provider '{}' does not support device code flow. Use API key instead.",
            provider
        ))?;
    
    let client = reqwest::Client::new();
    let url = provider_enum.device_code_url();
    
    info!("Requesting device code from {} for provider: {}", url, provider);
    
    // Build request based on provider
    // **Build Request** (Xây dựng request – tùy theo provider)
    let response = match provider_enum {
        DeviceCodeProvider::Google => {
            client.post(url)
                .form(&[
                    ("client_id", client_id),
                    ("scope", scope.unwrap_or("https://www.googleapis.com/auth/generative-language.retriever")),
                ])
                .send()
                .await
                .context("Failed to send device code request to Google")?
        }
        DeviceCodeProvider::GitHub => {
            client.post(url)
                .header("Accept", "application/json")
                .form(&[
                    ("client_id", client_id),
                    ("scope", scope.unwrap_or("read:user")),
                ])
                .send()
                .await
                .context("Failed to send device code request to GitHub")?
        }
    };
    
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("Device code request failed ({}): {}", status, body);
    }
    
    let device_response: DeviceCodeResponse = response.json().await
        .context("Failed to parse device code response")?;
    
    debug!("Received device code response: user_code={}, expires_in={}s", 
        device_response.user_code, device_response.expires_in);
    
    Ok(device_response)
}

/// Poll for access token after user authorization
/// **poll_for_token** (Poll token – kiểm tra liên tục cho đến khi user authorize)
///
/// # Arguments
/// * `provider` - Provider name
/// * `client_id` - OAuth client ID
/// * `device_code` - Device code from initial request
/// * `interval` - Polling interval in seconds
///
/// # Returns
/// TokenResponse with access_token on success
pub async fn poll_for_token(
    provider: &str,
    client_id: &str,
    device_code: &str,
    interval: u64,
) -> Result<TokenResponse> {
    let provider_enum = DeviceCodeProvider::from_str(provider)
        .ok_or_else(|| anyhow::anyhow!("Invalid provider: {}", provider))?;
    
    let client = reqwest::Client::new();
    let url = provider_enum.token_url();
    let poll_interval = Duration::from_secs(interval.max(DEFAULT_POLL_INTERVAL));
    let start_time = std::time::Instant::now();
    let max_duration = Duration::from_secs(MAX_POLL_DURATION);
    
    info!("Starting token polling for {} (interval: {}s)", provider, interval);
    
    loop {
        // Check timeout
        // **Timeout Check** (Kiểm tra timeout – dừng nếu quá 15 phút)
        if start_time.elapsed() > max_duration {
            bail!("Device code expired. Please try again.");
        }
        
        // Wait before polling
        // **Wait** (Chờ – đợi interval giữa các lần poll)
        tokio::time::sleep(poll_interval).await;
        
        debug!("Polling for token...");
        
        // Send poll request
        // **Poll Request** (Gửi request poll – kiểm tra authorization)
        let response = match provider_enum {
            DeviceCodeProvider::Google => {
                client.post(url)
                    .form(&[
                        ("client_id", client_id),
                        ("device_code", device_code),
                        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                    ])
                    .send()
                    .await
            }
            DeviceCodeProvider::GitHub => {
                client.post(url)
                    .header("Accept", "application/json")
                    .form(&[
                        ("client_id", client_id),
                        ("device_code", device_code),
                        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                    ])
                    .send()
                    .await
            }
        };
        
        let response = match response {
            Ok(r) => r,
            Err(e) => {
                warn!("Poll request failed: {}. Retrying...", e);
                continue;
            }
        };
        
        let body = response.text().await.unwrap_or_default();
        
        // Parse response
        // **Parse Response** (Phân tích response – kiểm tra success hoặc error)
        let poll_result = parse_poll_response(&body);
        
        match poll_result {
            PollResult::Success(token) => {
                info!("Authorization successful! Token received.");
                return Ok(token);
            }
            PollResult::Pending => {
                debug!("Authorization pending, waiting for user...");
                continue;
            }
            PollResult::SlowDown => {
                debug!("Rate limited, slowing down...");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
            PollResult::Expired => {
                bail!("Device code expired. Please try again.");
            }
            PollResult::Error(msg) => {
                bail!("Authorization failed: {}", msg);
            }
        }
    }
}

/// Parse poll response body
/// **parse_poll_response** (Phân tích poll response – xác định trạng thái)
fn parse_poll_response(body: &str) -> PollResult {
    // Try to parse as token response first
    if let Ok(token) = serde_json::from_str::<TokenResponse>(body) {
        if !token.access_token.is_empty() {
            return PollResult::Success(token);
        }
    }
    
    // Try to parse as error response
    if let Ok(error) = serde_json::from_str::<TokenError>(body) {
        return match error.error.as_str() {
            "authorization_pending" => PollResult::Pending,
            "slow_down" => PollResult::SlowDown,
            "expired_token" | "expired" => PollResult::Expired,
            "access_denied" => PollResult::Error("User denied access".to_string()),
            _ => PollResult::Error(
                error.error_description.unwrap_or(error.error)
            ),
        };
    }
    
    // Unknown response
    PollResult::Error(format!("Unknown response: {}", body))
}

/// Display device code instructions to user
/// **display_device_code_instructions** (Hiển thị hướng dẫn – in ra terminal)
pub fn display_device_code_instructions(response: &DeviceCodeResponse) {
    println!();
    println!("════════════════════════════════════════════════════════════");
    println!("                    DEVICE AUTHORIZATION");
    println!("════════════════════════════════════════════════════════════");
    println!();
    println!("  1. Open this URL in your browser:");
    println!();
    if let Some(ref complete_uri) = response.verification_uri_complete {
        println!("     {}", complete_uri);
    } else {
        println!("     {}", response.verification_uri);
    }
    println!();
    println!("  2. Enter this code:");
    println!();
    println!("     ┌─────────────────┐");
    println!("     │   {}   │", response.user_code);
    println!("     └─────────────────┘");
    println!();
    println!("  Code expires in {} seconds", response.expires_in);
    println!();
    println!("════════════════════════════════════════════════════════════");
    println!();
    println!("Waiting for authorization...");
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_provider_from_str() {
        assert_eq!(DeviceCodeProvider::from_str("gemini"), Some(DeviceCodeProvider::Google));
        assert_eq!(DeviceCodeProvider::from_str("google"), Some(DeviceCodeProvider::Google));
        assert_eq!(DeviceCodeProvider::from_str("copilot"), Some(DeviceCodeProvider::GitHub));
        assert_eq!(DeviceCodeProvider::from_str("github"), Some(DeviceCodeProvider::GitHub));
        assert_eq!(DeviceCodeProvider::from_str("claude"), None);
        assert_eq!(DeviceCodeProvider::from_str("openai"), None);
    }
    
    #[test]
    fn test_supports_device_code() {
        assert!(DeviceCodeProvider::supports_device_code("gemini"));
        assert!(DeviceCodeProvider::supports_device_code("copilot"));
        assert!(!DeviceCodeProvider::supports_device_code("claude"));
        assert!(!DeviceCodeProvider::supports_device_code("openai"));
    }
    
    #[test]
    fn test_parse_poll_response_pending() {
        let body = r#"{"error": "authorization_pending"}"#;
        assert!(matches!(parse_poll_response(body), PollResult::Pending));
    }
    
    #[test]
    fn test_parse_poll_response_success() {
        let body = r#"{"access_token": "test_token", "token_type": "Bearer"}"#;
        let result = parse_poll_response(body);
        assert!(matches!(result, PollResult::Success(_)));
        if let PollResult::Success(token) = result {
            assert_eq!(token.access_token, "test_token");
        }
    }
    
    #[test]
    fn test_parse_poll_response_expired() {
        let body = r#"{"error": "expired_token"}"#;
        assert!(matches!(parse_poll_response(body), PollResult::Expired));
    }
}
