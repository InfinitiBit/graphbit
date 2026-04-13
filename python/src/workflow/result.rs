//! Workflow result for GraphBit Python bindings

use graphbit_core::types::{WorkflowContext, WorkflowState};
use pyo3::prelude::*;
use serde_json;
use std::collections::HashMap;

/// Result of workflow execution containing output data and execution metadata
#[pyclass]
pub struct WorkflowResult {
    pub(crate) inner: WorkflowContext,
}

/// Sanitize a single tool-call-like object: use parameters_masked as parameters and
/// output_masked as output when present, then remove the _masked keys.
fn sanitize_tool_call_entry(entry: &mut serde_json::Value) {
    if let Some(obj) = entry.as_object_mut() {
        if let Some(pm) = obj.remove("parameters_masked") {
            obj.insert("parameters".to_string(), pm);
        }
        if let Some(om) = obj.remove("output_masked") {
            obj.insert("output".to_string(), om);
        }
    }
}

/// Sanitize a node so that tool call parameters and output exposed in metadata are masked.
/// Applies to executions[].type=="tool_call", node["tool_calls"], and node["initial_response"]["tool_calls"].
fn sanitize_node_metadata(node: &mut serde_json::Value) {
    let obj = match node.as_object_mut() {
        Some(o) => o,
        None => return,
    };
    // Sanitize executions array
    if let Some(executions) = obj.get_mut("executions") {
        if let Some(execs_arr) = executions.as_array_mut() {
            for entry in execs_arr.iter_mut() {
                if entry.get("type").and_then(|t| t.as_str()) == Some("tool_call") {
                    sanitize_tool_call_entry(entry);
                }
            }
        }
    }
    // Sanitize top-level tool_calls if present (e.g. flattened view)
    if let Some(tool_calls) = obj.get_mut("tool_calls") {
        if let Some(arr) = tool_calls.as_array_mut() {
            for entry in arr.iter_mut() {
                sanitize_tool_call_entry(entry);
            }
        }
    }
    // Sanitize initial_response.tool_calls if present
    if let Some(initial_response) = obj.get_mut("initial_response") {
        if let Some(ir_obj) = initial_response.as_object_mut() {
            if let Some(tc) = ir_obj.get_mut("tool_calls") {
                if let Some(arr) = tc.as_array_mut() {
                    for entry in arr.iter_mut() {
                        sanitize_tool_call_entry(entry);
                    }
                }
            }
        }
    }
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

    /// Get node execution metadata for a specific node
    ///
    /// Returns the full node-level metadata object containing:
    /// - node_id, node_name, node_type, user_input, final_output
    /// - tools_available, total_tools_available
    /// - start_time, end_time, duration_ms, success, error
    /// - total_iterations, max_iterations, exit_reason
    /// - total_usage (aggregated token usage)
    /// - total_tool_calls, total_retries, tools_used
    /// - executions: chronological array of llm_call, tool_call, guardrail_policy entries
    ///
    /// # Arguments
    /// * `node_id` - Node ID or node name
    ///
    /// # Returns
    /// Dictionary with node execution metadata, or None if not found
    fn get_node_response_metadata(
        &self,
        py: Python<'_>,
        node_id: &str,
    ) -> PyResult<Option<PyObject>> {
        let key = format!("node_response_{}", node_id);
        match self.inner.metadata.get(&key) {
            Some(value) => {
                let mut node = value.clone();
                sanitize_node_metadata(&mut node);
                let py_obj = pythonize::pythonize(py, &node)?;
                Ok(Some(py_obj.into()))
            }
            None => Ok(None),
        }
    }

    /// Get complete workflow execution metadata
    ///
    /// Returns the full workflow-level schema containing:
    /// - workflow_id, workflow_name
    /// - start_time, end_time, duration_ms
    /// - user_input, final_output (from first/last nodes)
    /// - workflow_state: completed/failed/cancelled/paused
    /// - nodes: array of per-node metadata objects (each with executions array)
    /// - total_usage: aggregated token usage across all nodes
    /// - total_tool_calls: sum of tool calls across all nodes
    ///
    /// # Returns
    /// Dictionary with the complete workflow-level metadata
    fn get_all_node_response_metadata(&self, py: Python<'_>) -> PyResult<PyObject> {
        // Collect node metadata entries (by node ID only, skip name duplicates)
        let mut nodes: Vec<serde_json::Value> = Vec::new();
        let mut seen_node_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

        for (k, v) in self.inner.metadata.iter() {
            if let Some(node_id) = k.strip_prefix("node_response_") {
                // Skip if this is a name-based duplicate (node names are typically not UUIDs)
                // We include if the node_id is a UUID format or if the value has a node_id field matching
                if let Some(stored_node_id) = v.get("node_id").and_then(|v| v.as_str()) {
                    if seen_node_ids.contains(stored_node_id) {
                        continue;
                    }
                    seen_node_ids.insert(stored_node_id.to_string());
                } else if seen_node_ids.contains(node_id) {
                    continue;
                } else {
                    seen_node_ids.insert(node_id.to_string());
                }
                // Sanitize node: expose parameters_masked as parameters and output_masked as output
                // so the returned metadata does not leak real PII (executions, tool_calls, initial_response.tool_calls).
                let mut node = v.clone();
                sanitize_node_metadata(&mut node);
                nodes.push(node);
            }
        }

        // Sort nodes chronologically by start_time
        nodes.sort_by(|a, b| {
            let a_start = a.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
            let b_start = b.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
            a_start.cmp(b_start)
        });

        // Aggregate total_usage and total_tool_calls across all nodes
        let mut total_prompt_tokens: u64 = 0;
        let mut total_completion_tokens: u64 = 0;
        let mut total_tokens: u64 = 0;
        let mut total_cached_tokens: u64 = 0;
        let mut total_cache_creation_tokens: u64 = 0;
        let mut total_tool_calls: u64 = 0;

        for node in &nodes {
            if let Some(usage) = node.get("total_usage") {
                total_prompt_tokens += usage
                    .get("prompt_tokens")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                total_completion_tokens += usage
                    .get("completion_tokens")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                total_tokens += usage
                    .get("total_tokens")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                if let Some(details) = usage.get("prompt_tokens_details") {
                    total_cached_tokens += details
                        .get("cached_tokens")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    total_cache_creation_tokens += details
                        .get("cache_creation_tokens")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                }
            }
            total_tool_calls += node
                .get("total_tool_calls")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
        }

        // Determine user_input (from first node) and final_output (from last node)
        let user_input = nodes
            .first()
            .and_then(|n| n.get("user_input"))
            .cloned()
            .unwrap_or(serde_json::Value::String(String::new()));
        let final_output = nodes
            .last()
            .and_then(|n| n.get("final_output"))
            .cloned()
            .unwrap_or(serde_json::Value::String(String::new()));

        // Determine workflow_state from context state
        let workflow_state = match &self.inner.state {
            WorkflowState::Completed => "completed",
            WorkflowState::Failed { .. } => "failed",
            WorkflowState::Cancelled => "cancelled",
            WorkflowState::Paused { .. } => "paused",
            WorkflowState::Running { .. } => "running",
            WorkflowState::Pending => "pending",
        };

        // Get workflow name from metadata (stored during execution)
        let workflow_name = self
            .inner
            .metadata
            .get("workflow_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Build timing fields
        let start_time = self.inner.started_at.to_rfc3339();
        let end_time = self
            .inner
            .completed_at
            .map(|t| t.to_rfc3339())
            .unwrap_or_default();
        let duration_ms = self.inner.execution_duration_ms().unwrap_or(0) as f64;

        // Build the workflow-level metadata object
        let workflow_metadata = serde_json::json!({
            "workflow_id": self.inner.workflow_id.to_string(),
            "workflow_name": workflow_name,
            "start_time": start_time,
            "end_time": end_time,
            "duration_ms": duration_ms,
            "user_input": user_input,
            // TODO: Remove these placeholder fields in a future release
            "user_input_masked": "",
            "final_output": final_output,
            "final_output_masked": "",
            "workflow_state": workflow_state,
            "nodes": nodes,
            "total_usage": {
                "prompt_tokens": total_prompt_tokens,
                "completion_tokens": total_completion_tokens,
                "total_tokens": total_tokens,
                "prompt_tokens_details": {
                    "cached_tokens": total_cached_tokens,
                    "cache_creation_tokens": total_cache_creation_tokens,
                    "audio_tokens": 0
                },
                "completion_tokens_details": {
                    "reasoning_tokens": 0,
                    "audio_tokens": 0,
                    "accepted_prediction_tokens": 0,
                    "rejected_prediction_tokens": 0
                }
            },
            "total_tool_calls": total_tool_calls
        });

        let py_obj = pythonize::pythonize(py, &workflow_metadata)?;
        Ok(py_obj.into())
    }
}
