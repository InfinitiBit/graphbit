//! `Anthropic` `Claude` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// `Anthropic` `Claude` API provider
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl AnthropicProvider {
    /// Create a new `Anthropic` provider
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

    /// Convert `GraphBit` tool to Anthropic tool format
    fn convert_tool(&self, tool: &LlmTool) -> AnthropicTool {
        AnthropicTool {
            name: tool.name.clone(),
            description: tool.description.clone(),
            input_schema: tool.parameters.clone(),
        }
    }

    /// Convert `GraphBit` messages to Anthropic format
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

    /// Parse Anthropic response to `GraphBit` response
    fn parse_response(&self, response: AnthropicResponse) -> GraphBitResult<LlmResponse> {
        let mut content_text = String::new();
        let mut tool_calls = Vec::new();

        // Process content blocks
        for block in &response.content {
            match block.r#type.as_str() {
                "text" => {
                    if let Some(text) = &block.text {
                        if !content_text.is_empty() {
                            content_text.push('\n');
                        }
                        content_text.push_str(text);
                    }
                }
                "tool_use" => {
                    if let (Some(id), Some(name), Some(input)) =
                        (&block.id, &block.name, &block.input)
                    {
                        tool_calls.push(LlmToolCall {
                            id: id.clone(),
                            name: name.clone(),
                            parameters: input.clone(),
                        });
                    }
                }
                _ => {
                    // Handle other content types if needed
                }
            }
        }

        let finish_reason = match response.stop_reason.as_deref() {
            Some("end_turn") | Some("stop_sequence") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("tool_use") => FinishReason::Other("tool_use".into()),
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

        let usage = LlmUsage::new(response.usage.input_tokens, response.usage.output_tokens);

        let mut llm_response = LlmResponse::new(content_text, &self.model)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response.id);

        // Add tool calls if any
        if !tool_calls.is_empty() {
            llm_response = llm_response.with_tool_calls(tool_calls);
        }

        Ok(llm_response)
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

        // Convert tools to Anthropic format
        let tools: Option<Vec<AnthropicTool>> = if request.tools.is_empty() {
            tracing::info!("No tools provided in request");
            None
        } else {
            tracing::info!("Converting {} tools for Anthropic", request.tools.len());
            Some(request.tools.iter().map(|t| self.convert_tool(t)).collect())
        };

        let body = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: request.max_tokens.unwrap_or(4096),
            messages,
            system: system_prompt,
            temperature: request.temperature,
            top_p: request.top_p,
            tools,
        };

        tracing::info!(
            "Sending request to Anthropic with {} tools",
            body.tools.as_ref().map(|t| t.len()).unwrap_or(0)
        );

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
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    content: Vec<ContentBlock>,
    stop_reason: Option<String>,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    r#type: String, // "text", "tool_use", "tool_result", "thinking", etc.
    text: Option<String>,             // present when type == "text"
    id: Option<String>,               // present when type == "tool_use"
    name: Option<String>,             // present when type == "tool_use"
    input: Option<serde_json::Value>, // present when type == "tool_use"
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}
