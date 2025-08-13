//! TypeScript/JavaScript bindings for the GraphBit agentic workflow automation framework
//!
//! This module provides comprehensive Node.js bindings with:
//! - Robust error handling and validation
//! - Performance monitoring and metrics
//! - Configurable logging and tracing
//! - Resource management and cleanup
//! - Thread-safe operations
//! - Full TypeScript support
//!
//! # Usage
//!
//! ```typescript
//! import { init, version, LlmConfig, LlmClient, Workflow, Node } from 'graphbit';
//!
//! // Initialize the library
//! await init();
//!
//! // Check version
//! console.log(`GraphBit version: ${version()}`);
//!
//! // Create and configure components
//! const config = new LlmConfig("openai", { apiKey: "your-key" });
//! const client = new LlmClient(config);
//! ```

#![deny(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Once;
use tracing::{error, info, warn};

// Module declarations
mod document_loader;
mod embeddings;
mod errors;
mod llm;
mod runtime;
mod text_splitter;
mod validation;
mod workflow;

// Re-export all public types and functions
pub use document_loader::{DocumentContent, DocumentLoader, DocumentLoaderConfig};
pub use embeddings::{EmbeddingClient, EmbeddingConfig};
pub use llm::{LlmClient, LlmConfig};
pub use text_splitter::{
    CharacterSplitter, RecursiveSplitter, SentenceSplitter, TextChunk, TextSplitterConfig,
    TokenSplitter,
};
pub use workflow::{Executor, Node, Workflow, WorkflowContext, WorkflowResult};

/// Global initialization flag to ensure init is called only once
static INIT: Once = Once::new();

/// Initialize the GraphBit library with production-grade configuration
///
/// This function should be called once before using any other GraphBit functionality.
/// It sets up:
/// - Logging and tracing infrastructure
/// - Runtime configuration
/// - Core library initialization
/// - Resource management
#[napi]
pub async fn init(
    log_level: Option<String>,
    enable_tracing: Option<bool>,
    debug: Option<bool>,
) -> Result<()> {
    let mut init_result = Ok(());

    INIT.call_once(|| {
        // Initialize logging if tracing is enabled
        // Default to false for tracing to reduce debug output
        let enable_tracing = enable_tracing.or(debug).unwrap_or(false);
        if enable_tracing {
            let log_level = log_level.unwrap_or_else(|| "warn".to_string());

            // Initialize tracing subscriber for production logging
            let subscriber = tracing_subscriber::FmtSubscriber::builder()
                .with_max_level(match log_level.as_str() {
                    "trace" => tracing::Level::TRACE,
                    "debug" => tracing::Level::DEBUG,
                    "info" => tracing::Level::INFO,
                    "warn" => tracing::Level::WARN,
                    "error" => tracing::Level::ERROR,
                    _ => tracing::Level::WARN,
                })
                .with_thread_ids(false) // Disable thread IDs for cleaner output
                .with_thread_names(false) // Disable thread names for cleaner output
                .with_file(false) // Disable file info for performance
                .with_line_number(false) // Disable line numbers for performance
                .finish();

            if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
                eprintln!("Warning: Failed to set tracing subscriber: {}", e);
            } else {
                info!(
                    "GraphBit Node.js bindings - tracing initialized with level: {}",
                    log_level
                );
            }
        }

        // Initialize the core library
        match graphbit_core::init() {
            Ok(_) => {
                if enable_tracing {
                    info!("GraphBit core library initialized successfully");
                }

                // Initialize runtime to ensure it's ready
                let _ = runtime::get_runtime();
                if enable_tracing {
                    info!("GraphBit runtime initialized successfully");
                }
            }
            Err(e) => {
                error!("Failed to initialize GraphBit core library: {}", e);
                init_result = Err(errors::to_napi_error(e));
            }
        }
    });

    init_result
}

/// Get the current version of GraphBit
///
/// Returns the version string of the GraphBit core library.
#[napi]
pub fn version() -> String {
    graphbit_core::VERSION.to_string()
}

/// System information structure
#[napi(object)]
pub struct SystemInfo {
    /// Version information
    pub version: String,
    /// Node.js binding version
    pub nodejs_binding_version: String,
    /// Runtime uptime in seconds
    pub runtime_uptime_seconds: Option<i64>,
    /// Number of worker threads
    pub runtime_worker_threads: Option<i32>,
    /// Maximum blocking threads
    pub runtime_max_blocking_threads: Option<i32>,
    /// CPU count
    pub cpu_count: i32,
    /// Runtime initialization status
    pub runtime_initialized: bool,
    /// Memory allocator
    pub memory_allocator: String,
    /// Build target
    pub build_target: String,
    /// Build profile
    pub build_profile: String,
}

/// Get comprehensive system information and health status
///
/// Returns an object containing:
/// - Version information
/// - Runtime statistics
/// - System capabilities
/// - Health status
#[napi]
pub async fn get_system_info() -> Result<SystemInfo> {
    let runtime_stats = runtime::get_runtime_stats();

    Ok(SystemInfo {
        version: version(),
        nodejs_binding_version: env!("CARGO_PKG_VERSION").to_string(),
        runtime_uptime_seconds: runtime_stats.as_ref().map(|s| s.uptime.as_secs() as i64),
        runtime_worker_threads: runtime_stats.as_ref().map(|s| s.worker_threads as i32),
        runtime_max_blocking_threads: runtime_stats.as_ref().map(|s| s.max_blocking_threads as i32),
        cpu_count: num_cpus::get() as i32,
        runtime_initialized: runtime::is_runtime_initialized(),
        memory_allocator: if cfg!(target_os = "linux") {
            "jemalloc"
        } else {
            "system"
        }
        .to_string(),
        build_target: std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()),
        build_profile: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
        .to_string(),
    })
}

/// Health check result structure
#[napi(object)]
pub struct HealthCheck {
    /// Overall health status
    pub overall_healthy: bool,
    /// Runtime health status
    pub runtime_healthy: bool,
    /// Runtime uptime status
    pub runtime_uptime_ok: Option<bool>,
    /// Worker threads status
    pub worker_threads_ok: Option<bool>,
    /// Runtime stats availability
    pub runtime_stats_available: bool,
    /// Total memory in MB
    pub total_memory_mb: Option<i64>,
    /// Available memory in MB
    pub available_memory_mb: Option<i64>,
    /// Memory health status
    pub memory_healthy: bool,
    /// Memory info availability
    pub memory_info_available: bool,
    /// Timestamp of the health check
    pub timestamp: i64,
}

/// Validate the current environment and configuration
///
/// Performs comprehensive health checks including:
/// - Runtime status
/// - Memory availability
/// - Thread pool status
/// - Core library health
#[napi]
pub async fn health_check() -> Result<HealthCheck> {
    let mut overall_healthy = true;

    // Check runtime health
    let runtime_healthy = runtime::is_runtime_initialized();
    if !runtime_healthy {
        overall_healthy = false;
        warn!("Runtime is not properly initialized");
    }

    // Check if we can get runtime stats
    let runtime_stats = runtime::get_runtime_stats();
    let runtime_stats_available = runtime_stats.is_some();
    if !runtime_stats_available {
        overall_healthy = false;
    }

    let runtime_uptime_ok = runtime_stats.as_ref().map(|s| s.uptime.as_secs() > 0);
    let worker_threads_ok = runtime_stats.as_ref().map(|s| s.worker_threads > 0);

    // Memory check (basic)
    let (_total_memory_mb, available_memory_mb, memory_info_available) =
        if let Ok(sys_info) = sys_info::mem_info() {
            (
                Some(sys_info.total / 1024),
                Some(sys_info.avail / 1024),
                true,
            )
        } else {
            (None, None, false)
        };

    let memory_healthy = available_memory_mb.map_or(true, |mem| mem > 100); // At least 100MB
    if !memory_healthy {
        overall_healthy = false;
        if let Some(available) = available_memory_mb {
            warn!("Low available memory: {} MB", available);
        }
    }

    if overall_healthy {
        info!("Health check passed");
    } else {
        warn!("Health check failed - some components are unhealthy");
    }

    Ok(HealthCheck {
        overall_healthy,
        runtime_healthy,
        runtime_uptime_ok,
        worker_threads_ok,
        runtime_stats_available,
        total_memory_mb: runtime_stats.as_ref().and_then(|_| sys_info::mem_info().ok()).map(|info| info.total as i64 / 1024),
        available_memory_mb: runtime_stats.as_ref().and_then(|_| sys_info::mem_info().ok()).map(|info| info.avail as i64 / 1024),
        memory_healthy,
        memory_info_available,
        timestamp: chrono::Utc::now().timestamp(),
    })
}

/// Runtime configuration structure
#[napi(object)]
pub struct RuntimeConfig {
    /// Number of worker threads
    pub worker_threads: Option<i32>,
    /// Maximum blocking threads
    pub max_blocking_threads: Option<i32>,
    /// Thread stack size in MB
    pub thread_stack_size_mb: Option<i32>,
}

/// Configure the global runtime with custom settings
///
/// This is an advanced function that allows customization of the async runtime.
/// It should be called before `init()` if custom configuration is needed.
#[napi]
pub async fn configure_runtime(config: RuntimeConfig) -> Result<()> {
    // Validate and convert worker_threads
    let validated_worker_threads = if let Some(threads) = config.worker_threads {
        if threads <= 0 {
            return Err(Error::new(
                Status::InvalidArg,
                format!("worker_threads must be positive, got: {}", threads),
            ));
        }
        Some(threads as usize)
    } else {
        None
    };

    // Validate and convert max_blocking_threads
    let validated_max_blocking_threads = if let Some(threads) = config.max_blocking_threads {
        if threads <= 0 {
            return Err(Error::new(
                Status::InvalidArg,
                format!("max_blocking_threads must be positive, got: {}", threads),
            ));
        }
        Some(threads as usize)
    } else {
        None
    };

    // Validate and convert thread_stack_size_mb
    let validated_thread_stack_size = if let Some(mb) = config.thread_stack_size_mb {
        if mb <= 0 {
            return Err(Error::new(
                Status::InvalidArg,
                format!("thread_stack_size_mb must be positive, got: {}", mb),
            ));
        }
        Some((mb as usize) * 1024 * 1024)
    } else {
        None
    };

    let runtime_config = runtime::RuntimeConfig {
        worker_threads: validated_worker_threads,
        thread_stack_size: validated_thread_stack_size,
        enable_blocking_pool: true,
        max_blocking_threads: validated_max_blocking_threads,
        thread_keep_alive: Some(std::time::Duration::from_secs(10)),
        thread_name_prefix: "graphbit-js".to_string(),
    };

    runtime::init_runtime_with_config(runtime_config).map_err(errors::to_napi_error)?;

    info!("Runtime configured with custom settings");
    Ok(())
}

/// Gracefully shutdown the library (for testing and cleanup)
///
/// This function cleans up resources and shuts down background threads.
/// It's primarily intended for testing and should not be called in normal usage.
#[napi]
pub async fn shutdown() -> Result<()> {
    info!("Shutting down GraphBit Node.js bindings");
    runtime::shutdown_runtime();
    Ok(())
}
