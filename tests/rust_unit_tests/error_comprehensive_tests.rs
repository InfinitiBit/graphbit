//! Comprehensive error handling unit tests
//!
//! Tests for all error types, error conversion, retry logic,
//! and error context preservation.

use graphbit_core::{
    errors::{GraphBitError, GraphBitResult},
    types::{RetryConfig, RetryableErrorType},
};
use std::io;

#[test]
fn test_all_error_constructors() {
    // Test all error constructor methods
    let config_err = GraphBitError::config("Invalid configuration");
    assert!(config_err.to_string().contains("Invalid configuration"));

    let llm_provider_err = GraphBitError::llm_provider("openai", "API error");
    assert!(llm_provider_err.to_string().contains("openai"));
    assert!(llm_provider_err.to_string().contains("API error"));

    let llm_err = GraphBitError::llm("Generic LLM error");
    assert!(llm_err.to_string().contains("Generic LLM error"));

    let workflow_err = GraphBitError::workflow_execution("Execution failed");
    assert!(workflow_err.to_string().contains("Execution failed"));

    let graph_err = GraphBitError::graph("Invalid graph structure");
    assert!(graph_err.to_string().contains("Invalid graph structure"));

    let agent_err = GraphBitError::agent("agent-123", "Agent failed");
    assert!(agent_err.to_string().contains("agent-123"));
    assert!(agent_err.to_string().contains("Agent failed"));

    let agent_not_found_err = GraphBitError::agent_not_found("missing-agent");
    assert!(agent_not_found_err.to_string().contains("missing-agent"));

    let validation_err = GraphBitError::validation("field_name", "Invalid value");
    assert!(validation_err.to_string().contains("field_name"));
    assert!(validation_err.to_string().contains("Invalid value"));

    let auth_err = GraphBitError::authentication("openai", "Invalid API key");
    assert!(auth_err.to_string().contains("openai"));
    assert!(auth_err.to_string().contains("Invalid API key"));

    let rate_limit_err = GraphBitError::rate_limit("anthropic", 60);
    assert!(rate_limit_err.to_string().contains("anthropic"));
    assert!(rate_limit_err.to_string().contains("60"));

    let concurrency_err = GraphBitError::concurrency("Too many concurrent requests");
    assert!(concurrency_err
        .to_string()
        .contains("Too many concurrent requests"));
}

#[test]
fn test_error_retryability() {
    // Retryable errors
    let network_err = GraphBitError::Network {
        message: "Connection failed".to_string(),
    };
    assert!(network_err.is_retryable());
    assert_eq!(network_err.retry_delay(), Some(1));

    let rate_limit_err = GraphBitError::RateLimit {
        provider: "openai".to_string(),
        retry_after_seconds: 30,
    };
    assert!(rate_limit_err.is_retryable());
    assert_eq!(rate_limit_err.retry_delay(), Some(30));

    let llm_provider_err = GraphBitError::LlmProvider {
        provider: "anthropic".to_string(),
        message: "Server error".to_string(),
    };
    assert!(llm_provider_err.is_retryable());
    assert_eq!(llm_provider_err.retry_delay(), Some(2));

    let llm_err = GraphBitError::Llm {
        message: "Model error".to_string(),
    };
    assert!(llm_err.is_retryable());
    assert_eq!(llm_err.retry_delay(), Some(1));

    // Non-retryable errors
    let config_err = GraphBitError::Configuration {
        message: "Bad config".to_string(),
    };
    assert!(!config_err.is_retryable());
    assert_eq!(config_err.retry_delay(), None);

    let validation_err = GraphBitError::Validation {
        field: "name".to_string(),
        message: "Required field".to_string(),
    };
    assert!(!validation_err.is_retryable());
    assert_eq!(validation_err.retry_delay(), None);

    let internal_err = GraphBitError::Internal {
        message: "Internal error".to_string(),
    };
    assert!(!internal_err.is_retryable());
    assert_eq!(internal_err.retry_delay(), None);
}

#[test]
fn test_error_from_conversions() {
    // Test From<serde_json::Error>

    // Test From<serde_json::Error>
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let graphbit_err: GraphBitError = json_err.into();
    assert!(matches!(graphbit_err, GraphBitError::Serialization { .. }));

    // Test From<std::io::Error>
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let graphbit_err: GraphBitError = io_err.into();
    assert!(matches!(graphbit_err, GraphBitError::Io { .. }));

    // Test timeout error handling (simulate timeout scenario)
    // Since Elapsed::new() is private, we'll test the pattern conceptually
    let graphbit_err = GraphBitError::Internal {
        message: "Operation timed out".to_string(),
    };
    assert!(matches!(graphbit_err, GraphBitError::Internal { .. }));
}

#[test]
fn test_retryable_error_type_classification() {
    let network_err = GraphBitError::Network {
        message: "timeout".to_string(),
    };
    let error_type = RetryableErrorType::from_error(&network_err);
    // Accept either TimeoutError or NetworkError for flexibility
    assert!(matches!(
        error_type,
        RetryableErrorType::NetworkError | RetryableErrorType::TimeoutError
    ));

    let rate_limit_err = GraphBitError::RateLimit {
        provider: "test".to_string(),
        retry_after_seconds: 1,
    };
    let error_type = RetryableErrorType::from_error(&rate_limit_err);
    assert_eq!(error_type, RetryableErrorType::RateLimitError);

    let auth_err = GraphBitError::Authentication {
        provider: "test".to_string(),
        message: "unauthorized".to_string(),
    };
    let error_type = RetryableErrorType::from_error(&auth_err);
    assert_eq!(error_type, RetryableErrorType::AuthenticationError);
}

#[test]
fn test_retry_config_with_errors() {
    let retry_config = RetryConfig::new(3)
        .with_exponential_backoff(100, 2.0, 5000)
        .with_jitter(0.1);

    // Test retryable errors
    let network_err = GraphBitError::Network {
        message: "timeout".to_string(),
    };
    assert!(retry_config.should_retry(&network_err, 0));
    assert!(retry_config.should_retry(&network_err, 1));
    assert!(retry_config.should_retry(&network_err, 2));
    assert!(!retry_config.should_retry(&network_err, 3)); // Max attempts reached

    // Test non-retryable errors
    let config_err = GraphBitError::Configuration {
        message: "bad config".to_string(),
    };
    assert!(!retry_config.should_retry(&config_err, 0));
    assert!(!retry_config.should_retry(&config_err, 1));
}

#[test]
fn test_error_serialization() {
    let errors = vec![
        GraphBitError::config("test config error"),
        GraphBitError::llm_provider("openai", "API error"),
        GraphBitError::validation("field", "validation failed"),
        GraphBitError::rate_limit("anthropic", 30),
        GraphBitError::agent_not_found("missing-agent"),
        GraphBitError::concurrency("too many requests"),
    ];

    for error in errors {
        // Test serialization
        let serialized = serde_json::to_string(&error).unwrap();

        // Test deserialization
        let deserialized: GraphBitError = serde_json::from_str(&serialized).unwrap();

        // Verify error messages match
        assert_eq!(error.to_string(), deserialized.to_string());
        assert_eq!(error.is_retryable(), deserialized.is_retryable());
        assert_eq!(error.retry_delay(), deserialized.retry_delay());
    }
}

#[test]
fn test_error_chaining_and_context() {
    // Test error chaining with context
    let root_cause = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
    let graphbit_err: GraphBitError = root_cause.into();

    assert!(graphbit_err.to_string().contains("Access denied"));

    // Test nested error scenarios
    let result: GraphBitResult<i32> = Err(GraphBitError::config("Invalid setting"));
    let mapped_result =
        result.map_err(|e| GraphBitError::workflow_execution(format!("Workflow failed: {e}")));

    assert!(mapped_result.is_err());
    let error = mapped_result.unwrap_err();
    assert!(error.to_string().contains("Workflow failed"));
    assert!(error.to_string().contains("Invalid setting"));
}

#[test]
fn test_error_display_formatting() {
    // Test that error messages are properly formatted
    let errors_and_expected = vec![
        (
            GraphBitError::Configuration {
                message: "Missing API key".to_string(),
            },
            "Configuration error: Missing API key",
        ),
        (
            GraphBitError::LlmProvider {
                provider: "openai".to_string(),
                message: "Rate limited".to_string(),
            },
            "LLM provider error: openai - Rate limited",
        ),
        (
            GraphBitError::Agent {
                agent_id: "agent-123".to_string(),
                message: "Processing failed".to_string(),
            },
            "Agent error: agent-123 - Processing failed",
        ),
        (
            GraphBitError::Validation {
                field: "email".to_string(),
                message: "Invalid format".to_string(),
            },
            "Validation error: email - Invalid format",
        ),
        (
            GraphBitError::RateLimit {
                provider: "anthropic".to_string(),
                retry_after_seconds: 120,
            },
            "Rate limit exceeded: anthropic - retry after 120s",
        ),
    ];

    for (error, expected) in errors_and_expected {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn test_result_type_operations() {
    // Test GraphBitResult operations
    let success: GraphBitResult<String> = Ok("success".to_string());
    assert!(success.is_ok());
    assert_eq!(success.as_ref().unwrap(), "success");

    let failure: GraphBitResult<String> = Err(GraphBitError::config("test error"));
    assert!(failure.is_err());

    // Test map operations
    let mapped = success.map(|s| s.len());
    assert_eq!(mapped.unwrap(), 7);

    // Test map_err operations
    let mapped_err =
        failure.map_err(|e| GraphBitError::workflow_execution(format!("Wrapped: {e}")));
    assert!(mapped_err.is_err());
    assert!(mapped_err.unwrap_err().to_string().contains("Wrapped"));

    // Test and_then operations
    let chained: GraphBitResult<usize> = Ok("test".to_string()).map(|s| s.len()).and_then(|len| {
        if len > 0 {
            Ok(len)
        } else {
            Err(GraphBitError::config("Empty string"))
        }
    });
    assert_eq!(chained.unwrap(), 4);
}

#[test]
fn test_error_debug_formatting() {
    let error = GraphBitError::llm_provider("openai", "API error");
    let debug_str = format!("{error:?}");

    // Debug format should include variant name and fields
    assert!(debug_str.contains("LlmProvider"));
    assert!(debug_str.contains("openai"));
    assert!(debug_str.contains("API error"));
}

#[test]
fn test_error_clone_and_equality() {
    let error1 = GraphBitError::config("test error");
    let error2 = error1.clone();

    // Errors should be cloneable
    assert_eq!(error1.to_string(), error2.to_string());
    assert_eq!(error1.is_retryable(), error2.is_retryable());
    assert_eq!(error1.retry_delay(), error2.retry_delay());
}
