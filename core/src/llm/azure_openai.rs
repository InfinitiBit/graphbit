//! `Azure OpenAI` LLM provider implementation
//!
//! `Azure OpenAI` provides `OpenAI` models through Microsoft `Azure` infrastructure.
//! It uses a different endpoint structure and authentication method compared to `OpenAI`.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};

/// `Azure OpenAI` API provider
pub struct AzureOpenAiProvider {
    client: Client,
    api_key: String,
    deployment_name: String,
    endpoint: String,
    api_version: String,
}

impl AzureOpenAiProvider {
    /// Create a new `Azure OpenAI` provider
    pub fn new(
        api_key: String,
        deployment_name: String,
        endpoint: String,
        api_version: String,
    ) -> GraphBitResult<Self> {
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10) // Increased connection pool size
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "azure_openai",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;

        Ok(Self {
            client,
            api_key,
            deployment_name,
            endpoint,
            api_version,
        })
    }

    /// Create a new `Azure OpenAI` provider with default API version
    pub fn with_defaults(
        api_key: String,
        deployment_name: String,
        endpoint: String,
    ) -> GraphBitResult<Self> {
        Self::new(api_key, deployment_name, endpoint, "2024-10-21".to_string())
    }

    /// Convert `GraphBit` message to `Azure OpenAI` message format
    fn convert_message(message: &LlmMessage) -> AzureOpenAiMessage {
        AzureOpenAiMessage {
            role: match message.role {
                LlmRole::System => "system".to_string(),
                LlmRole::User => "user".to_string(),
                LlmRole::Assistant => "assistant".to_string(),
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
                        .map(|tc| AzureOpenAiToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: AzureOpenAiFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `Azure OpenAI` tool format
    fn convert_tool(tool: &LlmTool) -> AzureOpenAiTool {
        AzureOpenAiTool {
            r#type: "function".to_string(),
            function: AzureOpenAiFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `Azure OpenAI` response to `GraphBit` response
    fn parse_response(&self, response: AzureOpenAiResponse) -> GraphBitResult<LlmResponse> {
        let choice =
            response.choices.into_iter().next().ok_or_else(|| {
                GraphBitError::llm_provider("azure_openai", "No choices in response")
            })?;

        let finish_reason = match choice.finish_reason.as_str() {
            "stop" => FinishReason::Stop,
            "length" => FinishReason::Length,
            "tool_calls" => FinishReason::ToolCalls,
            "content_filter" => FinishReason::ContentFilter,
            _ => FinishReason::Other(choice.finish_reason),
        };

        let tool_calls = if let Some(tool_calls) = choice.message.tool_calls {
            tool_calls
                .into_iter()
                .map(|tc| LlmToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    parameters: serde_json::from_str(&tc.function.arguments).unwrap_or_default(),
                })
                .collect()
        } else {
            Vec::new()
        };

        Ok(LlmResponse::new(
            choice.message.content.unwrap_or_default(),
            &self.deployment_name,
        )
        .with_tool_calls(tool_calls)
        .with_finish_reason(finish_reason)
        .with_usage(LlmUsage {
            prompt_tokens: response.usage.prompt_tokens,
            completion_tokens: response.usage.completion_tokens,
            total_tokens: response.usage.total_tokens,
        })
        .with_id(response.id))
    }
}

#[async_trait]
impl LlmProviderTrait for AzureOpenAiProvider {
    fn provider_name(&self) -> &str {
        "azure_openai"
    }

    fn model_name(&self) -> &str {
        &self.deployment_name
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint, self.deployment_name, self.api_version
        );

        let messages: Vec<AzureOpenAiMessage> = request
            .messages
            .iter()
            .map(|m| Self::convert_message(m))
            .collect();

        let tools: Option<Vec<AzureOpenAiTool>> = if request.tools.is_empty() {
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

        let body = AzureOpenAiRequest {
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
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_json)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("azure_openai", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "azure_openai",
                format!("API error: {error_text}"),
            ));
        }

        let azure_response: AzureOpenAiResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("azure_openai", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(azure_response)
    }

    fn supports_function_calling(&self) -> bool {
        // Most Azure OpenAI deployments support function calling
        // This could be made more specific based on the deployment model
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // Context length depends on the underlying model deployed
        // Common Azure OpenAI models and their context lengths
        // This is a simplified mapping - in practice, you'd want to query the deployment info
        Some(128_000) // Default to a common large context size
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Azure OpenAI pricing varies by region and model
        // This would need to be configured based on the specific deployment
        None
    }
}

// Request/Response structures for Azure OpenAI API
#[derive(Debug, Serialize)]
struct AzureOpenAiRequest {
    messages: Vec<AzureOpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AzureOpenAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AzureOpenAiMessage {
    role: String,
    #[serde(deserialize_with = "deserialize_nullable_content")]
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<AzureOpenAiToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AzureOpenAiToolCall {
    id: String,
    r#type: String,
    function: AzureOpenAiFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct AzureOpenAiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct AzureOpenAiTool {
    r#type: String,
    function: AzureOpenAiFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct AzureOpenAiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AzureOpenAiResponse {
    id: String,
    choices: Vec<AzureOpenAiChoice>,
    usage: AzureOpenAiUsage,
}

#[derive(Debug, Deserialize)]
struct AzureOpenAiChoice {
    message: AzureOpenAiResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct AzureOpenAiResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<AzureOpenAiToolCall>>,
}

#[derive(Debug, Deserialize)]
struct AzureOpenAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

// Helper function to handle nullable content in responses
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
    use crate::llm::{LlmMessage, LlmRole, LlmTool};
    use serde_json::json;

    #[test]
    fn test_azure_openai_provider_creation() {
        let provider = AzureOpenAiProvider::new(
            "test-api-key".to_string(),
            "test-deployment".to_string(),
            "https://test.openai.azure.com".to_string(),
            "2024-10-21".to_string(),
        );

        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "azure_openai");
        assert_eq!(provider.model_name(), "test-deployment");
    }

    #[test]
    fn test_azure_openai_provider_with_defaults() {
        let provider = AzureOpenAiProvider::with_defaults(
            "test-api-key".to_string(),
            "test-deployment".to_string(),
            "https://test.openai.azure.com".to_string(),
        );

        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "azure_openai");
        assert_eq!(provider.model_name(), "test-deployment");
    }

    #[test]
    fn test_azure_openai_supports_function_calling() {
        let provider = AzureOpenAiProvider::new(
            "test-api-key".to_string(),
            "test-deployment".to_string(),
            "https://test.openai.azure.com".to_string(),
            "2024-10-21".to_string(),
        )
        .unwrap();

        assert!(provider.supports_function_calling());
    }

    #[test]
    fn test_convert_message_user() {
        let message = LlmMessage {
            role: LlmRole::User,
            content: "Hello, world!".to_string(),
            tool_calls: Vec::new(),
        };

        let azure_message = AzureOpenAiProvider::convert_message(&message);
        assert_eq!(azure_message.role, "user");
        assert_eq!(azure_message.content, "Hello, world!");
        assert!(azure_message.tool_calls.is_none());
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

        let azure_tool = AzureOpenAiProvider::convert_tool(&tool);
        assert_eq!(azure_tool.r#type, "function");
        assert_eq!(azure_tool.function.name, "get_weather");
        assert_eq!(azure_tool.function.description, "Get the current weather");
        assert_eq!(azure_tool.function.parameters["type"], "object");
    }
}
