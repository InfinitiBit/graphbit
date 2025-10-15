//! `Cloudflare Worker AI` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cloudflare Worker AI provider implementation
pub struct CloudflareProvider {
    client: Client,
    api_key: String,
    model: String,
    account_id: String,
}

#[derive(Debug, Serialize)]
struct CloudflareRequest {
    messages: Vec<CloudflareMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<CloudflareTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<CloudflareToolCall>>,
}

#[derive(Debug, Deserialize)]
struct CloudflareResponse {
    result: CloudflareResult,
    success: bool,
    errors: Vec<CloudflareError>,
    messages: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CloudflareResult {
    response: String,
}

#[derive(Debug, Deserialize)]
struct CloudflareError {
    code: u32,
    message: String,
}

#[derive(Debug, Deserialize)]
struct CloudflareChoice {
    message: CloudflareMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CloudflareUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl CloudflareProvider {
    /// Get the base URL for Cloudflare Worker AI API
    fn get_base_url(&self) -> String {
        format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            self.account_id, self.model
        )
    }

    /// Create a new Cloudflare Worker AI provider
    pub fn new(api_key: String, model: String, account_id: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("cloudflare", format!("Failed to create HTTP client: {e}"))
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            account_id,
        })
    }

    /// Convert GraphBit message to Cloudflare message format
    fn convert_message(message: &LlmMessage) -> CloudflareMessage {
        CloudflareMessage {
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
                        .map(|tc| CloudflareToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: CloudflareFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
        }
    }
}

#[async_trait]
impl LlmProviderTrait for CloudflareProvider {
    fn provider_name(&self) -> &str {
        "cloudflare"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = self.get_base_url();

        let cloudflare_request = CloudflareRequest {
            messages: request
                .messages
                .iter()
                .map(Self::convert_message)
                .collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: request.tools.iter().map(|t| t.into()).collect(),
            stream: None,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&cloudflare_request)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "cloudflare",
                    format!("Failed to send request: {}", e),
                )
            })?;

        let response = response.error_for_status().map_err(|e| {
            GraphBitError::llm_provider(
                "cloudflare",
                format!("Request failed: {}", e),
            )
        })?;

        let cloudflare_response: CloudflareResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider(
                "cloudflare",
                format!("Failed to parse response: {}", e),
            )
        })?;

        if !cloudflare_response.success {
            let error_msg = cloudflare_response
                .errors
                .into_iter()
                .map(|e| format!("Error {}: {}", e.code, e.message))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(GraphBitError::llm_provider("cloudflare", error_msg));
        }

        let response_content = cloudflare_response.result.response.clone();
        Ok(LlmResponse {
            id: None,  // Cloudflare doesn't provide an ID
            content: response_content,
            tool_calls: vec![], // Cloudflare doesn't support tool calls in this format
            finish_reason: FinishReason::Stop, // Default to Stop since Cloudflare doesn't provide this
            usage: LlmUsage {
                prompt_tokens: 0,  // Cloudflare doesn't provide usage stats
                completion_tokens: 0,
                total_tokens: 0,
            },
            metadata: {
                let mut metadata = HashMap::new();
                // Add messages from the response for debugging
                if !cloudflare_response.messages.is_empty() {
                    metadata.insert("cloudflare_messages".to_string(), 
                        serde_json::to_value(&cloudflare_response.messages).unwrap_or_default());
                }
                // Add raw response for debugging
                metadata.insert("cloudflare_raw_response".to_string(), 
                    serde_json::to_value(&cloudflare_response.result.response).unwrap_or_default());
                metadata
            },
            model: self.model.clone(),
        })
    }
}

#[derive(Debug, Serialize)]
struct CloudflareTool {
    r#type: String,
    function: CloudflareFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareToolCall {
    id: String,
    r#type: String,
    function: CloudflareFunction,
}

impl From<&LlmTool> for CloudflareTool {
    fn from(tool: &LlmTool) -> Self {
        CloudflareTool {
            r#type: "function".to_string(),
            function: CloudflareFunction {
                name: tool.name.clone(),
                arguments: tool.parameters.to_string(),
            },
        }
    }
}