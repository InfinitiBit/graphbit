//! Workflow execution engine for `GraphBit`
//!
//! This module provides the main workflow execution capabilities,
//! orchestrating agents and managing the execution flow.

use crate::agents::AgentTrait;
use crate::document_loader::DocumentLoader;
use crate::errors::{GraphBitError, GraphBitResult};
use crate::graph::{AgentNodeConfig, NodeType, WorkflowGraph, WorkflowNode};
use crate::types::{
    AgentId, AgentMessage, CircuitBreaker, CircuitBreakerConfig, ConcurrencyConfig,
    ConcurrencyManager, ConcurrencyStats, MessageContent, NodeExecutionResult, NodeId, RetryConfig,
    TaskInfo, WorkflowContext, WorkflowExecutionStats, WorkflowId, WorkflowState,
};
use crate::{DecodeContext, EncodeContext, Enforcer};
use futures::future::join_all;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock};
use tokio::sync::{Mutex, RwLock};

static NODE_REF_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{\{node\.([a-zA-Z0-9_\-\.]+)\}\}").unwrap());

/// Snapshot passed to condition handlers: parent output plus shared workflow maps for routing.
#[derive(Debug, Clone)]
pub struct ConditionRoutingInput {
    /// Immediate parent node id (single in-edge required for condition nodes).
    pub parent_node_id: String,
    /// JSON output of the parent node (same as `WorkflowContext::get_node_output` for that id).
    pub parent_output: serde_json::Value,
    /// Clone of workflow variables at condition evaluation time.
    pub variables: HashMap<String, serde_json::Value>,
    /// Clone of all node outputs at condition evaluation time.
    pub node_outputs: HashMap<String, serde_json::Value>,
    /// Clone of execution metadata at condition evaluation time.
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Handler for conditional nodes: routing input in -> next node **name** out.
pub type ConditionalRouteFn =
    Arc<dyn Fn(ConditionRoutingInput) -> GraphBitResult<String> + Send + Sync>;

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
        // tracing::debug!("Workflow '{:#?}' validated successfully", self.graph);
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
    /// Registered agents - use `RwLock` for better read performance
    agents: Arc<RwLock<HashMap<crate::types::AgentId, Arc<dyn AgentTrait>>>>,
    /// Simplified concurrency management system
    concurrency_manager: Arc<ConcurrencyManager>,
    /// Maximum execution time per node in milliseconds
    max_node_execution_time_ms: Option<u64>,
    /// Whether to fail fast on first error or continue with other nodes
    fail_fast: bool,
    /// Default retry configuration for all nodes
    default_retry_config: Option<RetryConfig>,
    /// Circuit breakers per agent to prevent cascading failures - use `RwLock` for better performance
    circuit_breakers: Arc<RwLock<HashMap<crate::types::AgentId, CircuitBreaker>>>,
    /// Global circuit breaker configuration
    circuit_breaker_config: CircuitBreakerConfig,
    /// Default LLM configuration for auto-generated agents
    default_llm_config: Option<crate::llm::LlmConfig>,
    /// Runtime handlers for [`NodeType::Condition`] nodes (`handler_id` → callback).
    conditional_handlers: Arc<HashMap<String, ConditionalRouteFn>>,
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
            conditional_handlers: Arc::new(HashMap::new()),
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

    /// Register handlers for [`NodeType::Condition`] nodes (e.g. from Python callables).
    pub fn with_conditional_handlers(
        mut self,
        handlers: HashMap<String, ConditionalRouteFn>,
    ) -> Self {
        self.conditional_handlers = Arc::new(handlers);
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
    /// Priority: Node-level config > Executor-level config > ERROR (no defaults)
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
            }
            tracing::warn!(
                "Failed to deserialize node-level LLM config, falling back to executor config"
            );
        }

        // 2. Fall back to executor-level config (medium priority)
        if let Some(executor_config) = &self.default_llm_config {
            tracing::debug!(
                "Using executor-level LLM configuration: {:?}",
                executor_config.provider_name()
            );
            return executor_config.clone();
        }

        // 3. No default fallback - require explicit configuration as requested by user
        tracing::error!(
            "No LLM configuration found - neither node-level nor executor-level config provided. System requires explicit configuration."
        );
        crate::llm::LlmConfig::Unconfigured {
            message: "No LLM configuration provided. The system requires explicit configuration from program or user input rather than hardcoded defaults.".to_string()
        }
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

    /// Execute a workflow with enhanced performance monitoring.
    ///
    /// When `guardrail_enforcer` is `Some`, PII is encoded before each LLM call and
    /// decoded on LLM output; tool-call boundaries are handled by the executor layer.
    ///
    /// This is the non-streaming entry point. It delegates to [`execute_internal`]
    /// with no event channel, so zero overhead is added to the hot path.
    pub async fn execute(
        &self,
        workflow: Workflow,
        guardrail_enforcer: Option<Arc<Enforcer>>,
    ) -> GraphBitResult<WorkflowContext> {
        let context = WorkflowContext::new(workflow.id.clone());
        self.execute_internal(
            workflow,
            guardrail_enforcer,
            None,
            crate::stream::StreamMode::Updates,
            context,
        )
        .await
    }

    pub async fn execute_with_context(
        &self,
        workflow: Workflow,
        guardrail_enforcer: Option<Arc<Enforcer>>,
        event_tx: Option<tokio::sync::mpsc::Sender<crate::stream::StreamEvent>>,
        stream_mode: crate::stream::StreamMode,
        mut context: WorkflowContext,
    ) -> GraphBitResult<WorkflowContext> {
        self.execute_internal(
            workflow,
            guardrail_enforcer,
            event_tx,
            stream_mode,
            context,
        )
        .await
    }

    /// Execute a workflow with real-time streaming events.
    ///
    /// Identical to [`execute`] but emits [`StreamEvent`]s through `event_tx`
    /// at every node boundary. The caller consumes events from the corresponding
    /// `mpsc::Receiver` while this future runs (typically in a spawned task).
    ///
    /// # Arguments
    /// * `workflow` — the workflow to execute
    /// * `guardrail_enforcer` — optional PII masking enforcer
    /// * `event_tx` — sender half of an `mpsc` channel for streaming events
    /// * `stream_mode` — controls which event categories are emitted
    ///
    /// # Fire-and-forget semantics
    /// If the receiver is dropped (e.g. the caller stops iterating early),
    /// send failures are silently ignored — the workflow still runs to completion.
    pub async fn execute_streaming(
        &self,
        workflow: Workflow,
        guardrail_enforcer: Option<Arc<Enforcer>>,
        event_tx: tokio::sync::mpsc::Sender<crate::stream::StreamEvent>,
        stream_mode: crate::stream::StreamMode,
    ) -> GraphBitResult<WorkflowContext> {
        let context = WorkflowContext::new(workflow.id.clone());
        self.execute_internal(workflow, guardrail_enforcer, Some(event_tx), stream_mode, context)
            .await
    }

    /// Shared execution engine used by both [`execute`] and [`execute_streaming`].
    ///
    /// When `event_tx` is `Some`, node-level [`StreamEvent`]s are emitted at every
    /// milestone. When `None` (non-streaming path) no channel operations occur, so
    /// performance is identical to the original `execute()`.
    async fn execute_internal(
        &self,
        workflow: Workflow,
        guardrail_enforcer: Option<Arc<Enforcer>>,
        event_tx: Option<tokio::sync::mpsc::Sender<crate::stream::StreamEvent>>,
        stream_mode: crate::stream::StreamMode,
        mut context: WorkflowContext,
    ) -> GraphBitResult<WorkflowContext> {
        use crate::stream::{StreamEvent, error_type_from_graphbit_error, error_type_from_string};

        let start_time = std::time::Instant::now();

        // Set initial workflow state
        context.state = WorkflowState::Running {
            current_node: NodeId::new(),
        };

        // Validate workflow before execution
        if let Err(e) = workflow.validate() {
            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(StreamEvent::WorkflowFailed {
                        error: e.to_string(),
                        error_type: error_type_from_graphbit_error(&e),
                    })
                    .await;
            }
            return Err(e);
        }

        // Validate unique LLM configurations once per workflow (deduped by config fingerprint).
        // This prevents validating the same key/provider repeatedly during agent creation.
        {
            // If we are re-entering execution (e.g. tool-resolution downstream reruns), avoid
            // re-validating LLM configs again. This flag is workflow-run scoped and does not
            // persist outside the current execution context.
            let already_validated = context
                .metadata
                .get("workflow_llm_configs_validated")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if already_validated {
                tracing::info!("Workflow LLM validation: already validated; skipping");
            } else {
            use crate::llm::{LlmProvider, LlmProviderFactory, LlmRequest};
            use std::collections::HashMap;

            let mut unique: HashMap<String, crate::llm::LlmConfig> = HashMap::with_capacity(4);
            for node in workflow.graph.get_nodes().values() {
                if matches!(node.node_type, NodeType::Agent { .. }) {
                    let resolved = self.resolve_llm_config_for_node(&node.config);
                    if let Some(fp) = resolved.validation_fingerprint() {
                        unique.entry(fp).or_insert(resolved);
                    }
                }
            }

            tracing::info!(
                "Workflow LLM validation: {} unique LLM config(s) across {} node(s)",
                unique.len(),
                workflow.graph.node_count()
            );

            // If there is only one unique LLM configuration across the workflow, skip proactive
            // validation entirely (fail fast on the first real LLM call instead).
            //
            // If there are multiple unique configurations, validate them concurrently before any
            // node execution to avoid N sequential round trips.
            if unique.len() <= 1 {
                tracing::info!(
                    "Workflow LLM validation: {} unique config; skipping proactive validation (fail-fast mode)",
                    unique.len()
                );
            } else {
                tracing::info!(
                    "Workflow LLM validation: {} unique configs detected; validating concurrently",
                    unique.len()
                );
                let configs: Vec<crate::llm::LlmConfig> = unique.into_values().collect();
                let validations = configs.into_iter().map(|cfg| async move {
                    let provider_name = cfg.provider_name().to_string();
                    let model_name = cfg.model_name().to_string();

                    tracing::info!(
                        "Workflow LLM validation: validating provider={} model={}",
                        provider_name,
                        model_name
                    );

                    let provider = LlmProviderFactory::create_provider(cfg.clone())?;
                    let llm_provider = LlmProvider::new(provider, cfg);
                    let test_request = LlmRequest::new("Hello");
                    llm_provider.complete(test_request).await?;

                    Ok::<(), GraphBitError>(())
                });

                let results = futures::future::join_all(validations).await;
                let mut failures: Vec<String> = Vec::new();
                for r in results {
                    if let Err(e) = r {
                        // Do not include raw API keys here; provider errors already mask keys.
                        failures.push(e.to_string());
                    }
                }

                if !failures.is_empty() {
                    let err = GraphBitError::config(format!(
                        "LLM configuration validation failed for {} config(s): {}",
                        failures.len(),
                        failures.join(" | ")
                    ));
                    if let Some(ref tx) = event_tx {
                        let _ = tx
                            .send(StreamEvent::WorkflowFailed {
                                error: err.to_string(),
                                error_type: error_type_from_graphbit_error(&err),
                            })
                            .await;
                    }
                    return Err(err);
                }
            }

            context.set_metadata(
                "workflow_llm_configs_validated".to_string(),
                serde_json::Value::Bool(true),
            );
            }
        }


        // PERFORMANCE FIX: Auto-register agents for all agent nodes found in workflow
        let agent_ids = extract_agent_ids_from_workflow(&workflow);
        if agent_ids.is_empty() {
            let err = GraphBitError::validation("workflow", "No agents found in workflow");
            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(StreamEvent::WorkflowFailed {
                        error: err.to_string(),
                        error_type: error_type_from_graphbit_error(&err),
                    })
                    .await;
            }
            return Err(err);
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
                    // Find the node configuration for this agent to extract system_prompt, temperature, max_tokens, and LLM config
                    let mut system_prompt = String::new();
                    let mut temperature: Option<f32> = None;
                    let mut max_tokens: Option<u32> = None;
                    let mut resolved_llm_config = self.default_llm_config.clone()
                        .unwrap_or_else(|| crate::llm::LlmConfig::Unconfigured {
                            message: "No LLM configuration provided for agent creation. Please explicitly configure an LLM provider.".to_string()
                        });

                    for node in workflow.graph.get_nodes().values() {
                        if let NodeType::Agent { config } = &node.node_type {
                            if config.agent_id == agent_id {
                                // Extract system_prompt: Priority is AgentNodeConfig.system_prompt_override > node.config["system_prompt"]
                                if let Some(sys_override) = &config.system_prompt_override {
                                    system_prompt = sys_override.clone();
                                } else if let Some(prompt_value) = node.config.get("system_prompt")
                                {
                                    if let Some(prompt_str) = prompt_value.as_str() {
                                        system_prompt = prompt_str.to_string();
                                    }
                                }

                                // Extract temperature from node config if available
                                if let Some(temp_value) = node.config.get("temperature") {
                                    if let Some(temp_num) = temp_value.as_f64() {
                                        temperature = Some(temp_num as f32);
                                    }
                                }

                                // Extract max_tokens from node config if available
                                if let Some(max_tokens_value) = node.config.get("max_tokens") {
                                    if let Some(max_tokens_num) = max_tokens_value.as_u64() {
                                        max_tokens = Some(max_tokens_num as u32);
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
                        format!("Agent_{agent_id_str}"),
                        "Auto-generated agent for workflow execution",
                        resolved_llm_config,
                    )
                    .with_id(agent_id.clone());

                    // Set system prompt if found in node configuration
                    if !system_prompt.is_empty() {
                        default_config = default_config.with_system_prompt(system_prompt);
                    }

                    // Set temperature if found in node configuration
                    if let Some(temp) = temperature {
                        default_config = default_config.with_temperature(temp);
                    }

                    // Set max_tokens if found in node configuration
                    if let Some(tokens) = max_tokens {
                        default_config = default_config.with_max_tokens(tokens);
                    }

                    // Try to create agent - if it fails due to config issues, fail the workflow
                    match crate::agents::Agent::new(default_config).await {
                        Ok(agent) => {
                            let mut agents_guard = self.agents.write().await;
                            agents_guard.insert(agent_id.clone(), Arc::new(agent));
                            tracing::debug!("Auto-registered agent: {agent_id}");
                        }
                        Err(e) => {
                            let err = GraphBitError::workflow_execution(format!(
                                "Failed to create agent '{agent_id_str}': {e}. This may be due to invalid API key or configuration.",
                            ));
                            if let Some(ref tx) = event_tx {
                                let _ = tx
                                    .send(StreamEvent::WorkflowFailed {
                                        error: err.to_string(),
                                        error_type: error_type_from_graphbit_error(&err),
                                    })
                                    .await;
                            }
                            return Err(err);
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

        let nodes = Self::collect_executable_nodes(&workflow.graph)?;
        if nodes.is_empty() {
            context.complete();
            // Streaming: emit WorkflowCompleted even for an empty workflow
            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(StreamEvent::WorkflowCompleted {
                        context: context.clone(),
                    })
                    .await;
            }
            return Ok(context);
        }

        // ── Streaming: emit WorkflowStarted ──────────────────────────────────────
        if let Some(ref tx) = event_tx {
            let _ = tx
                .send(StreamEvent::WorkflowStarted {
                    workflow_id: workflow.id.to_string(),
                    workflow_name: workflow.name.clone(),
                    total_nodes: nodes.len(),
                })
                .await;
        }

        let mut total_executed = 0;
        let mut total_successful = 0;

        // Parent map from the canonical `edges` list (same source Python `connect` uses).
        // Using `get_dependencies` + petgraph/cache here has regressed to empty dependency lists
        // for some graphs, which makes `parents.iter().all(...)` vacuously true and schedules
        // condition successors (e.g. all advisor branches) in the same batch as the intake node.
        let node_ids: Vec<NodeId> = workflow.graph.get_nodes().keys().cloned().collect();
        let mut node_parents: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for nid in &node_ids {
            let deps: Vec<NodeId> = workflow
                .graph
                .get_edges()
                .iter()
                .filter_map(|(from, to, _)| (to == nid).then_some(from.clone()))
                .collect();
            node_parents.insert(nid.clone(), deps);
        }
        let node_parents = Arc::new(node_parents);
        let conditional_handlers = self.conditional_handlers.clone();
        let workflow_graph = Arc::new(workflow.graph.clone());

        let total_node_count = workflow.graph.node_count();
        let mut resolved: HashSet<NodeId> = HashSet::new();
        let mut skipped: HashSet<NodeId> = HashSet::new();
        // Streaming-only coordination: nodes that emitted `tool_calls_required` are treated as
        // pending until the Python streaming layer resolves them and performs a downstream rerun.
        let mut pending_tool_resolution: HashSet<NodeId> = HashSet::new();

        while resolved.len() + skipped.len() < total_node_count {
            let ready: Vec<WorkflowNode> = workflow
                .graph
                .get_nodes()
                .iter()
                .filter(|(id, _)| {
                    !resolved.contains(*id)
                        && !skipped.contains(*id)
                        && !pending_tool_resolution.contains(*id)
                })
                .filter(|(id, _)| {
                    node_parents
                        .get(*id)
                        .map(|parents| {
                            parents
                                .iter()
                                .all(|p| resolved.contains(p) || skipped.contains(p))
                        })
                        .unwrap_or(false)
                })
                .map(|(_, n)| n.clone())
                .collect();

            if ready.is_empty() {
                if event_tx.is_some() && !pending_tool_resolution.is_empty() {
                    let blocked_node_ids: Vec<String> = workflow
                        .graph
                        .get_nodes()
                        .iter()
                        .filter(|(id, _)| {
                            !resolved.contains(*id)
                                && !skipped.contains(*id)
                                && !pending_tool_resolution.contains(*id)
                        })
                        .map(|(id, _)| id.to_string())
                        .collect();
                    tracing::info!(
                        blocked_node_ids = ?blocked_node_ids,
                        "Deferring downstream nodes blocked on unresolved tool calls"
                    );
                    break;
                }
                let err_msg = "No runnable nodes but workflow not finished (cycle or invalid scheduling state)".to_string();
                if let Some(ref tx) = event_tx {
                    let _ = tx
                        .send(StreamEvent::WorkflowFailed {
                            error: err_msg.clone(),
                            error_type: "runtime_error".to_string(),
                        })
                        .await;
                }
                return Err(GraphBitError::workflow_execution(err_msg));
            }

            let batch_size = ready.len();
            let batch_ids: Vec<String> = ready.iter().map(|n| n.id.to_string()).collect();
            tracing::info!(batch_size, batch_node_ids = ?batch_ids, "Executing dynamic batch");

            // ── Streaming: emit NodeStarted for each node about to run ────────────
            if let Some(ref tx) = event_tx {
                for node in &ready {
                    let _ = tx
                        .send(StreamEvent::NodeStarted {
                            node_id: node.id.to_string(),
                            node_name: node.name.clone(),
                        })
                        .await;
                }
            }

            let shared_context = Arc::new(Mutex::new(context));
            let mut tasks = Vec::with_capacity(batch_size);

            for node in ready {
                let context_clone = shared_context.clone();
                let agents_clone = self.agents.clone();
                let circuit_breakers_clone = self.circuit_breakers.clone();
                let circuit_breaker_config = self.circuit_breaker_config.clone();
                let retry_config = self.default_retry_config.clone();
                let concurrency_manager = self.concurrency_manager.clone();
                let guardrail_enforcer = guardrail_enforcer.clone();
                let node_parents = node_parents.clone();
                let conditional_handlers = conditional_handlers.clone();
                let workflow_graph = workflow_graph.clone();
                // Clone the event channel sender for this task (cheap Arc clone inside Sender)
                let task_event_tx = event_tx.clone();
                let task_stream_mode = stream_mode;

                let task = tokio::spawn(async move {
                    let task_info = TaskInfo::from_node_type(&node.node_type, &node.id);

                    let _permits = if matches!(node.node_type, NodeType::Agent { .. }) {
                        Some(
                            concurrency_manager
                                .acquire_permits(&task_info)
                                .await
                                .map_err(|e| {
                                    GraphBitError::workflow_execution(format!(
                                        "Failed to acquire permits for node {}: {e}",
                                        node.id
                                    ))
                                })?,
                        )
                    } else {
                        None
                    };

                    Self::execute_node_with_retry(
                        node,
                        context_clone,
                        agents_clone,
                        circuit_breakers_clone,
                        circuit_breaker_config,
                        retry_config,
                        guardrail_enforcer,
                        node_parents,
                        conditional_handlers,
                        workflow_graph,
                        task_event_tx,
                        task_stream_mode,
                    )
                    .await
                });
                tasks.push(task);
            }

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
                        let node_requires_tool_resolution = event_tx.is_some()
                            && Self::is_tool_calls_required_output(&node_result.output);
                        if node_requires_tool_resolution {
                            pending_tool_resolution.insert(node_result.node_id.clone());
                        } else {
                            resolved.insert(node_result.node_id.clone());
                        }

                        // ── Streaming: emit NodeCompleted or NodeFailed ───────────────────
                        if let Some(ref tx) = event_tx {
                            let node_name = workflow
                                .graph
                                .get_node(&node_result.node_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();

                            if node_result.success {
                                let _ = tx
                                    .send(StreamEvent::NodeCompleted {
                                        node_id: node_result.node_id.to_string(),
                                        node_name,
                                        output: node_result.output.clone(),
                                    })
                                    .await;
                            } else {
                                let error_msg = node_result
                                    .error
                                    .as_deref()
                                    .unwrap_or("Unknown error")
                                    .to_string();
                                let _ = tx
                                    .send(StreamEvent::NodeFailed {
                                        node_id: node_result.node_id.to_string(),
                                        node_name,
                                        error: error_msg.clone(),
                                        error_type: error_type_from_string(&error_msg),
                                    })
                                    .await;
                            }
                        }

                        {
                            let mut ctx = shared_context.lock().await;
                            if let Some(node) = workflow.graph.get_node(&node_result.node_id) {
                                ctx.set_node_output(&node.id, node_result.output.clone());
                                ctx.set_node_output_by_name(&node.name, node_result.output.clone());

                                let keys_now: Vec<String> =
                                    ctx.node_outputs.keys().cloned().collect();
                                tracing::debug!(
                                    stored_node_id = %node.id,
                                    stored_node_name = %node.name,
                                    node_output_keys_now = ?keys_now,
                                    "Stored node output in context.node_outputs"
                                );

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
                            } else if let Ok(output_str) =
                                serde_json::to_string(&node_result.output)
                            {
                                ctx.set_variable(
                                    format!("node_result_{total_executed}"),
                                    serde_json::Value::String(output_str),
                                );
                                tracing::debug!(
                                    executed_index = total_executed,
                                    "Stored output under generic variable name (node not found)"
                                );
                            }
                        }

                        if node_result.success {
                            if let Some(node) = workflow.graph.get_node(&node_result.node_id) {
                                if matches!(node.node_type, NodeType::Condition { .. }) {
                                    match Self::condition_output_branch_name(&node_result.output) {
                                        Some(chosen_name) => {
                                            match Self::resolve_condition_branch_target(
                                                workflow_graph.as_ref(),
                                                &node.id,
                                                &chosen_name,
                                            ) {
                                                Ok(chosen_id) => {
                                                    Self::expand_skips_from_condition(
                                                        workflow_graph.as_ref(),
                                                        &node.id,
                                                        &chosen_id,
                                                        &resolved,
                                                        &mut skipped,
                                                    );
                                                }
                                                Err(e) => {
                                                    should_fail_fast = true;
                                                    failure_message = format!(
                                                        "Condition '{}': handler chose branch {:?} but it does not match exactly one direct successor node name: {}",
                                                        node.name, chosen_name, e
                                                    );
                                                }
                                            }
                                        }
                                        None => {
                                            should_fail_fast = true;
                                            failure_message = format!(
                                                "Condition '{}': output must be a non-empty branch node name (String); got output={:?}",
                                                node.name, node_result.output
                                            );
                                        }
                                    }
                                }
                            } else {
                                tracing::warn!(
                                    node_id = %node_result.node_id,
                                    "Workflow batch merge: executed node id not found in graph"
                                );
                                should_fail_fast = true;
                                failure_message = format!(
                                    "Workflow batch merge: executed node id {} not found in graph",
                                    node_result.node_id
                                );
                            }
                        } else if let Some(node) = workflow.graph.get_node(&node_result.node_id) {
                            // A failed condition leaves every successor "ready" (parent resolved) but
                            // never runs expand_skips_from_condition, so all branches would execute.
                            // Treat condition failure like an unrecoverable routing error.
                            if matches!(node.node_type, NodeType::Condition { .. }) {
                                should_fail_fast = true;
                                failure_message = node_result.error.clone().unwrap_or_else(|| {
                                    format!(
                                        "Condition '{}' failed (no branch chosen; successors were not skipped)",
                                        node.name
                                    )
                                });
                            }
                        }

                        if should_fail_fast {
                            break;
                        }
                    }
                    Ok(Err(e)) => {
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
                            failure_message = format!("Task execution failed: {e}");
                            break;
                        }
                        total_executed += 1;
                    }
                }
            }

            if should_fail_fast {
                let mut ctx = shared_context.lock().await;
                ctx.fail(failure_message.clone());
                drop(ctx);
                let final_ctx = Arc::try_unwrap(shared_context).unwrap().into_inner();

                // ── Streaming: emit WorkflowFailed on fail-fast ──────────────────
                if let Some(ref tx) = event_tx {
                    let error_type = error_type_from_string(&failure_message);
                    let _ = tx
                        .send(StreamEvent::WorkflowFailed {
                            error: failure_message,
                            error_type,
                        })
                        .await;
                }

                return Ok(final_ctx);
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

        // ── Streaming: emit WorkflowCompleted ────────────────────────────────────
        if let Some(ref tx) = event_tx {
            let _ = tx
                .send(StreamEvent::WorkflowCompleted {
                    context: context.clone(),
                })
                .await;
        }

        Ok(context)
    }

    /// Validates `chosen_name` matches exactly one direct successor of the condition node.
    fn resolve_condition_branch_target(
        graph: &WorkflowGraph,
        condition_id: &NodeId,
        chosen_name: &str,
    ) -> GraphBitResult<NodeId> {
        let children = graph.direct_successors(condition_id);
        if children.is_empty() {
            return Err(GraphBitError::workflow_execution(
                "Condition node has no outgoing edges to route to".to_string(),
            ));
        }
        let mut matches: Vec<NodeId> = Vec::new();
        for cid in children {
            if let Some(n) = graph.get_node(&cid) {
                if n.name.trim() == chosen_name.trim() {
                    matches.push(cid);
                }
            }
        }
        match matches.len() {
            0 => Err(GraphBitError::workflow_execution(format!(
                "Condition routing: no direct successor named '{chosen_name}'"
            ))),
            1 => Ok(matches[0].clone()),
            _ => Err(GraphBitError::workflow_execution(format!(
                "Condition routing: ambiguous successor name '{chosen_name}'"
            ))),
        }
    }

    /// Mark nodes on non-chosen branches as skipped (diamond-join safe using `R_chosen`).
    fn expand_skips_from_condition(
        graph: &WorkflowGraph,
        condition_id: &NodeId,
        chosen_id: &NodeId,
        resolved: &HashSet<NodeId>,
        skipped: &mut HashSet<NodeId>,
    ) {
        let r_chosen = graph.forward_reachable_from(chosen_id);
        for bad_id in graph.direct_successors(condition_id) {
            if &bad_id == chosen_id {
                continue;
            }
            for v in graph.forward_reachable_from(&bad_id) {
                if !r_chosen.contains(&v) && !resolved.contains(&v) {
                    skipped.insert(v);
                }
            }
        }
    }

    /// Stringify a parent node's output for `Condition` handler input.
    /// Extract the next-node name from a condition node's stored output value.
    fn condition_output_branch_name(output: &serde_json::Value) -> Option<String> {
        match output {
            serde_json::Value::String(s) => {
                let t = s.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            }
            other => other
                .as_str()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from),
        }
    }

    async fn execute_condition_node(
        node: &WorkflowNode,
        handler_id: &str,
        context: Arc<Mutex<WorkflowContext>>,
        parents_map: Arc<HashMap<NodeId, Vec<NodeId>>>,
        handlers: Arc<HashMap<String, ConditionalRouteFn>>,
        graph: Arc<WorkflowGraph>,
    ) -> GraphBitResult<serde_json::Value> {
        let parents = parents_map.get(&node.id).cloned().unwrap_or_default();
        if parents.len() != 1 {
            return Err(GraphBitError::workflow_execution(format!(
                "Condition node '{}' must have exactly one incoming dependency, found {}",
                node.name,
                parents.len()
            )));
        }
        let parent_id = parents[0].clone();
        let parent_key = parent_id.to_string();
        let routing_input = {
            let ctx = context.lock().await;
            let parent_output = ctx.get_node_output(&parent_key).ok_or_else(|| {
                GraphBitError::workflow_execution(format!(
                    "Condition node '{}': parent output not found for {}",
                    node.name, parent_key
                ))
            })?;
            ConditionRoutingInput {
                parent_node_id: parent_key.clone(),
                parent_output: parent_output.clone(),
                variables: ctx.variables.clone(),
                node_outputs: ctx.node_outputs.clone(),
                metadata: ctx.metadata.clone(),
            }
        };
        let handler = handlers.get(handler_id).ok_or_else(|| {
            GraphBitError::workflow_execution(format!(
                "Condition node '{}': no handler registered for handler_id '{handler_id}'",
                node.name
            ))
        })?;
        let out = handler(routing_input)?;
        let trimmed = out.trim().to_string();
        if trimmed.is_empty() {
            return Err(GraphBitError::workflow_execution(format!(
                "Condition handler returned an empty next-node name for node '{}'",
                node.name
            )));
        }
        Self::resolve_condition_branch_target(graph.as_ref(), &node.id, &trimmed)?;
        Ok(serde_json::Value::String(trimmed))
    }

    fn is_tool_calls_required_output(value: &serde_json::Value) -> bool {
        if let Some(obj) = value.as_object() {
            if let Some(ty) = obj.get("type").and_then(|v| v.as_str()) {
                return ty == "tool_calls_required";
            }
        }
        if let Some(s) = value.as_str() {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(s) {
                return Self::is_tool_calls_required_output(&parsed);
            }
        }
        false
    }

    /// Execute a node with retry logic and circuit breaker
    async fn execute_node_with_retry(
        node: WorkflowNode,
        context: Arc<Mutex<WorkflowContext>>,
        agents: Arc<RwLock<HashMap<crate::types::AgentId, Arc<dyn AgentTrait>>>>,
        circuit_breakers: Arc<RwLock<HashMap<crate::types::AgentId, CircuitBreaker>>>,
        circuit_breaker_config: CircuitBreakerConfig,
        retry_config: Option<RetryConfig>,
        guardrail_enforcer: Option<Arc<Enforcer>>,
        node_parents: Arc<HashMap<NodeId, Vec<NodeId>>>,
        conditional_handlers: Arc<HashMap<String, ConditionalRouteFn>>,
        workflow_graph: Arc<WorkflowGraph>,
        event_tx: Option<tokio::sync::mpsc::Sender<crate::stream::StreamEvent>>,
        stream_mode: crate::stream::StreamMode,
    ) -> GraphBitResult<NodeExecutionResult> {
        let start_time = std::time::Instant::now();
        let mut attempt = 0;

        // Get circuit breaker for agent nodes
        let mut circuit_breaker = if let NodeType::Agent { config } = &node.node_type {
            let agent_id = &config.agent_id;
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

        // Check whether this node already has a final resolved output in the provided context.
        // This avoids rerunning unchanged resolved nodes during the downstream rerun pass.
        if let Some(existing_output) = {
            let ctx = context.lock().await;
            ctx.get_node_output(&node.id.to_string()).cloned()
        } {
            if !Self::is_tool_calls_required_output(&existing_output) {
                tracing::debug!(node_id = %node.id, node_name = %node.name, "Skipping already-resolved node execution");
                return Ok(NodeExecutionResult::success(existing_output, node.id.clone())
                    .with_duration(start_time.elapsed().as_millis() as u64)
                    .with_retry_count(attempt));
            }
        }

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
                NodeType::Agent { config } => {
                    Self::execute_agent_node_static(
                        &node.id,
                        config,
                        &node.config,
                        context.clone(),
                        agents.clone(),
                        guardrail_enforcer.clone(),
                        event_tx.clone(),
                        stream_mode,
                    )
                    .await
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
                NodeType::Condition { handler_id } => {
                    Self::execute_condition_node(
                        &node,
                        handler_id,
                        context.clone(),
                        node_parents.clone(),
                        conditional_handlers.clone(),
                        workflow_graph.clone(),
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
                        if let NodeType::Agent { config } = &node.node_type {
                            let agent_id = &config.agent_id;
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
                        if let NodeType::Agent { config } = &node.node_type {
                            let agent_id = &config.agent_id;
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

    /// Execute an agent node (static version).
    /// When `guardrail_enforcer` is `Some`, encodes prompt before LLM and decodes response after.
    /// When `stream_mode.emits_tokens()` and the provider supports streaming, emits `Token`
    /// events per chunk via `event_tx` and accumulates into a full response. Falls back to
    /// `complete()` if the provider does not support streaming.
    async fn execute_agent_node_static(
        current_node_id: &NodeId,
        agent_node_config: &AgentNodeConfig,
        node_config: &std::collections::HashMap<String, serde_json::Value>,
        context: Arc<Mutex<WorkflowContext>>,
        agents: Arc<RwLock<HashMap<crate::types::AgentId, Arc<dyn AgentTrait>>>>,
        guardrail_enforcer: Option<Arc<Enforcer>>,
        event_tx: Option<tokio::sync::mpsc::Sender<crate::stream::StreamEvent>>,
        stream_mode: crate::stream::StreamMode,
    ) -> GraphBitResult<serde_json::Value> {
        let agent_id = &agent_node_config.agent_id;
        let prompt_template = &agent_node_config.prompt_template;
        let conversational_context = agent_node_config.conversational_context.as_deref();
        // Use read lock for better performance
        let agents_guard = agents.read().await;
        let agent = agents_guard
            .get(agent_id)
            .ok_or_else(|| GraphBitError::agent_not_found(agent_id.to_string()))?
            .clone();
        drop(agents_guard); // Release the lock early

        // Build implicit preamble from upstream (parent) node outputs, then resolve templates
        let (resolved_prompt, resolved_context, metadata_input_raw) = {
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
                        .filter_map(|v| v.as_str().map(str::to_string))
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

            // Preserve order by iterating parent_ids, using id->name for titles, and fetching outputs by key
            for pid in &parent_ids {
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

                    // Clean, natural format: just add the output value directly
                    sections.push(value_str.clone());

                    // Always add to JSON context by id as a fallback key
                    parents_json.insert(pid.to_string(), value.clone());

                    // Also add to JSON context by name if available
                    if let Some(parent_name) = id_name_obj
                        .and_then(|m| m.get(pid))
                        .and_then(|v| v.as_str())
                    {
                        parents_json.insert(parent_name.to_string(), value.clone());
                    }
                } else {
                    // Debug: could not find value for this parent id/name
                    let name_try = id_name_obj
                        .and_then(|m| m.get(pid))
                        .and_then(|v| v.as_str())
                        .map(str::to_string);
                    tracing::debug!(
                        current_node_id = %cur_id_str,
                        parent_id = %pid,
                        parent_name = ?name_try,
                        "Implicit preamble: no output found for parent"
                    );
                }
            }

            // Debug: summarize what we built
            tracing::debug!(
                current_node_id = %cur_id_str,
                section_count = sections.len(),
                "Implicit preamble: built sections"
            );

            // Build clean, natural prompt with context
            let implicit_preamble = if sections.is_empty() {
                // No prior outputs -> no preamble
                "".to_string()
            } else {
                // Clean format: just include the prior outputs naturally
                sections.join("\n\n") + "\n\n"
            };

            let combined_for_llm = format!("{implicit_preamble}{prompt_template}");
            let resolved_prompt = Self::resolve_template_variables(&combined_for_llm, &ctx);

            let resolved_context = conversational_context
                .map(|ctx_template| Self::resolve_template_variables(ctx_template, &ctx));

            // Use only the prompt part for cleaner metadata logging
            let metadata_input_raw = Self::resolve_template_variables(prompt_template, &ctx);

            // Debug log the resolved prompt (trimmed)
            let preview: String = resolved_prompt.chars().take(400).collect();
            tracing::debug!(
                current_node_id = %cur_id_str,
                parent_count = parent_ids.len(),
                preview = %preview,
                "Resolved prompt preview with implicit parent context"
            );
            (resolved_prompt, resolved_context, metadata_input_raw)
        };

        // Check if this node has tools configured
        let has_tools = node_config.contains_key("tool_schemas");

        // DEBUG: Log tool detection
        tracing::info!(
            "Agent tool detection - has_tools: {has_tools}, config keys: {:?}",
            node_config.keys().collect::<Vec<_>>()
        );
        if let Some(tool_schemas) = node_config.get("tool_schemas") {
            tracing::info!("Tool schemas found: {tool_schemas}");
        }

        if has_tools {
            // Execute agent with tool calling orchestration
            tracing::info!("Executing agent with tools - prompt: '{resolved_prompt}'");
            tracing::info!("ENTERING execute_agent_with_tools function");

            // Get node name from context
            let node_name = {
                let ctx = context.lock().await;
                ctx.metadata
                    .get("node_id_to_name")
                    .and_then(|m| m.as_object())
                    .and_then(|m| m.get(&current_node_id.to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string()
            };

            let result = Self::execute_agent_with_tools(
                agent_id,
                agent_node_config,
                &resolved_prompt,
                resolved_context.as_deref(),
                &metadata_input_raw,
                node_config,
                agent,
                current_node_id,
                &node_name,
                context.clone(),
                guardrail_enforcer.clone(),
                event_tx.clone(),
                stream_mode,
            )
            .await;
            tracing::info!("Agent with tools execution result: {:?}", result);
            result
        } else {
            // Execute agent without tools (original behavior)
            tracing::info!("NO TOOLS DETECTED - using standard agent execution");

            // Build the executions array for metadata
            let mut executions: Vec<serde_json::Value> = Vec::new();

            // Guardrail: encode context and prompt separately before sending to LLM
            let mut masked_input_for_meta = metadata_input_raw.clone();
            let prompt_for_llm = if let Some(ref enforcer) = guardrail_enforcer {
                tracing::debug!("Guardrail: encoding prompt and context before LLM call");

                // 1. Encode context if present
                let (masked_context, signature_ctx, ctx_rules, ctx_count) = if let Some(ctx) =
                    resolved_context
                {
                    let enc = enforcer.encode(serde_json::Value::String(ctx), EncodeContext::Llm);
                    (
                        enc.payload.as_str().unwrap_or_default().to_string(),
                        enc.signature_injection_text,
                        enc.rule_names,
                        enc.rules_applied_count,
                    )
                } else {
                    (String::new(), String::new(), Vec::new(), 0)
                };

                // 2. Encode prompt
                let enc_prompt = enforcer.encode(
                    serde_json::Value::String(resolved_prompt.clone()),
                    EncodeContext::Llm,
                );

                // 3. Encode metadata input specifically for clean logging
                let enc_meta = enforcer.encode(
                    serde_json::Value::String(metadata_input_raw.clone()),
                    EncodeContext::Llm,
                );
                masked_input_for_meta = enc_meta.payload.as_str().unwrap_or_default().to_string();

                // Cumulative Metadata: Combine rule names from context and prompt
                let mut all_rule_names = ctx_rules;
                for rule in enc_prompt.rule_names {
                    if !all_rule_names.contains(&rule) {
                        all_rule_names.push(rule);
                    }
                }

                // Record guardrail encode execution entry with cumulative stats
                executions.push(serde_json::json!({
                    "type": "guardrail_policy",
                    "operation": "encode",
                    "pii_rules_applied_count": ctx_count + enc_prompt.rules_applied_count,
                    "pii_rule_names": all_rule_names,
                    "policy_name": enc_prompt.policy_name
                }));

                // Combine for LLM: Use the prompt's signature (or context's if prompt doesn't have one, though it should)
                let final_signature = if !enc_prompt.signature_injection_text.is_empty() {
                    enc_prompt.signature_injection_text
                } else {
                    signature_ctx
                };

                format!(
                    "{}{}{}",
                    final_signature,
                    if !masked_context.is_empty() {
                        format!("{}\n\n", masked_context)
                    } else {
                        String::new()
                    },
                    enc_prompt.payload.as_str().unwrap_or("")
                )
            } else {
                if let Some(ctx) = resolved_context {
                    format!("{}\n\n{}", ctx, resolved_prompt)
                } else {
                    resolved_prompt.clone()
                }
            };

            // Call LLM provider directly to capture metadata
            use crate::llm::{LlmMessage, LlmRequest};

            // Resolve system prompt: Priority is Node Override > Agent Default
            let system_prompt =
                if let Some(sys_override) = &agent_node_config.system_prompt_override {
                    Some(sys_override.clone())
                } else if !agent.config().system_prompt.is_empty() {
                    Some(agent.config().system_prompt.clone())
                } else {
                    None
                };

            // Build messages array
            let mut messages = Vec::with_capacity(2);
            if let Some(content) = system_prompt {
                messages.push(LlmMessage::system(content));
            }
            messages.push(LlmMessage::user(prompt_for_llm.clone()));

            let mut request = LlmRequest::with_messages(messages);

            // Apply node-level configuration overrides (temperature, max_tokens, etc.)
            if let Some(temp_value) = node_config.get("temperature") {
                if let Some(temp_num) = temp_value.as_f64() {
                    request = request.with_temperature(temp_num as f32);
                }
            }

            if let Some(max_tokens_value) = node_config.get("max_tokens") {
                if let Some(max_tokens_num) = max_tokens_value.as_u64() {
                    request = request.with_max_tokens(max_tokens_num as u32);
                }
            }

            // Enable Anthropic prompt caching when configured on the node
            if node_config.get("enable_prompt_caching").and_then(|v| v.as_bool()).unwrap_or(false) {
                request = request.with_prompt_caching(true);
            }

            // Measure LLM call duration and capture execution timestamp
            let execution_timestamp = chrono::Utc::now();

            // Emit live LLM lifecycle events for non-tool agent execution.
            let llm_call_id_hint = format!("{}-llm-1", current_node_id);
            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(crate::stream::StreamEvent::LlmCallStarted {
                        node_id: current_node_id.to_string(),
                        node_name: {
                            let ctx = context.lock().await;
                            ctx.metadata
                                .get("node_id_to_name")
                                .and_then(|m| m.as_object())
                                .and_then(|m| m.get(&current_node_id.to_string()))
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string()
                        },
                        llm_call_id: llm_call_id_hint.clone(),
                        iteration: 1,
                        model: agent.llm_provider().config().model_name().to_string(),
                    })
                    .await;
            }

            tracing::info!("Final LLM Prompt: \n{}", prompt_for_llm);
            
            let llm_start = std::time::Instant::now();

            // ── Token-level streaming (stream_mode = Messages | All) ──────────────
            // When the mode requests tokens AND the provider supports streaming,
            // drive the stream and emit a Token event per chunk, accumulating the
            // full content for metadata/context. Falls back to complete() transparently.
            let llm_response = if stream_mode.emits_tokens()
                && event_tx.is_some()
                && agent.llm_provider().provider().supports_streaming()
            {
                use crate::stream::StreamEvent;
                use futures::StreamExt;

                // Get the node name for Token events (best-effort, doesn't block)
                let token_node_name = {
                    let ctx = context.lock().await;
                    ctx.metadata
                        .get("node_id_to_name")
                        .and_then(|m| m.as_object())
                        .and_then(|m| m.get(&current_node_id.to_string()))
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                        .unwrap_or_else(|| "unknown".to_string())
                };

                // Drive the streaming response
                // Pre-clone the request for the empty-stream fallback before moving into stream()
                let fallback_request = request.clone();
                let mut stream = agent.llm_provider().stream(request).await?;

                // Accumulate content and keep the last full LlmResponse for metadata
                let mut accumulated_content = String::new();
                let mut last_response: Option<crate::llm::LlmResponse> = None;

                while let Some(chunk_result) = stream.next().await {
                    let chunk = chunk_result?;

                    // Only emit non-empty content chunks as Token events
                    if !chunk.content.is_empty() {
                        if let Some(ref tx) = event_tx {
                            let _ = tx
                                .send(StreamEvent::Token {
                                    node_id: current_node_id.to_string(),
                                    node_name: token_node_name.clone(),
                                    llm_call_id: llm_call_id_hint.clone(),
                                    content: chunk.content.clone(),
                                })
                                .await;
                        }
                        accumulated_content.push_str(&chunk.content);
                    }
                    last_response = Some(chunk);
                }

                // Build a complete LlmResponse from the accumulated stream
                match last_response {
                    Some(mut final_resp) => {
                        // Streamed text tokens; keep provider `content` when no deltas had text
                        // (e.g. tool-only streams still set placeholder + tool_calls on the last chunk).
                        if !accumulated_content.is_empty() {
                            final_resp.content = accumulated_content;
                        }
                        final_resp
                    }
                    None => {
                        // Empty stream — fall back to complete() to get a proper response
                        tracing::warn!(
                            node_id = %current_node_id,
                            "LLM stream returned 0 chunks; falling back to complete()"
                        );
                        agent.llm_provider().complete(fallback_request).await?
                    }
                }
            } else {
                // Standard (non-streaming) path — identical to previous behaviour
                agent.llm_provider().complete(request).await?
            };

            let llm_duration_ms = llm_start.elapsed().as_secs_f64() * 1000.0;
            let llm_end_timestamp = chrono::Utc::now();

            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(crate::stream::StreamEvent::LlmCallCompleted {
                        node_id: current_node_id.to_string(),
                        node_name: {
                            let ctx = context.lock().await;
                            ctx.metadata
                                .get("node_id_to_name")
                                .and_then(|m| m.as_object())
                                .and_then(|m| m.get(&current_node_id.to_string()))
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string()
                        },
                        llm_call_id: llm_response.id.clone().unwrap_or(llm_call_id_hint),
                        iteration: 1,
                        finish_reason: format!("{}", llm_response.finish_reason),
                        output: Self::append_tool_calls_to_llm_output(
                            &llm_response.content,
                            &llm_response.tool_calls,
                        ),
                        duration_ms: llm_duration_ms,
                    })
                    .await;
            }

            // Get provider name for metadata
            let provider_name = agent.llm_provider().config().provider_name().to_string();

            // Capture raw LLM content before decode for metadata
            let raw_llm_content = llm_response.content.clone();
            if guardrail_enforcer.is_some() {
                tracing::debug!(
                    "[GuardRail] raw LLM response (before decode): content={:?}, tool_calls={:?}",
                    llm_response.content,
                    llm_response.tool_calls
                );
            }

            // Build the llm_call execution entry (before decode, captures raw LLM output)
            let llm_call_entry = serde_json::json!({
                "type": "llm_call",
                "id": llm_response.id.clone().unwrap_or_default(),
                "model": llm_response.model,
                "provider": provider_name,
                "input": if guardrail_enforcer.is_some() { masked_input_for_meta.clone() } else { metadata_input_raw.clone() },
                "output": llm_response.content,
                "finish_reason": format!("{}", llm_response.finish_reason),
                "tool_calls": [],
                "start_time": execution_timestamp.to_rfc3339(),
                "end_time": llm_end_timestamp.to_rfc3339(),
                "duration_ms": llm_duration_ms,
                "usage": {
                    "prompt_tokens": llm_response.usage.prompt_tokens,
                    "completion_tokens": llm_response.usage.completion_tokens,
                    "total_tokens": llm_response.usage.total_tokens,
                    "prompt_tokens_details": {
                        "cached_tokens": llm_response.usage.cache_read_tokens.unwrap_or(0),
                        "cache_creation_tokens": llm_response.usage.cache_creation_tokens.unwrap_or(0),
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
            });
            executions.push(llm_call_entry);

            // Guardrail: decode LLM output before storing in context
            let llm_response = if let Some(ref enforcer) = guardrail_enforcer {
                tracing::debug!(
                    "Guardrail: decoding LLM response (rehydrating for context) llm_response.content: {}",
                    llm_response.content
                );
                tracing::debug!("Guardrail: decoding LLM response (rehydrating for context)");
                let payload = serde_json::json!({
                    "content": llm_response.content,
                    "tool_calls": llm_response.tool_calls
                });
                let decoded_result = enforcer.decode(payload, DecodeContext::LlmResponse);
                tracing::debug!("Guardrail: LLM response decoded");

                // Record guardrail decode execution entry
                executions.push(serde_json::json!({
                    "type": "guardrail_policy",
                    "operation": "rehydrate",
                    "pii_rules_applied_count": decoded_result.rules_applied_count,
                    "pii_rule_names": decoded_result.rule_names,
                    "policy_name": decoded_result.policy_name
                }));

                let content = decoded_result
                    .payload
                    .get("content")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_else(|| llm_response.content.clone());
                let tool_calls = decoded_result
                    .payload
                    .get("tool_calls")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_else(|| llm_response.tool_calls.clone());
                crate::llm::LlmResponse {
                    content,
                    tool_calls,
                    ..llm_response
                }
            } else {
                llm_response
            };

            // Build the node-level metadata with executions array
            {
                // First, get the node name before mutable borrow
                let node_name = {
                    let ctx = context.lock().await;
                    ctx.metadata
                        .get("node_id_to_name")
                        .and_then(|m| m.as_object())
                        .and_then(|m| m.get(&current_node_id.to_string()))
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                        .unwrap_or_else(|| "unknown".to_string())
                };

                let max_iterations = node_config
                    .get("max_iterations")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(5) as u32;

                let node_metadata = serde_json::json!({
                    "node_id": current_node_id.to_string(),
                    "node_name": node_name,
                    "node_type": "Agent",
                    // When GR active: user_input = masked prompt, final_output = raw LLM content
                    // When GR inactive: user_input = original prompt, final_output = decoded content
                    "user_input": if guardrail_enforcer.is_some() { masked_input_for_meta.clone() } else { metadata_input_raw.clone() },
                    "tools_available": [],
                    "total_tools_available": 0,
                    "start_time": execution_timestamp.to_rfc3339(),
                    "end_time": llm_end_timestamp.to_rfc3339(),
                    "duration_ms": llm_duration_ms,
                    "success": true,
                    "error": serde_json::Value::Null,
                    "final_output": if guardrail_enforcer.is_some() { raw_llm_content } else { llm_response.content.clone() },
                    "total_iterations": 0,
                    "max_iterations": max_iterations,
                    "exit_reason": llm_response.finish_reason,
                    "total_usage": {
                        "prompt_tokens": llm_response.usage.prompt_tokens,
                        "completion_tokens": llm_response.usage.completion_tokens,
                        "total_tokens": llm_response.usage.total_tokens,
                        "prompt_tokens_details": {
                            "cached_tokens": llm_response.usage.cache_read_tokens.unwrap_or(0),
                            "cache_creation_tokens": llm_response.usage.cache_creation_tokens.unwrap_or(0),
                            "audio_tokens": 0
                        },
                        "completion_tokens_details": {
                            "reasoning_tokens": 0,
                            "audio_tokens": 0,
                            "accepted_prediction_tokens": 0,
                            "rejected_prediction_tokens": 0
                        }
                    },
                    "total_tool_calls": 0,
                    "total_retries": 0,
                    "tools_used": [],
                    "executions": executions
                });

                // Now store the metadata
                let mut ctx = context.lock().await;
                // Store by node ID
                ctx.metadata.insert(
                    format!("node_response_{current_node_id}"),
                    node_metadata.clone(),
                );
                // Store by node name
                ctx.metadata
                    .insert(format!("node_response_{node_name}"), node_metadata);
            }

            // Return the content as JSON value
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&llm_response.content)
            {
                Ok(json_value)
            } else {
                Ok(serde_json::Value::String(llm_response.content))
            }
        }
    }

    /// Execute an agent with tool calling orchestration.
    /// When `guardrail_enforcer` is `Some`, encodes prompt before LLM and decodes response after.
    /// When `stream_mode.emits_tool_events()` and `event_tx` is `Some`, emits `ToolCallStarted`
    /// for each tool call requested by the LLM before returning the `tool_calls_required` response
    /// to the Python layer (which handles actual execution and must emit `ToolCallCompleted` /
    /// `ToolCallFailed`).
    ///
    /// When `stream_mode.emits_tokens()`, `event_tx` is `Some`, and the provider supports
    /// streaming, the initial tool-selection LLM call uses [`LlmProvider::stream`] and emits
    /// [`crate::stream::StreamEvent::Token`] per chunk; otherwise that call uses
    /// [`LlmProvider::complete`] (same as non-streaming `execute()`).
    async fn execute_agent_with_tools(
        _agent_id: &crate::types::AgentId,
        agent_node_config: &AgentNodeConfig,
        prompt: &str,
        conversational_context: Option<&str>,
        metadata_input: &str,
        node_config: &std::collections::HashMap<String, serde_json::Value>,
        agent: Arc<dyn AgentTrait>,
        node_id: &NodeId,
        node_name: &str,
        context: Arc<Mutex<WorkflowContext>>,
        guardrail_enforcer: Option<Arc<Enforcer>>,
        event_tx: Option<tokio::sync::mpsc::Sender<crate::stream::StreamEvent>>,
        stream_mode: crate::stream::StreamMode,
    ) -> GraphBitResult<serde_json::Value> {
        tracing::info!("Starting execute_agent_with_tools for agent: {_agent_id}");
        use crate::llm::{LlmMessage, LlmRequest, LlmTool};

        // ... (rest of tool calling logic, but with LlmRequest::with_messages handled correctly below)

        // Build the executions array for metadata
        let mut executions: Vec<serde_json::Value> = Vec::new();

        // Guardrail: encode prompt and context individually before sending to LLM
        let mut masked_input_for_meta = metadata_input.to_string();
        let prompt_for_llm = if let Some(ref enforcer) = guardrail_enforcer {
            tracing::debug!("Guardrail: encoding prompt and context before LLM call (tool path)");

            // 1. Encode context if present
            let (masked_context, signature_ctx, ctx_rules, ctx_count) =
                if let Some(ctx) = conversational_context {
                    let enc = enforcer.encode(
                        serde_json::Value::String(ctx.to_string()),
                        EncodeContext::Llm,
                    );
                    (
                        enc.payload.as_str().unwrap_or_default().to_string(),
                        enc.signature_injection_text,
                        enc.rule_names,
                        enc.rules_applied_count,
                    )
                } else {
                    (String::new(), String::new(), Vec::new(), 0)
                };

            // 2. Encode prompt
            let enc_prompt = enforcer.encode(
                serde_json::Value::String(prompt.to_string()),
                EncodeContext::Llm,
            );

            // 3. Encode metadata input specifically
            let enc_meta = enforcer.encode(
                serde_json::Value::String(metadata_input.to_string()),
                EncodeContext::Llm,
            );
            masked_input_for_meta = enc_meta.payload.as_str().unwrap_or_default().to_string();

            // Cumulative Metadata
            let mut all_rule_names = ctx_rules;
            for rule in enc_prompt.rule_names {
                if !all_rule_names.contains(&rule) {
                    all_rule_names.push(rule);
                }
            }

            // Record guardrail encode execution entry
            executions.push(serde_json::json!({
                "type": "guardrail_policy",
                "operation": "encode",
                "pii_rules_applied_count": ctx_count + enc_prompt.rules_applied_count,
                "pii_rule_names": all_rule_names,
                "policy_name": enc_prompt.policy_name
            }));

            // Combine for LLM
            let final_signature = if !enc_prompt.signature_injection_text.is_empty() {
                enc_prompt.signature_injection_text
            } else {
                signature_ctx
            };

            format!(
                "{}{}{}",
                final_signature,
                if !masked_context.is_empty() {
                    format!("{}\n\n", masked_context)
                } else {
                    String::new()
                },
                enc_prompt.payload.as_str().unwrap_or_default()
            )
        } else {
            if let Some(ctx) = conversational_context {
                format!("{}\n\n{}", ctx, prompt)
            } else {
                prompt.to_string()
            }
        };

        // Extract tool schemas from node config
        let tool_schemas = node_config
            .get("tool_schemas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| GraphBitError::validation("node_config", "Missing tool_schemas"))?;

        tracing::info!("Found {} tool schemas", tool_schemas.len());

        // Collect tool names for metadata
        let tool_names: Vec<String> = tool_schemas
            .iter()
            .filter_map(|s| s.get("name").and_then(|v| v.as_str()).map(String::from))
            .collect();

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

        // Resolve system prompt: Priority is Node Override > Agent Default
        let system_prompt = if let Some(sys_override) = &agent_node_config.system_prompt_override {
            Some(sys_override.clone())
        } else if !agent.config().system_prompt.is_empty() {
            Some(agent.config().system_prompt.clone())
        } else {
            None
        };

        // Create initial LLM request with tools (using encoded prompt when guardrail is active)
        let mut messages = Vec::with_capacity(2);
        if let Some(content) = system_prompt {
            messages.push(LlmMessage::system(content));
        }
        messages.push(LlmMessage::user(prompt_for_llm.clone()));

        let mut request = LlmRequest::with_messages(messages);
        for tool in &tools {
            request = request.with_tool(tool.clone());
        }

        // Apply node-level configuration overrides (temperature, max_tokens, top_p)
        // This ensures the initial tool selection LLM call respects node configuration
        if let Some(temp_value) = node_config.get("temperature") {
            if let Some(temp_num) = temp_value.as_f64() {
                request = request.with_temperature(temp_num as f32);
                tracing::debug!("Applied temperature={} to tool selection request", temp_num);
            }
        }

        if let Some(max_tokens_value) = node_config.get("max_tokens") {
            if let Some(max_tokens_num) = max_tokens_value.as_u64() {
                request = request.with_max_tokens(max_tokens_num as u32);
                tracing::debug!(
                    "Applied max_tokens={} to tool selection request",
                    max_tokens_num
                );
            }
        }

        if let Some(top_p_value) = node_config.get("top_p") {
            if let Some(top_p_num) = top_p_value.as_f64() {
                request = request.with_top_p(top_p_num as f32);
                tracing::debug!("Applied top_p={} to tool selection request", top_p_num);
            }
        }

        // Enable Anthropic prompt caching when configured on the node
        if node_config.get("enable_prompt_caching").and_then(|v| v.as_bool()).unwrap_or(false) {
            request = request.with_prompt_caching(true);
            tracing::debug!("Applied enable_prompt_caching=true to tool selection request");
        }

        tracing::info!("Created LLM request with {} tools", request.tools.len());
        for (i, tool) in request.tools.iter().enumerate() {
            tracing::info!("Tool {i}: {} - {}", tool.name, tool.description);
        }

        // Execute LLM request directly to get tool calls
        tracing::info!(
            "About to call LLM provider with {} tools",
            request.tools.len()
        );

        // Emit live LLM call lifecycle events for the initial tool-selection call.
        let initial_llm_call_id = format!("{}-llm-1", node_id);
        if let Some(ref tx) = event_tx {
            let _ = tx
                .send(crate::stream::StreamEvent::LlmCallStarted {
                    node_id: node_id.to_string(),
                    node_name: node_name.to_string(),
                    llm_call_id: initial_llm_call_id.clone(),
                    iteration: 1,
                    model: agent.llm_provider().config().model_name().to_string(),
                })
                .await;
        }

        // Measure LLM call duration and capture execution timestamp
        let execution_timestamp = chrono::Utc::now();
        let llm_start = std::time::Instant::now();
        // Token streaming only when explicitly requested and a stream channel exists — matches
        // the no-tools agent path. Non-streaming `execute()` uses `event_tx: None` / Updates mode.
        let mut llm_response = if stream_mode.emits_tokens()
            && event_tx.is_some()
            && agent.llm_provider().provider().supports_streaming()
        {
            use crate::stream::StreamEvent;
            use futures::StreamExt;

            let token_node_name = node_name.to_string();
            let fallback_request = request.clone();
            let mut stream = agent.llm_provider().stream(request).await?;

            let mut accumulated_content = String::new();
            let mut last_response: Option<crate::llm::LlmResponse> = None;

            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result?;

                if !chunk.content.is_empty() {
                    if let Some(ref tx) = event_tx {
                        let _ = tx
                            .send(StreamEvent::Token {
                                node_id: node_id.to_string(),
                                node_name: token_node_name.clone(),
                                llm_call_id: initial_llm_call_id.clone(),
                                content: chunk.content.clone(),
                            })
                            .await;
                    }
                    accumulated_content.push_str(&chunk.content);
                }
                last_response = Some(chunk);
            }

            match last_response {
                Some(mut final_resp) => {
                    if !accumulated_content.is_empty() {
                        final_resp.content = accumulated_content;
                    }
                    final_resp
                }
                None => {
                    tracing::warn!(
                        node_id = %node_id,
                        "LLM stream returned 0 chunks (tool path); falling back to complete()"
                    );
                    agent.llm_provider().complete(fallback_request).await?
                }
            }
        } else {
            agent.llm_provider().complete(request).await?
        };
        let llm_duration_ms = llm_start.elapsed().as_secs_f64() * 1000.0;
        let llm_end_timestamp = chrono::Utc::now();

        if let Some(ref tx) = event_tx {
            let _ = tx
                .send(crate::stream::StreamEvent::LlmCallCompleted {
                    node_id: node_id.to_string(),
                    node_name: node_name.to_string(),
                    llm_call_id: llm_response.id.clone().unwrap_or(initial_llm_call_id),
                    iteration: 1,
                    finish_reason: format!("{}", llm_response.finish_reason),
                    output: Self::append_tool_calls_to_llm_output(
                        &llm_response.content,
                        &llm_response.tool_calls,
                    ),
                    duration_ms: llm_duration_ms,
                })
                .await;
        }

        // Get provider name for metadata
        let provider_name = agent.llm_provider().config().provider_name().to_string();

        // Capture raw LLM content before decode for metadata
        let raw_llm_content = llm_response.content.clone();
        if guardrail_enforcer.is_some() {
            tracing::debug!(
                "[GuardRail] raw LLM response (before decode): content={:?}, tool_calls={:?}",
                llm_response.content,
                llm_response.tool_calls
            );
        }

        // Build tool_calls array for the llm_call execution entry.
        // When GuardRail is on, store encoded (masked) parameters so metadata never exposes PII.
        let llm_tool_calls_for_metadata: Vec<serde_json::Value> = llm_response
            .tool_calls
            .iter()
            .map(|tc| {
                let params_for_meta = if let Some(ref enforcer) = guardrail_enforcer {
                    let enc = enforcer.encode(tc.parameters.clone(), EncodeContext::Llm);
                    if enc.payload.is_object() {
                        enc.payload
                    } else {
                        tc.parameters.clone()
                    }
                } else {
                    tc.parameters.clone()
                };
                let args_str = serde_json::to_string(&params_for_meta).unwrap_or_default();
                serde_json::json!({
                    "id": tc.id,
                    "type": "function",
                    "name": tc.name,
                    "parameters": params_for_meta,
                    "function": {
                        "name": tc.name,
                        "arguments": args_str
                    }
                })
            })
            .collect();

        // Build the llm_call execution entry
        let llm_call_entry = serde_json::json!({
            "type": "llm_call",
            "id": llm_response.id.clone().unwrap_or_default(),
            "model": llm_response.model,
            "provider": provider_name,
            "input": if guardrail_enforcer.is_some() { masked_input_for_meta.clone() } else { metadata_input.to_string() },
            "output": llm_response.content,
            "finish_reason": format!("{}", llm_response.finish_reason),
            "tool_calls": llm_tool_calls_for_metadata,
            "start_time": execution_timestamp.to_rfc3339(),
            "end_time": llm_end_timestamp.to_rfc3339(),
            "duration_ms": llm_duration_ms,
            "usage": {
                "prompt_tokens": llm_response.usage.prompt_tokens,
                "completion_tokens": llm_response.usage.completion_tokens,
                "total_tokens": llm_response.usage.total_tokens,
                "prompt_tokens_details": {
                    "cached_tokens": llm_response.usage.cache_read_tokens.unwrap_or(0),
                    "cache_creation_tokens": llm_response.usage.cache_creation_tokens.unwrap_or(0),
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
        });
        executions.push(llm_call_entry);

        // Guardrail: decode LLM output before storing in context
        if let Some(ref enforcer) = guardrail_enforcer {
            tracing::debug!(
                "Guardrail: decoding LLM response (tool path; rehydrating for context) llm_response.content: {}",
                llm_response.content
            );
            tracing::debug!(
                "Guardrail: decoding LLM response (tool path; rehydrating for context)"
            );
            let payload = serde_json::json!({
                "content": llm_response.content,
                "tool_calls": llm_response.tool_calls
            });
            let decoded_result = enforcer.decode(payload, DecodeContext::LlmResponse);
            tracing::debug!("Guardrail: LLM response decoded");

            // Record guardrail decode execution entry
            executions.push(serde_json::json!({
                "type": "guardrail_policy",
                "operation": "rehydrate",
                "pii_rules_applied_count": decoded_result.rules_applied_count,
                "pii_rule_names": decoded_result.rule_names,
                "policy_name": decoded_result.policy_name
            }));

            if let Some(c) = decoded_result
                .payload
                .get("content")
                .and_then(|v| v.as_str())
            {
                llm_response.content = c.to_string();
            }
            if let Some(tc) = decoded_result.payload.get("tool_calls") {
                if let Ok(parsed) = serde_json::from_value(tc.clone()) {
                    llm_response.tool_calls = parsed;
                }
            }
        }

        // Build the initial node-level metadata with executions array
        // The Python layer (handle_tool_calls_in_context) will extend this with tool_call and subsequent llm_call entries
        let max_iterations = node_config
            .get("max_iterations")
            .and_then(|v| v.as_u64())
            .unwrap_or(5) as u32;

        let node_metadata = serde_json::json!({
            "node_id": node_id.to_string(),
            "node_name": node_name,
            "node_type": "Agent",
            // When GR active: user_input = masked prompt, final_output = raw LLM content
            // When GR inactive: user_input = original prompt, final_output = decoded content
            "user_input": if guardrail_enforcer.is_some() { masked_input_for_meta.clone() } else { metadata_input.to_string() },
            "tools_available": tool_names,
            "total_tools_available": tool_names.len(),
            "start_time": execution_timestamp.to_rfc3339(),
            "end_time": llm_end_timestamp.to_rfc3339(),
            "duration_ms": llm_duration_ms,
            "success": true,
            "error": serde_json::Value::Null,
            "final_output": if guardrail_enforcer.is_some() { raw_llm_content.clone() } else { llm_response.content.clone() },
            "total_iterations": 0,
            "max_iterations": max_iterations,
            "exit_reason": format!("{}", llm_response.finish_reason),
            "total_usage": {
                "prompt_tokens": llm_response.usage.prompt_tokens,
                "completion_tokens": llm_response.usage.completion_tokens,
                "total_tokens": llm_response.usage.total_tokens,
                "prompt_tokens_details": {
                    "cached_tokens": llm_response.usage.cache_read_tokens.unwrap_or(0),
                    "cache_creation_tokens": llm_response.usage.cache_creation_tokens.unwrap_or(0),
                    "audio_tokens": 0
                },
                "completion_tokens_details": {
                    "reasoning_tokens": 0,
                    "audio_tokens": 0,
                    "accepted_prediction_tokens": 0,
                    "rejected_prediction_tokens": 0
                }
            },
            "total_tool_calls": 0,
            "total_retries": 0,
            "tools_used": [],
            "executions": executions
        });

        // Store the metadata
        {
            let mut ctx = context.lock().await;
            ctx.metadata
                .insert(format!("node_response_{node_id}"), node_metadata.clone());
            ctx.metadata
                .insert(format!("node_response_{node_name}"), node_metadata);
        }

        // DEBUG: Log LLM response details
        tracing::info!("LLM Response - Content: '{}'", llm_response.content);
        tracing::info!(
            "LLM Response - Tool calls count: {}",
            llm_response.tool_calls.len()
        );
        for (i, tool_call) in llm_response.tool_calls.iter().enumerate() {
            tracing::info!(
                "Tool call {i}: {} with params: {:?}",
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

            // ── Streaming: emit ToolCallStarted per requested tool call ──────────
            // Parameters are masked when guardrail is active (use the encoded params
            // from the metadata array which was built with encoding applied above).
            if let Some(ref tx) = event_tx {
                for tool_call in &llm_response.tool_calls {
                    // Mask parameters if guardrail is active
                    let params_for_event = if let Some(ref enforcer) = guardrail_enforcer {
                        let enc = enforcer.encode(tool_call.parameters.clone(), EncodeContext::Llm);
                        if enc.payload.is_object() {
                            enc.payload
                        } else {
                            tool_call.parameters.clone()
                        }
                    } else {
                        tool_call.parameters.clone()
                    };

                    let _ = tx
                        .send(crate::stream::StreamEvent::ToolCallStarted {
                            node_id: node_id.to_string(),
                            node_name: node_name.to_string(),
                            tool_name: tool_call.name.clone(),
                            tool_call_id: tool_call.id.clone(),
                            parameters: params_for_event,
                        })
                        .await;
                }
            }

            // Instead of executing tools in Rust, return a structured response that indicates
            // tool calls need to be executed by the Python layer
            let tool_calls_json = serde_json::to_value(&llm_response.tool_calls).map_err(|e| {
                GraphBitError::workflow_execution(format!("Failed to serialize tool calls: {e}"))
            })?;

            // Return a structured response that the Python layer can interpret.
            // When GuardRail is on, pass only the encoded payload (without the RULE signature
            // injection text) so the executor can reconstruct the final prompt cleanly.
            // The executor will re-encode the final prompt (adding a fresh RULE prefix) before
            // the second LLM call; including the RULE here would cause it to appear in metadata.
            let original_prompt_for_response = guardrail_enforcer
                .as_ref()
                .map(|_| masked_input_for_meta.clone())
                .unwrap_or_else(|| metadata_input.to_string());
            Ok(serde_json::json!({
                "type": "tool_calls_required",
                "content": llm_response.content,
                "tool_calls": tool_calls_json,
                "original_prompt": original_prompt_for_response,
                "initial_tokens_used": llm_response.usage.completion_tokens,
                "max_tokens_configured": node_config.get("max_tokens").and_then(|v| v.as_u64()),
                "node_id": node_id.to_string(),
                "node_name": node_name.to_string(),
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
                "Failed to load document: {e}",
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
                                "Failed to acquire permits for concurrent task {index}: {e}",
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
                    "Task join failed: {e}",
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
                        "Task execution timed out after {timeout_ms}ms",
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
                        "Task failed after {attempt} attempts: {error}",
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
                "Agent {agent_id} not found. Please register the agent first.",
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
                    let message = AgentMessage::new(
                        agent_id_clone.clone(),
                        None, // No specific recipient
                        MessageContent::Text(prompt),
                    );

                    // Execute the agent task directly using the execute method for better performance
                    agent_clone.execute(message).await.map_err(|e| {
                        GraphBitError::workflow_execution(
                            format!("Agent task {index} failed: {e}",),
                        )
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
                    "Task join failed: {e}"
                )))),
            }
        }

        Ok(task_results)
    }

    /// Helper method to collect nodes in executable order
    fn collect_executable_nodes(graph: &WorkflowGraph) -> GraphBitResult<Vec<WorkflowNode>> {
        // Simple topological sort - can be enhanced for better parallelism
        let nodes: Vec<WorkflowNode> = graph.get_nodes().values().cloned().collect();
        Ok(nodes)
    }

    /// Format LLM output for streaming lifecycle events.
    /// If tool calls are present, append them so function-call fragments are visible
    /// in `llm_call_completed.output` without introducing a new event type.
    fn append_tool_calls_to_llm_output(
        content: &str,
        tool_calls: &[crate::llm::LlmToolCall],
    ) -> String {
        if tool_calls.is_empty() {
            return content.to_string();
        }

        let tool_calls_json =
            serde_json::to_string(tool_calls).unwrap_or_else(|_| "[]".to_string());
        if content.contains("[tool_calls]") {
            return content.to_string();
        }
        if content.trim().is_empty() {
            format!("[tool_calls] {tool_calls_json}")
        } else {
            format!("{content}\n[tool_calls] {tool_calls_json}")
        }
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
        if let NodeType::Agent { config } = &node.node_type {
            agent_ids.insert(config.agent_id.to_string());
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
            let placeholder = format!("{{{key}}}");
            if let Ok(value_str) = serde_json::to_string(value) {
                let value_str = value_str.trim_matches('"');
                result = result.replace(&placeholder, value_str);
            }
        }

        result
    }
}
