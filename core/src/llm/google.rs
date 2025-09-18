//! `Google Gemini` LLM provider implementation
//!
//! `Google Gemini` provides access to Google's Gemini models through the official Gemini API.
//! It supports various Gemini models including Gemini 2.5 Pro, Gemini 2.5 Flash, and Gemini 1.5 models.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmRequest, LlmResponse, LlmRole, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// `Google Gemini` API provider
pub struct GoogleProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl GoogleProvider {
    /// Create a new `Google Gemini` provider
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
                    "google",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://generativelanguage.googleapis.com/v1beta/models".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Create a new `Google Gemini` provider with custom base URL
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
                    "google",
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

    /// Get the context length for a given model
    pub fn get_context_length(&self) -> u32 {
        match self.model.as_str() {
            // Gemini 2.5 models
            "gemini-2.5-pro" | "gemini-2.5-flash" | "gemini-2.5-flash-lite" => 1_048_576, // 1M tokens
            // Gemini 2.0 models  
            "gemini-2.0-flash" | "gemini-2.0-flash-lite" => 1_048_576, // 1M tokens
            // Gemini 1.5 models
            "gemini-1.5-pro" => 2_097_152, // 2M tokens
            "gemini-1.5-flash" | "gemini-1.5-flash-8b" => 1_048_576, // 1M tokens
            // Gemini 1.0 models
            "gemini-1.0-pro" => 32_768, // 32K tokens
            // Default for unknown models
            _ => 1_048_576, // 1M tokens as default
        }
    }

    /// Parse `Google Gemini` response to `GraphBit` response
    fn parse_response(&self, response: GoogleResponse) -> GraphBitResult<LlmResponse> {
        let candidate = response.candidates.into_iter().next().ok_or_else(|| {
            GraphBitError::llm_provider("google", "No candidates in response")
        })?;

        let parts = candidate.content.parts;

        let content = parts
            .iter()
            .map(|part| part.text.clone())
            .collect::<Vec<_>>()
            .join("");

        let finish_reason = match candidate.finish_reason.as_deref() {
            Some("STOP") => FinishReason::Stop,
            Some("MAX_TOKENS") => FinishReason::Length,
            Some("SAFETY") => FinishReason::ContentFilter,
            Some("RECITATION") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        };

        let usage = response.usage_metadata.map(|usage| LlmUsage {
            prompt_tokens: usage.prompt_token_count,
            completion_tokens: usage.candidates_token_count.unwrap_or(0),
            total_tokens: usage.total_token_count,
        }).unwrap_or_else(|| LlmUsage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        });

        Ok(LlmResponse {
            content,
            tool_calls: vec![], // Simplified for now, no tool calls
            usage,
            metadata: std::collections::HashMap::new(),
            finish_reason,
            model: self.model.clone(),
            id: None,
        })
    }
}

#[async_trait]
impl LlmProviderTrait for GoogleProvider {
    fn provider_name(&self) -> &str {
        "google"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        // Correct Google Gemini API URL format
        let url = format!("{}/{}:generateContent", self.base_url, self.model);

        // Convert messages to Google Gemini format - combine all messages into a single content
        let combined_content = request
            .messages
            .iter()
            .map(|m| match m.role {
                LlmRole::System => format!("System: {}", m.content),
                LlmRole::User => format!("User: {}", m.content),
                LlmRole::Assistant => format!("Assistant: {}", m.content),
                LlmRole::Tool => format!("Tool: {}", m.content),
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let contents = vec![GoogleContent {
            parts: vec![GooglePart {
                text: combined_content,
            }],
        }];

        let generation_config = GoogleGenerationConfig {
            max_output_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
        };

        let body = GoogleRequest {
            contents,
            generation_config: Some(generation_config),
        };

        let request_builder = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json");

        let response = request_builder
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("google", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Provide more specific error messages based on status code
            let error_message = match status_code {
                400 => format!("Bad request - check your API key format and request structure: {error_text}"),
                401 => format!("Unauthorized - invalid API key: {error_text}"),
                403 => format!("Forbidden - API key doesn't have permission or quota exceeded: {error_text}"),
                404 => format!("Model not found - check if the model name is correct: {error_text}"),
                429 => format!("Rate limit exceeded - please wait and try again: {error_text}"),
                500 => format!("Google API internal error - this is temporary, please retry: {error_text}"),
                503 => format!("Service unavailable - Google API is temporarily overloaded: {error_text}"),
                _ => format!("API error ({}): {error_text}", status_code),
            };

            return Err(GraphBitError::llm_provider("google", error_message));
        }

        let google_response: GoogleResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("google", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(google_response)
    }

    fn supports_streaming(&self) -> bool {
        false // Google Gemini streaming can be added later
    }
}

// Google Gemini API types
#[derive(Debug, Serialize)]
struct GoogleRequest {
    contents: Vec<GoogleContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "generationConfig")]
    generation_config: Option<GoogleGenerationConfig>,
}

#[derive(Debug, Serialize)]
struct GoogleGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "topP")]
    top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleContent {
    parts: Vec<GooglePart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GooglePart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GoogleResponse {
    candidates: Vec<GoogleCandidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<GoogleUsage>,
}

#[derive(Debug, Deserialize)]
struct GoogleCandidate {
    content: GoogleContent,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleUsage {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: Option<u32>,
    #[serde(rename = "totalTokenCount")]
    total_token_count: u32,
}
