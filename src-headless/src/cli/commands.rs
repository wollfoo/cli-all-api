//! CLI Commands Implementation
//! 
//! **Commands** (Lệnh - triển khai các CLI commands)

// Placeholder for future command implementations
// Các command chính được xử lý trong main.rs
// Module này sẽ chứa helper functions và shared logic

/// Helper function to prompt for confirmation
/// **prompt_confirm** (Xác nhận - hỏi user yes/no)
pub fn prompt_confirm(message: &str) -> bool {
    use std::io::{self, Write};
    
    print!("{} [y/N]: ", message);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Helper function to read secret from stdin
/// **read_secret** (Đọc bí mật - đọc API key từ stdin)
pub fn read_secret(prompt: &str) -> anyhow::Result<String> {
    use std::io::{self, BufRead, Write};
    
    eprint!("{}: ", prompt);
    io::stderr().flush()?;
    
    let stdin = io::stdin();
    let line = stdin.lock().lines().next()
        .ok_or_else(|| anyhow::anyhow!("No input provided"))??;
    
    Ok(line.trim().to_string())
}
