//! `MistralAI` LLM provider implementation

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

/// `MistralAI` API provider
pub struct MistralAiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl MistralAiProvider {
    /// Create a new `MistralAI` provider
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
                    "mistralai",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://api.mistral.ai/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `MistralAI` provider with custom base URL
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
                    "mistralai",
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

    /// Convert `GraphBit` message to `MistralAI` message format
    fn convert_message(message: &LlmMessage) -> MistralAiMessage {
        let (content, tool_call_id) = if message.role == LlmRole::Tool {
            // For tool messages, extract tool_call_id from content
            // GraphBit format: "Tool call {tool_call_id} result: {actual_result}"
            let content_str = &message.content;
            if let Some(start) = content_str.find("Tool call ") {
                if let Some(end) = content_str.find(" result: ") {
                    let tool_call_id = content_str[start + 10..end].to_string();
                    let actual_result = content_str[end + 9..].to_string();
                    (actual_result, Some(tool_call_id))
                } else {
                    // Fallback: use the entire content as result
                    (content_str.clone(), None)
                }
            } else {
                // Fallback: use the entire content as result
                (content_str.clone(), None)
            }
        } else {
            (message.content.clone(), None)
        };

        MistralAiMessage {
            role: match message.role {
                LlmRole::User => "user".to_string(),
                LlmRole::Assistant => "assistant".to_string(),
                LlmRole::System => "system".to_string(),
                LlmRole::Tool => "tool".to_string(),
            },
            content: if message.role == LlmRole::Assistant && !message.tool_calls.is_empty() {
                // Assistant messages with tool calls should have empty content
                None
            } else {
                Some(content)
            },
            tool_calls: if message.tool_calls.is_empty() {
                None
            } else {
                Some(
                    message
                        .tool_calls
                        .iter()
                        .map(|tc| MistralAiToolCall {
                            id: tc.id.clone(),
                            r#type: Some("function".to_string()),
                            function: MistralAiFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                            index: None,
                        })
                        .collect(),
                )
            },
            tool_call_id,
        }
    }

    /// Convert `GraphBit` tool to `MistralAI` tool format
    fn convert_tool(tool: &LlmTool) -> MistralAiTool {
        MistralAiTool {
            r#type: "function".to_string(),
            function: MistralAiFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `MistralAI` response to `GraphBit` response
    fn parse_response(&self, response: MistralAiResponse) -> GraphBitResult<LlmResponse> {
        let choice = response.choices.into_iter().next().ok_or_else(|| {
            GraphBitError::llm_provider("mistralai", "No choices in response".to_string())
        })?;

        let tool_calls: Vec<LlmToolCall> = choice
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

        // Handle content - provide default message for tool calls
        let mut content = choice.message.content.unwrap_or_default();
        if content.trim().is_empty() && !tool_calls.is_empty() {
            content = "I'll help you with that using the available tools.".to_string();
        }

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
impl LlmProviderTrait for MistralAiProvider {
    fn provider_name(&self) -> &str {
        "mistralai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<MistralAiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<MistralAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = MistralAiRequest {
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
            .map_err(|e| {
                GraphBitError::llm_provider("mistralai", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "mistralai",
                format!("API error: {error_text}"),
            ));
        }

        let mistralai_response: MistralAiResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("mistralai", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(mistralai_response)
    }

    fn supports_function_calling(&self) -> bool {
        // Most `MistralAI` models support function calling
        matches!(
            self.model.as_str(),
            "mistral-large-latest"
                | "mistral-medium-latest"
                | "mistral-small-latest"
                | "codestral-latest"
                | "ministral-8b-latest"
                | "ministral-3b-latest"
                | "pixtral-large-latest"
                | "pixtral-12b-latest"
        ) || self.model.starts_with("mistral-large")
            || self.model.starts_with("mistral-medium")
            || self.model.starts_with("mistral-small")
            || self.model.starts_with("codestral")
            || self.model.starts_with("ministral")
            || self.model.starts_with("pixtral")
    }

    fn supports_streaming(&self) -> bool {
        true // MistralAI supports streaming via OpenAI-compatible SSE
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<MistralAiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<MistralAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = MistralAiStreamRequest {
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
            stream: Some(true),
        };

        // Add extra parameters
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

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
                "mistralai",
                format!(
                    "Connection timeout after {:?} - MistralAI did not respond. \
                     Check network connectivity and MistralAI status.",
                    CONNECTION_TIMEOUT
                ),
            )
        })?
        .map_err(|e| GraphBitError::llm_provider("mistralai", format!("Request failed: {e}")))?;

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
                "mistralai",
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
                        // Process complete lines already in the buffer
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            // Skip empty lines and comments
                            if line.is_empty() || line.starts_with(':') {
                                continue;
                            }

                            // Check for data: prefix (OpenAI-compatible SSE format)
                            if let Some(data) = line.strip_prefix("data: ") {
                                // Check for [DONE] marker
                                if data.trim() == "[DONE]" {
                                    if total_parse_errors > 0 {
                                        tracing::warn!(
                                            "MistralAI stream completed with {} total parse errors.",
                                            total_parse_errors
                                        );
                                    }
                                    return None; // End of stream
                                }

                                // Parse JSON chunk
                                match serde_json::from_str::<MistralAiStreamChunk>(data) {
                                    Ok(stream_chunk) => {
                                        consecutive_parse_errors = 0;

                                        if let Some(choice) = stream_chunk.choices.first() {
                                            if let Some(content) = &choice.delta.content {
                                                if !content.is_empty() {
                                                    let response =
                                                        LlmResponse::new(content.clone(), &model)
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
                                        }
                                    }
                                    Err(e) => {
                                        consecutive_parse_errors += 1;
                                        total_parse_errors += 1;

                                        tracing::warn!(
                                            "Failed to parse MistralAI stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS
                                        {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "mistralai",
                                                    format!(
                                                        "Stream corrupted: {} consecutive parse errors. \
                                                         Last error: {}. Data may be incomplete.",
                                                        consecutive_parse_errors, e
                                                    ),
                                                )),
                                                (byte_stream, buffer, true, consecutive_parse_errors, total_parse_errors),
                                            ));
                                        }
                                    }
                                }
                            }
                        }

                        // Need more data from the network
                        let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                            Ok(Some(result)) => result,
                            Ok(None) => {
                                // Stream naturally ended
                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "MistralAI stream ended with {} total parse errors.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "MistralAI stream chunk timeout after {:?} - response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "mistralai",
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
                                        "mistralai",
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

    fn max_context_length(&self) -> Option<u32> {
        // Context lengths for different MistralAI models
        match self.model.as_str() {
            "mistral-large-latest" | "mistral-medium-latest" | "mistral-small-latest" => {
                Some(128_000)
            }
            "pixtral-large-latest" | "pixtral-12b-latest" => Some(128_000),
            "ministral-8b-latest" | "ministral-3b-latest" => Some(128_000),
            "codestral-latest" => Some(256_000),
            _ if self.model.starts_with("mistral-large") => Some(128_000),
            _ if self.model.starts_with("mistral-medium") => Some(128_000),
            _ if self.model.starts_with("mistral-small") => Some(128_000),
            _ if self.model.starts_with("codestral") => Some(256_000),
            _ if self.model.starts_with("ministral") => Some(128_000),
            _ if self.model.starts_with("pixtral") => Some(128_000),
            _ => None,
        }
    }
}

// `MistralAI` API types
#[derive(Debug, Serialize)]
struct MistralAiRequest {
    model: String,
    messages: Vec<MistralAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<MistralAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MistralAiMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<MistralAiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MistralAiToolCall {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    function: MistralAiFunction,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MistralAiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct MistralAiTool {
    r#type: String,
    function: MistralAiFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct MistralAiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct MistralAiResponse {
    id: String,
    choices: Vec<MistralAiChoice>,
    usage: MistralAiUsage,
}

#[derive(Debug, Deserialize)]
struct MistralAiChoice {
    message: MistralAiResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MistralAiResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<MistralAiToolCall>>,
}

#[derive(Debug, Deserialize)]
struct MistralAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

// Streaming-specific types (OpenAI-compatible SSE format)

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct MistralAiStreamRequest {
    model: String,
    messages: Vec<MistralAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<MistralAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Streaming chunk from MistralAI API (OpenAI-compatible format)
#[derive(Debug, Deserialize)]
struct MistralAiStreamChunk {
    id: String,
    choices: Vec<MistralAiStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct MistralAiStreamChoice {
    delta: MistralAiDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MistralAiDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
}
