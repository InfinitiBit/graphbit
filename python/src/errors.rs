//! Simplified error handling for GraphBit Python bindings

use pyo3::prelude::*;

/// Convert GraphBit error directly to appropriate Python exception
pub fn to_py_error(error: graphbit_core::errors::GraphBitError) -> PyErr {
    use graphbit_core::errors::GraphBitError;

    match error {
        GraphBitError::Network { .. } => {
            PyErr::new::<pyo3::exceptions::PyConnectionError, _>(error.to_string())
        }
        GraphBitError::Authentication { .. } => {
            PyErr::new::<pyo3::exceptions::PyPermissionError, _>(error.to_string())
        }
        GraphBitError::RateLimit { .. } => {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.to_string())
        }
        GraphBitError::Validation { .. } => {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(error.to_string())
        }
        GraphBitError::Configuration { .. } => {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(error.to_string())
        }
        _ => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.to_string()),
    }
}

/// Utility function to convert any error to PyErr
pub fn to_py_runtime_error<E: std::fmt::Display>(error: E) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error.to_string())
}
