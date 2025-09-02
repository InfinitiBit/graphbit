//! Production-grade error handling for GraphBit Node.js bindings
//!
//! This module provides comprehensive error handling with proper error context,
//! structured error types, and integration with logging systems.

use napi::bindgen_prelude::*;
use std::fmt;
use tracing::{error, warn};

/// Enhanced error types for better Node.js integration
#[derive(Debug, Clone)]
pub(crate) enum NodeJsBindingError {
    /// Core library error
    Core(String),
    /// Configuration error with context
    Configuration {
        message: String,
        field: Option<String>,
    },
    /// Runtime error with operation context
    Runtime { message: String, operation: String },
    /// Network error with retry information
    Network { message: String, retry_count: u32 },
    /// Authentication error with provider context
    Authentication {
        message: String,
        provider: Option<String>,
    },
    /// Validation error with field-specific details
    Validation {
        message: String,
        field: String,
        value: Option<String>,
    },
    /// Rate limiting error with backoff information
    RateLimit {
        message: String,
        retry_after: Option<u64>,
    },
    /// Timeout error with operation details
    Timeout {
        message: String,
        operation: String,
        duration_ms: u64,
    },
    /// Resource exhaustion error
    ResourceExhausted {
        message: String,
        resource_type: String,
    },
}

impl fmt::Display for NodeJsBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeJsBindingError::Core(msg) => write!(f, "Core error: {}", msg),
            NodeJsBindingError::Configuration { message, field } => {
                if let Some(field) = field {
                    write!(f, "Configuration error in field '{}': {}", field, message)
                } else {
                    write!(f, "Configuration error: {}", message)
                }
            }
            NodeJsBindingError::Runtime { message, operation } => {
                write!(f, "Runtime error during '{}': {}", operation, message)
            }
            NodeJsBindingError::Network {
                message,
                retry_count,
            } => {
                write!(f, "Network error (attempt {}): {}", retry_count, message)
            }
            NodeJsBindingError::Authentication { message, provider } => {
                if let Some(provider) = provider {
                    write!(f, "Authentication error with {}: {}", provider, message)
                } else {
                    write!(f, "Authentication error: {}", message)
                }
            }
            NodeJsBindingError::Validation {
                message,
                field,
                value,
            } => {
                if let Some(value) = value {
                    write!(
                        f,
                        "Validation error for field '{}' with value '{}': {}",
                        field, value, message
                    )
                } else {
                    write!(f, "Validation error for field '{}': {}", field, message)
                }
            }
            NodeJsBindingError::RateLimit {
                message,
                retry_after,
            } => {
                if let Some(retry_after) = retry_after {
                    write!(
                        f,
                        "Rate limit exceeded: {} (retry after {} seconds)",
                        message, retry_after
                    )
                } else {
                    write!(f, "Rate limit exceeded: {}", message)
                }
            }
            NodeJsBindingError::Timeout {
                message,
                operation,
                duration_ms,
            } => {
                write!(
                    f,
                    "Timeout error during '{}' after {}ms: {}",
                    operation, duration_ms, message
                )
            }
            NodeJsBindingError::ResourceExhausted {
                message,
                resource_type,
            } => {
                write!(f, "Resource exhausted ({}): {}", resource_type, message)
            }
        }
    }
}

/// Convert GraphBit core errors to Node.js exceptions with enhanced error mapping
pub(crate) fn to_napi_error(error: graphbit_core::errors::GraphBitError) -> Error {
    use graphbit_core::errors::GraphBitError;

    // Log the error for debugging
    error!("GraphBit error in Node.js binding: {}", error);

    let nodejs_error = match error {
        GraphBitError::Network { message, .. } => NodeJsBindingError::Network {
            message: message.clone(),
            retry_count: 1,
        },
        GraphBitError::Authentication { message, .. } => NodeJsBindingError::Authentication {
            message: message.clone(),
            provider: None,
        },
        GraphBitError::RateLimit { .. } => NodeJsBindingError::RateLimit {
            message: "Rate limit exceeded".to_string(),
            retry_after: None,
        },
        GraphBitError::Validation { message, .. } => NodeJsBindingError::Validation {
            message: message.clone(),
            field: "unknown".to_string(),
            value: None,
        },
        GraphBitError::Configuration { message, .. } => NodeJsBindingError::Configuration {
            message: message.clone(),
            field: None,
        },
        _ => NodeJsBindingError::Core(error.to_string()),
    };

    nodejs_error_to_napi_error(nodejs_error)
}

/// Convert Node.js binding errors to NAPI Error with appropriate status codes
pub(crate) fn nodejs_error_to_napi_error(error: NodeJsBindingError) -> Error {
    let status = match &error {
        NodeJsBindingError::Network { .. } => Status::GenericFailure,
        NodeJsBindingError::Authentication { .. } => Status::InvalidArg,
        NodeJsBindingError::RateLimit { .. } => Status::GenericFailure,
        NodeJsBindingError::Validation { .. } => Status::InvalidArg,
        NodeJsBindingError::Configuration { .. } => Status::InvalidArg,
        NodeJsBindingError::Runtime { .. } => Status::GenericFailure,
        NodeJsBindingError::Timeout { .. } => Status::GenericFailure,
        NodeJsBindingError::ResourceExhausted { .. } => Status::GenericFailure,
        NodeJsBindingError::Core(_) => Status::GenericFailure,
    };

    // Log warning for non-critical errors
    match &error {
        NodeJsBindingError::Network { retry_count, .. } if *retry_count <= 3 => {
            warn!("Network error (retryable): {}", error);
        }
        NodeJsBindingError::RateLimit { .. } => {
            warn!("Rate limit error (temporary): {}", error);
        }
        _ => {
            error!("Node.js binding error: {}", error);
        }
    }

    Error::new(status, error.to_string())
}

/// Convert a runtime error to a NAPI error with enhanced context
pub(crate) fn to_napi_runtime_error(error: graphbit_core::GraphBitError) -> Error {
    let nodejs_error = NodeJsBindingError::Runtime {
        message: error.to_string(),
        operation: "runtime_initialization".to_string(),
    };

    nodejs_error_to_napi_error(nodejs_error)
}

/// Create a validation error with detailed context
pub(crate) fn validation_error(field: &str, message: &str, value: Option<&str>) -> Error {
    let error = NodeJsBindingError::Validation {
        message: message.to_string(),
        field: field.to_string(),
        value: value.map(|v| v.to_string()),
    };

    nodejs_error_to_napi_error(error)
}

/// Create a configuration error with field context
pub(crate) fn configuration_error(field: Option<&str>, message: &str) -> Error {
    let error = NodeJsBindingError::Configuration {
        message: message.to_string(),
        field: field.map(|f| f.to_string()),
    };

    nodejs_error_to_napi_error(error)
}

/// Create an authentication error with provider context
pub(crate) fn authentication_error(provider: Option<&str>, message: &str) -> Error {
    let error = NodeJsBindingError::Authentication {
        message: message.to_string(),
        provider: provider.map(|p| p.to_string()),
    };

    nodejs_error_to_napi_error(error)
}
