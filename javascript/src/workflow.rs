//! Workflow bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::workflow::{
    Workflow as CoreWorkflow,
    WorkflowBuilder as CoreWorkflowBuilder,
    WorkflowExecutor as CoreWorkflowExecutor,
};
use graphbit_core::types::WorkflowContext as CoreWorkflowContext;
use crate::llm::LlmConfig;
use crate::types::{WorkflowState, WorkflowExecutionStats};
use crate::graph::{WorkflowNode, WorkflowEdge};
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

/// Workflow builder for constructing workflows
#[napi]
pub struct WorkflowBuilder {
    inner: Option<CoreWorkflowBuilder>,
}

#[napi]
impl WorkflowBuilder {
    /// Create a new workflow builder
    ///
    /// # Example
    ///
    /// ```javascript
    /// const builder = new WorkflowBuilder('My Workflow');
    /// ```
    #[napi(constructor)]
    pub fn new(name: String) -> Self {
        Self {
            inner: Some(CoreWorkflowBuilder::new(name)),
        }
    }

    /// Set workflow description
    #[napi]
    pub fn description(&mut self, description: String) -> Result<&Self> {
        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;
        self.inner = Some(builder.description(description));
        Ok(self)
    }

    /// Add metadata to the workflow
    #[napi]
    pub fn add_metadata(&mut self, key: String, value: String) -> Result<&Self> {
        let json_value: serde_json::Value = serde_json::from_str(&value)
            .map_err(|e| Error::from_reason(format!("Invalid JSON value: {}", e)))?;

        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;
        self.inner = Some(builder.metadata(key, json_value));
        Ok(self)
    }

    /// Build the workflow
    #[napi]
    pub fn build(&mut self) -> Result<Workflow> {
        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;
        let workflow = builder.build()
            .map_err(crate::errors::to_napi_error)?;

        Ok(Workflow {
            inner: Arc::new(TokioMutex::new(workflow)),
        })
    }
}

/// Workflow representation
#[napi]
pub struct Workflow {
    inner: Arc<TokioMutex<CoreWorkflow>>,
}

#[napi]
impl Workflow {
    /// Get workflow ID
    #[napi]
    pub async fn id(&self) -> Result<String> {
        let workflow = self.inner.lock().await;
        Ok(workflow.id.to_string())
    }

    /// Get workflow name
    #[napi]
    pub async fn name(&self) -> Result<String> {
        let workflow = self.inner.lock().await;
        Ok(workflow.name.clone())
    }

    /// Get workflow description
    #[napi]
    pub async fn description(&self) -> Result<String> {
        let workflow = self.inner.lock().await;
        Ok(workflow.description.clone())
    }

    /// Clone the inner workflow (internal use)
    pub(crate) async fn clone_inner(&self) -> CoreWorkflow {
        let workflow = self.inner.lock().await;
        workflow.clone()
    }

    /// Add a node to the workflow
    ///
    /// # Arguments
    /// * `node` - The workflow node to add
    ///
    /// # Returns
    /// The node ID as a string
    ///
    /// # Example
    /// ```javascript
    /// const workflow = new WorkflowBuilder('My Workflow').build();
    /// const node = {
    ///   id: "node1",
    ///   name: "My Node",
    ///   description: "A test node",
    ///   nodeType: "Agent"
    /// };
    /// const nodeId = await workflow.addNode(node);
    /// ```
    #[napi]
    pub async fn add_node(&self, node: WorkflowNode) -> Result<String> {
        let mut workflow = self.inner.lock().await;

        // Parse the node ID as a UUID
        let node_id = graphbit_core::types::NodeId::from_string(&node.id)
            .map_err(|e| Error::from_reason(format!("Invalid node ID: {}", e)))?;

        // Determine the node type from the string
        let node_type = match node.node_type.as_str() {
            "Agent" => graphbit_core::graph::NodeType::Agent {
                agent_id: graphbit_core::types::AgentId::new(),
                prompt_template: String::new(),
            },
            "Condition" => graphbit_core::graph::NodeType::Condition {
                expression: String::new(),
            },
            "Transform" => graphbit_core::graph::NodeType::Transform {
                transformation: String::new(),
            },
            "Split" => graphbit_core::graph::NodeType::Split,
            "Join" => graphbit_core::graph::NodeType::Join,
            "Delay" => graphbit_core::graph::NodeType::Delay {
                duration_seconds: 0,
            },
            _ => return Err(Error::from_reason(format!("Unknown node type: {}", node.node_type))),
        };

        // Create the core WorkflowNode
        let mut core_node = graphbit_core::graph::WorkflowNode::new(
            node.name,
            node.description,
            node_type
        );
        core_node.id = node_id.clone();

        if let Some(retry_config) = node.retry_config {
            let core_retry_config = graphbit_core::types::RetryConfig {
                max_attempts: retry_config.max_attempts,
                initial_delay_ms: retry_config.initial_delay_ms as u64,
                backoff_multiplier: retry_config.backoff_multiplier,
                max_delay_ms: retry_config.max_delay_ms as u64,
                jitter_factor: retry_config.jitter_factor,
                retryable_errors: retry_config.retryable_errors.into_iter().map(|e| match e {
                    crate::types::JsRetryableErrorType::NetworkError => graphbit_core::types::RetryableErrorType::NetworkError,
                    crate::types::JsRetryableErrorType::TimeoutError => graphbit_core::types::RetryableErrorType::TimeoutError,
                    crate::types::JsRetryableErrorType::RateLimitError => graphbit_core::types::RetryableErrorType::RateLimitError,
                    crate::types::JsRetryableErrorType::TemporaryUnavailable => graphbit_core::types::RetryableErrorType::TemporaryUnavailable,
                    crate::types::JsRetryableErrorType::InternalServerError => graphbit_core::types::RetryableErrorType::InternalServerError,
                    crate::types::JsRetryableErrorType::AuthenticationError => graphbit_core::types::RetryableErrorType::AuthenticationError,
                    crate::types::JsRetryableErrorType::ResourceConflict => graphbit_core::types::RetryableErrorType::ResourceConflict,
                    crate::types::JsRetryableErrorType::Other => graphbit_core::types::RetryableErrorType::Other,
                }).collect(),
            };
            core_node = core_node.with_retry_config(core_retry_config);
        }

        let result_id = workflow.add_node(core_node)
            .map_err(crate::errors::to_napi_error)?;

        Ok(result_id.to_string())
    }

    /// Connect two nodes with an edge
    ///
    /// # Arguments
    /// * `from` - The source node ID
    /// * `to` - The target node ID
    /// * `edge` - The workflow edge to add
    ///
    /// # Example
    /// ```javascript
    /// const workflow = new WorkflowBuilder('My Workflow').build();
    /// const edge = {
    ///   fromNode: "node1",
    ///   toNode: "node2",
    ///   condition: null
    /// };
    /// await workflow.addEdge("node1", "node2", edge);
    /// ```
    #[napi]
    pub async fn add_edge(&self, from: String, to: String, edge: WorkflowEdge) -> Result<()> {
        let mut workflow = self.inner.lock().await;

        // Parse the node IDs
        let from_id = graphbit_core::types::NodeId::from_string(&from)
            .map_err(|e| Error::from_reason(format!("Invalid from node ID: {}", e)))?;
        let to_id = graphbit_core::types::NodeId::from_string(&to)
            .map_err(|e| Error::from_reason(format!("Invalid to node ID: {}", e)))?;

        // Create a data flow edge (default)
        let mut core_edge = graphbit_core::graph::WorkflowEdge::data_flow();
        core_edge.condition = edge.condition;

        workflow.connect_nodes(from_id, to_id, core_edge)
            .map_err(crate::errors::to_napi_error)?;

        Ok(())
    }

    /// Validate the workflow structure
    ///
    /// # Returns
    /// True if the workflow is valid, throws an error otherwise
    ///
    /// # Example
    /// ```javascript
    /// const workflow = new WorkflowBuilder('My Workflow').build();
    /// const isValid = await workflow.validate();
    /// console.log('Workflow is valid:', isValid);
    /// ```
    #[napi]
    pub async fn validate(&self) -> Result<bool> {
        let workflow = self.inner.lock().await;
        workflow.validate()
            .map_err(crate::errors::to_napi_error)?;
        Ok(true)
    }
}

/// Workflow execution context
#[napi]
pub struct WorkflowContext {
    inner: Arc<TokioMutex<CoreWorkflowContext>>,
}

#[napi]
impl WorkflowContext {
    /// Check if workflow is completed
    #[napi]
    pub async fn is_completed(&self) -> Result<bool> {
        let ctx = self.inner.lock().await;
        Ok(matches!(ctx.state, graphbit_core::types::WorkflowState::Completed))
    }

    /// Check if workflow failed
    #[napi]
    pub async fn is_failed(&self) -> Result<bool> {
        let ctx = self.inner.lock().await;
        Ok(matches!(ctx.state, graphbit_core::types::WorkflowState::Failed { .. }))
    }

    /// Get workflow state
    #[napi]
    pub async fn state(&self) -> Result<WorkflowState> {
        let ctx = self.inner.lock().await;
        Ok(WorkflowState::from(ctx.state.clone()))
    }

    /// Get execution statistics
    #[napi]
    pub async fn stats(&self) -> Result<Option<WorkflowExecutionStats>> {
        let ctx = self.inner.lock().await;
        Ok(ctx.stats.clone().map(WorkflowExecutionStats::from))
    }

    /// Get error message if failed
    #[napi]
    pub async fn error(&self) -> Result<Option<String>> {
        let ctx = self.inner.lock().await;
        match &ctx.state {
            graphbit_core::types::WorkflowState::Failed { error } => Ok(Some(error.clone())),
            _ => Ok(None),
        }
    }

    /// Get all node outputs as JSON string
    #[napi]
    pub async fn get_all_outputs(&self) -> Result<String> {
        let ctx = self.inner.lock().await;
        serde_json::to_string(&ctx.node_outputs)
            .map_err(|e| Error::from_reason(format!("Failed to serialize outputs: {}", e)))
    }

    // ========== NEW METHODS FOR ENHANCED WORKFLOW INTROSPECTION ==========

    /// Set a variable in the workflow context
    ///
    /// # Arguments
    /// * `key` - Variable name
    /// * `value` - Variable value (as JSON string or primitive)
    ///
    /// # Example
    ///
    /// ```javascript
    /// await context.setVariable('user_id', '12345');
    /// await context.setVariable('config', JSON.stringify({ theme: 'dark' }));
    /// ```
    #[napi]
    pub async fn set_variable(&self, key: String, value: String) -> Result<()> {
        let mut ctx = self.inner.lock().await;
        
        // Try to parse as JSON, fall back to string
        let json_value = serde_json::from_str::<serde_json::Value>(&value)
            .unwrap_or_else(|_| serde_json::Value::String(value));
        
        ctx.variables.insert(key, json_value);
        Ok(())
    }

    /// Get a variable from the workflow context
    ///
    /// Returns the variable as a JSON string, or None if not found.
    ///
    /// # Arguments
    /// * `key` - Variable name
    ///
    /// # Example
    ///
    /// ```javascript
    /// const userId = await context.getVariable('user_id');
    /// if (userId) {
    ///   console.log('User ID:', userId);
    /// }
    /// ```
    #[napi]
    pub async fn get_variable(&self, key: String) -> Result<Option<String>> {
        let ctx = self.inner.lock().await;
        
        match ctx.variables.get(&key) {
            Some(value) => {
                let json_str = serde_json::to_string(value)
                    .map_err(|e| Error::from_reason(format!("Failed to serialize variable: {}", e)))?;
                Ok(Some(json_str))
            }
            None => Ok(None),
        }
    }

    /// Get all variables as a JSON object string
    ///
    /// Returns all workflow variables as a JSON string containing an object.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const vars = await context.getAllVariables();
    /// const varsObj = JSON.parse(vars);
    /// console.log(varsObj.user_id, varsObj.config);
    /// ```
    #[napi]
    pub async fn get_all_variables(&self) -> Result<String> {
        let ctx = self.inner.lock().await;
        serde_json::to_string(&ctx.variables)
            .map_err(|e| Error::from_reason(format!("Failed to serialize variables: {}", e)))
    }

    /// Get a specific node's output
    ///
    /// # Arguments
    /// * `node_id` - The node ID or name
    ///
    /// # Returns
    /// The node's output as a JSON string, or None if not found
    ///
    /// # Example
    ///
    /// ```javascript
    /// const analyzerOutput = await context.getNodeOutput('analyzer_node');
    /// if (analyzerOutput) {
    ///   const result = JSON.parse(analyzerOutput);
    ///   console.log('Analysis result:', result);
    /// }
    /// ```
    #[napi]
    pub async fn get_node_output(&self, node_id: String) -> Result<Option<String>> {
        let ctx = self.inner.lock().await;
        
        match ctx.get_node_output(&node_id) {
            Some(value) => {
                let json_str = serde_json::to_string(value)
                    .map_err(|e| Error::from_reason(format!("Failed to serialize node output: {}", e)))?;
                Ok(Some(json_str))
            }
            None => Ok(None),
        }
    }

    /// Get a nested value from a node's output using dot notation
    ///
    /// # Arguments
    /// * `reference` - Dot-notation reference (e.g., "node1.results.score")
    ///
    /// # Returns
    /// The nested value as a JSON string, or None if not found
    ///
    /// # Example
    ///
    /// ```javascript
    /// const score = await context.getNestedOutput('analyzer.results.score');
    /// if (score) {
    ///   console.log('Score:', JSON.parse(score));
    /// }
    /// ```
    #[napi]
    pub async fn get_nested_output(&self, reference: String) -> Result<Option<String>> {
        let ctx = self.inner.lock().await;
        
        match ctx.get_nested_output(&reference) {
            Some(value) => {
                let json_str = serde_json::to_string(value)
                    .map_err(|e| Error::from_reason(format!("Failed to serialize nested output: {}", e)))?;
                Ok(Some(json_str))
            }
            None => Ok(None),
        }
    }

    /// Get the workflow ID
    ///
    /// # Returns
    /// The workflow's unique identifier
    ///
    /// # Example
    ///
    /// ```javascript
    /// const workflowId = await context.getWorkflowId();
    /// console.log('Workflow ID:', workflowId);
    /// ```
    #[napi]
    pub async fn get_workflow_id(&self) -> Result<String> {
        let ctx = self.inner.lock().await;
        Ok(ctx.workflow_id.to_string())
    }

    /// Get the workflow execution duration in milliseconds
    ///
    /// Returns the time elapsed from workflow start to completion (or current time if still running).
    ///
    /// # Example
    ///
    /// ```javascript
    /// const duration = await context.getExecutionDuration();
    /// console.log(`Workflow took ${duration}ms`);
    /// ```
    #[napi]
    pub async fn get_execution_duration(&self) -> Result<f64> {
        let ctx = self.inner.lock().await;
        
        match ctx.execution_duration_ms() {
            Some(duration) => Ok(duration as f64),
            None => {
                // If not completed, calculate duration from start to now
                let started_at = ctx.started_at;
                let duration = chrono::Utc::now()
                    .signed_duration_since(started_at)
                    .num_milliseconds();
                Ok(duration as f64)
            }
        }
    }

    /// Convert the workflow context to a dictionary object (as JSON string)
    ///
    /// Returns a complete snapshot of the workflow context including:
    /// - variables: All workflow variables
    /// - nodeOutputs: All node outputs
    /// - state: Current workflow state
    /// - workflowId: Workflow identifier
    /// - executionDuration: Duration in milliseconds
    ///
    /// # Example
    ///
    /// ```javascript
    /// const contextData = await context.toDict();
    /// const data = JSON.parse(contextData);
    /// console.log('Variables:', data.variables);
    /// console.log('Node outputs:', data.nodeOutputs);
    /// console.log('State:', data.state);
    /// ```
    #[napi]
    pub async fn to_dict(&self) -> Result<String> {
        let ctx = self.inner.lock().await;
        
        let mut dict = serde_json::Map::new();
        
        // Add variables
        dict.insert("variables".to_string(), serde_json::to_value(&ctx.variables)
            .map_err(|e| Error::from_reason(format!("Failed to serialize variables: {}", e)))?);
        
        // Add node outputs
        dict.insert("nodeOutputs".to_string(), serde_json::to_value(&ctx.node_outputs)
            .map_err(|e| Error::from_reason(format!("Failed to serialize node outputs: {}", e)))?);
        
        // Add state
        dict.insert("state".to_string(), serde_json::Value::String(format!("{:?}", ctx.state)));
        
        // Add workflow ID
        dict.insert("workflowId".to_string(), serde_json::Value::String(ctx.workflow_id.to_string()));
        
        // Add execution duration
        if let Some(duration) = ctx.execution_duration_ms() {
            dict.insert("executionDurationMs".to_string(), serde_json::Value::Number(
                serde_json::Number::from(duration)
            ));
        }
        
        // Add metadata if present
        if !ctx.metadata.is_empty() {
            dict.insert("metadata".to_string(), serde_json::to_value(&ctx.metadata)
                .map_err(|e| Error::from_reason(format!("Failed to serialize metadata: {}", e)))?);
        }
        
        let json_obj = serde_json::Value::Object(dict);
        serde_json::to_string(&json_obj)
            .map_err(|e| Error::from_reason(format!("Failed to serialize context: {}", e)))
    }

    /// Create from core context (internal use)
    pub(crate) fn from_core(ctx: CoreWorkflowContext) -> Self {
        Self {
            inner: Arc::new(TokioMutex::new(ctx)),
        }
    }

    /// Get the underlying core context (internal use)
    pub(crate) async fn get_inner(&self) -> CoreWorkflowContext {
        let ctx = self.inner.lock().await;
        ctx.clone()
    }
}

/// Workflow execution result
///
/// Provides structured access to workflow execution results including:
/// - Success/failure status
/// - Node outputs
/// - Variables
/// - Execution metadata
/// - LLM response metadata
///
/// This is the primary way to access workflow results after execution.
///
/// # Example
///
/// ```javascript
/// const result = await executor.execute(workflow);
///
/// if (result.isSuccess()) {
///   console.log('Workflow completed successfully!');
///   console.log(`Execution time: ${result.executionTimeMs()}ms`);
///   
///   // Get node outputs
///   const finalOutput = result.getNodeOutput('final_node');
///   console.log('Result:', JSON.parse(finalOutput));
///   
///   // Get all results
///   const allOutputs = result.getAllNodeOutputs();
///   console.log(JSON.parse(allOutputs));
/// } else {
///   console.error('Workflow failed:', result.error());
/// }
/// ```
#[napi]
pub struct WorkflowResult {
    context: WorkflowContext,
}

#[napi]
impl WorkflowResult {
    /// Check if workflow completed successfully
    ///
    /// # Example
    ///
    /// ```javascript
    /// if (result.isSuccess()) {
    ///   console.log('Workflow completed successfully!');
    /// }
    /// ```
    #[napi]
    pub async fn is_success(&self) -> Result<bool> {
        self.context.is_completed().await
    }

    /// Check if workflow failed
    ///
    /// # Example
    ///
    /// ```javascript
    /// if (result.isFailed()) {
    ///   console.error('Workflow failed:', result.error());
    /// }
    /// ```
    #[napi]
    pub async fn is_failed(&self) -> Result<bool> {
        self.context.is_failed().await
    }

    /// Get workflow execution state
    ///
    /// # Example
    ///
    /// ```javascript
    /// const state = result.state();
    /// console.log('State:', state); // "Completed", "Failed", "Running", etc.
    /// ```
    #[napi]
    pub async fn state(&self) -> Result<String> {
        let state_enum = self.context.state().await?;
        // Convert WorkflowState enum to string
        let state_str = match state_enum {
            WorkflowState::Pending => "Pending",
            WorkflowState::Running => "Running",
            WorkflowState::Completed => "Completed",
            WorkflowState::Failed => "Failed",
            WorkflowState::Cancelled => "Cancelled",
        };
        Ok(state_str.to_string())
    }

    /// Get error message if workflow failed
    ///
    /// Returns the error message, or None if workflow didn't fail.
    ///
    /// # Example
    ///
    /// ```javascript
    /// if (result.isFailed()) {
    ///   const error = result.error();
    ///   console.error('Error:', error);
    /// }
    /// ```
    #[napi]
    pub async fn error(&self) -> Result<Option<String>> {
        self.context.error().await
    }

    /// Get a specific node's output
    ///
    /// # Arguments
    /// * `node_id` - The node ID or name
    ///
    /// # Returns
    /// The node's output as a JSON string, or null if not found
    ///
    /// # Example
    ///
    /// ```javascript
    /// const analyzerOutput = result.getNodeOutput('analyzer_node');
    /// if (analyzerOutput) {
    ///   const data = JSON.parse(analyzerOutput);
    ///   console.log('Analysis:', data);
    /// }
    /// ```
    #[napi]
    pub async fn get_node_output(&self, node_id: String) -> Result<Option<String>> {
        self.context.get_node_output(node_id).await
    }

    /// Get all node outputs
    ///
    /// Returns all node outputs as a JSON object string.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const outputs = result.getAllNodeOutputs();
    /// const outputsObj = JSON.parse(outputs);
    /// console.log('Node 1:', outputsObj.node1);
    /// console.log('Node 2:', outputsObj.node2);
    /// ```
    #[napi]
    pub async fn get_all_node_outputs(&self) -> Result<String> {
        self.context.get_all_outputs().await
    }

    /// Get a workflow variable
    ///
    /// # Arguments
    /// * `key` - Variable name
    ///
    /// # Example
    ///
    /// ```javascript
    /// const userId = result.getVariable('user_id');
    /// console.log('User ID:', userId);
    /// ```
    #[napi]
    pub async fn get_variable(&self, key: String) -> Result<Option<String>> {
        self.context.get_variable(key).await
    }

    /// Get all workflow variables
    ///
    /// Returns all variables as a JSON object string.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const vars = result.getAllVariables();
    /// const varsObj = JSON.parse(vars);
    /// console.log(varsObj);
    /// ```
    #[napi]
    pub async fn get_all_variables(&self) -> Result<String> {
        self.context.get_all_variables().await
    }

    /// Get workflow execution time in milliseconds
    ///
    /// # Example
    ///
    /// ```javascript
    /// const duration = result.executionTimeMs();
    /// console.log(`Workflow took ${duration}ms`);
    ///
    /// if (duration > 30000) {
    ///   console.warn('Slow execution detected');
    /// }
    /// ```
    #[napi]
    pub async fn execution_time_ms(&self) -> Result<f64> {
        self.context.get_execution_duration().await
    }

    /// Get workflow ID
    ///
    /// # Example
    ///
    /// ```javascript
    /// const workflowId = result.workflowId();
    /// console.log('Workflow ID:', workflowId);
    /// ```
    #[napi]
    pub async fn workflow_id(&self) -> Result<String> {
        self.context.get_workflow_id().await
    }

    /// Get execution statistics
    ///
    /// Returns detailed statistics about the workflow execution including:
    /// - Total nodes
    /// - Successful/failed nodes
    /// - Average execution time per node
    /// - Total execution time
    ///
    /// # Example
    ///
    /// ```javascript
    /// const stats = result.getStats();
    /// if (stats) {
    ///   console.log(`Nodes: ${stats.successfulNodes}/${stats.totalNodes}`);
    ///   console.log(`Avg time per node: ${stats.avgExecutionTimeMs}ms`);
    /// }
    /// ```
    #[napi]
    pub async fn get_stats(&self) -> Result<Option<WorkflowExecutionStats>> {
        self.context.stats().await
    }

    /// Get the underlying WorkflowContext
    ///
    /// Provides access to the underlying context for advanced use cases.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const context = result.getContext();
    /// const nestedValue = await context.getNestedOutput('node.data.field');
    /// ```
    #[napi]
    pub fn get_context(&self) -> WorkflowContext {
        WorkflowContext {
            inner: self.context.inner.clone(),
        }
    }

    /// Convert the result to a complete dictionary
    ///
    /// Returns all workflow data as a JSON string including:
    /// - variables
    /// - nodeOutputs
    /// - state
    /// - workflowId
    /// - executionDurationMs
    /// - stats
    ///
    /// # Example
    ///
    /// ```javascript
    /// const resultData = result.toDict();
    /// const data = JSON.parse(resultData);
    ///
    /// // Save complete result to database
    /// await database.saveWorkflowResult(data.workflowId, data);
    /// ```
    #[napi]
    pub async fn to_dict(&self) -> Result<String> {
        self.context.to_dict().await
    }

    /// Create from context (internal use)
    pub(crate) fn from_context(context: WorkflowContext) -> Self {
        Self { context }
    }
}

/// Workflow executor configuration
#[napi(object)]
pub struct ExecutorConfig {
    /// Timeout in seconds
    pub timeout_seconds: Option<i64>,
    /// Enable debug mode
    pub debug: Option<bool>,
    /// Maximum parallel executions
    pub max_parallel: Option<i32>,
    /// Default retry configuration
    pub default_retry_config: Option<crate::types::JsRetryConfig>,
}

/// Workflow executor
#[napi]
pub struct Executor {
    llm_config: LlmConfig,
    config: ExecutorConfig,
    lightweight_mode: Arc<TokioMutex<bool>>,
}

#[napi]
impl Executor {
    /// Create a new executor
    ///
    /// # Example
    ///
    /// ```javascript
    /// const executor = new Executor({
    ///   config: llmConfig,
    ///   timeoutSeconds: 300,
    ///   debug: false
    /// });
    /// ```
    #[napi(constructor)]
    pub fn new(llm_config: &LlmConfig, config: Option<ExecutorConfig>) -> Self {
        Self {
            llm_config: LlmConfig {
                inner: llm_config.clone_inner(),
            },
            config: config.unwrap_or(ExecutorConfig {
                timeout_seconds: Some(300),
                debug: Some(false),
                max_parallel: Some(4),
                default_retry_config: None,
            }),
            lightweight_mode: Arc::new(TokioMutex::new(false)),
        }
    }

    /// Execute a workflow
    ///
    /// Returns a WorkflowResult with complete access to execution outcomes.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const result = await executor.execute(workflow);
    /// if (result.isSuccess()) {
    ///   console.log(`Completed in ${result.executionTimeMs()}ms`);
    ///   const output = result.getNodeOutput('final_node');
    ///   console.log('Result:', JSON.parse(output));
    /// } else {
    ///   console.error('Failed:', result.error());
    /// }
    /// ```
    #[napi]
    pub async fn execute(&self, workflow: &Workflow) -> Result<WorkflowResult> {
        let core_workflow = workflow.clone_inner().await;

        let mut executor = CoreWorkflowExecutor::new()
            .with_default_llm_config(self.llm_config.clone_inner());

        if let Some(retry_config) = &self.config.default_retry_config {
            let core_retry_config = graphbit_core::types::RetryConfig {
                max_attempts: retry_config.max_attempts,
                initial_delay_ms: retry_config.initial_delay_ms as u64,
                backoff_multiplier: retry_config.backoff_multiplier,
                max_delay_ms: retry_config.max_delay_ms as u64,
                jitter_factor: retry_config.jitter_factor,
                retryable_errors: retry_config.retryable_errors.iter().map(|e| match e {
                    crate::types::JsRetryableErrorType::NetworkError => graphbit_core::types::RetryableErrorType::NetworkError,
                    crate::types::JsRetryableErrorType::TimeoutError => graphbit_core::types::RetryableErrorType::TimeoutError,
                    crate::types::JsRetryableErrorType::RateLimitError => graphbit_core::types::RetryableErrorType::RateLimitError,
                    crate::types::JsRetryableErrorType::TemporaryUnavailable => graphbit_core::types::RetryableErrorType::TemporaryUnavailable,
                    crate::types::JsRetryableErrorType::InternalServerError => graphbit_core::types::RetryableErrorType::InternalServerError,
                    crate::types::JsRetryableErrorType::AuthenticationError => graphbit_core::types::RetryableErrorType::AuthenticationError,
                    crate::types::JsRetryableErrorType::ResourceConflict => graphbit_core::types::RetryableErrorType::ResourceConflict,
                    crate::types::JsRetryableErrorType::Other => graphbit_core::types::RetryableErrorType::Other,
                }).collect(),
            };
            executor = executor.with_retry_config(core_retry_config);
        }

        let timeout = std::time::Duration::from_secs(
            self.config.timeout_seconds.unwrap_or(300) as u64
        );

        let context = tokio::time::timeout(timeout, executor.execute(core_workflow))
            .await
            .map_err(|_| Error::from_reason("Workflow execution timeout"))?
            .map_err(crate::errors::to_napi_error)?;

        let workflow_context = WorkflowContext::from_core(context);
        Ok(WorkflowResult::from_context(workflow_context))
    }
}
