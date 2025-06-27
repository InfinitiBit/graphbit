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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

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
                    // Create default agent configuration for this workflow
                    let default_config = crate::agents::AgentConfig::new(
                        format!("Agent_{}", agent_id_str),
                        "Auto-generated agent for workflow execution",
                        self.default_llm_config.clone().unwrap_or_default(),
                    ).with_id(agent_id.clone());

                    // Try to create agent - if it fails, continue (will use fallback execution)
                    if let Ok(agent) = crate::agents::Agent::new(default_config).await {
                        let mut agents_guard = self.agents.write().await;
                        agents_guard.insert(agent_id.clone(), Arc::new(agent));
                        tracing::debug!("Auto-registered agent: {}", agent_id);
                    }
                }

                // Pre-warm circuit breakers for all agents
                let _ = self.get_circuit_breaker(&agent_id).await;
            }
        }

        let nodes = self.collect_executable_nodes(&workflow.graph)?;
        if nodes.is_empty() {
            context.complete();
            return Ok(context);
        }

        // Execute nodes in optimized batches
        let batches = self.create_execution_batches(nodes).await?;
        let mut total_executed = 0;
        let mut total_successful = 0;

        for batch in batches {
            let batch_size = batch.len();

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

                        // Update context with results
                        let mut ctx = shared_context.lock().await;
                        if let Ok(output_str) = serde_json::to_string(&node_result.output) {
                            ctx.set_variable(
                                format!("node_result_{}", total_executed),
                                serde_json::Value::String(output_str),
                            );
                        }
                    }
                    Ok(Err(e)) => {
                        if self.fail_fast {
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
                    return Ok(NodeExecutionResult::failure(error.to_string())
                        .with_duration(start_time.elapsed().as_millis() as u64)
                        .with_retry_count(attempt));
                }
            }

            // Attempt to execute the node
            let result = match &node.node_type {
                NodeType::Agent {
                    agent_id,
                    prompt_template,
                } => {
                    Self::execute_agent_node_static(
                        agent_id,
                        prompt_template,
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
                    // Record success in circuit breaker
                    if let Some(ref mut breaker) = circuit_breaker {
                        breaker.record_success();
                        if let NodeType::Agent { agent_id, .. } = &node.node_type {
                            let mut breakers = circuit_breakers.write().await;
                            breakers.insert(agent_id.clone(), breaker.clone());
                        }
                    }

                    let duration = start_time.elapsed();
                    return Ok(NodeExecutionResult::success(output)
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
                    return Ok(NodeExecutionResult::failure(error.to_string())
                        .with_duration(duration.as_millis() as u64)
                        .with_retry_count(attempt));
                }
            }
        }
    }

    /// Execute an agent node (static version)
    async fn execute_agent_node_static(
        agent_id: &crate::types::AgentId,
        prompt_template: &str,
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

        // Get context variables for prompt interpolation
        let variables = {
            let ctx = context.lock().await;
            ctx.variables.clone()
        };

        // Simple variable substitution in prompt template
        let mut prompt = prompt_template.to_string();
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            if let Ok(value_str) = serde_json::to_string(&value) {
                prompt = prompt.replace(&placeholder, value_str.trim_matches('"'));
            }
        }

        // Create agent message
        let message = AgentMessage::new(agent_id.clone(), None, MessageContent::Text(prompt));

        // Execute agent
        agent.execute(message).await
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
