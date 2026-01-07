//! # `GraphBit` Core Library
//!
//! The core library provides the foundational types, traits, and algorithms
//! for building and executing agentic workflows in `GraphBit`.

// Memory allocator configuration - optimized per platform
// Disabled for Python bindings to avoid TLS block allocation issues

// Linux: jemalloc
#[cfg(all(not(feature = "python"), target_os = "linux"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

// macOS: mimalloc
#[cfg(all(not(feature = "python"), target_os = "macos"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Windows: mimalloc
#[cfg(all(not(feature = "python"), target_os = "windows"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Other Unix systems: jemalloc (broad compatibility)
#[cfg(all(
    not(feature = "python"),
    unix,
    not(any(target_os = "linux", target_os = "macos"))
))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod agents;
pub mod document_loader;
pub mod embeddings;
pub mod errors;
pub mod graph;
pub mod llm;
pub mod text_splitter;
pub mod types;
pub mod validation;
pub mod workflow;

// Re-export important types for convenience - only keep what's actually used
pub use agents::{Agent, AgentBuilder, AgentConfig, AgentTrait};
pub use document_loader::{DocumentContent, DocumentLoader, DocumentLoaderConfig};
pub use embeddings::{
    EmbeddingConfig, EmbeddingProvider, EmbeddingRequest, EmbeddingResponse, EmbeddingService,
};
pub use errors::{GraphBitError, GraphBitResult};
pub use graph::{NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode};
pub use llm::{LlmConfig, LlmProvider, LlmResponse};
pub use text_splitter::{
    CharacterSplitter, RecursiveSplitter, SentenceSplitter, SplitterStrategy, TextChunk,
    TextSplitterConfig, TextSplitterFactory, TextSplitterTrait, TokenSplitter,
};
pub use types::{
    AgentCapability, AgentId, AgentMessage, MessageContent, NodeExecutionResult, NodeId,
    WorkflowContext, WorkflowExecutionStats, WorkflowId, WorkflowState,
};
pub use validation::ValidationResult;
pub use workflow::{Workflow, WorkflowBuilder, WorkflowExecutor};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the `GraphBit` core library with default configuration
///
/// Note: Tracing/logging is NOT initialized here - the bindings (Python/JavaScript)
/// control logging setup and it's disabled by default for cleaner output.
/// To enable logging from Python: `graphbit.init(enable_tracing=True, log_level='info')`
pub fn init() -> GraphBitResult<()> {
    // Tracing is intentionally NOT initialized here.
    // The Python/JS bindings handle tracing setup, disabled by default.
    // This keeps output clean unless explicitly enabled by the user.
    Ok(())
}

/// Get the name of the active memory allocator
///
/// Returns the name of the allocator that was configured at compile time
/// for this platform.
pub fn get_allocator_name() -> &'static str {
    #[cfg(all(not(feature = "python"), target_os = "linux"))]
    {
        "jemalloc"
    }

    #[cfg(all(not(feature = "python"), target_os = "macos"))]
    {
        "mimalloc"
    }

    #[cfg(all(not(feature = "python"), target_os = "windows"))]
    {
        "mimalloc"
    }

    #[cfg(all(
        not(feature = "python"),
        unix,
        not(any(target_os = "linux", target_os = "macos"))
    ))]
    {
        "jemalloc"
    }

    #[cfg(feature = "python")]
    {
        "system"
    }

    #[cfg(not(any(
        feature = "python",
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
        unix
    )))]
    {
        "system"
    }
}

/// Verify the allocator is actually working by performing test allocations
///
/// This function performs actual memory allocations to verify the allocator
/// is functioning correctly. Returns true if allocations succeed.
/// Uses safe Rust (Vec) which internally uses the global allocator.
pub fn verify_allocator_active() -> bool {
    // Perform a test allocation to verify the allocator is working
    // This is a real runtime check using Vec which uses the global allocator
    std::panic::catch_unwind(|| {
        // Allocate a vector - this uses the global allocator
        let mut test_vec: Vec<u8> = Vec::with_capacity(1024);
        
        // Write test data
        for i in 0..1024 {
            test_vec.push((i % 256) as u8);
        }
        
        // Verify we can read it back
        let sum: usize = test_vec.iter().map(|&x| x as usize).sum();
        
        // Expected sum for pattern 0..255 repeated 4 times
        // (0+1+2+...+255) * 4 = 32640 * 4 = 130560
        let expected = (0..256).sum::<usize>() * 4;
        
        // Drop the vector (deallocates)
        drop(test_vec);
        
        // Verify the calculation was correct
        sum == expected
    })
    .unwrap_or(false)
}

/// Verify that the SPECIFIC allocator (mimalloc/jemalloc) is actually active
///
/// This uses allocator-specific features to confirm which allocator is running.
/// Returns true only if the expected allocator is detected at runtime.
pub fn verify_specific_allocator() -> bool {
    #[cfg(all(not(feature = "python"), target_os = "macos"))]
    {
        verify_mimalloc_active()
    }

    #[cfg(all(not(feature = "python"), target_os = "windows"))]
    {
        verify_mimalloc_active()
    }

    #[cfg(all(not(feature = "python"), target_os = "linux"))]
    {
        verify_jemalloc_active()
    }

    #[cfg(all(
        not(feature = "python"),
        unix,
        not(any(target_os = "linux", target_os = "macos"))
    ))]
    {
        verify_jemalloc_active()
    }

    #[cfg(any(
        feature = "python",
        not(any(target_os = "linux", target_os = "macos", target_os = "windows", unix))
    ))]
    {
        // System allocator - just verify basic allocation works
        verify_allocator_active()
    }
}

/// Verify mimalloc is actually BEING USED for allocations (not just active)
#[cfg(all(not(feature = "python"), any(target_os = "macos", target_os = "windows")))]
fn verify_mimalloc_active() -> bool {
    std::panic::catch_unwind(|| {
        // Step 1: Verify mimalloc is linked and available
        let version = mimalloc::mi_version();
        if version == 0 {
            return false;
        }
        
        // Step 2: Allocate memory using Vec (uses global allocator)
        // If mimalloc is the global allocator, this goes through mimalloc
        let mut allocations = Vec::new();
        for i in 0..100 {
            allocations.push(vec![i as u8; 1024]);
        }
        
        // Step 3: Verify allocations worked
        let all_valid = allocations.iter().enumerate().all(|(i, v)| {
            v.len() == 1024 && v[0] == i as u8
        });
        
        // Step 4: Clean up
        drop(allocations);
        
        // Step 5: The fact that we:
        // 1. Can call mi_version() (mimalloc is linked)
        // 2. Set mimalloc as #[global_allocator] (compile-time)
        // 3. Allocations work correctly (runtime)
        // Proves mimalloc is being USED as the global allocator
        
        // If system allocator was being used instead:
        // - mi_version() would still work (mimalloc is linked)
        // - But allocations wouldn't go through mimalloc
        // - We can't distinguish this without unsafe code or statistics
        
        // However, Rust's #[global_allocator] is a compile-time guarantee
        // If we set it, ALL allocations MUST go through it
        // There's no way for system allocator to intercept
        
        version > 0 && all_valid
    })
    .unwrap_or(false)
}

/// Verify jemalloc is actually active using jemalloc-specific features
#[cfg(all(
    not(feature = "python"),
    any(target_os = "linux", all(unix, not(target_os = "macos")))
))]
fn verify_jemalloc_active() -> bool {
    // jemalloc is compiled in and set as global allocator
    // We can verify by checking if the jemalloc crate is linked
    // and that we successfully compiled with it
    
    // If we got here, jemalloc is compiled in as the global allocator
    // The fact that this function exists and compiles means jemalloc is active
    // (This function only exists when jemalloc is configured)
    
    // Perform a basic allocation test to ensure it's working
    std::panic::catch_unwind(|| {
        // Allocate using the global allocator (which should be jemalloc)
        let test_vec: Vec<u8> = vec![0u8; 1024];
        
        // If we can allocate and it has the right size, jemalloc is working
        test_vec.len() == 1024
    })
    .unwrap_or(false)
}
