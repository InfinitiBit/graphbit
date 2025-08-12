//! Agent system for GraphBit
//!
//! This module provides the agent abstraction and implementations for
//! executing tasks within the workflow automation framework.

use crate::errors::GraphBitResult;
use crate::llm::{LlmProvider, LlmRequest, LlmResponse};
use crate::types::{
    AgentCapability, AgentId, AgentMessage, MessageContent, ToolCallResult, ToolDefinition,
    WorkflowContext,
};
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
    /// Available tools for this agent
    pub tools: Option<Vec<ToolDefinition>>,
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
            tools: None,
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

    /// Set available tools for the agent
    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Add a single tool to the agent
    pub fn add_tool(mut self, tool: ToolDefinition) -> Self {
        if let Some(ref mut existing_tools) = self.tools {
            existing_tools.push(tool);
        } else {
            self.tools = Some(vec![tool]);
        }
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
        let mut request = LlmRequest::new("");

        // Add system prompt if available
        if !self.config.system_prompt.is_empty() {
            request =
                request.with_message(crate::llm::LlmMessage::system(&self.config.system_prompt));
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

        request = request.with_message(crate::llm::LlmMessage::user(content));

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

    /// Execute a tool call if tools are available
    async fn execute_tool_call(
        &self,
        tool_name: &str,
        parameters: &serde_json::Value,
    ) -> ToolCallResult {
        let start_time = std::time::Instant::now();

        // Check if the agent has tools configured
        let tools = match &self.config.tools {
            Some(tools) => tools,
            None => {
                return ToolCallResult::failure("No tools configured for this agent");
            }
        };

        // Find the requested tool
        let tool = match tools.iter().find(|t| t.name == tool_name && t.enabled) {
            Some(tool) => tool,
            None => {
                return ToolCallResult::failure(format!(
                    "Tool '{}' not found or not enabled",
                    tool_name
                ));
            }
        };

        // For now, we'll simulate tool execution
        // In a full implementation, this would call the actual Python function
        // through the tool registry or similar mechanism
        let result = serde_json::json!({
            "message": format!("Tool '{}' executed successfully", tool_name),
            "parameters": parameters,
            "tool_description": tool.description,
            "simulated": true
        });

        ToolCallResult::success(result)
            .with_duration(start_time.elapsed().as_millis() as u64)
            .with_metadata(
                "tool_category".to_string(),
                serde_json::Value::String(
                    tool.category
                        .clone()
                        .unwrap_or_else(|| "general".to_string()),
                ),
            )
    }

    /// Check if a message contains a tool call request and should trigger tool execution
    fn should_execute_tool(&self, message: &AgentMessage) -> Option<(String, serde_json::Value)> {
        match &message.content {
            MessageContent::ToolCall {
                tool_name,
                parameters,
            } => Some((tool_name.clone(), parameters.clone())),
            _ => None,
        }
    }

    /// Parse LLM response for tool calls (detect if LLM is requesting to call a tool)
    fn parse_llm_response_for_tool_calls(
        &self,
        response: &str,
    ) -> Option<(String, serde_json::Value)> {
        // Simple pattern matching for tool calls in LLM response
        // In a more sophisticated implementation, this would use structured parsing
        // or the LLM's function calling capabilities

        if let Some(tools) = &self.config.tools {
            for tool in tools {
                if !tool.enabled {
                    continue;
                }

                // Look for patterns like "call tool_name" or "use tool_name with {params}"
                let tool_call_patterns = [
                    format!("call {}", tool.name),
                    format!("use {}", tool.name),
                    format!("execute {}", tool.name),
                    format!("invoke {}", tool.name),
                ];

                for pattern in &tool_call_patterns {
                    if response.to_lowercase().contains(&pattern.to_lowercase()) {
                        // Try to extract parameters - simplified JSON extraction
                        let params = if let Some(start) = response.find('{') {
                            if let Some(end) = response.rfind('}') {
                                if end > start {
                                    let param_str = &response[start..=end];
                                    serde_json::from_str(param_str)
                                        .unwrap_or_else(|_| serde_json::json!({}))
                                } else {
                                    serde_json::json!({})
                                }
                            } else {
                                serde_json::json!({})
                            }
                        } else {
                            serde_json::json!({})
                        };

                        return Some((tool.name.clone(), params));
                    }
                }
            }
        }

        None
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
        // Check if this is a direct tool call request
        if let Some((tool_name, parameters)) = self.should_execute_tool(&message) {
            let tool_result = self.execute_tool_call(&tool_name, &parameters).await;

            // Update context with tool execution information
            context.set_metadata(
                "last_tool_call".to_string(),
                serde_json::json!({
                    "tool_name": tool_name,
                    "parameters": parameters,
                    "success": tool_result.success,
                    "duration_ms": tool_result.duration_ms
                }),
            );

            return Ok(AgentMessage::new(
                self.config.id.clone(),
                Some(message.sender.clone()),
                MessageContent::ToolResponse {
                    tool_name,
                    result: tool_result.result,
                    success: tool_result.success,
                },
            ));
        }

        // Build and send LLM request
        let request = self.build_llm_request(&message);
        let response = self.llm_provider.complete(request).await?;

        // Update context with usage information
        context.set_metadata(
            "last_token_usage".to_string(),
            serde_json::to_value(&response.usage)?,
        );

        // Check if LLM response contains a tool call request
        if let Some((tool_name, parameters)) =
            self.parse_llm_response_for_tool_calls(&response.content)
        {
            let tool_result = self.execute_tool_call(&tool_name, &parameters).await;

            // Update context with tool execution information
            context.set_metadata(
                "last_tool_call".to_string(),
                serde_json::json!({
                    "tool_name": tool_name,
                    "parameters": parameters,
                    "success": tool_result.success,
                    "duration_ms": tool_result.duration_ms,
                    "triggered_by": "llm_response"
                }),
            );

            // Return tool response instead of the original LLM response
            return Ok(AgentMessage::new(
                self.config.id.clone(),
                Some(message.sender.clone()),
                MessageContent::ToolResponse {
                    tool_name,
                    result: tool_result.result,
                    success: tool_result.success,
                },
            ));
        }

        // No tool call detected, return the LLM response as normal
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

    /// Set available tools for the agent
    pub fn tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.config.tools = Some(tools);
        self
    }

    /// Add a single tool to the agent
    pub fn add_tool(mut self, tool: ToolDefinition) -> Self {
        if let Some(ref mut existing_tools) = self.config.tools {
            existing_tools.push(tool);
        } else {
            self.config.tools = Some(vec![tool]);
        }
        self
    }

    /// Build the agent
    pub async fn build(self) -> GraphBitResult<Agent> {
        Agent::new(self.config).await
    }
}
