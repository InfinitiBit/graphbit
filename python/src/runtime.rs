//! Ultra-fast tokio runtime optimized for GraphBit

use std::sync::OnceLock;

/// Ultra-fast tokio runtime optimized for GraphBit
static GRAPHBIT_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

/// Get the optimized runtime
pub fn get_runtime() -> &'static tokio::runtime::Runtime {
    GRAPHBIT_RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(20) // More workers for better concurrency
            .thread_name("graphbit-async")
            .thread_stack_size(1024 * 1024) // Smaller stack for efficiency
            .enable_all()
            .build()
            .expect("Failed to create GraphBit runtime")
    })
}
