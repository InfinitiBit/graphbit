//! Embeddings bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::embeddings::{
    EmbeddingConfig as CoreEmbeddingConfig,
    EmbeddingProvider,
    EmbeddingService,
    EmbeddingResponse as CoreEmbeddingResponse,
};
use std::collections::HashMap;

/// Embedding configuration
#[napi]
pub struct EmbeddingConfig {
    inner: CoreEmbeddingConfig,
}

impl EmbeddingConfig {
    pub(crate) fn clone_inner(&self) -> CoreEmbeddingConfig {
        self.inner.clone()
    }
}

#[napi]
impl EmbeddingConfig {
    /// Create OpenAI embedding configuration
    #[napi(factory)]
    pub fn openai(api_key: String, model: Option<String>) -> Self {
        Self {
            inner: CoreEmbeddingConfig {
                provider: EmbeddingProvider::OpenAI,
                api_key,
                model: model.unwrap_or_else(|| "text-embedding-3-small".to_string()),
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
            },
        }
    }

    /// Create HuggingFace embedding configuration
    #[napi(factory)]
    pub fn huggingface(api_key: String, model: String) -> Self {
        Self {
            inner: CoreEmbeddingConfig {
                provider: EmbeddingProvider::HuggingFace,
                api_key,
                model,
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
            },
        }
    }
}

/// Embedding request
#[napi(object)]
pub struct EmbeddingRequest {
    pub texts: Vec<String>,
    pub model: Option<String>,
}

/// Embedding response
#[napi(object)]
pub struct EmbeddingResponse {
    pub embeddings: Vec<Vec<f64>>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

/// Embedding usage statistics
#[napi(object)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

impl From<CoreEmbeddingResponse> for EmbeddingResponse {
    fn from(response: CoreEmbeddingResponse) -> Self {
        // Convert Vec<Vec<f32>> to Vec<Vec<f64>>
        let embeddings = response.embeddings
            .into_iter()
            .map(|vec| vec.into_iter().map(|f| f as f64).collect())
            .collect();

        Self {
            embeddings,
            model: response.model,
            usage: EmbeddingUsage {
                prompt_tokens: response.usage.prompt_tokens,
                total_tokens: response.usage.total_tokens,
            },
        }
    }
}

/// Embedding service
#[napi]
pub struct EmbeddingClient {
    service: EmbeddingService,
    model: String,
}

#[napi]
impl EmbeddingClient {
    /// Create a new embedding client
    #[napi(constructor)]
    pub fn new(config: &EmbeddingConfig) -> Result<Self> {
        let model = config.clone_inner().model.clone();
        let service = EmbeddingService::new(config.clone_inner())
            .map_err(crate::errors::to_napi_error)?;
        Ok(Self { service, model })
    }

    /// Generate embeddings for texts
    #[napi]
    pub async fn embed(&self, texts: Vec<String>) -> Result<EmbeddingResponse> {
        let embeddings = self.service.embed_texts(&texts)
            .await
            .map_err(crate::errors::to_napi_error)?;

        // Convert Vec<Vec<f32>> to Vec<Vec<f64>>
        let embeddings_f64 = embeddings
            .into_iter()
            .map(|vec| vec.into_iter().map(|f| f as f64).collect())
            .collect();

        Ok(EmbeddingResponse {
            embeddings: embeddings_f64,
            model: self.model.clone(),
            usage: EmbeddingUsage {
                prompt_tokens: 0,
                total_tokens: 0,
            },
        })
    }

    /// Calculate cosine similarity between two embeddings
    ///
    /// Returns a similarity score between -1.0 and 1.0, where:
    /// - 1.0 means identical vectors
    /// - 0.0 means orthogonal (unrelated)
    /// - -1.0 means opposite vectors
    ///
    /// # Arguments
    /// * `embedding1` - First embedding vector
    /// * `embedding2` - Second embedding vector
    ///
    /// # Returns
    /// Cosine similarity score
    ///
    /// # Example
    ///
    /// ```javascript
    /// const emb1 = [0.1, 0.2, 0.3, 0.4];
    /// const emb2 = [0.15, 0.25, 0.35, 0.45];
    /// 
    /// const similarity = EmbeddingClient.similarity(emb1, emb2);
    /// console.log(`Similarity: ${similarity.toFixed(4)}`); // ~0.9999
    ///
    /// if (similarity > 0.8) {
    ///   console.log('Highly similar!');
    /// }
    /// ```
    #[napi]
    pub fn similarity(embedding1: Vec<f64>, embedding2: Vec<f64>) -> Result<f64> {
        // Validate inputs
        if embedding1.is_empty() || embedding2.is_empty() {
            return Err(Error::from_reason("Embeddings cannot be empty"));
        }

        if embedding1.len() != embedding2.len() {
            return Err(Error::from_reason(format!(
                "Embeddings must have same length (got {} and {})",
                embedding1.len(),
                embedding2.len()
            )));
        }

        // Convert to f32 for core library
        let emb1_f32: Vec<f32> = embedding1.iter().map(|&x| x as f32).collect();
        let emb2_f32: Vec<f32> = embedding2.iter().map(|&x| x as f32).collect();

        // Calculate cosine similarity
        graphbit_core::embeddings::EmbeddingService::cosine_similarity(&emb1_f32, &emb2_f32)
            .map(|s| s as f64)
            .map_err(crate::errors::to_napi_error)
    }
}

/// Batch embedding options
#[napi(object)]
pub struct BatchEmbeddingOptions {
    /// Maximum concurrent requests
    pub max_concurrency: Option<u32>,
    /// Timeout for entire batch in milliseconds
    pub timeout_ms: Option<u32>,
}

/// Batch embedding statistics
#[napi(object)]
pub struct BatchEmbeddingStats {
    /// Number of successful requests
    pub successful_requests: u32,
    /// Number of failed requests
    pub failed_requests: u32,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Total embeddings generated
    pub total_embeddings: u32,
    /// Total processing time in milliseconds
    pub total_duration_ms: f64,
}

/// Batch embedding result
#[napi(object)]
pub struct BatchEmbeddingResult {
    /// Array of embedding arrays (one per batch)
    pub embeddings: Vec<Vec<Vec<f64>>>,
    /// Errors for failed batches
    pub errors: Vec<String>,
    /// Processing statistics
    pub stats: BatchEmbeddingStats,
}

// Note: Batch parallel processing requires more complex async handling
// For now, we provide the similarity method which is the most requested feature

