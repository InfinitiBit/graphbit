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

        let executor = CoreWorkflowExecutor::new()
            .with_default_llm_config(self.llm_config.clone_inner());

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

