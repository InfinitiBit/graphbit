//! `xAI` LLM provider implementation for Grok models

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::openai_compat::complete::execute_complete_request;
use crate::llm::openai_compat::finish_reason::parse_openai_finish_reason;
use crate::llm::openai_compat::http::build_http_client;
use crate::llm::openai_compat::request::build_request_json_with_extra_params;
use crate::llm::openai_compat::response::{
    TOOL_ONLY_FALLBACK_TEXT, fallback_content_if_tool_only, first_choice_or_error, has_tool_calls,
    parse_tool_arguments_openai_style, usage_from_prompt_completion,
};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall};
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

/// `xAI` API provider for Grok models
pub struct XaiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl XaiProvider {
    /// Create a new `xAI` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = build_http_client("xai", None)?;
        let base_url = "https://api.x.ai/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `xAI` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = build_http_client("xai", None)?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert `GraphBit` message to `xAI` message format
    fn convert_message(message: &LlmMessage) -> XaiMessage {
        XaiMessage {
            role: match message.role {
                LlmRole::User => "user".to_string(),
                LlmRole::Assistant => "assistant".to_string(),
                LlmRole::System => "system".to_string(),
                LlmRole::Tool => "tool".to_string(),
            },
            content: Some(message.content.clone()),
            tool_calls: if message.tool_calls.is_empty() {
                None
            } else {
                Some(
                    message
                        .tool_calls
                        .iter()
                        .map(|tc| XaiToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: XaiFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
            tool_call_id: message.tool_call_id.clone(),
        }
    }

    /// Convert `GraphBit` tool to `xAI` tool format
    fn convert_tool(tool: &LlmTool) -> XaiTool {
        XaiTool {
            r#type: "function".to_string(),
            function: XaiFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `xAI` response to `GraphBit` response
    fn parse_response(&self, response: XaiResponse) -> GraphBitResult<LlmResponse> {
        let choice = first_choice_or_error("xai", response.choices)?;
        let content = fallback_content_if_tool_only(
            choice.message.content.unwrap_or_default(),
            has_tool_calls(choice.message.tool_calls.as_ref()),
            TOOL_ONLY_FALLBACK_TEXT,
        );

        let tool_calls: Vec<LlmToolCall> = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| LlmToolCall {
                id: tc.id,
                name: tc.function.name.clone(),
                parameters: parse_tool_arguments_openai_style(&tc.function.name, &tc.function.arguments),
            })
            .collect();

        let finish_reason = parse_openai_finish_reason(choice.finish_reason.as_deref());

        let usage =
            usage_from_prompt_completion(response.usage.prompt_tokens, response.usage.completion_tokens);

        Ok(LlmResponse::new(content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response.id))
    }
}

#[async_trait]
impl LlmProviderTrait for XaiProvider {
    fn provider_name(&self) -> &str {
        "xai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<XaiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<XaiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = XaiRequest {
            model: self.model.clone(),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: tools.clone(),
            tool_choice: if tools.is_some() {
                Some("auto".to_string())
            } else {
                None
            },
        };

        execute_complete_request(
            "xai",
            &self.client,
            &url,
            &self.api_key,
            &body,
            request.extra_params,
            |rb| rb,
            |xai_response: XaiResponse| self.parse_response(xai_response),
        )
        .await
    }

    fn supports_function_calling(&self) -> bool {
        // Most Grok models support function calling
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // Context lengths for Grok models based on xAI documentation
        match self.model.as_str() {
            "grok-4" | "grok-4-0709" => Some(256_000),
            "grok-code-fast-1" => Some(256_000),
            "grok-3" => Some(131_072),
            "grok-3-mini" => Some(131_072),
            "grok-2-vision-1212" => Some(32_768),
            _ => None, // Unknown model, let the API handle it
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Costs per token in USD (input, output) for Grok models
        // Based on xAI pricing documentation
        match self.model.as_str() {
            "grok-4" | "grok-4-0709" => Some((0.000_003, 0.000_015)),
            "grok-code-fast-1" => Some((0.000_000_2, 0.000_001_5)),
            "grok-3" => Some((0.000_003, 0.000_015)),
            "grok-3-mini" => Some((0.000_000_3, 0.000_000_5)),
            "grok-2-vision-1212" => Some((0.000_002, 0.000_010)),
            _ => None, // Unknown model pricing
        }
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<XaiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<XaiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = XaiStreamRequest {
            model: self.model.clone(),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: tools.clone(),
            tool_choice: if tools.is_some() {
                Some("auto".to_string())
            } else {
                None
            },
            stream: Some(true), // Enable streaming
        };

        let request_json = build_request_json_with_extra_params("xai", &body, request.extra_params)?;

        // Timeout constants for different phases of the request
        const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);
        const ERROR_BODY_TIMEOUT: Duration = Duration::from_secs(10);
        const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);

        // Apply timeout to initial connection
        let response = timeout(
            CONNECTION_TIMEOUT,
            self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request_json)
                .send(),
        )
        .await
        .map_err(|_| {
            GraphBitError::llm_provider(
                "xai",
                format!(
                    "Connection timeout after {:?} - xAI did not respond. \
                     Check network connectivity and xAI status.",
                    CONNECTION_TIMEOUT
                ),
            )
        })?
        .map_err(|e| GraphBitError::llm_provider("xai", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = timeout(ERROR_BODY_TIMEOUT, response.text())
                .await
                .unwrap_or_else(|_| {
                    Ok(format!(
                        "Error body read timeout after {:?}",
                        ERROR_BODY_TIMEOUT
                    ))
                })
                .unwrap_or_else(|_| "Unknown error (failed to read body)".to_string());

            return Err(GraphBitError::llm_provider(
                "xai",
                format!("API error: {error_text}"),
            ));
        }

        // Parse SSE stream with proper line buffering and per-chunk timeout
        let model = self.model.clone();
        let byte_stream = response.bytes_stream();

        // State: (byte_stream, buffer, timeout_occurred, consecutive_parse_errors, total_parse_errors)
        const MAX_CONSECUTIVE_PARSE_ERRORS: u32 = 5;

        let stream = futures::stream::unfold(
            (byte_stream, String::new(), false, 0u32, 0u32),
            move |(
                mut byte_stream,
                mut buffer,
                timeout_occurred,
                mut consecutive_parse_errors,
                mut total_parse_errors,
            )| {
                let model = model.clone();
                async move {
                    // If we already had a timeout, don't continue
                    if timeout_occurred {
                        return None;
                    }

                    loop {
                        // Process already-buffered complete lines before waiting for more bytes.
                        // A single network chunk can contain multiple SSE events; if we return
                        // after yielding one token chunk, remaining complete lines must be
                        // drained first on the next poll or they can be lost at EOF.
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            // Skip empty lines and comments
                            if line.is_empty() || line.starts_with(':') {
                                continue;
                            }

                            if let Some(data) = line.strip_prefix("data: ") {
                                if data.trim() == "[DONE]" {
                                    if total_parse_errors > 0 {
                                        tracing::warn!(
                                            "Stream completed with {} total parse errors. Some data may have been lost.",
                                            total_parse_errors
                                        );
                                    }
                                    return None;
                                }

                                match serde_json::from_str::<XaiStreamChunk>(data) {
                                    Ok(stream_chunk) => {
                                        consecutive_parse_errors = 0;

                                        if let Some(choice) = stream_chunk.choices.first()
                                            && let Some(content) = &choice.delta.content
                                            && !content.is_empty()
                                        {
                                            let response = LlmResponse::new(content.clone(), &model)
                                                .with_id(stream_chunk.id);
                                            return Some((
                                                Ok(response),
                                                (
                                                    byte_stream,
                                                    buffer,
                                                    false,
                                                    consecutive_parse_errors,
                                                    total_parse_errors,
                                                ),
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        consecutive_parse_errors += 1;
                                        total_parse_errors += 1;

                                        tracing::warn!(
                                            "Failed to parse xAI stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "xai",
                                                    format!(
                                                        "Stream corrupted: {} consecutive parse errors. \
                                                         Last error: {}. Data may be incomplete.",
                                                        consecutive_parse_errors, e
                                                    ),
                                                )),
                                                (
                                                    byte_stream,
                                                    buffer,
                                                    true,
                                                    consecutive_parse_errors,
                                                    total_parse_errors,
                                                ),
                                            ));
                                        }
                                    }
                                }
                            }
                        }

                        // Apply timeout to each chunk read
                        let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                            Ok(Some(result)) => result,
                            Ok(None) => {
                                // Try to parse a final buffered line without trailing newline.
                                let tail_line = buffer.trim().to_string();
                                if let Some(data) = tail_line.strip_prefix("data: ") {
                                    if data.trim() != "[DONE]"
                                        && let Ok(stream_chunk) =
                                            serde_json::from_str::<XaiStreamChunk>(data)
                                        && let Some(choice) = stream_chunk.choices.first()
                                        && let Some(content) = &choice.delta.content
                                        && !content.is_empty()
                                    {
                                        let response = LlmResponse::new(content.clone(), &model)
                                            .with_id(stream_chunk.id);
                                        return Some((
                                            Ok(response),
                                            (
                                                byte_stream,
                                                String::new(),
                                                false,
                                                consecutive_parse_errors,
                                                total_parse_errors,
                                            ),
                                        ));
                                    }
                                }

                                // Stream naturally ended
                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "Stream ended with {} total parse errors. Some data may have been lost.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }
                            Err(_) => {
                                // Timeout occurred
                                tracing::warn!(
                                    "Stream chunk timeout after {:?} - xAI stopped responding. \
                                     Response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "xai",
                                        format!(
                                            "Stream timeout after {:?} - response may be incomplete",
                                            CHUNK_TIMEOUT
                                        ),
                                    )),
                                    (
                                        byte_stream,
                                        buffer,
                                        true,
                                        consecutive_parse_errors,
                                        total_parse_errors,
                                    ),
                                ));
                            }
                        };

                        let chunk = match chunk_result {
                            Ok(c) => c,
                            Err(e) => {
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "xai",
                                        format!("Stream error: {e}"),
                                    )),
                                    (
                                        byte_stream,
                                        buffer,
                                        false,
                                        consecutive_parse_errors,
                                        total_parse_errors,
                                    ),
                                ));
                            }
                        };

                        // Append new data to buffer
                        buffer.push_str(&String::from_utf8_lossy(&chunk));
                    }
                }
            },
        );

        Ok(Box::new(Box::pin(stream)))
    }

    fn supports_streaming(&self) -> bool {
        true // xAI supports streaming via OpenAI-compatible API
    }
}

// `xAI` API types
#[derive(Debug, Serialize)]
struct XaiRequest {
    model: String,
    messages: Vec<XaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<XaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiMessage {
    role: String,
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<XaiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiToolCall {
    id: String,
    r#type: String,
    function: XaiFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct XaiTool {
    r#type: String,
    function: XaiFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct XaiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct XaiResponse {
    id: String,
    choices: Vec<XaiChoice>,
    usage: XaiUsage,
}

#[derive(Debug, Deserialize)]
struct XaiChoice {
    message: XaiMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct XaiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

// Streaming-specific types (OpenAI-compatible format)

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct XaiStreamRequest {
    model: String,
    messages: Vec<XaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<XaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Streaming chunk from xAI API (OpenAI-compatible format)
#[derive(Debug, Deserialize)]
struct XaiStreamChunk {
    id: String,
    choices: Vec<XaiStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct XaiStreamChoice {
    delta: XaiDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct XaiDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
}
