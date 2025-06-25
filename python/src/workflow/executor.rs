//! Workflow executor for GraphBit Python bindings

use graphbit_core::workflow::WorkflowExecutor as CoreWorkflowExecutor;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use super::{result::WorkflowResult, workflow::Workflow};
use crate::errors::to_py_runtime_error;
use crate::llm::config::LlmConfig;
use crate::runtime::get_runtime;

#[pyclass]
pub struct Executor;

#[pymethods]
impl Executor {
    #[new]
    fn new(_config: LlmConfig) -> Self {
        Self
    }

    fn run(&self, workflow: &Workflow) -> PyResult<WorkflowResult> {
        let workflow_clone = workflow.inner.clone();

        get_runtime().block_on(async move {
            let executor = CoreWorkflowExecutor::new();
            let result = executor
                .execute(workflow_clone)
                .await
                .map_err(to_py_runtime_error)?;

            Ok(WorkflowResult { inner: result })
        })
    }

    fn run_async<'a>(&self, workflow: &Workflow, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let workflow_clone = workflow.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let executor = CoreWorkflowExecutor::new();
            let result = executor
                .execute(workflow_clone)
                .await
                .map_err(to_py_runtime_error)?;

            Ok(WorkflowResult { inner: result })
        })
    }
}
