//! Credential Storage
//! 
//! **Credentials Module** (Module credentials - lưu trữ và quản lý credentials)

use anyhow::{Result, Context, bail};
use std::path::PathBuf;
use tracing::debug;

/// Get the authentication directory
/// **get_auth_dir** (Lấy thư mục auth - ~/.cli-proxy-api)
pub fn get_auth_dir() -> Result<PathBuf> {
    let auth_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".cli-proxy-api");
    
    // Create if not exists
    // **Create** (Tạo - tạo thư mục nếu chưa có)
    if !auth_dir.exists() {
        std::fs::create_dir_all(&auth_dir)
            .context("Failed to create auth directory")?;
    }
    
    Ok(auth_dir)
}

/// Save an API key for a provider
/// **save_api_key** (Lưu API key - ghi key vào file)
pub fn save_api_key(provider: &str, api_key: &str) -> Result<()> {
    let auth_dir = get_auth_dir()?;
    
    // Generate unique filename
    // **Filename** (Tên file - tạo tên file unique)
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let filename = format!("{}-{}.json", provider.to_lowercase(), timestamp);
    let file_path = auth_dir.join(&filename);
    
    // Create credential JSON
    // **JSON** (JSON - định dạng credential)
    let credential = serde_json::json!({
        "api_key": api_key,
        "provider": provider.to_lowercase(),
        "created_at": chrono::Utc::now().to_rfc3339(),
    });
    
    let content = serde_json::to_string_pretty(&credential)
        .context("Failed to serialize credential")?;
    
    std::fs::write(&file_path, content)
        .context("Failed to write credential file")?;
    
    debug!("Saved API key to: {}", file_path.display());
    
    Ok(())
}

/// Import a credential file (JSON)
/// **import_credential_file** (Nhập file credential - copy và validate)
pub fn import_credential_file(provider: &str, file_path: &str) -> Result<()> {
    let source_path = std::path::Path::new(file_path);
    
    if !source_path.exists() {
        bail!("Credential file not found: {}", file_path);
    }
    
    // Read and validate JSON
    // **Validate** (Kiểm tra - đọc và xác nhận JSON hợp lệ)
    let content = std::fs::read_to_string(source_path)
        .context("Failed to read credential file")?;
    
    let json: serde_json::Value = serde_json::from_str(&content)
        .context("Invalid JSON in credential file")?;
    
    // Validate based on provider
    // **Provider Validation** (Kiểm tra provider - xác nhận format đúng)
    match provider.to_lowercase().as_str() {
        "vertex" => {
            // Vertex requires service account JSON
            if json.get("type").and_then(|v| v.as_str()) != Some("service_account") {
                bail!("Vertex credential must be a service account JSON with 'type': 'service_account'");
            }
            if json.get("project_id").is_none() {
                bail!("Vertex credential must contain 'project_id' field");
            }
        }
        _ => {
            // Other providers just need valid JSON
            // **Other** (Khác - chỉ cần JSON hợp lệ)
        }
    }
    
    // Copy to auth directory
    // **Copy** (Sao chép - copy file vào thư mục auth)
    let auth_dir = get_auth_dir()?;
    
    let source_filename = source_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("credential.json");
    
    // Ensure filename starts with provider prefix
    let dest_filename = if source_filename.starts_with(&format!("{}-", provider.to_lowercase())) {
        source_filename.to_string()
    } else {
        format!("{}-{}", provider.to_lowercase(), source_filename)
    };
    
    let dest_path = auth_dir.join(dest_filename);
    
    std::fs::copy(source_path, &dest_path)
        .context("Failed to copy credential file")?;
    
    debug!("Imported credential to: {}", dest_path.display());
    
    Ok(())
}

/// List all credential files for a provider
/// **list_credentials** (Liệt kê credentials - lấy danh sách files)
pub fn list_credentials(provider: &str) -> Result<Vec<PathBuf>> {
    let auth_dir = get_auth_dir()?;
    let prefix = format!("{}-", provider.to_lowercase());
    
    let mut credentials = Vec::new();
    
    for entry in std::fs::read_dir(&auth_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        if name.starts_with(&prefix) && name.ends_with(".json") {
            credentials.push(entry.path());
        }
    }
    
    Ok(credentials)
}
