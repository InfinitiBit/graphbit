# GraphBit Core

The Rust core library for GraphBit - a declarative agentic workflow automation framework.

## Overview

GraphBit Core is the foundational Rust library that provides:

- **Workflow Engine**: Graph-based workflow execution with dependency management and parallel processing
- **Agent System**: LLM-backed intelligent agents with tool calling support
- **LLM Providers**: Multi-provider support (OpenAI, Anthropic, Ollama, Azure, Mistral, and more)
- **Document Processing**: Load and extract content from PDF, TXT, DOCX, JSON, CSV, XML, HTML
- **Text Splitting**: Multiple strategies for chunking large documents
- **Embeddings**: Support for OpenAI and HuggingFace embedding providers
- **Validation System**: Type validation with JSON schema support
- **Type System**: Strong typing with UUID-based identifiers
- **Error Handling**: Comprehensive error types with retry logic and circuit breakers

## Documentation

- **Contributor guide to the core crate**: [`rust-core.md`](rust-core.md)
- **System architecture (3-tier)**: [`docs/development/architecture.md`](../docs/development/architecture.md)
- **Python bindings architecture**: [`docs/development/python-bindings.md`](../docs/development/python-bindings.md)

## Architecture

```
core/
├── src/
│   ├── lib.rs              # Public API and exports
│   ├── types.rs            # Core type definitions
│   ├── errors.rs           # Error types and handling
│   ├── agents.rs           # Agent abstraction layer
│   ├── graph.rs            # Graph data structures
│   ├── workflow.rs         # Workflow execution engine
│   ├── validation.rs       # Type validation system
│   ├── document_loader.rs  # Document loading and extraction
│   ├── text_splitter.rs    # Text chunking strategies
│   ├── embeddings.rs       # Embedding provider support
│   └── llm/               # LLM provider implementations
│       ├── mod.rs
│       ├── openai.rs
│       ├── anthropic.rs
│       ├── ollama.rs
│       ├── azurellm.rs
│       ├── mistralai.rs
│       ├── deepseek.rs
│       ├── fireworks.rs
│       ├── replicate.rs
│       ├── togetherai.rs
│       ├── xai.rs
│       └── ... (more providers)
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
graphbit-core = "0.6.4"

# Optional: enable Python bridge support (PyO3 integration points)
# graphbit-core = { version = "0.6.2", features = ["python"] }
```

The library uses async/await with Tokio runtime and includes serialization support via Serde.

## Quick Start

```rust
use graphbit_core::*;

#[tokio::main]
async fn main() -> GraphBitResult<()> {
    // Create LLM configuration
    let llm_config = LlmConfig::OpenAI {
        api_key: std::env::var("OPENAI_API_KEY")?,
        model: "gpt-4".to_string(),
        base_url: None,
        organization: None,
    };

    // Build a workflow
    let mut workflow = Workflow::new("Hello World", "A simple greeting workflow");

    // Add an agent node
    //
    // Deterministic IDs are useful for stable workflows.
    // `from_string` returns a `uuid::Error`, so we map it into `GraphBitError` for `GraphBitResult`.
    let agent_id =
        AgentId::from_string("greeter").map_err(|e| GraphBitError::config(e.to_string()))?;
    let node = WorkflowNode::new(
        "greeter",
        "Greet the user",
        NodeType::Agent {
            agent_id: agent_id.clone(),
            prompt_template: "Say hello to the user".to_string(),
        },
    );
    workflow.add_node(node)?;

    // Create executor and register agent
    let executor = WorkflowExecutor::new()
        .with_default_llm_config(llm_config);

    // Execute the workflow
    let context = executor.execute(workflow).await?;

    println!("Workflow completed: {:?}", context.state);
    Ok(())
}
```

## Core Types

### WorkflowGraph

The core graph data structure representing workflow topology.

```rust
use graphbit_core::*;

let mut graph = WorkflowGraph::new();

// Create nodes
let analyzer_agent_id =
    AgentId::from_string("analyzer").map_err(|e| GraphBitError::config(e.to_string()))?;
let node1 = WorkflowNode::new(
    "analyzer",
    "Analyze data",
    NodeType::Agent {
        agent_id: analyzer_agent_id,
        prompt_template: "Analyze the input data".to_string(),
    },
);

let formatter_agent_id =
    AgentId::from_string("formatter").map_err(|e| GraphBitError::config(e.to_string()))?;
let node2 = WorkflowNode::new(
    "formatter",
    "Format output",
    NodeType::Agent {
        agent_id: formatter_agent_id,
        prompt_template: "Format the result to json format".to_string(),
    },
);

// Add nodes to graph
graph.add_node(node1.clone())?;
graph.add_node(node2.clone())?;

// Add edge between nodes
graph.add_edge(node1.id, node2.id, WorkflowEdge::data_flow())?;

// Validate graph properties
assert!(!graph.has_cycles());
assert_eq!(graph.node_count(), 2);
```

### Agent

Intelligent agents backed by LLM providers.

```rust
use graphbit_core::*;

// Create LLM configuration
let llm_config = LlmConfig::OpenAI {
    api_key: std::env::var("OPENAI_API_KEY")?,
    model: "gpt-4".to_string(),
    base_url: None,
    organization: None,
};

// Create agent configuration
let config = AgentConfig::new(
    "assistant",
    "AI Assistant",
    llm_config,
)
.with_system_prompt("You are a helpful assistant")
.with_max_tokens(1000)
.with_temperature(0.7);

// Build the agent
let agent = Agent::new(config).await?;

// Execute agent task
let message = AgentMessage::new(
    AgentId::new(),
    None,
    MessageContent::Text("Analyze this text: Hello world".to_string()),
);
let response = agent.execute(message).await?;
println!("Response: {:?}", response);
```

### LLM Providers

Multi-provider LLM integration with consistent interface.

```rust
use graphbit_core::llm::*;

// OpenAI configuration
let openai_config = LlmConfig::OpenAI {
    api_key: std::env::var("OPENAI_API_KEY")?,
    model: "gpt-4".to_string(),
    base_url: None,
    organization: None,
};

// Anthropic configuration
let anthropic_config = LlmConfig::Anthropic {
    api_key: std::env::var("ANTHROPIC_API_KEY")?,
    model: "claude-3-5-sonnet-20241022".to_string(),
    base_url: None,
};

// Ollama configuration
let ollama_config = LlmConfig::Ollama {
    model: "llama3.1".to_string(),
    base_url: Some("http://localhost:11434".to_string()),
};

// Azure LLM configuration
let azure_config = LlmConfig::AzureLlm {
    api_key: std::env::var("AZURELLM_API_KEY")?,
    deployment_name: "gpt-4".to_string(),
    endpoint: "https://your-resource.openai.azure.com".to_string(),
    api_version: "2024-02-15-preview".to_string(),
};

// Create provider and make requests
use graphbit_core::llm::LlmProviderFactory;

let provider_trait = LlmProviderFactory::create_provider(openai_config.clone())?;
let provider = LlmProvider::new(provider_trait, openai_config);

let request = LlmRequest::new("Hello, how are you?");
let response = provider.complete(request).await?;
println!("Response: {}", response.content);
```

### Workflow Execution

Concurrent workflow execution with dependency management.

```rust
use graphbit_core::*;

// Create executor with different optimization profiles
let executor = WorkflowExecutor::new(); // Default balanced profile

// Configure executor
let executor = executor
    .with_default_llm_config(llm_config)
    .with_fail_fast(false)
    .with_max_node_execution_time(30000); // 30 seconds

// Execute workflow
let context = executor.execute(workflow).await?;

// Check execution state
match context.state {
    WorkflowState::Completed => println!("Success!"),
    WorkflowState::Failed { error } => println!("Failed: {}", error),
    _ => println!("Other state: {:?}", context.state),
}

// Access node outputs
for (node_id, output) in &context.node_outputs {
    println!("Node {}: {:?}", node_id, output);
}
```

## Advanced Usage

### Document Loading

Load and extract content from various document formats:

```rust
use graphbit_core::*;

// Create document loader
let loader = DocumentLoader::new();

// Load a PDF document
let doc = loader.load_document("path/to/document.pdf", "pdf").await?;
println!("Extracted text: {}", doc.content);
println!("Metadata: {:?}", doc.metadata);

// Load from URL
let doc = loader.load_document("https://example.com/doc.pdf", "pdf").await?;

// Supported formats: pdf, txt, docx, json, csv, xml, html
```

### Text Splitting

Split large documents into manageable chunks:

```rust
use graphbit_core::*;

// Create text splitter with character-based strategy
let config = TextSplitterConfig {
    strategy: SplitterStrategy::Character {
        chunk_size: 1000,
        chunk_overlap: 200,
    },
    preserve_word_boundaries: true,
    trim_whitespace: true,
    include_metadata: true,
    extra_params: HashMap::new(),
};

let splitter = TextSplitterFactory::create_splitter(config)?;
let chunks = splitter.split_text("Your long text here...")?;

for chunk in chunks {
    println!("Chunk {}: {} chars", chunk.chunk_index, chunk.content.len());
}

// Other strategies: Token, Sentence, Recursive, Paragraph, Markdown
```

### Embeddings

Generate embeddings for text using various providers:

```rust
use graphbit_core::*;

// Create embedding configuration
let config = EmbeddingConfig {
    provider: EmbeddingProvider::OpenAI,
    api_key: std::env::var("OPENAI_API_KEY")?,
    model: "text-embedding-3-small".to_string(),
    base_url: None,
    timeout_seconds: Some(30),
    max_batch_size: Some(100),
    extra_params: HashMap::new(),
};

// Create embedding service
let service = EmbeddingService::new(config)?;

// Generate embeddings for single text
let embedding = service.embed_text("Hello, world!").await?;
println!("Embedding dimensions: {}", embedding.len());

// Generate embeddings for multiple texts
let texts = vec![
    "First text".to_string(),
    "Second text".to_string(),
];
let embeddings = service.embed_texts(&texts).await?;
println!("Generated {} embeddings", embeddings.len());
```

### Retry and Circuit Breaker Configuration

Configure retry logic and circuit breakers for resilient execution:

```rust
use graphbit_core::*;
use graphbit_core::types::{RetryConfig, CircuitBreakerConfig, RetryableErrorType};

// Configure retry behavior
let retry_config = RetryConfig {
    max_attempts: 3,
    initial_delay_ms: 1000,
    max_delay_ms: 10000,
    backoff_multiplier: 2.0,
    jitter_factor: 0.1,
    retryable_errors: vec![
        RetryableErrorType::NetworkError,
        RetryableErrorType::TimeoutError,
    ],
};

// Configure circuit breaker
let circuit_breaker_config = CircuitBreakerConfig {
    failure_threshold: 5,
    success_threshold: 2,
    recovery_timeout_ms: 60000,
    failure_window_ms: 300000,
};

// Create executor with configurations
let executor = WorkflowExecutor::new()
    .with_retry_config(retry_config)
    .with_circuit_breaker_config(circuit_breaker_config);
```

## Performance Optimization

### Concurrent Execution

The workflow executor supports different optimization profiles:

```rust
use graphbit_core::*;

// Default balanced profile
let executor = WorkflowExecutor::new();

// Execute with performance monitoring
let start = std::time::Instant::now();
let context = executor.execute(workflow).await?;
let duration = start.elapsed();

println!("Execution time: {:?}", duration);
if let Some(stats) = &context.stats {
    println!("Total nodes: {}", stats.total_nodes);
    println!("Successful nodes: {}", stats.successful_nodes);
    println!("Failed nodes: {}", stats.failed_nodes);
    println!("Avg execution time: {} ms", stats.avg_execution_time_ms);
}

// Get concurrency statistics
let stats = executor.get_concurrency_stats().await;
println!("Active tasks: {}", stats.active_tasks);
println!("Queued tasks: {}", stats.queued_tasks);
```

## Key Types Reference

### Node Types

The `NodeType` enum defines different types of workflow nodes:

```rust
pub enum NodeType {
    Agent {
        agent_id: AgentId,
        prompt_template: String,
    },
    Condition {
        expression: String,
    },
    Transform {
        transformation: String,
    },
    Split,
    Join,
    Delay {
        duration_seconds: u64,
    },
    HttpRequest {
        url: String,
        method: String,
        headers: HashMap<String, String>,
    },
    Custom {
        function_name: String,
    },
    DocumentLoader {
        document_type: String,
        source_path: String,
        encoding: Option<String>,
    },
}
```

### Workflow State

The `WorkflowState` enum tracks workflow execution status:

```rust
pub enum WorkflowState {
    Pending,
    Running { current_node: NodeId },
    Completed,
    Failed { error: String },
    Cancelled,
}
```

### LLM Configuration

The `LlmConfig` enum supports multiple providers:

```rust
pub enum LlmConfig {
    OpenAI { api_key: String, model: String, base_url: Option<String>, organization: Option<String> },
    Anthropic { api_key: String, model: String, base_url: Option<String> },
    Ollama { model: String, base_url: Option<String> },
    AzureLlm { api_key: String, deployment_name: String, endpoint: String, api_version: String },
    MistralAI { api_key: String, model: String, base_url: Option<String> },
    DeepSeek { api_key: String, model: String, base_url: Option<String> },
    // ... and more providers
}
```

## Development

### Building

```bash
# Build library
cargo build

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open
```

## Supported LLM Providers

GraphBit Core supports the following LLM providers:

- **OpenAI** - GPT-4, GPT-3.5, and other OpenAI models
- **Anthropic** - Claude 3.5 Sonnet, Claude 3 Opus, and other Claude models
- **Azure LLM** - Azure-hosted models (OpenAI and other Azure AI models)
- **Ollama** - Local LLM execution
- **Mistral AI** - Mistral and Mixtral models
- **DeepSeek** - DeepSeek models
- **Fireworks AI** - Fireworks-hosted models
- **Replicate** - Replicate-hosted models
- ... and more providers

Each provider is configured through the `LlmConfig` enum with provider-specific parameters.

## Common Issues

### Build Errors

```bash
# Update Rust
rustup update

# Clean build
cargo clean && cargo build
```

### Test Failures

```bash
# Run single test
cargo test test_name --exact

# Show test output
cargo test -- --show-output
```

## License

See the [License](../LICENSE.md) file in the repository root for more information.
