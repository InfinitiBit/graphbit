//! Production-grade Tokio runtime optimized for GraphBit Python bindings
//!
//! This module provides a highly optimized, configurable async runtime with
//! proper resource management, monitoring, and graceful shutdown capabilities.

use std::sync::OnceLock;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tracing::{error, info, warn};

/// Production-grade runtime configuration
#[derive(Debug, Clone)]
pub(crate) struct RuntimeConfig {
    /// Number of worker threads (auto-detected if None)
    pub worker_threads: Option<usize>,
    /// Thread stack size in bytes
    pub thread_stack_size: Option<usize>,
    /// Enable blocking thread pool
    pub enable_blocking_pool: bool,
    /// Maximum blocking threads
    pub max_blocking_threads: Option<usize>,
    /// Thread keep alive duration
    pub thread_keep_alive: Option<Duration>,
    /// Thread name prefix
    pub thread_name_prefix: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            // Use 2x CPU cores for optimal performance, but cap at 32
            worker_threads: Some((cpu_count * 2).clamp(4, 32)),
            // Smaller stack size for memory efficiency
            thread_stack_size: Some(1024 * 1024), // 1MB
            enable_blocking_pool: true,
            // Blocking threads for I/O operations
            max_blocking_threads: Some(cpu_count * 4),
            thread_keep_alive: Some(Duration::from_secs(10)),
            thread_name_prefix: "graphbit-py".to_string(),
        }
    }
}

/// Runtime wrapper with monitoring and management capabilities
pub(crate) struct GraphBitRuntime {
    runtime: Runtime,
    config: RuntimeConfig,
    created_at: std::time::Instant,
}

impl GraphBitRuntime {
    /// Create a new runtime with the given configuration
    pub(crate) fn new(config: RuntimeConfig) -> Result<Self, std::io::Error> {
        info!("Creating GraphBit runtime with config: {:?}", config);

        let mut builder = Builder::new_multi_thread();

        // Configure worker threads
        if let Some(workers) = config.worker_threads {
            builder.worker_threads(workers);
            info!("Runtime configured with {} worker threads", workers);
        }

        // Configure thread stack size
        if let Some(stack_size) = config.thread_stack_size {
            builder.thread_stack_size(stack_size);
        }

        // Configure thread naming
        builder.thread_name(&config.thread_name_prefix);

        // Configure thread keep alive
        if let Some(keep_alive) = config.thread_keep_alive {
            builder.thread_keep_alive(keep_alive);
        }

        // Configure blocking thread pool
        if config.enable_blocking_pool {
            if let Some(max_blocking) = config.max_blocking_threads {
                builder.max_blocking_threads(max_blocking);
            }
        }

        // Enable all runtime features
        builder.enable_all();

        // Build the runtime
        let runtime = builder.build().map_err(|e| {
            error!("Failed to create GraphBit runtime: {}", e);
            e
        })?;

        info!("GraphBit runtime created successfully");

        Ok(Self {
            runtime,
            config,
            created_at: std::time::Instant::now(),
        })
    }

    /// Get a reference to the underlying runtime
    pub(crate) fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// Get runtime uptime
    pub(crate) fn uptime(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Get runtime statistics (placeholder for future metrics)
    pub(crate) fn stats(&self) -> RuntimeStats {
        RuntimeStats {
            uptime: self.uptime(),
            worker_threads: self.config.worker_threads.unwrap_or(0),
            max_blocking_threads: self.config.max_blocking_threads.unwrap_or(0),
        }
    }
}

/// Runtime statistics for monitoring
#[derive(Debug, Clone)]
pub(crate) struct RuntimeStats {
    pub uptime: Duration,
    pub worker_threads: usize,
    pub max_blocking_threads: usize,
}

/// Global runtime instance with lazy initialization and proper error handling
static GRAPHBIT_RUNTIME: OnceLock<GraphBitRuntime> = OnceLock::new();

/// Get the optimized runtime instance with proper error handling
pub(crate) fn get_runtime() -> &'static Runtime {
    GRAPHBIT_RUNTIME
        .get_or_init(|| {
            let config = RuntimeConfig::default();
            match GraphBitRuntime::new(config) {
                Ok(runtime) => runtime,
                Err(e) => {
                    error!("Critical error: Failed to create GraphBit runtime: {}", e);
                    panic!("Failed to create GraphBit runtime: {}", e);
                }
            }
        })
        .runtime()
}

/// Initialize runtime with custom configuration
pub(crate) fn init_runtime_with_config(config: RuntimeConfig) -> Result<(), std::io::Error> {
    if GRAPHBIT_RUNTIME.get().is_some() {
        warn!("Runtime already initialized, ignoring new configuration");
        return Ok(());
    }

    let runtime = GraphBitRuntime::new(config)?;
    GRAPHBIT_RUNTIME.set(runtime).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Runtime already initialized",
        )
    })?;

    Ok(())
}

/// Get runtime statistics for monitoring
pub(crate) fn get_runtime_stats() -> Option<RuntimeStats> {
    GRAPHBIT_RUNTIME.get().map(|runtime| runtime.stats())
}

/// Check if runtime is initialized
pub(crate) fn is_runtime_initialized() -> bool {
    GRAPHBIT_RUNTIME.get().is_some()
}

/// Shutdown runtime gracefully (for testing and cleanup)
pub(crate) fn shutdown_runtime() {
    if GRAPHBIT_RUNTIME.get().is_some() {
        info!("Shutting down GraphBit runtime gracefully");
        // Note: In production, runtime shutdown is handled automatically
        // This is primarily for testing scenarios
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_config_default() {
        let config = RuntimeConfig::default();
        assert!(config.worker_threads.is_some());
        assert!(config.thread_stack_size.is_some());
        assert!(config.enable_blocking_pool);
    }

    #[test]
    fn test_runtime_creation() {
        let config = RuntimeConfig::default();
        let runtime = GraphBitRuntime::new(config).expect("Failed to create runtime");
        assert!(runtime.uptime().as_nanos() > 0);
    }
}
