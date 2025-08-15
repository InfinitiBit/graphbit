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
        api_key: "test_key".to_string(),
        model: "test_model".to_string(),
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

// Skip macro intentionally removed to avoid duplicate definitions across modules
