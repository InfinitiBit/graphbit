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
    gateway_id: String,
}

#[derive(Debug, Serialize)]
struct CloudflareRequest {
    model: String,
    messages: Vec<CloudflareMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<CloudflareTool>,
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
    id: String,
    choices: Vec<CloudflareChoice>,
    usage: CloudflareUsage,
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
            "https://gateway.ai.cloudflare.com/v1/{}/{}/compat",
            self.account_id, self.gateway_id
        )
    }

    /// Create a new Cloudflare Worker AI provider
    pub fn new(api_key: String, model: String, account_id: String, gateway_id: String) -> GraphBitResult<Self> {
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
            gateway_id,
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
        let base_url = self.get_base_url();
        let url = format!("{}/chat/completions", base_url);

        let cloudflare_request = CloudflareRequest {
            model: self.model.clone(),
            messages: request
                .messages
                .iter()
                .map(Self::convert_message)
                .collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: request.tools.iter().map(|t| t.into()).collect(),
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

        let choice = cloudflare_response.choices.into_iter().next().ok_or_else(|| {
            GraphBitError::llm_provider("cloudflare", "No completion in response")
        })?;

        Ok(LlmResponse {
            id: Some(cloudflare_response.id),
            content: choice.message.content,
            tool_calls: choice
                .message
                .tool_calls
                .unwrap_or_default()
                .into_iter()
                .map(|tc| LlmToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    parameters: serde_json::from_str(&tc.function.arguments).unwrap_or_default(),
                })
                .collect(),
            finish_reason: match choice.finish_reason.as_deref() {
                Some("stop") => FinishReason::Stop,
                Some("length") => FinishReason::Length,
                Some("tool_calls") => FinishReason::ToolCalls,
                Some("content_filter") => FinishReason::ContentFilter,
                _ => FinishReason::Other(choice.finish_reason.unwrap_or_else(|| "unknown".to_string())),
            },
            usage: LlmUsage {
                prompt_tokens: cloudflare_response.usage.prompt_tokens,
                completion_tokens: cloudflare_response.usage.completion_tokens,
                total_tokens: cloudflare_response.usage.total_tokens,
            },
            metadata: HashMap::new(),
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