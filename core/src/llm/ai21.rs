//! `AI21` LLM provider implementation (Jamba / Chat + function calling)

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
use serde::{Deserialize, Deserializer, Serialize};
use std::time::Duration;
use tokio::time::timeout;

/// `AI21` (Jamba / chat) API provider
pub struct Ai21Provider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
    organization: Option<String>,
}

impl Ai21Provider {
    /// Create a new `AI21` Provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = build_http_client("ai21", None)?;
        // Base URL for AI21 chat API (Jamba)
        let base_url = "https://api.ai21.com/studio/v1".to_string();
        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Create a new `AI21` provider with custom base url
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = build_http_client("ai21", None)?;
        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Create a new `AI21` provider with custom organization
    pub fn with_organization(mut self, org: String) -> Self {
        self.organization = Some(org);
        self
    }

    /// Convert your internal message format to AI21’s chat message format
    fn convert_message(message: &LlmMessage) -> Ai21Message {
        Ai21Message {
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
                        .map(|tc| Ai21ToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: Ai21Function {
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

    /// Convert your internal tool definition to AI21’s tool schema
    fn convert_tool(tool: &LlmTool) -> Ai21Tool {
        Ai21Tool {
            r#type: "function".to_string(),
            function: Ai21FunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse the AI21 response into your internal `LlmResponse`
    fn parse_response(&self, resp: Ai21Response) -> GraphBitResult<LlmResponse> {
        let choice = first_choice_or_error("ai21", resp.choices)?;
        let content = fallback_content_if_tool_only(
            choice.message.content,
            has_tool_calls(choice.message.tool_calls.as_ref()),
            TOOL_ONLY_FALLBACK_TEXT,
        );

        let tool_calls = choice
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

        let usage = usage_from_prompt_completion(resp.usage.prompt_tokens, resp.usage.completion_tokens);

        Ok(LlmResponse::new(content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(resp.id))
    }
}

#[async_trait]
impl LlmProviderTrait for Ai21Provider {
    fn provider_name(&self) -> &str {
        "ai21"
    }
    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<Ai21Message> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<Ai21Tool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = Ai21Request {
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

        let organization = self.organization.clone();
        execute_complete_request(
            "ai21",
            &self.client,
            &url,
            &self.api_key,
            &body,
            request.extra_params,
            move |rb| {
                if let Some(org) = organization {
                    rb.header("Ai21-Organization", org)
                } else {
                    rb
                }
            },
            |ai21_resp: Ai21Response| self.parse_response(ai21_resp),
        )
        .await
    }

    fn supports_streaming(&self) -> bool {
        true // AI21 supports streaming via OpenAI-compatible SSE
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<Ai21Message> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<Ai21Tool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = Ai21StreamRequest {
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

        let request_json =
            build_request_json_with_extra_params("ai21", &body, request.extra_params)?;

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

        if let Some(org) = &self.organization {
            builder = builder.header("Ai21-Organization", org);
        }

        // Apply timeout to initial connection
        let response = timeout(CONNECTION_TIMEOUT, builder.send())
            .await
            .map_err(|_| {
                GraphBitError::llm_provider(
                    "ai21",
                    format!(
                        "Connection timeout after {:?} - AI21 did not respond. \
                         Check network connectivity and AI21 status.",
                        CONNECTION_TIMEOUT
                    ),
                )
            })?
            .map_err(|e| GraphBitError::llm_provider("ai21", format!("Request failed: {e}")))?;

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
                "ai21",
                format!("API error: {error_text}"),
            ));
        }

        // Parse SSE stream (OpenAI-compatible format: "data: " prefix, "[DONE]" terminator)
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
                            if line.is_empty() || line.starts_with(':') {
                                continue;
                            }

                            // Check for data: prefix (OpenAI-compatible SSE)
                            if let Some(data) = line.strip_prefix("data: ") {
                                // Check for [DONE] marker
                                if data.trim() == "[DONE]" {
                                    if total_parse_errors > 0 {
                                        tracing::warn!(
                                            "AI21 stream completed with {} total parse errors.",
                                            total_parse_errors
                                        );
                                    }
                                    return None;
                                }

                                // Parse JSON chunk
                                match serde_json::from_str::<Ai21StreamChunk>(data) {
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
                                            "Failed to parse AI21 stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                            consecutive_parse_errors,
                                            total_parse_errors,
                                            e,
                                            if data.len() > 200 { &data[..200] } else { data }
                                        );

                                        if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS
                                        {
                                            return Some((
                                                Err(GraphBitError::llm_provider(
                                                    "ai21",
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

                        // Need more data from the network
                        let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                            Ok(Some(result)) => result,
                            Ok(None) => {
                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "AI21 stream ended with {} total parse errors.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "AI21 stream chunk timeout after {:?} - response may be incomplete.",
                                    CHUNK_TIMEOUT
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        "ai21",
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
                                        "ai21",
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
        // AI21’s chat/Jamba models support function calling (tools) per their docs. :contentReference[oaicite:5]{index=5}
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // You should check AI21’s model docs for the exact context length
        // Placeholder: assume 8192 (you should adjust)
        // Context lengths for AI21 models based on their documentation
        match self.model.as_str() {
            "jamba-mini" | "jamba-large" => Some(256_000),
            _ => None, // Unknown model, let the API handle it
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // AI21’s pricing would have to be fetched from their docs. For now, None.
        // AI21's pricing based on their documentation
        // Returns (input_cost_per_token, output_cost_per_token)
        match self.model.as_str() {
            "jamba-mini" => Some((0.000_000_2, 0.000_000_4)), // $0.2/M input, $0.4/M output
            "jamba-large" => Some((0.000_002, 0.000_008)),    // $2/M input, $8/M output
            _ => None,                                        // Unknown model, no pricing info
        }
    }
}

// Types reflecting AI21’s chat API
#[derive(Debug, Serialize)]
struct Ai21Request {
    model: String,
    messages: Vec<Ai21Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Ai21Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ai21Message {
    role: String,
    #[serde(deserialize_with = "deserialize_nullable_content")]
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<Ai21ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ai21ToolCall {
    id: String,
    r#type: String,
    function: Ai21Function,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ai21Function {
    name: String,
    arguments: String,
}

#[derive(Debug, Serialize, Clone)]
struct Ai21Tool {
    r#type: String,
    function: Ai21FunctionDef,
}

#[derive(Debug, Serialize, Clone)]
struct Ai21FunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct Ai21Response {
    id: String,
    choices: Vec<Ai21Choice>,
    usage: Ai21Usage,
}

#[derive(Debug, Deserialize)]
struct Ai21Choice {
    message: Ai21Message,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Ai21Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

/// Same as in openai.rs: AI21 returns `null` for content when tool calls are made
fn deserialize_nullable_content<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

// Streaming-specific types (OpenAI-compatible SSE format)

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct Ai21StreamRequest {
    model: String,
    messages: Vec<Ai21Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Ai21Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Streaming chunk from AI21 API (OpenAI-compatible format)
#[derive(Debug, Deserialize)]
struct Ai21StreamChunk {
    id: String,
    choices: Vec<Ai21StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct Ai21StreamChoice {
    delta: Ai21Delta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Ai21Delta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
}
