//! Workflow executor for orchestrating workflow execution.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

use crate::agents::r#trait::AgentTrait;
use crate::errors::{GraphBitError, GraphBitResult};
use crate::graph::{NodeType, WorkflowNode};
use crate::llm::LlmConfig;
use crate::types::{
    AgentId, AgentMessage, CircuitBreaker, CircuitBreakerConfig, ConcurrencyConfig,
    ConcurrencyManager, ConcurrencyStats, MessageContent, NodeExecutionResult, NodeId,
    RetryConfig, TaskInfo, WorkflowContext, WorkflowExecutionStats, WorkflowState,
};
use futures::future::join_all;

use super::concurrent::execute_concurrent_tasks_with_retry;
use super::helpers::{create_dependency_batches, extract_agent_ids_from_workflow};
use super::node_execution::{
    execute_agent_node, execute_condition_node, execute_delay_node, execute_document_loader_node,
    execute_transform_node,
};
use crate::workflow::Workflow;

/// Workflow execution engine
pub struct WorkflowExecutor {
    agents: Arc<RwLock<HashMap<AgentId, Arc<dyn AgentTrait>>>>,
    concurrency_manager: Arc<ConcurrencyManager>,
    max_node_execution_time_ms: Option<u64>,
    fail_fast: bool,
    default_retry_config: Option<RetryConfig>,
    circuit_breakers: Arc<RwLock<HashMap<AgentId, CircuitBreaker>>>,
    circuit_breaker_config: CircuitBreakerConfig,
    default_llm_config: Option<LlmConfig>,
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
            fail_fast: true,
            default_retry_config: None,
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
    pub fn with_default_llm_config(mut self, llm_config: LlmConfig) -> Self {
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
    fn resolve_llm_config_for_node(
        &self,
        node_config: &HashMap<String, serde_json::Value>,
    ) -> LlmConfig {
        if let Some(node_llm_config) = node_config.get("llm_config") {
            if let Ok(config) = serde_json::from_value::<LlmConfig>(node_llm_config.clone()) {
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

        if let Some(executor_config) = &self.default_llm_config {
            tracing::debug!(
                "Using executor-level LLM configuration: {:?}",
                executor_config.provider_name()
            );
            return executor_config.clone();
        }

        tracing::error!("No LLM configuration found - neither node-level nor executor-level config provided. System requires explicit configuration.");
        LlmConfig::Unconfigured {
            message: "No LLM configuration provided. The system requires explicit configuration from program or user input rather than hardcoded defaults.".to_string()
        }
    }

    /// Get or create circuit breaker for an agent
    async fn get_circuit_breaker(&self, agent_id: &AgentId) -> CircuitBreaker {
        {
            let breakers = self.circuit_breakers.read().await;
            if let Some(breaker) = breakers.get(agent_id) {
                return breaker.clone();
            }
        }

        let mut breakers = self.circuit_breakers.write().await;
        breakers
            .entry(agent_id.clone())
            .or_insert_with(|| CircuitBreaker::new(self.circuit_breaker_config.clone()))
            .clone()
    }

    /// Get current concurrency limit
    pub async fn max_concurrency(&self) -> usize {
        let _stats = self.concurrency_manager.get_stats().await;
        let permits = self.concurrency_manager.get_available_permits().await;
        permits.get("global").copied().unwrap_or(16)
    }

    /// Get available permits in semaphore
    pub async fn available_permits(&self) -> HashMap<String, usize> {
        self.concurrency_manager.get_available_permits().await
    }

    /// Execute a workflow with enhanced performance monitoring
    pub async fn execute(&self, workflow: Workflow) -> GraphBitResult<WorkflowContext> {
        let start_time = std::time::Instant::now();

        let mut context = WorkflowContext::new(workflow.id.clone());
        context.state = WorkflowState::Running {
            current_node: NodeId::new(),
        };

        workflow.validate()?;

        let agent_ids = extract_agent_ids_from_workflow(&workflow);
        if agent_ids.is_empty() {
            return Err(GraphBitError::validation(
                "workflow",
                "No agents found in workflow",
            ));
        }

        // Auto-register missing agents
        for agent_id_str in &agent_ids {
            if let Ok(agent_id) = AgentId::from_string(agent_id_str) {
                let agent_exists = {
                    let agents_guard = self.agents.read().await;
                    agents_guard.contains_key(&agent_id)
                };

                if !agent_exists {
                    let mut system_prompt = String::new();
                    let mut temperature: Option<f32> = None;
                    let mut max_tokens: Option<u32> = None;
                    let mut resolved_llm_config = self.default_llm_config.clone()
                        .unwrap_or_else(|| LlmConfig::Unconfigured {
                            message: "No LLM configuration provided for agent creation. Please explicitly configure an LLM provider.".to_string()
                        });

                    for node in workflow.graph.get_nodes().values() {
                        if let NodeType::Agent {
                            agent_id: node_agent_id,
                            ..
                        } = &node.node_type
                        {
                            if node_agent_id == &agent_id {
                                if let Some(prompt_value) = node.config.get("system_prompt") {
                                    if let Some(prompt_str) = prompt_value.as_str() {
                                        system_prompt = prompt_str.to_string();
                                    }
                                }

                                if let Some(temp_value) = node.config.get("temperature") {
                                    if let Some(temp_num) = temp_value.as_f64() {
                                        temperature = Some(temp_num as f32);
                                    }
                                }

                                if let Some(max_tokens_value) = node.config.get("max_tokens") {
                                    if let Some(max_tokens_num) = max_tokens_value.as_u64() {
                                        max_tokens = Some(max_tokens_num as u32);
                                    }
                                }

                                resolved_llm_config = self.resolve_llm_config_for_node(&node.config);
                                break;
                            }
                        }
                    }

                    let mut default_config = crate::agents::config::AgentConfig::new(
                        format!("Agent_{agent_id_str}"),
                        "Auto-generated agent for workflow execution",
                        resolved_llm_config,
                    )
                    .with_id(agent_id.clone());

                    if !system_prompt.is_empty() {
                        default_config = default_config.with_system_prompt(system_prompt);
                    }

                    if let Some(temp) = temperature {
                        default_config = default_config.with_temperature(temp);
                    }

                    if let Some(tokens) = max_tokens {
                        default_config = default_config.with_max_tokens(tokens);
                    }

                    match crate::agents::agent::Agent::new(default_config).await {
                        Ok(agent) => {
                            let mut agents_guard = self.agents.write().await;
                            agents_guard.insert(agent_id.clone(), Arc::new(agent));
                            tracing::debug!("Auto-registered agent: {agent_id}");
                        }
                        Err(e) => {
                            return Err(GraphBitError::workflow_execution(format!(
                                "Failed to create agent '{agent_id_str}': {e}. This may be due to invalid API key or configuration.",
                            )));
                        }
                    }
                }

                let _ = self.get_circuit_breaker(&agent_id).await;
            }
        }

        // Pre-compute dependency map and id->name map
        {
            let mut deps_map: HashMap<String, Vec<String>> = HashMap::new();
            let mut id_name_map: HashMap<String, String> = HashMap::new();

            for (nid, node) in workflow.graph.get_nodes() {
                id_name_map.insert(nid.to_string(), node.name.clone());
            }

            let mut graph_clone = workflow.graph.clone();
            for nid in id_name_map.keys() {
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

        let nodes = super::helpers::collect_executable_nodes(&workflow.graph)?;
        if nodes.is_empty() {
            context.complete();
            return Ok(context);
        }

        let batches = create_dependency_batches(&workflow.graph).await?;
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

            let shared_context = Arc::new(Mutex::new(context));
            let mut tasks = Vec::with_capacity(batch_size);

            for node in batch {
                let context_clone = shared_context.clone();
                let agents_clone = self.agents.clone();
                let circuit_breakers_clone = self.circuit_breakers.clone();
                let circuit_breaker_config = self.circuit_breaker_config.clone();
                let retry_config = self.default_retry_config.clone();
                let concurrency_manager = self.concurrency_manager.clone();

                let task: JoinHandle<Result<_, GraphBitError>> = tokio::spawn(async move {
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

                        let mut ctx = shared_context.lock().await;
                        if let Some(node) = workflow.graph.get_node(&node_result.node_id) {
                            ctx.set_node_output(&node.id, node_result.output.clone());
                            ctx.set_node_output_by_name(&node.name, node_result.output.clone());

                            let keys_now: Vec<String> = ctx.node_outputs.keys().cloned().collect();
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
                        } else {
                            if let Ok(output_str) = serde_json::to_string(&node_result.output) {
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
                ctx.fail(failure_message);
                drop(ctx);
                return Ok(Arc::try_unwrap(shared_context).unwrap().into_inner());
            }

            context = Arc::try_unwrap(shared_context).unwrap().into_inner();
        }

        let total_time = start_time.elapsed();
        let stats = WorkflowExecutionStats {
            total_nodes: total_executed,
            successful_nodes: total_successful,
            failed_nodes: total_executed - total_successful,
            avg_execution_time_ms: total_time.as_millis() as f64 / total_executed.max(1) as f64,
            max_concurrent_nodes: self.max_concurrency().await,
            total_execution_time_ms: total_time.as_millis() as u64,
            peak_memory_usage_mb: None,
            semaphore_acquisitions: 0,
            avg_semaphore_wait_ms: 0.0,
        };

        context.set_stats(stats);
        context.complete();

        Ok(context)
    }

    /// Execute a node with retry logic and circuit breaker
    async fn execute_node_with_retry(
        node: WorkflowNode,
        context: Arc<Mutex<WorkflowContext>>,
        agents: Arc<RwLock<HashMap<AgentId, Arc<dyn AgentTrait>>>>,
        circuit_breakers: Arc<RwLock<HashMap<AgentId, CircuitBreaker>>>,
        circuit_breaker_config: CircuitBreakerConfig,
        retry_config: Option<RetryConfig>,
    ) -> GraphBitResult<NodeExecutionResult> {
        let start_time = std::time::Instant::now();
        let mut attempt = 0;

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

            let result = match &node.node_type {
                NodeType::Agent {
                    agent_id,
                    prompt_template,
                } => {
                    execute_agent_node(
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
                    execute_condition_node(expression).await
                }
                NodeType::Transform { transformation } => {
                    execute_transform_node(transformation, context.clone()).await
                }
                NodeType::Delay { duration_seconds } => {
                    execute_delay_node(*duration_seconds).await
                }
                NodeType::DocumentLoader {
                    document_type,
                    source_path,
                    ..
                } => {
                    execute_document_loader_node(document_type, source_path, context.clone())
                        .await
                }
                _ => Err(GraphBitError::workflow_execution(format!(
                    "Unsupported node type: {:?}",
                    node.node_type
                ))),
            };

            match result {
                Ok(output) => {
                    {
                        let mut ctx = context.lock().await;
                        ctx.set_node_output(&node.id, output.clone());
                        ctx.set_node_output_by_name(&node.name, output.clone());

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
                    if let Some(ref mut breaker) = circuit_breaker {
                        breaker.record_failure();
                        if let NodeType::Agent { agent_id, .. } = &node.node_type {
                            let mut breakers = circuit_breakers.write().await;
                            breakers.insert(agent_id.clone(), breaker.clone());
                        }
                    }

                    if let Some(ref config) = retry_config {
                        if config.should_retry(&error, attempt) {
                            attempt += 1;

                            let delay_ms = config.calculate_delay(attempt);
                            if delay_ms > 0 {
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                    .await;
                            }

                            continue;
                        }
                    }

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

    /// Execute concurrent tasks efficiently
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
        execute_concurrent_tasks_with_retry(
            tasks,
            task_fn,
            self.default_retry_config.clone(),
            self.max_node_execution_time_ms,
            self.concurrency_manager.clone(),
        )
        .await
    }

    /// Execute concurrent agent tasks with maximum efficiency
    pub async fn execute_concurrent_agent_tasks(
        &self,
        prompts: Vec<String>,
        agent_id: AgentId,
    ) -> GraphBitResult<Vec<Result<serde_json::Value, GraphBitError>>> {
        if prompts.is_empty() {
            return Ok(Vec::new());
        }

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

        let concurrent_tasks: Vec<_> = prompts
            .into_iter()
            .enumerate()
            .map(|(index, prompt)| {
                let agent_clone = agent.clone();
                let agent_id_clone = agent_id.clone();

                tokio::spawn(async move {
                    let message = AgentMessage::new(
                        agent_id_clone.clone(),
                        None,
                        MessageContent::Text(prompt),
                    );

                    agent_clone.execute(message).await.map_err(|e| {
                        GraphBitError::workflow_execution(
                            format!("Agent task {index} failed: {e}",),
                        )
                    })
                })
            })
            .collect();

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
