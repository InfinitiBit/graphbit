//! Embedding configuration for GraphBit Node.js bindings

use crate::validation::{validate_api_key, validate_non_empty_string};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
#[derive(Clone)]
pub struct EmbeddingConfig {
    /// Provider name (e.g., "openai", "huggingface", "local")
    pub provider: String,
    /// API key for the provider
    pub api_key: Option<String>,
    /// Base URL for the API
    pub base_url: Option<String>,
    /// Model name/ID to use
    pub model: Option<String>,
    /// Additional configuration parameters
    pub config: HashMap<String, String>,
    /// Request timeout in milliseconds
    pub timeout_ms: Option<i64>,
    /// Maximum retry attempts
    pub max_retries: Option<i32>,
}

#[napi]
impl EmbeddingConfig {
    /// Create a new embedding configuration
    #[napi(constructor)]
    pub fn new(
        provider: String,
        api_key: Option<String>,
        base_url: Option<String>,
        model: Option<String>,
        timeout_ms: Option<i64>,
        max_retries: Option<i32>,
    ) -> Result<Self> {
        validate_non_empty_string(&provider, "provider")?;

        Ok(Self {
            provider,
            api_key,
            base_url,
            model,
            config: HashMap::new(),
            timeout_ms,
            max_retries,
        })
    }

    /// Create a configuration for OpenAI embeddings
    #[napi(factory)]
    pub fn openai(api_key: String, model: Option<String>) -> Result<Self> {
        validate_api_key(&api_key, "OpenAI")?;

        Ok(Self {
            provider: "openai".to_string(),
            api_key: Some(api_key),
            base_url: Some("https://api.openai.com/v1".to_string()),
            model: model.or_else(|| Some("text-embedding-ada-002".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(30000), // 30 seconds
            max_retries: Some(3),
        })
    }

    /// Create a configuration for Hugging Face embeddings
    #[napi(factory)]
    pub fn huggingface(api_key: String, model: Option<String>) -> Result<Self> {
        validate_api_key(&api_key, "HuggingFace")?;

        Ok(Self {
            provider: "huggingface".to_string(),
            api_key: Some(api_key),
            base_url: Some("https://api-inference.huggingface.co".to_string()),
            model: model.or_else(|| Some("sentence-transformers/all-MiniLM-L6-v2".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(30000), // 30 seconds
            max_retries: Some(3),
        })
    }

    /// Create a configuration for local embeddings
    #[napi(factory)]
    pub fn local(base_url: String, model: Option<String>) -> Result<Self> {
        validate_non_empty_string(&base_url, "base_url")?;

        Ok(Self {
            provider: "local".to_string(),
            api_key: None,
            base_url: Some(base_url),
            model: model.or_else(|| Some("default".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(60000), // 60 seconds for local models
            max_retries: Some(1),
        })
    }

    /// Set a configuration parameter
    #[napi]
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    /// Get a configuration parameter
    #[napi]
    pub fn get_config(&self, key: String) -> Option<String> {
        self.config.get(&key).cloned()
    }

    /// Validate the configuration
    #[napi]
    pub fn validate(&self) -> Result<()> {
        validate_non_empty_string(&self.provider, "provider")?;

        // Check provider-specific requirements
        match self.provider.as_str() {
            "openai" | "huggingface" => {
                if self.api_key.is_none() {
                    return Err(Error::new(
                        Status::InvalidArg,
                        format!("{} provider requires an API key", self.provider),
                    ));
                }
            }
            "local" => {
                if self.base_url.is_none() {
                    return Err(Error::new(
                        Status::InvalidArg,
                        "Local provider requires a base URL",
                    ));
                }
            }
            _ => {
                return Err(Error::new(
                    Status::InvalidArg,
                    format!("Unsupported provider: {}", self.provider),
                ));
            }
        }

        Ok(())
    }
}
