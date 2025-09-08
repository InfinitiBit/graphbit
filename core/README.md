# GraphBit Core

The Rust core library for GraphBit - a declarative agentic workflow automation framework.

## Overview

GraphBit Core is the foundational Rust library that provides:
- **Workflow Engine**: Graph-based workflow execution with dependency management
- **Agent System**: LLM-backed intelligent agents with configurable capabilities
- **LLM Providers**: Multi-provider support (OpenAI, Anthropic, Ollama)
- **Validation System**: JSON schema and custom validation framework
- **Type System**: Strong typing with UUID-based identifiers
- **Error Handling**: Comprehensive error types with retry logic

## Architecture

```
graphbit-core/
├── src/
│   ├── lib.rs              # Public API and exports
│   ├── types.rs            # Core type definitions
│   ├── errors.rs           # Error types and handling
│   ├── agents.rs           # Agent abstraction layer
│   ├── graph.rs            # Graph data structures
│   ├── workflow.rs         # Workflow execution engine
│   ├── validation.rs       # Validation system
│   └── llm/               # LLM provider implementations
│       ├── mod.rs
│       ├── openai.rs
│       ├── anthropic.rs
│       ├── ollama.rs
│       └── providers.rs
├── tests/                  # Integration tests
├── benches/               # Performance benchmarks
└── examples/              # Usage examples
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
graphbit-core = "0.4.0"

# Optional features
graphbit-core = { version = "0.4.0", features = ["python-bindings"] }
```

### Features

- `python-bindings`: Enable PyO3 Python bindings
- `serde`: Enable serialization support (default)
- `tokio`: Enable async runtime support (default)

## Quick Start

```rust
use graphbit_core::*;

#[tokio::main]
async fn main() -> Result<(), GraphBitError> {
    // Create LLM configuration
    let llm_config = LlmConfig::openai("gpt-4", &api_key);
    
    // Build a simple workflow
    let mut workflow = WorkflowBuilder::new("Hello World")
        .description("A simple greeting workflow")
        .add_agent_node("greeter", "Say hello to {name}")
        .build()?;
    
    // Execute the workflow
    let executor = WorkflowExecutor::new(llm_config);
    let result = executor.execute(workflow, json!({"name": "Alice"})).await?;
    
    println!("Result: {}", result);
    Ok(())
}
```

## Core Types

### WorkflowGraph

The core graph data structure representing workflow topology.

```rust
use graphbit_core::graph::*;

let mut graph = WorkflowGraph::new();

// Add nodes
let node1 = graph.add_node(WorkflowNode::agent("analyzer", "Analyze data"));
let node2 = graph.add_node(WorkflowNode::transform("formatter", "Format output"));

// Add edges
graph.add_edge(node1, node2, EdgeType::DataFlow)?;

// Validate graph properties
assert!(!graph.has_cycles());
assert_eq!(graph.node_count(), 2);
```

### Agent

Intelligent agents backed by LLM providers.

```rust
use graphbit_core::agents::*;

// Create agent with configuration
let config = AgentConfig {
    max_tokens: 1000,
    temperature: 0.7,
    system_prompt: "You are a helpful assistant".to_string(),
    capabilities: vec!["text_processing".to_string()],
};

let agent = Agent::new(
    "assistant".to_string(),
    "AI Assistant".to_string(),
    Arc::new(openai_provider),
    config,
);

// Execute agent task
let response = agent.execute("Analyze this text: Hello world").await?;
println!("Response: {}", response.content);
```

### LLM Providers

Multi-provider LLM integration with consistent interface.

```rust
use graphbit_core::llm::*;

// OpenAI provider
let openai = OpenAiProvider::new("gpt-4", &api_key)?;

// Anthropic provider
let anthropic = AnthropicProvider::new("claude-3-sonnet", &api_key)?;

// Ollama provider
let ollama = OllamaProvider::new("llama3.1", "http://localhost:11434")?;

// Use any provider with the same interface
async fn generate_text(provider: Arc<dyn LlmProvider>) -> Result<String, LlmError> {
    let response = provider.generate("Hello, how are you?").await?;
    Ok(response.content)
}
```

### Workflow Execution

Concurrent workflow execution with dependency management.

```rust
use graphbit_core::workflow::*;

// Create executor with configuration
let executor = WorkflowExecutor::new(config)
    .with_max_concurrency(5)
    .with_timeout(Duration::from_secs(300));

// Execute workflow
let result = executor.execute(workflow, input_data).await?;

match result.status {
    ExecutionStatus::Completed => println!("Success: {:?}", result.outputs),
    ExecutionStatus::Failed => println!("Failed: {:?}", result.error),
    ExecutionStatus::PartialSuccess => println!("Partial: {:?}", result.outputs),
}
```

## Advanced Usage

### Custom Validators

Implement custom validation logic:

```rust
use graphbit_core::validation::*;

pub struct EmailValidator;

impl CustomValidator for EmailValidator {
    fn validate(
        &self, 
        data: &str, 
        _config: &HashMap<String, serde_json::Value>
    ) -> GraphBitResult<ValidationResult> {
        let is_valid = data.contains('@') && data.contains('.');
        
        if is_valid {
            Ok(ValidationResult::success())
        } else {
            Ok(ValidationResult::failure(vec![
                ValidationError::new(
                    "email",
                    "Invalid email format",
                    "INVALID_EMAIL"
                )
            ]))
        }
    }
    
    fn name(&self) -> &str { "email" }
    fn description(&self) -> &str { "Validates email addresses" }
}

// Register validator
let mut registry = ValidatorRegistry::new();
registry.register(Box::new(EmailValidator));
```

### Custom LLM Providers

Implement support for new LLM providers:

```rust
use graphbit_core::llm::*;
use async_trait::async_trait;

pub struct CustomProvider {
    model: String,
    base_url: String,
    client: reqwest::Client,
}

#[async_trait]
impl LlmProvider for CustomProvider {
    async fn generate(&self, prompt: &str) -> Result<LlmResponse, LlmError> {
        let request_body = json!({
            "model": self.model,
            "prompt": prompt,
            "max_tokens": 1000
        });
        
        let response = self.client
            .post(&format!("{}/generate", self.base_url))
            .json(&request_body)
            .send()
            .await
            .map_err(LlmError::NetworkError)?;
        
        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(LlmError::ParseError)?;
        
        Ok(LlmResponse {
            content: response_json["content"].as_str().unwrap().to_string(),
            usage: TokenUsage {
                prompt_tokens: response_json["usage"]["prompt_tokens"].as_u64().unwrap() as usize,
                completion_tokens: response_json["usage"]["completion_tokens"].as_u64().unwrap() as usize,
            },
        })
    }
    
    fn provider_name(&self) -> &str { "custom" }
    fn model_name(&self) -> &str { &self.model }
}
```

### Error Handling

Comprehensive error handling with context:

```rust
use graphbit_core::errors::*;

// Handle different error types
match executor.execute(workflow, input).await {
    Ok(result) => println!("Success: {:?}", result),
    Err(GraphBitError::Validation(e)) => {
        eprintln!("Validation error: {}", e);
        for error in &e.errors {
            eprintln!("  - {}: {}", error.field, error.message);
        }
    },
    Err(GraphBitError::Llm(e)) => {
        eprintln!("LLM error: {}", e);
        match e {
            LlmError::Authentication(_) => eprintln!("Check your API key"),
            LlmError::RateLimit(_) => eprintln!("Rate limited, try again later"),
            LlmError::NetworkError(_) => eprintln!("Network issue, check connection"),
            _ => eprintln!("Other LLM error occurred"),
        }
    },
    Err(GraphBitError::Workflow(e)) => {
        eprintln!("Workflow error: {}", e);
        if let Some(node_id) = &e.failed_node {
            eprintln!("Failed at node: {}", node_id);
        }
    },
    Err(e) => eprintln!("Unexpected error: {}", e),
}
```

### Async Streams

Process workflows as streams for real-time updates:

```rust
use graphbit_core::workflow::*;
use futures::StreamExt;

let mut stream = executor.execute_stream(workflow, input_data).await?;

while let Some(event) = stream.next().await {
    match event? {
        ExecutionEvent::NodeStarted { node_id, .. } => {
            println!("Started: {}", node_id);
        },
        ExecutionEvent::NodeCompleted { node_id, output } => {
            println!("Completed: {} -> {:?}", node_id, output);
        },
        ExecutionEvent::NodeFailed { node_id, error } => {
            eprintln!("Failed: {} - {}", node_id, error);
        },
        ExecutionEvent::WorkflowCompleted { result } => {
            println!("Workflow finished: {:?}", result);
            break;
        },
    }
}
```

## Performance Optimization

### Concurrent Execution

```rust
use graphbit_core::workflow::*;

// Configure concurrency
let executor = WorkflowExecutor::new(config)
    .with_max_concurrency(10)  // Max parallel nodes
    .with_batch_size(5)        // Batch processing size
    .with_timeout(Duration::from_secs(300));

// Execute with performance monitoring
let start = std::time::Instant::now();
let result = executor.execute(workflow, input).await?;
let duration = start.elapsed();

println!("Execution time: {:?}", duration);
println!("Nodes processed: {}", result.node_count);
println!("Average time per node: {:?}", duration / result.node_count as u32);
```

### Memory Management

```rust
use graphbit_core::workflow::*;

// Configure memory limits
let executor = WorkflowExecutor::new(config)
    .with_memory_limit(1024 * 1024 * 512)  // 512MB limit
    .with_cache_size(100)                   // Cache last 100 results
    .with_streaming_threshold(10);          // Stream for >10 nodes

// Monitor memory usage
let memory_usage = executor.memory_usage();
println!("Memory used: {} bytes", memory_usage.current);
println!("Peak memory: {} bytes", memory_usage.peak);
```

### Caching

```rust
use graphbit_core::cache::*;

// Configure result caching
let cache = LruCache::new(1000);  // Cache 1000 results
let executor = WorkflowExecutor::new(config)
    .with_cache(Arc::new(Mutex::new(cache)));

// Cache keys are automatically generated from:
// - Node configuration hash
// - Input data hash
// - LLM provider settings

// Manual cache management
let cache_key = CacheKey::new("node_id", &input_hash);
if let Some(cached_result) = executor.cache().get(&cache_key) {
    return Ok(cached_result.clone());
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use graphbit_core::test_utils::*;

    #[test]
    fn test_graph_construction() {
        let mut graph = WorkflowGraph::new();
        
        let node1 = graph.add_node(WorkflowNode::agent("agent1", "Test"));
        let node2 = graph.add_node(WorkflowNode::agent("agent2", "Test"));
        
        assert_eq!(graph.node_count(), 2);
        
        graph.add_edge(node1, node2, EdgeType::DataFlow).unwrap();
        assert_eq!(graph.edge_count(), 1);
        assert!(!graph.has_cycles());
    }

    #[tokio::test]
    async fn test_agent_execution() {
        let mock_provider = MockLlmProvider::new(vec![
            "Mock response 1".to_string(),
        ]);
        
        let agent = Agent::new(
            "test".to_string(),
            "Test Agent".to_string(),
            Arc::new(mock_provider),
            AgentConfig::default(),
        );

        let response = agent.execute("Test prompt").await.unwrap();
        assert_eq!(response.content, "Mock response 1");
    }
}
```

### Integration Tests

```rust
// tests/integration_tests.rs
use graphbit_core::*;

#[tokio::test]
async fn test_end_to_end_workflow() {
    // Setup
    let config = test_config();
    let workflow = test_workflow();
    let executor = WorkflowExecutor::new(config);
    
    // Execute
    let result = executor.execute(workflow, test_input()).await;
    
    // Verify
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.status, ExecutionStatus::Completed);
    assert!(!result.outputs.is_empty());
}

fn test_config() -> GraphBitConfig {
    GraphBitConfig {
        llm: LlmConfig::mock(),
        agents: HashMap::new(),
        validation: ValidationConfig::default(),
    }
}
```

### Benchmarks

```rust
// benches/workflow_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use graphbit_core::*;

fn bench_workflow_execution(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("workflow_execution");
    
    for size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("parallel_nodes", size),
            size,
            |b, &size| {
                let workflow = create_parallel_workflow(size);
                let executor = WorkflowExecutor::new(mock_config());
                
                b.to_async(&rt).iter(|| async {
                    executor.execute(workflow.clone()).await
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_workflow_execution);
criterion_main!(benches);
```

## API Documentation

### Core Traits

#### LlmProvider

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate text response from prompt
    async fn generate(&self, prompt: &str) -> Result<LlmResponse, LlmError>;
    
    /// Get provider name
    fn provider_name(&self) -> &str;
    
    /// Get model name
    fn model_name(&self) -> &str;
    
    /// Health check (optional)
    async fn health_check(&self) -> Result<(), LlmError> {
        Ok(())
    }
    
    /// Get provider capabilities (optional)
    fn capabilities(&self) -> Vec<String> {
        vec![]
    }
}
```

#### CustomValidator

```rust
pub trait CustomValidator: Send + Sync {
    /// Validate data with optional configuration
    fn validate(
        &self,
        data: &str,
        config: &HashMap<String, serde_json::Value>,
    ) -> GraphBitResult<ValidationResult>;
    
    /// Validator name
    fn name(&self) -> &str;
    
    /// Validator description
    fn description(&self) -> &str;
    
    /// JSON schema for validator configuration (optional)
    fn config_schema(&self) -> Option<serde_json::Value> {
        None
    }
}
```

### Key Structs

#### WorkflowNode

```rust
pub struct WorkflowNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub name: String,
    pub description: String,
    pub config: NodeConfig,
}

impl WorkflowNode {
    pub fn agent(id: &str, name: &str) -> Self { /* */ }
    pub fn condition(id: &str, name: &str, expression: &str) -> Self { /* */ }
    pub fn transform(id: &str, name: &str, transformation: &str) -> Self { /* */ }
    pub fn delay(id: &str, name: &str, duration: Duration) -> Self { /* */ }
}
```

#### ExecutionResult

```rust
pub struct ExecutionResult {
    pub status: ExecutionStatus,
    pub outputs: HashMap<NodeId, serde_json::Value>,
    pub execution_time: Duration,
    pub node_count: usize,
    pub error: Option<GraphBitError>,
}

pub enum ExecutionStatus {
    Completed,
    Failed,
    PartialSuccess,
    Cancelled,
}
```

## Development Setup

### Prerequisites

```bash
# Install Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required tools
cargo install cargo-watch
cargo install cargo-tarpaulin  # For coverage
cargo install cargo-audit      # For security audits
```

### Building

```bash
# Clone repository
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/core

# Build library
cargo build

# Build with all features
cargo build --all-features

# Run tests
cargo test

# Run with output
cargo test -- --nocapture

# Test specific module
cargo test agents::
```

### Development Workflow

```bash
# Watch for changes and rebuild
cargo watch -x build

# Watch and run tests
cargo watch -x test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open

# Security audit
cargo audit
```

### Code Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Open coverage report
open coverage/tarpaulin-report.html
```

## Contributing

### Code Style

We follow standard Rust conventions:

```rust
// Use descriptive names
struct WorkflowExecutor {
    config: GraphBitConfig,
    llm_provider: Arc<dyn LlmProvider>,
}

// Document public APIs
/// Executes a workflow with the given input data.
/// 
/// # Arguments
/// * `workflow` - The workflow to execute
/// * `input` - Input data for the workflow
/// 
/// # Returns
/// Result containing execution results or error
pub async fn execute(
    &self,
    workflow: Workflow,
    input: serde_json::Value,
) -> Result<ExecutionResult, GraphBitError> {
    // Implementation
}

// Use Result types for error handling
type GraphBitResult<T> = Result<T, GraphBitError>;

// Prefer owned types in public APIs
pub fn add_node(&mut self, node: WorkflowNode) -> NodeId {
    // Implementation
}
```

### Error Handling

```rust
// Define specific error types
#[derive(Debug, thiserror::Error)]
pub enum GraphBitError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("LLM error: {0}")]
    Llm(#[from] LlmError),
    
    #[error("Workflow error: {0}")]
    Workflow(#[from] WorkflowError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Provide context with errors
impl WorkflowError {
    pub fn new(message: String, node_id: Option<NodeId>) -> Self {
        Self {
            message,
            failed_node: node_id,
            context: HashMap::new(),
        }
    }
    
    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        self.context.insert(key.to_string(), value);
        self
    }
}
```

### Testing Guidelines

1. **Unit tests** for individual functions
2. **Integration tests** for component interactions
3. **Property tests** for complex logic
4. **Benchmarks** for performance-critical code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert!(result.is_ok());
    }

    proptest! {
        #[test]
        fn test_property_holds(
            input in any::<String>(),
        ) {
            let result = function_under_test(input);
            prop_assert!(invariant_holds(&result));
        }
    }
}
```

## Troubleshooting

### Common Issues

#### Build Errors

```bash
# Update Rust
rustup update

# Clean build
cargo clean && cargo build

# Check for conflicting dependencies
cargo tree --duplicates
```

#### Test Failures

```bash
# Run single test
cargo test test_name --exact

# Show test output
cargo test -- --show-output

# Run ignored tests
cargo test -- --ignored
```

#### Performance Issues

```bash
# Profile with perf
cargo build --release
perf record target/release/your_binary
perf report

# Memory profiling with valgrind
cargo build
valgrind --tool=memcheck target/debug/your_binary
```

---

For more detailed documentation, see the [API docs](https://docs.rs/graphbit-core) or join our [Discord community](https://discord.gg/graphbit). 
