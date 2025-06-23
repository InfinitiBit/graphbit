//! This module provides Python bindings for the GraphBit agentic workflow
//! automation framework using PyO3.

#![allow(non_local_definitions)]

use graphbit_core::{
    embeddings::{
        EmbeddingConfig as CoreEmbeddingConfig, EmbeddingProvider, EmbeddingService,
    },
    graph::{NodeType, WorkflowEdge, WorkflowNode},
    llm::{LlmConfig as CoreLlmConfig, LlmMessage, LlmRequest, LlmProviderTrait},
    types::{
        AgentId, NodeId, WorkflowContext, WorkflowState,
    },
    workflow::{Workflow as CoreWorkflow, WorkflowExecutor},
};
use futures::StreamExt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyAny};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ===== VALIDATION HELPERS =====

/// Validate that an API key is provided and not empty
fn validate_api_key(api_key: &str, provider: &str) -> PyResult<()> {
    if api_key.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("{} API key cannot be empty. Please provide a valid API key.", provider)
        ));
    }
    
    // Check for common placeholder values that indicate missing keys
    let invalid_keys = [
        "your-api-key-here",
        "sk-...",
        "claude-...",
        "hf_...",
        "test",
        "demo",
        "placeholder",
    ];
    
    let key_lower = api_key.to_lowercase();
    for invalid in &invalid_keys {
        if key_lower.contains(invalid) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("{} API key appears to be a placeholder. Please provide a valid API key.", provider)
            ));
        }
    }
    
    // Validate minimum length based on provider
    let min_length = match provider.to_lowercase().as_str() {
        "openai" => 20,      // OpenAI keys are typically longer
        "anthropic" => 15,   // Anthropic keys
        "huggingface" => 10, // HuggingFace tokens
        _ => 8,              // Generic minimum
    };
    
    if api_key.len() < min_length {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("{} API key is too short. Please check your API key.", provider)
        ));
    }
    
    Ok(())
}

// ===== OPTIMIZED ERROR HANDLING =====

/// Pre-allocated error types to avoid string conversion overhead
#[derive(Debug, Clone)]
enum OptimizedError {
    NetworkError,
    AuthenticationError, 
    RateLimitError,
    InvalidRequest,
    ModelNotFound,
    QuotaExceeded,
    Timeout,
    Generic(String),
}

impl OptimizedError {
    fn to_py_err(self) -> PyErr {
        match self {
            OptimizedError::NetworkError => PyErr::new::<pyo3::exceptions::PyConnectionError, _>("Network connection failed"),
            OptimizedError::AuthenticationError => PyErr::new::<pyo3::exceptions::PyPermissionError, _>("Authentication failed"),
            OptimizedError::RateLimitError => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Rate limit exceeded"),
            OptimizedError::InvalidRequest => PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid request"),
            OptimizedError::ModelNotFound => PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>("Model not found"),
            OptimizedError::QuotaExceeded => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Quota exceeded"),
            OptimizedError::Timeout => PyErr::new::<pyo3::exceptions::PyTimeoutError, _>("Request timeout"),
            OptimizedError::Generic(msg) => PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg),
        }
    }

    fn from_graphbit_error(error: &graphbit_core::errors::GraphBitError) -> Self {
        let error_str = error.to_string().to_lowercase();
        
        if error_str.contains("network") || error_str.contains("connection") {
            OptimizedError::NetworkError
        } else if error_str.contains("auth") || error_str.contains("unauthorized") {
            OptimizedError::AuthenticationError
        } else if error_str.contains("rate limit") || error_str.contains("too many requests") {
            OptimizedError::RateLimitError
        } else if error_str.contains("invalid") || error_str.contains("bad request") {
            OptimizedError::InvalidRequest
        } else if error_str.contains("model") && error_str.contains("not found") {
            OptimizedError::ModelNotFound
        } else if error_str.contains("quota") || error_str.contains("limit exceeded") {
            OptimizedError::QuotaExceeded
        } else if error_str.contains("timeout") {
            OptimizedError::Timeout
        } else {
            OptimizedError::Generic(error.to_string())
        }
    }
}

// ===== CONNECTION POOL MANAGER =====

#[derive(Clone)]
struct ConnectionPoolManager {
    pools: Arc<tokio::sync::RwLock<HashMap<String, Arc<reqwest::Client>>>>,
}

impl ConnectionPoolManager {
    fn new() -> Self {
        Self {
            pools: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    async fn get_or_create_client(&self, provider_key: &str) -> Arc<reqwest::Client> {
        // Try to get existing client first
        {
            let pools = self.pools.read().await;
            if let Some(client) = pools.get(provider_key) {
                return Arc::clone(client);
            }
        }

        // Create new optimized client
        let client = Arc::new(
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .pool_max_idle_per_host(20) // Increased pool size
                .pool_idle_timeout(std::time::Duration::from_secs(90))
                .tcp_keepalive(std::time::Duration::from_secs(120))
                .tcp_nodelay(true) // Reduce latency
                .http2_prior_knowledge() // Use HTTP/2 when possible
                .build()
                .unwrap_or_else(|_| reqwest::Client::new())
        );

        // Store in pool
        let mut pools = self.pools.write().await;
        pools.insert(provider_key.to_string(), Arc::clone(&client));
        
        client
    }
}

// ===== ZERO-COPY DATA STRUCTURES =====
// RequestBuilder implementation removed for now to avoid PyO3 conversion issues
// Will be re-implemented when needed

// ===== SIMPLIFIED LLM CONFIGURATION =====

/// Python wrapper for LLM configuration
#[pyclass]
#[derive(Clone)]
pub struct LlmConfig {
    inner: CoreLlmConfig,
}

#[pymethods]
impl LlmConfig {
    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn openai(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "OpenAI")?;
        
        Ok(Self {
            inner: CoreLlmConfig::openai(
                api_key,
                model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn anthropic(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "Anthropic")?;
        
        Ok(Self {
            inner: CoreLlmConfig::anthropic(
                api_key,
                model.unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (model=None))]
    fn ollama(model: Option<String>) -> Self {
        // Ollama doesn't require an API key for local usage
        Self {
            inner: CoreLlmConfig::ollama(
                model.unwrap_or_else(|| "llama3.2".to_string()),
            ),
        }
    }

    fn provider(&self) -> String {
        self.inner.provider_name().to_string()
    }

    fn model(&self) -> String {
        self.inner.model_name().to_string()
    }
}

// ===== OPTIMIZED DIRECT LLM CLIENT =====

/// High-performance LLM client with optimized FFI
#[pyclass]
pub struct LlmClient {
    config: LlmConfig,
    provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
    // Optimized connection pool manager
    connection_manager: ConnectionPoolManager,
    // Request buffer pool for zero-copy operations
    buffer_pool: Arc<tokio::sync::RwLock<Vec<Vec<u8>>>>,
}

#[pymethods]
impl LlmClient {
    #[new]
    fn new(config: LlmConfig) -> PyResult<Self> {
        let provider = graphbit_core::llm::LlmProviderFactory::create_provider(config.inner.clone())
            .map_err(|e| OptimizedError::from_graphbit_error(&e).to_py_err())?;
        
        Ok(Self { 
            config, 
            provider: Arc::new(RwLock::new(provider)),
            connection_manager: ConnectionPoolManager::new(),
            buffer_pool: Arc::new(tokio::sync::RwLock::new(Vec::with_capacity(8))),
        })
    }

    /// Optimized async text completion - no block_on()
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_async<'a>(&self, prompt: String, max_tokens: Option<u32>, temperature: Option<f32>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider_arc = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut request = LlmRequest::new(prompt);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let provider = provider_arc.read().await;
            let response = provider.complete(request).await
                .map_err(|e| OptimizedError::from_graphbit_error(&e).to_py_err())?;

            Ok(response.content)
        })
    }

    /// Synchronous version using cached runtime - reduced overhead
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete(&self, prompt: String, max_tokens: Option<u32>, temperature: Option<f32>) -> PyResult<String> {
        // Use existing runtime instead of creating new one
        let handle = tokio::runtime::Handle::try_current()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("No async runtime available"))?;
        
        handle.block_on(async {
            let mut request = LlmRequest::new(prompt);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let provider = self.provider.read().await;
            let response = provider.complete(request).await
                .map_err(|e| OptimizedError::from_graphbit_error(&e).to_py_err())?;

            Ok(response.content)
        })
    }

    /// Batch processing to amortize FFI overhead
    #[pyo3(signature = (prompts, max_tokens=None, temperature=None, max_concurrency=None))]
    fn complete_batch<'a>(
        &self, 
        prompts: Vec<String>, 
        max_tokens: Option<u32>, 
        temperature: Option<f32>,
        max_concurrency: Option<usize>,
        py: Python<'a>
    ) -> PyResult<Bound<'a, PyAny>> {
        let provider_arc = Arc::clone(&self.provider);
        
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let batch_requests: Vec<LlmRequest> = prompts
                .into_iter()
                .map(|prompt| {
                    let mut request = LlmRequest::new(prompt);
                    if let Some(tokens) = max_tokens {
                        request = request.with_max_tokens(tokens);
                    }
                    if let Some(temp) = temperature {
                        request = request.with_temperature(temp);
                    }
                    request
                })
                .collect();

            // Process in parallel with concurrency limit
            let concurrency = max_concurrency.unwrap_or(5);
            let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
            
            let tasks: Vec<_> = batch_requests
                .into_iter()
                .map(|request| {
                    let provider_arc = Arc::clone(&provider_arc);
                    let permit = Arc::clone(&semaphore);
                    
                    tokio::spawn(async move {
                        let _permit = permit.acquire().await.unwrap();
                        let provider = provider_arc.read().await;
                        provider.complete(request).await
                    })
                })
                .collect();

            let results = futures::future::join_all(tasks).await;
            
            let responses: Vec<String> = results
                .into_iter()
                .map(|task_result| {
                    match task_result {
                        Ok(Ok(response)) => response.content,
                        Ok(Err(e)) => format!("Error: {}", e),
                        Err(e) => format!("Task failed: {}", e),
                    }
                })
                .collect();

            Ok(responses)
        })
    }

    /// Zero-copy chat with pre-allocated message structures
    #[pyo3(signature = (messages, max_tokens=None, temperature=None))]
    fn chat_optimized<'a>(&self, messages: Vec<(String, String)>, max_tokens: Option<u32>, temperature: Option<f32>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider_arc = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Pre-allocate message vector to avoid reallocations
            let mut llm_messages = Vec::with_capacity(messages.len());
            
            for (role, content) in messages {
                let msg = match role.as_str() {
                    "system" => LlmMessage::system(content),
                    "assistant" => LlmMessage::assistant(content),
                    _ => LlmMessage::user(content),
                };
                llm_messages.push(msg);
            }

            let mut request = LlmRequest::with_messages(llm_messages);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let provider = provider_arc.read().await;
            let response = provider.complete(request).await
                .map_err(|e| OptimizedError::from_graphbit_error(&e).to_py_err())?;

            Ok(response.content)
        })
    }

    /// Stream response to avoid large memory allocation
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_stream<'a>(&self, prompt: String, max_tokens: Option<u32>, temperature: Option<f32>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider_arc = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut request = LlmRequest::new(prompt);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let provider = provider_arc.read().await;
            
            // Try streaming first, fallback to regular completion
            match provider.stream(request.clone()).await {
                Ok(mut stream) => {
                    let mut content = String::new();
                                         while let Some(chunk_result) = StreamExt::next(&mut stream).await {
                        match chunk_result {
                            Ok(chunk) => content.push_str(&chunk.content),
                            Err(_) => break,
                        }
                    }
                    Ok(content)
                },
                Err(_) => {
                                         // Fallback to regular completion
                     let response = provider.complete(request).await
                         .map_err(|e| OptimizedError::from_graphbit_error(&e).to_py_err())?;
                     Ok(response.content)
                }
            }
        })
    }

    /// Get client statistics
    fn get_stats<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);
        
        dict.set_item("provider", self.config.provider())?;
        dict.set_item("model", self.config.model())?;
        
        Ok(dict)
    }

    /// Warm up connections to reduce first-call latency
    fn warmup<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider_arc = Arc::clone(&self.provider);
        
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let warmup_request = LlmRequest::new("test".to_string()).with_max_tokens(1);
            let provider = provider_arc.read().await;
            
            // Make a minimal request to establish connection
            let _ = provider.complete(warmup_request).await;
            
            Ok(true)
        })
    }
}

// ===== SIMPLIFIED WORKFLOW SYSTEM =====

/// Simplified workflow node
#[pyclass]
#[derive(Clone)]
pub struct Node {
    inner: WorkflowNode,
}

#[pymethods]
impl Node {
    #[staticmethod]
    fn agent(name: String, prompt: String, agent_id: Option<String>) -> PyResult<Self> {
        let id = agent_id.unwrap_or_else(|| format!("agent_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let node = WorkflowNode::new(
            name.clone(),
            format!("Agent node: {}", name),
            NodeType::Agent {
                agent_id: AgentId::from_string(&id).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
                })?,
                prompt_template: prompt,
            },
        );
        Ok(Self { inner: node })
    }

    #[staticmethod]
    fn transform(name: String, transformation: String) -> Self {
        let node = WorkflowNode::new(
            name.clone(),
            format!("Transform node: {}", name),
            NodeType::Transform { transformation },
        );
        Self { inner: node }
    }

    #[staticmethod]
    fn condition(name: String, expression: String) -> Self {
        let node = WorkflowNode::new(
            name.clone(),
            format!("Condition node: {}", name),
            NodeType::Condition { expression },
        );
        Self { inner: node }
    }

    fn id(&self) -> String {
        self.inner.id.to_string()
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }
}

/// Simplified workflow
#[pyclass]
pub struct Workflow {
    inner: CoreWorkflow,
}

#[pymethods]
impl Workflow {
    #[new]
    fn new(name: String) -> Self {
        Self {
            inner: CoreWorkflow::new(name, ""),
        }
    }

    fn add_node(&mut self, node: Node) -> PyResult<String> {
        let node_id = self.inner.add_node(node.inner).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
        })?;
        Ok(node_id.to_string())
    }

    fn connect(&mut self, from_id: String, to_id: String) -> PyResult<()> {
        let from_node_id = NodeId::from_string(&from_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        let to_node_id = NodeId::from_string(&to_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;

        self.inner.connect_nodes(from_node_id, to_node_id, WorkflowEdge::data_flow()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
        })?;
        Ok(())
    }

    fn validate(&self) -> PyResult<()> {
        self.inner.validate().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })?;
        Ok(())
    }
}

/// Simplified workflow result
#[pyclass]
pub struct WorkflowResult {
    inner: WorkflowContext,
}

#[pymethods]
impl WorkflowResult {
    fn is_success(&self) -> bool {
        matches!(self.inner.state, WorkflowState::Completed)
    }

    fn get_variable(&self, key: &str) -> Option<String> {
        self.inner.get_variable(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn get_all_variables(&self) -> HashMap<String, String> {
        self.inner.variables
            .iter()
            .filter_map(|(k, v)| {
                v.as_str().map(|s| (k.clone(), s.to_string()))
            })
            .collect()
    }
}

// ===== SIMPLIFIED EXECUTOR =====

/// Simplified executor for workflows
#[pyclass]
pub struct Executor {
    config: LlmConfig,
    executor: WorkflowExecutor,
    max_timeout: Option<u64>,
    max_retries: Option<u32>,
}

#[pymethods]
impl Executor {
    #[new]
    fn new(config: LlmConfig) -> Self {
        let executor = WorkflowExecutor::new();
        Self {
            config,
            executor,
            max_timeout: None,
            max_retries: None,
        }
    }

    fn timeout(&mut self, timeout_seconds: u64) {
        self.max_timeout = Some(timeout_seconds * 1000); // Convert to ms
    }

    fn retries(&mut self, max_retries: u32) {
        self.max_retries = Some(max_retries);
    }

    fn run(&self, workflow: &Workflow) -> PyResult<WorkflowResult> {
        pyo3_async_runtimes::tokio::get_runtime().block_on(async {
            let result = self.executor.execute(workflow.inner.clone()).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            Ok(WorkflowResult { inner: result })
        })
    }

    fn run_async<'a>(&self, workflow: &Workflow, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let workflow_clone = workflow.inner.clone();
        // Note: We can't move self.executor due to borrowing rules, but we can create
        // a new one for now. In a real optimization, we'd need Arc<Mutex<WorkflowExecutor>>
        
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let executor = WorkflowExecutor::new();
            let result = executor.execute(workflow_clone).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            Ok(WorkflowResult { inner: result })
        })
    }
}

// ===== SIMPLIFIED EMBEDDINGS =====

/// Simplified embedding configuration
#[pyclass]
#[derive(Clone)]
pub struct EmbeddingConfig {
    inner: CoreEmbeddingConfig,
}

#[pymethods]
impl EmbeddingConfig {
    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn openai(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "OpenAI")?;
        
        Ok(Self {
            inner: CoreEmbeddingConfig {
                provider: EmbeddingProvider::OpenAI,
                api_key,
                model: model.unwrap_or_else(|| "text-embedding-3-small".to_string()),
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
            },
        })
    }

        #[staticmethod]
    fn huggingface(api_key: String, model: String) -> PyResult<Self> {
        validate_api_key(&api_key, "HuggingFace")?;
        
        Ok(Self {
            inner: CoreEmbeddingConfig {
                provider: EmbeddingProvider::HuggingFace,
                api_key,
                model,
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
            },
        })
    }
    }

/// Simplified embedding client
#[pyclass]
pub struct EmbeddingClient {
    service: Arc<EmbeddingService>,
}

#[pymethods]
impl EmbeddingClient {
    #[new]
    fn new(config: EmbeddingConfig) -> PyResult<Self> {
        let service = Arc::new(EmbeddingService::new(config.inner).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
        })?);
        Ok(Self { service })
    }

    fn embed(&self, text: String) -> PyResult<Vec<f32>> {
        pyo3_async_runtimes::tokio::get_runtime().block_on(async {
            let response = self.service.embed_text(&text).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            
            Ok(response)
        })
    }

    fn embed_many(&self, texts: Vec<String>) -> PyResult<Vec<Vec<f32>>> {
        pyo3_async_runtimes::tokio::get_runtime().block_on(async {
            let response = self.service.embed_texts(&texts).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            
            Ok(response)
        })
    }

    #[staticmethod]
    fn similarity(a: Vec<f32>, b: Vec<f32>) -> PyResult<f32> {
        graphbit_core::embeddings::EmbeddingService::cosine_similarity(&a, &b)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

// ===== MODULE INITIALIZATION =====

#[pyfunction]
fn init() -> PyResult<()> {
    graphbit_core::init().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
    })?;
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
    
    // Core classes
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
