use graphbit_core::llm::response::{FinishReason, LlmResponse, LlmUsage};
use graphbit_core::llm::{LlmConfig, LlmMessage, LlmProviderFactory, LlmRequest, LlmRole};

// `DeepSeek` Provider Tests
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
    assert_eq!(provider.max_context_length(), Some(128_000));

    // Test cost per token
    let (input_cost, output_cost) = provider.cost_per_token().unwrap();
    assert_eq!(input_cost, 0.000_000_14);
    assert_eq!(output_cost, 0.000_000_28);
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

// `Fireworks AI` Provider Tests
#[tokio::test]
async fn test_fireworks_provider_creation() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::Fireworks {
        api_key: "test-key".to_string(),
        model: "accounts/fireworks/models/llama-v3p1-8b-instruct".to_string(),
        base_url: None,
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "fireworks");
    assert_eq!(
        provider.model_name(),
        "accounts/fireworks/models/llama-v3p1-8b-instruct"
    );
    assert!(provider.supports_function_calling());
    assert_eq!(provider.max_context_length(), Some(131_072));

    // Test cost per token
    let (input_cost, output_cost) = provider.cost_per_token().unwrap();
    assert_eq!(input_cost, 0.000_000_2);
    assert_eq!(output_cost, 0.000_000_2);
}

#[tokio::test]
async fn test_fireworks_model_configs() {
    let test_models = vec![
        (
            "accounts/fireworks/models/llama-v3p1-8b-instruct",
            Some(131_072),
            Some((0.000_000_2, 0.000_000_2)),
        ),
        (
            "accounts/fireworks/models/llama-v3p1-70b-instruct",
            Some(131_072),
            Some((0.000_000_9, 0.000_000_9)),
        ),
        (
            "accounts/fireworks/models/llama-v3p1-405b-instruct",
            Some(131_072),
            Some((0.000_003, 0.000_003)),
        ),
        (
            "accounts/fireworks/models/llama-v3-8b-instruct",
            Some(8192),
            None,
        ),
        (
            "accounts/fireworks/models/llama-v3-70b-instruct",
            Some(8192),
            None,
        ),
        (
            "accounts/fireworks/models/mixtral-8x7b-instruct",
            Some(32_768),
            Some((0.000_000_5, 0.000_000_5)),
        ),
        (
            "accounts/fireworks/models/mixtral-8x22b-instruct",
            Some(32_768),
            Some((0.000_000_9, 0.000_000_9)),
        ),
        ("unknown-model", None, None),
    ];

    for (model, context_length, cost) in test_models {
        let provider = LlmProviderFactory::create_provider(LlmConfig::Fireworks {
            api_key: "test-key".to_string(),
            model: model.to_string(),
            base_url: None,
        })
        .unwrap();

        assert_eq!(provider.max_context_length(), context_length);
        assert_eq!(provider.cost_per_token(), cost);
    }
}

#[tokio::test]
async fn test_fireworks_message_formatting() {
    let _provider = LlmProviderFactory::create_provider(LlmConfig::Fireworks {
        api_key: "test-key".to_string(),
        model: "accounts/fireworks/models/llama-v3p1-8b-instruct".to_string(),
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

#[tokio::test]
async fn test_fireworks_with_custom_base_url() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::Fireworks {
        api_key: "test-key".to_string(),
        model: "accounts/fireworks/models/llama-v3p1-8b-instruct".to_string(),
        base_url: Some("https://custom.fireworks.ai/v1".to_string()),
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "fireworks");
    assert_eq!(
        provider.model_name(),
        "accounts/fireworks/models/llama-v3p1-8b-instruct"
    );
}

// `xAI` Provider Tests
#[tokio::test]
async fn test_xai_provider_creation() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::Xai {
        api_key: "test-key".to_string(),
        model: "grok-4".to_string(),
        base_url: None,
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "xai");
    assert_eq!(provider.model_name(), "grok-4");
    assert!(provider.supports_function_calling());
    assert_eq!(provider.max_context_length(), Some(256_000));

    // Test cost per token
    let (input_cost, output_cost) = provider.cost_per_token().unwrap();
    assert_eq!(input_cost, 0.000_003);
    assert_eq!(output_cost, 0.000_015);
}

#[tokio::test]
async fn test_xai_model_configs() {
    let test_models = vec![
        ("grok-4", Some(256_000), Some((0.000_003, 0.000_015))),
        ("grok-4-0709", Some(256_000), Some((0.000_003, 0.000_015))),
        (
            "grok-code-fast-1",
            Some(256_000),
            Some((0.000_000_2, 0.000_001_5)),
        ),
        ("grok-3", Some(131_072), Some((0.000_003, 0.000_015))),
        (
            "grok-3-mini",
            Some(131_072),
            Some((0.000_000_3, 0.000_000_5)),
        ),
        (
            "grok-2-vision-1212",
            Some(32_768),
            Some((0.000_002, 0.000_010)),
        ),
        ("unknown-model", None, None),
    ];

    for (model, context_length, cost) in test_models {
        let provider = LlmProviderFactory::create_provider(LlmConfig::Xai {
            api_key: "test-key".to_string(),
            model: model.to_string(),
            base_url: None,
        })
        .unwrap();

        assert_eq!(provider.max_context_length(), context_length);
        assert_eq!(provider.cost_per_token(), cost);
    }
}

#[tokio::test]
async fn test_xai_message_formatting() {
    let _provider = LlmProviderFactory::create_provider(LlmConfig::Xai {
        api_key: "test-key".to_string(),
        model: "grok-4".to_string(),
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

#[tokio::test]
async fn test_xai_with_custom_base_url() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::Xai {
        api_key: "test-key".to_string(),
        model: "grok-4".to_string(),
        base_url: Some("https://custom.x.ai/v1".to_string()),
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "xai");
    assert_eq!(provider.model_name(), "grok-4");
}

// `Perplexity` Provider Tests
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
    assert_eq!(input_cost, 0.000_000_2);
    assert_eq!(output_cost, 0.000_000_2);
}

#[tokio::test]
async fn test_perplexity_model_configs() {
    let test_models = vec![
        ("pplx-7b-online", 4096, (0.000_000_2, 0.000_000_2)),
        ("pplx-70b-online", 4096, (0.000_001, 0.000_001)),
        ("pplx-7b-chat", 8192, (0.000_000_2, 0.000_000_2)),
        ("pplx-70b-chat", 8192, (0.000_001, 0.000_001)),
        ("llama-2-70b-chat", 4096, (0.000_001, 0.000_001)),
        (
            "codellama-34b-instruct",
            16384,
            (0.000_000_35, 0.000_001_40),
        ),
        ("mistral-7b-instruct", 16384, (0.000_000_2, 0.000_000_2)),
        ("sonar", 8192, (0.000_001, 0.000_001)),
        ("sonar-reasoning", 8192, (0.000_002, 0.000_002)),
        ("sonar-deep-research", 32768, (0.000_005, 0.000_005)),
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

// OpenRouter Provider Tests
#[tokio::test]
async fn test_openrouter_provider_creation() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::OpenRouter {
        api_key: "test-key".to_string(),
        model: "openai/gpt-4o-mini".to_string(),
        base_url: None,
        site_url: None,
        site_name: None,
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "openrouter");
    assert_eq!(provider.model_name(), "openai/gpt-4o-mini");
    assert!(provider.supports_function_calling());
    assert_eq!(provider.max_context_length(), Some(128000));

    // Test cost per token for OpenAI model through OpenRouter
    let (input_cost, output_cost) = provider.cost_per_token().unwrap();
    assert_eq!(input_cost, 0.00000015);
    assert_eq!(output_cost, 0.0000006);
}

#[tokio::test]
async fn test_openrouter_provider_with_site_info() {
    let provider = LlmProviderFactory::create_provider(LlmConfig::OpenRouter {
        api_key: "test-key".to_string(),
        model: "anthropic/claude-3-5-sonnet".to_string(),
        base_url: None,
        site_url: Some("https://example.com".to_string()),
        site_name: Some("Test App".to_string()),
    })
    .unwrap();

    assert_eq!(provider.provider_name(), "openrouter");
    assert_eq!(provider.model_name(), "anthropic/claude-3-5-sonnet");
    assert!(provider.supports_function_calling());
    assert_eq!(provider.max_context_length(), Some(200000));

    // Test cost per token for Claude model through OpenRouter
    let (input_cost, output_cost) = provider.cost_per_token().unwrap();
    assert_eq!(input_cost, 0.000003);
    assert_eq!(output_cost, 0.000015);
}

#[tokio::test]
async fn test_openrouter_message_formatting() {
    let _provider = LlmProviderFactory::create_provider(LlmConfig::OpenRouter {
        api_key: "test-key".to_string(),
        model: "openai/gpt-4o-mini".to_string(),
        base_url: None,
        site_url: None,
        site_name: None,
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
    assert_eq!(request.max_tokens, Some(100));
    assert_eq!(request.temperature, Some(0.7));
    assert_eq!(request.top_p, Some(0.9));
}

#[tokio::test]
async fn test_openrouter_config_helpers() {
    // Test basic config
    let config = LlmConfig::openrouter("test-key", "openai/gpt-4o-mini");
    match config {
        LlmConfig::OpenRouter {
            api_key,
            model,
            base_url,
            site_url,
            site_name,
        } => {
            assert_eq!(api_key, "test-key");
            assert_eq!(model, "openai/gpt-4o-mini");
            assert_eq!(base_url, None);
            assert_eq!(site_url, None);
            assert_eq!(site_name, None);
        }
        _ => panic!("Expected OpenRouter config"),
    }

    // Test config with site info
    let config = LlmConfig::openrouter_with_site(
        "test-key",
        "anthropic/claude-3-5-sonnet",
        Some("https://example.com".to_string()),
        Some("Test App".to_string()),
    );
    match config {
        LlmConfig::OpenRouter {
            api_key,
            model,
            base_url,
            site_url,
            site_name,
        } => {
            assert_eq!(api_key, "test-key");
            assert_eq!(model, "anthropic/claude-3-5-sonnet");
            assert_eq!(base_url, None);
            assert_eq!(site_url, Some("https://example.com".to_string()));
            assert_eq!(site_name, Some("Test App".to_string()));
        }
        _ => panic!("Expected OpenRouter config"),
    }
}

#[tokio::test]
async fn test_openrouter_model_context_lengths() {
    // Test various model context lengths
    let test_cases = vec![
        ("openai/gpt-4o", Some(128000)),
        ("openai/gpt-4o-mini", Some(128000)),
        ("anthropic/claude-3-5-sonnet", Some(200000)),
        ("google/gemini-pro-1.5", Some(1000000)),
        ("meta-llama/llama-3.1-405b-instruct", Some(131072)),
        ("unknown/model", Some(4096)), // Default fallback
    ];

    for (model, expected_context) in test_cases {
        let provider = LlmProviderFactory::create_provider(LlmConfig::OpenRouter {
            api_key: "test-key".to_string(),
            model: model.to_string(),
            base_url: None,
            site_url: None,
            site_name: None,
        })
        .unwrap();

        assert_eq!(provider.max_context_length(), expected_context);
    }
}
