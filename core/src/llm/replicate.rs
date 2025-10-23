//! `Replicate` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// `Replicate` API provider
pub struct ReplicateProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
    version: Option<String>,
}

impl ReplicateProvider {
    /// Create a new `Replicate` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300)) // Longer timeout for Replicate predictions
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30))
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "replicate",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;
        let base_url = "https://api.replicate.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            version: None,
        })
    }

    /// Create a new `Replicate` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30))
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "replicate",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            version: None,
        })
    }

    /// Set model version
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Convert `GraphBit` messages to Replicate prompt format
    fn convert_messages_to_prompt(messages: &[LlmMessage]) -> String {
        let mut prompt = String::new();

        for message in messages {
            match message.role {
                LlmRole::System => {
                    prompt.push_str(&format!("System: {}\n", message.content));
                }
                LlmRole::User => {
                    prompt.push_str(&format!("User: {}\n", message.content));
                }
                LlmRole::Assistant => {
                    prompt.push_str(&format!("Assistant: {}\n", message.content));

                    // Add tool calls if present
                    for tool_call in &message.tool_calls {
                        prompt.push_str(&format!(
                            "Tool Call: {} with parameters: {}\n",
                            tool_call.name, tool_call.parameters
                        ));
                    }
                }
                LlmRole::Tool => {
                    prompt.push_str(&format!("Tool Result: {}\n", message.content));
                }
            }
        }

        // Add instruction for tool calling if tools are available
        prompt.push_str("Assistant: ");
        prompt
    }

    /// Convert tools to function calling format for supported models
    fn format_tools_for_prompt(tools: &[LlmTool]) -> String {
        if tools.is_empty() {
            return String::new();
        }

        let mut tools_prompt = String::from("You have access to the following tools:\n");
        for tool in tools {
            tools_prompt.push_str(&format!(
                "- {}: {} (Parameters: {})\n",
                tool.name, tool.description, tool.parameters
            ));
        }
        tools_prompt.push_str(
            "When you need to use a tool, respond with: TOOL_CALL: tool_name(parameters)\n\n",
        );
        tools_prompt
    }

    /// Parse tool calls from response text
    fn parse_tool_calls_from_response(content: &str) -> (String, Vec<LlmToolCall>) {
        let mut tool_calls = Vec::new();
        let mut clean_content = String::new();
        let mut tool_call_id = 0;

        for line in content.lines() {
            if line.starts_with("TOOL_CALL:") {
                // Parse tool call format: TOOL_CALL: tool_name(parameters)
                if let Some(tool_part) = line.strip_prefix("TOOL_CALL:").map(|s| s.trim()) {
                    if let Some(paren_pos) = tool_part.find('(') {
                        let tool_name = tool_part[..paren_pos].trim().to_string();
                        let params_str = &tool_part[paren_pos + 1..];
                        if let Some(end_paren) = params_str.rfind(')') {
                            let params = &params_str[..end_paren];

                            // Try to parse as JSON, fallback to simple string
                            let parameters = if let Ok(json_params) = serde_json::from_str(params) {
                                json_params
                            } else {
                                serde_json::json!({"input": params})
                            };

                            tool_calls.push(LlmToolCall {
                                id: format!("call_{}", tool_call_id),
                                name: tool_name,
                                parameters,
                            });
                            tool_call_id += 1;
                        }
                    }
                }
            } else {
                clean_content.push_str(line);
                clean_content.push('\n');
            }
        }

        (clean_content.trim().to_string(), tool_calls)
    }

    /// Get the model identifier for API calls
    fn get_model_identifier(&self) -> String {
        if let Some(version) = &self.version {
            format!("{}:{}", self.model, version)
        } else {
            self.model.clone()
        }
    }

    /// Check if this model supports function calling
    fn model_supports_function_calling(&self) -> bool {
        // List of known function calling models on Replicate
        let function_calling_models = [
            "openai/gpt-5",
            "openai/gpt-5-structured",
            "lucataco/glaive-function-calling-v1",
            "homanp/llama-2-13b-function-calling",
            "lucataco/hermes-2-pro-llama-3-8b",
            "lucataco/dolphin-2.9-llama3-8b",
            "ibm-granite/granite-3.3-8b-instruct",
        ];

        function_calling_models
            .iter()
            .any(|&model| self.model.starts_with(model))
    }
}

/// Replicate prediction request structure
#[derive(Debug, Serialize)]
struct ReplicatePredictionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    input: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<String>,
}

/// Replicate prediction response structure
#[derive(Debug, Deserialize)]
struct ReplicatePredictionResponse {
    id: String,
    status: String,
    #[serde(default)]
    output: Option<serde_json::Value>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    metrics: Option<ReplicateMetrics>,
}

/// Replicate metrics structure
#[derive(Debug, Deserialize)]
struct ReplicateMetrics {
    #[serde(default)]
    predict_time: Option<f64>,
}

#[async_trait]
impl LlmProviderTrait for ReplicateProvider {
    fn provider_name(&self) -> &str {
        "replicate"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let model_identifier = self.get_model_identifier();

        // Convert messages to prompt format
        let mut prompt = String::new();

        // Add tools information if available and model supports it
        if !request.tools.is_empty() && self.model_supports_function_calling() {
            prompt.push_str(&Self::format_tools_for_prompt(&request.tools));
        }

        prompt.push_str(&Self::convert_messages_to_prompt(&request.messages));

        // Prepare input based on model type
        let mut input = serde_json::json!({
            "prompt": prompt
        });

        // Add common parameters
        if let Some(max_tokens) = request.max_tokens {
            input["max_new_tokens"] = serde_json::Value::Number(max_tokens.into());
        }
        if let Some(temperature) = request.temperature {
            input["temperature"] = serde_json::json!(temperature);
        }
        if let Some(top_p) = request.top_p {
            input["top_p"] = serde_json::json!(top_p);
        }

        // Add extra parameters
        for (key, value) in request.extra_params {
            input[key] = value;
        }

        let prediction_request = if self.version.is_some() {
            ReplicatePredictionRequest {
                version: self.version.clone(),
                input,
                webhook: None,
            }
        } else {
            // For models without explicit version, we need to use the model endpoint
            ReplicatePredictionRequest {
                version: Some(model_identifier),
                input,
                webhook: None,
            }
        };

        // Create prediction
        let url = format!("{}/predictions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&prediction_request)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm_provider("replicate", format!("Request failed: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "replicate",
                format!("API error: {error_text}"),
            ));
        }

        let mut prediction: ReplicatePredictionResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("replicate", format!("Failed to parse response: {e}"))
        })?;

        // Poll for completion if not already completed
        if prediction.status == "starting" || prediction.status == "processing" {
            prediction = self.poll_prediction(&prediction.id).await?;
        }

        self.parse_prediction_response(prediction)
    }

    fn supports_function_calling(&self) -> bool {
        self.model_supports_function_calling()
    }

    fn max_context_length(&self) -> Option<u32> {
        // Different models have different context lengths
        if self.model.contains("openai/gpt-5") {
            Some(128_000) // GPT-5 has 128K context length
        } else if self.model.contains("llama-2-13b") {
            Some(4096)
        } else if self.model.contains("glaive-function-calling") {
            Some(10_192)
        } else if self.model.contains("hermes-2-pro") {
            Some(8192)
        } else if self.model.contains("dolphin") {
            Some(8192)
        } else if self.model.contains("granite-3.3") {
            Some(8192)
        } else {
            Some(4096) // Default fallback
        }
    }
}

impl ReplicateProvider {
    /// Poll for prediction completion
    async fn poll_prediction(
        &self,
        prediction_id: &str,
    ) -> GraphBitResult<ReplicatePredictionResponse> {
        let url = format!("{}/predictions/{}", self.base_url, prediction_id);
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 60; // 5 minutes with 5-second intervals

        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            attempts += 1;

            let response = self
                .client
                .get(&url)
                .header("Authorization", format!("Token {}", self.api_key))
                .send()
                .await
                .map_err(|e| {
                    GraphBitError::llm_provider("replicate", format!("Polling request failed: {e}"))
                })?;

            if !response.status().is_success() {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(GraphBitError::llm_provider(
                    "replicate",
                    format!("Polling API error: {error_text}"),
                ));
            }

            let prediction: ReplicatePredictionResponse = response.json().await.map_err(|e| {
                GraphBitError::llm_provider(
                    "replicate",
                    format!("Failed to parse polling response: {e}"),
                )
            })?;

            match prediction.status.as_str() {
                "succeeded" => return Ok(prediction),
                "failed" | "canceled" => {
                    let error_msg = prediction.error.unwrap_or_else(|| {
                        format!(
                            "Prediction {} with status: {}",
                            prediction.status, prediction.status
                        )
                    });
                    return Err(GraphBitError::llm_provider("replicate", error_msg));
                }
                "starting" | "processing" => {
                    if attempts >= MAX_ATTEMPTS {
                        return Err(GraphBitError::llm_provider(
                            "replicate",
                            "Prediction timed out after 5 minutes".to_string(),
                        ));
                    }
                    // Continue polling
                }
                _ => {
                    return Err(GraphBitError::llm_provider(
                        "replicate",
                        format!("Unknown prediction status: {}", prediction.status),
                    ));
                }
            }
        }
    }

    /// Parse prediction response into LlmResponse
    fn parse_prediction_response(
        &self,
        prediction: ReplicatePredictionResponse,
    ) -> GraphBitResult<LlmResponse> {
        if prediction.status != "succeeded" {
            let error_msg = prediction
                .error
                .unwrap_or_else(|| format!("Prediction failed with status: {}", prediction.status));
            return Err(GraphBitError::llm_provider("replicate", error_msg));
        }

        let output = prediction.output.ok_or_else(|| {
            GraphBitError::llm_provider(
                "replicate",
                "No output in successful prediction".to_string(),
            )
        })?;

        // Extract content from output
        let content = match output {
            serde_json::Value::String(s) => s,
            serde_json::Value::Array(arr) => {
                // Join array elements (common for streaming models)
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => output.to_string(),
        };

        // Parse tool calls if present
        let (clean_content, tool_calls) = if self.model_supports_function_calling() {
            Self::parse_tool_calls_from_response(&content)
        } else {
            (content, Vec::new())
        };

        // Create usage information
        let usage = LlmUsage::new(
            0, // Replicate doesn't provide input token count
            u32::try_from(clean_content.split_whitespace().count()), // Rough estimate
        );

        let finish_reason = if !tool_calls.is_empty() {
            FinishReason::ToolCalls
        } else {
            FinishReason::Stop
        };

        let mut response = LlmResponse::new(clean_content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(prediction.id);

        // Add prediction time to metadata if available
        if let Some(metrics) = prediction.metrics {
            if let Some(predict_time) = metrics.predict_time {
                response = response.with_metadata(
                    "predict_time_seconds".to_string(),
                    serde_json::json!(predict_time),
                );
            }
        }

        Ok(response)
    }
}
