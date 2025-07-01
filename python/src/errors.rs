//! Production-grade error handling for GraphBit Python bindings
//!
//! This module provides comprehensive error handling with proper error context,
//! structured error types, and integration with logging systems.

use pyo3::prelude::*;
use std::fmt;
use tracing::{error, warn};

/// Enhanced error types for better Python integration
#[derive(Debug, Clone)]
pub(crate) enum PythonBindingError {
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

impl fmt::Display for PythonBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonBindingError::Core(msg) => write!(f, "Core error: {}", msg),
            PythonBindingError::Configuration { message, field } => {
                if let Some(field) = field {
                    write!(f, "Configuration error in field '{}': {}", field, message)
                } else {
                    write!(f, "Configuration error: {}", message)
                }
            }
            PythonBindingError::Runtime { message, operation } => {
                write!(f, "Runtime error during '{}': {}", operation, message)
            }
            PythonBindingError::Network {
                message,
                retry_count,
            } => {
                write!(f, "Network error (attempt {}): {}", retry_count, message)
            }
            PythonBindingError::Authentication { message, provider } => {
                if let Some(provider) = provider {
                    write!(f, "Authentication error with {}: {}", provider, message)
                } else {
                    write!(f, "Authentication error: {}", message)
                }
            }
            PythonBindingError::Validation {
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
            PythonBindingError::RateLimit {
                message,
                retry_after,
            } => {
                if let Some(retry_after) = retry_after {
                    write!(
                        f,
                        "Rate limit exceeded (retry after {}s): {}",
                        retry_after, message
                    )
                } else {
                    write!(f, "Rate limit exceeded: {}", message)
                }
            }
            PythonBindingError::Timeout {
                message,
                operation,
                duration_ms,
            } => {
                write!(
                    f,
                    "Timeout in '{}' after {}ms: {}",
                    operation, duration_ms, message
                )
            }
            PythonBindingError::ResourceExhausted {
                message,
                resource_type,
            } => {
                write!(f, "Resource exhausted ({}): {}", resource_type, message)
            }
        }
    }
}

/// Convert GraphBit core errors to Python exceptions with enhanced error mapping
pub(crate) fn to_py_error(error: graphbit_core::errors::GraphBitError) -> PyErr {
    use graphbit_core::errors::GraphBitError;

    // Log the error for debugging
    error!("GraphBit error in Python binding: {}", error);

    let python_error = match error {
        GraphBitError::Network { message, .. } => PythonBindingError::Network {
            message: message.clone(),
            retry_count: 1,
        },
        GraphBitError::Authentication { message, .. } => PythonBindingError::Authentication {
            message: message.clone(),
            provider: None,
        },
        GraphBitError::RateLimit { .. } => PythonBindingError::RateLimit {
            message: "Rate limit exceeded".to_string(),
            retry_after: None,
        },
        GraphBitError::Validation { message, .. } => PythonBindingError::Validation {
            message: message.clone(),
            field: "unknown".to_string(),
            value: None,
        },
        GraphBitError::Configuration { message, .. } => PythonBindingError::Configuration {
            message: message.clone(),
            field: None,
        },
        _ => PythonBindingError::Core(error.to_string()),
    };

    python_error_to_py_err(python_error)
}

/// Convert Python binding errors to PyErr with appropriate exception types
pub(crate) fn python_error_to_py_err(error: PythonBindingError) -> PyErr {
    match &error {
        PythonBindingError::Network { .. } => {
            PyErr::new::<pyo3::exceptions::PyConnectionError, _>(error.to_string())
        }
        PythonBindingError::Authentication { .. } => {
            PyErr::new::<pyo3::exceptions::PyPermissionError, _>(error.to_string())
        }
        PythonBindingError::Validation { .. } => {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(error.to_string())
        }
        PythonBindingError::Configuration { .. } => {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(error.to_string())
        }
        PythonBindingError::RateLimit { .. } => {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.to_string())
        }
        PythonBindingError::Timeout { .. } => {
            PyErr::new::<pyo3::exceptions::PyTimeoutError, _>(error.to_string())
        }
        PythonBindingError::ResourceExhausted { .. } => {
            PyErr::new::<pyo3::exceptions::PyMemoryError, _>(error.to_string())
        }
        _ => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.to_string()),
    }
}

/// Enhanced utility function to convert any error to PyErr with context
pub(crate) fn to_py_runtime_error<E: std::fmt::Display>(error: E) -> PyErr {
    let error_msg = error.to_string();
    warn!(
        "Converting runtime error to Python exception: {}",
        error_msg
    );

    // Check for common error patterns and map to appropriate exceptions
    if error_msg.contains("timeout") || error_msg.contains("deadline") {
        PyErr::new::<pyo3::exceptions::PyTimeoutError, _>(error_msg)
    } else if error_msg.contains("permission") || error_msg.contains("unauthorized") {
        PyErr::new::<pyo3::exceptions::PyPermissionError, _>(error_msg)
    } else if error_msg.contains("not found") || error_msg.contains("missing") {
        PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(error_msg)
    } else if error_msg.contains("invalid") || error_msg.contains("malformed") {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(error_msg)
    } else if error_msg.contains("connection") || error_msg.contains("network") {
        PyErr::new::<pyo3::exceptions::PyConnectionError, _>(error_msg)
    } else {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error_msg)
    }
}

/// Create configuration error with field context
pub(crate) fn config_error(field: &str, message: &str) -> PyErr {
    let error = PythonBindingError::Configuration {
        message: message.to_string(),
        field: Some(field.to_string()),
    };
    python_error_to_py_err(error)
}

/// Create validation error with field and value context
pub(crate) fn validation_error(field: &str, value: Option<&str>, message: &str) -> PyErr {
    let error = PythonBindingError::Validation {
        message: message.to_string(),
        field: field.to_string(),
        value: value.map(|v| v.to_string()),
    };
    python_error_to_py_err(error)
}

/// Create timeout error with operation context
pub(crate) fn timeout_error(operation: &str, duration_ms: u64, message: &str) -> PyErr {
    let error = PythonBindingError::Timeout {
        message: message.to_string(),
        operation: operation.to_string(),
        duration_ms,
    };
    python_error_to_py_err(error)
}
