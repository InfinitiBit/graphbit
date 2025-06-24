//! Fast error handling for GraphBit Python bindings

use pyo3::prelude::*;

#[derive(Debug)]
pub enum FastError {
    Network,
    Auth, 
    RateLimit,
    Invalid,
    Timeout,
    Other(String),
}

impl FastError {
    pub fn to_py_err(self) -> PyErr {
        match self {
            FastError::Network => PyErr::new::<pyo3::exceptions::PyConnectionError, _>("Network error"),
            FastError::Auth => PyErr::new::<pyo3::exceptions::PyPermissionError, _>("Auth error"),
            FastError::RateLimit => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Rate limit"),
            FastError::Invalid => PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid request"),
            FastError::Timeout => PyErr::new::<pyo3::exceptions::PyTimeoutError, _>("Timeout"),
            FastError::Other(msg) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg),
        }
    }

    pub fn from_graphbit_error(error: &graphbit_core::errors::GraphBitError) -> Self {
        let error_str = error.to_string().to_lowercase();
        
        if error_str.contains("network") || error_str.contains("connection") {
            FastError::Network
        } else if error_str.contains("auth") || error_str.contains("unauthorized") {
            FastError::Auth
        } else if error_str.contains("rate limit") {
            FastError::RateLimit
        } else if error_str.contains("invalid") {
            FastError::Invalid
        } else if error_str.contains("timeout") {
            FastError::Timeout
        } else {
            FastError::Other(error.to_string())
        }
    }
} 