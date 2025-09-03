//! Runtime management for GraphBit Node.js bindings

use graphbit_core::GraphBitError;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::runtime::{Builder, Runtime};
use tracing::{error, info, warn};

/// Runtime statistics
#[derive(Debug, Clone)]
pub struct RuntimeStats {
    pub uptime: Duration,
    pub worker_threads: usize,
    pub max_blocking_threads: usize,
}

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub worker_threads: Option<usize>,
    pub thread_stack_size: Option<usize>,
    pub enable_blocking_pool: bool,
    pub max_blocking_threads: Option<usize>,
    pub thread_keep_alive: Option<Duration>,
    pub thread_name_prefix: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: None,    // Use default (number of CPU cores)
            thread_stack_size: None, // Use system default
            enable_blocking_pool: true,
            max_blocking_threads: None, // Use default (512)
            thread_keep_alive: Some(Duration::from_secs(10)),
            thread_name_prefix: "graphbit-js".to_string(),
        }
    }
}

/// Global runtime instance
static RUNTIME: OnceLock<Arc<RuntimeManager>> = OnceLock::new();

/// Runtime manager with initialization tracking
pub struct RuntimeManager {
    pub runtime: Runtime,
    start_time: Instant,
    config: RuntimeConfig,
}

impl RuntimeManager {
    fn new(config: RuntimeConfig) -> Result<Self, GraphBitError> {
        info!("Initializing GraphBit runtime with config: {:?}", config);

        let mut builder = Builder::new_multi_thread();

        // Configure worker threads
        if let Some(threads) = config.worker_threads {
            builder.worker_threads(threads);
        }

        // Configure thread stack size
        if let Some(stack_size) = config.thread_stack_size {
            builder.thread_stack_size(stack_size);
        }

        // Configure blocking thread pool
        if config.enable_blocking_pool {
            if let Some(max_threads) = config.max_blocking_threads {
                builder.max_blocking_threads(max_threads);
            }
        }

        // Configure thread keep-alive
        if let Some(keep_alive) = config.thread_keep_alive {
            builder.thread_keep_alive(keep_alive);
        }

        // Configure thread naming
        builder.thread_name(&config.thread_name_prefix);

        // Enable all features for production use
        builder.enable_all();

        let runtime = builder.build().map_err(|e| {
            error!("Failed to build Tokio runtime: {}", e);
            GraphBitError::config(format!("Failed to initialize runtime: {}", e))
        })?;

        info!("GraphBit runtime initialized successfully");

        Ok(RuntimeManager {
            runtime,
            start_time: Instant::now(),
            config,
        })
    }

    fn get_stats(&self) -> RuntimeStats {
        RuntimeStats {
            uptime: self.start_time.elapsed(),
            worker_threads: self.config.worker_threads.unwrap_or_else(num_cpus::get),
            max_blocking_threads: self.config.max_blocking_threads.unwrap_or(512),
        }
    }
}

/// Initialize runtime with default configuration
pub fn get_runtime() -> Arc<RuntimeManager> {
    RUNTIME
        .get_or_init(|| match RuntimeManager::new(RuntimeConfig::default()) {
            Ok(manager) => Arc::new(manager),
            Err(e) => {
                error!("Failed to initialize runtime: {}", e);
                panic!("Critical failure: Could not initialize runtime: {}", e);
            }
        })
        .clone()
}

/// Initialize runtime with custom configuration
pub fn init_runtime_with_config(config: RuntimeConfig) -> Result<(), GraphBitError> {
    if RUNTIME.get().is_some() {
        warn!("Runtime already initialized, ignoring new configuration");
        return Ok(());
    }

    let manager = RuntimeManager::new(config)?;
    let _ = RUNTIME.set(Arc::new(manager));
    Ok(())
}

/// Check if runtime is initialized
pub fn is_runtime_initialized() -> bool {
    RUNTIME.get().is_some()
}

/// Get runtime statistics
pub fn get_runtime_stats() -> Option<RuntimeStats> {
    RUNTIME.get().map(|manager| manager.get_stats())
}

/// Shutdown the runtime (for testing and cleanup)
pub fn shutdown_runtime() {
    if let Some(_manager) = RUNTIME.get() {
        info!("Shutting down GraphBit runtime");
        // Runtime shutdown is handled automatically when dropped
        info!("GraphBit runtime shutdown complete");
    }
}
