//! `DeepSeek` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

/// `DeepSeek` API provider
pub struct DeepSeekProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl DeepSeekProvider {
    /// Create a new `DeepSeek` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10) // Increased connection pool size
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "deepseek",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://api.deepseek.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `DeepSeek` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        // Use same optimized client settings
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "deepseek",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert `GraphBit` message to `DeepSeek` message format
    fn convert_message(message: &LlmMessage) -> DeepSeekMessage {
        DeepSeekMessage {
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
                        .map(|tc| DeepSeekToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: DeepSeekFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `DeepSeek` tool format
    fn convert_tool(tool: &LlmTool) -> DeepSeekTool {
        DeepSeekTool {
            r#type: "function".to_string(),
            function: DeepSeekFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `DeepSeek` response to `GraphBit` response
    fn parse_response(&self, response: DeepSeekResponse) -> GraphBitResult<LlmResponse> {
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| GraphBitError::llm_provider("deepseek", "No choices in response"))?;

        let content = choice.message.content;
        let tool_calls = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| LlmToolCall {
                id: tc.id,
                name: tc.function.name,
                parameters: serde_json::from_str(&tc.function.arguments).unwrap_or_default(),
            })
            .collect();

        let finish_reason = match choice.finish_reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("content_filter") => FinishReason::ContentFilter,
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

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
impl LlmProviderTrait for DeepSeekProvider {
    fn provider_name(&self) -> &str {
        "deepseek"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<DeepSeekMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<DeepSeekTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = DeepSeekRequest {
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

        // Add extra parameters
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_json)
            .send()
            .await
            .map_err(|e| GraphBitError::llm_provider("deepseek", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "deepseek",
                format!("API error: {error_text}"),
            ));
        }

        let deepseek_response: DeepSeekResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("deepseek", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(deepseek_response)
    }

    fn supports_function_calling(&self) -> bool {
        // `DeepSeek` models support function calling
        matches!(
            self.model.as_str(),
            "deepseek-chat" | "deepseek-coder" | "deepseek-reasoner"
        ) || self.model.starts_with("deepseek-")
    }

    fn max_context_length(&self) -> Option<u32> {
        match self.model.as_str() {
            "deepseek-chat" => Some(128_000),
            "deepseek-coder" => Some(128_000),
            "deepseek-reasoner" => Some(128_000),
            _ if self.model.starts_with("deepseek-") => Some(128_000),
            _ => None,
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Cost per token in USD (input, output) - `DeepSeek` is very competitive
        match self.model.as_str() {
            "deepseek-chat" => Some((0.000_000_14, 0.000_000_28)), // $0.14/$0.28 per 1M tokens
            "deepseek-coder" => Some((0.000_000_14, 0.000_000_28)), // $0.14/$0.28 per 1M tokens
            "deepseek-reasoner" => Some((0.000_000_55, 0.000_002_2)), // $0.55/$2.19 per 1M tokens
            _ if self.model.starts_with("deepseek-") => Some((0.000_000_14, 0.000_000_28)),
            _ => None,
        }
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<DeepSeekMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<DeepSeekTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = DeepSeekStreamRequest {
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

        // Add extra parameters
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

        // Timeout constants for different phases of the request
        // These values balance responsiveness with network variability:
        // - CONNECTION_TIMEOUT: Generous time for initial TLS handshake and HTTP connection
        //   (DeepSeek API can be slow to establish connections under load)
        // - ERROR_BODY_TIMEOUT: Short timeout since error responses are typically small JSON
        //   (don't want to wait long if API is unresponsive)
        // - CHUNK_TIMEOUT: Moderate timeout for each SSE chunk; DeepSeek streams can have
        //   pauses between tokens but should not hang indefinitely
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
                "deepseek",
                format!(
                    "Connection timeout after {:?} - DeepSeek did not respond. \
                     Check network connectivity and DeepSeek status.",
                    CONNECTION_TIMEOUT
                ),
            )
        })?
        .map_err(|e| GraphBitError::llm_provider("deepseek", format!("Request failed: {e}")))?;

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
                "deepseek",
                format!("API error: {error_text}"),
            ));
        }

        // Parse SSE stream with proper line buffering and per-chunk timeout
        let model = self.model.clone();
        let byte_stream = response.bytes_stream();

        // State tuple for stream processing (unfold accumulator):
        // 0. byte_stream: The HTTP response body stream from reqwest
        // 1. buffer: String buffer for incomplete SSE lines (SSE events are newline-delimited)
        // 2. timeout_occurred: Circuit breaker flag - true if any timeout happened, stops further processing
        // 3. consecutive_parse_errors: Counter for back-to-back JSON parse failures (circuit breaker at MAX)
        // 4. total_parse_errors: Running count of all parse errors (for end-of-stream warning logs)
        const MAX_CONSECUTIVE_PARSE_ERRORS: u32 = 5;
        // Threshold of 5 consecutive errors chosen because:
        // - 1-2 errors could be transient network glitches
        // - 3-4 errors suggest potential format issues but may recover
        // - 5+ errors strongly indicates stream corruption or API malfunction
        // After this threshold, we abort to prevent infinite loops on corrupted streams

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
                        // Apply timeout to each chunk read
                        let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                            Ok(Some(result)) => result,
                            Ok(None) => {
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
                                    "Stream chunk timeout after {:?} - DeepSeek stopped responding. \
                                     Response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "deepseek",
                                        format!(
                                            "Stream timeout after {:?} - response may be incomplete",
                                            CHUNK_TIMEOUT
                                        ),
                                    )),
                                    (byte_stream, buffer, true, consecutive_parse_errors, total_parse_errors),
                                ));
                            }
                        };

                        let chunk = match chunk_result {
                            Ok(c) => c,
                            Err(e) => {
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "deepseek",
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

                        // SSE (Server-Sent Events) buffering strategy:
                        // HTTP chunks may split SSE events mid-line, so we buffer until complete lines are received.
                        // SSE format: each event is a line starting with "data: " and ending with \n\n (double newline)
                        // We use `drain()` to efficiently remove processed lines from the buffer front
                        buffer.push_str(&String::from_utf8_lossy(&chunk));

                        // Process all complete lines (those ending with newline)
                        // drain() is used to remove processed characters from the front of the buffer,
                        // preserving any incomplete line data for the next chunk
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            // SSE protocol: skip empty lines (event separators) and comment lines (start with ':')
                            // Comments are often used for keep-alive heartbeats
                            if line.is_empty() || line.starts_with(':') {
                                continue;
                            }

                            // Parse SSE data field format: "data: <json_content>"
                            // DeepSeek uses OpenAI-compatible SSE streaming format
                            if let Some(data) = line.strip_prefix("data: ") {
                                // [DONE] is the standard SSE termination marker in OpenAI-compatible APIs
                                // It indicates the stream has ended successfully (not an error)
                                if data.trim() == "[DONE]" {
                                    // Warn if we had parse errors during the stream - data may be incomplete
                                    if total_parse_errors > 0 {
                                        tracing::warn!(
                                            "Stream completed with {} total parse errors. Some data may have been lost.",
                                            total_parse_errors
                                        );
                                    }
                                    return None; // Signal stream completion to unfold
                                }

                                // Parse the JSON payload containing the streaming response chunk
                                // DeepSeekStreamChunk follows OpenAI's streaming response format:
                                // { "id": "...", "choices": [{ "delta": { "content": "..." } }] }
                                match serde_json::from_str::<DeepSeekStreamChunk>(data) {
                                    Ok(stream_chunk) => {
                                        // Reset consecutive error counter on successful parse
                                        consecutive_parse_errors = 0;

                                        // Extract content from the first choice's delta
                                        // (DeepSeek typically returns only one choice per chunk)
                                        if let Some(choice) = stream_chunk.choices.first() {
                                            if let Some(content) = &choice.delta.content {
                                                // Only yield non-empty content to reduce noise
                                                if !content.is_empty() {
                                                    let response =
                                                        LlmResponse::new(content.clone(), &model)
                                                            .with_id(stream_chunk.id);
                                                    // Return this chunk and continue streaming
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
                                        }
                                    }
                                    Err(e) => {
                                        // Increment both error counters for circuit breaker logic
                                        consecutive_parse_errors += 1;
                                        total_parse_errors += 1;

                                        // Log truncated data (first 200 chars) for debugging
                                        // while preventing log spam from huge malformed payloads
                                        tracing::warn!(
                                            "Failed to parse DeepSeek stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        // Circuit breaker: abort stream after too many consecutive errors
                                        // Prevents infinite loops on corrupted or malformed streams
                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS
                                        {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "deepseek",
                                                    format!(
                                                        "Stream corrupted: {} consecutive parse errors. \
                                                         Last error: {}. Data may be incomplete.",
                                                        consecutive_parse_errors,
                                                        e
                                                    ),
                                                )),
                                                (byte_stream, buffer, true, consecutive_parse_errors, total_parse_errors),
                                            ));
                                        }
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
        true // DeepSeek supports streaming via OpenAI-compatible API
    }
}

// `DeepSeek` API types (similar to `OpenAI` since `DeepSeek` follows `OpenAI` API format)
#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<DeepSeekTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<DeepSeekToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekToolCall {
    id: String,
    r#type: String,
    function: DeepSeekFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct DeepSeekTool {
    r#type: String,
    function: DeepSeekFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct DeepSeekFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    id: String,
    choices: Vec<DeepSeekChoice>,
    usage: DeepSeekUsage,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

// Streaming-specific types (OpenAI-compatible format)

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct DeepSeekStreamRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<DeepSeekTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Streaming chunk from DeepSeek API (OpenAI-compatible format)
#[derive(Debug, Deserialize)]
struct DeepSeekStreamChunk {
    id: String,
    choices: Vec<DeepSeekStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamChoice {
    delta: DeepSeekDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
}
