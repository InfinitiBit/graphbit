//! Agent builder module - provides a fluent API for constructing agents with various configurations and capabilities
//! This builder simplifies the creation of agents by allowing users to specify configurations in a step-by-step manner, 
//! and then build the final agent instance ready for use in workflows.

use crate::{AgentCapability, AgentId, GraphBitResult, agents::{agent::Agent, config::AgentConfig}};

/// Builder for creating agents with fluent API
pub struct AgentBuilder {
    config: AgentConfig,
}

impl AgentBuilder {
    /// Start building an agent
    pub fn new(name: impl Into<String>, llm_config: crate::llm::LlmConfig) -> Self {
        let config = AgentConfig::new(name, "", llm_config);
        Self { config }
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.config.description = description.into();
        self
    }

    /// Add capabilities
    pub fn capabilities(mut self, capabilities: Vec<AgentCapability>) -> Self {
        self.config.capabilities = capabilities;
        self
    }

    /// Set system prompt
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.config.system_prompt = prompt.into();
        self
    }

    /// Set max tokens
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.config.max_tokens = Some(max_tokens);
        self
    }

    /// Set temperature
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.config.temperature = Some(temperature);
        self
    }

    /// Set a specific agent ID (overrides the auto-generated ID)
    pub fn with_id(mut self, id: AgentId) -> Self {
        self.config.id = id;
        self
    }

    /// Build the agent
    pub async fn build(self) -> GraphBitResult<Agent> {
        Agent::new(self.config).await
    }
}
