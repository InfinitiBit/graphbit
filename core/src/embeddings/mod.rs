//! Embeddings support for `GraphBit`
//!
//! This module provides a unified interface for working with different
//! embedding providers including `HuggingFace` and `OpenAI`.

pub mod huggingface;
pub mod openai;
pub mod providers;

pub use huggingface::HuggingFaceEmbeddingProvider;
pub use openai::OpenAIEmbeddingProvider;
pub use providers::{EmbeddingConfig, EmbeddingProvider, EmbeddingProviderFactory};

use crate::errors::{GraphBitError, GraphBitResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

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
            Self::Single(text) => vec![text.as_str()],
            Self::Multiple(texts) => texts.iter().map(|s| s.as_str()).collect(),
        }
    }

    /// Get the number of texts
    pub fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multiple(texts) => texts.len(),
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Single(text) => text.is_empty(),
            Self::Multiple(texts) => texts.is_empty(),
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
                    }
                    tokio::task::yield_now().await;
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
            let timeout_duration = Duration::from_millis(timeout_ms);
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

