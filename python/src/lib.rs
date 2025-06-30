//! Python bindings for the GraphBit agentic workflow automation framework
//!
//! This module provides comprehensive Python bindings with:
//! - Robust error handling and validation
//! - Performance monitoring and metrics
//! - Configurable logging and tracing
//! - Resource management and cleanup
//! - Thread-safe operations
//!
//! # Usage
//!
//! ```python
//! import graphbit
//!
//! # Initialize the library
//! graphbit.init()
//!
//! # Check version
//! print(f"GraphBit version: {graphbit.version()}")
//!
//! # Create and configure components
//! config = graphbit.LlmConfig("openai", api_key="your-key")
//! client = graphbit.LlmClient(config)
//! ```

#![allow(non_local_definitions)]
#![deny(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
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

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::Once;
use tracing::{error, info, warn};

// Module declarations
mod embeddings;
mod errors;
mod llm;
mod runtime;
mod validation;
mod workflow;

// Re-export all public types and functions
pub use embeddings::{EmbeddingClient, EmbeddingConfig};
pub use llm::{LlmClient, LlmConfig};
pub use workflow::{Executor, Node, Workflow, WorkflowResult};

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
#[pyfunction]
#[pyo3(signature = (log_level=None, enable_tracing=None, debug=None))]
fn init(
    log_level: Option<String>,
    enable_tracing: Option<bool>,
    debug: Option<bool>,
) -> PyResult<()> {
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
                    "GraphBit Python bindings - tracing initialized with level: {}",
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
                init_result = Err(errors::to_py_runtime_error(e));
            }
        }
    });

    init_result
}

/// Get the current version of GraphBit
///
/// Returns the version string of the GraphBit core library.
#[pyfunction]
fn version() -> String {
    graphbit_core::VERSION.to_string()
}

/// Get comprehensive system information and health status
///
/// Returns a dictionary containing:
/// - Version information
/// - Runtime statistics
/// - System capabilities
/// - Health status
#[pyfunction]
fn get_system_info(py: Python<'_>) -> PyResult<Bound<'_, PyDict>> {
    let dict = PyDict::new(py);

    // Version information
    dict.set_item("version", version())?;
    dict.set_item("python_binding_version", env!("CARGO_PKG_VERSION"))?;

    // Runtime information
    if let Some(stats) = runtime::get_runtime_stats() {
        dict.set_item("runtime_uptime_seconds", stats.uptime.as_secs())?;
        dict.set_item("runtime_worker_threads", stats.worker_threads)?;
        dict.set_item("runtime_max_blocking_threads", stats.max_blocking_threads)?;
    }

    // System information
    dict.set_item("cpu_count", num_cpus::get())?;
    dict.set_item("runtime_initialized", runtime::is_runtime_initialized())?;

    // Memory allocator information
    #[cfg(target_os = "linux")]
    dict.set_item("memory_allocator", "jemalloc")?;
    #[cfg(not(target_os = "linux"))]
    dict.set_item("memory_allocator", "system")?;

    // Build information
    dict.set_item(
        "build_target",
        std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()),
    )?;
    dict.set_item(
        "build_profile",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    )?;

    Ok(dict)
}

/// Validate the current environment and configuration
///
/// Performs comprehensive health checks including:
/// - Runtime status
/// - Memory availability
/// - Thread pool status
/// - Core library health
#[pyfunction]
fn health_check(py: Python<'_>) -> PyResult<Bound<'_, PyDict>> {
    let dict = PyDict::new(py);
    let mut overall_healthy = true;

    // Check runtime health
    let runtime_healthy = runtime::is_runtime_initialized();
    dict.set_item("runtime_healthy", runtime_healthy)?;
    if !runtime_healthy {
        overall_healthy = false;
        warn!("Runtime is not properly initialized");
    }

    // Check if we can get runtime stats
    if let Some(stats) = runtime::get_runtime_stats() {
        dict.set_item("runtime_uptime_ok", stats.uptime.as_secs() > 0)?;
        dict.set_item("worker_threads_ok", stats.worker_threads > 0)?;
    } else {
        dict.set_item("runtime_stats_available", false)?;
        overall_healthy = false;
    }

    // Memory check (basic)
    let available_memory_mb = if let Ok(sys_info) = sys_info::mem_info() {
        dict.set_item("total_memory_mb", sys_info.total / 1024)?;
        dict.set_item("available_memory_mb", sys_info.avail / 1024)?;
        sys_info.avail / 1024
    } else {
        dict.set_item("memory_info_available", false)?;
        1024 // Assume 1GB if we can't detect
    };

    let memory_healthy = available_memory_mb > 100; // At least 100MB
    dict.set_item("memory_healthy", memory_healthy)?;
    if !memory_healthy {
        overall_healthy = false;
        warn!("Low available memory: {} MB", available_memory_mb);
    }

    dict.set_item("overall_healthy", overall_healthy)?;
    dict.set_item("timestamp", chrono::Utc::now().timestamp())?;

    if overall_healthy {
        info!("Health check passed");
    } else {
        warn!("Health check failed - some components are unhealthy");
    }

    Ok(dict)
}

/// Configure the global runtime with custom settings
///
/// This is an advanced function that allows customization of the async runtime.
/// It should be called before `init()` if custom configuration is needed.
#[pyfunction]
#[pyo3(signature = (worker_threads=None, max_blocking_threads=None, thread_stack_size_mb=None))]
fn configure_runtime(
    worker_threads: Option<i32>,
    max_blocking_threads: Option<i32>,
    thread_stack_size_mb: Option<i32>,
) -> PyResult<()> {
    // Validate and convert worker_threads
    let validated_worker_threads = if let Some(threads) = worker_threads {
        if threads <= 0 {
            return Err(PyValueError::new_err(format!(
                "worker_threads must be positive, got: {}",
                threads
            )));
        }
        Some(threads as usize)
    } else {
        None
    };

    // Validate and convert max_blocking_threads
    let validated_max_blocking_threads = if let Some(threads) = max_blocking_threads {
        if threads <= 0 {
            return Err(PyValueError::new_err(format!(
                "max_blocking_threads must be positive, got: {}",
                threads
            )));
        }
        Some(threads as usize)
    } else {
        None
    };

    // Validate and convert thread_stack_size_mb
    let validated_thread_stack_size = if let Some(mb) = thread_stack_size_mb {
        if mb <= 0 {
            return Err(PyValueError::new_err(format!(
                "thread_stack_size_mb must be positive, got: {}",
                mb
            )));
        }
        Some((mb as usize) * 1024 * 1024)
    } else {
        None
    };

    let config = runtime::RuntimeConfig {
        worker_threads: validated_worker_threads,
        thread_stack_size: validated_thread_stack_size,
        enable_blocking_pool: true,
        max_blocking_threads: validated_max_blocking_threads,
        thread_keep_alive: Some(std::time::Duration::from_secs(10)),
        thread_name_prefix: "graphbit-py".to_string(),
    };

    runtime::init_runtime_with_config(config).map_err(errors::to_py_runtime_error)?;

    info!("Runtime configured with custom settings");
    Ok(())
}

/// Gracefully shutdown the library (for testing and cleanup)
///
/// This function cleans up resources and shuts down background threads.
/// It's primarily intended for testing and should not be called in normal usage.
#[pyfunction]
fn shutdown() -> PyResult<()> {
    info!("Shutting down GraphBit Python bindings");
    runtime::shutdown_runtime();
    Ok(())
}

/// The main Python module definition
///
/// This exposes all GraphBit functionality to Python with proper organization
/// and comprehensive error handling.
#[pymodule]
fn graphbit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core functions
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(get_system_info, m)?)?;
    m.add_function(wrap_pyfunction!(health_check, m)?)?;
    m.add_function(wrap_pyfunction!(configure_runtime, m)?)?;
    m.add_function(wrap_pyfunction!(shutdown, m)?)?;

    // LLM classes
    m.add_class::<LlmConfig>()?;
    m.add_class::<LlmClient>()?;

    // Workflow classes
    m.add_class::<Node>()?;
    m.add_class::<Workflow>()?;
    m.add_class::<WorkflowResult>()?;
    m.add_class::<Executor>()?;

    // Embedding classes
    m.add_class::<EmbeddingConfig>()?;
    m.add_class::<EmbeddingClient>()?;

    // Module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "GraphBit Team")?;
    m.add(
        "__description__",
        "Production-grade Python bindings for GraphBit agentic workflow automation",
    )?;

    Ok(())
}
