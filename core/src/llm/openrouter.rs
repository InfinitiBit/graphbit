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
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

        let messages: Vec<OpenRouterMessage> = request
            .messages
            .iter()
            .map(|m| Self::convert_message(m))
            .collect();

        let tools: Option<Vec<OpenRouterTool>> = if request.tools.is_empty() {
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
