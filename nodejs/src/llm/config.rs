//! LLM configuration for GraphBit Node.js bindings

use crate::validation::{validate_non_empty_string, validate_api_key};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
#[derive(Clone)]
pub struct LlmConfig {
    /// Provider name (e.g., "openai", "anthropic", "local")
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
impl LlmConfig {
    /// Create a new LLM configuration
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

    /// Create a configuration for OpenAI
    #[napi(factory)]
    pub fn openai(api_key: String, model: Option<String>) -> Result<Self> {
        validate_api_key(&api_key, "OpenAI")?;

        Ok(Self {
            provider: "openai".to_string(),
            api_key: Some(api_key),
            base_url: Some("https://api.openai.com/v1".to_string()),
            model: model.or_else(|| Some("gpt-3.5-turbo".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(30000), // 30 seconds
            max_retries: Some(3),
        })
    }

    /// Create a configuration for Anthropic
    #[napi(factory)]
    pub fn anthropic(api_key: String, model: Option<String>) -> Result<Self> {
        validate_api_key(&api_key, "Anthropic")?;

        Ok(Self {
            provider: "anthropic".to_string(),
            api_key: Some(api_key),
            base_url: Some("https://api.anthropic.com".to_string()),
            model: model.or_else(|| Some("claude-3-sonnet-20240229".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(30000), // 30 seconds
            max_retries: Some(3),
        })
    }

    /// Create a configuration for DeepSeek
    #[napi(factory)]
    pub fn deepseek(api_key: String, model: Option<String>) -> Result<Self> {
        validate_api_key(&api_key, "DeepSeek")?;

        Ok(Self {
            provider: "deepseek".to_string(),
            api_key: Some(api_key),
            base_url: Some("https://api.deepseek.com".to_string()),
            model: model.or_else(|| Some("deepseek-chat".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(30000), // 30 seconds
            max_retries: Some(3),
        })
    }

    /// Create a configuration for HuggingFace
    #[napi(factory)]
    pub fn huggingface(api_key: String, model: Option<String>, base_url: Option<String>) -> Result<Self> {
        validate_api_key(&api_key, "HuggingFace")?;

        Ok(Self {
            provider: "huggingface".to_string(),
            api_key: Some(api_key),
            base_url: base_url.or_else(|| Some("https://api-inference.huggingface.co".to_string())),
            model: model.or_else(|| Some("gpt2".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(30000), // 30 seconds
            max_retries: Some(3),
        })
    }

    /// Create a configuration for Ollama
    #[napi(factory)]
    pub fn ollama(model: Option<String>, base_url: Option<String>) -> Result<Self> {
        Ok(Self {
            provider: "ollama".to_string(),
            api_key: None, // Ollama doesn't require API key
            base_url: base_url.or_else(|| Some("http://localhost:11434".to_string())),
            model: model.or_else(|| Some("llama3.2".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(60000), // 60 seconds for local models
            max_retries: Some(1),
        })
    }

    /// Create a configuration for Perplexity
    #[napi(factory)]
    pub fn perplexity(api_key: String, model: Option<String>) -> Result<Self> {
        validate_api_key(&api_key, "Perplexity")?;

        Ok(Self {
            provider: "perplexity".to_string(),
            api_key: Some(api_key),
            base_url: Some("https://api.perplexity.ai".to_string()),
            model: model.or_else(|| Some("sonar".to_string())),
            config: HashMap::new(),
            timeout_ms: Some(30000), // 30 seconds
            max_retries: Some(3),
        })
    }

    /// Create a configuration for local LLM
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

    /// Set temperature for generation
    #[napi]
    pub fn set_temperature(&mut self, temperature: f64) {
        self.config.insert("temperature".to_string(), temperature.to_string());
    }

    /// Set max tokens for generation
    #[napi]
    pub fn set_max_tokens(&mut self, max_tokens: i32) {
        self.config.insert("max_tokens".to_string(), max_tokens.to_string());
    }

    /// Set top_p for generation
    #[napi]
    pub fn set_top_p(&mut self, top_p: f64) {
        self.config.insert("top_p".to_string(), top_p.to_string());
    }

    /// Validate the configuration
    #[napi]
    pub fn validate(&self) -> Result<()> {
        validate_non_empty_string(&self.provider, "provider")?;

        // Check provider-specific requirements
        match self.provider.as_str() {
            "openai" | "anthropic" | "deepseek" | "huggingface" | "perplexity" => {
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
            "ollama" => {
                // Ollama doesn't require API key, just validate base_url exists
                if self.base_url.is_none() {
                    return Err(Error::new(
                        Status::InvalidArg,
                        "Ollama provider requires a base URL",
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