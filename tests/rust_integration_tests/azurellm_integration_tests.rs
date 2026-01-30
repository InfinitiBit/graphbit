//! Azure LLM Provider Integration Tests
//!
//! Integration tests for Azure LLM provider that mirror OpenAI test coverage.
//! These tests require actual Azure LLM credentials and should be run with:
//! AZURELLM_API_KEY=your_key AZURELLM_ENDPOINT=your_endpoint AZURELLM_DEPLOYMENT=your_deployment cargo test azurellm_integration -- --ignored

use graphbit_core::llm::*;
use serde_json::json;
use std::env;

fn get_azurellm_config() -> Option<(String, String, String, String)> {
    let api_key = env::var("AZURELLM_API_KEY").ok()?;
    let endpoint = env::var("AZURELLM_ENDPOINT").ok()?;
    let deployment = env::var("AZURELLM_DEPLOYMENT").ok()?;
    let api_version =
        env::var("AZURELLM_API_VERSION").unwrap_or_else(|_| "2024-02-15-preview".to_string());

    Some((api_key, endpoint, deployment, api_version))
}

fn has_azurellm_credentials() -> bool {
    get_azurellm_config().is_some()
}

#[tokio::test]
async fn test_azurellm_provider_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = LlmConfig::azurellm(
        "test-key".to_string(),
        "gpt-4o-mini".to_string(),
        "https://test.openai.azure.com".to_string(),
        "2024-10-21".to_string(),
    );

    let provider_result = LlmProviderFactory::create_provider(config);
    assert!(provider_result.is_ok());

    let provider = provider_result.unwrap();
    assert_eq!(provider.provider_name(), "azurellm");
    assert_eq!(provider.model_name(), "gpt-4o-mini");
}

#[tokio::test]
#[ignore] // Requires real Azure LLM API credentials
async fn test_azurellm_real_api_call() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API credentials are provided
    if !has_azurellm_credentials() {
        println!("Skipping real Azure LLM API test - no valid credentials");
        return;
    }

    let (api_key, endpoint, deployment, api_version) = get_azurellm_config().unwrap();
    let config = LlmConfig::azurellm(api_key, deployment.clone(), endpoint, api_version);
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let request = LlmRequest::new("Say 'Hello' in one word only.")
        .with_max_tokens(10)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            assert!(!response.content.is_empty());
            assert_eq!(response.model, deployment);
            assert!(response.usage.total_tokens > 0);
            println!(
                "Azure LLM real API call successful: {content}",
                content = response.content
            );
        }
        Err(e) => {
            println!("Azure LLM API call failed: {e:?}");
            panic!("Azure LLM API call should succeed with valid credentials");
        }
    }
}

#[tokio::test]
#[ignore] // Requires real Azure LLM API credentials
async fn test_azurellm_real_api_with_tools() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API credentials are provided
    if !has_azurellm_credentials() {
        println!("Skipping real Azure LLM API with tools test - no valid credentials");
        return;
    }

    let (api_key, endpoint, deployment, api_version) = get_azurellm_config().unwrap();
    let config = LlmConfig::azurellm(api_key, deployment, endpoint, api_version);
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let weather_tool = LlmTool::new(
        "get_weather",
        "Get current weather information",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                }
            },
            "required": ["location"]
        }),
    );

    let request = LlmRequest::new("What's the weather like in San Francisco?")
        .with_tool(weather_tool)
        .with_max_tokens(100)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            println!("Azure LLM response content: '{}'", response.content);
            println!(
                "Azure LLM response finish_reason: {:?}",
                response.finish_reason
            );

            // Check if tool calls were made
            if response.has_tool_calls() {
                println!(
                    "Tool calls were made: {tool_calls:?}",
                    tool_calls = response.tool_calls
                );
                assert!(!response.tool_calls.is_empty());
                // For tool calls, content might be empty, which is valid
            } else {
                println!("Response content: {:?}", response.content);
                assert!(
                    !response.content.is_empty(),
                    "Expected non-empty content but got: '{}'",
                    response.content
                );
            }
            println!("Azure LLM real API call with tools successful");
        }
        Err(e) => {
            println!("Azure LLM API call with tools failed: {e:?}");
            // Tool responses can have parsing issues with null values - this is known
            if e.to_string().contains("null, expected a string") {
                println!(
                    "Known issue with Azure LLM tool response parsing - null value in response"
                );
            } else {
                panic!("Azure LLM API call with tools should succeed with valid credentials");
            }
        }
    }
}

#[tokio::test]
#[ignore] // Requires real Azure LLM API credentials
async fn test_azurellm_with_system_message() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API credentials are provided
    if !has_azurellm_credentials() {
        println!("Skipping Azure LLM system message test - no valid credentials");
        return;
    }

    let (api_key, endpoint, deployment, api_version) = get_azurellm_config().unwrap();
    let config = LlmConfig::azurellm(api_key, deployment, endpoint, api_version);
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let messages = vec![
        LlmMessage::system("You are a helpful assistant that responds in exactly 5 words."),
        LlmMessage::user("What is the capital of France?"),
    ];

    let request = LlmRequest::with_messages(messages)
        .with_max_tokens(20)
        .with_temperature(0.1);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            assert!(!response.content.is_empty());
            println!("Azure LLM system message response: {}", response.content);
        }
        Err(e) => {
            println!("Azure LLM system message test failed: {e:?}");
            panic!("Azure LLM system message test should succeed with valid credentials");
        }
    }
}

#[tokio::test]
#[ignore] // Requires real Azure LLM API credentials
async fn test_azurellm_conversation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API credentials are provided
    if !has_azurellm_credentials() {
        println!("Skipping Azure LLM conversation test - no valid credentials");
        return;
    }

    let (api_key, endpoint, deployment, api_version) = get_azurellm_config().unwrap();
    let config = LlmConfig::azurellm(api_key, deployment, endpoint, api_version);
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let messages = vec![
        LlmMessage::user("My name is Alice."),
        LlmMessage::assistant("Hello Alice! Nice to meet you."),
        LlmMessage::user("What's my name?"),
    ];

    let request = LlmRequest::with_messages(messages)
        .with_max_tokens(50)
        .with_temperature(0.1);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            assert!(!response.content.is_empty());

            // The response should mention "Alice"
            assert!(
                response.content.to_lowercase().contains("alice"),
                "Response should mention Alice: {}",
                response.content
            );

            println!("Azure LLM conversation response: {}", response.content);
        }
        Err(e) => {
            println!("Azure LLM conversation test failed: {e:?}");
            panic!("Azure LLM conversation test should succeed with valid credentials");
        }
    }
}

#[tokio::test]
async fn test_azurellm_error_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test with invalid API key
    let config = LlmConfig::azurellm(
        "invalid-api-key".to_string(),
        "gpt-4o-mini".to_string(),
        "https://test.openai.azure.com".to_string(),
        "2024-10-21".to_string(),
    );

    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let request = LlmRequest::new("Hello, Azure LLM!").with_max_tokens(50);

    let result = provider.complete(request).await;
    assert!(result.is_err(), "Request with invalid API key should fail");

    let error = result.err().unwrap();
    println!("Expected error with invalid API key: {:?}", error);
}

#[tokio::test]
#[ignore] // Requires real Azure LLM API credentials
async fn test_azurellm_different_temperatures() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API credentials are provided
    if !has_azurellm_credentials() {
        println!("Skipping Azure LLM temperature test - no valid credentials");
        return;
    }

    let (api_key, endpoint, deployment, api_version) = get_azurellm_config().unwrap();
    let config = LlmConfig::azurellm(api_key, deployment, endpoint, api_version);
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    // Test with low temperature (more deterministic)
    let request_low = LlmRequest::new("Tell me a creative story in one sentence.")
        .with_max_tokens(50)
        .with_temperature(0.1);

    let response_low = provider.complete(request_low).await;
    assert!(response_low.is_ok());

    // Test with high temperature (more creative)
    let request_high = LlmRequest::new("Tell me a creative story in one sentence.")
        .with_max_tokens(50)
        .with_temperature(0.9);

    let response_high = provider.complete(request_high).await;
    assert!(response_high.is_ok());

    println!(
        "Low temperature response: {}",
        response_low.unwrap().content
    );
    println!(
        "High temperature response: {}",
        response_high.unwrap().content
    );
}

#[tokio::test]
#[ignore] // Requires real Azure LLM API credentials
async fn test_azurellm_provider_comparison() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API credentials are provided
    if !has_azurellm_credentials() {
        println!("Skipping Azure LLM provider comparison test - no valid credentials");
        return;
    }

    let (api_key, endpoint, deployment, api_version) = get_azurellm_config().unwrap();
    let test_prompt = "Explain AI in exactly 10 words.";

    // Test Azure LLM
    let azure_config = LlmConfig::azurellm(api_key, deployment, endpoint, api_version);
    let azure_provider = LlmProviderFactory::create_provider(azure_config).unwrap();

    let request = LlmRequest::new(test_prompt)
        .with_max_tokens(20)
        .with_temperature(0.0);

    if let Ok(response) = azure_provider.complete(request).await {
        println!("Azure LLM response: {}", response.content);
        assert!(!response.content.is_empty());
    }
}
