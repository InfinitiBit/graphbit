# Architecture Overview

GraphBit is a high-performance AI agent workflow automation framework built with a three-tier architecture that combines the performance of Rust with the accessibility of Python.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Python API Layer                        │
├─────────────────────────────────────────────────────────────┤
│                    PyO3 Bindings                          │
├─────────────────────────────────────────────────────────────┤
│                    Rust Core Engine                        │
├─────────────────────────────────────────────────────────────┤
│     LLM Providers     │     Storage      │    Telemetry   │
│  OpenAI | Anthropic   │  Memory | Disk   │  Tracing | Logs │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Python API Layer

The Python API provides a user-friendly interface for creating and managing workflows:

```python
# High-level Python interface
import graphbit

graphbit.init()
builder = graphbit.PyWorkflowBuilder("My Workflow")
# ... build workflow
executor = graphbit.PyWorkflowExecutor(config)
result = executor.execute(workflow)
```

**Key Classes:**
- `PyWorkflowBuilder` - Fluent workflow construction
- `PyWorkflowExecutor` - Workflow execution engine
- `PyWorkflowNode` - Node creation and management
- `PyLlmConfig` - LLM provider configuration

### 2. PyO3 Bindings Layer

PyO3 provides seamless interoperability between Python and Rust with production-grade features:

```rust
// Python class exposed via PyO3
#[pyclass]
pub struct LlmClient {
    provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
    circuit_breaker: Arc<CircuitBreaker>,
    config: ClientConfig,
    stats: Arc<RwLock<ClientStats>>,
}

#[pymethods]
impl LlmClient {
    #[new]
    fn new(config: LlmConfig, debug: Option<bool>) -> PyResult<Self> {
        // Production-grade initialization with error handling
    }
    
    fn complete(&self, prompt: String, max_tokens: Option<u32>) -> PyResult<String> {
        // High-performance completion with resilience patterns
    }
}
```

**Key Features:**
- **Type Safety**: Comprehensive input validation and type checking
- **Memory Management**: Proper resource cleanup across language boundaries
- **Error Handling**: Structured error types mapped to Python exceptions
- **Async Support**: Full async/await compatibility with Tokio runtime
- **Performance**: Zero-copy operations and optimized data structures
- **Observability**: Built-in metrics, tracing, and health monitoring

**Module Structure:**
- `lib.rs`: Global initialization and system management
- `runtime.rs`: Optimized Tokio runtime management
- `errors.rs`: Comprehensive error handling and conversion
- `llm/`: LLM provider integration with resilience patterns
- `embeddings/`: Embedding provider support
- `workflow/`: Workflow execution engine
- `validation.rs`: Input validation utilities

### 3. Rust Core Engine

The Rust core provides high-performance execution and reliability:

```rust
// Core workflow execution
pub struct WorkflowExecutor {
    llm_client: Arc<dyn LlmClient>,
    circuit_breaker: CircuitBreaker,
    retry_policy: RetryPolicy,
}

impl WorkflowExecutor {
    pub async fn execute(&self, workflow: &Workflow) -> Result<ExecutionResult> {
        // High-performance execution logic
    }
}
```

**Key Modules:**
- `workflow` - Workflow definition and validation
- `executor` - Execution engine and orchestration
- `agents` - Agent management and processing  
- `llm` - LLM provider integrations
- `reliability` - Circuit breakers, retries, rate limiting

## Data Flow Architecture

### Workflow Execution Flow

```
Input Data
    ↓
┌─────────────────┐
│  Workflow       │
│  Validation     │
└─────────────────┘
    ↓
┌─────────────────┐
│  Execution      │
│  Planning       │
└─────────────────┘
    ↓
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Node Execution │────│  LLM Provider   │────│  Response       │
│  (Parallel/Seq) │    │  Integration    │    │  Processing     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
    ↓
┌─────────────────┐
│  Result         │
│  Aggregation    │
└─────────────────┘
    ↓
Output Data
```

### Node Processing Pipeline

```
Node Input
    ↓
┌─────────────────┐
│  Input          │
│  Validation     │
└─────────────────┘
    ↓
┌─────────────────┐
│  Prompt         │
│  Template       │
│  Processing     │
└─────────────────┘
    ↓
┌─────────────────┐
│  LLM Request    │
│  (with Circuit  │
│   Breaker)      │
└─────────────────┘
    ↓
┌─────────────────┐
│  Response       │
│  Processing &   │
│  Validation     │
└─────────────────┘
    ↓
Node Output
```

## Component Details

### Workflow Management

#### Workflow Builder Pattern

```rust
pub struct WorkflowBuilder {
    name: String,
    description: Option<String>,
    nodes: HashMap<NodeId, Node>,
    edges: Vec<Edge>,
}

impl WorkflowBuilder {
    pub fn add_node(&mut self, node: Node) -> NodeId {
        // Add node and return unique ID
    }
    
    pub fn connect(&mut self, from: NodeId, to: NodeId, edge: Edge) {
        // Connect nodes with typed edges
    }
    
    pub fn build(self) -> Result<Workflow> {
        // Validate and build immutable workflow
    }
}
```

#### Workflow Validation

GraphBit performs comprehensive validation:

- **Structural Validation**: No cycles, connected graph
- **Type Validation**: Compatible data types between nodes
- **Dependency Validation**: All required inputs available
- **Resource Validation**: API keys, model availability

### Execution Engine

#### Execution Strategies

**Sequential Execution:**
```rust
async fn execute_sequential(&self, workflow: &Workflow) -> Result<ExecutionResult> {
    let execution_order = self.topological_sort(workflow)?;
    
    for node_id in execution_order {
        let result = self.execute_node(node_id).await?;
        self.context.set_result(node_id, result);
    }
    
    Ok(self.context.into_result())
}
```

**Parallel Execution:**
```rust
async fn execute_parallel(&self, workflow: &Workflow) -> Result<ExecutionResult> {
    let execution_batches = self.create_execution_batches(workflow)?;
    
    for batch in execution_batches {
        let batch_results = join_all(
            batch.into_iter().map(|node_id| self.execute_node(node_id))
        ).await;
        
        self.context.merge_results(batch_results)?;
    }
    
    Ok(self.context.into_result())
}
```

#### Context Management

```rust
pub struct ExecutionContext {
    variables: HashMap<String, Value>,
    node_results: HashMap<NodeId, NodeResult>,
    metadata: ExecutionMetadata,
}

impl ExecutionContext {
    pub fn get_variable(&self, key: &str) -> Option<&Value> {
        self.variables.get(key)
    }
    
    pub fn set_variable(&mut self, key: String, value: Value) {
        self.variables.insert(key, value);
    }
    
    pub fn substitute_template(&self, template: &str) -> String {
        // Template variable substitution
    }
}
```

### LLM Integration Layer

#### Provider Abstraction

```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse>;
    async fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse>;
    
    fn provider_name(&self) -> &str;
    fn model_info(&self) -> ModelInfo;
}
```

#### Provider Implementations

**OpenAI Client:**
```rust
pub struct OpenAiClient {
    client: Client,
    api_key: String,
    model: String,
}

#[async_trait]
impl LlmClient for OpenAiClient {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let openai_request = self.convert_request(request);
        let response = self.client.chat().create(openai_request).await?;
        Ok(self.convert_response(response))
    }
}
```

**Anthropic Client:**
```rust
pub struct AnthropicClient {
    client: Client, 
    api_key: String,
    model: String,
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let claude_request = self.convert_request(request);
        let response = self.client.messages().create(claude_request).await?;
        Ok(self.convert_response(response))
    }
}
```

### Reliability Layer

#### Circuit Breaker Pattern

```rust
pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitState>>,
    failure_threshold: u32,
    timeout_duration: Duration,
    half_open_max_calls: u32,
}

impl CircuitBreaker {
    pub async fn call<F, T>(&self, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        match self.state() {
            CircuitState::Closed => self.execute_operation(operation).await,
            CircuitState::Open => Err(Error::CircuitBreakerOpen),
            CircuitState::HalfOpen => self.try_half_open_operation(operation).await,
        }
    }
}
```

#### Retry Policy

```rust
pub struct RetryPolicy {
    max_attempts: u32,
    base_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
    jitter: f64,
}

impl RetryPolicy {
    pub async fn execute<F, T>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut() -> Pin<Box<dyn Future<Output = Result<T>>>>,
    {
        let mut attempt = 0;
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt >= self.max_attempts => return Err(e),
                Err(_) => {
                    attempt += 1;
                    let delay = self.calculate_delay(attempt);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}
```

## Memory Management

### Python-Rust Boundary

```rust
// Efficient data transfer
#[pyclass]
pub struct PyExecutionResult {
    #[pyo3(get)]
    pub status: String,
    
    // Internal Rust data (not exposed to Python)
    result: Arc<ExecutionResult>,
}

impl PyExecutionResult {
    // Zero-copy access to internal data when possible
    pub fn get_variable(&self, key: &str) -> Option<String> {
        self.result.variables.get(key).map(|v| v.to_string())
    }
}
```

### Resource Management

**Connection Pooling:**
```rust
pub struct ConnectionPool {
    pools: HashMap<String, Pool<Connection>>,
    max_connections: usize,
}
```

**Memory Efficient Node Storage:**
```rust
// Nodes stored with minimal overhead
pub struct NodeStorage {
    nodes: SlotMap<NodeId, Node>,
    node_index: HashMap<String, NodeId>,
}
```

## Performance Characteristics

### Benchmarks

| Operation | Performance | Notes |
|-----------|-------------|-------|
| Workflow Build | ~1ms | For typical 10-node workflow |
| Node Execution | ~100-500ms | Depends on LLM provider |
| Parallel Processing | 2-5x speedup | For independent nodes |
| Memory Usage | <50MB base | Scales with workflow complexity |

### Optimization Strategies

#### 1. Async Processing
```rust
// All I/O operations are async
pub async fn execute_workflow(&self, workflow: &Workflow) -> Result<ExecutionResult> {
    // Concurrent execution of independent nodes
    let futures = ready_nodes.into_iter()
        .map(|node| self.execute_node(node))
        .collect::<Vec<_>>();
    
    let results = join_all(futures).await;
    // Process results...
}
```

#### 2. Connection Reuse
```rust
// HTTP client reuse
pub struct LlmClientManager {
    clients: HashMap<String, Arc<dyn LlmClient>>,
}
```

#### 3. Memory Pooling
```rust
// Reuse allocations where possible
pub struct NodeExecutor {
    buffer_pool: ObjectPool<Vec<u8>>,
}
```

## Security Architecture

### API Key Management

```rust
pub struct SecureConfig {
    // API keys stored securely
    encrypted_keys: HashMap<String, Vec<u8>>,
    key_cipher: ChaCha20Poly1305,
}
```

### Input Validation

```rust
pub fn validate_input(input: &str) -> Result<()> {
    // Comprehensive input validation
    if input.len() > MAX_INPUT_SIZE {
        return Err(Error::InputTooLarge);
    }
    
    // Check for potentially dangerous content
    validate_content_safety(input)?;
    
    Ok(())
}
```

### Safe Template Processing

```rust
pub struct SafeTemplateEngine {
    // Prevent template injection attacks
    allowed_functions: HashSet<String>,
}
```

## Extensibility

### Plugin Architecture

```rust
pub trait NodeProcessor: Send + Sync {
    fn process(&self, input: &NodeInput) -> Result<NodeOutput>;
    fn node_type(&self) -> &str;
}

// Register custom processors
pub struct ProcessorRegistry {
    processors: HashMap<String, Box<dyn NodeProcessor>>,
}
```

### Custom LLM Providers

```rust
// Implement custom provider
pub struct CustomLlmClient {
    // Custom implementation
}

#[async_trait]
impl LlmClient for CustomLlmClient {
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        // Custom LLM integration
    }
}
```

## Monitoring and Observability

### Structured Logging

```rust
use tracing::{info, warn, error, instrument};

#[instrument]
pub async fn execute_node(&self, node: &Node) -> Result<NodeResult> {
    info!(node_id = %node.id(), "Starting node execution");
    
    let start = Instant::now();
    let result = self.process_node(node).await;
    let duration = start.elapsed();
    
    match &result {
        Ok(_) => info!(
            node_id = %node.id(),
            duration_ms = duration.as_millis(),
            "Node execution completed successfully"
        ),
        Err(e) => error!(
            node_id = %node.id(),
            error = %e,
            "Node execution failed"
        ),
    }
    
    result
}
```

### Metrics Collection

```rust
pub struct MetricsCollector {
    execution_times: Histogram,
    success_counter: Counter,
    error_counter: Counter,
}
```

This architecture provides a solid foundation for building scalable, reliable AI workflows while maintaining high performance and developer productivity. 
