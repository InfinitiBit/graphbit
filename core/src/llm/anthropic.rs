//! Anthropic Claude LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmUsage};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Anthropic Claude API provider
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = Client::new();
        let base_url = "https://api.anthropic.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert GraphBit messages to Anthropic format
    fn convert_messages(&self, messages: &[LlmMessage]) -> (Option<String>, Vec<AnthropicMessage>) {
        let mut system_prompt = None;
        let mut anthropic_messages = Vec::new();

        for message in messages {
            match message.role {
                LlmRole::System => {
                    system_prompt = Some(message.content.clone());
                }
                LlmRole::User => {
                    anthropic_messages.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: message.content.clone(),
                    });
                }
                LlmRole::Assistant => {
                    anthropic_messages.push(AnthropicMessage {
                        role: "assistant".to_string(),
                        content: message.content.clone(),
                    });
                }
                LlmRole::Tool => {
                    anthropic_messages.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: format!("Tool result: {}", message.content),
                    });
                }
            }
        }

        (system_prompt, anthropic_messages)
    }

    /// Parse Anthropic response to GraphBit response
    fn parse_response(&self, response: AnthropicResponse) -> GraphBitResult<LlmResponse> {
        let content_text = response
            .content
            .iter()
            .filter_map(|b| (b.r#type == "text").then(|| b.text.as_deref().unwrap_or("")))
            .collect::<Vec<_>>()
            .join("\n");

        let finish_reason = match response.stop_reason.as_deref() {
            Some("end_turn") | Some("stop_sequence") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("tool_use") => FinishReason::Other("tool_use".into()),
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

        let usage = LlmUsage::new(response.usage.input_tokens, response.usage.output_tokens);

        Ok(LlmResponse::new(content_text, &self.model)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response.id))
    }
}

#[async_trait]
impl LlmProviderTrait for AnthropicProvider {
    fn provider_name(&self) -> &str {
        "anthropic"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/messages", self.base_url);

        let (system_prompt, messages) = self.convert_messages(&request.messages);

        let body = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages,
            system: system_prompt,
            temperature: request.temperature,
            top_p: request.top_p,
        };

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("anthropic", format!("Request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "anthropic",
                format!("API error: {}", error_text),
            ));
        }

        let anthropic_response: AnthropicResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("anthropic", format!("Failed to parse response: {}", e))
        })?;

        self.parse_response(anthropic_response)
    }

    fn max_context_length(&self) -> Option<u32> {
        match self.model.as_str() {
            "claude-instant-1.2" => Some(100000),
            "claude-2.0" => Some(100000),
            "claude-2.1" => Some(200000),
            "claude-3-sonnet-20240229" => Some(200000),
            "claude-3-opus-20240229" => Some(200000),
            "claude-3-haiku-20240307" => Some(200000),
            _ if self.model.starts_with("claude-3") => Some(200000),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    model: String,
    role: String,
    content: Vec<ContentBlock>,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    r#type: String, // "text", "tool_use", "tool_result", "thinking", etc.
    text: Option<String>, // present when type == "text"
                          // add optional fields for other types as needed
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}
