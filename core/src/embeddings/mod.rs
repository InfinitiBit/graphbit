//! Embeddings support for `GraphBit`
//!
//! This module provides a unified interface for working with different
//! embedding providers including `HuggingFace` and `OpenAI`.

mod azure;
mod huggingface;
mod openai;
pub mod python_bridge;
mod types;

pub use azure::AzureEmbeddingProvider;
pub use huggingface::HuggingFaceEmbeddingProvider;
pub use openai::OpenAIEmbeddingProvider;
pub use python_bridge::PythonBridgeEmbeddingProvider;
pub use types::*;

use std::collections::HashMap;
use std::sync::Arc;

use crate::errors::{GraphBitError, GraphBitResult};

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
            EmbeddingProvider::Azure => {
                let provider = AzureEmbeddingProvider::new(config)?;
                Ok(Box::new(provider))
            }
            EmbeddingProvider::HuggingFace => {
                let provider = HuggingFaceEmbeddingProvider::new(config)?;
                Ok(Box::new(provider))
            }
            #[cfg(feature = "python")]
            EmbeddingProvider::PythonBridge => {
                let provider = PythonBridgeEmbeddingProvider::new(config)?;
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
                loop {
                    let current = current_requests.load(std::sync::atomic::Ordering::Acquire);
                    if current < max_concurrency {
                        match current_requests.compare_exchange(
                            current,
                            current + 1,
                            std::sync::atomic::Ordering::AcqRel,
                            std::sync::atomic::Ordering::Acquire,
                        ) {
                            Ok(_) => break,
                            Err(_) => continue,
                        }
                    }
                    tokio::task::yield_now().await;
                }

                let result = async {
                    let provider = EmbeddingProviderFactory::create_provider(config)?;
                    provider.generate_embeddings(request).await
                }
                .await;

                current_requests.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);

                result
            });

            tasks.push(task);
        }

        let responses = if let Some(timeout_ms) = batch.timeout_ms {
            let timeout_duration = tokio::time::Duration::from_millis(timeout_ms);
            match tokio::time::timeout(timeout_duration, futures::future::join_all(tasks)).await {
                Ok(results) => results,
                Err(_) => return Err(GraphBitError::llm("Batch request timed out".to_string())),
            }
        } else {
            futures::future::join_all(tasks).await
        };

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
