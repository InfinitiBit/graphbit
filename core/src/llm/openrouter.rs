//! `OpenRouter` LLM provider implementation
//!
//! `OpenRouter` provides unified access to multiple AI models through a single API.
//! It supports OpenAI-compatible endpoints with additional features like model routing
//! and provider preferences.

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

/// `OpenRouter` API provider
pub struct OpenRouterProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
    site_url: Option<String>,
    site_name: Option<String>,
}

impl OpenRouterProvider {
    /// Create a new `OpenRouter` provider
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
                    "openrouter",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://openrouter.ai/api/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            site_url: None,
            site_name: None,
        })
    }

    /// Create a new `OpenRouter` provider with custom base URL
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
                    "openrouter",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            site_url: None,
            site_name: None,
        })
    }

    /// Create a new `OpenRouter` provider with site information for rankings
    pub fn with_site_info(
        api_key: String,
        model: String,
        site_url: Option<String>,
        site_name: Option<String>,
    ) -> GraphBitResult<Self> {
        let mut provider = Self::new(api_key, model)?;
        provider.site_url = site_url;
        provider.site_name = site_name;
        Ok(provider)
    }

    /// Convert `GraphBit` message to `OpenRouter` message format (`OpenAI`-compatible)
    fn convert_message(message: &LlmMessage) -> OpenRouterMessage {
        OpenRouterMessage {
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
                        .map(|tc| OpenRouterToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: OpenRouterFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `OpenRouter` tool format (`OpenAI`-compatible)
    fn convert_tool(tool: &LlmTool) -> OpenRouterTool {
        OpenRouterTool {
            r#type: "function".to_string(),
            function: OpenRouterFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `OpenRouter` response to `GraphBit` response
    fn parse_response(&self, response: OpenRouterResponse) -> GraphBitResult<LlmResponse> {
        let choice =
            response.choices.into_iter().next().ok_or_else(|| {
                GraphBitError::llm_provider("openrouter", "No choices in response")
            })?;

        let content = choice.message.content.unwrap_or_default();
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

        let usage = if let Some(usage) = response.usage {
            LlmUsage::new(usage.prompt_tokens, usage.completion_tokens)
        } else {
            LlmUsage::new(0, 0)
        };

        Ok(LlmResponse::new(content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response.id))
    }
}

#[async_trait]
impl LlmProviderTrait for OpenRouterProvider {
    fn provider_name(&self) -> &str {
        "openrouter"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenRouterMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OpenRouterTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = OpenRouterRequest {
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

        let mut request_builder = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json");

        // Add optional site information for `OpenRouter` rankings
        if let Some(ref site_url) = self.site_url {
            request_builder = request_builder.header("HTTP-Referer", site_url);
        }
        if let Some(ref site_name) = self.site_name {
            request_builder = request_builder.header("X-Title", site_name);
        }

        let response = request_builder
            .json(&request_json)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("openrouter", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "openrouter",
                format!("API error: {error_text}"),
            ));
        }

        let openrouter_response: OpenRouterResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("openrouter", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(openrouter_response)
    }

    fn supports_streaming(&self) -> bool {
        true // OpenRouter supports streaming via OpenAI-compatible SSE
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenRouterMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OpenRouterTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = OpenRouterStreamRequest {
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

        // Merge extra_params into request JSON
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

        // Timeout constants
        const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);
        const ERROR_BODY_TIMEOUT: Duration = Duration::from_secs(10);
        const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);

        // Build request with auth headers
        let mut builder = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_json);

        // Add optional site information for OpenRouter rankings
        if let Some(ref site_url) = self.site_url {
            builder = builder.header("HTTP-Referer", site_url);
        }
        if let Some(ref site_name) = self.site_name {
            builder = builder.header("X-Title", site_name);
        }

        // Apply timeout to initial connection
        let response = timeout(CONNECTION_TIMEOUT, builder.send())
            .await
            .map_err(|_| {
                GraphBitError::llm_provider(
                    "openrouter",
                    format!(
                        "Connection timeout after {:?} - OpenRouter did not respond. \
                         Check network connectivity and OpenRouter status.",
                        CONNECTION_TIMEOUT
                    ),
                )
            })?
            .map_err(|e| {
                GraphBitError::llm_provider("openrouter", format!("Request failed: {e}"))
            })?;

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
                "openrouter",
                format!("API error: {error_text}"),
            ));
        }

        // Parse SSE stream (OpenAI-compatible format: "data: " prefix, "[DONE]" terminator)
        // Note: OpenRouter may send `: OPENROUTER PROCESSING` comments to prevent timeouts
        let model = self.model.clone();
        let byte_stream = response.bytes_stream();

        const MAX_CONSECUTIVE_PARSE_ERRORS: u32 = 5;

        let stream = futures::stream::unfold(
            (byte_stream, String::new(), false, 0u32, 0u32),
            move |(
                mut byte_stream,
                mut buffer,
                done,
                mut consecutive_parse_errors,
                mut total_parse_errors,
            )| {
                let model = model.clone();
                async move {
                    if done {
                        return None;
                    }

                    loop {
                        // Process complete lines in the buffer
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            // Skip empty lines and SSE comments
                            // OpenRouter sends `: OPENROUTER PROCESSING` as keepalive
                            if line.is_empty() || line.starts_with(':') {
                                continue;
                            }

                            // Check for data: prefix (OpenAI-compatible SSE)
                            if let Some(data) = line.strip_prefix("data: ") {
                                // Check for [DONE] marker
                                if data.trim() == "[DONE]" {
                                    if total_parse_errors > 0 {
                                        tracing::warn!(
                                            "OpenRouter stream completed with {} total parse errors.",
                                            total_parse_errors
                                        );
                                    }
                                    return None;
                                }

                                // Parse JSON chunk
                                match serde_json::from_str::<OpenRouterStreamChunk>(data) {
                                    Ok(chunk) => {
                                        consecutive_parse_errors = 0;

                                        if let Some(choice) = chunk.choices.first() {
                                            if let Some(content) = &choice.delta.content {
                                                if !content.is_empty() {
                                                    let response =
                                                        LlmResponse::new(content.clone(), &model)
                                                            .with_id(chunk.id);
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
                                            "Failed to parse OpenRouter stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS
                                        {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "openrouter",
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
                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "OpenRouter stream ended with {} total parse errors.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "OpenRouter stream chunk timeout after {:?} - response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "openrouter",
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
                                        "openrouter",
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

                        buffer.push_str(&String::from_utf8_lossy(&chunk));
                    }
                }
            },
        );

        Ok(Box::new(Box::pin(stream)))
    }

    fn supports_function_calling(&self) -> bool {
        // `OpenRouter` supports function calling through OpenAI-compatible interface
        // Most models on `OpenRouter` support this, but it depends on the specific model
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // Context length varies by model on `OpenRouter`
        // Common models and their approximate context lengths
        match self.model.as_str() {
            // `OpenAI` models
            "openai/gpt-4o" | "openai/gpt-4o-mini" => Some(128_000),
            "openai/gpt-4-turbo" => Some(128_000),
            "openai/gpt-4" => Some(8192),
            "openai/gpt-3.5-turbo" => Some(16_385),

            // `Anthropic` models
            "anthropic/claude-3-5-sonnet" | "anthropic/claude-3-5-haiku" => Some(200_000),
            "anthropic/claude-3-opus"
            | "anthropic/claude-3-sonnet"
            | "anthropic/claude-3-haiku" => Some(200_000),

            // `Google` models
            "google/gemini-pro" => Some(32_768),
            "google/gemini-pro-1.5" => Some(1_000_000),

            // `Meta` models
            "meta-llama/llama-3.1-405b-instruct" => Some(131_072),
            "meta-llama/llama-3.1-70b-instruct" => Some(131_072),
            "meta-llama/llama-3.1-8b-instruct" => Some(131_072),

            // `Mistral` models
            "mistralai/mistral-large" => Some(128_000),
            "mistralai/mistral-medium" => Some(32_768),

            // Default for unknown models
            _ => Some(4096),
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Cost per token in USD (input, output) - varies by model on `OpenRouter`
        // These are approximate costs and may change
        match self.model.as_str() {
            // `OpenAI` models (approximate OpenRouter pricing)
            "openai/gpt-4o" => Some((0.000_002_5, 0.000_01)),
            "openai/gpt-4o-mini" => Some((0.000_000_15, 0.000_000_6)),
            "openai/gpt-4-turbo" => Some((0.000_01, 0.000_03)),
            "openai/gpt-4" => Some((0.000_03, 0.000_06)),
            "openai/gpt-3.5-turbo" => Some((0.000_000_5, 0.000_001_5)),

            // `Anthropic` models
            "anthropic/claude-3-5-sonnet" => Some((0.000_003, 0.000_015)),
            "anthropic/claude-3-opus" => Some((0.000_015, 0.000_075)),
            "anthropic/claude-3-sonnet" => Some((0.000_003, 0.000_015)),
            "anthropic/claude-3-haiku" => Some((0.000_000_25, 0.000_001_25)),

            // Many other models on OpenRouter are free or very low cost
            _ => None,
        }
    }
}

// `OpenRouter` API types (OpenAI-compatible with some extensions)
#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<OpenRouterMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenRouterTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenRouterMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenRouterToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenRouterToolCall {
    id: String,
    r#type: String,
    function: OpenRouterFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenRouterFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct OpenRouterTool {
    r#type: String,
    function: OpenRouterFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct OpenRouterFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    id: String,
    choices: Vec<OpenRouterChoice>,
    usage: Option<OpenRouterUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponseMessage {
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenRouterToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

// Streaming-specific types (OpenAI-compatible SSE format)

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct OpenRouterStreamRequest {
    model: String,
    messages: Vec<OpenRouterMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenRouterTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Streaming chunk from OpenRouter API (OpenAI-compatible format)
#[derive(Debug, Deserialize)]
struct OpenRouterStreamChunk {
    id: String,
    choices: Vec<OpenRouterStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterStreamChoice {
    delta: OpenRouterDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
}
