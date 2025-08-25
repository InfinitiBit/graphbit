//! LLM client for GraphBit Node.js bindings

use super::config::LlmConfig;
use crate::errors::to_napi_error;
use crate::validation::validate_non_empty_string;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi(object)]
pub struct LlmResponse {
    /// Generated text content
    pub content: String,
    /// Token usage information
    pub token_usage: Option<TokenUsage>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Whether the response was successful
    pub success: bool,
    /// Error message if unsuccessful
    pub error: Option<String>,
}

#[napi(object)]
pub struct TokenUsage {
    /// Number of prompt tokens
    pub prompt_tokens: i32,
    /// Number of completion tokens
    pub completion_tokens: i32,
    /// Total number of tokens
    pub total_tokens: i32,
}

#[napi]
pub struct LlmClient {
    config: LlmConfig,
}

#[napi]
impl LlmClient {
    /// Create a new LLM client
    #[napi(constructor)]
    pub fn new(config: &LlmConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Generate text from a prompt
    #[napi]
    pub async fn generate(
        &self,
        prompt: String,
        options: Option<GenerationOptions>,
    ) -> Result<LlmResponse> {
        validate_non_empty_string(&prompt, "prompt")?;

        let options = options.unwrap_or_default();

        // Convert Node.js config to core config
        let core_config = self.convert_to_core_config()?;

        // Create the provider
        let provider = graphbit_core::llm::LlmProviderFactory::create_provider(core_config)
            .map_err(to_napi_error)?;

        // Build the LLM request
        let mut llm_request = graphbit_core::llm::LlmRequest::new(prompt);

        // Apply options
        if let Some(max_tokens) = options.max_tokens {
            if max_tokens > 0 {
                llm_request = llm_request.with_max_tokens(max_tokens as u32);
            }
        }

        if let Some(temperature) = options.temperature {
            if temperature >= 0.0 && temperature <= 2.0 {
                llm_request = llm_request.with_temperature(temperature as f32);
            }
        }

        if let Some(top_p) = options.top_p {
            if top_p >= 0.0 && top_p <= 1.0 {
                llm_request = llm_request.with_top_p(top_p as f32);
            }
        }

        // Execute the request
        let core_response = provider
            .complete(llm_request)
            .await
            .map_err(to_napi_error)?;

        // Convert core response to Node.js response
        let mut metadata = HashMap::new();
        for (key, value) in core_response.metadata {
            metadata.insert(key, value.to_string());
        }

        Ok(LlmResponse {
            content: core_response.content,
            token_usage: Some(TokenUsage {
                prompt_tokens: core_response.usage.prompt_tokens as i32,
                completion_tokens: core_response.usage.completion_tokens as i32,
                total_tokens: core_response.usage.total_tokens as i32,
            }),
            metadata,
            success: true,
            error: None,
        })
    }

    /// Generate text with streaming (future enhancement)
    #[napi]
    pub async fn generate_stream(
        &self,
        prompt: String,
        options: Option<GenerationOptions>,
    ) -> Result<LlmResponse> {
        validate_non_empty_string(&prompt, "prompt")?;

        let options = options.unwrap_or_default();

        // Convert Node.js config to core config
        let core_config = self.convert_to_core_config()?;

        // Create the provider
        let provider = graphbit_core::llm::LlmProviderFactory::create_provider(core_config)
            .map_err(to_napi_error)?;

        // Check if streaming is supported
        if !provider.supports_streaming() {
            // Fall back to regular generation
            return self.generate(prompt, Some(options)).await;
        }

        // Build the LLM request
        let mut llm_request = graphbit_core::llm::LlmRequest::new(prompt);

        // Apply options
        if let Some(max_tokens) = options.max_tokens {
            if max_tokens > 0 {
                llm_request = llm_request.with_max_tokens(max_tokens as u32);
            }
        }

        if let Some(temperature) = options.temperature {
            if temperature >= 0.0 && temperature <= 2.0 {
                llm_request = llm_request.with_temperature(temperature as f32);
            }
        }

        if let Some(top_p) = options.top_p {
            if top_p >= 0.0 && top_p <= 1.0 {
                llm_request = llm_request.with_top_p(top_p as f32);
            }
        }

        // For now, use regular completion since streaming in Node.js bindings is complex
        // TODO: Implement actual streaming with JavaScript callbacks or async iterators
        let core_response = provider
            .complete(llm_request)
            .await
            .map_err(to_napi_error)?;

        // Convert core response to Node.js response
        let mut metadata = HashMap::new();
        for (key, value) in core_response.metadata {
            metadata.insert(key, value.to_string());
        }

        Ok(LlmResponse {
            content: core_response.content,
            token_usage: Some(TokenUsage {
                prompt_tokens: core_response.usage.prompt_tokens as i32,
                completion_tokens: core_response.usage.completion_tokens as i32,
                total_tokens: core_response.usage.total_tokens as i32,
            }),
            metadata,
            success: true,
            error: None,
        })
    }

    /// Get the current configuration
    #[napi(getter)]
    pub fn config(&self) -> LlmConfig {
        self.config.clone()
    }

    /// Update the configuration
    #[napi]
    pub fn update_config(&mut self, config: &LlmConfig) -> Result<()> {
        config.validate()?;
        self.config = config.clone();
        Ok(())
    }

    /// Convert Node.js config to core config
    fn convert_to_core_config(&self) -> Result<graphbit_core::llm::LlmConfig> {
        match self.config.provider.as_str() {
            "openai" => {
                let api_key =
                    self.config.api_key.as_ref().ok_or_else(|| {
                        Error::new(Status::InvalidArg, "OpenAI API key is required")
                    })?;
                let model = self.config.model.as_deref().unwrap_or("gpt-3.5-turbo");

                Ok(graphbit_core::llm::LlmConfig::OpenAI {
                    api_key: api_key.clone(),
                    model: model.to_string(),
                    base_url: self.config.base_url.clone(),
                    organization: None,
                })
            }
            "anthropic" => {
                let api_key = self.config.api_key.as_ref().ok_or_else(|| {
                    Error::new(Status::InvalidArg, "Anthropic API key is required")
                })?;
                let model = self
                    .config
                    .model
                    .as_deref()
                    .unwrap_or("claude-3-sonnet-20240229");

                Ok(graphbit_core::llm::LlmConfig::Anthropic {
                    api_key: api_key.clone(),
                    model: model.to_string(),
                    base_url: self.config.base_url.clone(),
                })
            }
            "deepseek" => {
                let api_key = self.config.api_key.as_ref().ok_or_else(|| {
                    Error::new(Status::InvalidArg, "DeepSeek API key is required")
                })?;
                let model = self.config.model.as_deref().unwrap_or("deepseek-chat");

                Ok(graphbit_core::llm::LlmConfig::DeepSeek {
                    api_key: api_key.clone(),
                    model: model.to_string(),
                    base_url: self.config.base_url.clone(),
                })
            }
            "huggingface" => {
                let api_key = self.config.api_key.as_ref().ok_or_else(|| {
                    Error::new(Status::InvalidArg, "HuggingFace API key is required")
                })?;
                let model = self.config.model.as_deref().unwrap_or("gpt2");

                Ok(graphbit_core::llm::LlmConfig::HuggingFace {
                    api_key: api_key.clone(),
                    model: model.to_string(),
                    base_url: self.config.base_url.clone(),
                })
            }
            "ollama" => {
                let model = self.config.model.as_deref().unwrap_or("llama3.2");

                Ok(graphbit_core::llm::LlmConfig::Ollama {
                    model: model.to_string(),
                    base_url: self.config.base_url.clone(),
                })
            }
            "perplexity" => {
                let api_key = self.config.api_key.as_ref().ok_or_else(|| {
                    Error::new(Status::InvalidArg, "Perplexity API key is required")
                })?;
                let model = self.config.model.as_deref().unwrap_or("sonar");

                Ok(graphbit_core::llm::LlmConfig::Perplexity {
                    api_key: api_key.clone(),
                    model: model.to_string(),
                    base_url: self.config.base_url.clone(),
                })
            }
            provider => Err(Error::new(
                Status::InvalidArg,
                format!("Unsupported provider: {}", provider),
            )),
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct GenerationOptions {
    /// Temperature for generation (0.0 to 2.0)
    pub temperature: Option<f64>,
    /// Maximum number of tokens to generate
    pub max_tokens: Option<i32>,
    /// Top-p for nucleus sampling
    pub top_p: Option<f64>,
    /// Stop sequences
    pub stop: Option<Vec<String>>,
    /// Whether to stream the response
    pub stream: Option<bool>,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            temperature: Some(0.7),
            max_tokens: Some(1000),
            top_p: Some(1.0),
            stop: None,
            stream: Some(false),
        }
    }
}
