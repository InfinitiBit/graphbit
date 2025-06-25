//! Workflow result for GraphBit Python bindings

use graphbit_core::types::{WorkflowContext, WorkflowState};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct WorkflowResult {
    pub(crate) inner: WorkflowContext,
}

#[pymethods]
impl WorkflowResult {
    fn is_success(&self) -> bool {
        matches!(self.inner.state, WorkflowState::Completed)
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
