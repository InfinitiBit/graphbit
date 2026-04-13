//! `OpenAI` LLM provider implementation

use crate::errors::GraphBitResult;
use crate::llm::openai_compat::advanced_stream::{
    execute_openai_advanced_stream,
};
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
use futures::stream::Stream;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};

/// `OpenAI` API provider
pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
    organization: Option<String>,
}

impl OpenAiProvider {
    /// Create a new `OpenAI` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = build_http_client("openai", None)?;
        let base_url = "https://api.openai.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Create a new `OpenAI` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = build_http_client("openai", None)?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Set organization ID
    pub fn with_organization(mut self, organization: String) -> Self {
        self.organization = Some(organization);
        self
    }

    /// Convert `GraphBit` message to `OpenAI` message format
    fn convert_message(message: &LlmMessage) -> OpenAiMessage {
        OpenAiMessage {
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
                        .map(|tc| OpenAiToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: OpenAiFunction {
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

    /// Convert `GraphBit` tool to `OpenAI` tool format
    fn convert_tool(tool: &LlmTool) -> OpenAiTool {
        OpenAiTool {
            r#type: "function".to_string(),
            function: OpenAiFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `OpenAI` response to `GraphBit` response
    fn parse_response(&self, response: OpenAiResponse) -> GraphBitResult<LlmResponse> {
        let choice = first_choice_or_error("openai", response.choices)?;
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
impl LlmProviderTrait for OpenAiProvider {
    fn provider_name(&self) -> &str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenAiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OpenAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = OpenAiRequest {
            model: self.model.clone(),
            messages,
            max_completion_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: tools.clone(),
            tool_choice: if tools.is_some() {
                Some("auto".to_string())
            } else {
                None
            },
            stream: None, // Disable streaming for complete method
            stream_options: None,
        };

        let organization = self.organization.clone();
        execute_complete_request(
            "openai",
            &self.client,
            &url,
            &self.api_key,
            &body,
            request.extra_params,
            move |rb| {
                if let Some(org) = organization {
                    rb.header("OpenAI-Organization", org)
                } else {
                    rb
                }
            },
            |openai_response: OpenAiResponse| self.parse_response(openai_response),
        )
        .await
    }

    fn supports_function_calling(&self) -> bool {
        // Most `OpenAI` models support function calling
        matches!(
            self.model.as_str(),
            "gpt-4" | "gpt-4-turbo" | "gpt-3.5-turbo" | "gpt-4o" | "gpt-4o-mini"
        ) || self.model.starts_with("gpt-4")
            || self.model.starts_with("gpt-3.5-turbo")
    }

    async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenAiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OpenAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = OpenAiRequest {
            model: self.model.clone(),
            messages,
            max_completion_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: tools.clone(),
            tool_choice: if tools.is_some() {
                Some("auto".to_string())
            } else {
                None
            },
            stream: Some(true), // Enable streaming
            // Request terminal usage chunk so workflow metadata token usage matches non-streaming.
            stream_options: Some(OpenAiStreamOptions {
                include_usage: true,
            }),
        };

        let request_json =
            build_request_json_with_extra_params("openai", &body, request.extra_params)?;
        let organization = self.organization.clone();
        execute_openai_advanced_stream(
            &self.client,
            &url,
            &self.api_key,
            &request_json,
            move |rb| {
                if let Some(org) = organization {
                    rb.header("OpenAI-Organization", org)
                } else {
                    rb
                }
            },
            self.model.clone(),
        )
        .await
    }

    fn supports_streaming(&self) -> bool {
        true // OpenAI supports streaming
    }

    fn max_context_length(&self) -> Option<u32> {
        match self.model.as_str() {
            "gpt-4" => Some(8192),
            "gpt-4-32k" => Some(32_768),
            "gpt-4-turbo" => Some(128_000),
            "gpt-4o" => Some(128_000),
            "gpt-4o-mini" => Some(128_000),
            "gpt-3.5-turbo" => Some(4096),
            "gpt-3.5-turbo-16k" => Some(16_384),
            _ => None,
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Cost per token in USD (input, output) as of late 2023
        match self.model.as_str() {
            "gpt-4" => Some((0.000_03, 0.000_06)),
            "gpt-4-32k" => Some((0.000_06, 0.000_12)),
            "gpt-4-turbo" => Some((0.000_01, 0.000_03)),
            "gpt-4o" => Some((0.000_005, 0.000_015)),
            "gpt-4o-mini" => Some((0.000_000_15, 0.000_000_6)),
            "gpt-3.5-turbo" => Some((0.000_001_5, 0.000_002)),
            "gpt-3.5-turbo-16k" => Some((0.000_003, 0.000_004)),
            _ => None,
        }
    }
}

// `OpenAI` API types
#[derive(Debug, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<OpenAiStreamOptions>,
}

#[derive(Debug, Serialize)]
struct OpenAiStreamOptions {
    include_usage: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    #[serde(deserialize_with = "deserialize_nullable_content")]
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiToolCall {
    id: String,
    r#type: String,
    function: OpenAiFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiTool {
    r#type: String,
    function: OpenAiFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    id: String,
    choices: Vec<OpenAiChoice>,
    usage: OpenAiUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

/// Custom deserializer for nullable content field
/// `OpenAI` returns null for content when tool calls are made
fn deserialize_nullable_content<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::openai_compat::advanced_stream::{
        OpenAiStreamChunk, merge_openai_stream_tool_deltas, tool_accum_map_to_llm_calls,
    };
    use std::collections::HashMap;

    #[test]
    fn stream_usage_chunk_parses_with_empty_choices() {
        let chunk = r#"{
            "id":"chatcmpl-test",
            "object":"chat.completion.chunk",
            "created":1775204971,
            "model":"gpt-4o-mini-2024-07-18",
            "choices":[],
            "usage":{"prompt_tokens":15,"completion_tokens":14,"total_tokens":29}
        }"#;

        let parsed: OpenAiStreamChunk =
            serde_json::from_str(chunk).expect("usage chunk should parse");
        assert!(parsed.choices.is_empty());
        let usage = parsed.usage.expect("usage should be present");
        assert_eq!(usage.prompt_tokens, 15);
        assert_eq!(usage.completion_tokens, 14);
    }

    #[test]
    fn stream_tool_call_deltas_merge_to_llm_calls() {
        let chunk1: OpenAiStreamChunk = serde_json::from_value(serde_json::json!({
            "id": "chatcmpl-toolstream",
            "choices": [{
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "id": "call_abc",
                        "type": "function",
                        "function": { "name": "add", "arguments": "" }
                    }]
                }
            }]
        }))
        .expect("chunk1");

        let chunk2: OpenAiStreamChunk = serde_json::from_value(serde_json::json!({
            "id": "chatcmpl-toolstream",
            "choices": [{
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "function": { "arguments": "{\"a\": 10, \"b\": 20}" }
                    }]
                },
                "finish_reason": "tool_calls"
            }]
        }))
        .expect("chunk2");

        let mut acc = HashMap::new();
        merge_openai_stream_tool_deltas(&mut acc, &chunk1.choices[0].delta.tool_calls);
        merge_openai_stream_tool_deltas(&mut acc, &chunk2.choices[0].delta.tool_calls);
        let calls = tool_accum_map_to_llm_calls(&acc);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id, "call_abc");
        assert_eq!(calls[0].name, "add");
        assert_eq!(calls[0].parameters["a"], 10);
        assert_eq!(calls[0].parameters["b"], 20);
    }

    #[test]
    fn stream_request_serializes_include_usage_option() {
        let req = OpenAiRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![OpenAiMessage {
                role: "user".to_string(),
                content: "hello".to_string(),
                tool_calls: None,
                tool_call_id: None,
            }],
            max_completion_tokens: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            stream: Some(true),
            stream_options: Some(OpenAiStreamOptions {
                include_usage: true,
            }),
        };

        let value = serde_json::to_value(req).expect("request should serialize");
        assert_eq!(value.get("stream").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            value
                .get("stream_options")
                .and_then(|v| v.get("include_usage"))
                .and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[test]
    fn fallback_json_usage_extraction_shape_is_supported() {
        let chunk = serde_json::json!({
            "id": "chatcmpl-fallback",
            "object": "chat.completion.chunk",
            // Intentionally omit "choices" to mimic a schema variant that would fail typed parsing.
            "usage": {
                "prompt_tokens": 21,
                "completion_tokens": 9,
                "total_tokens": 30
            }
        });

        let prompt_tokens = chunk
            .get("usage")
            .and_then(|u| u.get("prompt_tokens"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let completion_tokens = chunk
            .get("usage")
            .and_then(|u| u.get("completion_tokens"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        assert_eq!(prompt_tokens, 21);
        assert_eq!(completion_tokens, 9);
    }
}
