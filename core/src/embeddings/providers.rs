//! Embedding provider configuration and factory

use crate::errors::GraphBitResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::huggingface::HuggingFaceEmbeddingProvider;
use super::openai::OpenAIEmbeddingProvider;
use super::EmbeddingProviderTrait;

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingProvider {
    /// `OpenAI` embedding provider
    OpenAI,
    /// `HuggingFace` embedding provider
    HuggingFace,
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

