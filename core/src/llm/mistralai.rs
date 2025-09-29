//! `MistralAI` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
                GraphBitError::llm_provider("mistralai", format!("Failed to create HTTP client: {e}"))
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
                GraphBitError::llm_provider("mistralai", format!("Failed to create HTTP client: {e}"))
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
        MistralAiMessage {
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
                        .map(|tc| MistralAiToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: MistralAiFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
            tool_call_id: None, // Set when role is "tool"
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

        let content = choice.message.content.unwrap_or_default();

        let tool_calls = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| {
                let parameters = serde_json::from_str(&tc.function.arguments).unwrap_or_else(|_| {
                    serde_json::json!({})
                });

                LlmToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    parameters,
                }
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

        let messages: Vec<MistralAiMessage> = request
            .messages
            .iter()
            .map(|m| Self::convert_message(m))
            .collect();

        let tools: Option<Vec<MistralAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(
                request
                    .tools
                    .iter()
                    .map(|t| Self::convert_tool(t))
                    .collect(),
            )
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
            .map_err(|e| GraphBitError::llm_provider("mistralai", format!("Request failed: {e}")))?;

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

    fn max_context_length(&self) -> Option<u32> {
        // Context lengths for different MistralAI models
        match self.model.as_str() {
            "mistral-large-latest" | "mistral-medium-latest" | "mistral-small-latest" => Some(128_000),
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
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<MistralAiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MistralAiToolCall {
    id: String,
    r#type: String,
    function: MistralAiFunction,
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
    object: String,
    created: u64,
    model: String,
    choices: Vec<MistralAiChoice>,
    usage: MistralAiUsage,
}

#[derive(Debug, Deserialize)]
struct MistralAiChoice {
    index: u32,
    message: MistralAiResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MistralAiResponseMessage {
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<MistralAiToolCall>>,
}

#[derive(Debug, Deserialize)]
struct MistralAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}
