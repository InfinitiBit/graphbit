//! `ByteDance ModelArk` LLM provider implementation
//!
//! `ByteDance ModelArk` provides access to various LLM models through ByteDance's ModelArk platform.
//! It uses an OpenAI-compatible API format for easy integration.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// `ByteDance ModelArk` API provider
pub struct ByteDanceProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl ByteDanceProvider {
    /// Create a new `ByteDance ModelArk` provider
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
                    "bytedance",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://ark.ap-southeast.bytepluses.com/api/v3".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `ByteDance ModelArk` provider with custom base URL
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
                    "bytedance",
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

    /// Convert `GraphBit` message to `ByteDance` message format
    fn convert_message(message: &LlmMessage) -> ByteDanceMessage {
        ByteDanceMessage {
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
                        .map(|tc| ByteDanceToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: ByteDanceFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `ByteDance` tool format
    fn convert_tool(tool: &LlmTool) -> ByteDanceTool {
        ByteDanceTool {
            r#type: "function".to_string(),
            function: ByteDanceFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `ByteDance` response to `GraphBit` response
    fn parse_response(&self, response: ByteDanceResponse) -> GraphBitResult<LlmResponse> {
        let choice =
            response.choices.into_iter().next().ok_or_else(|| {
                GraphBitError::llm_provider("bytedance", "No choices in response")
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
impl LlmProviderTrait for ByteDanceProvider {
    fn provider_name(&self) -> &str {
        "bytedance"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<ByteDanceMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<ByteDanceTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        let body = ByteDanceRequest {
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
                GraphBitError::llm_provider("bytedance", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "bytedance",
                format!("API error: {error_text}"),
            ));
        }

        let bytedance_response: ByteDanceResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("bytedance", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(bytedance_response)
    }

    fn supports_function_calling(&self) -> bool {
        // Most ByteDance ModelArk models support function calling
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // Common context lengths for ByteDance ModelArk models
        match self.model.as_str() {
            // ByteDance Seed models
            "seed-1-6-flash-250715" => Some(256_000),
            "seed-1-6-250915" => Some(256_000),
            // Default for other models
            _ if self.model.starts_with("skylark") => Some(128_000),
            _ if self.model.starts_with("seed") => Some(256_000),
            _ => Some(256_000), // Default context length
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Cost per token in USD (input, output) for ByteDance ModelArk models
        // These are estimated costs - actual pricing may vary
        match self.model.as_str() {
            "skylark-lite" => Some((0.000_001, 0.000_002)), // $1/$2 per 1M tokens
            "skylark-plus" => Some((0.000_002, 0.000_004)), // $2/$4 per 1M tokens
            "skylark-pro" => Some((0.000_003, 0.000_006)),  // $3/$6 per 1M tokens
            "skylark-chat" => Some((0.000_002, 0.000_004)), // $2/$4 per 1M tokens
            "seed-1-6-flash-250715" => Some((0.000_000_1, 0.000_000_8)), // $0.1/$0.8 per 1K tokens
            "seed-1-6-250915" => Some((0.000_000_5, 0.000_004)), // $0.5/$4 per 1M tokens
            _ if self.model.starts_with("skylark") => Some((0.000_002, 0.000_004)),
            _ if self.model.starts_with("seed") => Some((0.000_001, 0.000_002)),
            _ => Some((0.000_002, 0.000_004)), // Default pricing
        }
    }
}

// `ByteDance ModelArk` API types (OpenAI-compatible format)
#[derive(Debug, Serialize)]
struct ByteDanceRequest {
    model: String,
    messages: Vec<ByteDanceMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ByteDanceTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ByteDanceMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ByteDanceToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ByteDanceToolCall {
    id: String,
    r#type: String,
    function: ByteDanceFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct ByteDanceFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct ByteDanceTool {
    r#type: String,
    function: ByteDanceFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct ByteDanceFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ByteDanceResponse {
    id: String,
    choices: Vec<ByteDanceChoice>,
    usage: ByteDanceUsage,
}

#[derive(Debug, Deserialize)]
struct ByteDanceChoice {
    message: ByteDanceMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ByteDanceUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::{LlmMessage, LlmRole, LlmTool};
    use serde_json::json;

    #[test]
    fn test_bytedance_provider_creation() {
        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "skylark-lite".to_string());

        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "bytedance");
        assert_eq!(provider.model_name(), "skylark-lite");
    }

    #[test]
    fn test_bytedance_provider_with_base_url() {
        let provider = ByteDanceProvider::with_base_url(
            "test-api-key".to_string(),
            "skylark-pro".to_string(),
            "https://custom.bytedance.com/api/v3".to_string(),
        );

        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "bytedance");
        assert_eq!(provider.model_name(), "skylark-pro");
        assert_eq!(provider.base_url, "https://custom.bytedance.com/api/v3");
    }

    #[test]
    fn test_bytedance_supports_function_calling() {
        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "skylark-lite".to_string()).unwrap();

        assert!(provider.supports_function_calling());
    }

    #[test]
    fn test_convert_message_user() {
        let message = LlmMessage {
            role: LlmRole::User,
            content: "Hello, world!".to_string(),
            tool_calls: Vec::new(),
        };

        let bytedance_message = ByteDanceProvider::convert_message(&message);
        assert_eq!(bytedance_message.role, "user");
        assert_eq!(bytedance_message.content, "Hello, world!");
        assert!(bytedance_message.tool_calls.is_none());
    }

    #[test]
    fn test_convert_message_assistant() {
        let message = LlmMessage {
            role: LlmRole::Assistant,
            content: "Hello! How can I help you?".to_string(),
            tool_calls: Vec::new(),
        };

        let bytedance_message = ByteDanceProvider::convert_message(&message);
        assert_eq!(bytedance_message.role, "assistant");
        assert_eq!(bytedance_message.content, "Hello! How can I help you?");
        assert!(bytedance_message.tool_calls.is_none());
    }

    #[test]
    fn test_convert_message_system() {
        let message = LlmMessage {
            role: LlmRole::System,
            content: "You are a helpful assistant.".to_string(),
            tool_calls: Vec::new(),
        };

        let bytedance_message = ByteDanceProvider::convert_message(&message);
        assert_eq!(bytedance_message.role, "system");
        assert_eq!(bytedance_message.content, "You are a helpful assistant.");
        assert!(bytedance_message.tool_calls.is_none());
    }

    #[test]
    fn test_convert_tool() {
        let tool = LlmTool {
            name: "get_weather".to_string(),
            description: "Get the current weather".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city name"
                    }
                },
                "required": ["location"]
            }),
        };

        let bytedance_tool = ByteDanceProvider::convert_tool(&tool);
        assert_eq!(bytedance_tool.r#type, "function");
        assert_eq!(bytedance_tool.function.name, "get_weather");
        assert_eq!(
            bytedance_tool.function.description,
            "Get the current weather"
        );
        assert_eq!(bytedance_tool.function.parameters["type"], "object");
    }

    #[test]
    fn test_max_context_length_skylark_models() {
        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "skylark-lite".to_string()).unwrap();
        assert_eq!(provider.max_context_length(), Some(32_768));

        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "skylark-pro".to_string()).unwrap();
        assert_eq!(provider.max_context_length(), Some(32_768));
    }

    #[test]
    fn test_max_context_length_seedance_models() {
        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "seedance-1.0-lite".to_string())
                .unwrap();
        assert_eq!(provider.max_context_length(), Some(32_768));

        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "seedance-1.0-pro".to_string())
                .unwrap();
        assert_eq!(provider.max_context_length(), Some(32_768));
    }

    #[test]
    fn test_cost_per_token_skylark_models() {
        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "skylark-lite".to_string()).unwrap();
        assert_eq!(provider.cost_per_token(), Some((0.000_001, 0.000_002)));

        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "skylark-pro".to_string()).unwrap();
        assert_eq!(provider.cost_per_token(), Some((0.000_003, 0.000_006)));
    }

    #[test]
    fn test_cost_per_token_seedance_models() {
        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "seedance-1.0-lite".to_string())
                .unwrap();
        assert_eq!(provider.cost_per_token(), Some((0.000_001_5, 0.000_003)));

        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "seedance-1.0-pro".to_string())
                .unwrap();
        assert_eq!(provider.cost_per_token(), Some((0.000_003, 0.000_006)));
    }

    #[test]
    fn test_cost_per_token_unknown_model() {
        let provider =
            ByteDanceProvider::new("test-api-key".to_string(), "unknown-model".to_string())
                .unwrap();
        assert_eq!(provider.cost_per_token(), Some((0.000_002, 0.000_004)));
    }
}
