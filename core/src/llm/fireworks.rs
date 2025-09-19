//! `Fireworks AI` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// `Fireworks AI` API provider
pub struct FireworksProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl FireworksProvider {
    /// Create a new `Fireworks AI` provider
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
                    "fireworks",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://api.fireworks.ai/inference/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `Fireworks AI` provider with custom base URL
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
                    "fireworks",
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

    /// Convert `GraphBit` message to `Fireworks AI` message format
    fn convert_message(&self, message: &LlmMessage) -> FireworksMessage {
        FireworksMessage {
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
                        .map(|tc| FireworksToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: FireworksFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `Fireworks AI` tool format
    fn convert_tool(&self, tool: &LlmTool) -> FireworksTool {
        FireworksTool {
            r#type: "function".to_string(),
            function: FireworksFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `Fireworks AI` response to `GraphBit` response
    fn parse_response(&self, response: FireworksResponse) -> GraphBitResult<LlmResponse> {
        let choice =
            response.choices.into_iter().next().ok_or_else(|| {
                GraphBitError::llm_provider("fireworks", "No choices in response")
            })?;

        let mut content = choice.message.content.unwrap_or_default();
        if content.trim().is_empty()
            && !choice
                .message
                .tool_calls
                .as_ref()
                .unwrap_or(&vec![])
                .is_empty()
        {
            content = "I'll help you with that using the available tools.".to_string();
        }

        let tool_calls = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| {
                // Production-grade argument parsing with error handling
                let parameters = if tc.function.arguments.trim().is_empty() {
                    serde_json::Value::Object(serde_json::Map::new())
                } else {
                    match serde_json::from_str(&tc.function.arguments) {
                        Ok(params) => params,
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse tool call arguments for {}: {e}. Arguments: '{}'",
                                tc.function.name,
                                tc.function.arguments
                            );
                            // Try to create a simple object with the raw arguments
                            serde_json::json!({ "raw_arguments": tc.function.arguments })
                        }
                    }
                };

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
impl LlmProviderTrait for FireworksProvider {
    fn provider_name(&self) -> &str {
        "fireworks"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<FireworksMessage> = request
            .messages
            .iter()
            .map(|m| self.convert_message(m))
            .collect();

        let tools: Option<Vec<FireworksTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(|t| self.convert_tool(t)).collect())
        };

        let body = FireworksRequest {
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
            .map_err(|e| {
                GraphBitError::llm_provider("fireworks", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "fireworks",
                format!("API error: {error_text}"),
            ));
        }

        let fireworks_response: FireworksResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("fireworks", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(fireworks_response)
    }

    fn supports_function_calling(&self) -> bool {
        // Most Fireworks AI models support function calling
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // Common context lengths for popular Fireworks AI models
        match self.model.as_str() {
            m if m.contains("llama-v3p1-8b") => Some(131_072),
            m if m.contains("llama-v3p1-70b") => Some(131_072),
            m if m.contains("llama-v3p1-405b") => Some(131_072),
            m if m.contains("llama-v3-8b") => Some(8192),
            m if m.contains("llama-v3-70b") => Some(8192),
            m if m.contains("mixtral") => Some(32_768),
            m if m.contains("qwen") => Some(32_768),
            _ => None, // Unknown model, let the API handle it
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Approximate costs per token in USD (input, output) for popular models
        // Note: These are estimates and may vary
        match self.model.as_str() {
            m if m.contains("llama-v3p1-8b") => Some((0.000_000_2, 0.000_000_2)),
            m if m.contains("llama-v3p1-70b") => Some((0.000_000_9, 0.000_000_9)),
            m if m.contains("llama-v3p1-405b") => Some((0.000_003, 0.000_003)),
            m if m.contains("mixtral-8x7b") => Some((0.000_000_5, 0.000_000_5)),
            m if m.contains("mixtral-8x22b") => Some((0.000_000_9, 0.000_000_9)),
            _ => None, // Unknown model pricing
        }
    }
}

// `Fireworks AI` API types
#[derive(Debug, Serialize)]
struct FireworksRequest {
    model: String,
    messages: Vec<FireworksMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<FireworksTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FireworksMessage {
    role: String,
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<FireworksToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FireworksToolCall {
    id: String,
    r#type: String,
    function: FireworksFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct FireworksFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct FireworksTool {
    r#type: String,
    function: FireworksFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct FireworksFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct FireworksResponse {
    id: String,
    choices: Vec<FireworksChoice>,
    usage: FireworksUsage,
}

#[derive(Debug, Deserialize)]
struct FireworksChoice {
    message: FireworksMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FireworksUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}
