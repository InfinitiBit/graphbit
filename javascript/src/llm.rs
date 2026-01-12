//! LLM provider bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::llm::{LlmConfig as CoreLlmConfig, LlmProvider as CoreLlmProvider};
use std::collections::HashMap;

/// LLM configuration for JavaScript
#[napi]
#[derive(Clone)]
pub struct LlmConfig {
    pub(crate) inner: CoreLlmConfig,
}

#[napi]
impl LlmConfig {
    /// Create OpenAI configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.openai({
    ///   apiKey: process.env.OPENAI_API_KEY,
    ///   model: 'gpt-4o-mini'
    /// });
    /// ```
    #[napi(factory)]
    pub fn openai(options: OpenAiOptions) -> Result<Self> {
        let config = CoreLlmConfig::openai(
            options.api_key,
            options.model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
        );

        Ok(Self { inner: config })
    }

    /// Create Anthropic configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.anthropic({
    ///   apiKey: process.env.ANTHROPIC_API_KEY,
    ///   model: 'claude-3-5-sonnet-20241022'
    /// });
    /// ```
    #[napi(factory)]
    pub fn anthropic(options: AnthropicOptions) -> Result<Self> {
        let config = CoreLlmConfig::anthropic(
            options.api_key,
            options.model.unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string()),
        );

        Ok(Self { inner: config })
    }

    /// Create Ollama configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.ollama({
    ///   model: 'llama2',
    ///   baseUrl: 'http://localhost:11434'
    /// });
    /// ```
    #[napi(factory)]
    pub fn ollama(options: OllamaOptions) -> Result<Self> {
        let config = if let Some(base_url) = options.base_url {
            CoreLlmConfig::ollama_with_base_url(options.model, base_url)
        } else {
            CoreLlmConfig::ollama(options.model)
        };

        Ok(Self { inner: config })
    }

    /// Create Azure OpenAI configuration
    #[napi(factory)]
    pub fn azure_openai(options: AzureOpenAiOptions) -> Result<Self> {
        let config = CoreLlmConfig::azure_openai(
            options.api_key,
            options.deployment_name,
            options.endpoint,
            options.api_version.unwrap_or_else(|| "2024-10-21".to_string()),
        );

        Ok(Self { inner: config })
    }

    /// Create ByteDance ModelArk configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.bytedance({
    ///   apiKey: process.env.BYTEDANCE_API_KEY,
    ///   model: 'skylark-lite'
    /// });
    /// ```
    #[napi(factory)]
    pub fn bytedance(options: ByteDanceOptions) -> Result<Self> {
        let config = CoreLlmConfig::bytedance(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create DeepSeek configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.deepseek({
    ///   apiKey: process.env.DEEPSEEK_API_KEY,
    ///   model: 'deepseek-chat'
    /// });
    /// ```
    #[napi(factory)]
    pub fn deepseek(options: DeepSeekOptions) -> Result<Self> {
        let config = CoreLlmConfig::deepseek(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create HuggingFace configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.huggingface({
    ///   apiKey: process.env.HUGGINGFACE_API_KEY,
    ///   model: 'meta-llama/Llama-2-7b-chat-hf'
    /// });
    /// ```
    #[napi(factory)]
    pub fn huggingface(options: HuggingFaceOptions) -> Result<Self> {
        let config = CoreLlmConfig::huggingface(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create Perplexity configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.perplexity({
    ///   apiKey: process.env.PERPLEXITY_API_KEY,
    ///   model: 'llama-3.1-sonar-small-128k-online'
    /// });
    /// ```
    #[napi(factory)]
    pub fn perplexity(options: PerplexityOptions) -> Result<Self> {
        let config = CoreLlmConfig::perplexity(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create OpenRouter configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.openrouter({
    ///   apiKey: process.env.OPENROUTER_API_KEY,
    ///   model: 'openai/gpt-4o'
    /// });
    /// ```
    #[napi(factory)]
    pub fn openrouter(options: OpenRouterOptions) -> Result<Self> {
        let config = CoreLlmConfig::openrouter(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create Fireworks AI configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.fireworks({
    ///   apiKey: process.env.FIREWORKS_API_KEY,
    ///   model: 'accounts/fireworks/models/llama-v3p1-70b-instruct'
    /// });
    /// ```
    #[napi(factory)]
    pub fn fireworks(options: FireworksOptions) -> Result<Self> {
        let config = CoreLlmConfig::fireworks(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create Replicate configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.replicate({
    ///   apiKey: process.env.REPLICATE_API_KEY,
    ///   model: 'meta/llama-2-70b-chat'
    /// });
    /// ```
    #[napi(factory)]
    pub fn replicate(options: ReplicateOptions) -> Result<Self> {
        let config = CoreLlmConfig::replicate(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create TogetherAI configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.togetherai({
    ///   apiKey: process.env.TOGETHER_API_KEY,
    ///   model: 'meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo'
    /// });
    /// ```
    #[napi(factory)]
    pub fn togetherai(options: TogetherAiOptions) -> Result<Self> {
        let config = CoreLlmConfig::togetherai(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create xAI configuration for Grok models
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.xai({
    ///   apiKey: process.env.XAI_API_KEY,
    ///   model: 'grok-beta'
    /// });
    /// ```
    #[napi(factory)]
    pub fn xai(options: XaiOptions) -> Result<Self> {
        let config = CoreLlmConfig::xai(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create AI21 configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.ai21({
    ///   apiKey: process.env.AI21_API_KEY,
    ///   model: 'jamba-1.5-large'
    /// });
    /// ```
    #[napi(factory)]
    pub fn ai21(options: Ai21Options) -> Result<Self> {
        let config = CoreLlmConfig::ai21(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Create MistralAI configuration
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.mistralai({
    ///   apiKey: process.env.MISTRAL_API_KEY,
    ///   model: 'mistral-large-latest'
    /// });
    /// ```
    #[napi(factory)]
    pub fn mistralai(options: MistralAiOptions) -> Result<Self> {
        let config = CoreLlmConfig::mistralai(
            options.api_key,
            options.model,
        );

        Ok(Self { inner: config })
    }

    /// Get the inner core config (internal use)
    pub(crate) fn inner(&self) -> &CoreLlmConfig {
        &self.inner
    }

    /// Clone the inner config
    pub(crate) fn clone_inner(&self) -> CoreLlmConfig {
        self.inner.clone()
    }
}

/// OpenAI configuration options
#[napi(object)]
pub struct OpenAiOptions {
    pub api_key: String,
    pub model: Option<String>,
}

/// Anthropic configuration options
#[napi(object)]
pub struct AnthropicOptions {
    pub api_key: String,
    pub model: Option<String>,
}

/// Ollama configuration options
#[napi(object)]
pub struct OllamaOptions {
    pub model: String,
    pub base_url: Option<String>,
}

/// Azure OpenAI configuration options
#[napi(object)]
pub struct AzureOpenAiOptions {
    pub api_key: String,
    pub deployment_name: String,
    pub endpoint: String,
    pub api_version: Option<String>,
}

/// ByteDance ModelArk configuration options
#[napi(object)]
pub struct ByteDanceOptions {
    pub api_key: String,
    pub model: String,
}

/// DeepSeek configuration options
#[napi(object)]
pub struct DeepSeekOptions {
    pub api_key: String,
    pub model: String,
}

/// HuggingFace configuration options
#[napi(object)]
pub struct HuggingFaceOptions {
    pub api_key: String,
    pub model: String,
}

/// Perplexity configuration options
#[napi(object)]
pub struct PerplexityOptions {
    pub api_key: String,
    pub model: String,
}

/// OpenRouter configuration options
#[napi(object)]
pub struct OpenRouterOptions {
    pub api_key: String,
    pub model: String,
}

/// Fireworks AI configuration options
#[napi(object)]
pub struct FireworksOptions {
    pub api_key: String,
    pub model: String,
}

/// Replicate configuration options
#[napi(object)]
pub struct ReplicateOptions {
    pub api_key: String,
    pub model: String,
}

/// TogetherAI configuration options
#[napi(object)]
pub struct TogetherAiOptions {
    pub api_key: String,
    pub model: String,
}

/// xAI configuration options
#[napi(object)]
pub struct XaiOptions {
    pub api_key: String,
    pub model: String,
}

/// AI21 configuration options
#[napi(object)]
pub struct Ai21Options {
    pub api_key: String,
    pub model: String,
}

/// MistralAI configuration options
#[napi(object)]
pub struct MistralAiOptions {
    pub api_key: String,
    pub model: String,
}

/// LLM response finish reason
#[napi]
pub enum FinishReason {
    /// Response completed normally
    Stop,
    /// Hit token limit
    Length,
    /// Content filtered
    ContentFilter,
    /// Tool/function call
    ToolCalls,
    /// Unknown reason
    Unknown,
}

/// LLM usage statistics
#[napi(object)]
pub struct LlmUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// LLM tool call
#[napi(object)]
pub struct LlmToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

/// LLM response
#[napi(object)]
pub struct LlmResponse {
    pub content: String,
    pub finish_reason: Option<FinishReason>,
    pub usage: Option<LlmUsage>,
    pub tool_calls: Option<Vec<LlmToolCall>>,
    pub model: String,
}

impl From<graphbit_core::llm::LlmResponse> for LlmResponse {
    fn from(response: graphbit_core::llm::LlmResponse) -> Self {
        use graphbit_core::llm::FinishReason as CoreFinishReason;

        let finish_reason = match response.finish_reason {
            CoreFinishReason::Stop => Some(FinishReason::Stop),
            CoreFinishReason::Length => Some(FinishReason::Length),
            CoreFinishReason::ToolCalls => Some(FinishReason::ToolCalls),
            CoreFinishReason::ContentFilter => Some(FinishReason::ContentFilter),
            CoreFinishReason::Error => Some(FinishReason::Unknown),
            CoreFinishReason::Other(_) => Some(FinishReason::Unknown),
        };

        let usage = Some(LlmUsage {
            prompt_tokens: response.usage.prompt_tokens,
            completion_tokens: response.usage.completion_tokens,
            total_tokens: response.usage.total_tokens,
        });

        let tool_calls = if response.tool_calls.is_empty() {
            None
        } else {
            Some(
                response
                    .tool_calls
                    .into_iter()
                    .map(|call| LlmToolCall {
                        id: call.id,
                        name: call.name,
                        arguments: call.parameters.to_string(),
                    })
                    .collect(),
            )
        };

        Self {
            content: response.content,
            finish_reason,
            usage,
            tool_calls,
            model: response.model,
        }
    }
}

