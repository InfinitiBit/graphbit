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
use tokio::sync::Mutex;

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
            inner: Arc::new(Mutex::new(workflow)),
        })
    }
}

/// Workflow representation
#[napi]
pub struct Workflow {
    inner: Arc<Mutex<CoreWorkflow>>,
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
            core_node = core_node.with_retry_config(retry_config.into());
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
    inner: Arc<Mutex<CoreWorkflowContext>>,
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

    /// Create from core context (internal use)
    pub(crate) fn from_core(ctx: CoreWorkflowContext) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ctx)),
        }
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
    pub default_retry_config: Option<crate::types::RetryConfig>,
}

/// Workflow executor
#[napi]
pub struct Executor {
    llm_config: LlmConfig,
    config: ExecutorConfig,
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
        }
    }

    /// Execute a workflow
    ///
    /// # Example
    ///
    /// ```javascript
    /// const result = await executor.execute(workflow);
    /// if (result.isCompleted()) {
    ///   console.log(result.getAllOutputs());
    /// }
    /// ```
    #[napi]
    pub async fn execute(&self, workflow: &Workflow) -> Result<WorkflowContext> {
        let core_workflow = workflow.clone_inner().await;

        let mut executor = CoreWorkflowExecutor::new()
            .with_default_llm_config(self.llm_config.clone_inner());

        if let Some(retry_config) = &self.config.default_retry_config {
            executor = executor.with_retry_config(retry_config.clone().into());
        }

        let timeout = std::time::Duration::from_secs(
            self.config.timeout_seconds.unwrap_or(300) as u64
        );

        let context = tokio::time::timeout(timeout, executor.execute(core_workflow))
            .await
            .map_err(|_| Error::from_reason("Workflow execution timeout"))?
            .map_err(crate::errors::to_napi_error)?;

        Ok(WorkflowContext::from_core(context))
    }
}

