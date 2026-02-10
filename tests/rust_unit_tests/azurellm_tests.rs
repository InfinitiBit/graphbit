//! Azure LLM Provider Unit Tests
//!
//! Comprehensive unit tests for Azure LLM provider that mirror OpenAI test coverage.

use graphbit_core::llm::{LlmConfig, LlmMessage, LlmProviderFactory, LlmRequest, LlmRole, LlmTool};
use serde_json::json;

#[tokio::test]
async fn test_azurellm_provider_creation() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::AzureLlm {
        api_key: "test-key".to_string(),
        deployment_name: "gpt-4o-mini".to_string(),
        endpoint: "https://test.openai.azure.com".to_string(),
        api_version: "2024-10-21".to_string(),
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "azurellm");
    assert_eq!(provider.model_name(), "gpt-4o-mini");
    assert!(provider.supports_function_calling());
    assert_eq!(provider.max_context_length(), Some(128_000));
}

#[tokio::test]
async fn test_azurellm_config_helpers() {
    // Test basic config
    let config = LlmConfig::azurellm(
        "test-key".to_string(),
        "gpt-4o-mini".to_string(),
        "https://test.openai.azure.com".to_string(),
        "2024-10-21".to_string(),
    );

    match config {
        LlmConfig::AzureLlm {
            api_key,
            deployment_name,
            endpoint,
            api_version,
        } => {
            assert_eq!(api_key, "test-key");
            assert_eq!(deployment_name, "gpt-4o-mini");
            assert_eq!(endpoint, "https://test.openai.azure.com");
            assert_eq!(api_version, "2024-10-21");
        }
        _ => panic!("Expected AzureLlm config"),
    }

    // Test config with defaults
    let config_defaults = LlmConfig::azurellm_with_defaults(
        "test-key".to_string(),
        "gpt-4o".to_string(),
        "https://test.openai.azure.com".to_string(),
    );

    match config_defaults {
        LlmConfig::AzureLlm {
            api_key,
            deployment_name,
            endpoint,
            api_version,
        } => {
            assert_eq!(api_key, "test-key");
            assert_eq!(deployment_name, "gpt-4o");
            assert_eq!(endpoint, "https://test.openai.azure.com");
            assert_eq!(api_version, "2024-10-21"); // Default version
        }
        _ => panic!("Expected AzureLlm config"),
    }
}

#[tokio::test]
async fn test_azurellm_message_formatting() {
    let _provider = LlmProviderFactory::create_provider(LlmConfig::AzureLlm {
        api_key: "test-key".to_string(),
        deployment_name: "gpt-4o-mini".to_string(),
        endpoint: "https://test.openai.azure.com".to_string(),
        api_version: "2024-10-21".to_string(),
    })
    .unwrap();

    let request = LlmRequest::with_messages(vec![])
        .with_message(LlmMessage::system("You are a helpful assistant."))
        .with_message(LlmMessage::user("What is the capital of France?"))
        .with_message(LlmMessage::assistant("The capital of France is Paris."))
        .with_max_tokens(100)
        .with_temperature(0.7)
        .with_top_p(0.9);

    assert_eq!(request.messages.len(), 3);
    assert!(request
        .messages
        .iter()
        .any(|m| matches!(m.role, LlmRole::System)));
    assert!(request
        .messages
        .iter()
        .any(|m| matches!(m.role, LlmRole::User)));
    assert!(request
        .messages
        .iter()
        .any(|m| matches!(m.role, LlmRole::Assistant)));
    assert_eq!(request.max_tokens, Some(100));
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.top_p, Some(0.9));
}

#[tokio::test]
async fn test_azurellm_with_tools() {
    let _provider = LlmProviderFactory::create_provider(LlmConfig::AzureLlm {
        api_key: "test-key".to_string(),
        deployment_name: "gpt-4o".to_string(),
        endpoint: "https://test.openai.azure.com".to_string(),
        api_version: "2024-10-21".to_string(),
    })
    .unwrap();

    let weather_tool = LlmTool::new(
        "get_weather",
        "Get current weather information",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Temperature unit"
                }
            },
            "required": ["location"]
        }),
    );

    let calculator_tool = LlmTool::new(
        "calculate",
        "Perform mathematical calculations",
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        }),
    );

    let request = LlmRequest::new("What's the weather like in San Francisco and calculate 2+2?")
        .with_tool(weather_tool)
        .with_tool(calculator_tool)
        .with_max_tokens(100)
        .with_temperature(0.0);

    assert_eq!(request.tools.len(), 2);
    assert_eq!(request.tools[0].name, "get_weather");
    assert_eq!(request.tools[1].name, "calculate");
}

#[tokio::test]
async fn test_azurellm_different_api_versions() {
    let versions = vec!["2024-10-21", "2024-06-01", "2023-12-01-preview"];

    for version in versions {
        let provider = LlmProviderFactory::create_provider(LlmConfig::AzureLlm {
            api_key: "test-key".to_string(),
            deployment_name: "gpt-4o-mini".to_string(),
            endpoint: "https://test.openai.azure.com".to_string(),
            api_version: version.to_string(),
        })
        .unwrap();

        assert_eq!(provider.provider_name(), "azurellm");
        assert_eq!(provider.model_name(), "gpt-4o-mini");
    }
}

#[tokio::test]
async fn test_azurellm_different_deployments() {
    let deployments = vec![
        ("gpt-4o", Some(128_000)),
        ("gpt-4o-mini", Some(128_000)),
        ("gpt-4-turbo", Some(128_000)),
        ("gpt-4", Some(128_000)),
        ("gpt-3.5-turbo", Some(128_000)),
        ("custom-deployment", Some(128_000)), // Default fallback
    ];

    for (deployment, expected_context) in deployments {
        let provider = LlmProviderFactory::create_provider(LlmConfig::AzureLlm {
            api_key: "test-key".to_string(),
            deployment_name: deployment.to_string(),
            endpoint: "https://test.openai.azure.com".to_string(),
            api_version: "2024-10-21".to_string(),
        })
        .unwrap();

        assert_eq!(provider.max_context_length(), expected_context);
        assert_eq!(provider.model_name(), deployment);
    }
}

#[tokio::test]
async fn test_azurellm_config_provider_detection() {
    let config = LlmConfig::azurellm(
        "test-key".to_string(),
        "gpt-4o".to_string(),
        "https://test.openai.azure.com".to_string(),
        "2024-10-21".to_string(),
    );

    assert_eq!(config.provider_name(), "azurellm");
    assert_eq!(config.model_name(), "gpt-4o");
}

#[tokio::test]
async fn test_azurellm_provider_capabilities() {
    let config = LlmConfig::AzureLlm {
        api_key: "test-key".to_string(),
        deployment_name: "gpt-4o-mini".to_string(),
        endpoint: "https://test.openai.azure.com".to_string(),
        api_version: "2024-10-21".to_string(),
    };

    let provider = LlmProviderFactory::create_provider(config).unwrap();

    // Azure LLM should support function calling
    assert!(provider.supports_function_calling());

    // Should have context length information
    assert!(provider.max_context_length().is_some());
    assert!(provider.max_context_length().unwrap() > 0);
}

#[tokio::test]
async fn test_azurellm_error_handling() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::AzureLlm {
        api_key: "test-key".to_string(),
        deployment_name: "gpt-4o-mini".to_string(),
        endpoint: "https://test.openai.azure.com".to_string(),
        api_version: "2024-10-21".to_string(),
    })
    .unwrap();

    // Test empty messages
    let request = LlmRequest::new("");
    let result = provider.complete(request).await;
    assert!(result.is_err());

    // Test invalid parameters
    let mut request = LlmRequest::new("test");
    request.extra_params.insert(
        "invalid_param".to_string(),
        serde_json::Value::String("invalid_value".to_string()),
    );
    let result = provider.complete(request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_azurellm_request_building() {
    let request = LlmRequest::new("Hello, Azure LLM!")
        .with_max_tokens(100)
        .with_temperature(0.7)
        .with_top_p(0.9);

    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.messages[0].content, "Hello, Azure LLM!");
    assert_eq!(request.max_tokens, Some(100));
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.top_p, Some(0.9));
}

#[test]
fn test_azurellm_token_estimation() {
    let request = LlmRequest::new("This is a test message for Azure LLM token estimation");
    let estimated_tokens = request.estimated_token_count();

    // Rough estimation: should be non-zero for non-empty content
    assert!(estimated_tokens > 0);
    assert!(estimated_tokens < 100); // Should be reasonable for short text
}
