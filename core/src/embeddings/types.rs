//! Embedding types, configuration, and provider trait.

#[cfg(feature = "python")]
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::{GraphBitError, GraphBitResult};

#[cfg(feature = "python")]
fn default_python_instance() -> Option<Arc<pyo3::PyObject>> {
    None
}

/// Configuration for embedding providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Provider type (e.g., "openai", "huggingface", "pythonbridge")
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
    /// Python instance for PythonBridge provider (not serialized)
    #[cfg(feature = "python")]
    #[serde(skip, default = "default_python_instance")]
    pub python_instance: Option<Arc<pyo3::PyObject>>,
}

/// Supported embedding providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingProvider {
    /// `OpenAI` embedding provider
    OpenAI,
    /// `Azure` embedding provider
    #[serde(rename = "azure")]
    Azure,
    /// `HuggingFace` embedding provider
    HuggingFace,
    #[cfg(feature = "python")]
    /// Python bridge provider
    PythonBridge,
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
            Self::Single(text) => vec![text.as_str()],
            Self::Multiple(texts) => texts.iter().map(String::as_str).collect(),
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
