//! Agent system for `GraphBit`
//!
//! This module provides the agent abstraction and implementations for
//! executing tasks within the workflow automation framework.

use crate::errors::GraphBitResult;
use crate::llm::{LlmProvider, LlmRequest, LlmResponse};
use crate::types::{AgentCapability, AgentId, AgentMessage, MessageContent, WorkflowContext};
use crate::validation::{TypeValidator, ValidationResult};
use async_trait::async_trait;
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

/// Trait that all agents must implement
#[async_trait]
pub trait AgentTrait: Send + Sync {
    /// Get agent ID
    fn id(&self) -> &AgentId;

    /// Get agent configuration
    fn config(&self) -> &AgentConfig;

    /// Process a message and return a response
    async fn process_message(
        &self,
        message: AgentMessage,
        context: &mut WorkflowContext,
    ) -> GraphBitResult<AgentMessage>;

    /// Execute a message and return structured data (for optimized workflow execution)
    async fn execute(&self, message: AgentMessage) -> GraphBitResult<serde_json::Value>;

    /// Validate agent output against expected schema
    async fn validate_output(&self, output: &str, schema: &serde_json::Value) -> ValidationResult;

    /// Check if agent has a specific capability
    fn has_capability(&self, capability: &AgentCapability) -> bool {
        self.config().capabilities.contains(capability)
    }

    /// Get supported capabilities
    fn capabilities(&self) -> &[AgentCapability] {
        &self.config().capabilities
    }

    /// Get access to the LLM provider for direct tool calling
    fn llm_provider(&self) -> &LlmProvider;
}

/// Standard LLM-based agent implementation
pub struct Agent {
    config: AgentConfig,
    llm_provider: LlmProvider,
    validator: TypeValidator,
}

impl Agent {
    /// Create a new agent
    pub async fn new(config: AgentConfig) -> GraphBitResult<Self> {
        let provider = crate::llm::LlmProviderFactory::create_provider(config.llm_config.clone())?;
        let llm_provider = LlmProvider::new(provider, config.llm_config.clone());

        // Validate the LLM configuration by attempting a simple test call
        // This ensures invalid API keys are caught during agent creation
        let test_request = LlmRequest::new("test").with_max_tokens(1);
        if let Err(e) = llm_provider.complete(test_request).await {
            return Err(crate::errors::GraphBitError::config(format!(
                "LLM configuration validation failed: {}",
                e
            )));
        }

        Ok(Self {
            config,
            llm_provider,
            validator: TypeValidator::new(),
        })
    }

    /// Build an LLM request from a message
    fn build_llm_request(&self, message: &AgentMessage) -> LlmRequest {
        let mut messages = Vec::new();

        // Add system prompt if available
        if !self.config.system_prompt.is_empty() {
            messages.push(crate::llm::LlmMessage::system(&self.config.system_prompt));
        }

        // Add the user message
        let content = match &message.content {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Data(data) => data.to_string(),
            MessageContent::ToolCall {
                tool_name,
                parameters,
            } => {
                format!("Tool call: {} with parameters: {}", tool_name, parameters)
            }
            MessageContent::ToolResponse {
                tool_name,
                result,
                success,
            } => {
                format!(
                    "Tool {} response (success: {}): {}",
                    tool_name, success, result
                )
            }
            MessageContent::Error {
                error_code,
                error_message,
            } => {
                format!("Error {}: {}", error_code, error_message)
            }
        };

        messages.push(crate::llm::LlmMessage::user(content));

        // Create request with messages
        let mut request = LlmRequest::with_messages(messages);

        // Apply configuration
        if let Some(max_tokens) = self.config.max_tokens {
            request = request.with_max_tokens(max_tokens);
        }

        if let Some(temperature) = self.config.temperature {
            request = request.with_temperature(temperature);
        }

        request
    }

    /// Convert LLM response to agent message
    fn llm_response_to_message(
        &self,
        response: LlmResponse,
        original_message: &AgentMessage,
    ) -> AgentMessage {
        AgentMessage::new(
            self.config.id.clone(),
            Some(original_message.sender.clone()),
            MessageContent::Text(response.content),
        )
    }
}

#[async_trait]
impl AgentTrait for Agent {
    fn id(&self) -> &AgentId {
        &self.config.id
    }

    fn config(&self) -> &AgentConfig {
        &self.config
    }

    async fn process_message(
        &self,
        message: AgentMessage,
        context: &mut WorkflowContext,
    ) -> GraphBitResult<AgentMessage> {
        let request = self.build_llm_request(&message);

        let response = self.llm_provider.complete(request).await?;

        // Update context with usage information
        context.set_metadata(
            "last_token_usage".to_string(),
            serde_json::to_value(&response.usage)?,
        );

        Ok(self.llm_response_to_message(response, &message))
    }

    async fn execute(&self, message: AgentMessage) -> GraphBitResult<serde_json::Value> {
        let request = self.build_llm_request(&message);

        let response = self.llm_provider.complete(request).await?;

        // Try to parse response as JSON, fallback to string
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response.content) {
            Ok(json_value)
        } else {
            Ok(serde_json::Value::String(response.content))
        }
    }

    async fn validate_output(&self, output: &str, schema: &serde_json::Value) -> ValidationResult {
        self.validator.validate_against_schema(output, schema)
    }

    fn llm_provider(&self) -> &LlmProvider {
        &self.llm_provider
    }
}

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
