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
}

