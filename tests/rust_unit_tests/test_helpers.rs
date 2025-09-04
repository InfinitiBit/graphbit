use graphbit_core::{
    agents::{Agent, AgentBuilder},
    embeddings::{
        EmbeddingConfig, EmbeddingProvider, EmbeddingProviderFactory, EmbeddingProviderTrait,
    },
    llm::{LlmConfig, LlmProviderFactory, LlmProviderTrait},
    types::AgentCapability,
    workflow::{WorkflowBuilder, WorkflowExecutor},
};
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

// API Key Helpers
#[allow(dead_code)]
pub fn has_openai_key() -> bool {
    env::var("OPENAI_API_KEY").is_ok()
}

#[allow(dead_code)]
pub fn has_anthropic_key() -> bool {
    env::var("ANTHROPIC_API_KEY").is_ok()
}

#[allow(dead_code)]
pub fn has_huggingface_key() -> bool {
    env::var("HUGGINGFACE_API_KEY").is_ok()
}

#[allow(dead_code)]
pub fn has_perplexity_key() -> bool {
    env::var("PERPLEXITY_API_KEY").is_ok()
}

#[allow(dead_code)]
pub fn has_deepseek_key() -> bool {
    env::var("DEEPSEEK_API_KEY").is_ok()
}

#[allow(dead_code)]
// Ollama Helper
pub async fn is_ollama_available() -> bool {
    let client = reqwest::Client::new();
    match client
        .get("http://localhost:11434/api/version")
        .send()
        .await
    {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[allow(dead_code)]
// Test LLM Provider Factory
pub fn create_test_llm_provider() -> Box<dyn LlmProviderTrait> {
    let config = LlmConfig::OpenAI {
        api_key: "test_key".to_string(),
        model: "test_model".to_string(),
        base_url: None,
        organization: None,
    };
    LlmProviderFactory::create_provider(config).unwrap()
}

#[allow(dead_code)]
// Test Embedding Provider Factory
pub fn create_test_embedding_provider() -> Box<dyn EmbeddingProviderTrait> {
    let config = EmbeddingConfig {
        provider: EmbeddingProvider::OpenAI,
        api_key: "test_key".to_string(),
        model: "text-embedding-ada-002".to_string(),
        base_url: None,
        timeout_seconds: Some(5),
        max_batch_size: Some(1),
        extra_params: Default::default(),
    };
    EmbeddingProviderFactory::create_provider(config).unwrap()
}

#[allow(dead_code)]
// Test Agent Builder
pub async fn create_test_agent() -> Arc<Agent> {
    let llm_config = LlmConfig::OpenAI {
        api_key: std::env::var("OPENAI_API_KEY").unwrap(),
        model: "gpt-3.5-turbo".to_string(),
        base_url: None,
        organization: None,
    };

    Arc::new(
        AgentBuilder::new("test_agent", llm_config)
            .description("A test agent")
            .capabilities(vec![AgentCapability::TextProcessing])
            .build()
            .await
            .unwrap(),
    )
}

#[allow(dead_code)]
// Test Workflow Builder
pub fn create_test_workflow() -> graphbit_core::workflow::Workflow {
    WorkflowBuilder::new("test_workflow")
        .description("A test workflow")
        .metadata("test_key".to_string(), serde_json::json!("test_value"))
        .build()
        .unwrap()
}

#[allow(dead_code)]
// Test Node Creation
pub fn create_test_node(
    node_type: graphbit_core::graph::NodeType,
) -> graphbit_core::graph::WorkflowNode {
    let mut node = graphbit_core::graph::WorkflowNode::new("test_node", "A test node", node_type);
    node.tags = Vec::new();
    node
}

#[allow(dead_code)]
// Test Executor
pub fn create_test_executor() -> WorkflowExecutor {
    WorkflowExecutor::new()
        .with_max_node_execution_time(5000)
        .with_fail_fast(true)
}

#[allow(dead_code)]
// Temporary File Helper
pub fn create_temp_file(content: &str) -> tempfile::NamedTempFile {
    use std::io::Write;
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    temp_file.write_all(content.as_bytes()).unwrap();
    temp_file.flush().unwrap();
    temp_file
}

#[allow(dead_code)]
// Test Data Generator
pub struct TestData {
    pub text: String,
    pub json: serde_json::Value,
    pub file: tempfile::NamedTempFile,
}

impl TestData {
    pub fn new() -> Self {
        let text = "This is test content".to_string();
        let json = serde_json::json!({
            "key": "value",
            "number": 42,
            "array": [1, 2, 3],
            "nested": {
                "inner": "data"
            }
        });
        let file = create_temp_file(&text);

        Self { text, json, file }
    }
}

impl Default for TestData {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
// Async Test Helper
pub fn run_async<F>(future: F) -> F::Output
where
    F: std::future::Future,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

// Additional helpers for comprehensive tests
#[allow(dead_code)]
/// Create a mock LLM configuration for testing
pub fn create_mock_llm_config() -> graphbit_core::llm::LlmConfig {
    graphbit_core::llm::LlmConfig::OpenAI {
        api_key: "test-api-key".to_string(),
        model: "gpt-3.5-turbo".to_string(),
        base_url: None,
        organization: None,
    }
}

#[allow(dead_code)]
/// Create a test retry configuration
pub fn create_test_retry_config() -> graphbit_core::types::RetryConfig {
    graphbit_core::types::RetryConfig::new(3)
        .with_exponential_backoff(100, 2.0, 5000)
        .with_jitter(0.1)
}

#[allow(dead_code)]
/// Create various test error types
pub fn create_test_errors() -> Vec<graphbit_core::errors::GraphBitError> {
    vec![
        graphbit_core::errors::GraphBitError::Configuration {
            message: "Test config error".to_string(),
        },
        graphbit_core::errors::GraphBitError::Network {
            message: "Test network error".to_string(),
        },
        graphbit_core::errors::GraphBitError::LlmProvider {
            provider: "test_provider".to_string(),
            message: "Test LLM error".to_string(),
        },
        graphbit_core::errors::GraphBitError::Agent {
            agent_id: "test-agent".to_string(),
            message: "Test agent error".to_string(),
        },
        graphbit_core::errors::GraphBitError::Validation {
            field: "test_field".to_string(),
            message: "Test validation error".to_string(),
        },
        graphbit_core::errors::GraphBitError::RateLimit {
            provider: "test_provider".to_string(),
            retry_after_seconds: 30,
        },
    ]
}

#[allow(dead_code)]
/// Wait for a condition to be true with timeout
pub async fn wait_for_condition<F>(
    mut condition: F,
    timeout_ms: u64,
    check_interval_ms: u64,
) -> bool
where
    F: FnMut() -> bool,
{
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);
    let interval = std::time::Duration::from_millis(check_interval_ms);

    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        tokio::time::sleep(interval).await;
    }

    false
}

#[allow(dead_code)]
/// Measure execution time of an async operation
pub async fn measure_async_execution_time<F, Fut, T>(operation: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let start = std::time::Instant::now();
    let result = operation().await;
    let duration = start.elapsed();
    (result, duration)
}

#[allow(dead_code)]
/// Measure execution time of a sync operation
pub fn measure_execution_time<F, T>(operation: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let start = std::time::Instant::now();
    let result = operation();
    let duration = start.elapsed();
    (result, duration)
}

// Skip macro intentionally removed to avoid duplicate definitions across modules
