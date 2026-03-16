//! `Ollama` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::timeout;

/// `Ollama` API provider with performance optimizations
pub struct OllamaProvider {
    client: Client,
    model: String,
    base_url: String,
    /// Cache to avoid repeated model availability checks
    model_verified: Arc<RwLock<bool>>,
}

impl OllamaProvider {
    /// Create a new `Ollama` provider
    pub fn new(model: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120)) // Reasonable timeout
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("ollama", format!("Failed to create HTTP client: {e}"))
            })?;
        let base_url = "http://localhost:11434".to_string();

        Ok(Self {
            client,
            model,
            base_url,
            model_verified: Arc::new(RwLock::new(false)),
        })
    }

    /// Create a new `Ollama` provider with custom base URL
    pub fn with_base_url(model: String, base_url: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("ollama", format!("Failed to create HTTP client: {e}"))
            })?;

        Ok(Self {
            client,
            model,
            base_url,
            model_verified: Arc::new(RwLock::new(false)),
        })
    }

    /// Convert `GraphBit` message to `Ollama` message format
    fn convert_message(message: &LlmMessage) -> OllamaMessage {
        OllamaMessage {
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
                        .map(|tc| OllamaToolCall {
                            function: OllamaFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.clone(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `Ollama` tool format
    fn convert_tool(tool: &LlmTool) -> OllamaTool {
        OllamaTool {
            r#type: "function".to_string(),
            function: OllamaFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `Ollama` response to `GraphBit` response
    fn parse_response(&self, response: OllamaResponse) -> GraphBitResult<LlmResponse> {
        let content = response.message.content;
        let tool_calls = response
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| LlmToolCall {
                id: format!("ollama_{}", uuid::Uuid::new_v4()),
                name: tc.function.name,
                parameters: tc.function.arguments,
            })
            .collect();

        // `Ollama` uses "stop" for completion
        let finish_reason = if response.done {
            FinishReason::Stop
        } else {
            FinishReason::Length
        };

        // `Ollama` doesn't provide detailed token usage, so we estimate
        let prompt_tokens = response.prompt_eval_count.unwrap_or(0);
        let completion_tokens = response.eval_count.unwrap_or(0);
        let usage = LlmUsage::new(prompt_tokens, completion_tokens);

        Ok(LlmResponse::new(content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(format!("ollama_{}", uuid::Uuid::new_v4())))
    }

    /// Check if `Ollama` is available
    pub async fn check_availability(&self) -> GraphBitResult<bool> {
        let url = format!("{}/api/tags", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// List available models
    pub async fn list_models(&self) -> GraphBitResult<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);

        let response = self.client.get(&url).send().await.map_err(|e| {
            GraphBitError::llm_provider(
                "ollama",
                format!(
                    "Failed to fetch models: {e}. Make sure Ollama is running on {}",
                    self.base_url
                ),
            )
        })?;

        if !response.status().is_success() {
            return Err(GraphBitError::llm_provider(
                "ollama",
                format!("Failed to fetch models: HTTP {}", response.status()),
            ));
        }

        let models_response: OllamaModelsResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("ollama", format!("Failed to parse models response: {e}"))
        })?;

        Ok(models_response.models.into_iter().map(|m| m.name).collect())
    }

    /// Pull a model if it doesn't exist - OPTIMIZED VERSION
    pub async fn ensure_model(&self) -> GraphBitResult<()> {
        // Fast path: check cache first to avoid repeated API calls
        {
            let verified = self.model_verified.read().await;
            if *verified {
                return Ok(());
            }
        }

        // Check if model exists (only if not cached)
        let models = self.list_models().await?;
        if models.iter().any(|m| m == &self.model) {
            // Cache the result to avoid future checks
            let mut verified = self.model_verified.write().await;
            *verified = true;
            return Ok(());
        }

        // Pull the model
        let url = format!("{}/api/pull", self.base_url);
        let pull_request = OllamaPullRequest {
            name: self.model.clone(),
        };

        let response = self
            .client
            .post(&url)
            .json(&pull_request)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "ollama",
                    format!("Failed to pull model '{}': {e}", self.model),
                )
            })?;

        if !response.status().is_success() {
            return Err(GraphBitError::llm_provider(
                "ollama",
                format!(
                    "Failed to pull model '{}': HTTP {}",
                    self.model,
                    response.status()
                ),
            ));
        }

        // Cache successful model verification
        let mut verified = self.model_verified.write().await;
        *verified = true;

        Ok(())
    }
}

#[async_trait]
impl LlmProviderTrait for OllamaProvider {
    fn provider_name(&self) -> &str {
        "ollama"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        // PERFORMANCE OPTIMIZATION: Only ensure model on first call, not every call
        // Check cache first before making expensive API calls
        {
            let verified = self.model_verified.read().await;
            if !*verified {
                // Only do the expensive model check if we haven't verified before
                drop(verified); // Release read lock before acquiring write lock
                self.ensure_model().await?;
            }
        }

        let url = format!("{}/api/chat", self.base_url);

        let messages: Vec<OllamaMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OllamaTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let mut options = serde_json::Map::new();

        if let Some(temp) = request.temperature {
            options.insert(
                "temperature".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(temp as f64).unwrap()),
            );
        }

        if let Some(top_p) = request.top_p {
            options.insert(
                "top_p".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(top_p as f64).unwrap()),
            );
        }

        // Add extra parameters to options
        for (key, value) in request.extra_params {
            options.insert(key, value);
        }

        let body = OllamaRequest {
            model: self.model.clone(),
            messages,
            tools,
            stream: false,
            options: if options.is_empty() {
                None
            } else {
                Some(serde_json::Value::Object(options))
            },
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "ollama",
                    format!(
                        "Request failed: {e}. Make sure Ollama is running on {}",
                        self.base_url
                    ),
                )
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(GraphBitError::llm_provider(
                "ollama",
                format!("HTTP {status}: {error_text}"),
            ));
        }

        let ollama_response: OllamaResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("ollama", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(ollama_response)
    }

    fn supports_function_calling(&self) -> bool {
        // `Ollama` supports function calling for some models
        true
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        // Ensure model is available (cached after first check)
        {
            let verified = self.model_verified.read().await;
            if !*verified {
                drop(verified);
                self.ensure_model().await?;
            }
        }

        let url = format!("{}/api/chat", self.base_url);

        let messages: Vec<OllamaMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OllamaTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let mut options = serde_json::Map::new();

        if let Some(temp) = request.temperature {
            options.insert(
                "temperature".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(temp as f64).unwrap()),
            );
        }

        if let Some(top_p) = request.top_p {
            options.insert(
                "top_p".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(top_p as f64).unwrap()),
            );
        }

        // Add extra parameters to options
        for (key, value) in request.extra_params {
            options.insert(key, value);
        }

        let body = OllamaRequest {
            model: self.model.clone(),
            messages,
            tools,
            stream: true, // Enable streaming
            options: if options.is_empty() {
                None
            } else {
                Some(serde_json::Value::Object(options))
            },
        };

        // Timeout constants
        const CONNECTION_TIMEOUT: Duration = Duration::from_secs(120);
        const ERROR_BODY_TIMEOUT: Duration = Duration::from_secs(10);
        const CHUNK_TIMEOUT: Duration = Duration::from_secs(60); // Longer for local inference

        // Apply timeout to initial connection
        let response = timeout(
            CONNECTION_TIMEOUT,
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&body)
                .send(),
        )
        .await
        .map_err(|_| {
            GraphBitError::llm_provider(
                "ollama",
                format!(
                    "Connection timeout after {:?} - Ollama did not respond. \
                     Make sure Ollama is running on {}",
                    CONNECTION_TIMEOUT, self.base_url
                ),
            )
        })?
        .map_err(|e| {
            GraphBitError::llm_provider(
                "ollama",
                format!(
                    "Request failed: {e}. Make sure Ollama is running on {}",
                    self.base_url
                ),
            )
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
                "ollama",
                format!("HTTP error: {error_text}"),
            ));
        }

        // Parse NDJSON stream (newline-delimited JSON - Ollama's native format)
        // Unlike OpenAI/Perplexity SSE, Ollama streams plain JSON objects separated by newlines
        let model = self.model.clone();
        let byte_stream = response.bytes_stream();

        // State: (byte_stream, buffer, done, consecutive_parse_errors, total_parse_errors)
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
                    // If stream is already done, stop
                    if done {
                        return None;
                    }

                    loop {
                        // First, try to process any complete lines already in the buffer
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            // Skip empty lines
                            if line.is_empty() {
                                continue;
                            }

                            // Parse the JSON chunk (Ollama NDJSON - no "data: " prefix)
                            match serde_json::from_str::<OllamaStreamResponse>(line) {
                                Ok(chunk) => {
                                    consecutive_parse_errors = 0;

                                    // Check if this is the final chunk
                                    if chunk.done {
                                        if total_parse_errors > 0 {
                                            tracing::warn!(
                                                "Ollama stream completed with {} total parse errors.",
                                                total_parse_errors
                                            );
                                        }
                                        return None; // End of stream
                                    }

                                    // Yield non-empty content chunks
                                    let content = &chunk.message.content;
                                    if !content.is_empty() {
                                        let response = LlmResponse::new(content.clone(), &model)
                                            .with_id(format!("ollama_{}", uuid::Uuid::new_v4()));
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
                                        "Failed to parse Ollama stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                        consecutive_parse_errors,
                                        total_parse_errors,
                                        e,
                                        if line.len() > 200 { &line[..200] } else { line }
                                    );

                                    if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS {
                                        return Some((
                                            Err(GraphBitError::llm_provider(
                                                "ollama",
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

                        // Need more data from the network
                        let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                            Ok(Some(result)) => result,
                            Ok(None) => {
                                // Stream naturally ended without done:true
                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "Ollama stream ended with {} total parse errors.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "Ollama stream chunk timeout after {:?} - response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "ollama",
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
                                        "ollama",
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
        // Context length varies by model, common defaults
        match self.model.as_str() {
            m if m.contains("llama3") => Some(8192),
            m if m.contains("llama2") => Some(4096),
            m if m.contains("codellama") => Some(16_384),
            m if m.contains("mixtral") => Some(32_768),
            m if m.contains("gemma") => Some(8192),
            _ => Some(4096), // Conservative default
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // `Ollama` is typically free for local usage
        Some((0.0, 0.0))
    }
}

// `Ollama` API request structures
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OllamaTool>>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OllamaToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaToolCall {
    function: OllamaFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaFunction {
    name: String,
    arguments: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct OllamaTool {
    r#type: String,
    function: OllamaFunctionDef,
}

#[derive(Debug, Serialize)]
struct OllamaFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

/// Streaming response chunk from Ollama (NDJSON format)
/// Same structure as OllamaResponse but with relaxed field requirements
#[derive(Debug, Deserialize)]
struct OllamaStreamResponse {
    message: OllamaStreamMessage,
    done: bool,
}

/// Message within a streaming chunk
#[derive(Debug, Deserialize)]
struct OllamaStreamMessage {
    #[serde(default)]
    content: String,
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
}

#[derive(Debug, Serialize)]
struct OllamaPullRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct OllamaModelsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}
