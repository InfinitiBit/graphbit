//! `OpenAI` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
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
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10) // Increased connection pool size
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("openai", format!("Failed to create HTTP client: {e}"))
            })?;
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
        // Use same optimized client settings
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("openai", format!("Failed to create HTTP client: {e}"))
            })?;

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
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| GraphBitError::llm_provider("openai", "No choices in response"))?;

        let mut content = choice.message.content;
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
impl LlmProviderTrait for OpenAiProvider {
    fn provider_name(&self) -> &str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenAiMessage> = request
            .messages
            .iter()
            .map(|m| Self::convert_message(m))
            .collect();

        let tools: Option<Vec<OpenAiTool>> = if request.tools.is_empty() {
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

        let body = OpenAiRequest {
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

        let mut req_builder = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_json);

        if let Some(org) = &self.organization {
            req_builder = req_builder.header("OpenAI-Organization", org);
        }

        let response = req_builder
            .send()
            .await
            .map_err(|e| GraphBitError::llm_provider("openai", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "openai",
                format!("API error: {error_text}"),
            ));
        }

        let openai_response: OpenAiResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("openai", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(openai_response)
    }

    fn supports_function_calling(&self) -> bool {
        // Most `OpenAI` models support function calling
        matches!(
            self.model.as_str(),
            "gpt-4" | "gpt-4-turbo" | "gpt-3.5-turbo" | "gpt-4o" | "gpt-4o-mini"
        ) || self.model.starts_with("gpt-4")
            || self.model.starts_with("gpt-3.5-turbo")
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
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    #[serde(deserialize_with = "deserialize_nullable_content")]
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAiToolCall>>,
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
