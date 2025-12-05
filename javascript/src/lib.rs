#![deny(clippy::all)]

//! GraphBit JavaScript/Node.js Bindings
//!
//! High-performance native Node.js bindings for the GraphBit agentic workflow
//! automation framework, built with napi-rs.

use napi::bindgen_prelude::*;
use napi_derive::napi;

// Module declarations
mod errors;
mod types;
mod llm;
mod workflow;
mod agent;
mod graph;
mod document_loader;
mod text_splitter;
mod embeddings;
mod validation;
mod tools;
pub use tools::*;

/// Initialize the GraphBit library
///
/// This should be called once at the start of your application.
/// It sets up logging and other global state.
///
/// # Example
///
/// ```javascript
/// const { init } = require('@graphbit/core');
/// init();
/// ```
#[napi]
pub fn init() -> Result<()> {
    // Initialize tracing subscriber for logging
    let _ = tracing_subscriber::fmt::try_init();
    
    tracing::info!("GraphBit JavaScript bindings v{} initialized", env!("CARGO_PKG_VERSION"));
    
    Ok(())
}

/// Get the version of the GraphBit library
///
/// # Example
///
/// ```javascript
/// const { version } = require('@graphbit/core');
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
/// const { versionInfo } = require('@graphbit/core');
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

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

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

