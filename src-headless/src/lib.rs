//! Library Entry Point - Public API
//! 
//! **ProxyPal Core** (Core library - exports cho cả Tauri và headless binary)
//! 
//! This library provides the core functionality shared between
//! the Tauri GUI app and the headless CLI binary.
//! 
//! ## Modules
//! - `config`: Configuration types and I/O operations
//! - `proxy`: Proxy management and config generation
//! - `auth`: Authentication handling
//! - `cli`: CLI command definitions

pub mod cli;
pub mod config;
pub mod proxy;
pub mod auth;

// Re-export commonly used types at crate root
pub use config::types::*;
pub use config::loader::{
    get_config_dir,
    get_config_path,
    get_auth_path,
    get_history_path,
    load_config,
    load_config_yaml,
    save_config_to_file,
    save_config_yaml,
    load_auth_status,
    save_auth_to_file,
    load_request_history,
    save_request_history,
    estimate_request_cost,
    detect_provider_from_model,
    detect_provider_from_path,
};

pub use proxy::process::{
    ProcessHandle,
    ProcessManager,
    StdProcessManager,
    start_cliproxyapi,
    find_cliproxyapi_binary,
    is_process_running,
    kill_by_pid,
};

pub use proxy::config_gen::generate_proxy_config;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
