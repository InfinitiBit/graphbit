//! Agent bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::agents::{Agent as CoreAgent, AgentBuilder as CoreAgentBuilder, AgentTrait};
use graphbit_core::types::{AgentMessage as CoreAgentMessage, MessageContent as CoreMessageContent};
use crate::llm::LlmConfig;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Agent builder for constructing agents
#[napi]
pub struct AgentBuilder {
    inner: Option<CoreAgentBuilder>,
}

#[napi]
impl AgentBuilder {
    /// Create a new agent builder
    #[napi(constructor)]
    pub fn new(name: String, llm_config: &LlmConfig) -> Self {
        let core_llm_config = llm_config.clone_inner();
        Self {
            inner: Some(CoreAgentBuilder::new(name, core_llm_config)),
        }
    }

    /// Set agent description
    #[napi]
    pub fn description(&mut self, description: String) -> Result<&Self> {
        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;
        self.inner = Some(builder.description(description));
        Ok(self)
    }

    /// Set system prompt
    #[napi]
    pub fn system_prompt(&mut self, prompt: String) -> Result<&Self> {
        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;
        self.inner = Some(builder.system_prompt(prompt));
        Ok(self)
    }

    /// Set temperature
    #[napi]
    pub fn temperature(&mut self, temp: f64) -> Result<&Self> {
        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;
        self.inner = Some(builder.temperature(temp as f32));
        Ok(self)
    }

    /// Set max tokens
    #[napi]
    pub fn max_tokens(&mut self, tokens: u32) -> Result<&Self> {
        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;
        self.inner = Some(builder.max_tokens(tokens));
        Ok(self)
    }

    /// Build the agent
    ///
    /// Note: This method is async and marked as unsafe because it mutates self.
    /// This is required by napi-rs for async methods that take &mut self.
    #[napi]
    pub async unsafe fn build(&mut self) -> Result<Agent> {
        let builder = self.inner.take()
            .ok_or_else(|| Error::from_reason("Builder already consumed"))?;

        let agent = builder.build().await
            .map_err(crate::errors::to_napi_error)?;

        Ok(Agent {
            inner: Arc::new(Mutex::new(agent)),
        })
    }
}

/// Agent representation
#[napi]
pub struct Agent {
    inner: Arc<Mutex<CoreAgent>>,
}

#[napi]
impl Agent {
    /// Get agent name
    #[napi]
    pub async fn name(&self) -> Result<String> {
        let agent = self.inner.lock().await;
        Ok(agent.config().name.clone())
    }

    /// Get agent description
    #[napi]
    pub async fn description(&self) -> Result<String> {
        let agent = self.inner.lock().await;
        Ok(agent.config().description.clone())
    }

    /// Get agent ID
    #[napi]
    pub async fn id(&self) -> Result<crate::types::AgentId> {
        let agent = self.inner.lock().await;
        Ok(agent.config().id.clone().into())
    }

    /// Execute the agent with a message and return the response
    ///
    /// # Arguments
    /// * `message` - The message text to send to the agent
    ///
    /// # Returns
    /// The agent's response as a JSON string or plain text
    ///
    /// # Example
    /// ```javascript
    /// const response = await agent.execute("What is the capital of France?");
    /// console.log(response); // "Paris"
    /// ```
    #[napi]
    pub async fn execute(&self, message: String) -> Result<String> {
        let agent = self.inner.lock().await;

        // Create a simple AgentMessage with the text content
        let agent_id = agent.config().id.clone();
        let agent_message = CoreAgentMessage::new(
            agent_id,
            None,
            CoreMessageContent::Text(message),
        );

        // Execute the agent
        let result = agent.execute(agent_message).await
            .map_err(crate::errors::to_napi_error)?;

        // Convert the result to a string
        Ok(result.to_string())
    }

    /// Get agent configuration
    #[napi]
    pub async fn config(&self) -> Result<AgentConfig> {
        let agent = self.inner.lock().await;
        let config = agent.config();
        
        Ok(AgentConfig {
            id: config.id.clone().into(),
            name: config.name.clone(),
            description: config.description.clone(),
            capabilities: config.capabilities.iter().map(|c| c.clone().into()).collect(),
            system_prompt: config.system_prompt.clone(),
            llm_config: serde_json::to_value(&config.llm_config).map_err(|e| Error::from_reason(e.to_string()))?,
            max_tokens: config.max_tokens,
            temperature: config.temperature.map(|t| t as f64),
        })
    }
}

/// Agent configuration
#[napi(object)]
pub struct AgentConfig {
    /// Agent ID
    pub id: crate::types::AgentId,
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: String,
    /// Agent capabilities
    pub capabilities: Vec<crate::types::AgentCapability>,
    /// System prompt
    pub system_prompt: String,
    /// LLM configuration
    #[napi(ts_type = "any")]
    pub llm_config: serde_json::Value,
    /// Max tokens
    pub max_tokens: Option<u32>,
    /// Temperature
    pub temperature: Option<f64>,
}

