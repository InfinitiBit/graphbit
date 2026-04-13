//! `xAI` LLM provider implementation for Grok models

use crate::errors::GraphBitResult;
use crate::llm::openai_compat::complete::execute_complete_request;
use crate::llm::openai_compat::finish_reason::parse_openai_finish_reason;
use crate::llm::openai_compat::http::build_http_client;
use crate::llm::openai_compat::request::build_request_json_with_extra_params;
use crate::llm::openai_compat::response::{
    TOOL_ONLY_FALLBACK_TEXT, fallback_content_if_tool_only, first_choice_or_error, has_tool_calls,
    parse_tool_arguments_openai_style, usage_from_prompt_completion,
};
use crate::llm::openai_compat::advanced_stream::execute_advanced_stream_for_provider;
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall};
use async_trait::async_trait;
use futures::stream::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// `xAI` API provider for Grok models
pub struct XaiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl XaiProvider {
    /// Create a new `xAI` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = build_http_client("xai", None)?;
        let base_url = "https://api.x.ai/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `xAI` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = build_http_client("xai", None)?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert `GraphBit` message to `xAI` message format
    fn convert_message(message: &LlmMessage) -> XaiMessage {
        XaiMessage {
            role: match message.role {
                LlmRole::User => "user".to_string(),
                LlmRole::Assistant => "assistant".to_string(),
                LlmRole::System => "system".to_string(),
                LlmRole::Tool => "tool".to_string(),
            },
            content: Some(message.content.clone()),
            tool_calls: if message.tool_calls.is_empty() {
                None
            } else {
                Some(
                    message
                        .tool_calls
                        .iter()
                        .map(|tc| XaiToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: XaiFunction {
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

    /// Convert `GraphBit` tool to `xAI` tool format
    fn convert_tool(tool: &LlmTool) -> XaiTool {
        XaiTool {
            r#type: "function".to_string(),
            function: XaiFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `xAI` response to `GraphBit` response
    fn parse_response(&self, response: XaiResponse) -> GraphBitResult<LlmResponse> {
        let choice = first_choice_or_error("xai", response.choices)?;
        let content = fallback_content_if_tool_only(
            choice.message.content.unwrap_or_default(),
            has_tool_calls(choice.message.tool_calls.as_ref()),
            TOOL_ONLY_FALLBACK_TEXT,
        );

        let tool_calls: Vec<LlmToolCall> = choice
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
impl LlmProviderTrait for XaiProvider {
    fn provider_name(&self) -> &str {
        "xai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<XaiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<XaiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = XaiRequest {
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
            "xai",
            &self.client,
            &url,
            &self.api_key,
            &body,
            request.extra_params,
            |rb| rb,
            |xai_response: XaiResponse| self.parse_response(xai_response),
        )
        .await
    }

    fn supports_function_calling(&self) -> bool {
        // Most Grok models support function calling
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // Context lengths for Grok models based on xAI documentation
        match self.model.as_str() {
            "grok-4" | "grok-4-0709" => Some(256_000),
            "grok-code-fast-1" => Some(256_000),
            "grok-3" => Some(131_072),
            "grok-3-mini" => Some(131_072),
            "grok-2-vision-1212" => Some(32_768),
            _ => None, // Unknown model, let the API handle it
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Costs per token in USD (input, output) for Grok models
        // Based on xAI pricing documentation
        match self.model.as_str() {
            "grok-4" | "grok-4-0709" => Some((0.000_003, 0.000_015)),
            "grok-code-fast-1" => Some((0.000_000_2, 0.000_001_5)),
            "grok-3" => Some((0.000_003, 0.000_015)),
            "grok-3-mini" => Some((0.000_000_3, 0.000_000_5)),
            "grok-2-vision-1212" => Some((0.000_002, 0.000_010)),
            _ => None, // Unknown model pricing
        }
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<XaiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<XaiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = XaiStreamRequest {
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
            stream_options: Some(XaiStreamOptions {
                include_usage: true,
            }),
        };

        let request_json = build_request_json_with_extra_params("xai", &body, request.extra_params)?;
        execute_advanced_stream_for_provider(
            "xai",
            "xAI",
            &self.client,
            &url,
            &self.api_key,
            &request_json,
            |rb| rb,
            self.model.clone(),
        )
        .await
    }

    fn supports_streaming(&self) -> bool {
        true // xAI supports streaming via OpenAI-compatible API
    }
}

// `xAI` API types
#[derive(Debug, Serialize)]
struct XaiRequest {
    model: String,
    messages: Vec<XaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<XaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiMessage {
    role: String,
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<XaiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiToolCall {
    id: String,
    r#type: String,
    function: XaiFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct XaiTool {
    r#type: String,
    function: XaiFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct XaiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct XaiResponse {
    id: String,
    choices: Vec<XaiChoice>,
    usage: XaiUsage,
}

#[derive(Debug, Deserialize)]
struct XaiChoice {
    message: XaiMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct XaiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

// Streaming-specific types (OpenAI-compatible format)

/// Request body for streaming API calls (includes stream: true)
#[derive(Debug, Serialize)]
struct XaiStreamRequest {
    model: String,
    messages: Vec<XaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<XaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<XaiStreamOptions>,
}

#[derive(Debug, Serialize)]
struct XaiStreamOptions {
    include_usage: bool,
}
