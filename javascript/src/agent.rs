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
}

