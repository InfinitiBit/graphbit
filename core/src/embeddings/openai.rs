//! OpenAI embedding provider.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::errors::{GraphBitError, GraphBitResult};

use super::types::{
    EmbeddingConfig, EmbeddingInput, EmbeddingProvider, EmbeddingProviderTrait, EmbeddingRequest,
    EmbeddingResponse, EmbeddingUsage,
};

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

        if let Some(user) = &request.user {
            body["user"] = serde_json::Value::String(user.clone());
        }

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
        match self.config.model.as_str() {
            "text-embedding-ada-002" => Ok(1536),
            "text-embedding-3-small" => Ok(1536),
            "text-embedding-3-large" => Ok(3072),
            _ => {
                let test_request = EmbeddingRequest {
                    input: EmbeddingInput::Single("test".to_string()),
                    user: None,
                    params: HashMap::new(),
                };
                let response = self.generate_embeddings(test_request).await?;
                Ok(response
                    .embeddings
                    .first()
                    .map(Vec::<f32>::len)
                    .unwrap_or(1536))
            }
        }
    }

    fn max_batch_size(&self) -> usize {
        2048
    }
}
