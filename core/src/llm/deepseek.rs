//! `DeepSeek` LLM provider implementation

use crate::errors::GraphBitResult;
use crate::llm::openai_compat::complete::execute_complete_request;
use crate::llm::openai_compat::finish_reason::parse_openai_finish_reason;
use crate::llm::openai_compat::http::build_http_client;
use crate::llm::openai_compat::request::build_request_json_with_extra_params;
use crate::llm::openai_compat::response::{
    TOOL_ONLY_FALLBACK_TEXT, fallback_content_if_tool_only, first_choice_or_error, has_tool_calls,
    parse_tool_arguments_openai_style, usage_from_prompt_completion,
};
use crate::llm::openai_compat::simple_stream::execute_openai_style_text_stream;
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall};
use async_trait::async_trait;
use futures::stream::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
        let client = build_http_client("deepseek", None)?;
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
        let client = build_http_client("deepseek", None)?;

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
            tool_call_id: message.tool_call_id.clone(),
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
        let choice = first_choice_or_error("deepseek", response.choices)?;

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

        execute_complete_request(
            "deepseek",
            &self.client,
            &url,
            &self.api_key,
            &body,
            request.extra_params,
            |rb| rb,
            |deepseek_response: DeepSeekResponse| self.parse_response(deepseek_response),
        )
        .await
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

        let request_json =
            build_request_json_with_extra_params("deepseek", &body, request.extra_params)?;
        execute_openai_style_text_stream(
            "deepseek",
            "DeepSeek",
            &self.client,
            &url,
            &self.api_key,
            &request_json,
            |rb| rb,
            self.model.clone(),
            true,
            extract_deepseek_stream_text,
        )
        .await
    }

    fn supports_streaming(&self) -> bool {
        true // DeepSeek supports streaming via OpenAI-compatible API
    }
}

fn extract_deepseek_stream_text(chunk: &DeepSeekStreamChunk) -> Option<(String, String)> {
    chunk
        .choices
        .first()
        .and_then(|choice| choice.delta.content.as_ref())
        .filter(|content| !content.is_empty())
        .map(|content| (chunk.id.clone(), content.clone()))
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
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
