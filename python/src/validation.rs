//! Validation utilities for GraphBit Python bindings

use pyo3::prelude::*;

/// Validate API key for different providers
pub(crate) fn validate_api_key(api_key: &str, provider: &str) -> PyResult<()> {
    if api_key.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "{} API key cannot be empty",
            provider
        )));
    }

    let min_length = match provider.to_lowercase().as_str() {
        "openai" => 20,
        "anthropic" => 15,
        "huggingface" => 10,
        _ => 8,
    };

    if api_key.len() < min_length {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "{} API key too short",
            provider
        )));
    }

    Ok(())
}

/// Validate Cloudflare account ID
pub(crate) fn validate_cloudflare_account_id(account_id: &str) -> PyResult<()> {
    if account_id.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Cloudflare account ID cannot be empty"
        ));
    }

    if account_id.len() < 32 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Invalid Cloudflare account ID format"
        ));
    }

    Ok(())
}
