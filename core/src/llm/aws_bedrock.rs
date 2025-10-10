//! `AWS Bedrock` LLM provider implementation
//!
//! This connector interacts with AWS Bedrock-hosted models (e.g. Claude, Llama, Titan).
//! It uses the official AWS SDK for Rust to send prompts and parse generated text.
//!
//! **Changes in this version**
//! * No longer uses `aws_config::defaults(...).load()`.
//! * Builds the `BedrockClient` directly via `aws_sdk_bedrockruntime::config::Builder`.
//! * Sets region, credentials, and the latest behavior version explicitly.
//! * Keeps the same public API – callers still pass `region`, `model_id`, `access_key_id`, `secret_access_key`, and optional `session_token`.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmUsage};

use async_trait::async_trait;
use aws_sdk_bedrockruntime::config::Builder as BedrockConfigBuilder;
use aws_sdk_bedrockruntime::config::Credentials;
use aws_sdk_bedrockruntime::config::Region;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use aws_sdk_bedrockruntime::config::BehaviorVersion;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// `AWS Bedrock` API provider
pub struct AwsBedrockProvider {
    client: BedrockClient,
    model_id: String,
}

impl AwsBedrockProvider {
    /// Create a new `AWS Bedrock` provider **without** using `aws_config::load`.
    ///
    /// - `region`: e.g. `"us-east-1"`
    /// - `model_id`: e.g. `"anthropic.claude-v2"`
    /// - `access_key_id`, `secret_access_key`, `session_token` (optional) are used to build static credentials.
    ///
    /// The SDK's `Config` is built manually:
    /// ```rust
    /// let conf = BedrockConfigBuilder::default()
    ///     .behavior_version(BehaviorVersion::latest())
    ///     .region(Region::new(region.clone()))
    ///     .credentials_provider(Credentials::new(...))
    ///     .build();
    /// let client = BedrockClient::from_conf(conf);
    /// ```
    pub fn new(
        region: String,
        model_id: String,
        access_key_id: String,
        secret_access_key: String,
        session_token: Option<String>,
    ) -> GraphBitResult<Self> {
        // 1. Build static AWS credentials (same shape as before)
        let credentials = Credentials::new(
            access_key_id,
            secret_access_key,
            session_token,
            None,               // expiration – None means “no expiry” for static creds
            "graphbit",         // provider name (only for debugging)
        );

        // 2. Construct the SDK config **without** invoking `aws_config::load`.
        let sdk_config = BedrockConfigBuilder::default()
            // Use the latest SDK behavior (v2 client, etc.)
            .behavior_version(BehaviorVersion::latest())
            // Explicit region (string → Region)
            .region(Region::new(region.clone()))
            // Static credentials provider
            .credentials_provider(credentials)
            // Optional: force HTTP/1.1, endpoint overrides, etc. can be added here.
            .build();

        // 3. Create the Bedrock runtime client from the manual config.
        let client = BedrockClient::from_conf(sdk_config);

        Ok(Self {
            client,
            model_id,
        })
    }

    /// Convert internal `GraphBit` messages to a single chat prompt string.
    fn format_messages_for_chat(messages: &[LlmMessage]) -> String {
        let mut formatted = String::new();

        for message in messages {
            match message.role {
                LlmRole::System => writeln!(formatted, "System: {}", message.content).unwrap(),
                LlmRole::User => writeln!(formatted, "User: {}", message.content).unwrap(),
                LlmRole::Assistant => writeln!(formatted, "Assistant: {}", message.content).unwrap(),
                LlmRole::Tool => writeln!(formatted, "Tool: {}", message.content).unwrap(),
            }
        }

        formatted.push_str("Assistant: ");
        formatted
    }

    /// Parse a Bedrock model response into our standard `LlmResponse`.
    fn parse_response(&self, response: BedrockTextResponse) -> GraphBitResult<LlmResponse> {
        // Extract the generated text (some models use `completion`, others `output_text`)
        let content = response
            .completion
            .or(response.output_text)
            .unwrap_or_default()
            .trim()
            .to_string();

        // Estimate usage if not provided by API
        let tokens = (content.len() / 4) as u32;
        let usage = LlmUsage::new(tokens, tokens);

        Ok(LlmResponse::new(content, &self.model_id)
            .with_usage(usage)
            .with_finish_reason(FinishReason::Stop))
    }
}

#[async_trait]
impl LlmProviderTrait for AwsBedrockProvider {
    fn provider_name(&self) -> &str {
        "aws_bedrock"
    }

    fn model_name(&self) -> &str {
        &self.model_id
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let prompt = Self::format_messages_for_chat(&request.messages);

        // Build the Bedrock input JSON
        let body = BedrockTextRequest {
            prompt,
            max_tokens_to_sample: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
        };

        let json_body = serde_json::to_vec(&body).map_err(|e| {
            GraphBitError::llm_provider("aws_bedrock", format!("Failed to serialize body: {e}"))
        })?;

        // Send request via Bedrock SDK
        let response = self
            .client
            .invoke_model()
            .model_id(&self.model_id)
            .content_type("application/json")
            .body(json_body.into())
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("aws_bedrock", format!("Bedrock request failed: {e}"))
            })?;

        // ✅ Fixed: `Blob` is not a stream — just get bytes directly
        let bytes = response.body.into_inner(); // Blob → Vec<u8>

        // Try to parse as JSON
        let parsed: BedrockTextResponse = serde_json::from_slice(&bytes).map_err(|e| {
            let raw = String::from_utf8_lossy(&bytes);
            GraphBitError::llm_provider(
                "aws_bedrock",
                format!("Failed to parse Bedrock response: {e}\nRaw: {raw}"),
            )
        })?;

        self.parse_response(parsed)
    }

    fn supports_function_calling(&self) -> bool {
        false
    }

    fn supports_streaming(&self) -> bool {
        false
    }

    fn max_context_length(&self) -> Option<u32> {
        Some(8192) // Most Bedrock models have larger context windows
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Could be updated with AWS pricing for the specific model
        Some((0.000002, 0.000002))
    }
}

/// Request payload for AWS Bedrock text models.
#[derive(Debug, Serialize)]
struct BedrockTextRequest {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens_to_sample: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

/// Response payload returned by Bedrock models.
/// Supports multiple possible output formats.
#[derive(Debug, Deserialize)]
struct BedrockTextResponse {
    #[serde(default)]
    completion: Option<String>,
    #[serde(default)]
    output_text: Option<String>,
}