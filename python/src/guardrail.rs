//! GuardRail policy config exposed to Python as `GuardRailPolicyConfig`.
//! Used with `Executor.execute(workflow, policy=...)` for PII masking/mapping.

use graphbit_core::GuardRailConfig;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::sync::Arc;

/// Python-facing guardrail policy configuration.
///
/// Create via `GuardRailPolicyConfig.from_json(...)`, `from_file(...)`, or `from_url(...)`,
/// then pass to `executor.execute(workflow, policy=config)`.
#[pyclass]
#[derive(Clone)]
pub struct GuardRailPolicyConfig {
    pub(crate) inner: Arc<GuardRailConfig>,
}

#[pymethods]
impl GuardRailPolicyConfig {
    /// Create a config from a JSON string.
    ///
    /// # Errors
    /// Raises `ValueError` if the JSON is invalid or validation fails.
    #[staticmethod]
    pub fn from_json(json_str: &str) -> PyResult<Self> {
        let config = GuardRailConfig::new(json_str)
            .map_err(|e| PyValueError::new_err(format!("GuardRail config error: {}", e)))?;
        Ok(Self {
            inner: Arc::new(config),
        })
    }

    /// Create a config from a local file path.
    ///
    /// # Errors
    /// Raises `ValueError` if the file cannot be read or validation fails.
    #[staticmethod]
    pub fn from_file(path: &str) -> PyResult<Self> {
        let config = GuardRailConfig::from_file(std::path::Path::new(path))
            .map_err(|e| PyValueError::new_err(format!("GuardRail config error: {}", e)))?;
        Ok(Self {
            inner: Arc::new(config),
        })
    }

    /// Create a config from a remote URL (HTTP GET).
    ///
    /// # Errors
    /// Raises `ValueError` if the URL cannot be fetched or validation fails.
    #[staticmethod]
    pub fn from_url(url: &str) -> PyResult<Self> {
        let config = GuardRailConfig::from_url(url)
            .map_err(|e| PyValueError::new_err(format!("GuardRail config error: {}", e)))?;
        Ok(Self {
            inner: Arc::new(config),
        })
    }

    /// Default (inactive) policy — no masking or mapping.
    #[staticmethod]
    pub fn default_config() -> Self {
        Self {
            inner: Arc::new(GuardRailConfig::default_config()),
        }
    }

    /// Policy name.
    pub fn policy_name(&self) -> String {
        self.inner.policy_name()
    }

    /// Policy version.
    pub fn policy_version(&self) -> String {
        self.inner.policy_version()
    }

    /// Whether the policy is active.
    pub fn is_active(&self) -> bool {
        self.inner.active()
    }
}

impl GuardRailPolicyConfig {
    /// Return the inner config for use by the executor (same-crate only). Not exposed to Python.
    #[inline]
    pub(crate) fn get_inner(&self) -> Arc<GuardRailConfig> {
        Arc::clone(&self.inner)
    }
}
