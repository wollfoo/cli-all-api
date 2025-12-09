//! Authentication Management Module
//! 
//! **Auth Module** (Module xác thực - quản lý API keys và credentials)
//! 
//! This module handles:
//! - Adding/removing API keys for providers
//! - Importing auth credential files
//! - Listing configured authentications

mod credentials;

pub use credentials::*;

use anyhow::{Result, Context, bail};
use tracing::{info, warn};

/// Add authentication for a provider
/// **add** (Thêm - thêm xác thực cho provider)
pub async fn add(
    provider: &str,
    api_key: Option<&str>,
    file: Option<&str>,
) -> Result<()> {
    // Validate provider
    // **Validate** (Kiểm tra - xác nhận provider hợp lệ)
    let valid_providers = ["gemini", "claude", "openai", "codex", "qwen", "vertex"];
    if !valid_providers.contains(&provider.to_lowercase().as_str()) {
        bail!(
            "Unknown provider: {}\nValid providers: {}",
            provider,
            valid_providers.join(", ")
        );
    }
    
    if let Some(file_path) = file {
        // Import from file
        // **Import** (Nhập - đọc credentials từ file)
        import_credential_file(provider, file_path)?;
        info!("Imported credential from file: {}", file_path);
    } else if let Some(key) = api_key {
        // Direct API key
        // **Direct Key** (Key trực tiếp - lưu API key)
        save_api_key(provider, key)?;
        info!("API key saved for provider: {}", provider);
    } else {
        // Read from stdin
        // **Stdin** (Đọc stdin - nhập key từ terminal)
        use crate::cli::read_secret;
        let key = read_secret(&format!("Enter API key for {}", provider))?;
        save_api_key(provider, &key)?;
        info!("API key saved for provider: {}", provider);
    }
    
    Ok(())
}

/// Remove authentication for a provider
/// **remove** (Xóa - xóa xác thực cho provider)
pub async fn remove(provider: &str) -> Result<()> {
    let auth_dir = get_auth_dir()?;
    let _pattern = format!("{}-*.json", provider.to_lowercase());
    
    // Find and remove matching files
    // **Remove Files** (Xóa files - xóa credential files)
    let mut removed = 0;
    for entry in std::fs::read_dir(&auth_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        if name.starts_with(&format!("{}-", provider.to_lowercase())) 
            && name.ends_with(".json") 
        {
            std::fs::remove_file(entry.path())?;
            info!("Removed: {}", name);
            removed += 1;
        }
    }
    
    if removed == 0 {
        warn!("No credentials found for provider: {}", provider);
    } else {
        info!("Removed {} credential(s) for {}", removed, provider);
    }
    
    Ok(())
}

/// List configured authentications
/// **list** (Liệt kê - hiển thị các xác thực đã cấu hình)
pub async fn list() -> Result<()> {
    let auth_dir = get_auth_dir()?;
    
    println!("Configured authentications:");
    println!("{}", "-".repeat(50));
    
    let mut count = 0;
    
    for entry in std::fs::read_dir(&auth_dir)
        .context("Failed to read auth directory")?
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        if name.ends_with(".json") {
            // Parse provider from filename
            // **Parse** (Phân tích - lấy provider từ tên file)
            let provider = name.split('-').next().unwrap_or("unknown");
            
            // Get file metadata
            let metadata = entry.metadata()?;
            let modified = metadata.modified()
                .map(|t| {
                    let datetime: chrono::DateTime<chrono::Local> = t.into();
                    datetime.format("%Y-%m-%d %H:%M").to_string()
                })
                .unwrap_or_else(|_| "unknown".to_string());
            
            println!("  {} | {} | {}", provider, name, modified);
            count += 1;
        }
    }
    
    if count == 0 {
        println!("  No credentials configured");
        println!();
        println!("Add credentials with:");
        println!("  proxypal auth add --provider gemini --api-key YOUR_KEY");
        println!("  proxypal auth add --provider vertex --file service-account.json");
    }
    
    println!("{}", "-".repeat(50));
    println!("Total: {} credential(s)", count);
    
    Ok(())
}
