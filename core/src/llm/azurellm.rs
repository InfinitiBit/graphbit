//! `Azure LLM` provider implementation
//!
//! `Azure LLM` provides various models through Microsoft `Azure` infrastructure.
//! It uses a different endpoint structure and authentication method compared to `OpenAI`.
//! This provider supports all Azure-deployed models,not just OpenAI models.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};

/// `Azure LLM` API provider
pub struct AzureLlmProvider {
    client: Client,
    api_key: String,
    deployment_name: String,
    endpoint: String,
    api_version: String,
}

impl AzureLlmProvider {
    /// Create a new `Azure LLM` provider
    pub fn new(
        api_key: String,
        deployment_name: String,
        endpoint: String,
        api_version: String,
    ) -> GraphBitResult<Self> {
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120)) // Increased timeout for Azure LLM
            .pool_max_idle_per_host(10) // Increased connection pool size
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "azurellm",
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

    /// Create a new `Azure LLM` provider with default API version
    pub fn with_defaults(
        api_key: String,
        deployment_name: String,
        endpoint: String,
    ) -> GraphBitResult<Self> {
        Self::new(api_key, deployment_name, endpoint, "2024-10-21".to_string())
    }

    /// Convert `GraphBit` message to `Azure LLM` message format
    fn convert_message(message: &LlmMessage) -> AzureLlmMessage {
        AzureLlmMessage {
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
                        .map(|tc| AzureLlmToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: AzureLlmFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }

    /// Convert `GraphBit` tool to `Azure LLM` tool format
    fn convert_tool(tool: &LlmTool) -> AzureLlmTool {
        AzureLlmTool {
            r#type: "function".to_string(),
            function: AzureLlmFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Check if the deployment is an OpenAI model
    /// OpenAI models require `max_completion_tokens` instead of `max_tokens`
    /// Other models (Claude, Llama, Mistral, etc.) use `max_tokens`
    fn is_openai_model(&self) -> bool {
        let name = self.deployment_name.to_lowercase();
        // OpenAI model patterns: gpt-*, o1*, o3*, o4*, gpt4*, gpt5*, etc.
        name.contains("gpt")
            || name.starts_with("o1")
            || name.starts_with("o3")
            || name.starts_with("o4")
            || name.contains("-o1")
            || name.contains("-o3")
            || name.contains("-o4")
            || name.starts_with("text-davinci")
            || name.starts_with("davinci")
            || name.starts_with("curie")
            || name.starts_with("babbage")
            || name.starts_with("ada")
    }

    /// Parse `Azure LLM` response to `GraphBit` response
    fn parse_response(&self, response: AzureLlmResponse) -> GraphBitResult<LlmResponse> {
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| GraphBitError::llm_provider("azurellm", "No choices in response"))?;

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

        // Handle Azure's null/empty content bug
        // When finish_reason is "length" or "content_filter", Azure returns EMPTY STRING (not null!)
        // despite consuming completion tokens. This is a known Azure API quirk.
        let has_content = choice
            .message
            .content
            .as_ref()
            .map(|s| !s.is_empty())
            .unwrap_or(false);
        let content_value = choice.message.content.unwrap_or_default();

        let content = if has_content {
            // Normal case: content exists and is not empty
            tracing::debug!("Azure response has content: {} chars", content_value.len());
            content_value
        } else if response.usage.completion_tokens > 0 {
            // Bug case: empty/null content but tokens were used
            match &finish_reason {
                FinishReason::Length => {
                    tracing::warn!(
                        "Azure API returned empty content despite using {} completion tokens (finish_reason: Length). \
                        This typically occurs with very low max_tokens limits. Consider increasing max_tokens for better results.",
                        response.usage.completion_tokens
                    );
                    let msg = format!(
                        "[Azure API used {} tokens but returned no content. This occurs when max_tokens is set too low. \
                        The model may have started generating a response but was cut off before producing visible text. \
                        Increase max_tokens for meaningful output.]",
                        response.usage.completion_tokens
                    );
                    tracing::debug!("Returning Azure empty content message");
                    msg
                }
                FinishReason::ContentFilter => {
                    tracing::warn!(
                        "Azure API filtered content after using {} tokens",
                        response.usage.completion_tokens
                    );
                    "[Content was filtered by Azure's content policy]".to_string()
                }
                FinishReason::ToolCalls => {
                    // Tool calls typically have no content - this is expected
                    tracing::debug!("Tool calls response - no content expected");
                    String::new()
                }
                _ => {
                    tracing::debug!(
                        "Azure empty content - other case: finish_reason={:?}, tokens={}",
                        finish_reason,
                        response.usage.completion_tokens
                    );
                    String::new()
                }
            }
        } else {
            // No content and no tokens - truly empty response
            String::new()
        };

        Ok(LlmResponse::new(content, &self.deployment_name)
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
impl LlmProviderTrait for AzureLlmProvider {
    fn provider_name(&self) -> &str {
        "azurellm"
    }

    fn model_name(&self) -> &str {
        &self.deployment_name
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        // Normalize endpoint URL to avoid double slashes
        let endpoint = self.endpoint.trim_end_matches('/');
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            endpoint, self.deployment_name, self.api_version
        );

        let messages: Vec<AzureLlmMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<AzureLlmTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        // Use max_completion_tokens for OpenAI models, max_tokens for others (Claude, Llama, etc.)
        let (max_tokens, max_completion_tokens) = if self.is_openai_model() {
            (None, request.max_tokens)
        } else {
            (request.max_tokens, None)
        };

        let body = AzureLlmRequest {
            messages,
            max_tokens,
            max_completion_tokens,
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
            .map_err(|e| GraphBitError::llm_provider("azurellm", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "azurellm",
                format!("API error: {error_text}"),
            ));
        }

        let azure_response: AzureLlmResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("azurellm", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(azure_response)
    }

    fn supports_function_calling(&self) -> bool {
        // Most Azure LLM deployments support function calling
        // This could be made more specific based on the deployment model
        true
    }

    fn max_context_length(&self) -> Option<u32> {
        // Context length depends on the underlying model deployed
        // Common Azure LLM models and their context lengths
        // This is a simplified mapping - in practice, you'd want to query the deployment info
        Some(128_000) // Default to a common large context size
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Azure LLM pricing varies by region and model
        // This would need to be configured based on the specific deployment
        None
    }
}

// Request/Response structures for Azure LLM API
#[derive(Debug, Serialize)]
struct AzureLlmRequest {
    messages: Vec<AzureLlmMessage>,
    /// Used for non-OpenAI models (Claude, Llama, Mistral, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    /// Used for OpenAI models (gpt-*, o1*, o3*, etc.) - deprecated max_tokens replacement
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AzureLlmTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AzureLlmMessage {
    role: String,
    #[serde(deserialize_with = "deserialize_nullable_content")]
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<AzureLlmToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AzureLlmToolCall {
    id: String,
    r#type: String,
    function: AzureLlmFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct AzureLlmFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct AzureLlmTool {
    r#type: String,
    function: AzureLlmFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct AzureLlmFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AzureLlmResponse {
    id: String,
    choices: Vec<AzureLlmChoice>,
    usage: AzureLlmUsage,
}

#[derive(Debug, Deserialize)]
struct AzureLlmChoice {
    message: AzureLlmResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct AzureLlmResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<AzureLlmToolCall>>,
}

#[derive(Debug, Deserialize)]
struct AzureLlmUsage {
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
    fn test_azurellm_provider_creation() {
        let provider = AzureLlmProvider::new(
            "test-api-key".to_string(),
            "test-deployment".to_string(),
            "https://test.openai.azure.com".to_string(),
            "2024-10-21".to_string(),
        );

        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "azurellm");
        assert_eq!(provider.model_name(), "test-deployment");
    }

    #[test]
    fn test_azurellm_provider_with_defaults() {
        let provider = AzureLlmProvider::with_defaults(
            "test-api-key".to_string(),
            "test-deployment".to_string(),
            "https://test.openai.azure.com".to_string(),
        );

        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "azurellm");
        assert_eq!(provider.model_name(), "test-deployment");
    }

    #[test]
    fn test_azurellm_supports_function_calling() {
        let provider = AzureLlmProvider::new(
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

        let azure_message = AzureLlmProvider::convert_message(&message);
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

        let azure_tool = AzureLlmProvider::convert_tool(&tool);
        assert_eq!(azure_tool.r#type, "function");
        assert_eq!(azure_tool.function.name, "get_weather");
        assert_eq!(azure_tool.function.description, "Get the current weather");
        assert_eq!(azure_tool.function.parameters["type"], "object");
    }
}
