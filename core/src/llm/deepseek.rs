//! `DeepSeek` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
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
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10) // Increased connection pool size
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "deepseek",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
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
        // Use same optimized client settings
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "deepseek",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert `GraphBit` message to `DeepSeek` message format
    fn convert_message(&self, message: &LlmMessage) -> DeepSeekMessage {
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
        }
    }

    /// Convert `GraphBit` tool to `DeepSeek` tool format
    fn convert_tool(&self, tool: &LlmTool) -> DeepSeekTool {
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
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| GraphBitError::llm_provider("deepseek", "No choices in response"))?;

        let content = choice.message.content;
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
impl LlmProviderTrait for DeepSeekProvider {
    fn provider_name(&self) -> &str {
        "deepseek"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<DeepSeekMessage> = request
            .messages
            .iter()
            .map(|m| self.convert_message(m))
            .collect();

        let tools: Option<Vec<DeepSeekTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(|t| self.convert_tool(t)).collect())
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
            .map_err(|e| GraphBitError::llm_provider("deepseek", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "deepseek",
                format!("API error: {error_text}"),
            ));
        }

        let deepseek_response: DeepSeekResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("deepseek", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(deepseek_response)
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
