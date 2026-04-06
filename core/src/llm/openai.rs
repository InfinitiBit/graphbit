//! `OpenAI` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;

/// `OpenAI` API provider
pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
    organization: Option<String>,
}

impl OpenAiProvider {
    /// Create a new `OpenAI` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10) // Increased connection pool size
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("openai", format!("Failed to create HTTP client: {e}"))
            })?;
        let base_url = "https://api.openai.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Create a new `OpenAI` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        // Use same optimized client settings
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("openai", format!("Failed to create HTTP client: {e}"))
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Set organization ID
    pub fn with_organization(mut self, organization: String) -> Self {
        self.organization = Some(organization);
        self
    }

    /// Convert `GraphBit` message to `OpenAI` message format
    fn convert_message(message: &LlmMessage) -> OpenAiMessage {
        OpenAiMessage {
            role: match message.role {
                LlmRole::User => "user".to_string(),
                LlmRole::Assistant => "assistant".to_string(),
                LlmRole::System => "system".to_string(),
                LlmRole::Tool => "tool".to_string(),
            },
            content: message.content.clone(),
            tool_calls: if message.tool_calls.is_empty() {
                None
            } else {
                Some(
                    message
                        .tool_calls
                        .iter()
                        .map(|tc| OpenAiToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: OpenAiFunction {
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

    /// Convert `GraphBit` tool to `OpenAI` tool format
    fn convert_tool(tool: &LlmTool) -> OpenAiTool {
        OpenAiTool {
            r#type: "function".to_string(),
            function: OpenAiFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `OpenAI` response to `GraphBit` response
    fn parse_response(&self, response: OpenAiResponse) -> GraphBitResult<LlmResponse> {
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| GraphBitError::llm_provider("openai", "No choices in response"))?;

        let mut content = choice.message.content;
        if content.trim().is_empty()
            && !choice
                .message
                .tool_calls
                .as_ref()
                .unwrap_or(&vec![])
                .is_empty()
        {
            content = "I'll help you with that using the available tools.".to_string();
        }
        let tool_calls = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| {
                // Production-grade argument parsing with error handling
                let parameters = if tc.function.arguments.trim().is_empty() {
                    serde_json::Value::Object(serde_json::Map::new())
                } else {
                    match serde_json::from_str(&tc.function.arguments) {
                        Ok(params) => params,
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse tool call arguments for {}: {e}. Arguments: '{}'",
                                tc.function.name,
                                tc.function.arguments
                            );
                            // Try to create a simple object with the raw arguments
                            serde_json::json!({ "raw_arguments": tc.function.arguments })
                        }
                    }
                };

                LlmToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    parameters,
                }
            })
            .collect();

        let finish_reason = parse_finish_reason(choice.finish_reason.as_deref());

        let usage = LlmUsage::new(
            response.usage.prompt_tokens,
            response.usage.completion_tokens,
        );

        Ok(LlmResponse::new(content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response.id))
    }
}

#[async_trait]
impl LlmProviderTrait for OpenAiProvider {
    fn provider_name(&self) -> &str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenAiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OpenAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = OpenAiRequest {
            model: self.model.clone(),
            messages,
            max_completion_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: tools.clone(),
            tool_choice: if tools.is_some() {
                Some("auto".to_string())
            } else {
                None
            },
            stream: None, // Disable streaming for complete method
            stream_options: None,
        };

        // Add extra parameters
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

        let mut req_builder = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_json);

        if let Some(org) = &self.organization {
            req_builder = req_builder.header("OpenAI-Organization", org);
        }

        let response = req_builder
            .send()
            .await
            .map_err(|e| GraphBitError::llm_provider("openai", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "openai",
                format!("API error: {error_text}"),
            ));
        }

        let openai_response: OpenAiResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("openai", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(openai_response)
    }

    fn supports_function_calling(&self) -> bool {
        // Most `OpenAI` models support function calling
        matches!(
            self.model.as_str(),
            "gpt-4" | "gpt-4-turbo" | "gpt-3.5-turbo" | "gpt-4o" | "gpt-4o-mini"
        ) || self.model.starts_with("gpt-4")
            || self.model.starts_with("gpt-3.5-turbo")
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenAiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OpenAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = OpenAiRequest {
            model: self.model.clone(),
            messages,
            max_completion_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: tools.clone(),
            tool_choice: if tools.is_some() {
                Some("auto".to_string())
            } else {
                None
            },
            stream: Some(true), // Enable streaming
            // Request terminal usage chunk so workflow metadata token usage matches non-streaming.
            stream_options: Some(OpenAiStreamOptions {
                include_usage: true,
            }),
        };

        // Add extra parameters
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

        let mut req_builder = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_json);

        if let Some(org) = &self.organization {
            req_builder = req_builder.header("OpenAI-Organization", org);
        }

        // Timeout constants for different phases of the request
        // CONNECTION_TIMEOUT: Covers DNS resolution, TCP connection, TLS handshake, and first byte
        // Should be generous for slow networks but prevent infinite hangs
        const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);
        // ERROR_BODY_TIMEOUT: Time to read error response body (usually small)
        const ERROR_BODY_TIMEOUT: Duration = Duration::from_secs(10);
        // CHUNK_TIMEOUT: Maximum time between streaming chunks
        // If OpenAI stops sending chunks for this long, we timeout
        const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);

        // Apply timeout to initial connection and first response byte
        // This prevents hanging forever if OpenAI is unreachable or slow
        let response = timeout(CONNECTION_TIMEOUT, req_builder.send())
            .await
            .map_err(|_| {
                GraphBitError::llm_provider(
                    "openai",
                    format!(
                        "Connection timeout after {:?} - OpenAI did not respond. \
                         Check network connectivity and OpenAI status.",
                        CONNECTION_TIMEOUT
                    ),
                )
            })?
            .map_err(|e| GraphBitError::llm_provider("openai", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            // Apply timeout to reading error body to prevent hanging on malformed responses
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
                "openai",
                format!("API error: {error_text}"),
            ));
        }

        // Parse SSE stream with proper line buffering and per-chunk timeout
        // This prevents hanging forever if OpenAI stops responding mid-stream
        let model = self.model.clone();
        let byte_stream = response.bytes_stream();

        // State: byte_stream, buffer, timeout / parse error counters, and accumulated tool-call
        // fragments from `delta.tool_calls` (required for streaming + function calling).
        // - consecutive_parse_errors: Resets on successful parse, triggers error if too high
        // - total_parse_errors: Running count for logging
        const MAX_CONSECUTIVE_PARSE_ERRORS: u32 = 5;

        // Use a stateful stream that buffers incomplete lines with timeout protection
        let stream = futures::stream::unfold(
            (
                byte_stream,
                String::new(),
                false,
                0u32,
                0u32,
                HashMap::<u32, OpenAiStreamToolAccum>::new(),
            ),
            move |(
                mut byte_stream,
                mut buffer,
                timeout_occurred,
                mut consecutive_parse_errors,
                mut total_parse_errors,
                mut tool_call_accum,
            )| {
                let model = model.clone();
                async move {
                    // If we already had a timeout, don't continue
                    if timeout_occurred {
                        return None;
                    }

                    loop {
                        // If the buffer has no complete line yet, read more bytes.
                        // This ensures we don't block waiting for network data while
                        // already-buffered SSE lines (e.g. terminal usage chunk) are pending.
                        if buffer.find('\n').is_none() {
                            // Apply timeout to each chunk read to prevent indefinite hanging
                            let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next())
                                .await
                            {
                                Ok(Some(result)) => result,
                                Ok(None) => {
                                    // Stream naturally ended
                                    // Log if there were parse errors during the stream
                                    if total_parse_errors > 0 {
                                        tracing::warn!(
                                            "Stream ended with {} total parse errors. Some data may have been lost.",
                                            total_parse_errors
                                        );
                                    }
                                    return None;
                                }
                                Err(_) => {
                                    // Timeout occurred - OpenAI stopped responding
                                    // Return an error so user knows the response may be incomplete
                                    tracing::warn!(
                                        "Stream chunk timeout after {:?} - OpenAI stopped responding. \
                                             Response may be incomplete.",
                                        CHUNK_TIMEOUT
                                    );
                                    // Return error to notify user, then end stream
                                    return Some((
                                        Err(GraphBitError::llm_provider(
                                            "openai",
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
                                            tool_call_accum,
                                        ),
                                    ));
                                }
                            };

                            let chunk = match chunk_result {
                                Ok(c) => c,
                                Err(e) => {
                                    return Some((
                                        Err(GraphBitError::llm_provider(
                                            "openai",
                                            format!("Stream error: {e}"),
                                        )),
                                        (
                                            byte_stream,
                                            buffer,
                                            false,
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            tool_call_accum,
                                        ),
                                    ));
                                }
                            };

                            // Append new data to buffer
                            buffer.push_str(&String::from_utf8_lossy(&chunk));
                        }

                        // Process complete lines using drain() to avoid allocations
                        while let Some(newline_pos) = buffer.find('\n') {
                            // Extract the line without allocating a new String
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            // Skip empty lines and comments
                            if line.is_empty() || line.starts_with(':') {
                                continue;
                            }

                            // Check for data: prefix
                            if let Some(data) = line.strip_prefix("data: ") {
                                // Check for [DONE] marker
                                if data.trim() == "[DONE]" {
                                    // Log if there were parse errors during the stream
                                    if total_parse_errors > 0 {
                                        tracing::warn!(
                                            "Stream completed with {} total parse errors. Some data may have been lost.",
                                            total_parse_errors
                                        );
                                    }
                                    return None; // End of stream
                                }

                                // Parse JSON chunk
                                match serde_json::from_str::<OpenAiStreamChunk>(data) {
                                    Ok(stream_chunk) => {
                                        // Reset consecutive error counter on success
                                        consecutive_parse_errors = 0;

                                        let OpenAiStreamChunk { id, choices, usage } = stream_chunk;
                                        if let Some(choice) = choices.first() {
                                            merge_openai_stream_tool_deltas(
                                                &mut tool_call_accum,
                                                &choice.delta.tool_calls,
                                            );
                                        }
                                        let streamed_tool_calls =
                                            tool_accum_map_to_llm_calls(&tool_call_accum);
                                        let usage = usage.map(|u| {
                                            LlmUsage::new(u.prompt_tokens, u.completion_tokens)
                                        });
                                        let finish_reason = choices
                                            .first()
                                            .and_then(|c| c.finish_reason.as_deref())
                                            .map(|reason| parse_finish_reason(Some(reason)));

                                        if let Some(choice) = choices.first() {
                                            if let Some(content) = &choice.delta.content {
                                                if !content.is_empty() {
                                                    let mut response =
                                                        LlmResponse::new(content.clone(), &model)
                                                            .with_id(id.clone())
                                                            .with_tool_calls(
                                                                streamed_tool_calls.clone(),
                                                            );
                                                    if let Some(usage) = usage.clone() {
                                                        response = response.with_usage(usage);
                                                    }
                                                    if let Some(finish_reason) =
                                                        finish_reason.clone()
                                                    {
                                                        response = response
                                                            .with_finish_reason(finish_reason);
                                                    }
                                                    return Some((
                                                        Ok(response),
                                                        (
                                                            byte_stream,
                                                            buffer,
                                                            false,
                                                            consecutive_parse_errors,
                                                            total_parse_errors,
                                                            tool_call_accum,
                                                        ),
                                                    ));
                                                }
                                            }
                                        }

                                        // Terminal usage chunks are often emitted without content.
                                        // Surface them so caller can preserve usage in final metadata.
                                        if usage.is_some() || finish_reason.is_some() {
                                            let text = stream_assistant_text_for_tool_calls(
                                                String::new(),
                                                &streamed_tool_calls,
                                            );
                                            let mut response = LlmResponse::new(text, &model)
                                                .with_id(id)
                                                .with_tool_calls(streamed_tool_calls);
                                            if let Some(usage) = usage {
                                                response = response.with_usage(usage);
                                            }
                                            if let Some(finish_reason) = finish_reason {
                                                response =
                                                    response.with_finish_reason(finish_reason);
                                            }
                                            return Some((
                                                Ok(response),
                                                (
                                                    byte_stream,
                                                    buffer,
                                                    false,
                                                    consecutive_parse_errors,
                                                    total_parse_errors,
                                                    tool_call_accum,
                                                ),
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        // Fallback: OpenAI may evolve chunk schema in ways our typed
                                        // deserializer doesn't yet model. Try a best-effort JSON parse
                                        // and extract terminal usage so workflow metadata remains accurate.
                                        if let Ok(value) =
                                            serde_json::from_str::<serde_json::Value>(data)
                                        {
                                            let prompt_tokens = value
                                                .get("usage")
                                                .and_then(|u| u.get("prompt_tokens"))
                                                .and_then(|v| v.as_u64())
                                                .unwrap_or(0)
                                                as u32;
                                            let completion_tokens = value
                                                .get("usage")
                                                .and_then(|u| u.get("completion_tokens"))
                                                .and_then(|v| v.as_u64())
                                                .unwrap_or(0)
                                                as u32;

                                            if prompt_tokens > 0 || completion_tokens > 0 {
                                                let id = value
                                                    .get("id")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or_default()
                                                    .to_string();
                                                let finish_reason = value
                                                    .get("choices")
                                                    .and_then(|c| c.as_array())
                                                    .and_then(|arr| arr.first())
                                                    .and_then(|c| c.get("finish_reason"))
                                                    .and_then(|v| v.as_str())
                                                    .map(|r| parse_finish_reason(Some(r)));

                                                let streamed_tool_calls =
                                                    tool_accum_map_to_llm_calls(&tool_call_accum);
                                                let text = stream_assistant_text_for_tool_calls(
                                                    String::new(),
                                                    &streamed_tool_calls,
                                                );
                                                let mut response = LlmResponse::new(text, &model)
                                                    .with_usage(LlmUsage::new(
                                                        prompt_tokens,
                                                        completion_tokens,
                                                    ))
                                                    .with_tool_calls(streamed_tool_calls);
                                                if !id.is_empty() {
                                                    response = response.with_id(id);
                                                }
                                                if let Some(finish_reason) = finish_reason {
                                                    response =
                                                        response.with_finish_reason(finish_reason);
                                                }
                                                return Some((
                                                    Ok(response),
                                                    (
                                                        byte_stream,
                                                        buffer,
                                                        false,
                                                        consecutive_parse_errors,
                                                        total_parse_errors,
                                                        tool_call_accum,
                                                    ),
                                                ));
                                            }
                                        }

                                        // Track parse errors
                                        consecutive_parse_errors += 1;
                                        total_parse_errors += 1;

                                        // Log the parse error with context
                                        tracing::warn!(
                                            "Failed to parse stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        // If too many consecutive parse errors, fail the stream
                                        // This indicates something is seriously wrong (corrupted stream, API change, etc.)
                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS
                                        {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "openai",
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
                                                    tool_call_accum,
                                                ),
                                            ));
                                        }

                                        // For occasional errors (< threshold), continue but track
                                        // This handles edge cases like partial chunks or unusual formatting
                                    }
                                }
                            }
                        }
                    }
                }
            },
        );

        Ok(Box::new(Box::pin(stream)))
    }

    fn supports_streaming(&self) -> bool {
        true // OpenAI supports streaming
    }

    fn max_context_length(&self) -> Option<u32> {
        match self.model.as_str() {
            "gpt-4" => Some(8192),
            "gpt-4-32k" => Some(32_768),
            "gpt-4-turbo" => Some(128_000),
            "gpt-4o" => Some(128_000),
            "gpt-4o-mini" => Some(128_000),
            "gpt-3.5-turbo" => Some(4096),
            "gpt-3.5-turbo-16k" => Some(16_384),
            _ => None,
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Cost per token in USD (input, output) as of late 2023
        match self.model.as_str() {
            "gpt-4" => Some((0.000_03, 0.000_06)),
            "gpt-4-32k" => Some((0.000_06, 0.000_12)),
            "gpt-4-turbo" => Some((0.000_01, 0.000_03)),
            "gpt-4o" => Some((0.000_005, 0.000_015)),
            "gpt-4o-mini" => Some((0.000_000_15, 0.000_000_6)),
            "gpt-3.5-turbo" => Some((0.000_001_5, 0.000_002)),
            "gpt-3.5-turbo-16k" => Some((0.000_003, 0.000_004)),
            _ => None,
        }
    }
}

// `OpenAI` API types
#[derive(Debug, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<OpenAiStreamOptions>,
}

#[derive(Debug, Serialize)]
struct OpenAiStreamOptions {
    include_usage: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    #[serde(deserialize_with = "deserialize_nullable_content")]
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiToolCall {
    id: String,
    r#type: String,
    function: OpenAiFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiTool {
    r#type: String,
    function: OpenAiFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    id: String,
    choices: Vec<OpenAiChoice>,
    usage: OpenAiUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

/// Custom deserializer for nullable content field
/// `OpenAI` returns null for content when tool calls are made
fn deserialize_nullable_content<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

// Streaming-specific types
#[derive(Debug, Deserialize)]
struct OpenAiStreamChunk {
    id: String,
    choices: Vec<OpenAiStreamChoice>,
    #[serde(default)]
    usage: Option<OpenAiUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAiStreamChoice {
    delta: OpenAiDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiDeltaToolCall {
    #[serde(default)]
    index: Option<u32>,
    #[serde(default)]
    id: Option<String>,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    tool_type: Option<String>,
    #[serde(default)]
    function: Option<OpenAiDeltaFunctionPart>,
}

#[derive(Debug, Deserialize)]
struct OpenAiDeltaFunctionPart {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    _role: Option<String>,
    #[serde(default)]
    tool_calls: Vec<OpenAiDeltaToolCall>,
}

/// One tool-call slot in an SSE stream (`index`), merged across chunks.
#[derive(Debug, Default, Clone)]
struct OpenAiStreamToolAccum {
    id: String,
    name: String,
    arguments: String,
}

fn merge_openai_stream_tool_deltas(
    acc: &mut HashMap<u32, OpenAiStreamToolAccum>,
    deltas: &[OpenAiDeltaToolCall],
) {
    for d in deltas {
        let idx = d.index.unwrap_or(0);
        let entry = acc.entry(idx).or_default();
        if let Some(id) = &d.id {
            if !id.is_empty() {
                entry.id.clone_from(id);
            }
        }
        if let Some(f) = &d.function {
            if let Some(name) = &f.name {
                if !name.is_empty() {
                    entry.name.clone_from(name);
                }
            }
            if let Some(args) = &f.arguments {
                entry.arguments.push_str(args);
            }
        }
    }
}

fn tool_accum_map_to_llm_calls(acc: &HashMap<u32, OpenAiStreamToolAccum>) -> Vec<LlmToolCall> {
    let mut pairs: Vec<(u32, &OpenAiStreamToolAccum)> = acc.iter().map(|(i, t)| (*i, t)).collect();
    pairs.sort_by_key(|(i, _)| *i);
    pairs
        .into_iter()
        .map(|(_, tc)| {
            let parameters = if tc.arguments.trim().is_empty() {
                serde_json::Value::Object(serde_json::Map::new())
            } else {
                match serde_json::from_str(&tc.arguments) {
                    Ok(p) => p,
                    Err(e) => {
                        tracing::warn!(
                            "Failed to parse streamed tool call arguments for '{}': {e}",
                            tc.name
                        );
                        serde_json::json!(
                            { "raw_arguments": tc.arguments }
                        )
                    }
                }
            };
            LlmToolCall {
                id: tc.id.clone(),
                name: tc.name.clone(),
                parameters,
            }
        })
        .collect()
}

/// Align with non-streaming [`OpenAiProvider::parse_response`]: non-empty assistant text when the
/// model only requests tools (`content` is null in the REST response).
fn stream_assistant_text_for_tool_calls(mut content: String, tool_calls: &[LlmToolCall]) -> String {
    if content.trim().is_empty() && !tool_calls.is_empty() {
        content = "I'll help you with that using the available tools.".to_string();
    }
    content
}

fn parse_finish_reason(reason: Option<&str>) -> FinishReason {
    match reason {
        Some("stop") => FinishReason::Stop,
        Some("length") => FinishReason::Length,
        Some("tool_calls") => FinishReason::ToolCalls,
        Some("content_filter") => FinishReason::ContentFilter,
        Some(other) => FinishReason::Other(other.to_string()),
        None => FinishReason::Stop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_usage_chunk_parses_with_empty_choices() {
        let chunk = r#"{
            "id":"chatcmpl-test",
            "object":"chat.completion.chunk",
            "created":1775204971,
            "model":"gpt-4o-mini-2024-07-18",
            "choices":[],
            "usage":{"prompt_tokens":15,"completion_tokens":14,"total_tokens":29}
        }"#;

        let parsed: OpenAiStreamChunk =
            serde_json::from_str(chunk).expect("usage chunk should parse");
        assert!(parsed.choices.is_empty());
        let usage = parsed.usage.expect("usage should be present");
        assert_eq!(usage.prompt_tokens, 15);
        assert_eq!(usage.completion_tokens, 14);
    }

    #[test]
    fn stream_tool_call_deltas_merge_to_llm_calls() {
        let chunk1: OpenAiStreamChunk = serde_json::from_value(serde_json::json!({
            "id": "chatcmpl-toolstream",
            "choices": [{
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "id": "call_abc",
                        "type": "function",
                        "function": { "name": "add", "arguments": "" }
                    }]
                }
            }]
        }))
        .expect("chunk1");

        let chunk2: OpenAiStreamChunk = serde_json::from_value(serde_json::json!({
            "id": "chatcmpl-toolstream",
            "choices": [{
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "function": { "arguments": "{\"a\": 10, \"b\": 20}" }
                    }]
                },
                "finish_reason": "tool_calls"
            }]
        }))
        .expect("chunk2");

        let mut acc = HashMap::new();
        merge_openai_stream_tool_deltas(&mut acc, &chunk1.choices[0].delta.tool_calls);
        merge_openai_stream_tool_deltas(&mut acc, &chunk2.choices[0].delta.tool_calls);
        let calls = tool_accum_map_to_llm_calls(&acc);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id, "call_abc");
        assert_eq!(calls[0].name, "add");
        assert_eq!(calls[0].parameters["a"], 10);
        assert_eq!(calls[0].parameters["b"], 20);
    }

    #[test]
    fn stream_request_serializes_include_usage_option() {
        let req = OpenAiRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![OpenAiMessage {
                role: "user".to_string(),
                content: "hello".to_string(),
                tool_calls: None,
                tool_call_id: None,
            }],
            max_completion_tokens: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            stream: Some(true),
            stream_options: Some(OpenAiStreamOptions {
                include_usage: true,
            }),
        };

        let value = serde_json::to_value(req).expect("request should serialize");
        assert_eq!(value.get("stream").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            value
                .get("stream_options")
                .and_then(|v| v.get("include_usage"))
                .and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[test]
    fn fallback_json_usage_extraction_shape_is_supported() {
        let chunk = serde_json::json!({
            "id": "chatcmpl-fallback",
            "object": "chat.completion.chunk",
            // Intentionally omit "choices" to mimic a schema variant that would fail typed parsing.
            "usage": {
                "prompt_tokens": 21,
                "completion_tokens": 9,
                "total_tokens": 30
            }
        });

        let prompt_tokens = chunk
            .get("usage")
            .and_then(|u| u.get("prompt_tokens"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let completion_tokens = chunk
            .get("usage")
            .and_then(|u| u.get("completion_tokens"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        assert_eq!(prompt_tokens, 21);
        assert_eq!(completion_tokens, 9);
    }
}
