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
    /// `OpenAI` LLM provider configuration
    OpenAI {
        /// API key for authentication
        api_key: String,
        /// Model name to use
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
        /// Optional organization ID
        organization: Option<String>,
    },
    /// `Anthropic` LLM provider configuration
    Anthropic {
        /// API key for authentication
        api_key: String,
        /// Model name to use
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
    },
    /// `Azure OpenAI` LLM provider configuration
    AzureOpenAI {
        /// API key for authentication
        api_key: String,
        /// Deployment name to use
        deployment_name: String,
        /// `Azure OpenAI` endpoint URL
        endpoint: String,
        /// API version to use
        api_version: String,
    },
    /// `DeepSeek` LLM provider configuration
    DeepSeek {
        /// API key for authentication
        api_key: String,
        /// Model name to use
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
    },
    /// `HuggingFace` LLM provider configuration
    HuggingFace {
        /// API key for authentication
        api_key: String,
        /// Model name to use
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
    },
    /// `Ollama` LLM provider configuration
    Ollama {
        /// Model name to use
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
    },
    /// `Perplexity` LLM provider configuration
    Perplexity {
        /// API key for authentication
        api_key: String,
        /// Model name to use
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
    },
    /// `OpenRouter` LLM provider configuration
    OpenRouter {
        /// API key for authentication
        api_key: String,
        /// Model name to use (e.g., "openai/gpt-4o", "anthropic/claude-3-5-sonnet")
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
        /// Optional site URL for `OpenRouter` rankings
        site_url: Option<String>,
        /// Optional site name for `OpenRouter` rankings
        site_name: Option<String>,
    },
    /// `Fireworks AI` LLM provider configuration
    Fireworks {
        /// API key for authentication
        api_key: String,
        /// Model name to use
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
    },
    /// `xAI` LLM provider configuration for Grok models
    Xai {
        /// API key for authentication
        api_key: String,
        /// Model name to use (e.g., "grok-4", "grok-3", "grok-code-fast-1")
        model: String,
        /// Optional custom base URL
        base_url: Option<String>,
    },
    /// Custom LLM provider configuration
    Custom {
        /// Provider type identifier
        provider_type: String,
        /// Custom configuration parameters
        config: HashMap<String, serde_json::Value>,
    },
    /// Unconfigured state - requires explicit configuration
    Unconfigured {
        /// Error message explaining the configuration requirement
        message: String,
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

    /// Create `Anthropic` configuration
    pub fn anthropic(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Anthropic {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create `Azure OpenAI` configuration
    pub fn azure_openai(
        api_key: impl Into<String>,
        deployment_name: impl Into<String>,
        endpoint: impl Into<String>,
        api_version: impl Into<String>,
    ) -> Self {
        Self::AzureOpenAI {
            api_key: api_key.into(),
            deployment_name: deployment_name.into(),
            endpoint: endpoint.into(),
            api_version: api_version.into(),
        }
    }

    /// Create `Azure OpenAI` configuration with default API version
    pub fn azure_openai_with_defaults(
        api_key: impl Into<String>,
        deployment_name: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self::AzureOpenAI {
            api_key: api_key.into(),
            deployment_name: deployment_name.into(),
            endpoint: endpoint.into(),
            api_version: "2024-10-21".to_string(),
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

    /// Create `Perplexity` configuration
    pub fn perplexity(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Perplexity {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create `OpenRouter` configuration
    pub fn openrouter(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::OpenRouter {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
            site_url: None,
            site_name: None,
        }
    }

    /// Create `OpenRouter` configuration with site information
    pub fn openrouter_with_site(
        api_key: impl Into<String>,
        model: impl Into<String>,
        site_url: Option<String>,
        site_name: Option<String>,
    ) -> Self {
        Self::OpenRouter {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
            site_url,
            site_name,
        }
    }

    /// Create `Fireworks AI` configuration
    pub fn fireworks(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Fireworks {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create `xAI` configuration for Grok models
    pub fn xai(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Xai {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Create `Ollama` configuration
    pub fn ollama(model: impl Into<String>) -> Self {
        Self::Ollama {
            model: model.into(),
            base_url: None,
        }
    }

    /// Create `Ollama` configuration with custom base URL
    pub fn ollama_with_base_url(model: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::Ollama {
            model: model.into(),
            base_url: Some(base_url.into()),
        }
    }

    /// Get the provider name
    pub fn provider_name(&self) -> &str {
        match self {
            LlmConfig::OpenAI { .. } => "openai",
            LlmConfig::Anthropic { .. } => "anthropic",
            LlmConfig::AzureOpenAI { .. } => "azure_openai",
            LlmConfig::DeepSeek { .. } => "deepseek",
            LlmConfig::HuggingFace { .. } => "huggingface",
            LlmConfig::Ollama { .. } => "ollama",
            LlmConfig::Perplexity { .. } => "perplexity",
            LlmConfig::OpenRouter { .. } => "openrouter",
            LlmConfig::Fireworks { .. } => "fireworks",
            LlmConfig::Xai { .. } => "xai",
            LlmConfig::Custom { provider_type, .. } => provider_type,
            LlmConfig::Unconfigured { .. } => "unconfigured",
        }
    }

    /// Get the model name
    pub fn model_name(&self) -> &str {
        match self {
            LlmConfig::OpenAI { model, .. } => model,
            LlmConfig::Anthropic { model, .. } => model,
            LlmConfig::AzureOpenAI {
                deployment_name, ..
            } => deployment_name,
            LlmConfig::DeepSeek { model, .. } => model,
            LlmConfig::HuggingFace { model, .. } => model,
            LlmConfig::Ollama { model, .. } => model,
            LlmConfig::Perplexity { model, .. } => model,
            LlmConfig::OpenRouter { model, .. } => model,
            LlmConfig::Fireworks { model, .. } => model,
            LlmConfig::Xai { model, .. } => model,
            LlmConfig::Custom { config, .. } => config
                .get("model")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown"),
            LlmConfig::Unconfigured { .. } => "none",
        }
    }
}

impl Default for LlmConfig {
    /// Default configuration requires explicit setup - no hardcoded provider defaults
    fn default() -> Self {
        Self::Unconfigured {
            message: "LLM provider not configured. Please explicitly set an LLM configuration using LlmConfig::openai(), LlmConfig::anthropic(), etc.".to_string(),
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
    pub fn new(provider: Box<dyn LlmProviderTrait>, config: LlmConfig) -> Self {
        Self {
            inner: provider,
            config,
        }
    }

    /// Get the provider configuration
    pub fn config(&self) -> &LlmConfig {
        &self.config
    }

    /// Get the underlying provider
    pub fn provider(&self) -> &dyn LlmProviderTrait {
        self.inner.as_ref()
    }

    /// Send a request to the LLM
    pub async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        tracing::info!(
            "LlmProvider wrapper: Forwarding request with {} tools to {} provider",
            request.tools.len(),
            self.config.provider_name()
        );
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
