//! ProxyPal Headless - CLI Daemon for AI Coding Agents
//! 
//! **ProxyPal Headless** (CLI daemon - chạy proxy server như background service)
//! 
//! A headless (không giao diện) version of ProxyPal that can run as a daemon
//! service on Ubuntu (systemd) and Windows.

mod cli;
mod config;
mod proxy;
mod auth;

use clap::{Parser, Subcommand, ValueHint};
use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter};
use anyhow::Result;

// ============================================================================
// CLI Definition
// ============================================================================

/// ProxyPal - Headless CLI Proxy Server for AI Coding Agents
/// 
/// A unified proxy server that routes AI API requests to multiple providers
/// (Claude, Gemini, OpenAI, etc.) with authentication management, load
/// balancing, and quota handling.
#[derive(Parser)]
#[command(name = "proxypal")]
#[command(author = "ProxyPal Team <team@proxypal.dev>")]
#[command(version)]
#[command(about = "Headless proxy server for AI coding agents")]
#[command(long_about = "ProxyPal is a unified proxy server that routes AI API requests to multiple \
providers (Claude, Gemini, OpenAI, Copilot, etc.) with authentication management, \
load balancing, and quota handling. It enables AI coding tools like Cursor, Continue, \
and Claude Code to work seamlessly with various AI providers.")]
#[command(propagate_version = true)]
#[command(after_help = "\
EXAMPLES:
    # Start the proxy server in foreground
    proxypal serve --foreground

    # Start on a custom port with verbose logging
    proxypal serve --port 9000 -vv

    # Initialize a new configuration file
    proxypal config init

    # Validate configuration before starting
    proxypal config validate

    # Add a Gemini API key
    proxypal auth add --provider gemini --api-key YOUR_API_KEY

    # Add a Vertex AI service account
    proxypal auth add --provider vertex --file service-account.json

    # List all configured authentications
    proxypal auth list

    # Stop a running daemon
    proxypal stop

For more information, visit: https://github.com/wollfoo/cli-all-api
")]
struct Cli {
    /// Increase logging verbosity (-v: info, -vv: debug, -vvv: trace)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Path to configuration file
    #[arg(
        short, 
        long, 
        global = true, 
        default_value = "~/.config/proxypal/config.yaml",
        value_hint = ValueHint::FilePath,
        env = "PROXYPAL_CONFIG"
    )]
    config: String,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

// ============================================================================
// Subcommands
// ============================================================================

/// Available commands
#[derive(Subcommand)]
enum Commands {
    /// Start the proxy server (daemon mode by default)
    #[command(visible_alias = "start")]
    #[command(after_help = "\
EXAMPLES:
    # Start in daemon mode (background)
    proxypal serve

    # Start in foreground with debug logging
    proxypal serve --foreground -vv

    # Start on custom port
    proxypal serve --port 9000

    # Use custom config and PID file
    proxypal serve --config /etc/proxypal/config.yaml --pid-file /var/run/proxypal.pid
")]
    Serve {
        /// Port to listen on (overrides config file)
        #[arg(short, long, value_name = "PORT")]
        port: Option<u16>,

        /// Run in foreground instead of daemonizing
        #[arg(short, long)]
        foreground: bool,

        /// PID file path for daemon mode
        #[arg(
            long, 
            default_value = "/tmp/proxypal.pid",
            value_hint = ValueHint::FilePath,
            value_name = "PATH"
        )]
        pid_file: String,
    },

    /// Configuration file management
    #[command(visible_alias = "cfg")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Authentication and API key management
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },

    /// Stop a running daemon
    #[command(after_help = "\
EXAMPLES:
    # Stop daemon using default PID file
    proxypal stop

    # Stop daemon using custom PID file
    proxypal stop --pid-file /var/run/proxypal.pid
")]
    Stop {
        /// PID file path
        #[arg(
            long, 
            default_value = "/tmp/proxypal.pid",
            value_hint = ValueHint::FilePath,
            value_name = "PATH"
        )]
        pid_file: String,
    },

    /// Show daemon status
    Status {
        /// PID file path
        #[arg(
            long, 
            default_value = "/tmp/proxypal.pid",
            value_hint = ValueHint::FilePath,
            value_name = "PATH"
        )]
        pid_file: String,
    },

    /// Show proxy health and provider status
    #[command(visible_alias = "health")]
    #[command(after_help = "\
EXAMPLES:
    # Basic health check
    proxypal check

    # Health check with custom PID file and port
    proxypal check --pid-file /var/run/proxypal.pid --port 8080

    # JSON output for scripting
    proxypal check --json

    # Check specific provider
    proxypal check --provider gemini
")]
    Check {
        /// PID file path
        #[arg(
            long, 
            default_value = "/tmp/proxypal.pid",
            value_hint = ValueHint::FilePath,
            value_name = "PATH"
        )]
        pid_file: String,

        /// Port to check (default: 8317)
        #[arg(short = 'P', long, default_value = "8317", value_name = "PORT")]
        port: u16,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

        /// Specific provider to check (omit for all)
        #[arg(short, long, value_name = "PROVIDER")]
        provider: Option<String>,
    },
}

/// Config subcommands
#[derive(Subcommand)]
enum ConfigAction {
    /// Validate configuration file syntax and values
    #[command(after_help = "\
EXAMPLES:
    # Validate default config
    proxypal config validate

    # Validate specific config file
    proxypal config validate --config /path/to/config.yaml
")]
    Validate,

    /// Show current configuration (YAML format)
    Show,

    /// Initialize a new configuration file with defaults
    #[command(after_help = "\
EXAMPLES:
    # Create default config at ~/.config/proxypal/config.yaml
    proxypal config init

    # Create config at custom location
    proxypal config init --config /etc/proxypal/config.yaml

    # Overwrite existing config
    proxypal config init --force
")]
    Init {
        /// Overwrite existing configuration file
        #[arg(short, long)]
        force: bool,
    },

    /// Edit configuration file in default editor
    Edit,
}

/// Auth subcommands
#[derive(Subcommand)]
enum AuthAction {
    /// Add authentication credentials for a provider
    #[command(after_help = "\
SUPPORTED PROVIDERS:
    gemini   - Google Gemini API (API key or device code flow)
    copilot  - GitHub Copilot (device code flow)
    claude   - Anthropic Claude API (API key only)
    openai   - OpenAI API (API key only)
    codex    - OpenAI Codex API (API key only)
    vertex   - Google Vertex AI (service account JSON)
    qwen     - Alibaba Qwen API (API key only)

EXAMPLES:
    # Use device code flow for Gemini (recommended)
    proxypal auth add --provider gemini --device-code

    # Use device code flow for Copilot
    proxypal auth add --provider copilot --device-code

    # Add Gemini API key directly
    proxypal auth add --provider gemini --api-key AIzaSy...

    # Add API key from stdin (more secure)
    echo 'YOUR_KEY' | proxypal auth add --provider claude --api-key -

    # Import Vertex AI service account
    proxypal auth add --provider vertex --file service-account.json

    # Interactive mode (select provider and method)
    proxypal auth add
")]
    Add {
        /// Provider name (omit for interactive selection)
        #[arg(short, long, value_name = "PROVIDER")]
        provider: Option<String>,

        /// API key (use '-' to read from stdin)
        #[arg(short = 'k', long, value_name = "KEY")]
        api_key: Option<String>,

        /// Import credential file (JSON format)
        #[arg(short, long, value_hint = ValueHint::FilePath, value_name = "FILE")]
        file: Option<String>,

        /// Use device code flow (for gemini, copilot)
        #[arg(short = 'd', long)]
        device_code: bool,

        /// OAuth client ID (required for device code flow)
        #[arg(long, value_name = "CLIENT_ID", env = "PROXYPAL_OAUTH_CLIENT_ID")]
        client_id: Option<String>,
    },

    /// Remove authentication for a provider
    Remove {
        /// Provider name to remove
        #[arg(short, long, value_name = "PROVIDER")]
        provider: String,
    },

    /// List all configured authentications
    List,

    /// Test authentication for a provider
    Test {
        /// Provider name to test
        #[arg(short, long, value_name = "PROVIDER")]
        provider: String,
    },
}

// ============================================================================
// Logging Setup
// ============================================================================

/// Initialize logging based on verbosity level
fn init_logging(verbosity: u8, quiet: bool) {
    if quiet {
        // Only errors in quiet mode
        let filter = EnvFilter::new("error");
        fmt().with_env_filter(filter).init();
        return;
    }

    let level = match verbosity {
        0 => "info",
        1 => "info,proxypal=debug",
        2 => "debug",
        _ => "trace",
    };

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    fmt()
        .with_env_filter(filter)
        .with_target(verbosity >= 2)
        .with_thread_ids(verbosity >= 3)
        .with_file(verbosity >= 2)
        .with_line_number(verbosity >= 2)
        .init();
}

/// Expand tilde in paths
fn expand_path(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return path.replacen("~", home.to_str().unwrap_or("~"), 1);
        }
    }
    path.to_string()
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    init_logging(cli.verbose, cli.quiet);
    
    let config_path = expand_path(&cli.config);
    
    if !cli.quiet {
        info!("ProxyPal Headless v{}", env!("CARGO_PKG_VERSION"));
    }

    match cli.command {
        Commands::Serve { port, foreground, pid_file } => {
            info!("Starting proxy server...");
            proxy::serve(&config_path, port, foreground, &pid_file).await?;
        }
        
        Commands::Config { action } => {
            match action {
                ConfigAction::Validate => {
                    info!("Validating configuration: {}", config_path);
                    config::validate(&config_path)?;
                }
                ConfigAction::Show => {
                    config::show(&config_path)?;
                }
                ConfigAction::Init { force } => {
                    info!("Initializing configuration file...");
                    config::init(&config_path, force)?;
                }
                ConfigAction::Edit => {
                    // Open in default editor
                    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
                    info!("Opening {} in {}", config_path, editor);
                    std::process::Command::new(&editor)
                        .arg(&config_path)
                        .status()?;
                }
            }
        }
        
        Commands::Auth { action } => {
            match action {
                AuthAction::Add { provider, api_key, file, device_code, client_id } => {
                    match provider {
                        Some(p) => {
                            info!("Adding authentication for provider: {}", p);
                            auth::add(&p, api_key.as_deref(), file.as_deref(), device_code, client_id.as_deref()).await?;
                        }
                        None => {
                            info!("Starting interactive authentication flow...");
                            auth::add_interactive(client_id.as_deref()).await?;
                        }
                    }
                }
                AuthAction::Remove { provider } => {
                    info!("Removing authentication for provider: {}", provider);
                    auth::remove(&provider).await?;
                }
                AuthAction::List => {
                    auth::list().await?;
                }
                AuthAction::Test { provider } => {
                    info!("Testing authentication for provider: {}", provider);
                    // TODO: Implement auth test
                    println!("Auth test not yet implemented for: {}", provider);
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

        Commands::Check { pid_file, port, json, provider } => {
            // Run health check - kiểm tra sức khỏe service
            match provider {
                Some(p) => {
                    // Check specific provider
                    info!("Checking health for provider: {}", p);
                    let check = cli::check_provider_health(&p).await?;
                    if json {
                        println!(r#"{{"provider":"{}","passed":{},"message":"{}"}}"#, 
                            p, check.passed, check.message);
                    } else {
                        let icon = if check.passed { "✓" } else { "✗" };
                        println!("{} Provider '{}': {}", icon, p, check.message);
                    }
                    if !check.passed {
                        std::process::exit(1);
                    }
                }
                None => {
                    // Full health check
                    let exit_code = cli::run_health_check(&pid_file, port, json).await?;
                    if exit_code != 0 {
                        std::process::exit(exit_code);
                    }
                }
            }
        }
    }

    Ok(())
}
