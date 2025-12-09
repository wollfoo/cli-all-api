//! Auth File Import Module
//!
//! **Import Module** (Module import - nhập credentials từ files)
//!
//! Supports multiple file formats:
//! - JSON: Standard API key or OAuth credential
//! - YAML: Configuration file format
//! - Env file: Environment variables (KEY=value)
//! - Vertex JSON: Google service account format

use anyhow::{Result, Context, bail};
use std::path::{Path, PathBuf};
use tracing::{info, warn, debug};

use super::credentials::{get_auth_dir, save_api_key};
use super::interactive::validate_api_key_format;

// ============================================================================
// File Format Detection
// ============================================================================

/// Detected file format
/// **FileFormat** (Định dạng file - loại credential file)
#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    /// JSON format (API key, OAuth token, or generic credential)
    Json,
    /// YAML format (configuration file)
    Yaml,
    /// Environment file (KEY=value pairs)
    EnvFile,
    /// Google Vertex AI service account JSON
    VertexServiceAccount,
    /// Unknown format
    Unknown,
}

/// Parsed credential from file
/// **ParsedCredential** (Credential đã parse - thông tin từ file)
#[derive(Debug, Clone)]
pub struct ParsedCredential {
    pub provider: String,
    pub api_key: Option<String>,
    pub file_format: FileFormat,
    pub raw_content: String,
    pub source_path: PathBuf,
}

/// Import result for a single file
/// **ImportResult** (Kết quả import - success hoặc failure)
#[derive(Debug)]
pub struct ImportResult {
    pub path: PathBuf,
    pub success: bool,
    pub message: String,
    pub provider: Option<String>,
}

// ============================================================================
// Format Detection Functions
// ============================================================================

/// Detect file format from content and extension
/// **detect_format** (Phát hiện format - từ content và extension)
pub fn detect_format(path: &Path, content: &str) -> FileFormat {
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());
    
    // Check by extension first
    match extension.as_deref() {
        Some("yaml") | Some("yml") => return FileFormat::Yaml,
        Some("env") => return FileFormat::EnvFile,
        _ => {}
    }
    
    // Check content for JSON
    if content.trim().starts_with('{') {
        // Check if it's a Vertex service account
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            if json.get("type").and_then(|v| v.as_str()) == Some("service_account") 
                && json.get("project_id").is_some() 
            {
                return FileFormat::VertexServiceAccount;
            }
        }
        return FileFormat::Json;
    }
    
    // Check for YAML-like content
    if content.contains(": ") && !content.contains("=") {
        return FileFormat::Yaml;
    }
    
    // Check for env file format (KEY=value)
    if content.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.is_empty() && !trimmed.starts_with('#') && trimmed.contains('=')
    }) {
        return FileFormat::EnvFile;
    }
    
    FileFormat::Unknown
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse credential file and extract API key
/// **parse_credential_file** (Parse file credential - trích xuất API key)
pub fn parse_credential_file(path: &Path) -> Result<ParsedCredential> {
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }
    
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read file: {}", path.display()))?;
    
    let format = detect_format(path, &content);
    
    match format {
        FileFormat::Json => parse_json_credential(path, &content),
        FileFormat::Yaml => parse_yaml_credential(path, &content),
        FileFormat::EnvFile => parse_env_credential(path, &content),
        FileFormat::VertexServiceAccount => parse_vertex_credential(path, &content),
        FileFormat::Unknown => bail!("Unknown file format: {}", path.display()),
    }
}

/// Parse JSON credential file
/// **parse_json_credential** (Parse JSON - trích xuất từ JSON)
fn parse_json_credential(path: &Path, content: &str) -> Result<ParsedCredential> {
    let json: serde_json::Value = serde_json::from_str(content)
        .context("Invalid JSON format")?;
    
    // Try to extract provider and API key
    let provider = json.get("provider")
        .and_then(|v| v.as_str())
        .map(|s| s.to_lowercase());
    
    let api_key = json.get("api_key")
        .or_else(|| json.get("apiKey"))
        .or_else(|| json.get("key"))
        .or_else(|| json.get("access_token"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // Try to detect provider from key format if not specified
    let detected_provider = if provider.is_none() {
        api_key.as_ref().and_then(|k| detect_provider_from_key(k))
    } else {
        provider
    };
    
    Ok(ParsedCredential {
        provider: detected_provider.unwrap_or_else(|| "unknown".to_string()),
        api_key,
        file_format: FileFormat::Json,
        raw_content: content.to_string(),
        source_path: path.to_path_buf(),
    })
}

/// Parse YAML credential file
/// **parse_yaml_credential** (Parse YAML - trích xuất từ YAML)
fn parse_yaml_credential(path: &Path, content: &str) -> Result<ParsedCredential> {
    let yaml: serde_yaml::Value = serde_yaml::from_str(content)
        .context("Invalid YAML format")?;
    
    // Try to extract provider and API key from various structures
    let provider = yaml.get("provider")
        .and_then(|v| v.as_str())
        .map(|s| s.to_lowercase());
    
    let api_key = yaml.get("api_key")
        .or_else(|| yaml.get("apiKey"))
        .or_else(|| yaml.get("key"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let detected_provider = if provider.is_none() {
        api_key.as_ref().and_then(|k| detect_provider_from_key(k))
    } else {
        provider
    };
    
    Ok(ParsedCredential {
        provider: detected_provider.unwrap_or_else(|| "unknown".to_string()),
        api_key,
        file_format: FileFormat::Yaml,
        raw_content: content.to_string(),
        source_path: path.to_path_buf(),
    })
}

/// Parse env file credential
/// **parse_env_credential** (Parse env file - trích xuất từ KEY=value)
fn parse_env_credential(path: &Path, content: &str) -> Result<ParsedCredential> {
    let mut provider = None;
    let mut api_key = None;
    
    // Known environment variable patterns
    let patterns = [
        ("ANTHROPIC_API_KEY", "claude"),
        ("CLAUDE_API_KEY", "claude"),
        ("OPENAI_API_KEY", "openai"),
        ("GEMINI_API_KEY", "gemini"),
        ("GOOGLE_API_KEY", "gemini"),
        ("COPILOT_TOKEN", "copilot"),
        ("GITHUB_TOKEN", "copilot"),
        ("QWEN_API_KEY", "qwen"),
        ("DASHSCOPE_API_KEY", "qwen"),
    ];
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            
            for (pattern, prov) in &patterns {
                if key == *pattern {
                    provider = Some(prov.to_string());
                    api_key = Some(value.to_string());
                    break;
                }
            }
            
            if provider.is_some() {
                break;
            }
        }
    }
    
    Ok(ParsedCredential {
        provider: provider.unwrap_or_else(|| "unknown".to_string()),
        api_key,
        file_format: FileFormat::EnvFile,
        raw_content: content.to_string(),
        source_path: path.to_path_buf(),
    })
}

/// Parse Vertex AI service account JSON
/// **parse_vertex_credential** (Parse Vertex - service account JSON)
fn parse_vertex_credential(path: &Path, content: &str) -> Result<ParsedCredential> {
    let json: serde_json::Value = serde_json::from_str(content)
        .context("Invalid JSON format for Vertex service account")?;
    
    // Validate service account structure
    if json.get("type").and_then(|v| v.as_str()) != Some("service_account") {
        bail!("Not a valid service account JSON");
    }
    
    if json.get("project_id").is_none() {
        bail!("Service account must contain 'project_id' field");
    }
    
    Ok(ParsedCredential {
        provider: "vertex".to_string(),
        api_key: None, // Vertex uses service account, not API key
        file_format: FileFormat::VertexServiceAccount,
        raw_content: content.to_string(),
        source_path: path.to_path_buf(),
    })
}

/// Detect provider from API key format
/// **detect_provider_from_key** (Phát hiện provider - từ format key)
fn detect_provider_from_key(key: &str) -> Option<String> {
    if key.starts_with("sk-ant-") {
        Some("claude".to_string())
    } else if key.starts_with("sk-") {
        Some("openai".to_string())
    } else if key.starts_with("AIza") {
        Some("gemini".to_string())
    } else if key.starts_with("ghp_") || key.starts_with("gho_") {
        Some("copilot".to_string())
    } else {
        None
    }
}

// ============================================================================
// Import Functions
// ============================================================================

/// Import a single credential file
/// **import_file** (Import file - nhập một file credential)
pub fn import_file(path: &Path, provider_override: Option<&str>) -> Result<ImportResult> {
    debug!("Importing credential file: {}", path.display());
    
    let parsed = match parse_credential_file(path) {
        Ok(p) => p,
        Err(e) => {
            return Ok(ImportResult {
                path: path.to_path_buf(),
                success: false,
                message: e.to_string(),
                provider: None,
            });
        }
    };
    
    let provider = provider_override
        .map(|s| s.to_string())
        .unwrap_or(parsed.provider.clone());
    
    if provider == "unknown" {
        return Ok(ImportResult {
            path: path.to_path_buf(),
            success: false,
            message: "Could not detect provider. Use --provider flag.".to_string(),
            provider: None,
        });
    }
    
    // Handle based on format
    let result = match parsed.file_format {
        FileFormat::VertexServiceAccount => {
            // Copy service account file directly
            import_vertex_file(path, &provider)
        }
        _ => {
            // For other formats, extract API key and save
            if let Some(ref api_key) = parsed.api_key {
                // Validate API key format
                if let Err(e) = validate_api_key_format(&provider, api_key) {
                    warn!("API key validation warning for {}: {}", provider, e);
                }
                save_api_key(&provider, api_key)
            } else {
                bail!("No API key found in file")
            }
        }
    };
    
    match result {
        Ok(_) => {
            info!("Imported credential for provider: {}", provider);
            Ok(ImportResult {
                path: path.to_path_buf(),
                success: true,
                message: format!("Imported {} credential", provider),
                provider: Some(provider),
            })
        }
        Err(e) => Ok(ImportResult {
            path: path.to_path_buf(),
            success: false,
            message: e.to_string(),
            provider: Some(provider),
        }),
    }
}

/// Import Vertex AI service account file
/// **import_vertex_file** (Import Vertex - copy service account file)
fn import_vertex_file(path: &Path, _provider: &str) -> Result<()> {
    let auth_dir = get_auth_dir()?;
    
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let filename = format!("vertex-serviceaccount-{}.json", timestamp);
    let dest_path = auth_dir.join(&filename);
    
    std::fs::copy(path, &dest_path)
        .context("Failed to copy service account file")?;
    
    debug!("Imported Vertex service account to: {}", dest_path.display());
    Ok(())
}

/// Import all credential files from a directory
/// **import_directory** (Import directory - nhập tất cả files từ thư mục)
pub fn import_directory(dir_path: &Path, provider_override: Option<&str>) -> Result<Vec<ImportResult>> {
    if !dir_path.is_dir() {
        bail!("Not a directory: {}", dir_path.display());
    }
    
    let mut results = Vec::new();
    
    for entry in std::fs::read_dir(dir_path)
        .context(format!("Failed to read directory: {}", dir_path.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        
        // Skip directories and hidden files
        if path.is_dir() {
            continue;
        }
        
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') {
                continue;
            }
        }
        
        // Check extension - only process known formats
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "json" | "yaml" | "yml" | "env" => {
                    let result = import_file(&path, provider_override)?;
                    results.push(result);
                }
                _ => {
                    // Skip unknown extensions
                    debug!("Skipping file with unknown extension: {}", path.display());
                }
            }
        }
    }
    
    Ok(results)
}

/// Display import results summary
/// **display_import_summary** (Hiển thị tóm tắt - kết quả import)
pub fn display_import_summary(results: &[ImportResult]) {
    let success_count = results.iter().filter(|r| r.success).count();
    let failed_count = results.len() - success_count;
    
    println!();
    println!("════════════════════════════════════════════════════════════");
    println!("                    IMPORT SUMMARY");
    println!("════════════════════════════════════════════════════════════");
    println!();
    
    if success_count > 0 {
        println!("  ✓ Successfully imported: {} credential(s)", success_count);
        for result in results.iter().filter(|r| r.success) {
            if let Some(ref provider) = result.provider {
                println!("    • {} ({})", provider, result.path.file_name().unwrap_or_default().to_string_lossy());
            }
        }
    }
    
    if failed_count > 0 {
        println!();
        println!("  ✗ Failed to import: {} file(s)", failed_count);
        for result in results.iter().filter(|r| !r.success) {
            println!("    • {}: {}", 
                result.path.file_name().unwrap_or_default().to_string_lossy(),
                result.message
            );
        }
    }
    
    println!();
    println!("════════════════════════════════════════════════════════════");
    println!();
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_format_json() {
        let content = r#"{"api_key": "test"}"#;
        let path = Path::new("test.json");
        assert_eq!(detect_format(path, content), FileFormat::Json);
    }
    
    #[test]
    fn test_detect_format_yaml() {
        let content = "api_key: test\nprovider: claude";
        let path = Path::new("test.yaml");
        assert_eq!(detect_format(path, content), FileFormat::Yaml);
    }
    
    #[test]
    fn test_detect_format_env() {
        let content = "ANTHROPIC_API_KEY=sk-ant-test";
        let path = Path::new("test.env");
        assert_eq!(detect_format(path, content), FileFormat::EnvFile);
    }
    
    #[test]
    fn test_detect_format_vertex() {
        let content = r#"{"type": "service_account", "project_id": "test-project"}"#;
        let path = Path::new("service-account.json");
        assert_eq!(detect_format(path, content), FileFormat::VertexServiceAccount);
    }
    
    #[test]
    fn test_detect_provider_from_key() {
        assert_eq!(detect_provider_from_key("sk-ant-test"), Some("claude".to_string()));
        assert_eq!(detect_provider_from_key("sk-proj-test"), Some("openai".to_string()));
        assert_eq!(detect_provider_from_key("AIzaTest123"), Some("gemini".to_string()));
        assert_eq!(detect_provider_from_key("ghp_test"), Some("copilot".to_string()));
        assert_eq!(detect_provider_from_key("unknown"), None);
    }
}
