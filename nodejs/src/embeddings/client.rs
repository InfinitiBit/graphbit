//! Embedding client for GraphBit Node.js bindings

use super::config::EmbeddingConfig;
use crate::errors::to_napi_error;
use crate::validation::validate_non_empty_string;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi(object)]
pub struct EmbeddingResponse {
    /// Generated embeddings
    pub embeddings: Vec<Vec<f64>>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Whether the response was successful
    pub success: bool,
    /// Error message if unsuccessful
    pub error: Option<String>,
    /// Token usage information
    pub token_usage: Option<i32>,
}

#[napi]
pub struct EmbeddingClient {
    config: EmbeddingConfig,
}

#[napi]
impl EmbeddingClient {
    /// Create a new embedding client
    #[napi(constructor)]
    pub fn new(config: &EmbeddingConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Generate embeddings for text
    #[napi]
    pub async fn embed(&self, text: String) -> Result<EmbeddingResponse> {
        validate_non_empty_string(&text, "text")?;
        self.embed_batch(vec![text]).await
    }

    /// Generate embeddings for multiple texts
    #[napi]
    pub async fn embed_batch(&self, texts: Vec<String>) -> Result<EmbeddingResponse> {
        if texts.is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "Input texts cannot be empty",
            ));
        }

        for (i, text) in texts.iter().enumerate() {
            if text.trim().is_empty() {
                return Err(Error::new(
                    Status::InvalidArg,
                    format!("Text at index {} cannot be empty", i),
                ));
            }
        }

        // Convert Node.js config to core config
        let core_config = self.convert_to_core_config()?;

        // Create the embedding service
        let embedding_service =
            graphbit_core::embeddings::EmbeddingService::new(core_config).map_err(to_napi_error)?;

        // Generate embeddings using the core service
        let core_embeddings = embedding_service
            .embed_texts(&texts)
            .await
            .map_err(to_napi_error)?;

        // Convert f32 to f64 for Node.js compatibility
        let embeddings: Vec<Vec<f64>> = core_embeddings
            .into_iter()
            .map(|embedding| embedding.into_iter().map(|f| f as f64).collect())
            .collect();

        // Estimate token usage (since not all providers give exact counts)
        let total_chars: usize = texts.iter().map(|s| s.len()).sum();
        let estimated_tokens = (total_chars / 4) as i32; // Rough estimate: 4 chars per token

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), self.config.provider.clone());
        if let Some(model) = &self.config.model {
            metadata.insert("model".to_string(), model.clone());
        }

        Ok(EmbeddingResponse {
            embeddings,
            metadata,
            success: true,
            error: None,
            token_usage: Some(estimated_tokens),
        })
    }

    /// Get embedding dimensions for the current model
    #[napi]
    pub async fn get_dimensions(&self) -> Result<i32> {
        // Convert Node.js config to core config
        let core_config = self.convert_to_core_config()?;

        // Create the embedding service
        let embedding_service =
            graphbit_core::embeddings::EmbeddingService::new(core_config).map_err(to_napi_error)?;

        // Get dimensions from the service
        let dimensions = embedding_service
            .get_dimensions()
            .await
            .map_err(to_napi_error)?;

        Ok(dimensions as i32)
    }

    /// Get the current configuration
    #[napi(getter)]
    pub fn config(&self) -> EmbeddingConfig {
        self.config.clone()
    }

    /// Update the configuration
    #[napi]
    pub fn update_config(&mut self, config: &EmbeddingConfig) -> Result<()> {
        config.validate()?;
        self.config = config.clone();
        Ok(())
    }

    /// Convert Node.js config to core config
    fn convert_to_core_config(&self) -> Result<graphbit_core::embeddings::EmbeddingConfig> {
        let provider = match self.config.provider.as_str() {
            "openai" => graphbit_core::embeddings::EmbeddingProvider::OpenAI,
            "huggingface" => graphbit_core::embeddings::EmbeddingProvider::HuggingFace,
            provider => {
                return Err(Error::new(
                    Status::InvalidArg,
                    format!("Unsupported embedding provider: {}", provider),
                ))
            }
        };

        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| Error::new(Status::InvalidArg, "API key is required"))?;

        let model = self
            .config
            .model
            .as_deref()
            .unwrap_or("text-embedding-ada-002");

        Ok(graphbit_core::embeddings::EmbeddingConfig {
            provider,
            api_key: api_key.clone(),
            model: model.to_string(),
            base_url: self.config.base_url.clone(),
            timeout_seconds: self.config.timeout_ms.map(|ms| ms as u64 / 1000),
            max_batch_size: Some(100),
            extra_params: HashMap::new(),
        })
    }
}
