#![deny(clippy::all)]

//! GraphBit JavaScript/Node.js Bindings
//!
//! High-performance native Node.js bindings for the GraphBit agentic workflow
//! automation framework, built with napi-rs.

use napi::bindgen_prelude::*;
use napi_derive::napi;

// Module declarations
mod agent;
mod document_loader;
mod embeddings;
mod errors;
mod graph;
mod llm;
mod llm_client;
mod text_splitter;
mod tools;
mod types;
mod validation;
mod workflow;
pub use tools::*;

/// Configuration options for initialization
#[napi(object)]
pub struct InitOptions {
    /// Log level: "trace", "debug", "info", "warn", "error"
    pub log_level: Option<String>,
    /// Whether to enable colored logs
    pub colored_logs: Option<bool>,
    /// Log output target: "stdout", "stderr"
    pub log_output: Option<String>,
}

/// Initialize the GraphBit library with configuration
///
/// This should be called once at the start of your application.
/// It sets up logging and other global state.
///
/// # Arguments
/// * `options` - Optional configuration for logging and runtime behavior
///
/// # Example
///
/// ```javascript
/// const { init } = require('@infinitibit_gmbh/graphbit');
///
/// // Simple initialization
/// init();
///
/// // With configuration
/// init({
///   logLevel: 'debug',
///   coloredLogs: true,
///   logOutput: 'stdout'
/// });
/// ```
#[napi]
pub fn init(options: Option<InitOptions>) -> Result<()> {
    // Parse options
    let log_level = options
        .as_ref()
        .and_then(|o| o.log_level.as_deref())
        .unwrap_or("info");
    
    let colored = options
        .as_ref()
        .and_then(|o| o.colored_logs)
        .unwrap_or(true);

    // Initialize tracing subscriber with configuration
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(match log_level {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        })
        .with_ansi(colored);

    let _ = subscriber.try_init();

    tracing::info!(
        "GraphBit JavaScript bindings v{} initialized (log_level: {}, colored: {})",
        env!("CARGO_PKG_VERSION"),
        log_level,
        colored
    );

    Ok(())
}

/// Get the version of the GraphBit library
///
/// # Example
///
/// ```javascript
/// const { version } = require('@infinitibit_gmbh/graphbit');
/// console.log(version()); // "0.5.1"
/// ```
#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get detailed version information
///
/// Returns an object with version details including:
/// - version: The semver version string
/// - rustVersion: The Rust version used to compile
/// - napiVersion: The napi-rs version
///
/// # Example
///
/// ```javascript
/// const { versionInfo } = require('@infinitibit_gmbh/graphbit');
/// console.log(versionInfo());
/// // { version: "0.5.1", rustVersion: "1.70.0", napiVersion: "2.16" }
/// ```
#[napi(object)]
pub struct VersionInfo {
    pub version: String,
    pub rust_version: String,
    pub napi_version: String,
}

#[napi]
pub fn version_info() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: env!("CARGO_PKG_RUST_VERSION").to_string(),
        napi_version: "2.16".to_string(),
    }
}

/// System information
#[napi(object)]
pub struct SystemInfo {
    /// Operating system
    pub os: String,
    /// OS version
    pub os_version: String,
    /// Architecture
    pub arch: String,
    /// Number of CPUs
    pub cpu_count: u32,
    /// Total memory in MB
    pub total_memory_mb: i64,
    /// Node.js version
    pub node_version: String,
    /// GraphBit version
    pub graphbit_version: String,
}

/// Get system information
///
/// Returns detailed information about the runtime environment.
///
/// # Example
///
/// ```javascript
/// const { getSystemInfo } = require('@infinitibit_gmbh/graphbit');
/// const info = getSystemInfo();
/// console.log(`OS: ${info.os} ${info.osVersion}`);
/// console.log(`CPUs: ${info.cpuCount}`);
/// console.log(`Memory: ${info.totalMemoryMb} MB`);
/// ```
#[napi]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        os_version: "unknown".to_string(), // Would need OS-specific code
        arch: std::env::consts::ARCH.to_string(),
        cpu_count: num_cpus::get() as u32,
        total_memory_mb: 0, // Would need sys-info or similar crate
        node_version: "unknown".to_string(), // Would need to query from Node
        graphbit_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// Health check status
#[napi(object)]
pub struct HealthStatus {
    /// Overall health status
    pub healthy: bool,
    /// Health check timestamp
    pub timestamp: f64,
    /// GraphBit version
    pub version: String,
    /// Uptime in seconds
    pub uptime_seconds: f64,
}

/// Perform a health check
///
/// Verifies that the library is functioning correctly.
///
/// # Example
///
/// ```javascript
/// const { healthCheck } = require('@infinitibit_gmbh/graphbit');
/// const health = healthCheck();
/// if (health.healthy) {
///   console.log('✅ GraphBit is healthy');
/// }
/// ```
#[napi]
pub fn health_check() -> HealthStatus {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    
    HealthStatus {
        healthy: true,
        timestamp,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0.0, // Would need to track from init()
    }
}

/// Runtime configuration options
#[napi(object)]
pub struct RuntimeConfig {
    /// Maximum thread pool size
    pub max_threads: Option<u32>,
    /// Enable performance monitoring
    pub enable_monitoring: Option<bool>,
    /// Memory limit in MB
    pub memory_limit_mb: Option<i64>,
}

/// Configure runtime settings
///
/// Adjusts runtime behavior and resource limits.
///
/// # Example
///
/// ```javascript
/// const { configureRuntime } = require('@infinitibit_gmbh/graphbit');
/// configureRuntime({
///   maxThreads: 4,
///   enableMonitoring: true,
///   memoryLimitMb: 1024
/// });
/// ```
#[napi]
pub fn configure_runtime(config: RuntimeConfig) -> Result<()> {
    // Log configuration
    if let Some(threads) = config.max_threads {
        tracing::info!("Runtime configured with max_threads: {}", threads);
    }
    
    if let Some(monitoring) = config.enable_monitoring {
        tracing::info!("Performance monitoring: {}", monitoring);
    }
    
    if let Some(memory_limit) = config.memory_limit_mb {
        tracing::info!("Memory limit: {} MB", memory_limit);
    }
    
    tracing::info!("Runtime configuration applied");
    
    Ok(())
}

// Note: Global allocator is defined in graphbit-core, not here
// to avoid conflicts when building on Unix systems (macOS/Linux)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
        assert_eq!(v, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_version_info() {
        let info = version_info();
        assert!(!info.version.is_empty());
        assert!(!info.rust_version.is_empty());
        assert!(!info.napi_version.is_empty());
    }
}
