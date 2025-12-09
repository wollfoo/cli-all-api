//! Authentication Management Module
//! 
//! **Auth Module** (Module xác thực - quản lý API keys và credentials)
//! 
//! This module handles:
//! - Adding/removing API keys for providers
//! - Importing auth credential files
//! - Listing configured authentications

mod credentials;
mod device_code;
mod interactive;

pub use credentials::*;
pub use device_code::*;
pub use interactive::*;
use anyhow::{Result, Context, bail};
use tracing::{info, warn};

/// Add authentication for a provider (with arguments)
/// **add** (Thêm - thêm xác thực cho provider với CLI arguments)
pub async fn add(
    provider: &str,
    api_key: Option<&str>,
    file: Option<&str>,
    use_device_code: bool,
    client_id: Option<&str>,
) -> Result<()> {
    // Get provider info for validation and display
    // **Get Provider Info** (Lấy thông tin provider - cho validation)
    let provider_info = get_provider_info(provider)
        .ok_or_else(|| anyhow::anyhow!(
            "Unknown provider: {}\nValid providers: gemini, copilot, claude, openai, codex, qwen, vertex",
            provider
        ))?;
    
    // Determine auth method based on flags
    // **Determine Method** (Xác định method - dựa trên flags)
    let auth_method = if use_device_code {
        AuthMethod::DeviceCode
    } else if file.is_some() {
        AuthMethod::FileImport
    } else {
        AuthMethod::ApiKey
    };
    
    // Execute based on method
    // **Execute** (Thực thi - theo auth method)
    let result = match auth_method {
        AuthMethod::DeviceCode => {
            add_with_device_code(provider, client_id).await
        }
        AuthMethod::FileImport => {
            let file_path = file.unwrap();
            import_credential_file(provider, file_path)?;
            println!();
            println!("✓ Imported credential from file: {}", file_path);
            info!("Imported credential from file: {}", file_path);
            Ok(())
        }
        AuthMethod::ApiKey => {
            if let Some(key) = api_key {
                // Validate format
                if let Err(e) = validate_api_key_format(provider, key) {
                    warn!("API key validation warning: {}", e);
                }
                save_api_key(provider, key)?;
                println!();
                println!("✓ API key saved for provider: {}", provider);
                info!("API key saved for provider: {}", provider);
                Ok(())
            } else {
                // Prompt for key interactively
                let key = prompt_api_key(provider_info)?;
                save_api_key(provider, &key)?;
                println!();
                println!("✓ API key saved for provider: {}", provider);
                info!("API key saved for provider: {}", provider);
                Ok(())
            }
        }
    };
    
    // Display result
    // **Display Result** (Hiển thị kết quả - success hoặc failure)
    match &result {
        Ok(_) => display_success(provider_info, &auth_method),
        Err(e) => display_failure(provider_info, &e.to_string()),
    }
    
    result
}

/// Fully interactive auth flow (no arguments provided)
/// **add_interactive** (Thêm interactive - khi không có arguments)
pub async fn add_interactive(client_id: Option<&str>) -> Result<()> {
    // Step 1: Select provider
    // **Select Provider** (Chọn provider - hiển thị menu)
    let provider_info = select_provider()?;
    info!("Selected provider: {}", provider_info.id);
    
    // Step 2: Select auth method
    // **Select Method** (Chọn method - hiển thị options)
    let auth_method = select_auth_method(provider_info)?;
    info!("Selected auth method: {:?}", auth_method);
    
    // Step 3: Execute auth flow
    // **Execute Flow** (Thực thi flow - theo method được chọn)
    let result = match auth_method {
        AuthMethod::DeviceCode => {
            add_with_device_code(provider_info.id, client_id).await
        }
        AuthMethod::ApiKey => {
            let key = prompt_api_key(provider_info)?;
            save_api_key(provider_info.id, &key)?;
            info!("API key saved for provider: {}", provider_info.id);
            Ok(())
        }
        AuthMethod::FileImport => {
            let file_path = prompt_file_path(provider_info)?;
            import_credential_file(provider_info.id, &file_path)?;
            info!("Imported credential from file: {}", file_path);
            Ok(())
        }
    };
    
    // Display result
    match &result {
        Ok(_) => display_success(provider_info, &auth_method),
        Err(e) => display_failure(provider_info, &e.to_string()),
    }
    
    result
}

/// Add authentication using device code flow
/// **add_with_device_code** (Thêm với device code - OAuth cho CLI headless)
async fn add_with_device_code(provider: &str, client_id: Option<&str>) -> Result<()> {
    // Check if provider supports device code
    // **Check Support** (Kiểm tra hỗ trợ - provider có dùng device code không)
    if !DeviceCodeProvider::supports_device_code(provider) {
        bail!(
            "Provider '{}' does not support device code flow.\n\
            Use --api-key instead for this provider.",
            provider
        );
    }
    
    // Get client ID
    // **Client ID** (ID ứng dụng OAuth - bắt buộc cho device code)
    let oauth_client_id: String = match client_id {
        Some(id) => id.to_string(),
        None => std::env::var("PROXYPAL_OAUTH_CLIENT_ID")
            .map_err(|_| anyhow::anyhow!(
                "OAuth client ID required for device code flow.\n\
                Provide --client-id or set PROXYPAL_OAUTH_CLIENT_ID environment variable."
            ))?,
    };
    
    info!("Starting device code flow for provider: {}", provider);
    
    // Step 1: Request device code
    // **Request Code** (Yêu cầu code - gọi OAuth server)
    let device_response = request_device_code(provider, &oauth_client_id, None).await?;
    
    // Step 2: Display instructions to user
    // **Display Instructions** (Hiển thị hướng dẫn - in ra terminal)
    display_device_code_instructions(&device_response);
    
    // Step 3: Poll for token
    // **Poll Token** (Chờ token - poll cho đến khi user authorize)
    let token_response = poll_for_token(
        provider,
        &oauth_client_id,
        &device_response.device_code,
        device_response.interval,
    ).await?;
    
    // Step 4: Save token as credential
    // **Save Token** (Lưu token - ghi vào thư mục auth)
    save_oauth_token(provider, &token_response)?;
    
    println!();
    println!("✓ Authentication successful for {}!", provider);
    info!("OAuth token saved for provider: {}", provider);
    
    Ok(())
}

/// Save OAuth token as credential file
/// **save_oauth_token** (Lưu OAuth token - ghi token vào file JSON)
fn save_oauth_token(provider: &str, token: &TokenResponse) -> Result<()> {
    let auth_dir = get_auth_dir()?;
    
    // Generate unique filename
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let filename = format!("{}-oauth-{}.json", provider.to_lowercase(), timestamp);
    let file_path = auth_dir.join(&filename);
    
    // Create credential JSON with OAuth token
    // **OAuth Credential** (Credential OAuth - format khác với API key)
    let credential = serde_json::json!({
        "type": "oauth",
        "provider": provider.to_lowercase(),
        "access_token": token.access_token,
        "refresh_token": token.refresh_token,
        "token_type": token.token_type.clone().unwrap_or_else(|| "Bearer".to_string()),
        "expires_in": token.expires_in,
        "scope": token.scope,
        "created_at": chrono::Utc::now().to_rfc3339(),
    });
    
    let content = serde_json::to_string_pretty(&credential)
        .context("Failed to serialize OAuth credential")?;
    
    std::fs::write(&file_path, content)
        .context("Failed to write OAuth credential file")?;
    
    tracing::debug!("Saved OAuth token to: {}", file_path.display());
    
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
