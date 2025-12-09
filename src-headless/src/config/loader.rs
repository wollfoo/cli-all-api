//! Configuration Loader
//! 
//! **Config Loader** (Bộ tải cấu hình - đọc config từ file)

use super::AppConfig;
use anyhow::{Result, Context};
use std::path::Path;
use tracing::debug;

/// Load configuration from a YAML file
/// **load_config** (Tải config - đọc và parse file YAML)
pub fn load_config(path: &str) -> Result<AppConfig> {
    let path = Path::new(path);
    
    debug!("Loading configuration from: {}", path.display());
    
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    
    let config: AppConfig = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    
    debug!("Configuration loaded successfully");
    
    Ok(config)
}

/// Get default configuration directory
/// **get_config_dir** (Lấy thư mục config - đường dẫn mặc định)
pub fn get_config_dir() -> Result<std::path::PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("proxypal");
    
    // Create if not exists
    // **Create** (Tạo - tạo thư mục nếu chưa có)
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
    }
    
    Ok(config_dir)
}

/// Get default configuration file path
/// **get_default_config_path** (Lấy đường dẫn config mặc định)
pub fn get_default_config_path() -> Result<std::path::PathBuf> {
    Ok(get_config_dir()?.join("config.yaml"))
}

/// Save configuration to a YAML file
/// **save_config** (Lưu config - ghi config ra file YAML)
pub fn save_config(config: &AppConfig, path: &str) -> Result<()> {
    let yaml = serde_yaml::to_string(config)
        .context("Failed to serialize configuration")?;
    
    std::fs::write(path, yaml)
        .with_context(|| format!("Failed to write config file: {}", path))?;
    
    debug!("Configuration saved to: {}", path);
    
    Ok(())
}
