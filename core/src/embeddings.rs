//! Embeddings support for `GraphBit`
//!
//! This module provides a unified interface for working with different
//! embedding providers including `HuggingFace` and `OpenAI`.

use crate::errors::{GraphBitError, GraphBitResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for embedding providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Provider type (e.g., "openai", "huggingface")
    pub provider: EmbeddingProvider,
    /// API key for the provider
    pub api_key: String,
    /// Model name to use for embeddings
    pub model: String,
    /// Base URL for the API (optional, for custom endpoints)
    pub base_url: Option<String>,
    /// Request timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Maximum batch size for processing multiple texts
    pub max_batch_size: Option<usize>,
    /// Additional provider-specific parameters
    pub extra_params: HashMap<String, serde_json::Value>,
}

/// Supported embedding providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingProvider {
    /// `OpenAI` embedding provider
    OpenAI,
    /// `HuggingFace` embedding provider
    HuggingFace,
}

/// Request for generating embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Text(s) to generate embeddings for
    pub input: EmbeddingInput,
    /// Optional user identifier for tracking
    pub user: Option<String>,
    /// Model-specific parameters
    pub params: HashMap<String, serde_json::Value>,
}

/// Input for embedding generation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text input
    Single(String),
    /// Multiple text inputs
    Multiple(Vec<String>),
}

impl EmbeddingInput {
    /// Get the texts as a vector
    pub fn as_texts(&self) -> Vec<&str> {
        match self {
            EmbeddingInput::Single(text) => vec![text.as_str()],
            EmbeddingInput::Multiple(texts) => texts.iter().map(|s| s.as_str()).collect(),
        }
    }

    /// Get the number of texts
    pub fn len(&self) -> usize {
        match self {
            EmbeddingInput::Single(_) => 1,
            EmbeddingInput::Multiple(texts) => texts.len(),
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        match self {
            EmbeddingInput::Single(text) => text.is_empty(),
            EmbeddingInput::Multiple(texts) => texts.is_empty(),
        }
    }
}

/// Response from embedding generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Generated embeddings
    pub embeddings: Vec<Vec<f32>>,
    /// Model used for generation
    pub model: String,
    /// Usage statistics
    pub usage: EmbeddingUsage,
    /// Provider-specific metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Usage statistics for embedding generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    /// Number of tokens processed
    pub prompt_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
}

/// Batch request for processing multiple embedding requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingBatchRequest {
    /// Multiple embedding requests
    pub requests: Vec<EmbeddingRequest>,
    /// Maximum concurrent requests
    pub max_concurrency: Option<usize>,
    /// Timeout for the entire batch in milliseconds
    pub timeout_ms: Option<u64>,
}

/// Batch response for multiple embedding requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingBatchResponse {
    /// Responses corresponding to the requests
    pub responses: Vec<Result<EmbeddingResponse, GraphBitError>>,
    /// Total processing time in milliseconds
    pub total_duration_ms: u64,
    /// Batch processing statistics
    pub stats: EmbeddingBatchStats,
}

/// Statistics for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingBatchStats {
    /// Number of successful requests
    pub successful_requests: usize,
    /// Number of failed requests
    pub failed_requests: usize,
    /// Average response time per request
    pub avg_response_time_ms: f64,
    /// Total embeddings generated
    pub total_embeddings: usize,
    /// Total tokens processed
    pub total_tokens: u32,
}

/// Trait for embedding providers
#[async_trait]
pub trait EmbeddingProviderTrait: Send + Sync {
    /// Generate embeddings for the given request
    async fn generate_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> GraphBitResult<EmbeddingResponse>;

    /// Get the provider name
    fn provider_name(&self) -> &str;

    /// Get the model name
    fn model_name(&self) -> &str;

    /// Get embedding dimensions for this model
    async fn get_embedding_dimensions(&self) -> GraphBitResult<usize>;

    /// Check if the provider supports batch processing
    fn supports_batch(&self) -> bool {
        true
    }

    /// Get maximum batch size supported by the provider
    fn max_batch_size(&self) -> usize {
        100
    }

    /// Validate the configuration
    fn validate_config(&self) -> GraphBitResult<()> {
        Ok(())
    }
}

/// `OpenAI` embedding provider
#[derive(Debug, Clone)]
pub struct OpenAIEmbeddingProvider {
    config: EmbeddingConfig,
    client: reqwest::Client,
}

impl OpenAIEmbeddingProvider {
    /// Create a new `OpenAI` embedding provider
    pub fn new(config: EmbeddingConfig) -> GraphBitResult<Self> {
        if config.provider != EmbeddingProvider::OpenAI {
            return Err(GraphBitError::config(
                "Invalid provider type for OpenAI".to_string(),
            ));
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_seconds.unwrap_or(30),
            ))
            .build()
            .map_err(|e| GraphBitError::llm(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self { config, client })
    }

    /// Get the API base URL
    fn base_url(&self) -> &str {
        self.config
            .base_url
            .as_deref()
            .unwrap_or("https://api.openai.com/v1")
    }
}

#[async_trait]
impl EmbeddingProviderTrait for OpenAIEmbeddingProvider {
    async fn generate_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> GraphBitResult<EmbeddingResponse> {
        let url = format!("{}/embeddings", self.base_url());

        let input = match &request.input {
            EmbeddingInput::Single(text) => serde_json::Value::String(text.clone()),
            EmbeddingInput::Multiple(texts) => serde_json::Value::Array(
                texts
                    .iter()
                    .map(|t| serde_json::Value::String(t.clone()))
                    .collect(),
            ),
        };

        let mut body = serde_json::json!({
            "model": self.config.model,
            "input": input,
        });

        // Add user if provided
        if let Some(user) = &request.user {
            body["user"] = serde_json::Value::String(user.clone());
        }

        // Add extra parameters
        for (key, value) in &request.params {
            body[key] = value.clone();
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| GraphBitError::llm(format!("Failed to send request to OpenAI: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm(format!(
                "OpenAI API error: {error_text}"
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| GraphBitError::llm(format!("Failed to parse OpenAI response: {e}")))?;

        // Parse embeddings
        let embeddings_data = response_json["data"]
            .as_array()
            .ok_or_else(|| GraphBitError::llm("Invalid response format from OpenAI".to_string()))?;

        let mut embeddings = Vec::new();
        for item in embeddings_data {
            let embedding_array = item["embedding"]
                .as_array()
                .ok_or_else(|| GraphBitError::llm("Invalid embedding format".to_string()))?;

            let embedding: Vec<f32> = embedding_array
                .iter()
                .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                .collect();

            embeddings.push(embedding);
        }

        // Parse usage
        let usage_data = &response_json["usage"];
        let usage = EmbeddingUsage {
            prompt_tokens: usage_data["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: usage_data["total_tokens"].as_u64().unwrap_or(0) as u32,
        };

        Ok(EmbeddingResponse {
            embeddings,
            model: response_json["model"]
                .as_str()
                .unwrap_or(&self.config.model)
                .to_string(),
            usage,
            metadata: HashMap::new(),
        })
    }

    fn provider_name(&self) -> &str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }

    async fn get_embedding_dimensions(&self) -> GraphBitResult<usize> {
        // Common `OpenAI` embedding dimensions
        match self.config.model.as_str() {
            "text-embedding-ada-002" => Ok(1536),
            "text-embedding-3-small" => Ok(1536),
            "text-embedding-3-large" => Ok(3072),
            _ => {
                // Make a test request to determine dimensions
                let test_request = EmbeddingRequest {
                    input: EmbeddingInput::Single("test".to_string()),
                    user: None,
                    params: HashMap::new(),
                };
                let response = self.generate_embeddings(test_request).await?;
                Ok(response.embeddings.first().map(|e| e.len()).unwrap_or(1536))
            }
        }
    }

    fn max_batch_size(&self) -> usize {
        2048 // `OpenAI`'s current limit
    }
}

/// `HuggingFace` embedding provider
#[derive(Debug, Clone)]
pub struct HuggingFaceEmbeddingProvider {
    config: EmbeddingConfig,
    client: reqwest::Client,
}

impl HuggingFaceEmbeddingProvider {
    /// Create a new `HuggingFace` embedding provider
    pub fn new(config: EmbeddingConfig) -> GraphBitResult<Self> {
        if config.provider != EmbeddingProvider::HuggingFace {
            return Err(GraphBitError::config(
                "Invalid provider type for HuggingFace".to_string(),
            ));
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_seconds.unwrap_or(60),
            ))
            .build()
            .map_err(|e| GraphBitError::llm(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self { config, client })
    }

    /// Get the API base URL
    fn base_url(&self) -> String {
        self.config
            .base_url
            .as_deref()
            .map(|url| url.to_string())
            .unwrap_or_else(|| {
                format!(
                    "https://api-inference.huggingface.co/models/{}",
                    self.config.model
                )
            })
    }
}

#[async_trait]
impl EmbeddingProviderTrait for HuggingFaceEmbeddingProvider {
    async fn generate_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> GraphBitResult<EmbeddingResponse> {
        let url = self.base_url();

        let inputs = request.input.as_texts();

        let mut body = serde_json::json!({
            "inputs": inputs,
        });

        // Add extra parameters
        let mut options = serde_json::Map::new();
        for (key, value) in &request.params {
            options.insert(key.clone(), value.clone());
        }

        if !options.is_empty() {
            body["options"] = serde_json::Value::Object(options);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                GraphBitError::llm(format!("Failed to send request to HuggingFace: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm(format!(
                "HuggingFace API error: {error_text}"
            )));
        }

        let response_json: serde_json::Value = response.json().await.map_err(|e| {
            GraphBitError::llm(format!("Failed to parse HuggingFace response: {e}"))
        })?;

        // Parse embeddings - `HuggingFace` returns arrays directly
        let embeddings: Vec<Vec<f32>> = if response_json.is_array() {
            response_json
                .as_array()
                .unwrap()
                .iter()
                .map(|item| {
                    if item.is_array() {
                        // Single embedding (2D array)
                        item.as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                            .collect()
                    } else {
                        // Multiple embeddings (3D array) - take first dimension
                        vec![]
                    }
                })
                .filter(|v| !v.is_empty())
                .collect()
        } else {
            return Err(GraphBitError::llm(
                "Unexpected response format from HuggingFace".to_string(),
            ));
        };

        // Estimate token usage (`HuggingFace` doesn't provide this)
        let total_chars: usize = inputs.iter().map(|s| s.len()).sum();
        let estimated_tokens = (total_chars / 4) as u32; // Rough estimate

        let usage = EmbeddingUsage {
            prompt_tokens: estimated_tokens,
            total_tokens: estimated_tokens,
        };

        Ok(EmbeddingResponse {
            embeddings,
            model: self.config.model.clone(),
            usage,
            metadata: HashMap::new(),
        })
    }

    fn provider_name(&self) -> &str {
        "huggingface"
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }

    async fn get_embedding_dimensions(&self) -> GraphBitResult<usize> {
        // Make a test request to determine dimensions
        let test_request = EmbeddingRequest {
            input: EmbeddingInput::Single("test".to_string()),
            user: None,
            params: HashMap::new(),
        };
        let response = self.generate_embeddings(test_request).await?;
        Ok(response.embeddings.first().map(|e| e.len()).unwrap_or(768))
    }

    fn max_batch_size(&self) -> usize {
        100 // Conservative default for `HuggingFace`
    }
}

/// Factory for creating embedding providers
pub struct EmbeddingProviderFactory;

impl EmbeddingProviderFactory {
    /// Create an embedding provider from configuration
    pub fn create_provider(
        config: EmbeddingConfig,
    ) -> GraphBitResult<Box<dyn EmbeddingProviderTrait>> {
        match config.provider {
            EmbeddingProvider::OpenAI => {
                let provider = OpenAIEmbeddingProvider::new(config)?;
                Ok(Box::new(provider))
            }
            EmbeddingProvider::HuggingFace => {
                let provider = HuggingFaceEmbeddingProvider::new(config)?;
                Ok(Box::new(provider))
            }
        }
    }
}

/// Embedding service for high-level operations
pub struct EmbeddingService {
    provider: Box<dyn EmbeddingProviderTrait>,
    config: EmbeddingConfig,
    max_concurrency: usize,
    current_requests: Arc<std::sync::atomic::AtomicUsize>,
}

impl EmbeddingService {
    /// Create a new embedding service
    pub fn new(config: EmbeddingConfig) -> GraphBitResult<Self> {
        let max_concurrency = config.max_batch_size.unwrap_or(10);
        let provider = EmbeddingProviderFactory::create_provider(config.clone())?;

        Ok(Self {
            provider,
            config,
            max_concurrency,
            current_requests: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        })
    }

    /// Generate embeddings for a single text
    pub async fn embed_text(&self, text: &str) -> GraphBitResult<Vec<f32>> {
        let request = EmbeddingRequest {
            input: EmbeddingInput::Single(text.to_string()),
            user: None,
            params: HashMap::new(),
        };

        let response = self.provider.generate_embeddings(request).await?;

        response
            .embeddings
            .into_iter()
            .next()
            .ok_or_else(|| GraphBitError::llm("No embeddings returned".to_string()))
    }

    /// Generate embeddings for multiple texts
    pub async fn embed_texts(&self, texts: &[String]) -> GraphBitResult<Vec<Vec<f32>>> {
        let request = EmbeddingRequest {
            input: EmbeddingInput::Multiple(texts.to_vec()),
            user: None,
            params: HashMap::new(),
        };

        let response = self.provider.generate_embeddings(request).await?;
        Ok(response.embeddings)
    }

    /// Process a batch of embedding requests with lock-free concurrency control
    pub async fn process_batch(
        &self,
        batch: EmbeddingBatchRequest,
    ) -> GraphBitResult<EmbeddingBatchResponse> {
        let start_time = std::time::Instant::now();
        let max_concurrency = batch.max_concurrency.unwrap_or(self.max_concurrency);

        let mut tasks = Vec::with_capacity(batch.requests.len());
        let current_requests = Arc::clone(&self.current_requests);

        for request in batch.requests {
            let config = self.config.clone();
            let current_requests = Arc::clone(&current_requests);

            let task = tokio::spawn(async move {
                // Wait for available slot using atomic operations (no semaphore bottleneck)
                loop {
                    let current = current_requests.load(std::sync::atomic::Ordering::Acquire);
                    if current < max_concurrency {
                        match current_requests.compare_exchange(
                            current,
                            current + 1,
                            std::sync::atomic::Ordering::AcqRel,
                            std::sync::atomic::Ordering::Acquire,
                        ) {
                            Ok(_) => break,     // Successfully acquired slot
                            Err(_) => continue, // Retry
                        }
                    } else {
                        // Brief yield to avoid busy waiting
                        tokio::task::yield_now().await;
                    }
                }

                // Execute the request
                let result = async {
                    let provider = EmbeddingProviderFactory::create_provider(config)?;
                    provider.generate_embeddings(request).await
                }
                .await;

                // Release slot
                current_requests.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);

                result
            });

            tasks.push(task);
        }

        // Wait for all tasks with optional timeout
        let responses = if let Some(timeout_ms) = batch.timeout_ms {
            let timeout_duration = tokio::time::Duration::from_millis(timeout_ms);
            match tokio::time::timeout(timeout_duration, futures::future::join_all(tasks)).await {
                Ok(results) => results,
                Err(_) => return Err(GraphBitError::llm("Batch request timed out".to_string())),
            }
        } else {
            futures::future::join_all(tasks).await
        };

        // Process results
        let mut successful = 0;
        let mut failed = 0;
        let mut total_embeddings = 0;
        let mut total_tokens = 0;

        let final_responses: Vec<Result<EmbeddingResponse, GraphBitError>> = responses
            .into_iter()
            .map(|task_result| match task_result {
                Ok(embedding_result) => match embedding_result {
                    Ok(response) => {
                        successful += 1;
                        total_embeddings += response.embeddings.len();
                        total_tokens += response.usage.total_tokens;
                        Ok(response)
                    }
                    Err(e) => {
                        failed += 1;
                        Err(e)
                    }
                },
                Err(e) => {
                    failed += 1;
                    Err(GraphBitError::llm(format!("Task execution failed: {e}")))
                }
            })
            .collect();

        let total_duration_ms = start_time.elapsed().as_millis() as u64;
        let avg_response_time_ms = if total_duration_ms > 0 && successful > 0 {
            total_duration_ms as f64 / successful as f64
        } else {
            0.0
        };

        Ok(EmbeddingBatchResponse {
            responses: final_responses,
            total_duration_ms,
            stats: EmbeddingBatchStats {
                successful_requests: successful,
                failed_requests: failed,
                avg_response_time_ms,
                total_embeddings,
                total_tokens,
            },
        })
    }

    /// Calculate cosine similarity between two embeddings
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> GraphBitResult<f32> {
        if a.len() != b.len() {
            return Err(GraphBitError::validation(
                "dimensions".to_string(),
                "Embedding dimensions must match".to_string(),
            ));
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(0.0);
        }

        Ok(dot_product / (norm_a * norm_b))
    }

    /// Get embedding dimensions for the current provider
    pub async fn get_dimensions(&self) -> GraphBitResult<usize> {
        self.provider.get_embedding_dimensions().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_input() {
        let single = EmbeddingInput::Single("test".to_string());
        assert_eq!(single.len(), 1);
        assert_eq!(single.as_texts(), vec!["test"]);

        let multiple = EmbeddingInput::Multiple(vec!["test1".to_string(), "test2".to_string()]);
        assert_eq!(multiple.len(), 2);
        assert_eq!(multiple.as_texts(), vec!["test1", "test2"]);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let similarity = EmbeddingService::cosine_similarity(&a, &b).unwrap();
        assert!((similarity - 0.0).abs() < f32::EPSILON);

        let a = vec![1.0, 0.0];
        let b = vec![1.0, 0.0];
        let similarity = EmbeddingService::cosine_similarity(&a, &b).unwrap();
        assert!((similarity - 1.0).abs() < f32::EPSILON);
    }
}
