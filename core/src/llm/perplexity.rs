//! `Perplexity` AI LLM provider implementation

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
        let client = build_http_client("perplexity", None)?;
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
        let client = build_http_client("perplexity", None)?;

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
            tool_call_id: message.tool_call_id.clone(),
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
        let choice = first_choice_or_error("perplexity", response.choices)?;

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

        execute_complete_request(
            "perplexity",
            &self.client,
            &url,
            &self.api_key,
            &body,
            request.extra_params,
            |rb| rb.header("Accept", "application/json"),
            |perplexity_response: PerplexityResponse| self.parse_response(perplexity_response),
        )
        .await
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

        let request_json =
            build_request_json_with_extra_params("perplexity", &body, request.extra_params)?;
        execute_openai_style_text_stream(
            "perplexity",
            "Perplexity",
            &self.client,
            &url,
            &self.api_key,
            &request_json,
            |rb| rb,
            self.model.clone(),
            true,
            extract_perplexity_stream_text,
        )
        .await
    }

    fn supports_streaming(&self) -> bool {
        true // Perplexity supports streaming via OpenAI-compatible API
    }
}

fn extract_perplexity_stream_text(chunk: &PerplexityStreamChunk) -> Option<(String, String)> {
    chunk
        .choices
        .first()
        .and_then(|choice| choice.delta.content.as_ref())
        .filter(|content| !content.is_empty())
        .map(|content| (chunk.id.clone(), content.clone()))
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
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
