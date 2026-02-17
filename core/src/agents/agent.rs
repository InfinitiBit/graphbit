//! Agent module - defines the core Agent struct and related traits/configurations
//! for executing tasks within the workflow automation framework

use async_trait::async_trait;
use crate::{AgentId, AgentMessage, GraphBitResult, LlmProvider, LlmResponse, MessageContent, WorkflowContext, agents::{config::AgentConfig, r#trait::AgentTrait}, llm::LlmRequest, validation::TypeValidator, ValidationResult};


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
        // Skip validation for Python bridge providers to avoid GIL-related issues during initialization
        #[cfg(feature = "python")]
        let is_python_bridge = matches!(
            config.llm_config,
            crate::llm::LlmConfig::PythonBridge { .. }
        );
        #[cfg(not(feature = "python"))]
        let is_python_bridge = false;

        if !is_python_bridge {
            // Use a small prompt for validation.
            let test_request = LlmRequest::new("Hello");
            if let Err(e) = llm_provider.complete(test_request).await {
                return Err(crate::errors::GraphBitError::config(format!(
                    "LLM configuration validation failed: {e}"
                )));
            }
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
                format!("Tool call: {tool_name} with parameters: {parameters}")
            }
            MessageContent::ToolResponse {
                tool_name,
                result,
                success,
            } => {
                format!("Tool {tool_name} response (success: {success}): {result}")
            }
            MessageContent::Error {
                error_code,
                error_message,
            } => {
                format!("Error {error_code}: {error_message}")
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
