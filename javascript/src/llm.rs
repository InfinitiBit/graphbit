//! LLM provider bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::llm::{LlmConfig as CoreLlmConfig, LlmProvider as CoreLlmProvider};
use std::collections::HashMap;

/// LLM configuration for JavaScript
#[napi]
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

