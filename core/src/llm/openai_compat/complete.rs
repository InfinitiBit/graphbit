use crate::errors::{GraphBitError, GraphBitResult};
use reqwest::{Client, RequestBuilder, Response};
use serde_json::Value;

/// Send a chat-completions request with shared auth/error handling.
pub(crate) async fn send_chat_completion_request<F>(
    provider_name: &str,
    client: &Client,
    url: &str,
    api_key: &str,
    request_json: &Value,
    customize_builder: F,
) -> GraphBitResult<Response>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    let request_builder = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(request_json);

    let response = customize_builder(request_builder)
        .send()
        .await
        .map_err(|e| GraphBitError::llm_provider(provider_name, format!("Request failed: {e}")))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(GraphBitError::llm_provider(
            provider_name,
            format!("API error: {error_text}"),
        ));
    }

    Ok(response)
}
