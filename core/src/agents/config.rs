//! Agent configuration module - defines the AgentConfig struct which encapsulates all necessary
//! configuration for creating and running an agent, including capabilities, system prompts and LLM settings

use crate::types::{AgentCapability, AgentId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent ID
    pub id: AgentId,
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: String,
    /// Agent capabilities
    pub capabilities: Vec<AgentCapability>,
    /// System prompt/instructions
    pub system_prompt: String,
    /// LLM provider configuration
    pub llm_config: crate::llm::LlmConfig,
    /// Maximum tokens for responses
    pub max_tokens: Option<u32>,
    /// Temperature for LLM responses
    pub temperature: Option<f32>,
    /// Custom configuration
    pub custom_config: HashMap<String, serde_json::Value>,
}

impl AgentConfig {
    /// Create a new agent configuration
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        llm_config: crate::llm::LlmConfig,
    ) -> Self {
        Self {
            id: AgentId::new(),
            name: name.into(),
            description: description.into(),
            capabilities: Vec::new(),
            system_prompt: String::new(),
            llm_config,
            max_tokens: None,
            temperature: None,
            custom_config: HashMap::with_capacity(4), // Pre-allocate for custom config
        }
    }

    /// Add capabilities
    pub fn with_capabilities(mut self, capabilities: Vec<AgentCapability>) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Set system prompt
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set agent ID explicitly
    pub fn with_id(mut self, id: AgentId) -> Self {
        self.id = id;
        self
    }
}

