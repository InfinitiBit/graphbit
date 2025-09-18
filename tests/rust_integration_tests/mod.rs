//! Integration tests for GraphBit
//!
//! This module contains integration tests that test the complete functionality
//! of the GraphBit framework, including workflow execution,
//! and end-to-end scenarios.

pub mod agent_tests;
pub mod concurrency_tests;
pub mod doc_processing_tests;
/// Integration tests for document loading functionality
pub mod document_loader_tests;
pub mod embeddings_tests;
pub mod error_handling_tests;
pub mod error_propagation_integration;
pub mod file_io_tests;
pub mod full_workflow_tests;
pub mod graph_tests;
pub mod llm_tests;
pub mod validation_tests;
pub mod workflow_execution_integration;
/// Integration tests for workflow execution functionality
pub mod workflow_tests;

/// Check if a valid `OpenAI` API key is available
pub fn has_openai_api_key() -> bool {
    std::env::var("OPENAI_API_KEY")
        .map(|key| !key.is_empty() && key != "test-api-key-placeholder")
        .unwrap_or(false)
}

/// Check if a valid `Anthropic` API key is available
pub fn has_anthropic_api_key() -> bool {
    std::env::var("ANTHROPIC_API_KEY")
        .map(|key| !key.is_empty() && key != "test-api-key-placeholder")
        .unwrap_or(false)
}

/// Check if a valid `HuggingFace` API key is available
pub fn has_huggingface_api_key() -> bool {
    std::env::var("HUGGINGFACE_API_KEY")
        .map(|key| !key.is_empty() && key != "test-api-key-placeholder")
        .unwrap_or(false)
}

/// Check if `Ollama` is available (by checking if the service is running)
pub async fn has_ollama_available() -> bool {
    // Try to connect to default Ollama endpoint
    let client = reqwest::Client::new();
    client
        .get("http://localhost:11434/api/version")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
        .is_ok()
}

/// Get `OpenAI` API key or skip test if not available
pub fn get_openai_api_key_or_skip() -> String {
    match std::env::var("OPENAI_API_KEY") {
        Ok(key) if !key.is_empty() && key != "test-api-key-placeholder" => key,
        _ => {
            println!("Skipping test - no valid OPENAI_API_KEY found");
            panic!("TEST_SKIP"); // Use panic with special message to indicate test skip
        }
    }
}

/// Get `Anthropic` API key or skip test if not available
pub fn get_anthropic_api_key_or_skip() -> String {
    match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) if !key.is_empty() && key != "test-api-key-placeholder" => key,
        _ => {
            println!("Skipping test - no valid ANTHROPIC_API_KEY found");
            panic!("TEST_SKIP");
        }
    }
}

/// Get `HuggingFace` API key or skip test if not available
pub fn get_huggingface_api_key_or_skip() -> String {
    match std::env::var("HUGGINGFACE_API_KEY") {
        Ok(key) if !key.is_empty() && key != "test-api-key-placeholder" => key,
        _ => {
            println!("Skipping test - no valid HUGGINGFACE_API_KEY found");
            panic!("TEST_SKIP");
        }
    }
}

/// Get API key from environment or provide test default (for non-API tests)
pub fn get_test_api_key() -> String {
    std::env::var("OPENAI_API_KEY")
        .or_else(|_| std::env::var("TEST_OPENAI_API_KEY"))
        .unwrap_or_else(|_| "test-api-key-placeholder".to_string())
}

/// Get LLM model from environment or provide test default
pub fn get_test_model() -> String {
    std::env::var("TEST_LLM_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string())
}

/// Skip test if condition is not met
#[allow(dead_code)]
pub fn skip_test_if(condition: bool, reason: &str) {
    if condition {
        println!("Skipping test - {reason}");
        panic!("TEST_SKIP");
    }
}

/// Macro to skip test if API credentials are not available
#[macro_export]
macro_rules! skip_if_no_api {
    (openai) => {
        if !$crate::has_openai_api_key() {
            println!("Skipping test - no valid OPENAI_API_KEY found");
            return;
        }
    };
    (anthropic) => {
        if !$crate::has_anthropic_api_key() {
            println!("Skipping test - no valid ANTHROPIC_API_KEY found");
            return;
        }
    };
    (huggingface) => {
        if !$crate::has_huggingface_api_key() {
            println!("Skipping test - no valid HUGGINGFACE_API_KEY found");
            return;
        }
    };
}
