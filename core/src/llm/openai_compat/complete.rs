use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::openai_compat::request::build_request_json_with_extra_params;
use crate::llm::LlmResponse;
use reqwest::{Client, RequestBuilder, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

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

/// Execute a non-stream chat completion request end-to-end.
pub(crate) async fn execute_complete_request<Req, Resp, FH, FP>(
    provider_name: &str,
    client: &Client,
    url: &str,
    api_key: &str,
    body: &Req,
    extra_params: HashMap<String, Value>,
    customize_builder: FH,
    parse_response: FP,
) -> GraphBitResult<LlmResponse>
where
    Req: Serialize,
    Resp: DeserializeOwned,
    FH: FnOnce(RequestBuilder) -> RequestBuilder,
    FP: FnOnce(Resp) -> GraphBitResult<LlmResponse>,
{
    let request_json = build_request_json_with_extra_params(provider_name, body, extra_params)?;
    let response = send_chat_completion_request(
        provider_name,
        client,
        url,
        api_key,
        &request_json,
        customize_builder,
    )
    .await?;

    let parsed: Resp = response.json().await.map_err(|e| {
        GraphBitError::llm_provider(provider_name, format!("Failed to parse response: {e}"))
    })?;

    parse_response(parsed)
}
