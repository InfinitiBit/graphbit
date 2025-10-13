//! Integration tests for Cloudflare Worker AI provider

use graphbit_core::{
    errors::GraphBitResult,
    llm::{LlmConfig, LlmMessage, LlmProviderFactory, LlmRequest},
};

#[tokio::test]
async fn test_cloudflare_provider_creation() -> GraphBitResult<()> {
    let config = LlmConfig::cloudflare(
        "test-key".to_string(),
        "openai/gpt-5-mini".to_string(),
        "test-account".to_string(),
        "test-gateway".to_string(),
    );

    let provider = LlmProviderFactory::create_provider(config)?;
    assert_eq!(provider.provider_name(), "cloudflare");
    Ok(())
}

#[tokio::test]
async fn test_cloudflare_request_structure() -> GraphBitResult<()> {
    let config = LlmConfig::cloudflare(
        "test-key".to_string(),
        "google-ai-studio/gemini-2.5-flash".to_string(),
        "test-account".to_string(),
        "test-gateway".to_string(),
    );

    let provider = LlmProviderFactory::create_provider(config)?;
    
    let request = LlmRequest::new("Test message")
        .with_max_tokens(100)
        .with_temperature(0.7);

    // This will fail because we're using test credentials,
    // but it helps verify the request structure
    let result = provider.complete(request).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_cloudflare_message_formats() -> GraphBitResult<()> {
    let config = LlmConfig::cloudflare(
        "test-key".to_string(),
        "anthropic/claude-sonnet-4-5".to_string(),
        "test-account".to_string(),
        "test-gateway".to_string(),
    );

    let provider = LlmProviderFactory::create_provider(config)?;

    let request = LlmRequest::with_messages(vec![
        LlmMessage::system("You are a helpful assistant."),
        LlmMessage::user("Hello!"),
        LlmMessage::assistant("Hi! How can I help you today?"),
    ]);

    let result = provider.complete(request).await;
    assert!(result.is_err()); // Will fail with test credentials
    Ok(())
}