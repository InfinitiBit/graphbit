//! Agent trait definition - defines the core AgentTrait that all agents must implement, along with
//! default implementations for common functionality like capability checking and LLM interactions

use async_trait::async_trait;

use crate::{AgentCapability, AgentId, AgentMessage, GraphBitResult, LlmProvider, ValidationResult, WorkflowContext, agents::config::AgentConfig};

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
