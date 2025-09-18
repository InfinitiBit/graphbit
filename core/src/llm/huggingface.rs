//! `HuggingFace` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmUsage};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

/// `HuggingFace` API provider
pub struct HuggingFaceProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl HuggingFaceProvider {
    /// Create a new `HuggingFace` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = Client::new();
        let base_url = "https://api-inference.huggingface.co/models".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `HuggingFace` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = Client::new();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert `GraphBit` messages to `HuggingFace` chat format
    fn format_messages_for_chat(messages: &[LlmMessage]) -> String {
        let mut formatted = String::new();

        for message in messages {
            match message.role {
                LlmRole::System => {
                    write!(formatted, "System: {}\n", message.content).unwrap();
                }
                LlmRole::User => {
                    write!(formatted, "User: {}\n", message.content).unwrap();
                }
                LlmRole::Assistant => {
                    write!(formatted, "Assistant: {}\n", message.content).unwrap();
                }
                LlmRole::Tool => {
                    write!(formatted, "Tool: {}\n", message.content).unwrap();
                }
            }
        }

        // Add assistant prefix for generation
        formatted.push_str("Assistant: ");
        formatted
    }

    /// Parse `HuggingFace` response to `GraphBit` response
    fn parse_response(&self, response: HuggingFaceResponse) -> GraphBitResult<LlmResponse> {
        let generated_text = response
            .into_iter()
            .next()
            .ok_or_else(|| {
                GraphBitError::llm_provider("huggingface", "No generated text in response")
            })?
            .generated_text;

        // Extract only the assistant's response (after the last "Assistant: ")
        let content = if let Some(last_assistant) = generated_text.rfind("Assistant: ") {
            generated_text[last_assistant + "Assistant: ".len()..]
                .trim()
                .to_string()
        } else {
            generated_text.trim().to_string()
        };

        // `HuggingFace` doesn't provide usage stats in the same way, so we estimate
        let usage = LlmUsage::new(
            (content.len() / 4) as u32, // Rough estimate: 4 chars per token
            (content.len() / 4) as u32,
        );

        Ok(LlmResponse::new(content, &self.model)
            .with_usage(usage)
            .with_finish_reason(FinishReason::Stop))
    }
}

#[async_trait]
impl LlmProviderTrait for HuggingFaceProvider {
    fn provider_name(&self) -> &str {
        "huggingface"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/{}", self.base_url, self.model);

        // Format messages for `HuggingFace`
        let inputs = Self::format_messages_for_chat(&request.messages);

        let mut parameters = HashMap::new();
        if let Some(max_tokens) = request.max_tokens {
            parameters.insert(
                "max_new_tokens".to_string(),
                serde_json::Value::Number(max_tokens.into()),
            );
        }
        if let Some(temperature) = request.temperature {
            parameters.insert(
                "temperature".to_string(),
                serde_json::Value::Number(
                    serde_json::Number::from_f64(temperature as f64).unwrap(),
                ),
            );
        }
        if let Some(top_p) = request.top_p {
            parameters.insert(
                "top_p".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(top_p as f64).unwrap()),
            );
        }

        // Add extra parameters
        for (key, value) in request.extra_params {
            parameters.insert(key, value);
        }

        let body = HuggingFaceRequest {
            inputs,
            parameters: if parameters.is_empty() {
                None
            } else {
                Some(parameters)
            },
            options: Some(HuggingFaceOptions {
                wait_for_model: true,
                use_cache: false,
            }),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("huggingface", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(GraphBitError::llm_provider(
                "huggingface",
                format!("API error: {error_text}"),
            ));
        }

        let hf_response: HuggingFaceResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("huggingface", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(hf_response)
    }

    fn supports_function_calling(&self) -> bool {
        false // Most `HuggingFace` models don't support function calling out of the box
    }

    fn supports_streaming(&self) -> bool {
        false // Streaming would require different endpoint
    }

    fn max_context_length(&self) -> Option<u32> {
        // Common context lengths for popular models, can be model-specific
        if self.model.contains("llama") {
            Some(4096)
        } else if self.model.contains("mistral") {
            Some(8192)
        } else {
            Some(2048) // Conservative default
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // `HuggingFace` inference API pricing varies by model
        // This would need to be updated based on actual pricing
        Some((0.0001, 0.0001)) // Placeholder values
    }
}

#[derive(Debug, Serialize)]
struct HuggingFaceRequest {
    inputs: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<HuggingFaceOptions>,
}

#[derive(Debug, Serialize)]
struct HuggingFaceOptions {
    wait_for_model: bool,
    use_cache: bool,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceResponseItem {
    generated_text: String,
}

type HuggingFaceResponse = Vec<HuggingFaceResponseItem>;
