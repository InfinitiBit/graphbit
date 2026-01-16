//! `Anthropic` `Claude` LLM provider implementation

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

/// `Anthropic` `Claude` API provider
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl AnthropicProvider {
    /// Create a new `Anthropic` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = Client::new();
        let base_url = "https://api.anthropic.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert `GraphBit` tool to `Anthropic` tool format
    fn convert_tool(tool: &LlmTool) -> AnthropicTool {
        AnthropicTool {
            name: tool.name.clone(),
            description: tool.description.clone(),
            input_schema: tool.parameters.clone(),
        }
    }

    /// Convert `GraphBit` messages to `Anthropic` format
    fn convert_messages(messages: &[LlmMessage]) -> (Option<String>, Vec<AnthropicMessage>) {
        let mut system_prompt = None;
        let mut anthropic_messages = Vec::new();

        for message in messages {
            match message.role {
                LlmRole::System => {
                    system_prompt = Some(message.content.clone());
                }
                LlmRole::User => {
                    anthropic_messages.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: message.content.clone(),
                    });
                }
                LlmRole::Assistant => {
                    anthropic_messages.push(AnthropicMessage {
                        role: "assistant".to_string(),
                        content: message.content.clone(),
                    });
                }
                LlmRole::Tool => {
                    anthropic_messages.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: format!("Tool result: {}", message.content),
                    });
                }
            }
        }

        (system_prompt, anthropic_messages)
    }

    /// Parse `Anthropic` response to `GraphBit` response
    fn parse_response(&self, response: AnthropicResponse) -> GraphBitResult<LlmResponse> {
        let mut content_text = String::new();
        let mut tool_calls = Vec::new();

        // Process content blocks
        for block in &response.content {
            match block.r#type.as_str() {
                "text" => {
                    if let Some(text) = &block.text {
                        if !content_text.is_empty() {
                            content_text.push('\n');
                        }
                        content_text.push_str(text);
                    }
                }
                "tool_use" => {
                    if let (Some(id), Some(name), Some(input)) =
                        (&block.id, &block.name, &block.input)
                    {
                        tool_calls.push(LlmToolCall {
                            id: id.clone(),
                            name: name.clone(),
                            parameters: input.clone(),
                        });
                    }
                }
                _ => {
                    // Handle other content types if needed
                }
            }
        }

        let finish_reason = match response.stop_reason.as_deref() {
            Some("end_turn" | "stop_sequence") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("tool_use") => FinishReason::Other("tool_use".into()),
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

        let usage = LlmUsage::new(response.usage.input_tokens, response.usage.output_tokens);

        let mut llm_response = LlmResponse::new(content_text, &self.model)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response.id);

        // Add tool calls if any
        if !tool_calls.is_empty() {
            llm_response = llm_response.with_tool_calls(tool_calls);
        }

        Ok(llm_response)
    }
}

#[async_trait]
impl LlmProviderTrait for AnthropicProvider {
    fn provider_name(&self) -> &str {
        "anthropic"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/messages", self.base_url);

        let (system_prompt, messages) = Self::convert_messages(&request.messages);

        // Convert tools to `Anthropic` format
        let tools: Option<Vec<AnthropicTool>> = if request.tools.is_empty() {
            tracing::info!("No tools provided in request");
            None
        } else {
            tracing::info!("Converting {} tools for Anthropic", request.tools.len());
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages,
            system: system_prompt,
            temperature: request.temperature,
            top_p: request.top_p,
            tools,
        };

        tracing::info!(
            "Sending request to Anthropic with {} tools",
            body.tools.as_ref().map_or(0, Vec::len)
        );

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("anthropic", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "anthropic",
                format!("API error: {error_text}"),
            ));
        }

        let anthropic_response: AnthropicResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("anthropic", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(anthropic_response)
    }

    fn max_context_length(&self) -> Option<u32> {
        match self.model.as_str() {
            "claude-instant-1.2" => Some(100_000),
            "claude-2.0" => Some(100_000),
            "claude-2.1" => Some(200_000),
            "claude-3-sonnet-20240229" => Some(200_000),
            "claude-3-opus-20240229" => Some(200_000),
            "claude-3-haiku-20240307" => Some(200_000),
            _ if self.model.starts_with("claude-3") => Some(200_000),
            _ => None,
        }
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/messages", self.base_url);

        let (system_prompt, messages) = Self::convert_messages(&request.messages);

        // Convert tools to `Anthropic` format
        let tools: Option<Vec<AnthropicTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = AnthropicStreamRequest {
            model: self.model.clone(),
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages,
            system: system_prompt,
            temperature: request.temperature,
            top_p: request.top_p,
            tools,
            stream: true, // Enable streaming
        };

        // Timeout constants for different phases of the request
        const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);
        const ERROR_BODY_TIMEOUT: Duration = Duration::from_secs(10);
        const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);

        // Apply timeout to initial connection
        let response = timeout(
            CONNECTION_TIMEOUT,
            self.client
                .post(&url)
                .header("x-api-key", &self.api_key)
                .header("Content-Type", "application/json")
                .header("anthropic-version", "2023-06-01")
                .json(&body)
                .send(),
        )
        .await
        .map_err(|_| {
            GraphBitError::llm_provider(
                "anthropic",
                format!(
                    "Connection timeout after {:?} - Anthropic did not respond. \
                     Check network connectivity and Anthropic status.",
                    CONNECTION_TIMEOUT
                ),
            )
        })?
        .map_err(|e| GraphBitError::llm_provider("anthropic", format!("Request failed: {e}")))?;

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
                "anthropic",
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
                                    "Stream chunk timeout after {:?} - Anthropic stopped responding. \
                                     Response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "anthropic",
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
                                        "anthropic",
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

                        // Process complete lines
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            // Skip empty lines
                            if line.is_empty() {
                                continue;
                            }

                            // Anthropic SSE format: "event: <event_type>" followed by "data: <json>"
                            // We primarily care about content_block_delta events with text deltas
                            if let Some(data) = line.strip_prefix("data: ") {
                                // Parse the JSON data
                                match serde_json::from_str::<AnthropicStreamEvent>(data) {
                                    Ok(event) => {
                                        consecutive_parse_errors = 0;

                                        match event.r#type.as_str() {
                                            "content_block_delta" => {
                                                // Extract text from delta
                                                if let Some(delta) = &event.delta {
                                                    if delta.r#type == "text_delta" {
                                                        if let Some(text) = &delta.text {
                                                            if !text.is_empty() {
                                                                let response = LlmResponse::new(
                                                                    text.clone(),
                                                                    &model,
                                                                );
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
                                            }
                                            "message_stop" => {
                                                // End of message
                                                if total_parse_errors > 0 {
                                                    tracing::warn!(
                                                        "Stream completed with {} total parse errors.",
                                                        total_parse_errors
                                                    );
                                                }
                                                return None;
                                            }
                                            "error" => {
                                                // Handle error event
                                                let error_msg = event
                                                    .error
                                                    .as_ref()
                                                    .map(|e| e.message.clone())
                                                    .unwrap_or_else(|| "Unknown error".to_string());
                                                return Some((
                                                    Err(GraphBitError::llm_provider(
                                                        "anthropic",
                                                        format!("Stream error: {}", error_msg),
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
                                            // message_start, content_block_start, content_block_stop,
                                            // message_delta, ping - ignore these
                                            _ => {}
                                        }
                                    }
                                    Err(e) => {
                                        consecutive_parse_errors += 1;
                                        total_parse_errors += 1;

                                        tracing::warn!(
                                            "Failed to parse Anthropic stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS
                                        {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "anthropic",
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
        true // Anthropic supports streaming
    }
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    content: Vec<ContentBlock>,
    stop_reason: Option<String>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    r#type: String, // "text", "tool_use", "tool_result", "thinking", etc.
    text: Option<String>,             // present when type == "text"
    id: Option<String>,               // present when type == "tool_use"
    name: Option<String>,             // present when type == "tool_use"
    input: Option<serde_json::Value>, // present when type == "tool_use"
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

// Streaming-specific types

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct AnthropicStreamRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
    stream: bool,
}

/// Represents an event in the Anthropic SSE stream
/// Anthropic sends various event types: message_start, content_block_start,
/// content_block_delta, content_block_stop, message_delta, message_stop, ping, error
#[derive(Debug, Deserialize)]
struct AnthropicStreamEvent {
    #[serde(rename = "type")]
    r#type: String,
    /// Present in content_block_delta events
    #[serde(default)]
    delta: Option<StreamDelta>,
    /// Present in error events
    #[serde(default)]
    error: Option<StreamError>,
}

/// Delta content in a content_block_delta event
#[derive(Debug, Deserialize)]
struct StreamDelta {
    #[serde(rename = "type")]
    r#type: String, // "text_delta", "input_json_delta", etc.
    /// Text content (present when type == "text_delta")
    #[serde(default)]
    text: Option<String>,
}

/// Error information in an error event
#[derive(Debug, Deserialize)]
struct StreamError {
    #[serde(rename = "type")]
    r#type: String,
    message: String,
}
