//! `Replicate` AI LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

/// `Replicate` AI API provider
pub struct ReplicateProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
    max_wait_time: Duration,
    poll_interval: Duration,
}

impl ReplicateProvider {
    /// Create a new `Replicate` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        // Optimized client with connection pooling for better performance
        let client = Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minutes timeout for long-running predictions
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30))
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .map_err(|e| {
                GraphBitError::llm_provider("replicate", format!("Failed to create HTTP client: {e}"))
            })?;
        
        let base_url = "https://api.replicate.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            max_wait_time: Duration::from_secs(300), // 5 minutes max wait
            poll_interval: Duration::from_secs(2),   // Poll every 2 seconds
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
                GraphBitError::llm_provider("replicate", format!("Failed to create HTTP client: {e}"))
            })?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            max_wait_time: Duration::from_secs(300),
            poll_interval: Duration::from_secs(2),
        })
    }

    /// Set custom wait time and poll interval
    pub fn with_timing(mut self, max_wait_time: Duration, poll_interval: Duration) -> Self {
        self.max_wait_time = max_wait_time;
        self.poll_interval = poll_interval;
        self
    }

    /// Convert `GraphBit` messages to `Replicate` input format
    fn convert_messages_to_input(&self, messages: &[LlmMessage]) -> serde_json::Value {
        // For most Replicate models, we need to format messages as a single prompt
        // This is a simplified approach - different models may require different formats
        let mut prompt_parts = Vec::new();
        
        for message in messages {
            match message.role {
                LlmRole::System => {
                    prompt_parts.push(format!("System: {}", message.content));
                }
                LlmRole::User => {
                    prompt_parts.push(format!("User: {}", message.content));
                }
                LlmRole::Assistant => {
                    prompt_parts.push(format!("Assistant: {}", message.content));
                }
                LlmRole::Tool => {
                    prompt_parts.push(format!("Tool: {}", message.content));
                }
            }
        }
        
        serde_json::json!({
            "prompt": prompt_parts.join("\n\n")
        })
    }

    /// Create a prediction on Replicate
    async fn create_prediction(&self, input: serde_json::Value) -> GraphBitResult<ReplicatePrediction> {
        let url = format!("{}/predictions", self.base_url);
        
        let body = ReplicateCreateRequest {
            version: self.model.clone(),
            input,
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

        let prediction: ReplicatePrediction = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("replicate", format!("Failed to parse response: {e}"))
        })?;

        Ok(prediction)
    }

    /// Get prediction status and result
    async fn get_prediction(&self, prediction_id: &str) -> GraphBitResult<ReplicatePrediction> {
        let url = format!("{}/predictions/{}", self.base_url, prediction_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
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

        let prediction: ReplicatePrediction = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("replicate", format!("Failed to parse response: {e}"))
        })?;

        Ok(prediction)
    }

    /// Wait for prediction to complete and return result
    async fn wait_for_completion(&self, prediction_id: &str) -> GraphBitResult<ReplicatePrediction> {
        let start_time = std::time::Instant::now();
        
        loop {
            let prediction = self.get_prediction(prediction_id).await?;
            
            match prediction.status.as_str() {
                "succeeded" => return Ok(prediction),
                "failed" => {
                    let error_msg = prediction.error.unwrap_or_else(|| "Unknown error".to_string());
                    return Err(GraphBitError::llm_provider(
                        "replicate",
                        format!("Prediction failed: {error_msg}"),
                    ));
                }
                "canceled" => {
                    return Err(GraphBitError::llm_provider(
                        "replicate",
                        "Prediction was canceled".to_string(),
                    ));
                }
                "starting" | "processing" => {
                    // Continue polling
                    if start_time.elapsed() > self.max_wait_time {
                        return Err(GraphBitError::llm_provider(
                            "replicate",
                            format!("Prediction timed out after {:?}", self.max_wait_time),
                        ));
                    }
                    
                    sleep(self.poll_interval).await;
                }
                _ => {
                    // Unknown status, continue polling but log it
                    tracing::warn!("Unknown prediction status: {}", prediction.status);
                    if start_time.elapsed() > self.max_wait_time {
                        return Err(GraphBitError::llm_provider(
                            "replicate",
                            format!("Prediction timed out after {:?}", self.max_wait_time),
                        ));
                    }
                    
                    sleep(self.poll_interval).await;
                }
            }
        }
    }

    /// Parse Replicate prediction output to GraphBit response
    fn parse_prediction_output(&self, prediction: &ReplicatePrediction) -> GraphBitResult<LlmResponse> {
        let content = match &prediction.output {
            Some(output) => {
                // Handle different output formats
                match output {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Array(arr) => {
                        // Join array elements (common for text generation)
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join("")
                    }
                    serde_json::Value::Object(obj) => {
                        // Try to extract text from common fields
                        if let Some(text) = obj.get("text").and_then(|v| v.as_str()) {
                            text.to_string()
                        } else if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                            content.to_string()
                        } else {
                            // Fallback to JSON string representation
                            serde_json::to_string(output).unwrap_or_else(|_| "".to_string())
                        }
                    }
                    _ => serde_json::to_string(output).unwrap_or_else(|_| "".to_string()),
                }
            }
            None => String::new(),
        };

        // Extract usage information if available
        let usage = if let Some(metrics) = &prediction.metrics {
            let _predict_time = metrics.get("predict_time")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            
            // Estimate token usage based on content length (rough approximation)
            let estimated_prompt_tokens = content.len() as u32 / 4; // Rough estimate
            let estimated_completion_tokens = content.len() as u32 / 4;
            
            LlmUsage::new(estimated_prompt_tokens, estimated_completion_tokens)
        } else {
            LlmUsage::empty()
        };

        Ok(LlmResponse::new(content, &self.model)
            .with_usage(usage)
            .with_finish_reason(FinishReason::Stop)
            .with_id(prediction.id.clone()))
    }
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
        tracing::info!("Creating Replicate prediction for model: {}", self.model);

        // Convert messages to Replicate input format
        let mut input = self.convert_messages_to_input(&request.messages);

        // Add additional parameters from the request
        if let Some(max_tokens) = request.max_tokens {
            if let serde_json::Value::Object(ref mut map) = input {
                map.insert("max_tokens".to_string(), serde_json::Value::Number(max_tokens.into()));
            }
        }

        if let Some(temperature) = request.temperature {
            if let serde_json::Value::Object(ref mut map) = input {
                map.insert("temperature".to_string(), serde_json::Value::Number(
                    serde_json::Number::from_f64(temperature as f64).unwrap_or_else(|| serde_json::Number::from(0))
                ));
            }
        }

        if let Some(top_p) = request.top_p {
            if let serde_json::Value::Object(ref mut map) = input {
                map.insert("top_p".to_string(), serde_json::Value::Number(
                    serde_json::Number::from_f64(top_p as f64).unwrap_or_else(|| serde_json::Number::from(0))
                ));
            }
        }

        // Add extra parameters
        if let serde_json::Value::Object(ref mut map) = input {
            for (key, value) in request.extra_params {
                map.insert(key, value);
            }
        }

        // Create prediction
        let prediction = self.create_prediction(input).await?;
        tracing::info!("Created prediction with ID: {}", prediction.id);

        // Wait for completion
        let completed_prediction = self.wait_for_completion(&prediction.id).await?;
        tracing::info!("Prediction completed: {}", completed_prediction.id);

        // Parse and return response
        self.parse_prediction_output(&completed_prediction)
    }

    fn supports_streaming(&self) -> bool {
        false // Replicate doesn't support streaming in the traditional sense
    }

    fn supports_function_calling(&self) -> bool {
        false // Most Replicate models don't support function calling
    }

    fn max_context_length(&self) -> Option<u32> {
        // This varies by model, return None to indicate unknown
        None
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        // Replicate pricing is typically per prediction, not per token
        None
    }
}

// Replicate API request structures
#[derive(Debug, Serialize)]
struct ReplicateCreateRequest {
    version: String,
    input: serde_json::Value,
}

// Replicate API response structures
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Some fields are part of API response but not used in current implementation
struct ReplicatePrediction {
    id: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    output: Option<serde_json::Value>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    logs: Option<String>,
    #[serde(default)]
    metrics: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    started_at: Option<String>,
    #[serde(default)]
    completed_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmMessage;
    use serde_json::json;

    #[test]
    fn test_replicate_provider_creation() {
        let provider = ReplicateProvider::new(
            "test-api-key".to_string(),
            "meta/llama-2-70b-chat:test-version".to_string(),
        );
        assert!(provider.is_ok());

        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "replicate");
        assert_eq!(provider.model_name(), "meta/llama-2-70b-chat:test-version");
        assert!(!provider.supports_streaming());
        assert!(!provider.supports_function_calling());
        assert_eq!(provider.max_context_length(), None);
        assert_eq!(provider.cost_per_token(), None);
    }

    #[test]
    fn test_replicate_provider_with_custom_timing() {
        let provider = ReplicateProvider::new(
            "test-api-key".to_string(),
            "test-model".to_string(),
        )
        .unwrap()
        .with_timing(
            Duration::from_secs(600),
            Duration::from_secs(5),
        );

        assert_eq!(provider.max_wait_time, Duration::from_secs(600));
        assert_eq!(provider.poll_interval, Duration::from_secs(5));
    }

    #[test]
    fn test_convert_messages_to_input() {
        let provider = ReplicateProvider::new(
            "test-api-key".to_string(),
            "test-model".to_string(),
        ).unwrap();

        let messages = vec![
            LlmMessage::system("You are a helpful assistant."),
            LlmMessage::user("Hello, how are you?"),
            LlmMessage::assistant("I'm doing well, thank you!"),
        ];

        let input = provider.convert_messages_to_input(&messages);

        let expected_prompt = "System: You are a helpful assistant.\n\nUser: Hello, how are you?\n\nAssistant: I'm doing well, thank you!";
        assert_eq!(input["prompt"], expected_prompt);
    }

    #[test]
    fn test_parse_prediction_output_string() {
        let provider = ReplicateProvider::new(
            "test-api-key".to_string(),
            "test-model".to_string(),
        ).unwrap();

        let prediction = ReplicatePrediction {
            id: "test-id".to_string(),
            status: "succeeded".to_string(),
            output: Some(json!("Hello, world!")),
            error: None,
            logs: None,
            metrics: None,
            created_at: None,
            started_at: None,
            completed_at: None,
        };

        let response = provider.parse_prediction_output(&prediction).unwrap();
        assert_eq!(response.content, "Hello, world!");
        assert_eq!(response.model, "test-model");
        assert_eq!(response.id, Some("test-id".to_string()));
    }

    #[test]
    fn test_parse_prediction_output_array() {
        let provider = ReplicateProvider::new(
            "test-api-key".to_string(),
            "test-model".to_string(),
        ).unwrap();

        let prediction = ReplicatePrediction {
            id: "test-id".to_string(),
            status: "succeeded".to_string(),
            output: Some(json!(["Hello, ", "world!"])),
            error: None,
            logs: None,
            metrics: None,
            created_at: None,
            started_at: None,
            completed_at: None,
        };

        let response = provider.parse_prediction_output(&prediction).unwrap();
        assert_eq!(response.content, "Hello, world!");
    }

    #[test]
    fn test_parse_prediction_output_object() {
        let provider = ReplicateProvider::new(
            "test-api-key".to_string(),
            "test-model".to_string(),
        ).unwrap();

        let prediction = ReplicatePrediction {
            id: "test-id".to_string(),
            status: "succeeded".to_string(),
            output: Some(json!({"text": "Hello from object!"})),
            error: None,
            logs: None,
            metrics: None,
            created_at: None,
            started_at: None,
            completed_at: None,
        };

        let response = provider.parse_prediction_output(&prediction).unwrap();
        assert_eq!(response.content, "Hello from object!");
    }

    #[test]
    fn test_parse_prediction_output_with_metrics() {
        let provider = ReplicateProvider::new(
            "test-api-key".to_string(),
            "test-model".to_string(),
        ).unwrap();

        let mut metrics = serde_json::Map::new();
        metrics.insert("predict_time".to_string(), json!(1.5));

        let prediction = ReplicatePrediction {
            id: "test-id".to_string(),
            status: "succeeded".to_string(),
            output: Some(json!("Test output")),
            error: None,
            logs: None,
            metrics: Some(metrics),
            created_at: None,
            started_at: None,
            completed_at: None,
        };

        let response = provider.parse_prediction_output(&prediction).unwrap();
        assert_eq!(response.content, "Test output");
        // Usage should be estimated based on content length
        assert!(response.usage.total_tokens > 0);
    }

    #[test]
    fn test_replicate_create_request_serialization() {
        let request = ReplicateCreateRequest {
            version: "test-version".to_string(),
            input: json!({"prompt": "Hello, world!"}),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("test-version"));
        assert!(serialized.contains("Hello, world!"));
    }

    #[test]
    fn test_replicate_prediction_deserialization() {
        let json_str = r#"{
            "id": "test-id",
            "status": "succeeded",
            "output": "Hello, world!",
            "created_at": "2023-01-01T00:00:00Z"
        }"#;

        let prediction: ReplicatePrediction = serde_json::from_str(json_str).unwrap();
        assert_eq!(prediction.id, "test-id");
        assert_eq!(prediction.status, "succeeded");
        assert_eq!(prediction.output, Some(json!("Hello, world!")));
        assert_eq!(prediction.created_at, Some("2023-01-01T00:00:00Z".to_string()));
    }
}
