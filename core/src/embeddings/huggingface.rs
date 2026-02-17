//! HuggingFace embedding provider.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::errors::{GraphBitError, GraphBitResult};

use super::types::{
    EmbeddingConfig, EmbeddingProvider, EmbeddingProviderTrait, EmbeddingRequest,
    EmbeddingResponse, EmbeddingUsage,
};

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

    fn base_url(&self) -> String {
        self.config
            .base_url
            .as_deref()
            .map(str::to_string)
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

        let embeddings: Vec<Vec<f32>> = if response_json.is_array() {
            response_json
                .as_array()
                .unwrap()
                .iter()
                .map(|item| {
                    if item.is_array() {
                        item.as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                            .collect()
                    } else {
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

        let total_chars: usize = inputs.iter().map(|s| s.len()).sum();
        let estimated_tokens = (total_chars / 4) as u32;

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
        let test_request = EmbeddingRequest {
            input: super::types::EmbeddingInput::Single("test".to_string()),
            user: None,
            params: HashMap::new(),
        };
        let response = self.generate_embeddings(test_request).await?;
        Ok(response
            .embeddings
            .first()
            .map(Vec::<f32>::len)
            .unwrap_or(768))
    }

    fn max_batch_size(&self) -> usize {
        100
    }
}
