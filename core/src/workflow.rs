//! Workflow execution engine for GraphBit
//!
//! This module provides the main workflow execution capabilities,
//! orchestrating agents and managing the execution flow.

use crate::agents::AgentTrait;
use crate::document_loader::DocumentLoader;
use crate::errors::{GraphBitError, GraphBitResult};
use crate::graph::{NodeType, WorkflowGraph, WorkflowNode};
use crate::types::{
    AgentId, AgentMessage, CircuitBreaker, CircuitBreakerConfig, ConcurrencyConfig,
    ConcurrencyManager, ConcurrencyStats, MessageContent, NodeExecutionResult, NodeId, RetryConfig,
    TaskInfo, WorkflowContext, WorkflowExecutionStats, WorkflowId, WorkflowState,
};
use futures::future::join_all;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

lazy_static! {
    static ref NODE_REF_PATTERN: Regex = Regex::new(r"\{\{node\.([a-zA-Z0-9_\-\.]+)\}\}").unwrap();
}

/// A complete workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique workflow identifier
    pub id: WorkflowId,
    /// Workflow name
    pub name: String,
    /// Workflow description
    pub description: String,
    /// The workflow graph
    pub graph: WorkflowGraph,
    /// Workflow metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Workflow {
    /// Create a new workflow
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: WorkflowId::new(),
            name: name.into(),
            description: description.into(),
            graph: WorkflowGraph::new(),
            metadata: HashMap::with_capacity(4),
        }
    }

    /// Add a node to the workflow
    pub fn add_node(&mut self, node: WorkflowNode) -> GraphBitResult<NodeId> {
        let node_id = node.id.clone();
        self.graph.add_node(node)?;
        Ok(node_id)
    }

    /// Connect two nodes with an edge
    pub fn connect_nodes(
        &mut self,
        from: NodeId,
        to: NodeId,
        edge: crate::graph::WorkflowEdge,
    ) -> GraphBitResult<()> {
        self.graph.add_edge(from, to, edge)
    }

    /// Validate the workflow
    pub fn validate(&self) -> GraphBitResult<()> {
        self.graph.validate()
    }

    /// Set workflow metadata
    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }
}

/// Builder for creating workflows with fluent API
pub struct WorkflowBuilder {
    workflow: Workflow,
}

impl WorkflowBuilder {
    /// Start building a new workflow
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            workflow: Workflow::new(name, ""),
        }
    }

    /// Set workflow description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.workflow.description = description.into();
        self
    }

    /// Add a node to the workflow
    pub fn add_node(mut self, node: WorkflowNode) -> GraphBitResult<(Self, NodeId)> {
        let node_id = self.workflow.add_node(node)?;
        Ok((self, node_id))
    }

    /// Connect two nodes
    pub fn connect(
        mut self,
        from: NodeId,
        to: NodeId,
        edge: crate::graph::WorkflowEdge,
    ) -> GraphBitResult<Self> {
        self.workflow.connect_nodes(from, to, edge)?;
        Ok(self)
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.workflow.set_metadata(key, value);
        self
    }

    /// Build the workflow
    pub fn build(self) -> GraphBitResult<Workflow> {
        self.workflow.validate()?;
        Ok(self.workflow)
    }
}

/// Workflow execution engine
pub struct WorkflowExecutor {
    /// Registered agents - use RwLock for better read performance
    agents: Arc<RwLock<HashMap<crate::types::AgentId, Arc<dyn AgentTrait>>>>,
    /// Simplified concurrency management system
    concurrency_manager: Arc<ConcurrencyManager>,
    /// Maximum execution time per node in milliseconds
    max_node_execution_time_ms: Option<u64>,
    /// Whether to fail fast on first error or continue with other nodes
    fail_fast: bool,
    /// Default retry configuration for all nodes
    default_retry_config: Option<RetryConfig>,
    /// Circuit breakers per agent to prevent cascading failures - use RwLock for better performance
    circuit_breakers: Arc<RwLock<HashMap<crate::types::AgentId, CircuitBreaker>>>,
    /// Global circuit breaker configuration
    circuit_breaker_config: CircuitBreakerConfig,
    /// Default LLM configuration for auto-generated agents
    default_llm_config: Option<crate::llm::LlmConfig>,
}

impl WorkflowExecutor {
    /// Create a new workflow executor with sensible defaults
    pub fn new() -> Self {
        let concurrency_config = ConcurrencyConfig::default();
        let concurrency_manager = Arc::new(ConcurrencyManager::new(concurrency_config));

        Self {
            agents: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            concurrency_manager,
            max_node_execution_time_ms: None,
            fail_fast: false,
            default_retry_config: Some(RetryConfig::default()),
            circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(8))),
            circuit_breaker_config: CircuitBreakerConfig::default(),
            default_llm_config: None,
        }
    }

    /// Create a workflow executor optimized for high throughput
    pub fn new_high_throughput() -> Self {
        let concurrency_config = ConcurrencyConfig::high_throughput();
        let concurrency_manager = Arc::new(ConcurrencyManager::new(concurrency_config));

        Self {
            agents: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            concurrency_manager,
            max_node_execution_time_ms: None,
            fail_fast: false,
            default_retry_config: Some(RetryConfig::default()),
            circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(8))),
            circuit_breaker_config: CircuitBreakerConfig::default(),
            default_llm_config: None,
        }
    }

    /// Create a workflow executor optimized for low latency
    pub fn new_low_latency() -> Self {
        let concurrency_config = ConcurrencyConfig::low_latency();
        let concurrency_manager = Arc::new(ConcurrencyManager::new(concurrency_config));

        Self {
            agents: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            concurrency_manager,
            max_node_execution_time_ms: None,
            fail_fast: true,            // Fail fast for low latency
            default_retry_config: None, // No retries for low latency
            circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(8))),
            circuit_breaker_config: CircuitBreakerConfig::default(),
            default_llm_config: None,
        }
    }

    /// Create a workflow executor optimized for memory usage
    pub fn new_memory_optimized() -> Self {
        let concurrency_config = ConcurrencyConfig::memory_optimized();
        let concurrency_manager = Arc::new(ConcurrencyManager::new(concurrency_config));

        Self {
            agents: Arc::new(RwLock::new(HashMap::with_capacity(8))),
            concurrency_manager,
            max_node_execution_time_ms: None,
            fail_fast: false,
            default_retry_config: Some(RetryConfig::default()),
            circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(4))),
            circuit_breaker_config: CircuitBreakerConfig::default(),
            default_llm_config: None,
        }
    }

    /// Register an agent with the executor
    pub async fn register_agent(&self, agent: Arc<dyn AgentTrait>) {
        let agent_id = agent.id().clone();
        self.agents.write().await.insert(agent_id, agent);
    }

    /// Set maximum execution time per node
    pub fn with_max_node_execution_time(mut self, timeout_ms: u64) -> Self {
        self.max_node_execution_time_ms = Some(timeout_ms);
        self
    }

    /// Configure whether to fail fast on errors
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    /// Set retry configuration
    pub fn with_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.default_retry_config = Some(retry_config);
        self
    }

    /// Set circuit breaker configuration
    pub fn with_circuit_breaker_config(mut self, config: CircuitBreakerConfig) -> Self {
        self.circuit_breaker_config = config;
        self
    }

    /// Set default LLM configuration for auto-generated agents
    pub fn with_default_llm_config(mut self, llm_config: crate::llm::LlmConfig) -> Self {
        self.default_llm_config = Some(llm_config);
        self
    }

    /// Disable retries
    pub fn without_retries(mut self) -> Self {
        self.default_retry_config = None;
        self
    }

    /// Get concurrency statistics
    pub async fn get_concurrency_stats(&self) -> ConcurrencyStats {
        self.concurrency_manager.get_stats().await
    }

    /// Resolve LLM configuration for a node with hierarchical priority
    /// Priority: Node-level config > Executor-level config > Default
    fn resolve_llm_config_for_node(
        &self,
        node_config: &std::collections::HashMap<String, serde_json::Value>,
    ) -> crate::llm::LlmConfig {
        // 1. Check for node-level LLM config first (highest priority)
        if let Some(node_llm_config) = node_config.get("llm_config") {
            if let Ok(config) =
                serde_json::from_value::<crate::llm::LlmConfig>(node_llm_config.clone())
            {
                tracing::debug!(
                    "Using node-level LLM configuration: {:?}",
                    config.provider_name()
                );
                return config;
            } else {
                tracing::warn!(
                    "Failed to deserialize node-level LLM config, falling back to executor config"
                );
            }
        }

        // 2. Fall back to executor-level config (medium priority)
        if let Some(executor_config) = &self.default_llm_config {
            tracing::debug!(
                "Using executor-level LLM configuration: {:?}",
                executor_config.provider_name()
            );
            return executor_config.clone();
        }

        // 3. Use default configuration (lowest priority)
        let default_config = crate::llm::LlmConfig::default();
        tracing::debug!(
            "Using default LLM configuration: {:?}",
            default_config.provider_name()
        );
        default_config
    }

    /// Get or create circuit breaker for an agent
    async fn get_circuit_breaker(&self, agent_id: &crate::types::AgentId) -> CircuitBreaker {
        // Try to read first (more efficient for existing breakers)
        {
            let breakers = self.circuit_breakers.read().await;
            if let Some(breaker) = breakers.get(agent_id) {
                return breaker.clone();
            }
        }

        // If not found, acquire write lock and create
        let mut breakers = self.circuit_breakers.write().await;
        breakers
            .entry(agent_id.clone())
            .or_insert_with(|| CircuitBreaker::new(self.circuit_breaker_config.clone()))
            .clone()
    }

    /// Get current concurrency limit
    pub async fn max_concurrency(&self) -> usize {
        // Get the global max concurrency from the concurrency manager
        let _stats = self.concurrency_manager.get_stats().await;
        let permits = self.concurrency_manager.get_available_permits().await;
        permits.get("global").copied().unwrap_or(16) // Default fallback
    }

    /// Get available permits in semaphore
    pub async fn available_permits(&self) -> HashMap<String, usize> {
        self.concurrency_manager.get_available_permits().await
    }

    /// Execute a workflow with enhanced performance monitoring
    pub async fn execute(&self, workflow: Workflow) -> GraphBitResult<WorkflowContext> {
        let start_time = std::time::Instant::now();

        // Initialize workflow context with simple constructor
        let mut context = WorkflowContext::new(workflow.id.clone());

        // Set initial workflow state
        context.state = WorkflowState::Running {
            current_node: NodeId::new(),
        };

        // Validate workflow before execution
        workflow.validate()?;

        // PERFORMANCE FIX: Auto-register agents for all agent nodes found in workflow
        let agent_ids = extract_agent_ids_from_workflow(&workflow);
        if agent_ids.is_empty() {
            return Err(GraphBitError::validation(
                "workflow",
                "No agents found in workflow",
            ));
        }

        // Auto-register missing agents to prevent lookup failures
        for agent_id_str in &agent_ids {
            if let Ok(agent_id) = AgentId::from_string(agent_id_str) {
                // Check if agent is already registered
                let agent_exists = {
                    let agents_guard = self.agents.read().await;
                    agents_guard.contains_key(&agent_id)
                };

                // If agent doesn't exist, create and register a default agent
                if !agent_exists {
                    // Find the node configuration for this agent to extract system_prompt and LLM config
                    let mut system_prompt = String::new();
                    let mut resolved_llm_config =
                        self.default_llm_config.clone().unwrap_or_default();

                    for node in workflow.graph.get_nodes().values() {
                        if let NodeType::Agent {
                            agent_id: node_agent_id,
                            ..
                        } = &node.node_type
                        {
                            if node_agent_id == &agent_id {
                                // Extract system_prompt from node config if available
                                if let Some(prompt_value) = node.config.get("system_prompt") {
                                    if let Some(prompt_str) = prompt_value.as_str() {
                                        system_prompt = prompt_str.to_string();
                                    }
                                }

                                // Resolve LLM configuration with hierarchical priority:
                                // 1. Node-level config > 2. Executor-level config > 3. Default
                                resolved_llm_config =
                                    self.resolve_llm_config_for_node(&node.config);
                                break;
                            }
                        }
                    }

                    // Create default agent configuration for this workflow
                    let mut default_config = crate::agents::AgentConfig::new(
                        format!("Agent_{}", agent_id_str),
                        "Auto-generated agent for workflow execution",
                        resolved_llm_config,
                    )
                    .with_id(agent_id.clone());

                    // Set system prompt if found in node configuration
                    if !system_prompt.is_empty() {
                        default_config = default_config.with_system_prompt(system_prompt);
                    }

                    // Try to create agent - if it fails due to config issues, fail the workflow
                    match crate::agents::Agent::new(default_config).await {
                        Ok(agent) => {
                            let mut agents_guard = self.agents.write().await;
                            agents_guard.insert(agent_id.clone(), Arc::new(agent));
                            tracing::debug!("Auto-registered agent: {}", agent_id);
                        }
                        Err(e) => {
                            return Err(GraphBitError::workflow_execution(format!(
                                "Failed to create agent '{}': {}. This may be due to invalid API key or configuration.",
                                agent_id_str, e
                            )));
                        }
                    }
                }

                // Pre-warm circuit breakers for all agents
                let _ = self.get_circuit_breaker(&agent_id).await;
            }
        }

        // Pre-compute and store dependency map and id->name map into context metadata
        {
            // Build dependency map: node_id -> [parent_node_ids...]
            let mut deps_map: HashMap<String, Vec<String>> = HashMap::new();
            // Build id->name map for better labeling
            let mut id_name_map: HashMap<String, String> = HashMap::new();

            for (nid, node) in workflow.graph.get_nodes() {
                id_name_map.insert(nid.to_string(), node.name.clone());
            }

            // We need a mutable graph to call get_dependencies (it caches)
            let mut graph_clone = workflow.graph.clone();
            for nid in id_name_map.keys() {
                // Convert back to NodeId via from_string (deterministic for UUIDs)
                if let Ok(node_id) = NodeId::from_string(nid) {
                    let parents = graph_clone
                        .get_dependencies(&node_id)
                        .into_iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>();
                    deps_map.insert(nid.clone(), parents);
                }
            }

            context.set_metadata(
                "node_dependencies".to_string(),
                serde_json::to_value(deps_map).unwrap_or(serde_json::json!({})),
            );
            context.set_metadata(
                "node_id_to_name".to_string(),
                serde_json::to_value(id_name_map).unwrap_or(serde_json::json!({})),
            );
        }

        let nodes = self.collect_executable_nodes(&workflow.graph)?;
        if nodes.is_empty() {
            context.complete();
            return Ok(context);
        }

        // Execute nodes in dependency-aware batches (parents before children)
        let batches = self.create_dependency_batches(&workflow.graph).await?;
        tracing::info!(
            batch_count = batches.len(),
            "Planned dependency-aware batches"
        );
        let mut total_executed = 0;
        let mut total_successful = 0;

        for batch in batches {
            let batch_size = batch.len();
            let batch_ids: Vec<String> = batch.iter().map(|n| n.id.to_string()).collect();
            tracing::info!(batch_size, batch_node_ids = ?batch_ids, "Executing batch");

            // Execute batch concurrently with optimized spawning
            let shared_context = Arc::new(Mutex::new(context));

            // Pre-allocate tasks vector for better memory efficiency
            let mut tasks = Vec::with_capacity(batch_size);

            for node in batch {
                let context_clone = shared_context.clone();
                let agents_clone = self.agents.clone();
                let circuit_breakers_clone = self.circuit_breakers.clone();
                let circuit_breaker_config = self.circuit_breaker_config.clone();
                let retry_config = self.default_retry_config.clone();
                let concurrency_manager = self.concurrency_manager.clone();

                // Use lightweight task spawning without unnecessary permit acquisition overhead
                let task = tokio::spawn(async move {
                    // Simplified concurrency control - just acquire basic permits
                    let task_info = TaskInfo::from_node_type(&node.node_type, &node.id);

                    // Fast path: skip permit acquisition for simple nodes to reduce overhead
                    let _permits = if matches!(node.node_type, NodeType::Agent { .. }) {
                        Some(
                            concurrency_manager
                                .acquire_permits(&task_info)
                                .await
                                .map_err(|e| {
                                    GraphBitError::workflow_execution(format!(
                                        "Failed to acquire permits for node {}: {}",
                                        node.id, e
                                    ))
                                })?,
                        )
                    } else {
                        None // Skip permit system for non-agent nodes
                    };

                    // Execute the node with retry logic
                    Self::execute_node_with_retry(
                        node,
                        context_clone,
                        agents_clone,
                        circuit_breakers_clone,
                        circuit_breaker_config,
                        retry_config,
                    )
                    .await
                });
                tasks.push(task);
            }

            // Wait for all tasks in the batch to complete
            let results = join_all(tasks).await;

            let mut should_fail_fast = false;
            let mut failure_message = String::new();

            for task_result in results {
                match task_result {
                    Ok(Ok(node_result)) => {
                        total_executed += 1;
                        if node_result.success {
                            total_successful += 1;
                        }

                        // Update context with results using meaningful variable names
                        // 1) Populate node_outputs (JSON) by ID and by Name for automatic data flow
                        // 2) Also populate variables with stringified output for backward compatibility
                        let mut ctx = shared_context.lock().await;
                        if let Some(node) = workflow.graph.get_node(&node_result.node_id) {
                            // Store raw JSON output for data flow
                            ctx.set_node_output(&node.id, node_result.output.clone());
                            ctx.set_node_output_by_name(&node.name, node_result.output.clone());

                            // Debug: confirm keys present after store
                            let keys_now: Vec<String> = ctx.node_outputs.keys().cloned().collect();
                            tracing::debug!(
                                stored_node_id = %node.id,
                                stored_node_name = %node.name,
                                node_output_keys_now = ?keys_now,
                                "Stored node output in context.node_outputs"
                            );

                            // Back-compat: also set variables as strings
                            if let Ok(output_str) = serde_json::to_string(&node_result.output) {
                                ctx.set_variable(
                                    node.name.clone(),
                                    serde_json::Value::String(output_str.clone()),
                                );
                                ctx.set_variable(
                                    node.id.to_string(),
                                    serde_json::Value::String(output_str),
                                );
                            }
                        } else {
                            // Fallback to generic naming if node not found (shouldn't happen)
                            if let Ok(output_str) = serde_json::to_string(&node_result.output) {
                                ctx.set_variable(
                                    format!("node_result_{}", total_executed),
                                    serde_json::Value::String(output_str),
                                );
                                tracing::debug!(
                                    executed_index = total_executed,
                                    "Stored output under generic variable name (node not found)"
                                );
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        // Check for critical authentication/configuration errors that should always fail the workflow
                        let error_msg = e.to_string().to_lowercase();
                        let is_auth_error = error_msg.contains("auth")
                            || error_msg.contains("key")
                            || error_msg.contains("invalid")
                            || error_msg.contains("unauthorized")
                            || error_msg.contains("permission")
                            || error_msg.contains("api error");

                        if is_auth_error || self.fail_fast {
                            should_fail_fast = true;
                            failure_message = e.to_string();
                            break;
                        }
                        total_executed += 1;
                    }
                    Err(e) => {
                        if self.fail_fast {
                            should_fail_fast = true;
                            failure_message = format!("Task execution failed: {}", e);
                            break;
                        }
                        total_executed += 1;
                    }
                }
            }

            // Handle fail fast outside the loop to avoid borrow conflicts
            if should_fail_fast {
                let mut ctx = shared_context.lock().await;
                ctx.fail(failure_message);
                drop(ctx); // Explicitly drop the guard
                return Ok(Arc::try_unwrap(shared_context).unwrap().into_inner());
            }

            context = Arc::try_unwrap(shared_context).unwrap().into_inner();
        }

        // Set execution statistics
        let total_time = start_time.elapsed();
        let stats = WorkflowExecutionStats {
            total_nodes: total_executed,
            successful_nodes: total_successful,
            failed_nodes: total_executed - total_successful,
            avg_execution_time_ms: total_time.as_millis() as f64 / total_executed.max(1) as f64,
            max_concurrent_nodes: self.max_concurrency().await,
            total_execution_time_ms: total_time.as_millis() as u64,
            peak_memory_usage_mb: None, // Could add memory tracking here
            semaphore_acquisitions: 0,  // Updated in the loop
            avg_semaphore_wait_ms: 0.0, // Updated in the loop
        };

        context.set_stats(stats);
        context.complete();

        Ok(context)
    }

    /// Execute a node with retry logic and circuit breaker
    async fn execute_node_with_retry(
        node: WorkflowNode,
        context: Arc<Mutex<WorkflowContext>>,
        agents: Arc<RwLock<HashMap<crate::types::AgentId, Arc<dyn AgentTrait>>>>,
        circuit_breakers: Arc<RwLock<HashMap<crate::types::AgentId, CircuitBreaker>>>,
        circuit_breaker_config: CircuitBreakerConfig,
        retry_config: Option<RetryConfig>,
    ) -> GraphBitResult<NodeExecutionResult> {
        let start_time = std::time::Instant::now();
        let mut attempt = 0;

        // Get circuit breaker for agent nodes
        let mut circuit_breaker = if let NodeType::Agent { agent_id, .. } = &node.node_type {
            let mut breakers = circuit_breakers.write().await;
            Some(
                breakers
                    .entry(agent_id.clone())
                    .or_insert_with(|| CircuitBreaker::new(circuit_breaker_config.clone()))
                    .clone(),
            )
        } else {
            None
        };

        loop {
            // Check circuit breaker before attempting execution
            if let Some(ref mut breaker) = circuit_breaker {
                if !breaker.should_allow_request() {
                    let error = GraphBitError::workflow_execution(
                        "Circuit breaker is open - requests are being rejected".to_string(),
                    );
                    return Ok(
                        NodeExecutionResult::failure(error.to_string(), node.id.clone())
                            .with_duration(start_time.elapsed().as_millis() as u64)
                            .with_retry_count(attempt),
                    );
                }
            }

            // Attempt to execute the node
            let result = match &node.node_type {
                NodeType::Agent {
                    agent_id,
                    prompt_template,
                } => {
                    Self::execute_agent_node_static(
                        &node.id,
                        agent_id,
                        prompt_template,
                        &node.config,
                        context.clone(),
                        agents.clone(),
                    )
                    .await
                }
                NodeType::Condition { expression } => {
                    Self::execute_condition_node_static(expression).await
                }
                NodeType::Transform { transformation } => {
                    Self::execute_transform_node_static(transformation, context.clone()).await
                }
                NodeType::Delay { duration_seconds } => {
                    Self::execute_delay_node_static(*duration_seconds).await
                }
                NodeType::DocumentLoader {
                    document_type,
                    source_path,
                    ..
                } => {
                    Self::execute_document_loader_node_static(
                        document_type,
                        source_path,
                        context.clone(),
                    )
                    .await
                }
                _ => Err(GraphBitError::workflow_execution(format!(
                    "Unsupported node type: {:?}",
                    node.node_type
                ))),
            };

            match result {
                Ok(output) => {
                    // Store the node output in the context for automatic data flow
                    // Store using both NodeId and node name for flexible access
                    {
                        let mut ctx = context.lock().await;
                        ctx.set_node_output(&node.id, output.clone());
                        ctx.set_node_output_by_name(&node.name, output.clone());

                        // PRODUCTION FIX: Also populate variables for backward compatibility
                        // This ensures extract_output() functions work correctly
                        if let Ok(output_str) = serde_json::to_string(&output) {
                            ctx.set_variable(
                                node.name.clone(),
                                serde_json::Value::String(output_str.clone()),
                            );
                            ctx.set_variable(
                                node.id.to_string(),
                                serde_json::Value::String(output_str),
                            );
                        }

                        // Debug: confirm storage keys available after this node completes
                        let keys: Vec<String> = ctx.node_outputs.keys().cloned().collect();
                        let var_keys: Vec<String> = ctx.variables.keys().cloned().collect();
                        tracing::debug!(
                            node_id = %node.id,
                            node_name = %node.name,
                            available_output_keys = ?keys,
                            available_variable_keys = ?var_keys,
                            "Stored node output and variables"
                        );
                    }

                    // Record success in circuit breaker
                    if let Some(ref mut breaker) = circuit_breaker {
                        breaker.record_success();
                        if let NodeType::Agent { agent_id, .. } = &node.node_type {
                            let mut breakers = circuit_breakers.write().await;
                            breakers.insert(agent_id.clone(), breaker.clone());
                        }
                    }

                    let duration = start_time.elapsed();
                    return Ok(NodeExecutionResult::success(output, node.id.clone())
                        .with_duration(duration.as_millis() as u64)
                        .with_retry_count(attempt));
                }
                Err(error) => {
                    // Record failure in circuit breaker
                    if let Some(ref mut breaker) = circuit_breaker {
                        breaker.record_failure();
                        if let NodeType::Agent { agent_id, .. } = &node.node_type {
                            let mut breakers = circuit_breakers.write().await;
                            breakers.insert(agent_id.clone(), breaker.clone());
                        }
                    }

                    // Check if we should retry
                    if let Some(ref config) = retry_config {
                        if config.should_retry(&error, attempt) {
                            attempt += 1;

                            // Calculate delay for this attempt
                            let delay_ms = config.calculate_delay(attempt);
                            if delay_ms > 0 {
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                    .await;
                            }

                            continue;
                        }
                    }

                    // No more retries, return the error
                    let duration = start_time.elapsed();
                    return Ok(
                        NodeExecutionResult::failure(error.to_string(), node.id.clone())
                            .with_duration(duration.as_millis() as u64)
                            .with_retry_count(attempt),
                    );
                }
            }
        }
    }

    /// Execute an agent node (static version)
    async fn execute_agent_node_static(
        current_node_id: &NodeId,
        agent_id: &crate::types::AgentId,
        prompt_template: &str,
        node_config: &std::collections::HashMap<String, serde_json::Value>,
        context: Arc<Mutex<WorkflowContext>>,
        agents: Arc<RwLock<HashMap<crate::types::AgentId, Arc<dyn AgentTrait>>>>,
    ) -> GraphBitResult<serde_json::Value> {
        // Use read lock for better performance
        let agents_guard = agents.read().await;
        let agent = agents_guard
            .get(agent_id)
            .ok_or_else(|| GraphBitError::agent_not_found(agent_id.to_string()))?
            .clone();
        drop(agents_guard); // Release the lock early

        // Build implicit preamble from upstream (parent) node outputs, then resolve templates
        let resolved_prompt = {
            let ctx = context.lock().await;

            // Extract dependency map and name map from metadata
            let deps_map = ctx
                .metadata
                .get("node_dependencies")
                .cloned()
                .unwrap_or(serde_json::json!({}));
            let id_name_map = ctx
                .metadata
                .get("node_id_to_name")
                .cloned()
                .unwrap_or(serde_json::json!({}));

            // Collect preamble sections from DIRECT parents of this node
            let mut sections: Vec<String> = Vec::new();
            // Also collect a JSON map of parent outputs for CrewAI-style context passing
            let mut parents_json: serde_json::Map<String, serde_json::Value> =
                serde_json::Map::new();

            // Use id->name map for titles
            let id_name_obj = id_name_map.as_object();

            // Resolve current node id string and direct parents from deps map
            let cur_id_str = current_node_id.to_string();
            let parent_ids: Vec<String> = deps_map
                .as_object()
                .and_then(|m| m.get(&cur_id_str))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();

            // Debug: log parent ids and available node_outputs keys
            let available_keys: Vec<String> = ctx.node_outputs.keys().cloned().collect();
            tracing::debug!(
                current_node_id = %cur_id_str,
                parent_ids = ?parent_ids,
                available_output_keys = ?available_keys,
                "Implicit preamble: checking direct parents and available outputs"
            );

            // Build a set of keys to include: each parent by id and by name (if known)
            let mut include_keys: Vec<String> = Vec::new();
            if let Some(map) = id_name_obj {
                for pid in &parent_ids {
                    include_keys.push(pid.clone());
                    if let Some(name_val) = map.get(pid).and_then(|v| v.as_str()) {
                        include_keys.push(name_val.to_string());
                    }
                }
            } else {
                include_keys.extend(parent_ids.iter().cloned());
            }

            // Preserve order by iterating parent_ids, using id->name for titles, and fetching outputs by key
            for pid in &parent_ids {
                // Determine title from id->name map; fallback to id
                let title = id_name_obj
                    .and_then(|m| m.get(pid))
                    .and_then(|v| v.as_str())
                    .unwrap_or(pid.as_str())
                    .to_string();

                // Prefer fetching by id first, then by name key
                let val_opt = ctx.node_outputs.get(pid).or_else(|| {
                    id_name_obj
                        .and_then(|m| m.get(pid))
                        .and_then(|v| v.as_str())
                        .and_then(|name| ctx.node_outputs.get(name))
                });

                if let Some(value) = val_opt {
                    let value_str = match value {
                        serde_json::Value::String(s) => s.clone(),
                        _ => value.to_string(),
                    };
                    // Original titled section
                    sections.push(format!("=== {} ===\n{}", title, value_str));

                    // Always add to JSON context by id as a fallback key
                    parents_json.insert(pid.to_string(), value.clone());

                    // Also add a generic alias label derived from parent name (fully generic behavior)
                    if let Some(parent_name) = id_name_obj
                        .and_then(|m| m.get(pid))
                        .and_then(|v| v.as_str())
                    {
                        // Add to JSON context by name
                        parents_json.insert(parent_name.to_string(), value.clone());

                        // Generic alias label: normalize separators, title-case tokens, uppercased heading
                        let generic_label = parent_name
                            .replace(['_', '-'], " ")
                            .split_whitespace()
                            .map(|w| {
                                let mut ch = w.chars();
                                match ch.next() {
                                    Some(first) => {
                                        first.to_uppercase().collect::<String>() + ch.as_str()
                                    }
                                    None => String::new(),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                            .to_uppercase();
                        sections.push(format!("{}:\n{}", generic_label, value_str));
                    }
                } else {
                    // Debug: could not find value for this parent id/name
                    let name_try = id_name_obj
                        .and_then(|m| m.get(pid))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    tracing::debug!(
                        current_node_id = %cur_id_str,
                        parent_id = %pid,
                        parent_name = ?name_try,
                        "Implicit preamble: no output found for parent"
                    );
                }
            }

            // Append a CrewAI-style Context JSON block aggregating direct parent outputs
            let context_json_block = if parents_json.is_empty() {
                String::new()
            } else {
                let pretty = serde_json::to_string_pretty(&serde_json::Value::Object(parents_json))
                    .unwrap_or("{}".to_string());
                format!("\n[Context JSON]\n{}\n\n", pretty)
            };

            // Debug: summarize what we built
            tracing::debug!(
                current_node_id = %cur_id_str,
                section_count = sections.len(),
                has_context_json = !context_json_block.is_empty(),
                "Implicit preamble: built sections and JSON presence"
            );

            let implicit_preamble = if sections.is_empty() && context_json_block.is_empty() {
                // No alias sections and no context JSON -> no preamble
                "".to_string()
            } else {
                // Strong CrewAI-style directive to ensure models use the JSON context
                let directive_line = "Instruction: You MUST use the [Context JSON] below as prior outputs from your direct parents. Base your answer strictly on it.";
                let sections_block = if sections.is_empty() {
                    String::new()
                } else {
                    sections.join("\n\n") + "\n\n"
                };
                format!(
                    "Context from prior nodes (auto-injected):\n{}{}\n{}",
                    sections_block, directive_line, context_json_block
                )
            };

            let combined = format!("{}[Task]\n{}", implicit_preamble, prompt_template);
            let resolved = Self::resolve_template_variables(&combined, &ctx);
            // Debug log the resolved prompt (trimmed) to verify implicit context presence
            let preview: String = resolved.chars().take(400).collect();
            tracing::debug!(
                current_node_id = %cur_id_str,
                parent_count = parent_ids.len(),
                preview = %preview,
                "Resolved prompt preview with implicit parent context"
            );
            resolved
        };

        // Check if this node has tools configured
        let has_tools = node_config.contains_key("tool_schemas");

        // DEBUG: Log tool detection
        tracing::info!(
            "Agent tool detection - has_tools: {}, config keys: {:?}",
            has_tools,
            node_config.keys().collect::<Vec<_>>()
        );
        if let Some(tool_schemas) = node_config.get("tool_schemas") {
            tracing::info!("Tool schemas found: {}", tool_schemas);
        }

        if has_tools {
            // Execute agent with tool calling orchestration
            tracing::info!("Executing agent with tools - prompt: '{}'", resolved_prompt);
            tracing::info!("ENTERING execute_agent_with_tools function");

            let result =
                Self::execute_agent_with_tools(agent_id, &resolved_prompt, node_config, agent)
                    .await;
            tracing::info!("Agent with tools execution result: {:?}", result);
            result
        } else {
            // Execute agent without tools (original behavior)
            tracing::info!("NO TOOLS DETECTED - using standard agent execution");
            let message = AgentMessage::new(
                agent_id.clone(),
                None,
                MessageContent::Text(resolved_prompt),
            );
            agent.execute(message).await
        }
    }

    /// Execute an agent with tool calling orchestration
    async fn execute_agent_with_tools(
        _agent_id: &crate::types::AgentId,
        prompt: &str,
        node_config: &std::collections::HashMap<String, serde_json::Value>,
        agent: Arc<dyn AgentTrait>,
    ) -> GraphBitResult<serde_json::Value> {
        tracing::info!("Starting execute_agent_with_tools for agent: {}", _agent_id);
        use crate::llm::{LlmRequest, LlmTool};

        // Extract tool schemas from node config
        let tool_schemas = node_config
            .get("tool_schemas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| GraphBitError::validation("node_config", "Missing tool_schemas"))?;

        tracing::info!("Found {} tool schemas", tool_schemas.len());

        // Convert tool schemas to LlmTool objects
        let mut tools = Vec::new();
        for schema in tool_schemas {
            if let (Some(name), Some(description), Some(parameters)) = (
                schema.get("name").and_then(|v| v.as_str()),
                schema.get("description").and_then(|v| v.as_str()),
                schema.get("parameters"),
            ) {
                tools.push(LlmTool::new(name, description, parameters.clone()));
            }
        }

        // Create initial LLM request with tools
        let mut request = LlmRequest::new(prompt);
        for tool in &tools {
            request = request.with_tool(tool.clone());
        }

        tracing::info!("Created LLM request with {} tools", request.tools.len());
        for (i, tool) in request.tools.iter().enumerate() {
            tracing::info!("Tool {}: {} - {}", i, tool.name, tool.description);
        }

        // Execute LLM request directly to get tool calls
        tracing::info!(
            "About to call LLM provider with {} tools",
            request.tools.len()
        );
        let llm_response = agent.llm_provider().complete(request).await?;

        // DEBUG: Log LLM response details
        tracing::info!("LLM Response - Content: '{}'", llm_response.content);
        tracing::info!(
            "LLM Response - Tool calls count: {}",
            llm_response.tool_calls.len()
        );
        for (i, tool_call) in llm_response.tool_calls.iter().enumerate() {
            tracing::info!(
                "Tool call {}: {} with params: {:?}",
                i,
                tool_call.name,
                tool_call.parameters
            );
        }

        // Check if the LLM made any tool calls
        if !llm_response.tool_calls.is_empty() {
            tracing::info!(
                "LLM made {} tool calls - these should be executed by the Python layer",
                llm_response.tool_calls.len()
            );

            // Instead of executing tools in Rust, return a structured response that indicates
            // tool calls need to be executed by the Python layer
            let tool_calls_json = serde_json::to_value(&llm_response.tool_calls).map_err(|e| {
                GraphBitError::workflow_execution(format!("Failed to serialize tool calls: {}", e))
            })?;

            // Return a structured response that the Python layer can interpret
            Ok(serde_json::json!({
                "type": "tool_calls_required",
                "content": llm_response.content,
                "tool_calls": tool_calls_json,
                "original_prompt": prompt,
                "message": "Tool execution should be handled by Python layer with proper tool registry"
            }))
        } else {
            // No tool calls, return the original response
            tracing::info!(
                "No tool calls made by LLM, returning original response: {}",
                llm_response.content
            );
            Ok(serde_json::Value::String(llm_response.content))
        }
    }

    /// Execute a condition node (static version)
    async fn execute_condition_node_static(_expression: &str) -> GraphBitResult<serde_json::Value> {
        // Simple condition evaluation (in a real implementation, you'd use a proper expression evaluator)
        Ok(serde_json::Value::Bool(true))
    }

    /// Execute a transform node (static version)
    async fn execute_transform_node_static(
        _transformation: &str,
        _context: Arc<Mutex<WorkflowContext>>,
    ) -> GraphBitResult<serde_json::Value> {
        // Simple transformation (in a real implementation, you'd use a proper transformation engine)
        Ok(serde_json::Value::String("transformed".to_string()))
    }

    /// Execute a delay node (static version)
    async fn execute_delay_node_static(duration_seconds: u64) -> GraphBitResult<serde_json::Value> {
        tokio::time::sleep(tokio::time::Duration::from_secs(duration_seconds)).await;
        Ok(serde_json::Value::String(format!(
            "Delayed for {} seconds",
            duration_seconds
        )))
    }

    /// Execute a document loader node (static version)
    async fn execute_document_loader_node_static(
        document_type: &str,
        source_path: &str,
        _context: Arc<Mutex<WorkflowContext>>,
    ) -> GraphBitResult<serde_json::Value> {
        let loader = DocumentLoader::new();

        match loader.load_document(source_path, document_type).await {
            Ok(document_content) => {
                // Return the document content as JSON
                let content_json = serde_json::json!({
                    "source": document_content.source,
                    "document_type": document_content.document_type,
                    "content": document_content.content,
                    "metadata": document_content.metadata,
                    "file_size": document_content.file_size,
                    "extracted_at": document_content.extracted_at
                });
                Ok(content_json)
            }
            Err(e) => Err(GraphBitError::workflow_execution(format!(
                "Failed to load document: {}",
                e
            ))),
        }
    }

    /// Execute concurrent tasks with retry logic
    pub async fn execute_concurrent_tasks_with_retry<T, F, R>(
        &self,
        tasks: Vec<T>,
        task_fn: F,
        retry_config: Option<RetryConfig>,
    ) -> GraphBitResult<Vec<Result<R, GraphBitError>>>
    where
        T: Send + Clone + 'static,
        F: Fn(T) -> futures::future::BoxFuture<'static, GraphBitResult<R>>
            + Send
            + Sync
            + Clone
            + 'static,
        R: Send + 'static,
    {
        if tasks.is_empty() {
            return Ok(Vec::new());
        }

        // Create concurrent tasks with the new concurrency management system
        let task_futures: Vec<_> = tasks
            .into_iter()
            .enumerate()
            .map(|(index, task)| {
                let task_fn = task_fn.clone();
                let max_execution_time = self.max_node_execution_time_ms;
                let retry_config = retry_config.clone();
                let concurrency_manager = self.concurrency_manager.clone();

                tokio::spawn(async move {
                    // Create task info for generic concurrent tasks
                    let task_info = TaskInfo {
                        node_type: "concurrent_task".to_string(),
                        task_id: NodeId::new(), // Generate a unique task ID
                    };

                    // Acquire permits for this task
                    let _permits = concurrency_manager
                        .acquire_permits(&task_info)
                        .await
                        .map_err(|e| {
                            GraphBitError::workflow_execution(format!(
                                "Failed to acquire permits for concurrent task {}: {}",
                                index, e
                            ))
                        })?;

                    // Execute task with retry logic
                    Self::execute_task_with_retry(task, task_fn, retry_config, max_execution_time)
                        .await
                })
            })
            .collect();

        // Collect results
        let mut results = Vec::with_capacity(task_futures.len());
        let join_results = join_all(task_futures).await;

        for join_result in join_results {
            match join_result {
                Ok(task_result) => results.push(task_result),
                Err(e) => results.push(Err(GraphBitError::workflow_execution(format!(
                    "Task join failed: {}",
                    e
                )))),
            }
        }

        Ok(results)
    }

    /// Execute a single task with retry logic
    async fn execute_task_with_retry<T, F, R>(
        task: T,
        task_fn: F,
        retry_config: Option<RetryConfig>,
        max_execution_time: Option<u64>,
    ) -> Result<R, GraphBitError>
    where
        T: Send + Clone + 'static,
        F: Fn(T) -> futures::future::BoxFuture<'static, GraphBitResult<R>>
            + Send
            + Sync
            + Clone
            + 'static,
        R: Send + 'static,
    {
        let mut attempt = 0;
        let max_attempts = retry_config.as_ref().map(|c| c.max_attempts).unwrap_or(1);

        loop {
            // Clone the task for this attempt
            let task_to_execute = task.clone();

            // Execute task with optional timeout
            let result = if let Some(timeout_ms) = max_execution_time {
                let task_future = task_fn(task_to_execute);
                let timeout_duration = tokio::time::Duration::from_millis(timeout_ms);

                match tokio::time::timeout(timeout_duration, task_future).await {
                    Ok(result) => result,
                    Err(_) => Err(GraphBitError::workflow_execution(format!(
                        "Task execution timed out after {}ms",
                        timeout_ms
                    ))),
                }
            } else {
                task_fn(task_to_execute).await
            };

            match result {
                Ok(output) => return Ok(output),
                Err(error) => {
                    attempt += 1;

                    // Check if we should retry
                    if let Some(ref config) = retry_config {
                        if attempt < max_attempts && config.should_retry(&error, attempt - 1) {
                            // Calculate delay for this attempt
                            let delay_ms = config.calculate_delay(attempt - 1);
                            if delay_ms > 0 {
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                    .await;
                            }

                            // Continue the loop to retry
                            continue;
                        }
                    }

                    // No more retries, return the error
                    return Err(GraphBitError::workflow_execution(format!(
                        "Task failed after {} attempts: {}",
                        attempt, error
                    )));
                }
            }
        }
    }

    /// Execute multiple concurrent tasks efficiently
    /// This is more efficient than creating separate workflows for each task
    pub async fn execute_concurrent_tasks<T, F, R>(
        &self,
        tasks: Vec<T>,
        task_fn: F,
    ) -> GraphBitResult<Vec<Result<R, GraphBitError>>>
    where
        T: Send + Clone + 'static,
        F: Fn(T) -> futures::future::BoxFuture<'static, GraphBitResult<R>>
            + Send
            + Sync
            + Clone
            + 'static,
        R: Send + 'static,
    {
        self.execute_concurrent_tasks_with_retry(tasks, task_fn, self.default_retry_config.clone())
            .await
    }

    /// Execute concurrent agent tasks with maximum efficiency
    /// This bypasses the workflow system entirely for pure speed
    pub async fn execute_concurrent_agent_tasks(
        &self,
        prompts: Vec<String>,
        agent_id: crate::types::AgentId,
    ) -> GraphBitResult<Vec<Result<serde_json::Value, GraphBitError>>> {
        if prompts.is_empty() {
            return Ok(Vec::new());
        }

        // Ensure the agent exists
        let agent = {
            let agents_guard = self.agents.read().await;
            agents_guard.get(&agent_id).cloned()
        };

        let agent = if let Some(agent) = agent {
            agent
        } else {
            return Err(GraphBitError::workflow_execution(format!(
                "Agent {} not found. Please register the agent first.",
                agent_id
            )));
        };

        // Execute all prompts concurrently with minimal overhead
        let concurrent_tasks: Vec<_> = prompts
            .into_iter()
            .enumerate()
            .map(|(index, prompt)| {
                let agent_clone = agent.clone();
                let agent_id_clone = agent_id.clone();

                tokio::spawn(async move {
                    // Create a minimal agent message for this prompt
                    let message = crate::types::AgentMessage::new(
                        agent_id_clone.clone(),
                        None, // No specific recipient
                        crate::types::MessageContent::Text(prompt),
                    );

                    // Execute the agent task directly using the execute method for better performance
                    agent_clone.execute(message).await.map_err(|e| {
                        GraphBitError::workflow_execution(format!(
                            "Agent task {} failed: {}",
                            index, e
                        ))
                    })
                })
            })
            .collect();

        // Collect all results
        let results = futures::future::join_all(concurrent_tasks).await;
        let mut task_results = Vec::with_capacity(results.len());

        for task_result in results {
            match task_result {
                Ok(result) => task_results.push(result),
                Err(e) => task_results.push(Err(GraphBitError::workflow_execution(format!(
                    "Task join failed: {}",
                    e
                )))),
            }
        }

        Ok(task_results)
    }

    /// Helper method to collect nodes in executable order
    fn collect_executable_nodes(&self, graph: &WorkflowGraph) -> GraphBitResult<Vec<WorkflowNode>> {
        // Simple topological sort - can be enhanced for better parallelism
        let nodes: Vec<WorkflowNode> = graph.get_nodes().values().cloned().collect();
        Ok(nodes)
    }

    /// Helper method to create execution batches for optimal concurrency
    #[allow(dead_code)]
    async fn create_execution_batches(
        &self,
        nodes: Vec<WorkflowNode>,
    ) -> GraphBitResult<Vec<Vec<WorkflowNode>>> {
        // Simple batching strategy - execute all independent nodes in parallel
        // This can be enhanced with dependency analysis for better batching
        let batch_size = self.max_concurrency().await.min(nodes.len());
        let mut batches = Vec::new();

        for chunk in nodes.chunks(batch_size) {
            batches.push(chunk.to_vec());
        }

        Ok(batches)
    }

    /// Create batches that strictly respect dependencies: only direct-ready nodes per layer
    async fn create_dependency_batches(
        &self,
        graph: &WorkflowGraph,
    ) -> GraphBitResult<Vec<Vec<WorkflowNode>>> {
        use std::collections::HashSet;

        let mut graph_clone = graph.clone();
        let mut completed: HashSet<NodeId> = HashSet::new();
        let mut remaining: HashSet<NodeId> = graph_clone.get_nodes().keys().cloned().collect();
        let mut batches: Vec<Vec<WorkflowNode>> = Vec::new();

        // Iterate until all nodes are scheduled
        while !remaining.is_empty() {
            // Select nodes whose dependencies are all completed
            let mut ready_ids: Vec<NodeId> = Vec::new();
            for nid in remaining.iter() {
                let deps = graph_clone.get_dependencies(nid);
                if deps.iter().all(|d| completed.contains(d)) {
                    ready_ids.push(nid.clone());
                }
            }

            if ready_ids.is_empty() {
                // Cycle or unresolved dependency
                return Err(GraphBitError::workflow_execution(
                    "No dependency-ready nodes found; graph may be cyclic or invalid".to_string(),
                ));
            }

            // Build the batch of WorkflowNode
            let mut batch: Vec<WorkflowNode> = Vec::with_capacity(ready_ids.len());
            for nid in &ready_ids {
                if let Some(node) = graph_clone.get_nodes().get(nid) {
                    batch.push(node.clone());
                }
            }
            batches.push(batch);

            // Update completed/remaining
            for nid in ready_ids {
                completed.insert(nid.clone());
                remaining.remove(&nid);
            }
        }

        Ok(batches)
    }

    /// Create with custom concurrency configuration
    pub fn with_concurrency_config(mut self, concurrency_config: ConcurrencyConfig) -> Self {
        self.concurrency_manager = Arc::new(ConcurrencyManager::new(concurrency_config));
        self
    }
}

impl Default for WorkflowExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to extract agent IDs from a workflow
fn extract_agent_ids_from_workflow(workflow: &Workflow) -> Vec<String> {
    let mut agent_ids = std::collections::HashSet::new();

    for node in workflow.graph.get_nodes().values() {
        if let NodeType::Agent { agent_id, .. } = &node.node_type {
            agent_ids.insert(agent_id.to_string());
        }
    }

    agent_ids.into_iter().collect()
}

impl WorkflowExecutor {
    /// Resolve template variables in a string, supporting both node references and regular variables
    pub fn resolve_template_variables(template: &str, context: &WorkflowContext) -> String {
        let mut result = template.to_string();

        // Replace node references like {{node.node_id}} or {{node.node_id.property}}
        for cap in NODE_REF_PATTERN.captures_iter(template) {
            if let Some(reference) = cap.get(1) {
                let reference = reference.as_str();
                if let Some(value) = context.get_nested_output(reference) {
                    let value_str = match value {
                        serde_json::Value::String(s) => s.clone(),
                        _ => value.to_string().trim_matches('"').to_string(),
                    };
                    result = result.replace(&cap[0], &value_str);
                }
            }
        }

        // Replace simple variables for backward compatibility
        for (key, value) in &context.variables {
            let placeholder = format!("{{{}}}", key);
            if let Ok(value_str) = serde_json::to_string(value) {
                let value_str = value_str.trim_matches('"');
                result = result.replace(&placeholder, value_str);
            }
        }

        result
    }
}
