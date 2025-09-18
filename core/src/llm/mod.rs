//! LLM provider abstraction for `GraphBit`
//!
//! This module provides a unified interface for working with different
//! LLM providers while maintaining strong type safety and validation.

pub mod anthropic;
pub mod deepseek;
pub mod fireworks;
pub mod huggingface;
pub mod ollama;
pub mod openai;
pub mod openrouter;
pub mod perplexity;
pub mod providers;
pub mod response;

pub use providers::{LlmConfig, LlmProvider, LlmProviderTrait};
pub use response::{FinishReason, LlmResponse, LlmUsage};

use crate::errors::{GraphBitError, GraphBitResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LLM request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    /// The prompt or messages to send to the LLM
    pub messages: Vec<LlmMessage>,
    /// Maximum number of tokens to generate
    pub max_tokens: Option<u32>,
    /// Temperature for response randomness (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Top-p sampling parameter
    pub top_p: Option<f32>,
    /// List of tools available to the LLM
    pub tools: Vec<LlmTool>,
    /// Additional provider-specific parameters
    pub extra_params: HashMap<String, serde_json::Value>,
}

impl LlmRequest {
    /// Create a new LLM request with a simple text prompt
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            messages: vec![LlmMessage::user(prompt)],
            max_tokens: None,
            temperature: None,
            top_p: None,
            tools: Vec::with_capacity(4), // Pre-allocate small capacity
            extra_params: HashMap::with_capacity(4), // Pre-allocate small capacity
        }
    }

    /// Create a new LLM request with multiple messages
    pub fn with_messages(messages: Vec<LlmMessage>) -> Self {
        Self {
            messages,
            max_tokens: None,
            temperature: None,
            top_p: None,
            tools: Vec::with_capacity(4),
            extra_params: HashMap::with_capacity(4),
        }
    }

    /// Add a message to the request
    #[inline]
    pub fn with_message(mut self, message: LlmMessage) -> Self {
        self.messages.push(message);
        self
    }

    /// Set maximum tokens
    #[inline]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set temperature
    #[inline]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }

    /// Set top-p
    #[inline]
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p.clamp(0.0, 1.0));
        self
    }

    /// Add a tool
    #[inline]
    pub fn with_tool(mut self, tool: LlmTool) -> Self {
        self.tools.push(tool);
        self
    }

    /// Add multiple tools
    pub fn with_tools(mut self, tools: Vec<LlmTool>) -> Self {
        self.tools.extend(tools);
        self
    }

    /// Add extra parameters
    #[inline]
    pub fn with_extra_param(mut self, key: String, value: serde_json::Value) -> Self {
        self.extra_params.insert(key, value);
        self
    }

    /// Get total message length estimate for performance planning
    pub fn estimated_token_count(&self) -> usize {
        self.messages
            .iter()
            .map(|msg| msg.content.len() / 4) // Rough estimate: 4 chars per token
            .sum()
    }
}

/// Individual message in an LLM conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    /// Role of the message sender
    pub role: LlmRole,
    /// Content of the message
    pub content: String,
    /// Optional tool calls in this message
    pub tool_calls: Vec<LlmToolCall>,
}

impl LlmMessage {
    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: LlmRole::User,
            content: content.into(),
            tool_calls: Vec::new(),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: LlmRole::Assistant,
            content: content.into(),
            tool_calls: Vec::new(),
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: LlmRole::System,
            content: content.into(),
            tool_calls: Vec::new(),
        }
    }

    /// Create a tool message (for tool call results)
    pub fn tool(tool_call_id: impl Into<String>, result: impl Into<String>) -> Self {
        Self {
            role: LlmRole::Tool,
            content: format!(
                "Tool call {} result: {}",
                tool_call_id.into(),
                result.into()
            ),
            tool_calls: Vec::new(),
        }
    }

    /// Add tool calls to the message
    #[inline]
    pub fn with_tool_calls(mut self, tool_calls: Vec<LlmToolCall>) -> Self {
        self.tool_calls = tool_calls;
        self
    }

    /// Get content length for performance estimation
    #[inline]
    pub fn content_length(&self) -> usize {
        self.content.len()
    }
}

/// Role of a message sender
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmRole {
    /// User message role
    User,
    /// Assistant message role
    Assistant,
    /// System message role
    System,
    /// Tool message role
    Tool,
}

/// Tool definition for LLM function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmTool {
    /// Name of the tool
    pub name: String,
    /// Description of what the tool does
    pub description: String,
    /// JSON schema for the tool parameters
    pub parameters: serde_json::Value,
}

impl LlmTool {
    /// Create a new tool definition
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters,
        }
    }
}

/// Tool call made by the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmToolCall {
    /// Unique ID for this tool call
    pub id: String,
    /// Name of the tool to call
    pub name: String,
    /// Parameters to pass to the tool
    pub parameters: serde_json::Value,
}

/// Factory for creating LLM providers
pub struct LlmProviderFactory;

impl LlmProviderFactory {
    /// Create a new LLM provider from configuration
    pub fn create_provider(config: LlmConfig) -> GraphBitResult<Box<dyn LlmProviderTrait>> {
        match config {
            LlmConfig::OpenAI { api_key, model, .. } => {
                Ok(Box::new(openai::OpenAiProvider::new(api_key, model)?))
            }
            LlmConfig::Anthropic { api_key, model, .. } => {
                Ok(Box::new(anthropic::AnthropicProvider::new(api_key, model)?))
            }
            LlmConfig::DeepSeek {
                api_key,
                model,
                base_url,
                ..
            } => {
                if let Some(base_url) = base_url {
                    Ok(Box::new(deepseek::DeepSeekProvider::with_base_url(
                        api_key, model, base_url,
                    )?))
                } else {
                    Ok(Box::new(deepseek::DeepSeekProvider::new(api_key, model)?))
                }
            }
            LlmConfig::HuggingFace {
                api_key,
                model,
                base_url,
                ..
            } => {
                if let Some(base_url) = base_url {
                    Ok(Box::new(huggingface::HuggingFaceProvider::with_base_url(
                        api_key, model, base_url,
                    )?))
                } else {
                    Ok(Box::new(huggingface::HuggingFaceProvider::new(
                        api_key, model,
                    )?))
                }
            }
            LlmConfig::Ollama {
                model, base_url, ..
            } => {
                if let Some(base_url) = base_url {
                    Ok(Box::new(ollama::OllamaProvider::with_base_url(
                        model, base_url,
                    )?))
                } else {
                    Ok(Box::new(ollama::OllamaProvider::new(model)?))
                }
            }
            LlmConfig::Perplexity {
                api_key,
                model,
                base_url,
                ..
            } => {
                if let Some(base_url) = base_url {
                    Ok(Box::new(perplexity::PerplexityProvider::with_base_url(
                        api_key, model, base_url,
                    )?))
                } else {
                    Ok(Box::new(perplexity::PerplexityProvider::new(
                        api_key, model,
                    )?))
                }
            }
            LlmConfig::OpenRouter {
                api_key,
                model,
                base_url,
                site_url,
                site_name,
                ..
            } => {
                if let Some(base_url) = base_url {
                    Ok(Box::new(openrouter::OpenRouterProvider::with_base_url(
                        api_key, model, base_url,
                    )?))
                } else if site_url.is_some() || site_name.is_some() {
                    Ok(Box::new(openrouter::OpenRouterProvider::with_site_info(
                        api_key, model, site_url, site_name,
                    )?))
                } else {
                    Ok(Box::new(openrouter::OpenRouterProvider::new(
                        api_key, model,
                    )?))
                }
            }
            LlmConfig::Fireworks {
                api_key,
                model,
                base_url,
                ..
            } => {
                if let Some(base_url) = base_url {
                    Ok(Box::new(fireworks::FireworksProvider::with_base_url(
                        api_key, model, base_url,
                    )?))
                } else {
                    Ok(Box::new(fireworks::FireworksProvider::new(api_key, model)?))
                }
            }
            LlmConfig::Custom { provider_type, .. } => Err(GraphBitError::config(format!(
                "Unsupported custom provider: {provider_type}",
            ))),
        }
    }

    /// Create multiple providers for load balancing
    pub fn create_providers(
        configs: Vec<LlmConfig>,
    ) -> GraphBitResult<Vec<Box<dyn LlmProviderTrait>>> {
        configs
            .into_iter()
            .map(Self::create_provider)
            .collect::<GraphBitResult<Vec<_>>>()
    }
}
