//! Workflow executor for GraphBit Node.js bindings

use super::{Workflow, WorkflowContext, WorkflowResult};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::time::Instant;

#[napi]
pub struct Executor {
    // Configuration for the executor
    timeout_ms: Option<i64>,
    max_retries: Option<i32>,
}

#[napi]
impl Executor {
    /// Create a new workflow executor
    #[napi(constructor)]
    pub fn new(timeout_ms: Option<i64>, max_retries: Option<i32>) -> Self {
        Self {
            timeout_ms,
            max_retries,
        }
    }

    /// Execute a workflow with the given context
    #[napi]
    pub async fn execute(
        &self,
        workflow: &Workflow,
        _context: &WorkflowContext,
    ) -> Result<WorkflowResult> {
        let start_time = Instant::now();

        // Validate workflow before execution
        workflow.validate()?;

        // Create a workflow executor from core
        let executor = graphbit_core::workflow::WorkflowExecutor::new();

        // Clone the workflow for async execution
        let workflow_clone = workflow.inner.clone();

        let execution_result = tokio::task::spawn(async move {
            // Execute the workflow
            executor.execute(workflow_clone).await
        })
        .await;

        let duration = start_time.elapsed();

        match execution_result {
            Ok(Ok(core_result)) => {
                // Convert core result to our result format
                let mut result_data = HashMap::new();

                // Extract data from core result - using variables field
                for (key, value) in core_result.variables {
                    result_data.insert(key, value.to_string());
                }

                let mut result =
                    WorkflowResult::success(result_data, Some(duration.as_millis() as i64));

                // Add execution metadata
                result.set_metadata("node_count".to_string(), workflow.node_count().to_string());
                result.set_metadata("edge_count".to_string(), workflow.edge_count().to_string());

                Ok(result)
            }
            Ok(Err(e)) => Ok(WorkflowResult::failure(
                format!("Workflow execution failed: {}", e),
                Some(duration.as_millis() as i64),
            )),
            Err(e) => Ok(WorkflowResult::failure(
                format!("Task execution failed: {}", e),
                Some(duration.as_millis() as i64),
            )),
        }
    }

    /// Execute a workflow with input data directly
    #[napi]
    pub async fn execute_with_input(
        &self,
        workflow: &Workflow,
        input_data: HashMap<String, String>,
    ) -> Result<WorkflowResult> {
        let mut context = WorkflowContext::new();
        for (key, value) in input_data {
            context.set(key, value);
        }
        self.execute(workflow, &context).await
    }

    /// Set execution timeout
    #[napi]
    pub fn set_timeout(&mut self, timeout_ms: i64) {
        self.timeout_ms = Some(timeout_ms);
    }

    /// Set maximum retry attempts
    #[napi]
    pub fn set_max_retries(&mut self, max_retries: i32) {
        self.max_retries = Some(max_retries);
    }

    /// Get current timeout setting
    #[napi(getter)]
    pub fn timeout(&self) -> Option<i64> {
        self.timeout_ms
    }

    /// Get current max retries setting
    #[napi(getter)]
    pub fn max_retries(&self) -> Option<i32> {
        self.max_retries
    }
}
