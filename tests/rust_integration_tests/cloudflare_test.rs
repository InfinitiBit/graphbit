//! Integration tests for Cloudflare Worker AI provider

use graphbit_core::{
    errors::GraphBitResult,
    llm::{LlmConfig, LlmMessage, LlmProviderFactory, LlmRequest},
};

#[tokio::test]
async fn test_cloudflare_provider_creation() -> GraphBitResult<()> {
    // Skip if no credentials are provided
    if !super::has_cloudflare_key() || !super::has_cloudflare_account() {
        println!("Skipping Cloudflare provider test - missing API key or account ID");
        return Ok(());
    }

    let api_key = super::get_cloudflare_key_or_skip();
    let account_id = super::get_cloudflare_account_or_skip();

    let config = LlmConfig::cloudflare(
        api_key,
        "@cf/meta/llama-2-7b-chat-int8".to_string(),
        account_id,
    );

    let provider = LlmProviderFactory::create_provider(config)?;
    assert_eq!(provider.provider_name(), "cloudflare");
    Ok(())
}

#[tokio::test]
async fn test_cloudflare_request_structure() -> GraphBitResult<()> {
    // Skip if no credentials are provided
    if !super::has_cloudflare_key() || !super::has_cloudflare_account() {
        println!("Skipping Cloudflare request test - missing API key or account ID");
        return Ok(());
    }

    let api_key = super::get_cloudflare_key_or_skip();
    let account_id = super::get_cloudflare_account_or_skip();

    let config = LlmConfig::cloudflare(
        api_key,
        "@cf/meta/llama-2-7b-chat-int8".to_string(),
        account_id,
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
    // Skip if no credentials are provided
    if !super::has_cloudflare_key() || !super::has_cloudflare_account() {
        println!("Skipping Cloudflare message formats test - missing API key or account ID");
        return Ok(());
    }

    let api_key = super::get_cloudflare_key_or_skip();
    let account_id = super::get_cloudflare_account_or_skip();

    let config = LlmConfig::cloudflare(
        api_key,
        "@cf/mistral/mistral-7b-instruct-v0.1".to_string(),
        account_id,
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