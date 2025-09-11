//! LLM provider abstraction and configuration

use crate::errors::GraphBitResult;
use crate::llm::{LlmRequest, LlmResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for different LLM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub enum LlmConfig {
    /// `OpenAI` language model configuration
    OpenAI {
        /// API key for `OpenAI` authentication
        api_key: String,
        /// Model name (e.g., "gpt-4", "gpt-3.5-turbo")
        model: String,
        /// Optional custom base URL for API requests
        base_url: Option<String>,
        /// Optional organization ID for `OpenAI`
        organization: Option<String>,
    },
    /// Anthropic Claude language model configuration
    Anthropic {
        /// API key for Anthropic authentication
        api_key: String,
        /// Model name (e.g., "claude-3-sonnet", "claude-3-opus")
        model: String,
        /// Optional custom base URL for API requests
        base_url: Option<String>,
    },
    /// `DeepSeek` language model configuration
    DeepSeek {
        /// API key for `DeepSeek` authentication
        api_key: String,
        /// Model name (e.g., "deepseek-chat", "deepseek-coder")
        model: String,
        /// Optional custom base URL for API requests
        base_url: Option<String>,
    },
    /// `HuggingFace` language model configuration
    HuggingFace {
        /// API key for `HuggingFace` authentication
        api_key: String,
        /// Model name or repository path
        model: String,
        /// Optional custom base URL for API requests
        base_url: Option<String>,
    },
    /// Ollama local language model configuration
    Ollama {
        /// Model name available in Ollama
        model: String,
        /// Optional custom base URL for Ollama server
        base_url: Option<String>,
    },
    /// Perplexity language model configuration
    Perplexity {
        /// API key for Perplexity authentication
        api_key: String,
        /// Model name (e.g., "pplx-7b-online", "pplx-70b-online")
        model: String,
        /// Optional custom base URL for API requests
        base_url: Option<String>,
    },
    /// Custom language model provider configuration
    Custom {
        /// Type identifier for the custom provider
        provider_type: String,
        /// Arbitrary configuration parameters for the custom provider
        config: HashMap<String, serde_json::Value>,
    },
}

impl LlmConfig {
    /// Create `OpenAI` configuration
    pub fn openai(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::OpenAI {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
            organization: None,
        }
    }

    /// Create Anthropic configuration
    pub fn anthropic(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Anthropic {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create `DeepSeek` configuration
    pub fn deepseek(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::DeepSeek {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create `HuggingFace` configuration
    pub fn huggingface(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::HuggingFace {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create Perplexity configuration
    pub fn perplexity(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Perplexity {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create Ollama configuration
    pub fn ollama(model: impl Into<String>) -> Self {
        Self::Ollama {
            model: model.into(),
            base_url: None,
        }
    }

    /// Create Ollama configuration with custom base URL
    pub fn ollama_with_base_url(model: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::Ollama {
            model: model.into(),
            base_url: Some(base_url.into()),
        }
    }

    /// Get the provider name
    #[must_use]
    pub fn provider_name(&self) -> &str {
        match self {
            LlmConfig::OpenAI { .. } => "openai",
            LlmConfig::Anthropic { .. } => "anthropic",
            LlmConfig::DeepSeek { .. } => "deepseek",
            LlmConfig::HuggingFace { .. } => "huggingface",
            LlmConfig::Ollama { .. } => "ollama",
            LlmConfig::Perplexity { .. } => "perplexity",
            LlmConfig::Custom { provider_type, .. } => provider_type,
        }
    }

    /// Get the model name
    #[must_use]
    pub fn model_name(&self) -> &str {
        match self {
            LlmConfig::OpenAI { model, .. } => model,
            LlmConfig::Anthropic { model, .. } => model,
            LlmConfig::DeepSeek { model, .. } => model,
            LlmConfig::HuggingFace { model, .. } => model,
            LlmConfig::Ollama { model, .. } => model,
            LlmConfig::Perplexity { model, .. } => model,
            LlmConfig::Custom { config, .. } => config
                .get("model")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown"),
        }
    }
}

impl Default for LlmConfig {
    /// Default configuration uses Ollama with llama3.2 model for local development
    fn default() -> Self {
        Self::Ollama {
            model: "llama3.2".to_string(),
            base_url: None,
        }
    }
}

/// Trait that all LLM providers must implement
#[async_trait]
pub trait LlmProviderTrait: Send + Sync {
    /// Get the provider name
    fn provider_name(&self) -> &str;

    /// Get the model name
    fn model_name(&self) -> &str;

    /// Send a request to the LLM and get a response
    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse>;

    /// Stream a response from the LLM (optional implementation)
    async fn stream(
        &self,
        _request: LlmRequest,
    ) -> GraphBitResult<Box<dyn futures::Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>>
    {
        Err(crate::errors::GraphBitError::config(
            "Streaming not supported by this provider",
        ))
    }

    /// Check if the provider supports streaming
    fn supports_streaming(&self) -> bool {
        false
    }

    /// Check if the provider supports function calling
    fn supports_function_calling(&self) -> bool {
        false
    }

    /// Get the maximum context length for this provider/model
    fn max_context_length(&self) -> Option<u32> {
        None
    }

    /// Get the cost per token for this provider/model
    fn cost_per_token(&self) -> Option<(f64, f64)> {
        None // (input_cost, output_cost)
    }
}

/// Wrapper for LLM providers
pub struct LlmProvider {
    inner: Box<dyn LlmProviderTrait>,
    config: LlmConfig,
}

impl LlmProvider {
    /// Create a new LLM provider wrapper
    #[must_use]
    pub fn new(provider: Box<dyn LlmProviderTrait>, config: LlmConfig) -> Self {
        Self {
            inner: provider,
            config,
        }
    }

    /// Get the provider configuration
    #[must_use]
    pub fn config(&self) -> &LlmConfig {
        &self.config
    }

    /// Get the underlying provider
    #[must_use]
    pub fn provider(&self) -> &dyn LlmProviderTrait {
        self.inner.as_ref()
    }

    /// Send a request to the LLM
    pub async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        self.inner.complete(request).await
    }

    /// Stream a response from the LLM
    pub async fn stream(
        &self,
        request: LlmRequest,
    ) -> GraphBitResult<Box<dyn futures::Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>>
    {
        self.inner.stream(request).await
    }
}
