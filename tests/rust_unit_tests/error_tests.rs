use graphbit_core::*;

#[test]
fn test_error_creation() {
    let error = GraphBitError::config("test error");
    assert!(error.to_string().contains("test error"));

    let error = GraphBitError::validation("field", "validation error");
    assert!(error.to_string().contains("validation error"));
}

#[test]
fn test_result_mapping() {
    let result: GraphBitResult<i32> = Ok(42);
    assert!(matches!(result, Ok(42)));

    let error_result: GraphBitResult<i32> = Err(GraphBitError::config("test error"));
    assert!(error_result.is_err());
}

#[test]
fn test_error_chain() {
    let source_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error = GraphBitError::from(source_error);
    assert!(error.to_string().contains("file not found"));
}

#[test]
fn test_error_retryable_and_delay() {
    // LLM provider error should be retryable with delay
    let e = GraphBitError::llm_provider("openai", "server error");
    assert!(e.is_retryable());
    assert_eq!(e.retry_delay(), Some(2));

    // Rate limit includes retry-after
    let e = GraphBitError::rate_limit("anthropic", 7);
    assert!(e.is_retryable());
    assert_eq!(e.retry_delay(), Some(7));

    // Internal error is not retryable
    let e = GraphBitError::Internal {
        message: "boom".into(),
    };
    assert!(!e.is_retryable());
    assert_eq!(e.retry_delay(), None);
}

#[test]
fn test_all_error_constructors() {
    // Test all error constructor methods
    let config_error = GraphBitError::config("config issue");
    assert!(config_error.to_string().contains("config issue"));

    let llm_provider_error = GraphBitError::llm_provider("openai", "API error");
    assert!(llm_provider_error.to_string().contains("openai"));
    assert!(llm_provider_error.to_string().contains("API error"));

    let llm_error = GraphBitError::llm("model error");
    assert!(llm_error.to_string().contains("model error"));

    let workflow_error = GraphBitError::workflow_execution("execution failed");
    assert!(workflow_error.to_string().contains("execution failed"));

    let graph_error = GraphBitError::graph("invalid graph");
    assert!(graph_error.to_string().contains("invalid graph"));

    let agent_error = GraphBitError::agent("agent-123", "agent failed");
    assert!(agent_error.to_string().contains("agent-123"));
    assert!(agent_error.to_string().contains("agent failed"));

    let agent_not_found = GraphBitError::agent_not_found("missing-agent");
    assert!(agent_not_found.to_string().contains("missing-agent"));

    let validation_error = GraphBitError::validation("email", "invalid format");
    assert!(validation_error.to_string().contains("email"));
    assert!(validation_error.to_string().contains("invalid format"));

    let auth_error = GraphBitError::authentication("google", "invalid token");
    assert!(auth_error.to_string().contains("google"));
    assert!(auth_error.to_string().contains("invalid token"));

    let rate_limit_error = GraphBitError::rate_limit("anthropic", 30);
    assert!(rate_limit_error.to_string().contains("anthropic"));
    assert!(rate_limit_error.to_string().contains("30"));

    let concurrency_error = GraphBitError::concurrency("too many requests");
    assert!(concurrency_error.to_string().contains("too many requests"));
}

#[test]
fn test_error_conversions() {
    // Test reqwest::Error conversion - create a timeout error
    let timeout_error = std::time::Duration::from_secs(1);
    let client = reqwest::Client::builder()
        .timeout(timeout_error)
        .build()
        .unwrap();

    // Create a mock reqwest error by trying to connect to a non-existent server
    // This will be converted to a GraphBitError::Network
    let rt = tokio::runtime::Runtime::new().unwrap();
    let reqwest_error =
        rt.block_on(async { client.get("http://127.0.0.1:1").send().await.unwrap_err() });

    let graphbit_error = GraphBitError::from(reqwest_error);
    assert!(matches!(graphbit_error, GraphBitError::Network { .. }));

    // Test serde_json::Error conversion
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let graphbit_error = GraphBitError::from(json_error);
    assert!(matches!(
        graphbit_error,
        GraphBitError::Serialization { .. }
    ));

    // Test std::io::Error conversion
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let graphbit_error = GraphBitError::from(io_error);
    assert!(matches!(graphbit_error, GraphBitError::Io { .. }));

    // Test anyhow::Error conversion
    let anyhow_error = anyhow::anyhow!("something went wrong");
    let graphbit_error = GraphBitError::from(anyhow_error);
    assert!(matches!(graphbit_error, GraphBitError::Internal { .. }));
}

#[test]
fn test_error_retryability_comprehensive() {
    // Test all retryable error types
    let network_error = GraphBitError::Network {
        message: "connection failed".to_string(),
    };
    assert!(network_error.is_retryable());
    assert_eq!(network_error.retry_delay(), Some(1));

    let rate_limit_error = GraphBitError::RateLimit {
        provider: "openai".to_string(),
        retry_after_seconds: 15,
    };
    assert!(rate_limit_error.is_retryable());
    assert_eq!(rate_limit_error.retry_delay(), Some(15));

    let llm_provider_error = GraphBitError::LlmProvider {
        provider: "anthropic".to_string(),
        message: "server error".to_string(),
    };
    assert!(llm_provider_error.is_retryable());
    assert_eq!(llm_provider_error.retry_delay(), Some(2));

    let llm_error = GraphBitError::Llm {
        message: "model timeout".to_string(),
    };
    assert!(llm_error.is_retryable());
    assert_eq!(llm_error.retry_delay(), Some(1));

    // Test non-retryable error types
    let config_error = GraphBitError::Configuration {
        message: "invalid config".to_string(),
    };
    assert!(!config_error.is_retryable());
    assert_eq!(config_error.retry_delay(), None);

    let validation_error = GraphBitError::Validation {
        field: "email".to_string(),
        message: "invalid format".to_string(),
    };
    assert!(!validation_error.is_retryable());
    assert_eq!(validation_error.retry_delay(), None);

    let auth_error = GraphBitError::Authentication {
        provider: "google".to_string(),
        message: "invalid credentials".to_string(),
    };
    assert!(!auth_error.is_retryable());
    assert_eq!(auth_error.retry_delay(), None);
}

#[test]
fn test_error_serialization() {
    // Test that errors can be serialized and deserialized
    let original_error = GraphBitError::agent("test-agent", "test message");

    let serialized = serde_json::to_string(&original_error).unwrap();
    let deserialized: GraphBitError = serde_json::from_str(&serialized).unwrap();

    assert_eq!(original_error.to_string(), deserialized.to_string());
}

#[test]
fn test_error_cloning() {
    // Test that all error variants can be cloned
    let errors = vec![
        GraphBitError::config("test"),
        GraphBitError::llm_provider("openai", "error"),
        GraphBitError::llm("error"),
        GraphBitError::workflow_execution("error"),
        GraphBitError::graph("error"),
        GraphBitError::agent("agent-1", "error"),
        GraphBitError::agent_not_found("agent-2"),
        GraphBitError::validation("field", "error"),
        GraphBitError::authentication("provider", "error"),
        GraphBitError::rate_limit("provider", 30),
        GraphBitError::concurrency("error"),
        GraphBitError::Network {
            message: "error".to_string(),
        },
        GraphBitError::Serialization {
            message: "error".to_string(),
        },
        GraphBitError::Internal {
            message: "error".to_string(),
        },
        GraphBitError::Io {
            message: "error".to_string(),
        },
    ];

    for error in errors {
        let cloned = error.clone();
        assert_eq!(error.to_string(), cloned.to_string());
        assert_eq!(error.is_retryable(), cloned.is_retryable());
        assert_eq!(error.retry_delay(), cloned.retry_delay());
    }
}

#[test]
fn test_error_debug_formatting() {
    let error = GraphBitError::agent("test-agent", "test message");
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("Agent"));
    assert!(debug_str.contains("test-agent"));
    assert!(debug_str.contains("test message"));
}

#[test]
fn test_result_type_alias() {
    // Test that GraphBitResult works as expected
    let success: GraphBitResult<String> = Ok("success".to_string());
    assert!(success.is_ok());
    if let Ok(value) = success {
        assert_eq!(value, "success");
    }

    let failure: GraphBitResult<String> = Err(GraphBitError::config("test error"));
    assert!(failure.is_err());
    if let Err(error) = failure {
        assert!(error.to_string().contains("test error"));
    }
}

#[test]
fn test_error_chaining_with_context() {
    // Test error chaining with different source errors
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let graphbit_error = GraphBitError::from(io_error);

    match graphbit_error {
        GraphBitError::Io { message } => {
            assert!(message.contains("access denied"));
        }
        _ => panic!("Expected IO error variant"),
    }

    // Test with network error - create another reqwest error
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let reqwest_error =
        rt.block_on(async { client.get("http://127.0.0.1:2").send().await.unwrap_err() });
    let graphbit_error = GraphBitError::from(reqwest_error);

    match graphbit_error {
        GraphBitError::Network { message: _ } => {
            // Success - correct variant
        }
        _ => panic!("Expected Network error variant"),
    }
}
