use graphbit_core::errors::GraphBitResult;
use graphbit_core::llm::cloudflare::CloudflareProvider;
use graphbit_core::llm::providers::LlmProviderTrait;
use graphbit_core::llm::{LlmMessage, LlmRequest, LlmRole};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use std::collections::HashMap;
use super::test_helpers::{has_cloudflare_key, has_cloudflare_account, get_cloudflare_key_or_skip, get_cloudflare_account_or_skip};

#[tokio::test(flavor = "multi_thread")]
async fn test_cloudflare_chat_completion() -> GraphBitResult<()> {    
    // Skip if no credentials are provided
    if !has_cloudflare_key() || !has_cloudflare_account() {
        println!("Skipping Cloudflare API test - missing API key or account ID");
        return Ok(());
    }

    let _api_key = get_cloudflare_key_or_skip();
    let account_id = get_cloudflare_account_or_skip();
    let model = "@cf/meta/llama-2-7b-chat-int8";
    let provider = CloudflareProvider::new(
        _api_key.to_string(),
        model.to_string(),
        account_id.to_string(),
    )?;

    let request = LlmRequest {
        messages: vec![LlmMessage {
            role: LlmRole::User,
            content: "Test message".to_string(),
            tool_calls: Vec::new(),
        }],
        temperature: Some(0.7),
        top_p: Some(1.0),
        max_tokens: Some(100),
        tools: Vec::new(),
        extra_params: HashMap::new(),
    };

    let response = provider.complete(request).await?;
    assert_eq!(response.content, "Test response");
    assert_eq!(response.id, Some("test_id".to_string()));
    assert_eq!(response.usage.prompt_tokens, 10);
    assert_eq!(response.usage.completion_tokens, 5);
    assert_eq!(response.usage.total_tokens, 15);

    sleep(Duration::from_millis(100)).await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_cloudflare_error_response() -> GraphBitResult<()> {    
    // Skip if no credentials are provided
    if !has_cloudflare_key() || !has_cloudflare_account() {
        println!("Skipping Cloudflare API test - missing API key or account ID");
        return Ok(());
    }
    let api_key = get_cloudflare_key_or_skip();
    let account_id = get_cloudflare_account_or_skip();
    let model = "@cf/meta/llama-2-7b-chat-int8";

    let provider = CloudflareProvider::new(
        api_key,
        model.to_string(),
        account_id.to_string(),
    )?;

    let request = LlmRequest {
        messages: vec![LlmMessage {
            role: LlmRole::User,
            content: "Test message".to_string(),
            tool_calls: Vec::new(),
        }],
        temperature: Some(0.7),
        top_p: Some(1.0),
        max_tokens: Some(100),
        tools: Vec::new(),
        extra_params: HashMap::new(),
    };

    let result = provider.complete(request).await;
    assert!(result.is_err());

    sleep(Duration::from_millis(100)).await;
    Ok(())
}