//! ProxyPal Headless - CLI Daemon for AI Coding Agents
//! 
//! **ProxyPal Headless** (CLI daemon - chạy proxy server như background service)
//! 
//! This binary provides a headless (không giao diện) version of ProxyPal
//! that can run as a daemon service on Ubuntu (systemd) and Windows.
//! 
//! ## Usage
//! 
//! ```bash
//! # Start the proxy server
//! proxypal serve --config config.yaml
//! 
//! # Validate configuration
//! proxypal config validate --config config.yaml
//! 
//! # Add authentication
//! proxypal auth add --provider gemini --api-key YOUR_KEY
//! ```

mod cli;
mod config;
mod proxy;
mod auth;

use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use anyhow::Result;

/// ProxyPal Headless - CLI Proxy Server for AI Coding Agents
/// 
/// **ProxyPal** (Proxy server - định tuyến và quản lý yêu cầu API cho AI coding tools)
#[derive(Parser)]
#[command(name = "proxypal")]
#[command(author = "ProxyPal Team")]
#[command(version)]
#[command(about = "Headless proxy server for AI coding agents", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Enable verbose logging
    /// **Verbose** (Chi tiết - hiển thị log debug)
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Path to configuration file
    /// **Config** (Cấu hình - đường dẫn file YAML cấu hình)
    #[arg(short, long, global = true, default_value = "config.yaml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

/// Available commands
/// **Commands** (Lệnh - các subcommand của CLI)
#[derive(Subcommand)]
enum Commands {
    /// Start the proxy server
    /// **Serve** (Khởi động server - chạy proxy daemon)
    Serve {
        /// Port to listen on (overrides config)
        /// **Port** (Cổng - cổng lắng nghe HTTP)
        #[arg(short, long)]
        port: Option<u16>,

        /// Run in foreground (don't daemonize)
        /// **Foreground** (Chế độ foreground - không chạy nền)
        #[arg(short, long)]
        foreground: bool,

        /// PID file path for daemon mode
        /// **PID File** (File PID - lưu process ID)
        #[arg(long, default_value = "/var/run/proxypal.pid")]
        pid_file: String,
    },

    /// Configuration management
    /// **Config** (Cấu hình - quản lý file config)
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Authentication management
    /// **Auth** (Xác thực - quản lý API keys và OAuth)
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },

    /// Stop a running daemon
    /// **Stop** (Dừng - dừng daemon đang chạy)
    Stop {
        /// PID file path
        #[arg(long, default_value = "/var/run/proxypal.pid")]
        pid_file: String,
    },

    /// Show daemon status
    /// **Status** (Trạng thái - hiển thị trạng thái daemon)
    Status {
        /// PID file path
        #[arg(long, default_value = "/var/run/proxypal.pid")]
        pid_file: String,
    },
}

/// Config subcommands
/// **ConfigAction** (Hành động cấu hình - validate/show/init config)
#[derive(Subcommand)]
enum ConfigAction {
    /// Validate configuration file
    Validate,
    /// Show current configuration
    Show,
    /// Initialize default configuration file
    Init {
        /// Overwrite existing config
        #[arg(short, long)]
        force: bool,
    },
}

/// Auth subcommands
/// **AuthAction** (Hành động xác thực - add/remove/list credentials)
#[derive(Subcommand)]
enum AuthAction {
    /// Add authentication credentials
    Add {
        /// Provider name (gemini, claude, openai, etc.)
        #[arg(short, long)]
        provider: String,

        /// API key (reads from stdin if not provided)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// Import credential file (JSON)
        #[arg(short, long)]
        file: Option<String>,
    },
    /// Remove authentication for a provider
    Remove {
        /// Provider name
        #[arg(short, long)]
        provider: String,
    },
    /// List configured authentications
    List,
}

/// Initialize logging
/// **init_logging** (Khởi tạo logging - setup tracing subscriber)
fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    init_logging(cli.verbose);
    
    info!("ProxyPal Headless v{}", env!("CARGO_PKG_VERSION"));

    match cli.command {
        Commands::Serve { port, foreground, pid_file } => {
            info!("Starting proxy server...");
            proxy::serve(&cli.config, port, foreground, &pid_file).await?;
        }
        
        Commands::Config { action } => {
            match action {
                ConfigAction::Validate => {
                    info!("Validating configuration: {}", cli.config);
                    config::validate(&cli.config)?;
                }
                ConfigAction::Show => {
                    config::show(&cli.config)?;
                }
                ConfigAction::Init { force } => {
                    info!("Initializing configuration file...");
                    config::init(&cli.config, force)?;
                }
            }
        }
        
        Commands::Auth { action } => {
            match action {
                AuthAction::Add { provider, api_key, file } => {
                    info!("Adding authentication for provider: {}", provider);
                    auth::add(&provider, api_key.as_deref(), file.as_deref()).await?;
                }
                AuthAction::Remove { provider } => {
                    info!("Removing authentication for provider: {}", provider);
                    auth::remove(&provider).await?;
                }
                AuthAction::List => {
                    auth::list().await?;
                }
            }
        }
        
        Commands::Stop { pid_file } => {
            info!("Stopping daemon...");
            proxy::stop(&pid_file)?;
        }
        
        Commands::Status { pid_file } => {
            proxy::status(&pid_file)?;
        }
    }

    Ok(())
}
