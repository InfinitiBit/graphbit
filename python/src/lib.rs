//! This module provides Python bindings for the GraphBit agentic workflow
//! automation framework using PyO3.

#![allow(non_local_definitions)]

use graphbit_core::{
    embeddings::{
        EmbeddingConfig, EmbeddingInput, EmbeddingProvider, EmbeddingRequest, EmbeddingResponse,
        EmbeddingService,
    },
    graph::{NodeType, WorkflowEdge, WorkflowNode},
    llm::LlmConfig,
    types::{
        AgentCapability, AgentId, CircuitBreakerConfig, NodeId, RetryConfig, WorkflowContext,
        WorkflowId, WorkflowState,
    },
    validation::ValidationResult,
    workflow::{Workflow, WorkflowBuilder, WorkflowExecutor},
};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

/// Python wrapper for LLM configuration
#[pyclass]
#[derive(Clone)]
pub struct PyLlmConfig {
    inner: LlmConfig,
}

#[pymethods]
impl PyLlmConfig {
    #[staticmethod]
    fn openai(api_key: String, model: String) -> Self {
        Self {
            inner: LlmConfig::openai(api_key, model),
        }
    }

    #[staticmethod]
    fn anthropic(api_key: String, model: String) -> Self {
        Self {
            inner: LlmConfig::anthropic(api_key, model),
        }
    }

    #[staticmethod]
    fn huggingface(api_key: String, model: String) -> Self {
        Self {
            inner: LlmConfig::huggingface(api_key, model),
        }
    }

    fn provider_name(&self) -> String {
        self.inner.provider_name().to_string()
    }

    fn model_name(&self) -> String {
        self.inner.model_name().to_string()
    }
}

/// Python wrapper for agent capabilities
#[pyclass]
#[derive(Clone)]
pub struct PyAgentCapability {
    inner: AgentCapability,
}

#[pymethods]
impl PyAgentCapability {
    #[staticmethod]
    fn text_processing() -> Self {
        Self {
            inner: AgentCapability::TextProcessing,
        }
    }

    #[staticmethod]
    fn data_analysis() -> Self {
        Self {
            inner: AgentCapability::DataAnalysis,
        }
    }

    #[staticmethod]
    fn tool_execution() -> Self {
        Self {
            inner: AgentCapability::ToolExecution,
        }
    }

    #[staticmethod]
    fn decision_making() -> Self {
        Self {
            inner: AgentCapability::DecisionMaking,
        }
    }

    #[staticmethod]
    fn custom(name: String) -> Self {
        Self {
            inner: AgentCapability::Custom(name),
        }
    }

    /// Get the string representation of this capability
    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Python wrapper for embedding configuration
#[pyclass]
#[derive(Clone)]
pub struct PyEmbeddingConfig {
    inner: EmbeddingConfig,
}

#[pymethods]
impl PyEmbeddingConfig {
    #[staticmethod]
    fn openai(api_key: String, model: String) -> Self {
        Self {
            inner: EmbeddingConfig {
                provider: EmbeddingProvider::OpenAI,
                api_key,
                model,
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
            },
        }
    }

    #[staticmethod]
    fn huggingface(api_key: String, model: String) -> Self {
        Self {
            inner: EmbeddingConfig {
                provider: EmbeddingProvider::HuggingFace,
                api_key,
                model,
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
            },
        }
    }

    fn with_base_url(&self, base_url: String) -> Self {
        let mut config = self.inner.clone();
        config.base_url = Some(base_url);
        Self { inner: config }
    }

    fn with_timeout(&self, timeout_seconds: u64) -> Self {
        let mut config = self.inner.clone();
        config.timeout_seconds = Some(timeout_seconds);
        Self { inner: config }
    }

    fn with_max_batch_size(&self, max_batch_size: usize) -> Self {
        let mut config = self.inner.clone();
        config.max_batch_size = Some(max_batch_size);
        Self { inner: config }
    }

    fn provider_name(&self) -> String {
        match self.inner.provider {
            EmbeddingProvider::OpenAI => "openai".to_string(),
            EmbeddingProvider::HuggingFace => "huggingface".to_string(),
        }
    }

    fn model_name(&self) -> String {
        self.inner.model.clone()
    }

    fn __str__(&self) -> String {
        format!(
            "EmbeddingConfig(provider={}, model={})",
            self.provider_name(),
            self.model_name()
        )
    }
}

/// Python wrapper for embedding service
#[pyclass]
pub struct PyEmbeddingService {
    inner: Arc<EmbeddingService>,
}

#[pymethods]
impl PyEmbeddingService {
    #[new]
    fn new(config: PyEmbeddingConfig) -> PyResult<Self> {
        let service = EmbeddingService::new(config.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(service),
        })
    }

    /// Generate embeddings for a single text
    fn embed_text<'a>(&self, text: &str, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let service = Arc::clone(&self.inner);
        let text = text.to_string();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let embedding = service
                .embed_text(&text)
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok(embedding)
        })
    }

    /// Generate embeddings for multiple texts
    fn embed_texts<'a>(&self, texts: Vec<String>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let service = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let embeddings = service
                .embed_texts(&texts)
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok(embeddings)
        })
    }

    /// Calculate cosine similarity between two embeddings
    #[staticmethod]
    fn cosine_similarity(a: Vec<f32>, b: Vec<f32>) -> PyResult<f32> {
        EmbeddingService::cosine_similarity(&a, &b)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Get embedding dimensions for the current provider
    fn get_dimensions<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let service = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let dimensions = service
                .get_dimensions()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok(dimensions)
        })
    }

    /// Get provider information
    fn get_provider_info(&self) -> (String, String) {
        self.inner.get_provider_info()
    }

    fn __str__(&self) -> String {
        let (provider, model) = self.inner.get_provider_info();
        format!("EmbeddingService(provider={}, model={})", provider, model)
    }
}

/// Python wrapper for embedding request
#[pyclass]
#[derive(Clone)]
pub struct PyEmbeddingRequest {
    inner: EmbeddingRequest,
}

#[pymethods]
impl PyEmbeddingRequest {
    #[new]
    fn new(text: String) -> Self {
        Self {
            inner: EmbeddingRequest {
                input: EmbeddingInput::Single(text),
                user: None,
                params: HashMap::new(),
            },
        }
    }

    #[staticmethod]
    fn multiple(texts: Vec<String>) -> Self {
        Self {
            inner: EmbeddingRequest {
                input: EmbeddingInput::Multiple(texts),
                user: None,
                params: HashMap::new(),
            },
        }
    }

    fn with_user(&self, user: String) -> Self {
        let mut request = self.inner.clone();
        request.user = Some(user);
        Self { inner: request }
    }

    fn input_count(&self) -> usize {
        self.inner.input.len()
    }

    fn __str__(&self) -> String {
        format!("EmbeddingRequest(inputs={})", self.input_count())
    }
}

/// Python wrapper for embedding response
#[pyclass]
#[derive(Clone)]
pub struct PyEmbeddingResponse {
    inner: EmbeddingResponse,
}

#[pymethods]
impl PyEmbeddingResponse {
    fn embeddings(&self) -> Vec<Vec<f32>> {
        self.inner.embeddings.clone()
    }

    fn model(&self) -> String {
        self.inner.model.clone()
    }

    fn prompt_tokens(&self) -> u32 {
        self.inner.usage.prompt_tokens
    }

    fn total_tokens(&self) -> u32 {
        self.inner.usage.total_tokens
    }

    fn embedding_count(&self) -> usize {
        self.inner.embeddings.len()
    }

    fn dimensions(&self) -> usize {
        self.inner.embeddings.first().map(|e| e.len()).unwrap_or(0)
    }

    fn __str__(&self) -> String {
        format!(
            "EmbeddingResponse(embeddings={}, dimensions={}, tokens={})",
            self.embedding_count(),
            self.dimensions(),
            self.total_tokens()
        )
    }
}

/// Python wrapper for workflow nodes
#[pyclass]
#[derive(Clone)]
pub struct PyWorkflowNode {
    inner: WorkflowNode,
}

#[pymethods]
impl PyWorkflowNode {
    #[staticmethod]
    fn agent_node(
        name: String,
        description: String,
        agent_id: String,
        prompt: String,
    ) -> PyResult<Self> {
        let agent_id = AgentId::from_string(&agent_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid agent ID: {}", e))
        })?;

        let node = WorkflowNode::new(
            name,
            description,
            NodeType::Agent {
                agent_id,
                prompt_template: prompt,
            },
        );

        Ok(Self { inner: node })
    }

    #[staticmethod]
    fn condition_node(name: String, description: String, expression: String) -> Self {
        let node = WorkflowNode::new(name, description, NodeType::Condition { expression });

        Self { inner: node }
    }

    #[staticmethod]
    fn transform_node(name: String, description: String, transformation: String) -> Self {
        let node = WorkflowNode::new(name, description, NodeType::Transform { transformation });

        Self { inner: node }
    }

    #[staticmethod]
    fn delay_node(name: String, description: String, duration_seconds: u64) -> Self {
        let node = WorkflowNode::new(name, description, NodeType::Delay { duration_seconds });

        Self { inner: node }
    }

    #[staticmethod]
    fn document_loader_node(
        name: String,
        description: String,
        document_type: String,
        source_path: String,
    ) -> PyResult<Self> {
        // Validate document type
        let supported_types = ["pdf", "txt", "docx", "json", "csv", "xml", "html"];
        if !supported_types.contains(&document_type.to_lowercase().as_str()) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Unsupported document type: {}. Supported types: {:?}",
                document_type, supported_types
            )));
        }

        let node = WorkflowNode::new(
            name,
            description,
            NodeType::DocumentLoader {
                document_type,
                source_path,
                encoding: None,
            },
        );

        Ok(Self { inner: node })
    }

    fn id(&self) -> String {
        self.inner.id.to_string()
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn description(&self) -> String {
        self.inner.description.clone()
    }
}

/// Python wrapper for workflow edges
#[pyclass]
#[derive(Clone)]
pub struct PyWorkflowEdge {
    inner: WorkflowEdge,
}

#[pymethods]
impl PyWorkflowEdge {
    #[staticmethod]
    fn data_flow() -> Self {
        Self {
            inner: WorkflowEdge::data_flow(),
        }
    }

    #[staticmethod]
    fn control_flow() -> Self {
        Self {
            inner: WorkflowEdge::control_flow(),
        }
    }

    #[staticmethod]
    fn conditional(condition: String) -> Self {
        Self {
            inner: WorkflowEdge::conditional(condition),
        }
    }
}

/// Python wrapper for workflows
#[pyclass]
pub struct PyWorkflow {
    inner: Workflow,
}

#[pymethods]
impl PyWorkflow {
    #[new]
    fn new(name: String, description: String) -> Self {
        Self {
            inner: Workflow::new(name, description),
        }
    }

    fn add_node(&mut self, node: PyWorkflowNode) -> PyResult<String> {
        let node_id = self
            .inner
            .add_node(node.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(node_id.to_string())
    }

    fn connect_nodes(
        &mut self,
        from_id: String,
        to_id: String,
        edge: PyWorkflowEdge,
    ) -> PyResult<()> {
        let from = NodeId::from_string(&from_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid from node ID: {}", e))
        })?;
        let to = NodeId::from_string(&to_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid to node ID: {}", e))
        })?;

        self.inner
            .connect_nodes(from, to, edge.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }

    fn id(&self) -> String {
        self.inner.id.to_string()
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn description(&self) -> String {
        self.inner.description.clone()
    }

    fn node_count(&self) -> usize {
        self.inner.graph.node_count()
    }

    fn edge_count(&self) -> usize {
        self.inner.graph.edge_count()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    #[staticmethod]
    fn from_json(json_str: String) -> PyResult<PyWorkflow> {
        let workflow: Workflow = serde_json::from_str(&json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(PyWorkflow { inner: workflow })
    }
}

/// Python wrapper for workflow builder
#[pyclass]
pub struct PyWorkflowBuilder {
    name: String,
    description: Option<String>,
    nodes: Vec<(PyWorkflowNode, String)>, // (node, node_id)
    connections: Vec<(String, String, PyWorkflowEdge)>, // (from_id, to_id, edge)
}

#[pymethods]
impl PyWorkflowBuilder {
    #[new]
    fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    fn description(&mut self, description: String) -> PyResult<()> {
        self.description = Some(description);
        Ok(())
    }

    fn add_node(&mut self, node: PyWorkflowNode) -> PyResult<String> {
        // Generate a simple node ID for tracking
        let node_id = format!("node_{}", self.nodes.len());
        self.nodes.push((node, node_id.clone()));
        Ok(node_id)
    }

    fn connect(&mut self, from_id: String, to_id: String, edge: PyWorkflowEdge) -> PyResult<()> {
        self.connections.push((from_id, to_id, edge));
        Ok(())
    }

    fn build(&self) -> PyResult<PyWorkflow> {
        // Create a new workflow builder
        let mut builder = WorkflowBuilder::new(&self.name);

        // Set description if provided
        if let Some(ref desc) = self.description {
            builder = builder.description(desc);
        }

        // Track actual node IDs from the builder
        let mut node_id_map = std::collections::HashMap::with_capacity(self.nodes.len().max(8));

        // Add all nodes
        for (py_node, temp_id) in &self.nodes {
            let (new_builder, actual_node_id) = builder
                .add_node(py_node.inner.clone())
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            builder = new_builder;
            node_id_map.insert(temp_id.clone(), actual_node_id);
        }

        // Add all connections
        for (from_temp_id, to_temp_id, edge) in &self.connections {
            let from_id = node_id_map.get(from_temp_id).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown from node ID: {}",
                    from_temp_id
                ))
            })?;
            let to_id = node_id_map.get(to_temp_id).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown to node ID: {}",
                    to_temp_id
                ))
            })?;

            builder = builder
                .connect(from_id.clone(), to_id.clone(), edge.inner.clone())
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        }

        // Build the final workflow
        let workflow = builder
            .build()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(PyWorkflow { inner: workflow })
    }
}

/// Python wrapper for validation results
#[pyclass]
#[derive(Clone)]
pub struct PyValidationResult {
    inner: ValidationResult,
}

#[pymethods]
impl PyValidationResult {
    fn is_valid(&self) -> bool {
        self.inner.is_valid
    }

    fn errors(&self) -> Vec<String> {
        self.inner
            .errors
            .iter()
            .map(|e| e.message.clone())
            .collect()
    }
}

/// Python wrapper for workflow context
#[pyclass]
#[derive(Clone)]
pub struct PyWorkflowContext {
    inner: WorkflowContext,
}

#[pymethods]
impl PyWorkflowContext {
    fn workflow_id(&self) -> String {
        self.inner.workflow_id.to_string()
    }

    fn state(&self) -> String {
        format!("{:?}", self.inner.state)
    }

    fn is_completed(&self) -> bool {
        matches!(self.inner.state, WorkflowState::Completed)
    }

    fn is_failed(&self) -> bool {
        matches!(self.inner.state, WorkflowState::Failed { .. })
    }

    fn get_variable(&self, key: &str) -> Option<String> {
        self.inner.get_variable(key).map(|v| v.to_string())
    }

    fn variables(&self) -> Vec<(String, String)> {
        self.inner
            .variables
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect()
    }

    fn execution_time_ms(&self) -> u64 {
        if let Some(completed_at) = self.inner.completed_at {
            completed_at
                .signed_duration_since(self.inner.started_at)
                .num_milliseconds() as u64
        } else {
            chrono::Utc::now()
                .signed_duration_since(self.inner.started_at)
                .num_milliseconds() as u64
        }
    }
}

/// Python wrapper for workflow executor
#[pyclass]
pub struct PyWorkflowExecutor {
    llm_config: PyLlmConfig,
    max_concurrency: Option<usize>,
    max_node_execution_time_ms: Option<u64>,
    fail_fast: Option<bool>,
    retry_config: Option<PyRetryConfig>,
    circuit_breaker_config: Option<PyCircuitBreakerConfig>,
}

#[pymethods]
impl PyWorkflowExecutor {
    /// Create a new workflow executor with default settings
    #[new]
    fn new(llm_config: PyLlmConfig) -> Self {
        Self {
            llm_config,
            max_concurrency: None,
            max_node_execution_time_ms: None,
            fail_fast: None,
            retry_config: None,
            circuit_breaker_config: None,
        }
    }

    /// Create a workflow executor optimized for high throughput
    #[staticmethod]
    fn new_high_throughput(llm_config: PyLlmConfig) -> Self {
        Self {
            llm_config,
            max_concurrency: Some(50),
            max_node_execution_time_ms: None,
            fail_fast: Some(false),
            retry_config: Some(PyRetryConfig::default()),
            circuit_breaker_config: Some(PyCircuitBreakerConfig::default()),
        }
    }

    /// Create a workflow executor optimized for low latency
    #[staticmethod]
    fn new_low_latency(llm_config: PyLlmConfig) -> Self {
        Self {
            llm_config,
            max_concurrency: Some(10),
            max_node_execution_time_ms: Some(5000),
            fail_fast: Some(true),
            retry_config: None,
            circuit_breaker_config: Some(PyCircuitBreakerConfig::default()),
        }
    }

    /// Create a workflow executor optimized for memory usage
    #[staticmethod]
    fn new_memory_optimized(llm_config: PyLlmConfig) -> Self {
        Self {
            llm_config,
            max_concurrency: Some(20),
            max_node_execution_time_ms: None,
            fail_fast: Some(false),
            retry_config: Some(PyRetryConfig::default()),
            circuit_breaker_config: Some(PyCircuitBreakerConfig::default()),
        }
    }

    /// Set maximum node execution time
    fn with_max_node_execution_time(&self, timeout_ms: u64) -> Self {
        Self {
            llm_config: self.llm_config.clone(),
            max_concurrency: self.max_concurrency,
            max_node_execution_time_ms: Some(timeout_ms),
            fail_fast: self.fail_fast,
            retry_config: self.retry_config.clone(),
            circuit_breaker_config: self.circuit_breaker_config.clone(),
        }
    }

    /// Set fail fast behavior
    fn with_fail_fast(&self, fail_fast: bool) -> Self {
        Self {
            llm_config: self.llm_config.clone(),
            max_concurrency: self.max_concurrency,
            max_node_execution_time_ms: self.max_node_execution_time_ms,
            fail_fast: Some(fail_fast),
            retry_config: self.retry_config.clone(),
            circuit_breaker_config: self.circuit_breaker_config.clone(),
        }
    }

    /// Set retry configuration
    fn with_retry_config(&self, retry_config: PyRetryConfig) -> Self {
        Self {
            llm_config: self.llm_config.clone(),
            max_concurrency: self.max_concurrency,
            max_node_execution_time_ms: self.max_node_execution_time_ms,
            fail_fast: self.fail_fast,
            retry_config: Some(retry_config),
            circuit_breaker_config: self.circuit_breaker_config.clone(),
        }
    }

    /// Set circuit breaker configuration
    fn with_circuit_breaker_config(&self, circuit_breaker_config: PyCircuitBreakerConfig) -> Self {
        Self {
            llm_config: self.llm_config.clone(),
            max_concurrency: self.max_concurrency,
            max_node_execution_time_ms: self.max_node_execution_time_ms,
            fail_fast: self.fail_fast,
            retry_config: self.retry_config.clone(),
            circuit_breaker_config: Some(circuit_breaker_config),
        }
    }

    /// Disable retries
    fn without_retries(&self) -> Self {
        Self {
            llm_config: self.llm_config.clone(),
            max_concurrency: self.max_concurrency,
            max_node_execution_time_ms: self.max_node_execution_time_ms,
            fail_fast: self.fail_fast,
            retry_config: None,
            circuit_breaker_config: self.circuit_breaker_config.clone(),
        }
    }

    /// Execute a workflow with optimized performance
    fn execute(&self, workflow: &PyWorkflow) -> PyResult<PyWorkflowContext> {
        // Use a shared runtime instead of creating new ones
        static GLOBAL_RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
        let rt = GLOBAL_RT.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(num_cpus::get())
                .thread_name("graphbit-global")
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime")
        });

        rt.block_on(self.execute_async_internal(workflow))
    }

    /// Execute workflow asynchronously (returns a coroutine for Python await)
    fn execute_async<'a>(
        &self,
        workflow: &PyWorkflow,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let workflow_clone = workflow.inner.clone();
        // Clone the configuration fields to avoid lifetime issues
        let llm_config = self.llm_config.clone();
        let max_node_execution_time_ms = self.max_node_execution_time_ms;
        let fail_fast = self.fail_fast;
        let retry_config = self.retry_config.clone();
        let circuit_breaker_config = self.circuit_breaker_config.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Recreate executor config from cloned values
            let executor_config = PyWorkflowExecutor {
                llm_config,
                max_concurrency: None,
                max_node_execution_time_ms,
                fail_fast,
                retry_config,
                circuit_breaker_config,
            };
            executor_config
                .execute_async_internal_core(workflow_clone)
                .await
        })
    }

    /// Optimized concurrent execution with minimal overhead
    fn execute_concurrent(
        &self,
        workflows: Vec<Py<PyWorkflow>>,
    ) -> PyResult<Vec<PyWorkflowContext>> {
        // Use a more efficient runtime setup
        static GLOBAL_RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
        let rt = GLOBAL_RT.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(std::cmp::min(num_cpus::get(), 6)) // Limit worker threads
                .thread_name("graphbit-worker")
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime")
        });

        rt.block_on(async {
            // Create a single optimized executor for all workflows
            let executor = WorkflowExecutor::new_low_latency() // Use low-latency for better concurrent performance
                .with_max_node_execution_time(self.max_node_execution_time_ms.unwrap_or(15000))
                .with_fail_fast(self.fail_fast.unwrap_or(true));

            // Extract workflows once to minimize GIL usage
            let rust_workflows = Python::with_gil(|py| {
                workflows
                    .iter()
                    .map(|workflow_py| workflow_py.borrow(py).inner.clone())
                    .collect::<Vec<_>>()
            });

            // Pre-register a single shared agent to avoid repeated agent creation
            let agent_ids: Vec<_> = rust_workflows
                .iter()
                .flat_map(extract_agent_ids_from_workflow)
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            for agent_id_str in &agent_ids {
                let agent_id = AgentId::from_string(agent_id_str).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid agent ID: {}",
                        e
                    ))
                })?;

                let agent_config = graphbit_core::agents::AgentConfig::new(
                    format!("Agent-{}", agent_id),
                    "Optimized concurrent agent",
                    self.llm_config.inner.clone(),
                )
                .with_max_tokens(2048)
                .with_temperature(0.1);

                let agent = graphbit_core::agents::Agent::new(agent_config)
                    .await
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to create agent: {}",
                            e
                        ))
                    })?;

                executor.register_agent(Arc::new(agent)).await;
            }

            // Use the new optimized execute_batch method for true concurrent execution
            let results = executor.execute_batch(rust_workflows).await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Batch execution failed: {}",
                    e
                ))
            })?;

            Ok(results
                .into_iter()
                .map(|ctx| PyWorkflowContext { inner: ctx })
                .collect())
        })
    }

    /// Execute multiple workflows in batch (alias for execute_concurrent for compatibility)
    fn execute_batch(&self, workflows: Vec<Py<PyWorkflow>>) -> PyResult<Vec<PyWorkflowContext>> {
        self.execute_concurrent(workflows)
    }

    /// Execute concurrent agent tasks with maximum performance (bypasses workflow overhead)
    /// This method is optimized for benchmarking and high-throughput scenarios
    fn execute_concurrent_agent_tasks(
        &self,
        prompts: Vec<String>,
        agent_id: String,
    ) -> PyResult<Vec<String>> {
        pyo3_async_runtimes::tokio::get_runtime().block_on(async {
            // Parse agent ID
            let parsed_agent_id = AgentId::from_string(&agent_id).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid agent ID: {}", e))
            })?;

            // Create optimized executor for concurrent agent tasks
            let executor = WorkflowExecutor::new_high_throughput()
                .with_max_node_execution_time(self.max_node_execution_time_ms.unwrap_or(30000))
                .with_fail_fast(false); // Don't fail fast for batch operations

            // Register the agent
            let agent_config = graphbit_core::agents::AgentConfig::new(
                format!("ConcurrentAgent-{}", parsed_agent_id),
                "High-performance concurrent agent",
                self.llm_config.inner.clone(),
            )
            .with_id(parsed_agent_id.clone())
            .with_max_tokens(2048) // Optimized for speed
            .with_temperature(0.1); // Consistent results

            let agent = graphbit_core::agents::Agent::new(agent_config)
                .await
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to create agent: {}",
                        e
                    ))
                })?;

            executor.register_agent(Arc::new(agent)).await;

            // Execute concurrent agent tasks
            let results = executor
                .execute_concurrent_agent_tasks(prompts, parsed_agent_id)
                .await
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Concurrent agent tasks failed: {}",
                        e
                    ))
                })?;

            // Convert results to Python, handling errors as strings
            let py_results: Vec<String> = results
                .into_iter()
                .map(|result| match result {
                    Ok(value) => {
                        // Convert JSON value to string
                        match value {
                            serde_json::Value::String(s) => s,
                            other => other.to_string(),
                        }
                    }
                    Err(e) => format!("ERROR: Agent task failed: {}", e),
                })
                .collect();

            Ok(py_results)
        })
    }
}

impl PyWorkflowExecutor {
    /// Internal async execution method for better error handling
    async fn execute_async_internal(&self, workflow: &PyWorkflow) -> PyResult<PyWorkflowContext> {
        self.execute_async_internal_core(workflow.inner.clone())
            .await
    }

    /// Core async execution logic
    async fn execute_async_internal_core(&self, workflow: Workflow) -> PyResult<PyWorkflowContext> {
        // Use default constructor since we removed pool configuration
        let mut executor = WorkflowExecutor::new();

        // Apply performance configurations
        if let Some(timeout_ms) = self.max_node_execution_time_ms {
            executor = executor.with_max_node_execution_time(timeout_ms);
        }

        if let Some(fail_fast) = self.fail_fast {
            executor = executor.with_fail_fast(fail_fast);
        }

        // Apply retry configuration for resilience
        if let Some(retry_config) = &self.retry_config {
            executor = executor.with_retry_config(retry_config.inner.clone());
        }

        // Apply circuit breaker configuration
        if let Some(circuit_breaker_config) = &self.circuit_breaker_config {
            executor = executor.with_circuit_breaker_config(circuit_breaker_config.inner.clone());
        }

        // FIXED: Extract agent IDs and create agents with proper error handling
        let agent_ids = extract_agent_ids_from_workflow(&workflow);

        for agent_id_str in agent_ids {
            // Parse the agent ID - this should match exactly what's stored in the workflow node
            let agent_id = AgentId::from_string(&agent_id_str).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid agent ID: {}", e))
            })?;

            // Create agent with optimized configuration
            let agent_config = graphbit_core::agents::AgentConfig::new(
                format!("Agent-{}", agent_id),
                "Auto-generated agent for workflow execution",
                self.llm_config.inner.clone(),
            )
            .with_max_tokens(4096) // Increased for better performance
            .with_temperature(0.1) // Lower temperature for consistent results
            .with_id(agent_id.clone()); // FIXED: Set the agent ID explicitly

            let agent = graphbit_core::agents::Agent::new(agent_config)
                .await
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to create agent {}: {}",
                        agent_id, e
                    ))
                })?;

            // Register agent with the executor
            executor.register_agent(Arc::new(agent)).await;
        }

        // Execute workflow with performance monitoring
        let start_time = std::time::Instant::now();
        let result = executor.execute(workflow).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Workflow execution failed: {}",
                e
            ))
        })?;

        let execution_time = start_time.elapsed();

        // Add performance metadata
        let mut context = result;
        context.set_metadata(
            "execution_time_ms".to_string(),
            serde_json::json!(execution_time.as_millis()),
        );
        context.set_metadata(
            "concurrency_system".to_string(),
            serde_json::json!("simplified"),
        );

        Ok(PyWorkflowContext { inner: context })
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

/// Python wrapper for retry configuration
#[pyclass]
#[derive(Clone)]
pub struct PyRetryConfig {
    inner: RetryConfig,
}

#[pymethods]
impl PyRetryConfig {
    #[new]
    fn new(max_attempts: u32) -> Self {
        Self {
            inner: RetryConfig::new(max_attempts),
        }
    }

    /// Create default retry configuration
    #[staticmethod]
    fn default() -> Self {
        Self {
            inner: RetryConfig::default(),
        }
    }

    /// Configure exponential backoff
    fn with_exponential_backoff(
        &self,
        initial_delay_ms: u64,
        multiplier: f64,
        max_delay_ms: u64,
    ) -> Self {
        Self {
            inner: self.inner.clone().with_exponential_backoff(
                initial_delay_ms,
                multiplier,
                max_delay_ms,
            ),
        }
    }

    /// Configure jitter for randomness
    fn with_jitter(&self, jitter_factor: f64) -> Self {
        Self {
            inner: self.inner.clone().with_jitter(jitter_factor),
        }
    }

    /// Get max attempts
    fn max_attempts(&self) -> u32 {
        self.inner.max_attempts
    }

    /// Get initial delay
    fn initial_delay_ms(&self) -> u64 {
        self.inner.initial_delay_ms
    }
}

/// Python wrapper for circuit breaker configuration
#[pyclass]
#[derive(Clone)]
pub struct PyCircuitBreakerConfig {
    inner: CircuitBreakerConfig,
}

#[pymethods]
impl PyCircuitBreakerConfig {
    #[new]
    fn new(failure_threshold: u32, recovery_timeout_ms: u64) -> Self {
        Self {
            inner: CircuitBreakerConfig {
                failure_threshold,
                recovery_timeout_ms,
                success_threshold: 3,
                failure_window_ms: 60000,
            },
        }
    }

    /// Create default circuit breaker configuration
    #[staticmethod]
    fn default() -> Self {
        Self {
            inner: CircuitBreakerConfig::default(),
        }
    }

    /// Get failure threshold
    fn failure_threshold(&self) -> u32 {
        self.inner.failure_threshold
    }

    /// Get recovery timeout
    fn recovery_timeout_ms(&self) -> u64 {
        self.inner.recovery_timeout_ms
    }
}

/// Initialize the GraphBit library
#[pyfunction]
fn init() -> PyResult<()> {
    graphbit_core::init()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(())
}

/// Get the GraphBit version
#[pyfunction]
fn version() -> String {
    graphbit_core::VERSION.to_string()
}

/// Python module definition
#[pymodule]
fn graphbit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLlmConfig>()?;
    m.add_class::<PyAgentCapability>()?;

    // Embedding classes
    m.add_class::<PyEmbeddingConfig>()?;
    m.add_class::<PyEmbeddingService>()?;
    m.add_class::<PyEmbeddingRequest>()?;
    m.add_class::<PyEmbeddingResponse>()?;

    m.add_class::<PyWorkflowNode>()?;
    m.add_class::<PyWorkflowEdge>()?;
    m.add_class::<PyWorkflow>()?;
    m.add_class::<PyWorkflowBuilder>()?;
    m.add_class::<PyValidationResult>()?;
    m.add_class::<PyWorkflowContext>()?;
    m.add_class::<PyWorkflowExecutor>()?;
    m.add_class::<PyRetryConfig>()?;
    m.add_class::<PyCircuitBreakerConfig>()?;

    // Dynamic Graph classes
    m.add_class::<PyDynamicGraphConfig>()?;
    m.add_class::<PyDynamicGraphAnalytics>()?;
    m.add_class::<PyDynamicGraphManager>()?;
    m.add_class::<PyDynamicNodeResponse>()?;
    m.add_class::<PySuggestedConnection>()?;
    m.add_class::<PyAutoCompletionResult>()?;
    m.add_class::<PyWorkflowAutoCompletion>()?;

    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

// Dynamic Graph Python Bindings

/// Python wrapper for Dynamic Graph Configuration
#[pyclass]
#[derive(Clone)]
pub struct PyDynamicGraphConfig {
    inner: graphbit_core::dynamic_graph::DynamicGraphConfig,
}

#[pymethods]
impl PyDynamicGraphConfig {
    #[new]
    fn new() -> Self {
        Self {
            inner: graphbit_core::dynamic_graph::DynamicGraphConfig::default(),
        }
    }

    fn with_max_auto_nodes(&self, max_nodes: usize) -> Self {
        let mut config = self.inner.clone();
        config.max_auto_nodes = max_nodes;
        Self { inner: config }
    }

    fn with_confidence_threshold(&self, threshold: f32) -> Self {
        let mut config = self.inner.clone();
        config.confidence_threshold = threshold;
        Self { inner: config }
    }

    fn with_generation_temperature(&self, temperature: f32) -> Self {
        let mut config = self.inner.clone();
        config.generation_temperature = temperature;
        Self { inner: config }
    }

    fn with_max_generation_depth(&self, depth: usize) -> Self {
        let mut config = self.inner.clone();
        config.max_generation_depth = depth;
        Self { inner: config }
    }

    fn with_completion_objectives(&self, objectives: Vec<String>) -> Self {
        let mut config = self.inner.clone();
        config.completion_objectives = objectives;
        Self { inner: config }
    }

    fn enable_validation(&self) -> Self {
        let mut config = self.inner.clone();
        config.validate_nodes = true;
        Self { inner: config }
    }

    fn disable_validation(&self) -> Self {
        let mut config = self.inner.clone();
        config.validate_nodes = false;
        Self { inner: config }
    }

    #[getter]
    fn max_auto_nodes(&self) -> usize {
        self.inner.max_auto_nodes
    }

    #[getter]
    fn confidence_threshold(&self) -> f32 {
        self.inner.confidence_threshold
    }

    #[getter]
    fn generation_temperature(&self) -> f32 {
        self.inner.generation_temperature
    }

    fn __str__(&self) -> String {
        format!(
            "PyDynamicGraphConfig(max_nodes={}, confidence={}, temperature={}, depth={})",
            self.inner.max_auto_nodes,
            self.inner.confidence_threshold,
            self.inner.generation_temperature,
            self.inner.max_generation_depth
        )
    }
}

/// Python wrapper for Dynamic Graph Analytics
#[pyclass]
#[derive(Clone)]
pub struct PyDynamicGraphAnalytics {
    inner: graphbit_core::dynamic_graph::DynamicGraphAnalytics,
}

#[pymethods]
impl PyDynamicGraphAnalytics {
    #[getter]
    fn total_nodes_generated(&self) -> usize {
        self.inner.total_nodes_generated
    }

    #[getter]
    fn successful_generations(&self) -> usize {
        self.inner.successful_generations
    }

    #[getter]
    fn failed_generations(&self) -> usize {
        self.inner.failed_generations
    }

    #[getter]
    fn avg_confidence(&self) -> f32 {
        self.inner.avg_confidence
    }

    #[getter]
    fn cache_hit_rate(&self) -> f32 {
        self.inner.cache_hit_rate
    }

    fn get_generation_times(&self) -> Vec<u64> {
        self.inner.generation_times_ms.clone()
    }

    fn success_rate(&self) -> f32 {
        if self.inner.total_nodes_generated == 0 {
            0.0
        } else {
            self.inner.successful_generations as f32 / self.inner.total_nodes_generated as f32
        }
    }

    fn avg_generation_time(&self) -> f64 {
        if self.inner.generation_times_ms.is_empty() {
            0.0
        } else {
            let sum: u64 = self.inner.generation_times_ms.iter().sum();
            sum as f64 / self.inner.generation_times_ms.len() as f64
        }
    }

    fn __str__(&self) -> String {
        format!(
            "PyDynamicGraphAnalytics(total={}, successful={}, failed={}, success_rate={:.2}%, avg_confidence={:.2}, cache_hit_rate={:.2}%)",
            self.inner.total_nodes_generated,
            self.inner.successful_generations,
            self.inner.failed_generations,
            self.success_rate() * 100.0,
            self.inner.avg_confidence,
            self.inner.cache_hit_rate * 100.0
        )
    }
}

/// Python wrapper for Dynamic Graph Manager
#[pyclass]
pub struct PyDynamicGraphManager {
    // Store as Option to allow for async initialization
    manager: Option<graphbit_core::dynamic_graph::DynamicGraphManager>,
}

#[pymethods]
impl PyDynamicGraphManager {
    #[new]
    fn new(_llm_config: PyLlmConfig, _config: PyDynamicGraphConfig) -> Self {
        Self {
            manager: None, // Will be initialized asynchronously
        }
    }

    /// Initialize the dynamic graph manager (must be called before use)
    fn initialize(
        &mut self,
        llm_config: PyLlmConfig,
        config: PyDynamicGraphConfig,
    ) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create async runtime: {}",
                e
            ))
        })?;

        let manager = rt
            .block_on(async {
                graphbit_core::dynamic_graph::DynamicGraphManager::new(
                    llm_config.inner,
                    config.inner,
                )
                .await
            })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to initialize dynamic graph manager: {}",
                    e
                ))
            })?;

        self.manager = Some(manager);
        Ok(())
    }

    /// Generate a dynamic node based on objective
    fn generate_node(
        &self,
        workflow_id: String,
        context: &PyWorkflowContext,
        objective: String,
        allowed_node_types: Option<Vec<String>>,
    ) -> PyResult<PyDynamicNodeResponse> {
        let manager = self.manager.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Manager not initialized. Call initialize() first.",
            )
        })?;

        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create async runtime: {}",
                e
            ))
        })?;

        let workflow_id = WorkflowId::from_string(&workflow_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid workflow ID: {}", e))
        })?;

        let mut constraints = graphbit_core::dynamic_graph::NodeGenerationConstraints::default();
        if let Some(node_types) = allowed_node_types {
            constraints.allowed_node_types = node_types;
        }

        let request = graphbit_core::dynamic_graph::DynamicNodeRequest {
            workflow_id,
            context: context.inner.clone(),
            objective,
            previous_outputs: std::collections::HashMap::new(),
            constraints,
        };

        let response = rt
            .block_on(async { manager.generate_node(request).await })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to generate dynamic node: {}",
                    e
                ))
            })?;

        Ok(PyDynamicNodeResponse { inner: response })
    }

    /// Auto-complete a workflow
    fn auto_complete_workflow(
        &self,
        workflow: &mut PyWorkflow,
        context: &PyWorkflowContext,
        objectives: Vec<String>,
    ) -> PyResult<PyAutoCompletionResult> {
        let manager = self.manager.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Manager not initialized. Call initialize() first.",
            )
        })?;

        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create async runtime: {}",
                e
            ))
        })?;

        let generated_nodes = rt
            .block_on(async {
                manager
                    .auto_complete_workflow(&mut workflow.inner.graph, &context.inner, &objectives)
                    .await
            })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to auto-complete workflow: {}",
                    e
                ))
            })?;

        let analytics = rt.block_on(async { manager.get_analytics().await });

        Ok(PyAutoCompletionResult {
            inner: graphbit_core::dynamic_graph::AutoCompletionResult {
                generated_nodes,
                completion_time_ms: 0, // TODO: Track actual time
                success: true,
                analytics,
            },
        })
    }

    /// Get analytics
    fn get_analytics(&self) -> PyResult<PyDynamicGraphAnalytics> {
        let manager = self.manager.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Manager not initialized. Call initialize() first.",
            )
        })?;

        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create async runtime: {}",
                e
            ))
        })?;

        let analytics = rt.block_on(async { manager.get_analytics().await });

        Ok(PyDynamicGraphAnalytics { inner: analytics })
    }
}

/// Python wrapper for Dynamic Node Response
#[pyclass]
#[derive(Clone)]
pub struct PyDynamicNodeResponse {
    inner: graphbit_core::dynamic_graph::DynamicNodeResponse,
}

#[pymethods]
impl PyDynamicNodeResponse {
    fn get_node(&self) -> PyWorkflowNode {
        PyWorkflowNode {
            inner: self.inner.node.clone(),
        }
    }

    #[getter]
    fn confidence(&self) -> f32 {
        self.inner.confidence
    }

    #[getter]
    fn reasoning(&self) -> String {
        self.inner.reasoning.clone()
    }

    #[getter]
    fn completes_workflow(&self) -> bool {
        self.inner.completes_workflow
    }

    fn get_suggested_connections(&self) -> Vec<PySuggestedConnection> {
        self.inner
            .suggested_connections
            .iter()
            .map(|conn| PySuggestedConnection {
                inner: conn.clone(),
            })
            .collect()
    }

    fn __str__(&self) -> String {
        format!(
            "PyDynamicNodeResponse(confidence={:.2}, completes_workflow={}, connections={})",
            self.inner.confidence,
            self.inner.completes_workflow,
            self.inner.suggested_connections.len()
        )
    }
}

/// Python wrapper for Suggested Connection
#[pyclass]
#[derive(Clone)]
pub struct PySuggestedConnection {
    inner: graphbit_core::dynamic_graph::SuggestedConnection,
}

#[pymethods]
impl PySuggestedConnection {
    #[getter]
    fn source_node_id(&self) -> String {
        self.inner.from_node.to_string()
    }

    #[getter]
    fn to_node(&self) -> String {
        self.inner.to_node.to_string()
    }

    #[getter]
    fn edge_type(&self) -> String {
        self.inner.edge_type.clone()
    }

    #[getter]
    fn confidence(&self) -> f32 {
        self.inner.confidence
    }

    #[getter]
    fn reasoning(&self) -> String {
        self.inner.reasoning.clone()
    }

    fn __str__(&self) -> String {
        format!(
            "PySuggestedConnection({} -> {}, type={}, confidence={:.2})",
            self.source_node_id(),
            self.to_node(),
            self.inner.edge_type,
            self.inner.confidence
        )
    }
}

/// Python wrapper for Auto-Completion Result
#[pyclass]
#[derive(Clone)]
pub struct PyAutoCompletionResult {
    inner: graphbit_core::dynamic_graph::AutoCompletionResult,
}

#[pymethods]
impl PyAutoCompletionResult {
    fn get_generated_nodes(&self) -> Vec<String> {
        self.inner
            .generated_nodes
            .iter()
            .map(|id| id.to_string())
            .collect()
    }

    #[getter]
    fn completion_time_ms(&self) -> u64 {
        self.inner.completion_time_ms
    }

    #[getter]
    fn success(&self) -> bool {
        self.inner.success
    }

    fn get_analytics(&self) -> PyDynamicGraphAnalytics {
        PyDynamicGraphAnalytics {
            inner: self.inner.analytics.clone(),
        }
    }

    fn node_count(&self) -> usize {
        self.inner.generated_nodes.len()
    }

    fn __str__(&self) -> String {
        format!(
            "PyAutoCompletionResult(nodes_generated={}, time={}ms, success={})",
            self.inner.generated_nodes.len(),
            self.inner.completion_time_ms,
            self.inner.success
        )
    }
}

/// Python wrapper for Workflow Auto-Completion
#[pyclass]
pub struct PyWorkflowAutoCompletion {
    // Store as Option to allow for async initialization
    engine: Option<graphbit_core::dynamic_graph::WorkflowAutoCompletion>,
}

#[pymethods]
impl PyWorkflowAutoCompletion {
    #[new]
    fn new(_manager: &PyDynamicGraphManager) -> PyResult<Self> {
        // This is a placeholder - in practice, we'd need async initialization
        Ok(Self { engine: None })
    }

    /// Initialize with a dynamic graph manager
    fn initialize(&mut self, manager: &PyDynamicGraphManager) -> PyResult<()> {
        let graph_manager = manager.manager.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Dynamic graph manager not initialized",
            )
        })?;

        let engine =
            graphbit_core::dynamic_graph::WorkflowAutoCompletion::new(graph_manager.clone());
        self.engine = Some(engine);
        Ok(())
    }

    /// Auto-complete a workflow
    fn complete_workflow(
        &self,
        workflow: &mut PyWorkflow,
        context: &PyWorkflowContext,
        objectives: Vec<String>,
    ) -> PyResult<PyAutoCompletionResult> {
        let engine = self.engine.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Engine not initialized. Call initialize() first.",
            )
        })?;

        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create async runtime: {}",
                e
            ))
        })?;

        let result = rt
            .block_on(async {
                engine
                    .complete_workflow(&mut workflow.inner.graph, &context.inner, objectives)
                    .await
            })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to complete workflow: {}",
                    e
                ))
            })?;

        Ok(PyAutoCompletionResult { inner: result })
    }

    /// Set maximum iterations
    fn with_max_iterations(&self, max_iterations: usize) -> PyResult<Self> {
        let engine = self
            .engine
            .as_ref()
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Engine not initialized")
            })?
            .clone()
            .with_max_iterations(max_iterations);

        Ok(Self {
            engine: Some(engine),
        })
    }

    /// Set timeout
    fn with_timeout_ms(&self, timeout_ms: u64) -> PyResult<Self> {
        let engine = self
            .engine
            .as_ref()
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Engine not initialized")
            })?
            .clone()
            .with_timeout_ms(timeout_ms);

        Ok(Self {
            engine: Some(engine),
        })
    }
}
