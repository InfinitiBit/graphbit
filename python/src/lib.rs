//! This module provides Python bindings for the GraphBit agentic workflow
//! automation framework using PyO3.

#![allow(non_local_definitions)]

use pyo3::prelude::*;

// Module declarations
mod embeddings;
mod errors;
mod llm;
mod runtime;
mod validation;
mod workflow;

// Re-export all public types and functions
pub use embeddings::{EmbeddingClient, EmbeddingConfig};
pub use llm::{LlmClient, LlmConfig};
pub use workflow::{Executor, Node, Workflow, WorkflowResult};

#[pyfunction]
fn init() -> PyResult<()> {
    let _ = runtime::get_runtime(); // Initialize runtime
    graphbit_core::init().map_err(errors::to_py_runtime_error)?;
    Ok(())
}

#[pyfunction]
fn version() -> String {
    graphbit_core::VERSION.to_string()
}

#[pymodule]
fn graphbit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;

    m.add_class::<LlmConfig>()?;
    m.add_class::<LlmClient>()?;
    m.add_class::<Node>()?;
    m.add_class::<Workflow>()?;
    m.add_class::<WorkflowResult>()?;
    m.add_class::<Executor>()?;
    m.add_class::<EmbeddingConfig>()?;
    m.add_class::<EmbeddingClient>()?;

    Ok(())
}
