//! `Google Gemini` LLM provider implementation

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

/// `Google Gemini` API provider
pub struct GeminiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl GeminiProvider {
    /// Create a new `Gemini` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("gemini", format!("Failed to create HTTP client: {e}"))
            })?;
        let base_url = "https://generativelanguage.googleapis.com/v1beta".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `Gemini` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("gemini", format!("Failed to create HTTP client: {e}"))
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert `GraphBit` messages to `Gemini` content format
    ///
    /// Gemini API uses a different format than OpenAI:
    /// - Messages are "contents" with "parts"
    /// - System instructions are sent separately via `systemInstruction`
    /// - Roles are "user" and "model" (not "assistant")
    fn convert_messages(messages: &[LlmMessage]) -> (Option<GeminiContent>, Vec<GeminiContent>) {
        let mut system_instruction: Option<GeminiContent> = None;
        let mut contents: Vec<GeminiContent> = Vec::new();

        for message in messages {
            match message.role {
                LlmRole::System => {
                    // Gemini handles system instructions separately
                    system_instruction = Some(GeminiContent {
                        role: None, // system instruction doesn't need a role
                        parts: vec![GeminiPart::Text {
                            text: message.content.clone(),
                        }],
                    });
                }
                LlmRole::User | LlmRole::Tool => {
                    let role = match message.role {
                        LlmRole::Tool => "user".to_string(),
                        _ => "user".to_string(),
                    };

                    // If tool role, wrap content as function response parts
                    let parts = if message.role == LlmRole::Tool {
                        vec![GeminiPart::Text {
                            text: message.content.clone(),
                        }]
                    } else {
                        vec![GeminiPart::Text {
                            text: message.content.clone(),
                        }]
                    };

                    contents.push(GeminiContent {
                        role: Some(role),
                        parts,
                    });
                }
                LlmRole::Assistant => {
                    let mut parts: Vec<GeminiPart> = Vec::new();

                    // Add text content if present
                    if !message.content.is_empty() {
                        parts.push(GeminiPart::Text {
                            text: message.content.clone(),
                        });
                    }

                    // Add function calls if present
                    for tc in &message.tool_calls {
                        parts.push(GeminiPart::FunctionCall {
                            function_call: GeminiFunctionCall {
                                name: tc.name.clone(),
                                args: tc.parameters.clone(),
                            },
                        });
                    }

                    if parts.is_empty() {
                        parts.push(GeminiPart::Text {
                            text: String::new(),
                        });
                    }

                    contents.push(GeminiContent {
                        role: Some("model".to_string()),
                        parts,
                    });
                }
            }
        }

        (system_instruction, contents)
    }

    /// Convert `GraphBit` tools to `Gemini` tool format
    fn convert_tools(tools: &[LlmTool]) -> Vec<GeminiTool> {
        if tools.is_empty() {
            return Vec::new();
        }

        let function_declarations: Vec<GeminiFunctionDeclaration> = tools
            .iter()
            .map(|tool| GeminiFunctionDeclaration {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            })
            .collect();

        vec![GeminiTool {
            function_declarations,
        }]
    }

    /// Parse `Gemini` response to `GraphBit` response
    fn parse_response(&self, response: GeminiResponse) -> GraphBitResult<LlmResponse> {
        let candidate = response
            .candidates
            .and_then(|c| c.into_iter().next())
            .ok_or_else(|| GraphBitError::llm_provider("gemini", "No candidates in response"))?;

        // Extract text content and tool calls from parts
        let mut content = String::new();
        let mut tool_calls: Vec<LlmToolCall> = Vec::new();

        if let Some(parts) = candidate.content.parts {
            for part in parts {
                match part {
                    GeminiResponsePart::Text { text } => {
                        content.push_str(&text);
                    }
                    GeminiResponsePart::FunctionCall { function_call } => {
                        tool_calls.push(LlmToolCall {
                            // Gemini doesn't provide tool call IDs, generate one
                            id: format!("call_{}", uuid::Uuid::new_v4()),
                            name: function_call.name,
                            parameters: function_call.args,
                        });
                    }
                    _ => {} // Ignore other part types
                }
            }
        }

        let finish_reason = match candidate.finish_reason.as_deref() {
            Some("STOP") => FinishReason::Stop,
            Some("MAX_TOKENS") => FinishReason::Length,
            Some("SAFETY") => FinishReason::ContentFilter,
            Some("RECITATION") => FinishReason::ContentFilter,
            Some("MALFORMED_FUNCTION_CALL") => FinishReason::Error,
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

        // Parse usage metadata
        let usage = if let Some(usage_meta) = response.usage_metadata {
            LlmUsage::new(
                usage_meta.prompt_token_count.unwrap_or(0),
                usage_meta.candidates_token_count.unwrap_or(0),
            )
        } else {
            LlmUsage::empty()
        };

        let response_id = response.response_id.unwrap_or_default();

        Ok(LlmResponse::new(content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response_id))
    }
}

#[async_trait]
impl LlmProviderTrait for GeminiProvider {
    fn provider_name(&self) -> &str {
        "gemini"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let (system_instruction, contents) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Build generation config
        let generation_config = GeminiGenerationConfig {
            max_output_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
        };

        let body = GeminiRequest {
            contents,
            system_instruction,
            tools: if tools.is_empty() { None } else { Some(tools) },
            generation_config: Some(generation_config),
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
            .header("Content-Type", "application/json")
            .json(&request_json)
            .send()
            .await
            .map_err(|e| GraphBitError::llm_provider("gemini", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "gemini",
                format!("API error: {error_text}"),
            ));
        }

        let gemini_response: GeminiResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("gemini", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(gemini_response)
    }

    fn supports_function_calling(&self) -> bool {
        // All Gemini models support function calling
        true
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!(
            "{}/models/{}:streamGenerateContent?alt=sse&key={}",
            self.base_url, self.model, self.api_key
        );

        let (system_instruction, contents) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let generation_config = GeminiGenerationConfig {
            max_output_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
        };

        let body = GeminiRequest {
            contents,
            system_instruction,
            tools: if tools.is_empty() { None } else { Some(tools) },
            generation_config: Some(generation_config),
        };

        // Add extra parameters
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

        const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);
        const ERROR_BODY_TIMEOUT: Duration = Duration::from_secs(10);
        const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);

        let response = timeout(
            CONNECTION_TIMEOUT,
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&request_json)
                .send(),
        )
        .await
        .map_err(|_| {
            GraphBitError::llm_provider(
                "gemini",
                format!(
                    "Connection timeout after {:?} - Gemini did not respond. \
                     Check network connectivity and Gemini status.",
                    CONNECTION_TIMEOUT
                ),
            )
        })?
        .map_err(|e| GraphBitError::llm_provider("gemini", format!("Request failed: {e}")))?;

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
                "gemini",
                format!("API error: {error_text}"),
            ));
        }

        // Parse SSE stream - Gemini uses ?alt=sse for Server-Sent Events
        let model = self.model.clone();
        let byte_stream = response.bytes_stream();

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
                    if timeout_occurred {
                        return None;
                    }

                    loop {
                        let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                            Ok(Some(result)) => result,
                            Ok(None) => {
                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "Stream ended with {} total parse errors. Some data may have been lost.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "Stream chunk timeout after {:?} - Gemini stopped responding. \
                                     Response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "gemini",
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
                                        "gemini",
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

                        // Process complete SSE lines
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line: String = buffer.drain(..=newline_pos).collect();
                            let line = line.trim();

                            if line.is_empty() || line.starts_with(':') {
                                continue;
                            }

                            // Gemini SSE format: "data: <json>"
                            if let Some(data) = line.strip_prefix("data: ") {
                                // Parse the Gemini streaming response chunk
                                match serde_json::from_str::<GeminiResponse>(data) {
                                    Ok(gemini_chunk) => {
                                        consecutive_parse_errors = 0;

                                        // Extract text from candidates
                                        if let Some(candidates) = &gemini_chunk.candidates {
                                            if let Some(candidate) = candidates.first() {
                                                if let Some(parts) = &candidate.content.parts {
                                                    for part in parts {
                                                        if let GeminiResponsePart::Text { text } =
                                                            part
                                                        {
                                                            if !text.is_empty() {
                                                                let response_id = gemini_chunk
                                                                    .response_id
                                                                    .clone()
                                                                    .unwrap_or_default();
                                                                let response = LlmResponse::new(
                                                                    text.clone(),
                                                                    &model,
                                                                )
                                                                .with_id(response_id);
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
                                        }
                                    }
                                    Err(e) => {
                                        consecutive_parse_errors += 1;
                                        total_parse_errors += 1;

                                        tracing::warn!(
                                            "Failed to parse Gemini stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS
                                        {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "gemini",
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
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        match self.model.as_str() {
            "gemini-2.5-flash" | "gemini-2.5-flash-preview-05-20" => Some(1_048_576),
            "gemini-2.5-pro" | "gemini-2.5-pro-preview-05-06" => Some(1_048_576),
            "gemini-2.0-flash" | "gemini-2.0-flash-001" => Some(1_048_576),
            "gemini-2.0-flash-lite" | "gemini-2.0-flash-lite-001" => Some(1_048_576),
            "gemini-1.5-pro" | "gemini-1.5-pro-latest" => Some(2_097_152),
            "gemini-1.5-flash" | "gemini-1.5-flash-latest" => Some(1_048_576),
            _ if self.model.starts_with("gemini-2.5-") => Some(1_048_576),
            _ if self.model.starts_with("gemini-2.0-") => Some(1_048_576),
            _ if self.model.starts_with("gemini-1.5-") => Some(1_048_576),
            _ if self.model.starts_with("gemini-3-") => Some(1_048_576),
            _ => None,
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Cost per token in USD (input, output) - as of early 2025
        match self.model.as_str() {
            m if m.starts_with("gemini-2.5-flash") => {
                Some((0.000_000_15, 0.000_000_60)) // $0.15/$0.60 per 1M tokens
            }
            m if m.starts_with("gemini-2.5-pro") => {
                Some((0.000_001_25, 0.000_010_00)) // $1.25/$10.00 per 1M tokens
            }
            m if m.starts_with("gemini-2.0-flash-lite") => {
                Some((0.000_000_075, 0.000_000_30)) // $0.075/$0.30 per 1M tokens
            }
            m if m.starts_with("gemini-2.0-flash") => {
                Some((0.000_000_10, 0.000_000_40)) // $0.10/$0.40 per 1M tokens
            }
            m if m.starts_with("gemini-1.5-pro") => {
                Some((0.000_001_25, 0.000_005_00)) // $1.25/$5.00 per 1M tokens
            }
            m if m.starts_with("gemini-1.5-flash") => {
                Some((0.000_000_075, 0.000_000_30)) // $0.075/$0.30 per 1M tokens
            }
            _ => None,
        }
    }
}

// ============================================================================
// Gemini API Request Types
// ============================================================================

/// Request body for Gemini `generateContent` API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GeminiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
}

/// Content object containing role and parts
#[derive(Debug, Serialize)]
struct GeminiContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    parts: Vec<GeminiPart>,
}

/// Parts of a content message (text, function call, etc.)
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum GeminiPart {
    Text {
        text: String,
    },
    FunctionCall {
        #[serde(rename = "functionCall")]
        function_call: GeminiFunctionCall,
    },
}

/// Function call in a content part
#[derive(Debug, Serialize, Deserialize)]
struct GeminiFunctionCall {
    name: String,
    args: serde_json::Value,
}

/// Tool definition for Gemini API
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiTool {
    function_declarations: Vec<GeminiFunctionDeclaration>,
}

/// Function declaration within a tool
#[derive(Debug, Clone, Serialize)]
struct GeminiFunctionDeclaration {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

/// Generation configuration
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

// ============================================================================
// Gemini API Response Types
// ============================================================================

/// Response from Gemini `generateContent` API
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
    #[serde(default)]
    candidates: Option<Vec<GeminiCandidate>>,
    #[serde(default)]
    usage_metadata: Option<GeminiUsageMetadata>,
    #[serde(default)]
    model_version: Option<String>,
    #[serde(default)]
    response_id: Option<String>,
}

/// Candidate response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiCandidate {
    content: GeminiResponseContent,
    #[serde(default)]
    finish_reason: Option<String>,
}

/// Content in a candidate response
#[derive(Debug, Deserialize)]
struct GeminiResponseContent {
    #[serde(default)]
    parts: Option<Vec<GeminiResponsePart>>,
    #[serde(default)]
    role: Option<String>,
}

/// Parts of a response content
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeminiResponsePart {
    FunctionCall {
        #[serde(rename = "functionCall")]
        function_call: GeminiFunctionCallResponse,
    },
    Text {
        text: String,
    },
    Other(serde_json::Value),
}

/// Function call in response
#[derive(Debug, Deserialize)]
struct GeminiFunctionCallResponse {
    name: String,
    args: serde_json::Value,
}

/// Usage metadata from Gemini API
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiUsageMetadata {
    #[serde(default)]
    prompt_token_count: Option<u32>,
    #[serde(default)]
    candidates_token_count: Option<u32>,
    #[serde(default)]
    total_token_count: Option<u32>,
}
