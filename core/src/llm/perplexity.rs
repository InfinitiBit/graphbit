//! `Perplexity` AI LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
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
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "perplexity",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
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
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "perplexity",
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
        let choice =
            response.choices.into_iter().next().ok_or_else(|| {
                GraphBitError::llm_provider("perplexity", "No choices in response")
            })?;

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
impl LlmProviderTrait for PerplexityProvider {
    fn provider_name(&self) -> &str {
        "perplexity"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<PerplexityMessage> = request
            .messages
            .iter()
            .map(|m| Self::convert_message(m))
            .collect();

        let tools: Option<Vec<PerplexityTool>> = if request.tools.is_empty() {
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
            .header("Accept", "application/json")
            .json(&request_json)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("perplexity", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "perplexity",
                format!("API error: {}", error_text),
            ));
        }

        let perplexity_response: PerplexityResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("perplexity", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(perplexity_response)
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
