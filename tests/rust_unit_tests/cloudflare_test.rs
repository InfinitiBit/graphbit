use graphbit::errors::GraphBitResult;
use graphbit::llm::providers::{CloudflareProvider, LlmProviderTrait};
use graphbit::llm::{LlmMessage, LlmRequest, LlmRole};
use mockito::mock;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_cloudflare_chat_completion() -> GraphBitResult<()> {
    let mut server = mockito::Server::new();
    
    // Skip if no credentials are provided
    if !super::has_cloudflare_key() || !super::has_cloudflare_account() {
        println!("Skipping Cloudflare API test - missing API key or account ID");
        return Ok(());
    }

    let api_key = super::get_cloudflare_key_or_skip();
    let account_id = super::get_cloudflare_account_or_skip();
    let model = "@cf/meta/llama-2-7b-chat-int8";
    let mock_url = format!("/client/v4/accounts/{}/ai/run/{}", account_id, model);

    let response_body = json!({
        "result": {
            "response": "Test response",
            "id": "test_id",
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "Test response"
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            }
        },
        "success": true,
        "errors": [],
        "messages": []
    });

    let _m = mock("POST", mock_url.as_str())
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response_body.to_string())
        .create_async()
        .await;

    let provider = CloudflareProvider::new(
        "test_key".to_string(),
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

#[tokio::test]
async fn test_cloudflare_error_response() -> GraphBitResult<()> {
    let mut server = mockito::Server::new();
    
    // Skip if no credentials are provided
    if !super::has_cloudflare_key() || !super::has_cloudflare_account() {
        println!("Skipping Cloudflare API test - missing API key or account ID");
        return Ok(());
    }

    let api_key = super::get_cloudflare_key_or_skip();
    let account_id = super::get_cloudflare_account_or_skip();
    let model = "@cf/meta/llama-2-7b-chat-int8";
    let mock_url = format!("/client/v4/accounts/{}/ai/run/{}", account_id, model);

    let response_body = json!({
        "result": null,
        "success": false,
        "errors": [{
            "code": 1000,
            "message": "Test error message"
        }],
        "messages": []
    });

    let _m = mock("POST", mock_url.as_str())
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(response_body.to_string())
        .create_async()
        .await;

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
    };

    let result = provider.complete(request).await;
    assert!(result.is_err());

    sleep(Duration::from_millis(100)).await;
    Ok(())
}