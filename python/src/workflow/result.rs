//! Workflow result for GraphBit Python bindings

use graphbit_core::types::{WorkflowContext, WorkflowState};
use pyo3::prelude::*;
use serde_json;
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

    /// Get a node's output by name or ID
    fn get_node_output(&self, node_name: &str) -> Option<String> {
        self.inner.get_node_output(node_name).and_then(|v| {
            // Handle different JSON value types properly
            match v {
                serde_json::Value::String(s) => Some(s.clone()),
                serde_json::Value::Null => None,
                _ => {
                    // For non-string values, serialize to JSON and then extract the string content
                    match serde_json::to_string(v) {
                        Ok(json_str) => {
                            // If it's a JSON string, try to extract the inner content
                            if json_str.starts_with('"')
                                && json_str.ends_with('"')
                                && json_str.len() > 2
                            {
                                Some(json_str[1..json_str.len() - 1].to_string())
                            } else {
                                Some(json_str)
                            }
                        }
                        Err(_) => Some(format!("{:?}", v)),
                    }
                }
            }
        })
    }

    /// Get all node outputs as a dictionary
    fn get_all_node_outputs(&self) -> HashMap<String, String> {
        self.inner
            .node_outputs
            .iter()
            .filter_map(|(k, v)| {
                // Handle different JSON value types properly
                let value_str = match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Null => return None,
                    _ => {
                        match serde_json::to_string(v) {
                            Ok(json_str) => {
                                // If it's a JSON string, try to extract the inner content
                                if json_str.starts_with('"')
                                    && json_str.ends_with('"')
                                    && json_str.len() > 2
                                {
                                    json_str[1..json_str.len() - 1].to_string()
                                } else {
                                    json_str
                                }
                            }
                            Err(_) => format!("{:?}", v),
                        }
                    }
                };
                Some((k.clone(), value_str))
            })
            .collect()
    }
}
