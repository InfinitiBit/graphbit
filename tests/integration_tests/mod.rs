//! Integration tests for GraphBit
//!
//! This module contains integration tests that test the complete functionality
//! of the GraphBit framework, including workflow execution,
//! and end-to-end scenarios.

pub mod agent_tests;
pub mod error_handling_tests;
pub mod file_io_tests;
pub mod graph_tests;
pub mod validation_tests;
pub mod workflow_tests;

/// Get API key from environment or provide test default
pub fn get_test_api_key() -> String {
    std::env::var("OPENAI_API_KEY")
        .or_else(|_| std::env::var("TEST_OPENAI_API_KEY"))
        .unwrap_or_else(|_| "test-api-key-placeholder".to_string())
}

/// Get LLM model from environment or provide test default
pub fn get_test_model() -> String {
    std::env::var("TEST_LLM_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string())
}
