//! Workflow result for GraphBit Python bindings

use graphbit_core::types::{WorkflowContext, WorkflowState};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct WorkflowResult {
    pub(crate) inner: WorkflowContext,
}

impl WorkflowResult {
    /// Create a new workflow result
    pub fn new(context: WorkflowContext) -> Self {
        Self { inner: context }
    }
}

#[pymethods]
impl WorkflowResult {
    fn is_success(&self) -> bool {
        matches!(self.inner.state, WorkflowState::Completed)
    }

    fn is_failed(&self) -> bool {
        matches!(self.inner.state, WorkflowState::Failed { .. })
    }

    fn state(&self) -> String {
        format!("{:?}", self.inner.state)
    }

    fn execution_time_ms(&self) -> u64 {
        // Use the built-in execution duration calculation
        self.inner.execution_duration_ms().unwrap_or(0)
    }

    fn variables(&self) -> Vec<(String, String)> {
        self.inner
            .variables
            .iter()
            .map(|(k, v)| {
                if let Ok(s) = serde_json::to_string(v) {
                    (k.clone(), s.trim_matches('"').to_string())
                } else {
                    (k.clone(), v.to_string())
                }
            })
            .collect()
    }

    fn get_variable(&self, key: &str) -> Option<String> {
        self.inner
            .get_variable(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn get_all_variables(&self) -> HashMap<String, String> {
        self.inner
            .variables
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect()
    }
}
