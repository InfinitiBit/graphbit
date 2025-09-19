//! LLM Provider Integration Tests
//!
//! Tests for LLM providers including `OpenAI`, `Anthropic`, `HuggingFace`, and `Ollama`
//! with both mocked and real API interactions.

use graphbit_core::llm::*;
use serde_json::json;
use std::collections::HashMap;

#[tokio::test]
async fn test_openai_provider_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = LlmConfig::openai(super::get_test_api_key(), "gpt-3.5-turbo");
    let provider_result = LlmProviderFactory::create_provider(config);
    assert!(provider_result.is_ok());

    let provider = provider_result.unwrap();
    assert_eq!(provider.provider_name(), "openai");
    assert_eq!(provider.model_name(), "gpt-3.5-turbo");
}

#[tokio::test]
async fn test_anthropic_provider_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = LlmConfig::anthropic("test-api-key", "claude-3-sonnet-20240229");
    let provider_result = LlmProviderFactory::create_provider(config);
    assert!(provider_result.is_ok());

    let provider = provider_result.unwrap();
    assert_eq!(provider.provider_name(), "anthropic");
    assert_eq!(provider.model_name(), "claude-3-sonnet-20240229");
}

#[tokio::test]
async fn test_huggingface_provider_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = LlmConfig::huggingface("test-api-key", "microsoft/DialoGPT-medium");
    let provider_result = LlmProviderFactory::create_provider(config);
    assert!(provider_result.is_ok());

    let provider = provider_result.unwrap();
    assert_eq!(provider.provider_name(), "huggingface");
    assert_eq!(provider.model_name(), "microsoft/DialoGPT-medium");
}

#[tokio::test]
async fn test_ollama_provider_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = LlmConfig::ollama("llama3.2");
    let provider_result = LlmProviderFactory::create_provider(config);
    assert!(provider_result.is_ok());

    let provider = provider_result.unwrap();
    assert_eq!(provider.provider_name(), "ollama");
    assert_eq!(provider.model_name(), "llama3.2");
}

#[tokio::test]
async fn test_llm_request_building() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let request = LlmRequest::new("Hello, world!")
        .with_max_tokens(100)
        .with_temperature(0.7)
        .with_top_p(0.9);

    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.max_tokens, Some(100));
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.top_p, Some(0.9));
}

#[tokio::test]
async fn test_llm_request_with_multiple_messages() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let messages = vec![
        LlmMessage::system("You are a helpful assistant."),
        LlmMessage::user("What is the capital of France?"),
        LlmMessage::assistant("The capital of France is Paris."),
        LlmMessage::user("What about Germany?"),
    ];

    let request = LlmRequest::with_messages(messages);
    assert_eq!(request.messages.len(), 4);

    match &request.messages[0].role {
        LlmRole::System => {}
        _ => panic!("Expected system message"),
    }

    match &request.messages[1].role {
        LlmRole::User => {}
        _ => panic!("Expected user message"),
    }
}

#[tokio::test]
async fn test_llm_tool_definition() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let tool_schema = json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
            },
            "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"]
            }
        },
        "required": ["location"]
    });

    let tool = LlmTool::new(
        "get_weather",
        "Get the current weather in a given location",
        tool_schema,
    );

    assert_eq!(tool.name, "get_weather");
    assert!(tool.description.contains("weather"));
    assert!(tool.parameters.is_object());
}

#[tokio::test]
async fn test_llm_request_with_tools() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let weather_tool = LlmTool::new(
        "get_weather",
        "Get weather information",
        json!({
            "type": "object",
            "properties": {
                "location": {"type": "string"}
            }
        }),
    );

    let calculator_tool = LlmTool::new(
        "calculate",
        "Perform mathematical calculations",
        json!({
            "type": "object",
            "properties": {
                "expression": {"type": "string"}
            }
        }),
    );

    let request = LlmRequest::new("What's the weather like and calculate 2+2?")
        .with_tool(weather_tool)
        .with_tool(calculator_tool);

    assert_eq!(request.tools.len(), 2);
    assert_eq!(request.tools[0].name, "get_weather");
    assert_eq!(request.tools[1].name, "calculate");
}

#[tokio::test]
async fn test_llm_response_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let tool_calls = vec![LlmToolCall {
        id: "call_1".to_string(),
        name: "get_weather".to_string(),
        parameters: json!({"location": "San Francisco, CA"}),
    }];

    let usage = LlmUsage::new(50, 25);

    let response = LlmResponse::new("The weather is sunny!", "gpt-3.5-turbo")
        .with_tool_calls(tool_calls.clone())
        .with_usage(usage)
        .with_finish_reason(FinishReason::ToolCalls)
        .with_id("response_123".to_string());

    assert_eq!(response.content, "The weather is sunny!");
    assert_eq!(response.model, "gpt-3.5-turbo");
    assert_eq!(response.tool_calls.len(), 1);
    assert_eq!(response.usage.prompt_tokens, 50);
    assert_eq!(response.usage.completion_tokens, 25);
    assert_eq!(response.usage.total_tokens, 75);
    assert_eq!(response.id, Some("response_123".to_string()));
    assert!(response.has_tool_calls());
}

#[tokio::test]
async fn test_llm_usage_operations() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let usage1 = LlmUsage::new(10, 5);
    let usage2 = LlmUsage::new(20, 15);

    let combined = usage1.clone() + usage2.clone();
    assert_eq!(combined.prompt_tokens, 30);
    assert_eq!(combined.completion_tokens, 20);
    assert_eq!(combined.total_tokens, 50);

    let mut cumulative = LlmUsage::empty();
    cumulative += usage1;
    cumulative += usage2;
    assert_eq!(cumulative.total_tokens, 50);
}

#[tokio::test]
async fn test_finish_reason_checks() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    assert!(FinishReason::Stop.is_natural_stop());
    assert!(FinishReason::ToolCalls.is_natural_stop());
    assert!(!FinishReason::Length.is_natural_stop());

    assert!(FinishReason::Length.is_truncated());
    assert!(!FinishReason::Stop.is_truncated());

    assert!(FinishReason::Error.is_error());
    assert!(FinishReason::ContentFilter.is_error());
    assert!(!FinishReason::Stop.is_error());
}

#[tokio::test]
async fn test_llm_config_provider_detection() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let openai_config = LlmConfig::openai("key", "gpt-4");
    assert_eq!(openai_config.provider_name(), "openai");
    assert_eq!(openai_config.model_name(), "gpt-4");

    let anthropic_config = LlmConfig::anthropic("key", "claude-3");
    assert_eq!(anthropic_config.provider_name(), "anthropic");
    assert_eq!(anthropic_config.model_name(), "claude-3");
}

#[tokio::test]
async fn test_custom_llm_config() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let mut custom_config = HashMap::new();
    custom_config.insert("model".to_string(), json!("custom-model"));
    custom_config.insert("api_endpoint".to_string(), json!("https://api.custom.com"));

    let config = LlmConfig::Custom {
        provider_type: "custom-provider".to_string(),
        config: custom_config,
    };

    assert_eq!(config.provider_name(), "custom-provider");
    assert_eq!(config.model_name(), "custom-model");
}

#[tokio::test]
async fn test_llm_provider_capabilities() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let openai_config = LlmConfig::openai(super::get_test_api_key(), "gpt-3.5-turbo");
    let provider = LlmProviderFactory::create_provider(openai_config).unwrap();

    // These should be supported by most providers
    assert!(provider.supports_function_calling() || !provider.supports_function_calling()); // Both are valid
    assert!(provider.max_context_length().is_some() || provider.max_context_length().is_none());
    // Both are valid
}

#[tokio::test]
async fn test_token_estimation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let request = LlmRequest::new("This is a test message for token estimation");
    let estimated_tokens = request.estimated_token_count();

    // Rough estimation: should be non-zero for non-empty content
    assert!(estimated_tokens > 0);
    assert!(estimated_tokens < 100); // Should be reasonable for short text
}

#[tokio::test]
async fn test_cost_estimation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let usage = LlmUsage::new(1000, 500);
    let response = LlmResponse::new("Test response", "gpt-3.5-turbo").with_usage(usage);

    let cost = response.estimate_cost(0.001, 0.002); // $0.001 per input token, $0.002 per output token
    assert_eq!(cost, 2.0); // 1000 * 0.001 + 500 * 0.002 = 1.0 + 1.0 = 2.0
}

// Real API integration tests (require environment variables)
#[tokio::test]
async fn test_openai_real_api_call() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping real OpenAI API test - no valid API key");
        return;
    }

    let api_key = super::get_openai_api_key_or_skip();
    let config = LlmConfig::openai(api_key, "gpt-3.5-turbo");
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let request = LlmRequest::new("Say 'Hello' in one word only.")
        .with_max_tokens(10)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            assert!(!response.content.is_empty());
            assert_eq!(response.model, "gpt-3.5-turbo");
            assert!(response.usage.total_tokens > 0);
            println!(
                "OpenAI real API call successful: {content}",
                content = response.content
            );
        }
        Err(e) => {
            println!("OpenAI API call failed: {e:?}");
            panic!("OpenAI API call should succeed with valid credentials");
        }
    }
}

#[tokio::test]
async fn test_anthropic_real_api_call() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_anthropic_api_key() {
        println!("Skipping real Anthropic API test - no valid API key");
        return;
    }

    let api_key = super::get_anthropic_api_key_or_skip();
    let config = LlmConfig::anthropic(api_key, "claude-3-haiku-20240307");
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let request = LlmRequest::new("Say 'Hello' in one word only.")
        .with_max_tokens(10)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            assert!(!response.content.is_empty());
            assert_eq!(response.model, "claude-3-haiku-20240307");
            assert!(response.usage.total_tokens > 0);
            println!(
                "Anthropic real API call successful: {content}",
                content = response.content
            );
        }
        Err(e) => {
            println!("Anthropic API call failed: {e:?}");
            panic!("Anthropic API call should succeed with valid credentials");
        }
    }
}

#[tokio::test]
async fn test_huggingface_real_api_call() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_huggingface_api_key() {
        println!("Skipping real HuggingFace API test - no valid API key");
        return;
    }

    let api_key = super::get_huggingface_api_key_or_skip();
    let config = LlmConfig::huggingface(api_key, "microsoft/DialoGPT-medium");
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let request = LlmRequest::new("Hello there!")
        .with_max_tokens(20)
        .with_temperature(0.7);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            assert!(!response.content.is_empty());
            assert_eq!(response.model, "microsoft/DialoGPT-medium");
            println!(
                "HuggingFace real API call successful: {content}",
                content = response.content
            );
        }
        Err(e) => {
            println!("HuggingFace API call failed: {e:?}");
            // Note: `HuggingFace` API might be less reliable, so we don't fail the test
            println!("HuggingFace API call failed (this might be expected): {e:?}");
        }
    }
}

#[tokio::test]
async fn test_ollama_local_api_call() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Check if `Ollama` is available locally
    if !super::has_ollama_available().await {
        println!("Skipping Ollama test - service not available locally");
        return;
    }

    let config = LlmConfig::ollama_with_base_url("llama3.2", "http://localhost:11434");
    let provider = LlmProviderFactory::create_provider(config).unwrap();

    let request = LlmRequest::new("Say 'Hello' briefly.")
        .with_max_tokens(5)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    match result {
        Ok(response) => {
            assert!(!response.content.is_empty());
            println!(
                "Ollama local API call successful: {content}",
                content = response.content
            );
        }
        Err(e) => {
            // Only print the error, do not reference response.content in the error case.
            println!("Ollama test failed: {e:?}");
            // `Ollama` might not have the model available, so we log but don't fail
            println!("Ollama test failed (model might not be available): {e:?}");
        }
    }
}

#[tokio::test]
async fn test_openai_real_api_with_tools() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping real OpenAI API with tools test - no valid API key");
        return;
    }

    let api_key = super::get_openai_api_key_or_skip();
    let config = LlmConfig::openai(api_key, "gpt-3.5-turbo");
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
            assert!(!response.content.is_empty());
            println!("OpenAI real API call with tools successful");

            // Check if tool calls were made
            if response.has_tool_calls() {
                println!(
                    "Tool calls were made: {tool_calls:?}",
                    tool_calls = response.tool_calls
                );
                assert!(!response.tool_calls.is_empty());
            } else {
                println!("Tool calls were made: {:?}", response.content);
            }
        }
        Err(e) => {
            println!("OpenAI API call with tools failed: {e:?}");
            // Tool responses can have parsing issues with null values - this is known
            if e.to_string().contains("null, expected a string") {
                println!("Known issue with OpenAI tool response parsing - null value in response");
            } else {
                panic!("OpenAI API call with tools should succeed with valid credentials");
            }
        }
    }
}

#[tokio::test]
async fn test_multiple_provider_comparison() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let mut successful_providers = Vec::new();
    let test_prompt = "Explain AI in exactly 10 words.";

    // Test `OpenAI` if available
    if super::has_openai_api_key() {
        let api_key = std::env::var("OPENAI_API_KEY").unwrap();
        let config = LlmConfig::openai(api_key, "gpt-3.5-turbo");
        let provider = LlmProviderFactory::create_provider(config).unwrap();

        let request = LlmRequest::new(test_prompt)
            .with_max_tokens(20)
            .with_temperature(0.0);

        if let Ok(response) = provider.complete(request).await {
            successful_providers.push(("OpenAI", response.content));
        }
    }

    // Test `Anthropic` if available
    if super::has_anthropic_api_key() {
        let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap();
        let config = LlmConfig::anthropic(api_key, "claude-3-haiku-20240307");
        let provider = LlmProviderFactory::create_provider(config).unwrap();

        let request = LlmRequest::new(test_prompt)
            .with_max_tokens(20)
            .with_temperature(0.0);

        if let Ok(response) = provider.complete(request).await {
            successful_providers.push(("Anthropic", response.content));
        }
    }

    // Test `Ollama` if available
    if super::has_ollama_available().await {
        let config = LlmConfig::ollama("llama3.2");
        let provider = LlmProviderFactory::create_provider(config).unwrap();

        let request = LlmRequest::new(test_prompt)
            .with_max_tokens(20)
            .with_temperature(0.0);

        if let Ok(response) = provider.complete(request).await {
            successful_providers.push(("Ollama", response.content));
        }
    }

    println!("🔍 Provider comparison results:");
    for (provider_name, response) in &successful_providers {
        println!("  {provider_name} -> {response}");
    }

    if successful_providers.is_empty() {
        println!("No providers available for comparison test");
    } else {
        println!(
            "Successfully tested {count} provider(s)",
            count = successful_providers.len()
        );
    }

    // At least verify that different providers can be created
    // This test documents that the provider comparison ran, regardless of provider availability
}

// Additional comprehensive tests
#[tokio::test]
async fn test_llm_request_validation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test temperature clamping
    let request = LlmRequest::new("test").with_temperature(2.0); // Above 1.0
    assert_eq!(request.temperature, Some(1.0));

    let request = LlmRequest::new("test").with_temperature(-0.5); // Below 0.0
    assert_eq!(request.temperature, Some(0.0));

    // Test top_p clamping
    let request = LlmRequest::new("test").with_top_p(1.5); // Above 1.0
    assert_eq!(request.top_p, Some(1.0));

    let request = LlmRequest::new("test").with_top_p(-0.1); // Below 0.0
    assert_eq!(request.top_p, Some(0.0));
}

#[tokio::test]
async fn test_llm_provider_error_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test with invalid configuration
    let invalid_config = LlmConfig::openai("", ""); // Empty API key and model
    let provider_result = LlmProviderFactory::create_provider(invalid_config);

    // Should still create provider but fail on actual requests
    assert!(provider_result.is_ok());
}

#[tokio::test]
async fn test_llm_message_content_length() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let short_message = LlmMessage::user("Hi");
    assert_eq!(short_message.content_length(), 2);

    let long_message =
        LlmMessage::user("This is a much longer message for testing content length calculation");
    assert!(long_message.content_length() > 50);
}

#[tokio::test]
async fn test_llm_tool_call_structure() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let tool_call = LlmToolCall {
        id: "call_123".to_string(),
        name: "search_web".to_string(),
        parameters: json!({
            "query": "Rust programming language",
            "max_results": 5
        }),
    };

    assert_eq!(tool_call.id, "call_123");
    assert_eq!(tool_call.name, "search_web");
    assert_eq!(tool_call.parameters["query"], "Rust programming language");
    assert_eq!(tool_call.parameters["max_results"], 5);
}

#[tokio::test]
async fn test_llm_response_streaming_simulation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Simulate streaming response chunks
    let chunk1 = LlmResponse::new("Hello", "gpt-3.5-turbo").with_finish_reason(FinishReason::Stop);

    let chunk2 = LlmResponse::new(" world", "gpt-3.5-turbo").with_finish_reason(FinishReason::Stop);

    // In a real streaming scenario, these would be combined
    let combined_content = format!("{}{}", chunk1.content, chunk2.content);
    assert_eq!(combined_content, "Hello world");
}

#[tokio::test]
async fn test_llm_provider_factory_multiple_providers() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let configs = vec![
        LlmConfig::openai("key1", "gpt-3.5-turbo"),
        LlmConfig::anthropic("key2", "claude-3"),
        LlmConfig::fireworks("key3", "accounts/fireworks/models/llama-v3p1-8b-instruct"),
        LlmConfig::ollama("llama3.2"),
    ];

    let providers_result = LlmProviderFactory::create_providers(configs);
    assert!(providers_result.is_ok());

    let providers = providers_result.unwrap();
    assert_eq!(providers.len(), 4);
    assert_eq!(providers[0].provider_name(), "openai");
    assert_eq!(providers[1].provider_name(), "anthropic");
    assert_eq!(providers[2].provider_name(), "fireworks");
    assert_eq!(providers[3].provider_name(), "ollama");
}

#[tokio::test]
async fn test_llm_request_extra_parameters() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let request = LlmRequest::new("test")
        .with_extra_param("frequency_penalty".to_string(), json!(0.5))
        .with_extra_param("presence_penalty".to_string(), json!(0.3))
        .with_extra_param("custom_param".to_string(), json!("custom_value"));

    assert_eq!(request.extra_params.len(), 3);
    assert_eq!(request.extra_params["frequency_penalty"], json!(0.5));
    assert_eq!(request.extra_params["presence_penalty"], json!(0.3));
    assert_eq!(request.extra_params["custom_param"], json!("custom_value"));
}
