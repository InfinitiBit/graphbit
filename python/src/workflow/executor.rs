//! Workflow executor for GraphBit Python bindings

use graphbit_core::workflow::WorkflowExecutor as CoreWorkflowExecutor;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::llm::config::LlmConfig;
use crate::runtime::get_runtime;
use super::{workflow::Workflow, result::WorkflowResult};

#[pyclass]
pub struct Executor {
    _config: LlmConfig,
}

#[pymethods]
impl Executor {
    #[new]
    fn new(config: LlmConfig) -> Self {
        Self { _config: config }
    }

    fn timeout(&mut self, _timeout_seconds: u64) {}
    fn retries(&mut self, _max_retries: u32) {}

    fn run(&self, workflow: &Workflow) -> PyResult<WorkflowResult> {
        let workflow_clone = workflow.inner.clone();
        
        get_runtime().block_on(async move {
            let executor = CoreWorkflowExecutor::new();
            let result = executor.execute(workflow_clone).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            Ok(WorkflowResult { inner: result })
        })
    }

    fn run_async<'a>(&self, workflow: &Workflow, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let workflow_clone = workflow.inner.clone();
        
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let executor = CoreWorkflowExecutor::new();
            let result = executor.execute(workflow_clone).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            Ok(WorkflowResult { inner: result })
        })
    }
} 