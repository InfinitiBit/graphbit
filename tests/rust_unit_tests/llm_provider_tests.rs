use graphbit_core::llm::response::{FinishReason, LlmResponse, LlmUsage};
use graphbit_core::llm::{LlmConfig, LlmMessage, LlmProviderFactory, LlmRequest, LlmRole};

// DeepSeek Provider Tests
#[tokio::test]
async fn test_deepseek_provider_creation() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::DeepSeek {
        api_key: "test-key".to_string(),
        model: "deepseek-chat".to_string(),
        base_url: None,
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "deepseek");
    assert_eq!(provider.model_name(), "deepseek-chat");
    assert!(provider.supports_function_calling());
    assert_eq!(provider.max_context_length(), Some(128000));

    // Test cost per token
    let (input_cost, output_cost) = provider.cost_per_token().unwrap();
    assert_eq!(input_cost, 0.00000014);
    assert_eq!(output_cost, 0.00000028);
}

#[tokio::test]
async fn test_deepseek_message_formatting() {
    let _provider = LlmProviderFactory::create_provider(LlmConfig::DeepSeek {
        api_key: "test-key".to_string(),
        model: "deepseek-chat".to_string(),
        base_url: None,
    })
    .unwrap();

    let request = LlmRequest::with_messages(vec![])
        .with_message(LlmMessage::system("system prompt"))
        .with_message(LlmMessage::user("user message"))
        .with_message(LlmMessage::assistant("assistant message"))
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
}

// Tool-calling tests removed per request

// Perplexity Provider Tests
#[tokio::test]
async fn test_perplexity_provider_creation() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::Perplexity {
        api_key: "test-key".to_string(),
        model: "pplx-7b-online".to_string(),
        base_url: None,
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "perplexity");
    assert_eq!(provider.model_name(), "pplx-7b-online");
    assert!(provider.supports_function_calling());
    assert_eq!(provider.max_context_length(), Some(4096));

    // Test cost per token
    let (input_cost, output_cost) = provider.cost_per_token().unwrap();
    assert_eq!(input_cost, 0.0000002);
    assert_eq!(output_cost, 0.0000002);
}

#[tokio::test]
async fn test_perplexity_model_configs() {
    let test_models = vec![
        ("pplx-7b-online", 4096, (0.0000002, 0.0000002)),
        ("pplx-70b-online", 4096, (0.000001, 0.000001)),
        ("pplx-7b-chat", 8192, (0.0000002, 0.0000002)),
        ("pplx-70b-chat", 8192, (0.000001, 0.000001)),
        ("llama-2-70b-chat", 4096, (0.000001, 0.000001)),
        ("codellama-34b-instruct", 16384, (0.00000035, 0.00000140)),
        ("mistral-7b-instruct", 16384, (0.0000002, 0.0000002)),
        ("sonar", 8192, (0.000001, 0.000001)),
        ("sonar-reasoning", 8192, (0.000002, 0.000002)),
        ("sonar-deep-research", 32768, (0.000005, 0.000005)),
    ];

    for (model, context_length, (input_cost, output_cost)) in test_models {
        let provider = LlmProviderFactory::create_provider(LlmConfig::Perplexity {
            api_key: "test-key".to_string(),
            model: model.to_string(),
            base_url: None,
        })
        .unwrap();

        assert_eq!(provider.max_context_length(), Some(context_length));
        let (actual_input, actual_output) = provider.cost_per_token().unwrap();
        assert_eq!(actual_input, input_cost);
        assert_eq!(actual_output, output_cost);
    }
}

#[tokio::test]
async fn test_perplexity_message_formatting() {
    let _provider = LlmProviderFactory::create_provider(LlmConfig::Perplexity {
        api_key: "test-key".to_string(),
        model: "pplx-7b-online".to_string(),
        base_url: None,
    })
    .unwrap();

    let request = LlmRequest::with_messages(vec![])
        .with_message(LlmMessage::system("system prompt"))
        .with_message(LlmMessage::user("user message"))
        .with_message(LlmMessage::assistant("assistant message"))
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
}

// Provider Factory Tests
#[tokio::test]
async fn test_provider_factory_error_handling() {
    // Test invalid model names
    let config = LlmConfig::DeepSeek {
        api_key: "test-key".to_string(),
        model: "invalid-model".to_string(),
        base_url: None,
    };
    let provider = LlmProviderFactory::create_provider(config).unwrap();
    assert!(provider.cost_per_token().is_none());
    assert!(provider.max_context_length().is_none());

    // Test invalid base URLs
    let config = LlmConfig::Perplexity {
        api_key: "test-key".to_string(),
        model: "pplx-7b-online".to_string(),
        base_url: Some("invalid-url".to_string()),
    };
    let provider = LlmProviderFactory::create_provider(config).unwrap();
    let request = LlmRequest::new("test");
    let result = provider.complete(request).await;
    assert!(result.is_err());
}

// LlmResponse utilities coverage
#[test]
fn test_llm_response_utilities() {
    let usage = LlmUsage::new(10, 5);
    let resp = LlmResponse::new("content", "model").with_usage(usage);
    assert_eq!(resp.total_tokens(), 15);
    assert!(!resp.has_tool_calls());
    assert!(!resp.is_truncated());
    assert!(FinishReason::Stop.is_natural_stop());
    assert!(FinishReason::Length.is_truncated());
    assert!(FinishReason::Error.is_error());
    // cost estimation path
    let cost = resp.estimate_cost(0.1, 0.2);
    assert!(cost > 0.0);
}

#[test]
fn test_llm_request_estimated_tokens_and_message_length() {
    let m1 = LlmMessage::user("abcdabcd");
    let m2 = LlmMessage::assistant("abcd");
    assert!(m1.content_length() > 0);
    let req = LlmRequest::with_messages(vec![m1, m2]);
    // Very rough estimate, just assert it's non-zero
    assert!(req.estimated_token_count() > 0);
}

// Common Provider Tests
#[tokio::test]
async fn test_provider_common_functionality() {
    let providers = vec![
        LlmConfig::DeepSeek {
            api_key: "test-key".to_string(),
            model: "deepseek-chat".to_string(),
            base_url: None,
        },
        LlmConfig::Perplexity {
            api_key: "test-key".to_string(),
            model: "pplx-7b-online".to_string(),
            base_url: None,
        },
    ];

    for config in providers {
        let provider = LlmProviderFactory::create_provider(config).unwrap();

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
}
