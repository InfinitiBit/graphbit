//! Production-grade workflow executor for GraphBit Python bindings
//!
//! This module provides a robust, high-performance workflow executor with:
//! - Comprehensive input validation
//! - Configurable execution modes and timeouts
//! - Resource monitoring and management
//! - Detailed execution metrics and logging
//! - Graceful error handling and recovery

use graphbit_core::workflow::WorkflowExecutor as CoreWorkflowExecutor;
use graphbit_core::{DecodeContext, EncodeContext, Enforcer, GuardRail};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};

use super::{result::WorkflowResult, workflow::Workflow};
use crate::errors::{timeout_error, to_py_runtime_error, validation_error};
use crate::guardrail::GuardRailPolicyConfig;
use crate::llm::config::LlmConfig;
use crate::runtime::get_runtime;

/// Execution mode for different performance characteristics
#[derive(Debug, Clone, Copy)]
pub(crate) enum ExecutionMode {
    /// Balanced mode for general use
    Balanced,
}

/// Execution configuration for fine-tuning performance
#[derive(Debug, Clone)]
pub(crate) struct ExecutionConfig {
    /// Execution mode
    pub mode: ExecutionMode,
    /// Request timeout in seconds
    pub timeout: Duration,
    /// Maximum retries for failed operations
    pub max_retries: u32,
    /// Enable detailed execution metrics
    pub enable_metrics: bool,
    /// Enable execution tracing
    pub enable_tracing: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            mode: ExecutionMode::Balanced,
            timeout: Duration::from_secs(300), // 5 minutes
            max_retries: 3,
            enable_metrics: true,
            enable_tracing: false, // Default to false to reduce debug output
        }
    }
}

/// Execution statistics for monitoring
#[derive(Debug, Clone)]
pub(crate) struct ExecutionStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_duration_ms: f64,
    pub total_duration_ms: u64,
    pub created_at: Instant,
}

impl Default for ExecutionStats {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_duration_ms: 0.0,
            total_duration_ms: 0,
            created_at: Instant::now(),
        }
    }
}

/// Production-grade workflow executor with comprehensive features
#[pyclass]
pub struct Executor {
    /// Execution configuration
    config: ExecutionConfig,
    /// LLM configuration for auto-generating agents
    llm_config: LlmConfig,
    /// Execution statistics
    stats: ExecutionStats,
}

#[pymethods]
impl Executor {
    #[new]
    #[pyo3(signature = (config, lightweight_mode=None, timeout_seconds=None, debug=None))]
    #[allow(unused_variables)]
    fn new(
        config: LlmConfig,
        lightweight_mode: Option<bool>,
        timeout_seconds: Option<u64>,
        debug: Option<bool>,
    ) -> PyResult<Self> {
        // Validate inputs
        if let Some(timeout) = timeout_seconds {
            if timeout == 0 || timeout > 3600 {
                return Err(validation_error(
                    "timeout_seconds",
                    Some(&timeout.to_string()),
                    "Timeout must be between 1 and 3600 seconds",
                ));
            }
        }

        let mut exec_config = ExecutionConfig::default();

        // Set timeout if specified
        if let Some(timeout) = timeout_seconds {
            exec_config.timeout = Duration::from_secs(timeout);
        }

        // Set debug mode - defaults to false
        exec_config.enable_tracing = debug.unwrap_or(false);

        if exec_config.enable_tracing {
            info!(
                "Created executor with mode: {:?}, timeout: {:?}",
                exec_config.mode, exec_config.timeout
            );
        }

        Ok(Self {
            config: exec_config,
            llm_config: config,
            stats: ExecutionStats::default(),
        })
    }

    /// Execute a workflow with comprehensive error handling and monitoring.
    ///
    /// `policy` is optional. When provided: encode before every LLM call, decode after every LLM call;
    /// before tool usage decode (so tools see real PII); after tool usage do nothing (no encode).
    #[instrument(skip(self, py, workflow, policy), fields(workflow_name = %workflow.inner.name))]
    #[pyo3(signature = (workflow, policy=None))]
    fn execute(
        &mut self,
        py: Python<'_>,
        workflow: &Workflow,
        policy: Option<&Bound<'_, GuardRailPolicyConfig>>,
    ) -> PyResult<WorkflowResult> {
        let start_time = Instant::now();

        // Validate workflow
        if workflow.inner.graph.node_count() == 0 {
            return Err(validation_error(
                "workflow",
                None,
                "Workflow cannot be empty",
            ));
        }

        // Validate the workflow structure
        if let Err(e) = workflow.inner.validate() {
            return Err(validation_error(
                "workflow",
                None,
                &format!("Invalid workflow: {}", e),
            ));
        }

        let llm_config = self.llm_config.inner.clone();
        let workflow_clone = workflow.inner.clone();
        let config = self.config.clone();
        let timeout_duration = config.timeout;
        let debug = config.enable_tracing; // Capture debug flag

        // Build optional guardrail enforcer from policy (for encode/decode at LLM and tool boundaries)
        let guardrail_enforcer = policy.map(|p| {
            let config = p.borrow().get_inner();
            Arc::new(GuardRail::enforcer_for(
                config,
                workflow_clone.id.to_string(),
            ))
        });

        if debug {
            debug!("Starting workflow execution with mode: {:?}", config.mode);
        }

        // Release the GIL before entering the async runtime to prevent deadlocks
        // when the async code needs to call back into Python
        let result = py.allow_threads(|| {
            get_runtime().block_on(async move {
                // Apply timeout to the entire execution
                tokio::time::timeout(timeout_duration, async move {
                    Self::execute_workflow_internal(
                        llm_config,
                        workflow_clone,
                        config,
                        guardrail_enforcer,
                    )
                    .await
                })
                .await
            })
        });

        let duration = start_time.elapsed();
        self.update_stats(result.is_ok(), duration);

        match result {
            Ok(Ok(workflow_result)) => {
                if debug {
                    info!(
                        "Workflow execution completed successfully in {:?}",
                        duration
                    );
                }
                Ok(WorkflowResult::new(workflow_result))
            }
            Ok(Err(e)) => {
                if debug {
                    error!("Workflow execution failed: {}", e);
                }
                Err(to_py_runtime_error(e))
            }
            Err(_) => {
                if debug {
                    error!("Workflow execution timed out after {:?}", duration);
                }
                Err(timeout_error(
                    "workflow_execution",
                    duration.as_millis() as u64,
                    &format!("Workflow execution timed out after {:?}", timeout_duration),
                ))
            }
        }
    }

    /// Async execution with enhanced performance optimizations
    #[instrument(skip(self, workflow, py, policy), fields(workflow_name = %workflow.inner.name))]
    #[pyo3(signature = (workflow, policy=None))]
    fn run_async<'a>(
        &mut self,
        workflow: &Workflow,
        py: Python<'a>,
        policy: Option<&Bound<'_, GuardRailPolicyConfig>>,
    ) -> PyResult<Bound<'a, PyAny>> {
        // Validate workflow
        if let Err(e) = workflow.inner.validate() {
            return Err(validation_error(
                "workflow",
                None,
                &format!("Invalid workflow: {}", e),
            ));
        }

        let workflow_clone = workflow.inner.clone();
        let llm_config = self.llm_config.inner.clone();
        let config = self.config.clone();
        let timeout_duration = config.timeout;
        let start_time = Instant::now();
        let debug = config.enable_tracing;
        let guardrail_enforcer = policy.map(|p| {
            let config = p.borrow().get_inner();
            Arc::new(GuardRail::enforcer_for(
                config,
                workflow_clone.id.to_string(),
            ))
        });

        if debug {
            debug!(
                "Starting async workflow execution with mode: {:?}",
                config.mode
            );
        }

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = tokio::time::timeout(timeout_duration, async move {
                Self::execute_workflow_internal(
                    llm_config,
                    workflow_clone,
                    config,
                    guardrail_enforcer,
                )
                .await
            })
            .await;

            match result {
                Ok(Ok(workflow_result)) => {
                    let duration = start_time.elapsed();
                    if debug {
                        info!(
                            "Async workflow execution completed successfully in {:?}",
                            duration
                        );
                    }
                    Ok(WorkflowResult {
                        inner: workflow_result,
                    })
                }
                Ok(Err(e)) => {
                    let duration = start_time.elapsed();
                    if debug {
                        error!(
                            "Async workflow execution failed after {:?}: {}",
                            duration, e
                        );
                    }
                    Err(to_py_runtime_error(e))
                }
                Err(_) => {
                    let duration = start_time.elapsed();
                    if debug {
                        error!("Async workflow execution timed out after {:?}", duration);
                    }
                    Err(timeout_error(
                        "async_workflow_execution",
                        duration.as_millis() as u64,
                        &format!(
                            "Async workflow execution timed out after {:?}",
                            timeout_duration
                        ),
                    ))
                }
            }
        })
    }

    /// Configure the executor with new settings
    #[pyo3(signature = (timeout_seconds=None, max_retries=None, enable_metrics=None, debug=None))]
    fn configure(
        &mut self,
        timeout_seconds: Option<u64>,
        max_retries: Option<u32>,
        enable_metrics: Option<bool>,
        debug: Option<bool>,
    ) -> PyResult<()> {
        // Validate timeout
        if let Some(timeout) = timeout_seconds {
            if timeout == 0 || timeout > 3600 {
                return Err(validation_error(
                    "timeout_seconds",
                    Some(&timeout.to_string()),
                    "Timeout must be between 1 and 3600 seconds",
                ));
            }
            self.config.timeout = Duration::from_secs(timeout);
        }

        // Validate retries
        if let Some(retries) = max_retries {
            if retries == 0 || retries > 10 {
                return Err(validation_error(
                    "max_retries",
                    Some(&retries.to_string()),
                    "Maximum retries must be between 1 and 10",
                ));
            }
            self.config.max_retries = retries;
        }

        if let Some(metrics) = enable_metrics {
            self.config.enable_metrics = metrics;
        }

        if let Some(debug_mode) = debug {
            self.config.enable_tracing = debug_mode;
        }

        if self.config.enable_tracing {
            info!(
                "Executor configuration updated: timeout={:?}, retries={}, metrics={}, debug={}",
                self.config.timeout,
                self.config.max_retries,
                self.config.enable_metrics,
                self.config.enable_tracing
            );
        }

        Ok(())
    }

    /// Get comprehensive execution statistics
    fn get_stats<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);

        dict.set_item("total_executions", self.stats.total_executions)?;
        dict.set_item("successful_executions", self.stats.successful_executions)?;
        dict.set_item("failed_executions", self.stats.failed_executions)?;
        dict.set_item(
            "success_rate",
            if self.stats.total_executions > 0 {
                self.stats.successful_executions as f64 / self.stats.total_executions as f64
            } else {
                0.0
            },
        )?;
        dict.set_item("average_duration_ms", self.stats.average_duration_ms)?;
        dict.set_item("total_duration_ms", self.stats.total_duration_ms)?;
        dict.set_item("uptime_seconds", self.stats.created_at.elapsed().as_secs())?;

        // Configuration info
        dict.set_item("execution_mode", format!("{:?}", self.config.mode))?;
        dict.set_item("timeout_seconds", self.config.timeout.as_secs())?;
        dict.set_item("max_retries", self.config.max_retries)?;
        dict.set_item("metrics_enabled", self.config.enable_metrics)?;

        Ok(dict)
    }

    /// Reset execution statistics
    fn reset_stats(&mut self) -> PyResult<()> {
        self.stats = ExecutionStats::default();
        if self.config.enable_tracing {
            info!("Execution statistics reset");
        }
        Ok(())
    }

    /// Check execution mode
    fn get_execution_mode(&self) -> String {
        format!("{:?}", self.config.mode)
    }
}

impl Executor {
    /// Internal workflow execution with mode-specific optimizations and tool call handling.
    /// When `guardrail_enforcer` is `Some`, the core encodes before LLM and decodes after LLM;
    /// we decode before tool usage only (no encode after tool).
    async fn execute_workflow_internal(
        llm_config: graphbit_core::llm::LlmConfig,
        workflow: graphbit_core::workflow::Workflow,
        config: ExecutionConfig,
        guardrail_enforcer: Option<Arc<Enforcer>>,
    ) -> Result<graphbit_core::types::WorkflowContext, graphbit_core::errors::GraphBitError> {
        let conditional_handlers =
            crate::workflow::node::build_core_conditional_handlers(&workflow)?;
        let executor = match config.mode {
            ExecutionMode::Balanced => {
                CoreWorkflowExecutor::new()
                    .with_default_llm_config(llm_config.clone())
                    .with_conditional_handlers(conditional_handlers)
            }
        };

        // Execute the workflow (core applies encode before LLM, decode after LLM when enforcer is Some)
        let mut context = executor
            .execute(workflow.clone(), guardrail_enforcer.clone())
            .await?;

        // Store LLM config in context metadata for tool call handling
        if let Ok(llm_config_json) = serde_json::to_value(&llm_config) {
            context
                .metadata
                .insert("llm_config".to_string(), llm_config_json);
        }

        // Store workflow name in context metadata for result schema
        context.metadata.insert(
            "workflow_name".to_string(),
            serde_json::Value::String(workflow.name.clone()),
        );

        // Check if any node outputs contain tool_calls_required responses and handle them
        let mut context = context;
        let mut rerun_attempts = 0;
        loop {
            let (ctx, nodes_with_tool_calls) = Self::handle_tool_calls_in_context(
                context,
                &workflow,
                guardrail_enforcer.as_ref().map(|arc| arc.as_ref()),
            )
            .await?;
            context = ctx;

            if nodes_with_tool_calls.is_empty() {
                break;
            }

            // Identify downstream nodes that depend on tool-resolved outputs and need rerun.
            // This includes nodes that themselves may have tool calls, as they can still depend on
            // upstream nodes whose outputs were just resolved.
            let mut downstream_nodes: HashSet<String> = HashSet::new();
            if let Some(deps_obj) = context.metadata.get("node_dependencies").and_then(|v| v.as_object()) {
                let mut queue: Vec<String> = nodes_with_tool_calls.clone();
                while let Some(parent_id) = queue.pop() {
                    for (node_id, parents) in deps_obj.iter() {
                        if downstream_nodes.contains(node_id) {
                            continue;
                        }
                        if let Some(parent_array) = parents.as_array() {
                            if parent_array.iter().any(|p| p.as_str() == Some(&parent_id)) {
                                downstream_nodes.insert(node_id.clone());
                                queue.push(node_id.clone());
                            }
                        }
                    }
                }
            }

            if downstream_nodes.is_empty() {
                break;
            }

            tracing::info!(
                "Rerunning downstream nodes after tool resolution: {:?}",
                downstream_nodes
            );

            // Clear outputs and metadata for downstream nodes so they will re-execute
            if let Some(id_name_map) = context
                .metadata
                .get("node_id_to_name")
                .and_then(|v| v.as_object())
                .cloned()
            {
                for node_id in &downstream_nodes {
                    context.node_outputs.remove(node_id);
                    context.variables.remove(node_id);
                    context.metadata.remove(&format!("node_response_{}", node_id));

                    if let Some(node_name_value) = id_name_map
                        .get(node_id)
                        .and_then(|v| v.as_str())
                    {
                        context.node_outputs.remove(node_name_value);
                        context.variables.remove(node_name_value);
                        context.metadata.remove(&format!("node_response_{}", node_name_value));
                    }
                }
            }

            let conditional_handlers = crate::workflow::node::build_core_conditional_handlers(&workflow)?;
            let executor_clone = CoreWorkflowExecutor::new()
                .with_default_llm_config(llm_config.clone())
                .with_conditional_handlers(conditional_handlers);

            context = executor_clone
                .execute_with_context(workflow.clone(), guardrail_enforcer.clone(), context)
                .await?;

            rerun_attempts += 1;
            if rerun_attempts >= 3 {
                tracing::warn!(
                    "Maximum rerun attempts reached while resolving tool-dependent nodes"
                );
                break;
            }
        }

        Ok(context)
    }

    /// Handle tool calls in workflow context using an iterative ReAct loop.
    ///
    /// When an agent node returns `tool_calls_required`, this function:
    /// 1. Executes the requested tools via Python
    /// 2. Sends tool results back to the LLM WITH tool definitions
    /// 3. If the LLM requests more tools, repeats from step 1
    /// 4. Exits when the LLM returns a final answer (no tool calls) or max_iterations is reached
    ///
    /// This enables multi-step reasoning where the agent can chain dependent tool calls
    /// across multiple iterations (e.g., "add 2+3, then multiply the result by 4").
    /// Handle tool calls in workflow context by executing them and updating the context.
    /// When `guardrail_enforcer` is `Some`, decodes tool-call parameters before execution only;
    /// after tool execution we do nothing (no encode of tool results).
    async fn handle_tool_calls_in_context(
        mut context: graphbit_core::types::WorkflowContext,
        workflow: &graphbit_core::workflow::Workflow,
        guardrail_enforcer: Option<&Enforcer>,
    ) -> Result<(graphbit_core::types::WorkflowContext, Vec<String>), graphbit_core::errors::GraphBitError> {
        use crate::workflow::node::execute_production_tool_calls;
        use graphbit_core::llm::{LlmMessage, LlmProvider, LlmRequest, LlmTool, LlmToolCall};

        // Check each node output for tool_calls_required responses
        let node_outputs = context.node_outputs.clone();
        let mut nodes_with_tool_calls: Vec<String> = Vec::new();

        for (node_id, output) in node_outputs {
            if let Ok(response_obj) = serde_json::from_value::<serde_json::Value>(output.clone()) {
                if let Some(response_type) = response_obj.get("type").and_then(|v| v.as_str()) {
                    if response_type == "tool_calls_required" {
                        // Extract initial tool calls and original prompt
                        if let (Some(initial_tool_calls), Some(original_prompt)) = (
                            response_obj.get("tool_calls"),
                            response_obj.get("original_prompt").and_then(|v| v.as_str()),
                        ) {
                            // Get the node from the workflow
                            let node = match workflow
                                .graph
                                .get_nodes()
                                .iter()
                                .find(|(id, _)| id.to_string() == node_id)
                                .map(|(_, node)| node.clone())
                            {
                                Some(n) => n,
                                None => continue,
                            };

                            // Only handle agent nodes
                            if !matches!(
                                node.node_type,
                                graphbit_core::graph::NodeType::Agent { .. }
                            ) {
                                continue;
                            }

                            nodes_with_tool_calls.push(node.id.to_string());

                            // Get node name for metadata storage
                            let node_name = workflow
                                .graph
                                .get_nodes()
                                .iter()
                                .find(|(id, _)| **id == node.id)
                                .map(|(_, n)| n.name.clone())
                                .unwrap_or_else(|| "unknown".to_string());

                            // Extract available tool names for this node
                            let node_tools = node
                                .config
                                .get("tools")
                                .and_then(|v| v.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect::<Vec<String>>()
                                })
                                .unwrap_or_default();

                            // Extract LlmTool definitions from node config for subsequent LLM calls
                            let llm_tools: Vec<LlmTool> = node
                                .config
                                .get("tool_schemas")
                                .and_then(|v| v.as_array())
                                .map(|schemas| {
                                    schemas
                                        .iter()
                                        .filter_map(|schema| {
                                            let name = schema.get("name")?.as_str()?;
                                            let description =
                                                schema.get("description")?.as_str()?;
                                            let parameters = schema.get("parameters")?;
                                            Some(LlmTool::new(
                                                name,
                                                description,
                                                parameters.clone(),
                                            ))
                                        })
                                        .collect()
                                })
                                .unwrap_or_default();

                            // Get max_iterations from node config (default: 10)
                            let max_iterations =
                                node.config
                                    .get("max_iterations")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(10) as usize;

                            // Get LLM config for subsequent calls
                            let llm_config = context.metadata.get("llm_config").and_then(|v| {
                                serde_json::from_value::<graphbit_core::llm::LlmConfig>(v.clone())
                                    .ok()
                            });

                            let llm_config = match llm_config {
                                Some(cfg) => cfg,
                                None => {
                                    tracing::warn!(
                                        "No LLM configuration found in context metadata for iterative tool loop."
                                    );
                                    continue;
                                }
                            };

                            let llm_provider =
                                match graphbit_core::llm::LlmProviderFactory::create_provider(
                                    llm_config.clone(),
                                ) {
                                    Ok(provider_trait) => {
                                        LlmProvider::new(provider_trait, llm_config.clone())
                                    }
                                    Err(e) => {
                                        tracing::error!(
                                            "Failed to create LLM provider for iterative loop: {}",
                                            e
                                        );
                                        continue;
                                    }
                                };

                            // Get the initial assistant content from the tool_calls_required response
                            let initial_content = response_obj
                                .get("content")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();

                            // ============================================================
                            // ITERATIVE REACT LOOP
                            // ============================================================

                            // Build message history starting with the original user prompt
                            let mut messages: Vec<LlmMessage> =
                                vec![LlmMessage::user(original_prompt)];

                            // Parse initial tool calls from the first LLM response
                            let mut current_tool_calls: Vec<serde_json::Value> =
                                initial_tool_calls.as_array().cloned().unwrap_or_default();

                            // Current assistant content
                            let mut current_content = initial_content.clone();

                            // Tracking for observability
                            let mut all_tool_executions: Vec<serde_json::Value> = Vec::new();
                            let mut iteration: usize = 0;
                            let mut llm_calls_in_loop: usize = 0;
                            let mut final_content = current_content.clone();
                            let mut final_finish_reason = "tool_calls_required".to_string();
                            let mut final_raw_output_for_meta = final_content.clone();
                            let overall_start = std::time::Instant::now();
                            let existing_node_metadata = context
                                .metadata
                                .get(&format!("node_response_{}", node.id))
                                .cloned();
                            let mut executions: Vec<serde_json::Value> = existing_node_metadata
                                .as_ref()
                                .and_then(|m| m.get("executions"))
                                .and_then(|e| e.as_array())
                                .cloned()
                                .unwrap_or_default();
                            let mut tools_used: Vec<String> = Vec::new();

                            loop {
                                // Safety check: max iterations (check BEFORE incrementing)
                                if iteration >= max_iterations {
                                    tracing::warn!(
                                        "Agent reached max iterations ({}) for node '{}'. Using last response as final answer.",
                                        max_iterations,
                                        node_name
                                    );
                                    final_content = current_content.clone();
                                    break;
                                }

                                iteration += 1;

                                tracing::info!(
                                    "Agent loop iteration {} / {} - processing {} tool call(s) for node '{}'",
                                    iteration,
                                    max_iterations,
                                    current_tool_calls.len(),
                                    node_name
                                );

                                // ---- Step 1: Append assistant message with tool calls to history ----
                                let assistant_tool_calls: Vec<LlmToolCall> = current_tool_calls
                                    .iter()
                                    .filter_map(|tc| {
                                        let id = tc
                                            .get("id")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        let name =
                                            tc.get("name").and_then(|v| v.as_str())?.to_string();
                                        let parameters = tc
                                            .get("parameters")
                                            .cloned()
                                            .unwrap_or(serde_json::json!({}));
                                        Some(LlmToolCall {
                                            id,
                                            name,
                                            parameters,
                                        })
                                    })
                                    .collect();

                                messages.push(
                                    LlmMessage::assistant(&current_content)
                                        .with_tool_calls(assistant_tool_calls.clone()),
                                );

                                // ---- Step 2: Execute tools via Python ----
                                let python_tool_calls: Vec<serde_json::Value> = current_tool_calls
                                    .iter()
                                    .map(|tc| {
                                        let name = tc
                                            .get("name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("unknown");
                                        let mut parameters = tc
                                            .get("parameters")
                                            .cloned()
                                            .unwrap_or(serde_json::json!({}));
                                        if let Some(enforcer) = guardrail_enforcer {
                                            tracing::debug!(
                                                "Guardrail: decoding tool call parameters (tool boundary)"
                                            );
                                            let decoded_result =
                                                enforcer.decode(parameters, DecodeContext::ToolBoundary);
                                            parameters = decoded_result.payload;
                                        }
                                        serde_json::json!({
                                            "id": tc.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                                            "tool_name": name,
                                            "parameters": parameters
                                        })
                                    })
                                    .collect();

                                let tool_calls_json = serde_json::to_string(&python_tool_calls)
                                    .map_err(|e| {
                                        graphbit_core::errors::GraphBitError::workflow_execution(
                                            format!("Failed to serialize tool calls: {}", e),
                                        )
                                    })?;

                                let tool_results_json = Python::with_gil(|py| {
                                    execute_production_tool_calls(
                                        py,
                                        tool_calls_json,
                                        node_tools.clone(),
                                    )
                                })
                                .map_err(|e| {
                                    graphbit_core::errors::GraphBitError::workflow_execution(
                                        format!(
                                            "Failed to execute tools in iteration {}: {}",
                                            iteration, e
                                        ),
                                    )
                                })?;

                                let tool_execution_results: Vec<serde_json::Value> =
                                    serde_json::from_str(&tool_results_json)
                                        .unwrap_or_else(|_| Vec::new());

                                // ---- Step 3: Append tool result messages to history ----
                                for (i, result) in tool_execution_results.iter().enumerate() {
                                    let tool_call_id = assistant_tool_calls
                                        .get(i)
                                        .map(|tc| tc.id.as_str())
                                        .unwrap_or("");
                                    let tool_name = result
                                        .get("tool_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown");
                                    let success = result
                                        .get("success")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false);
                                    let output_text = if success {
                                        result.get("output").and_then(|v| v.as_str()).unwrap_or("")
                                    } else {
                                        result
                                            .get("error")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Tool execution failed")
                                    };

                                    messages.push(LlmMessage::tool(tool_call_id, output_text));

                                    tracing::info!(
                                        "Iteration {} - Tool '{}' result: {} (success: {})",
                                        iteration,
                                        tool_name,
                                        output_text,
                                        success
                                    );
                                }

                                // Record tool executions for observability
                                for (i, result) in tool_execution_results.iter().enumerate() {
                                    let mut enriched = result.clone();
                                    if let Some(obj) = enriched.as_object_mut() {
                                        obj.insert(
                                            "iteration".to_string(),
                                            serde_json::json!(iteration),
                                        );
                                        // Add tool call ID and original parameters from the LLM request
                                        if let Some(tc) = current_tool_calls.get(i) {
                                            if let Some(id) = tc.get("id") {
                                                obj.insert("id".to_string(), id.clone());
                                            }
                                            if let Some(params) = tc.get("parameters") {
                                                obj.insert(
                                                    "parameters".to_string(),
                                                    params.clone(),
                                                );
                                            }
                                        }
                                    }
                                    all_tool_executions.push(enriched);
                                }

                                // ---- Step 4: Call LLM again WITH tools to let it decide next action ----
                                for (i, tc) in python_tool_calls.iter().enumerate() {
                                    let tool_name = tc
                                        .get("tool_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown")
                                        .to_string();
                                    let tool_result = tool_execution_results.get(i);
                                    let success = tool_result
                                        .and_then(|r| r.get("success").and_then(|v| v.as_bool()))
                                        .unwrap_or(false);
                                    let output = tool_result
                                        .and_then(|r| r.get("output").and_then(|v| v.as_str()))
                                        .unwrap_or("")
                                        .to_string();
                                    let error = tool_result
                                        .and_then(|r| r.get("error").and_then(|v| v.as_str()))
                                        .map(|e| serde_json::Value::String(e.to_string()))
                                        .unwrap_or(serde_json::Value::Null);
                                    let start_time = tool_result
                                        .and_then(|r| r.get("start_time"))
                                        .cloned()
                                        .unwrap_or(serde_json::Value::Null);
                                    let end_time = tool_result
                                        .and_then(|r| r.get("end_time"))
                                        .cloned()
                                        .unwrap_or(serde_json::Value::Null);
                                    let latency_ms = tool_result
                                        .and_then(|r| r.get("latency_ms"))
                                        .cloned()
                                        .unwrap_or(serde_json::json!(0.0));
                                    let parameters = tc
                                        .get("parameters")
                                        .cloned()
                                        .unwrap_or(serde_json::json!({}));

                                    if !tools_used.contains(&tool_name) {
                                        tools_used.push(tool_name.clone());
                                    }

                                    let (params_for_meta, output_for_meta) = if let Some(enforcer) =
                                        guardrail_enforcer
                                    {
                                        let parameters_masked = enforcer
                                            .encode(parameters.clone(), EncodeContext::Llm)
                                            .payload;
                                        let parameters_masked = if parameters_masked.is_object() {
                                            parameters_masked
                                        } else {
                                            serde_json::json!({})
                                        };
                                        let enc_output = enforcer.encode(
                                            serde_json::Value::String(output.clone()),
                                            EncodeContext::Llm,
                                        );
                                        let output_masked = enc_output
                                            .payload
                                            .as_str()
                                            .map(String::from)
                                            .unwrap_or_else(|| enc_output.payload.to_string());
                                        (parameters_masked, output_masked)
                                    } else {
                                        (parameters.clone(), output.clone())
                                    };

                                    let entry = serde_json::json!({
                                        "type": "tool_call",
                                        "id": tc.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                                        "tool_name": tool_name,
                                        "parameters": params_for_meta,
                                        "output": output_for_meta,
                                        "success": success,
                                        "error": error,
                                        "start_time": start_time,
                                        "end_time": end_time,
                                        "latency_ms": latency_ms,
                                        "retries": []
                                    });
                                    executions.push(entry);
                                }

                                let mut messages_for_llm = messages.clone();
                                if let Some(enforcer) = guardrail_enforcer {
                                    let payload = serde_json::to_value(&messages_for_llm)
                                        .unwrap_or(serde_json::json!([]));
                                    let decoded_result =
                                        enforcer.decode(payload, DecodeContext::LlmResponse);
                                    if decoded_result.rules_applied_count > 0 {
                                        executions.push(serde_json::json!({
                                            "type": "guardrail_policy",
                                            "operation": "decode",
                                            "pii_rules_applied_count": decoded_result.rules_applied_count,
                                            "pii_rule_names": decoded_result.rule_names,
                                            "policy_name": decoded_result.policy_name
                                        }));
                                    }
                                    if let Ok(decoded_messages) =
                                        serde_json::from_value::<Vec<LlmMessage>>(
                                            decoded_result.payload,
                                        )
                                    {
                                        messages_for_llm = decoded_messages;
                                    }
                                }

                                let mut next_request =
                                    LlmRequest::with_messages(messages_for_llm.clone());
                                for tool in &llm_tools {
                                    next_request = next_request.with_tool(tool.clone());
                                }

                                // Apply node-level configuration overrides
                                if let Some(temp_value) = node.config.get("temperature") {
                                    if let Some(temp_num) = temp_value.as_f64() {
                                        next_request =
                                            next_request.with_temperature(temp_num as f32);
                                    }
                                }
                                if let Some(max_tokens_value) = node.config.get("max_tokens") {
                                    if let Some(max_tokens_num) = max_tokens_value.as_u64() {
                                        next_request =
                                            next_request.with_max_tokens(max_tokens_num as u32);
                                    }
                                }
                                if let Some(top_p_value) = node.config.get("top_p") {
                                    if let Some(top_p_num) = top_p_value.as_f64() {
                                        next_request = next_request.with_top_p(top_p_num as f32);
                                    }
                                }

                                let llm_start = std::time::Instant::now();
                                let next_response = match llm_provider.complete(next_request).await
                                {
                                    Ok(resp) => resp,
                                    Err(e) => {
                                        tracing::error!(
                                            "LLM call failed in iteration {} for node '{}': {}",
                                            iteration,
                                            node_name,
                                            e
                                        );
                                        // Use accumulated tool results as fallback
                                        let fallback: Vec<String> = all_tool_executions
                                            .iter()
                                            .filter_map(|r| {
                                                let name = r.get("tool_name")?.as_str()?;
                                                let output = r.get("output")?.as_str()?;
                                                Some(format!("{}: {}", name, output))
                                            })
                                            .collect();
                                        final_content = fallback.join("\n");
                                        final_raw_output_for_meta = final_content.clone();
                                        final_finish_reason = "error".to_string();
                                        break;
                                    }
                                };
                                llm_calls_in_loop += 1;
                                let llm_duration_ms = llm_start.elapsed().as_secs_f64() * 1000.0;

                                tracing::info!(
                                    "Iteration {} - LLM response: content='{}', tool_calls={}, duration={:.1}ms",
                                    iteration,
                                    &next_response.content.chars().take(100).collect::<String>(),
                                    next_response.tool_calls.len(),
                                    llm_duration_ms
                                );

                                let tool_results_summary = tool_execution_results
                                    .iter()
                                    .map(|result| {
                                        let tool_name = result
                                            .get("tool_name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("unknown");
                                        let success = result
                                            .get("success")
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(false);
                                        if success {
                                            let output = result
                                                .get("output")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("");
                                            format!("{}: {}", tool_name, output)
                                        } else {
                                            let err = result
                                                .get("error")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Tool execution failed");
                                            format!("{}: ERROR - {}", tool_name, err)
                                        }
                                    })
                                    .collect::<Vec<String>>()
                                    .join("\n");
                                let llm_input_prompt_for_meta = format!(
                                    "{}\n\nTool execution results:\n{}\n\nPlease provide a comprehensive response based on the tool results.",
                                    original_prompt, tool_results_summary
                                );
                                let llm_input_for_meta = if let Some(enforcer) = guardrail_enforcer
                                {
                                    let encoded = enforcer.encode(
                                        serde_json::Value::String(llm_input_prompt_for_meta),
                                        EncodeContext::Llm,
                                    );
                                    encoded.payload.as_str().unwrap_or_default().to_string()
                                } else {
                                    llm_input_prompt_for_meta
                                };

                                let mut next_content = next_response.content.clone();
                                let mut next_tool_calls = next_response.tool_calls.clone();
                                let is_final_llm_call = next_response.tool_calls.is_empty();
                                if let Some(enforcer) = guardrail_enforcer {
                                    if !is_final_llm_call {
                                    let payload = serde_json::json!({
                                        "content": next_response.content.clone(),
                                        "tool_calls": next_response.tool_calls.clone(),
                                    });
                                    let decoded_result =
                                        enforcer.decode(payload, DecodeContext::LlmResponse);
                                        if decoded_result.rules_applied_count > 0 {
                                            executions.push(serde_json::json!({
                                                "type": "guardrail_policy",
                                                "operation": "rehydrate",
                                                "pii_rules_applied_count": decoded_result.rules_applied_count,
                                                "pii_rule_names": decoded_result.rule_names,
                                                "policy_name": decoded_result.policy_name
                                            }));
                                        }
                                    if let Some(content) = decoded_result
                                        .payload
                                        .get("content")
                                        .and_then(|v| v.as_str())
                                    {
                                        next_content = content.to_string();
                                    }
                                    if let Some(tc) = decoded_result.payload.get("tool_calls") {
                                        if let Ok(parsed) =
                                            serde_json::from_value::<Vec<LlmToolCall>>(tc.clone())
                                        {
                                            next_tool_calls = parsed;
                                        }
                                    }
                                    }
                                }

                                let llm_call_tool_calls_meta = if guardrail_enforcer.is_some() {
                                    serde_json::json!([])
                                } else {
                                    serde_json::to_value(&next_tool_calls)
                                        .unwrap_or(serde_json::json!([]))
                                };
                                let llm_end_timestamp = chrono::Utc::now();
                                executions.push(serde_json::json!({
                                    "type": "llm_call",
                                    "id": next_response.id.clone().unwrap_or_default(),
                                    "model": next_response.model,
                                    "provider": llm_config.provider_name(),
                                    "input": llm_input_for_meta,
                                    "output": next_response.content,
                                    "finish_reason": format!("{}", next_response.finish_reason),
                                    "tool_calls": llm_call_tool_calls_meta,
                                    "start_time": (llm_end_timestamp - chrono::Duration::milliseconds(llm_duration_ms as i64)).to_rfc3339(),
                                    "end_time": llm_end_timestamp.to_rfc3339(),
                                    "duration_ms": llm_duration_ms,
                                    "usage": {
                                        "prompt_tokens": next_response.usage.prompt_tokens,
                                        "completion_tokens": next_response.usage.completion_tokens,
                                        "total_tokens": next_response.usage.total_tokens,
                                        "prompt_tokens_details": {
                                            "cached_tokens": 0,
                                            "audio_tokens": 0
                                        },
                                        "completion_tokens_details": {
                                            "reasoning_tokens": 0,
                                            "audio_tokens": 0,
                                            "accepted_prediction_tokens": 0,
                                            "rejected_prediction_tokens": 0
                                        }
                                    },
                                    "retries": []
                                }));
                                final_finish_reason = format!("{}", next_response.finish_reason);
                                final_raw_output_for_meta = next_response.content.clone();

                                // ---- Step 5: Check if LLM wants more tools or is done ----
                                if next_tool_calls.is_empty() {
                                    // No more tool calls — LLM produced a final answer
                                    tracing::info!(
                                        "Agent loop completed after {} iteration(s) for node '{}' - final answer produced",
                                        iteration,
                                        node_name
                                    );
                                    final_content = next_content.clone();
                                    break;
                                }

                                // LLM wants to call more tools — update state and continue loop
                                current_content = next_content;
                                current_tool_calls = serde_json::to_value(&next_tool_calls)
                                    .and_then(|v| {
                                        serde_json::from_value::<Vec<serde_json::Value>>(v)
                                    })
                                    .unwrap_or_default();
                            }

                            // ============================================================
                            // STORE RESULTS AND METADATA
                            // ============================================================

                            let overall_duration_ms =
                                overall_start.elapsed().as_secs_f64() * 1000.0;
                            tracing::info!(
                                "Completed iterative loop for node '{}' with {} additional LLM call(s)",
                                node_name,
                                llm_calls_in_loop
                            );

                            // Aggregate usage from all LLM executions (initial + iterative)
                            let mut total_prompt_tokens: u32 = 0;
                            let mut total_completion_tokens: u32 = 0;
                            let mut total_tokens: u32 = 0;
                            for exec in &executions {
                                if exec.get("type").and_then(|v| v.as_str()) == Some("llm_call") {
                                    if let Some(usage) = exec.get("usage") {
                                        total_prompt_tokens += usage
                                            .get("prompt_tokens")
                                            .and_then(|v| v.as_u64())
                                            .unwrap_or(0)
                                            as u32;
                                        total_completion_tokens += usage
                                            .get("completion_tokens")
                                            .and_then(|v| v.as_u64())
                                            .unwrap_or(0)
                                            as u32;
                                        total_tokens += usage
                                            .get("total_tokens")
                                            .and_then(|v| v.as_u64())
                                            .unwrap_or(0)
                                            as u32;
                                    }
                                }
                            }

                            let llm_call_count = executions
                                .iter()
                                .filter(|e| {
                                    e.get("type").and_then(|v| v.as_str()) == Some("llm_call")
                                })
                                .count() as u64;
                            let total_iterations = llm_call_count.saturating_sub(1);
                            let total_tool_calls = executions
                                .iter()
                                .filter(|e| {
                                    e.get("type").and_then(|v| v.as_str()) == Some("tool_call")
                                })
                                .count() as u64;

                            let mut node_meta = existing_node_metadata
                                .clone()
                                .unwrap_or_else(|| serde_json::json!({}));
                            if let Some(obj) = node_meta.as_object_mut() {
                                let end_time = chrono::Utc::now();
                                obj.insert(
                                    "end_time".to_string(),
                                    serde_json::json!(end_time.to_rfc3339()),
                                );
                                if let Some(start_str) =
                                    obj.get("start_time").and_then(|v| v.as_str())
                                {
                                    if let Ok(start_dt) =
                                        chrono::DateTime::parse_from_rfc3339(start_str)
                                    {
                                        let total_duration = (end_time
                                            - start_dt.with_timezone(&chrono::Utc))
                                        .num_milliseconds()
                                            as f64;
                                        obj.insert(
                                            "duration_ms".to_string(),
                                            serde_json::json!(total_duration),
                                        );
                                    } else {
                                        obj.insert(
                                            "duration_ms".to_string(),
                                            serde_json::json!(overall_duration_ms),
                                        );
                                    }
                                } else {
                                    obj.insert(
                                        "duration_ms".to_string(),
                                        serde_json::json!(overall_duration_ms),
                                    );
                                }
                                if !obj.contains_key("max_iterations") {
                                    obj.insert(
                                        "max_iterations".to_string(),
                                        serde_json::json!(max_iterations),
                                    );
                                }
                                obj.insert(
                                    "final_output".to_string(),
                                    serde_json::Value::String(if guardrail_enforcer.is_some() {
                                        final_raw_output_for_meta.clone()
                                    } else {
                                        final_content.clone()
                                    }),
                                );
                                obj.insert(
                                    "exit_reason".to_string(),
                                    serde_json::json!(final_finish_reason.clone()),
                                );
                                obj.insert(
                                    "total_iterations".to_string(),
                                    serde_json::json!(total_iterations),
                                );
                                obj.insert(
                                    "total_tool_calls".to_string(),
                                    serde_json::json!(total_tool_calls),
                                );
                                obj.insert("tools_used".to_string(), serde_json::json!(tools_used));
                                obj.insert(
                                    "total_usage".to_string(),
                                    serde_json::json!({
                                        "prompt_tokens": total_prompt_tokens,
                                        "completion_tokens": total_completion_tokens,
                                        "total_tokens": total_tokens,
                                        "prompt_tokens_details": {
                                            "cached_tokens": 0,
                                            "audio_tokens": 0
                                        },
                                        "completion_tokens_details": {
                                            "reasoning_tokens": 0,
                                            "audio_tokens": 0,
                                            "accepted_prediction_tokens": 0,
                                            "rejected_prediction_tokens": 0
                                        }
                                    }),
                                );
                                obj.insert(
                                    "executions".to_string(),
                                    serde_json::json!(executions.clone()),
                                );
                            }

                            context
                                .metadata
                                .insert(format!("node_response_{}", node.id), node_meta.clone());
                            context
                                .metadata
                                .insert(format!("node_response_{}", node_name), node_meta);

                            // Create completely raw, unstructured output
                            let tool_results_summary = all_tool_executions
                                .iter()
                                .map(|result| {
                                    let success = result
                                        .get("success")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false);
                                    if success {
                                        let output = result
                                            .get("output")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("");
                                        output.to_string()
                                    } else {
                                        let err = result
                                            .get("error")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Tool execution failed");
                                        err.to_string()
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join("\n");

                            // Build completely raw, unstructured conversation output
                            let mut comprehensive_output = String::new();
                            
                            // Add original prompt (raw)
                            comprehensive_output.push_str(&original_prompt);
                            comprehensive_output.push_str("\n\n");
                            
                            // Add tool results if any were made (raw)
                            if !tool_results_summary.is_empty() {
                                comprehensive_output.push_str(&tool_results_summary);
                                comprehensive_output.push_str("\n\n");
                            }
                            
                            // Add final answer (raw)
                            comprehensive_output.push_str(&final_content);
                            
                            let final_value = serde_json::Value::String(comprehensive_output.clone());
                            
                            // CRITICAL: Update node outputs with resolved answer (replaces tool_calls_required)
                            tracing::info!(
                                "Storing resolved tool output for node '{}' ({}): {}",
                                node_name,
                                node.id,
                                &comprehensive_output.chars().take(200).collect::<String>()
                            );
                            
                            context.set_node_output(&node.id, final_value.clone());
                            context.set_node_output_by_name(&node_name, final_value.clone());
                            context.set_variable(node_name.clone(), final_value.clone());
                            context.set_variable(node.id.to_string(), final_value);
                            
                            tracing::info!(
                                "Node outputs updated: {} and {}",
                                node.id.to_string(),
                                node_name
                            );
                        }
                    }
                }
            }
        }

        Ok((context, nodes_with_tool_calls))
    }

    /// Update execution statistics
    fn update_stats(&mut self, success: bool, duration: Duration) {
        if !self.config.enable_metrics {
            return;
        }

        self.stats.total_executions += 1;
        let duration_ms = duration.as_millis() as u64;
        self.stats.total_duration_ms += duration_ms;

        if success {
            self.stats.successful_executions += 1;
        } else {
            self.stats.failed_executions += 1;
        }

        // Update average duration (simple moving average)
        self.stats.average_duration_ms =
            self.stats.total_duration_ms as f64 / self.stats.total_executions as f64;
    }
}
