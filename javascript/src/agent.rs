//! Agent bindings for JavaScript

use napi_derive::napi;
use crate::llm::LlmConfig;

/// Agent builder for constructing agents
#[napi]
pub struct AgentBuilder {
    name: String,
    description: Option<String>,
    system_prompt: Option<String>,
    temperature: Option<f64>,
    max_tokens: Option<u32>,
    llm_config: crate::llm::LlmConfig,
}

#[napi]
impl AgentBuilder {
    /// Create a new agent builder
    #[napi(constructor)]
    pub fn new(name: String, llm_config: &LlmConfig) -> Self {
        Self {
            name,
            description: None,
            system_prompt: None,
            temperature: None,
            max_tokens: None,
            llm_config: LlmConfig {
                inner: llm_config.clone_inner(),
            },
        }
    }

    /// Set agent description
    #[napi]
    pub fn description(&mut self, description: String) -> &Self {
        self.description = Some(description);
        self
    }

    /// Set system prompt
    #[napi]
    pub fn system_prompt(&mut self, prompt: String) -> &Self {
        self.system_prompt = Some(prompt);
        self
    }

    /// Set temperature
    #[napi]
    pub fn temperature(&mut self, temp: f64) -> &Self {
        self.temperature = Some(temp);
        self
    }

    /// Set max tokens
    #[napi]
    pub fn max_tokens(&mut self, tokens: u32) -> &Self {
        self.max_tokens = Some(tokens);
        self
    }

    /// Build the agent - returns agent config
    #[napi]
    pub fn build(&self) -> Agent {
        Agent {
            name: self.name.clone(),
            description: self.description.clone().unwrap_or_default(),
            system_prompt: self.system_prompt.clone().unwrap_or_default(),
        }
    }
}

/// Agent representation
#[napi]
pub struct Agent {
    name: String,
    description: String,
    system_prompt: String,
}

#[napi]
impl Agent {
    /// Get agent name
    #[napi]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Get agent description
    #[napi]
    pub fn description(&self) -> String {
        self.description.clone()
    }
}

