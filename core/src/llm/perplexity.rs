//! `Perplexity` AI LLM provider implementation

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

/// `Perplexity` AI provider
pub struct PerplexityProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl PerplexityProvider {
    /// Create a new `Perplexity` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "perplexity",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://api.perplexity.ai".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `Perplexity` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "perplexity",
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

    /// Convert `GraphBit` message to `Perplexity` message format (`OpenAI`-compatible)
    fn convert_message(message: &LlmMessage) -> PerplexityMessage {
        PerplexityMessage {
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
                        .map(|tc| PerplexityToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: PerplexityFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `Perplexity` tool format (`OpenAI`-compatible)
    fn convert_tool(tool: &LlmTool) -> PerplexityTool {
        PerplexityTool {
            r#type: "function".to_string(),
            function: PerplexityFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `Perplexity` response to `GraphBit` response
    fn parse_response(&self, response: PerplexityResponse) -> GraphBitResult<LlmResponse> {
        let choice =
            response.choices.into_iter().next().ok_or_else(|| {
                GraphBitError::llm_provider("perplexity", "No choices in response")
            })?;

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
impl LlmProviderTrait for PerplexityProvider {
    fn provider_name(&self) -> &str {
        "perplexity"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<PerplexityMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<PerplexityTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = PerplexityRequest {
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
            .header("Accept", "application/json")
            .json(&request_json)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("perplexity", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "perplexity",
                format!("API error: {error_text}"),
            ));
        }

        let perplexity_response: PerplexityResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("perplexity", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(perplexity_response)
    }

    fn supports_function_calling(&self) -> bool {
        // `Perplexity` models support function calling through `OpenAI`-compatible interface
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        match self.model.as_str() {
            "pplx-7b-online" | "pplx-70b-online" => Some(4096),
            "pplx-7b-chat" | "pplx-70b-chat" => Some(8192),
            "llama-2-70b-chat" => Some(4096),
            "codellama-34b-instruct" => Some(16_384),
            "mistral-7b-instruct" => Some(16_384),
            "sonar" | "sonar-reasoning" => Some(8192),
            "sonar-deep-research" => Some(32_768),
            _ if self.model.starts_with("sonar") => Some(8192),
            _ => Some(4096), // Conservative default
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Cost per token in USD (input, output) based on `Perplexity` pricing
        match self.model.as_str() {
            "pplx-7b-online" => Some((0.000_000_2, 0.000_000_2)),
            "pplx-70b-online" => Some((0.000_001, 0.000_001)),
            "pplx-7b-chat" => Some((0.000_000_2, 0.000_000_2)),
            "pplx-70b-chat" => Some((0.000_001, 0.000_001)),
            "llama-2-70b-chat" => Some((0.000_001, 0.000_001)),
            "codellama-34b-instruct" => Some((0.000_000_35, 0.000_001_40)),
            "mistral-7b-instruct" => Some((0.000_000_2, 0.000_000_2)),
            "sonar" => Some((0.000_001, 0.000_001)),
            "sonar-reasoning" => Some((0.000_002, 0.000_002)),
            "sonar-deep-research" => Some((0.000_005, 0.000_005)),
            _ => None,
        }
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<PerplexityMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<PerplexityTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = PerplexityStreamRequest {
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
        //   (Perplexity API can be slow to establish connections under load)
        // - ERROR_BODY_TIMEOUT: Short timeout since error responses are typically small JSON
        //   (don't want to wait long if API is unresponsive)
        // - CHUNK_TIMEOUT: Moderate timeout for each SSE chunk; Perplexity streams can have
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
                "perplexity",
                format!(
                    "Connection timeout after {:?} - Perplexity did not respond. \
                     Check network connectivity and Perplexity status.",
                    CONNECTION_TIMEOUT
                ),
            )
        })?
        .map_err(|e| GraphBitError::llm_provider("perplexity", format!("Request failed: {e}")))?;

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
                "perplexity",
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
                                    "Stream chunk timeout after {:?} - Perplexity stopped responding. \
                                     Response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "perplexity",
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
                                        "perplexity",
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
                            // Perplexity uses OpenAI-compatible SSE streaming format
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
                                // PerplexityStreamChunk follows OpenAI's streaming response format:
                                // { "id": "...", "choices": [{ "delta": { "content": "..." } }] }
                                match serde_json::from_str::<PerplexityStreamChunk>(data) {
                                    Ok(stream_chunk) => {
                                        // Reset consecutive error counter on successful parse
                                        consecutive_parse_errors = 0;

                                        // Extract content from the first choice's delta
                                        // (Perplexity typically returns only one choice per chunk)
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
                                            "Failed to parse Perplexity stream chunk (consecutive: {}, total: {}): {}, data: {}",
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
                                                    "perplexity",
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
        true // Perplexity supports streaming via OpenAI-compatible API
    }
}

// `Perplexity` API types (`OpenAI`-compatible)
#[derive(Debug, Serialize)]
struct PerplexityRequest {
    model: String,
    messages: Vec<PerplexityMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<PerplexityTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PerplexityMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<PerplexityToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PerplexityToolCall {
    id: String,
    r#type: String,
    function: PerplexityFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct PerplexityFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct PerplexityTool {
    r#type: String,
    function: PerplexityFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct PerplexityFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct PerplexityResponse {
    id: String,
    choices: Vec<PerplexityChoice>,
    usage: PerplexityUsage,
}

#[derive(Debug, Deserialize)]
struct PerplexityChoice {
    message: PerplexityMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PerplexityUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

// Streaming-specific types (OpenAI-compatible format)

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct PerplexityStreamRequest {
    model: String,
    messages: Vec<PerplexityMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<PerplexityTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Streaming chunk from Perplexity API (OpenAI-compatible format)
#[derive(Debug, Deserialize)]
struct PerplexityStreamChunk {
    id: String,
    choices: Vec<PerplexityStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct PerplexityStreamChoice {
    delta: PerplexityDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PerplexityDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
}
