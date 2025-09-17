use super::test_helpers::*;
use graphbit_core::*;

#[tokio::test]
#[ignore = "Requires OpenAI API key"]
async fn test_openai_llm() {
    if !has_openai_key() {
        return;
    }

    let config = llm::LlmConfig::OpenAI {
        api_key: std::env::var("OPENAI_API_KEY").unwrap(),
        model: "gpt-3.5-turbo".to_string(),
        base_url: None,
        organization: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("What is 2+2?")
        .with_max_tokens(10)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(matches!(response.finish_reason, llm::FinishReason::Stop));
}

#[tokio::test]
#[ignore = "Tests OpenAI authentication failure"]
async fn test_openai_llm_failure() {
    let config = llm::LlmConfig::OpenAI {
        api_key: "invalid-key".to_string(),
        model: "gpt-3.5-turbo".to_string(),
        base_url: None,
        organization: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("test");
    let result = provider.complete(request).await;

    assert!(result.is_err());
}

#[tokio::test]
#[ignore = "Requires Anthropic API key"]
async fn test_anthropic_llm() {
    if !has_anthropic_key() {
        return;
    }

    let config = llm::LlmConfig::Anthropic {
        api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
        model: "claude-3-opus-20240229".to_string(),
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("What is 2+2?")
        .with_max_tokens(10)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(matches!(response.finish_reason, llm::FinishReason::Stop));
}

#[tokio::test]
#[ignore = "Tests Anthropic authentication failure"]
async fn test_anthropic_llm_failure() {
    let config = llm::LlmConfig::Anthropic {
        api_key: "invalid-key".to_string(),
        model: "claude-3-opus-20240229".to_string(),
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("test");
    let result = provider.complete(request).await;

    assert!(result.is_err());
}

#[tokio::test]
#[ignore = "Requires HuggingFace API key"]
async fn test_huggingface_llm() {
    if !has_huggingface_key() {
        return;
    }

    let config = llm::LlmConfig::HuggingFace {
        api_key: std::env::var("HUGGINGFACE_API_KEY").unwrap(),
        model: "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string(),
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("What is 2+2?")
        .with_max_tokens(10)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(matches!(response.finish_reason, llm::FinishReason::Stop));
}

#[tokio::test]
#[ignore = "Tests HuggingFace authentication failure"]
async fn test_huggingface_llm_failure() {
    let config = llm::LlmConfig::HuggingFace {
        api_key: "invalid-key".to_string(),
        model: "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string(),
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("test");
    let result = provider.complete(request).await;

    assert!(result.is_err());
}

#[tokio::test]
#[ignore = "Requires Perplexity API key"]
async fn test_perplexity_llm() {
    if !has_perplexity_key() {
        return;
    }

    let config = llm::LlmConfig::Perplexity {
        api_key: std::env::var("PERPLEXITY_API_KEY").unwrap(),
        model: "sonar".to_string(), // Updated to current valid model
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("What is 2+2?")
        .with_max_tokens(50) // Increased token limit
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    if let Err(ref e) = result {
        println!("Perplexity API error: {:?}", e);
    }
    assert!(result.is_ok(), "Perplexity API call failed: {:?}", result.err());

    let response = result.unwrap();
    println!("Perplexity response finish_reason: {:?}", response.finish_reason);
    // Be more flexible with finish reason - some APIs might return different reasons
    assert!(
        matches!(response.finish_reason, llm::FinishReason::Stop | llm::FinishReason::Length),
        "Unexpected finish reason: {:?}", response.finish_reason
    );
}

#[tokio::test]
#[ignore = "Tests Perplexity authentication failure"]
async fn test_perplexity_llm_failure() {
    let config = llm::LlmConfig::Perplexity {
        api_key: "invalid-key".to_string(),
        model: "pplx-7b-online".to_string(),
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("test");
    let result = provider.complete(request).await;

    assert!(result.is_err());
}

#[tokio::test]
#[ignore = "Requires DeepSeek API key"]
async fn test_deepseek_llm() {
    if !has_deepseek_key() {
        return;
    }

    let config = llm::LlmConfig::DeepSeek {
        api_key: std::env::var("DEEPSEEK_API_KEY").unwrap(),
        model: "deepseek-chat".to_string(),
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("What is 2+2?")
        .with_max_tokens(50) // Increased token limit
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    if let Err(ref e) = result {
        println!("DeepSeek API error: {:?}", e);
    }
    assert!(result.is_ok(), "DeepSeek API call failed: {:?}", result.err());

    let response = result.unwrap();
    println!("DeepSeek response finish_reason: {:?}", response.finish_reason);
    // Be more flexible with finish reason - some APIs might return different reasons
    assert!(
        matches!(response.finish_reason, llm::FinishReason::Stop | llm::FinishReason::Length),
        "Unexpected finish reason: {:?}", response.finish_reason
    );
}

#[tokio::test]
#[ignore = "Tests DeepSeek authentication failure"]
async fn test_deepseek_llm_failure() {
    let config = llm::LlmConfig::DeepSeek {
        api_key: "invalid-key".to_string(),
        model: "deepseek-chat".to_string(),
        base_url: None,
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("test");
    let result = provider.complete(request).await;

    assert!(result.is_err());
}

#[tokio::test]
#[ignore = "Requires Ollama"]
async fn test_ollama_llm() {
    if !is_ollama_available().await {
        return;
    }

    let config = llm::LlmConfig::Ollama {
        base_url: Some("http://localhost:11434".to_string()),
        model: "llama3.2:latest".to_string(),
    };

    let provider = llm::LlmProviderFactory::create_provider(config).unwrap();
    let request = llm::LlmRequest::new("What is 2+2?")
        .with_max_tokens(10)
        .with_temperature(0.0);

    let result = provider.complete(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(matches!(response.finish_reason, llm::FinishReason::Stop));
}
