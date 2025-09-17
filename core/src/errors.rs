//! Error handling for GraphBit Core
//!
//! This module provides a comprehensive error handling system that covers
//! all potential failure modes in the agentic workflow framework.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type alias for GraphBit operations
pub type GraphBitResult<T> = Result<T, GraphBitError>;

/// Comprehensive error types for the GraphBit framework
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum GraphBitError {
    /// Configuration related errors
    #[error("Configuration error: {message}")]
    Configuration {
        /// Error message
        message: String,
    },

    /// LLM provider errors
    #[error("LLM provider error: {provider} - {message}")]
    LlmProvider {
        /// Provider name
        provider: String,
        /// Error message
        message: String,
    },

    /// Generic LLM errors
    #[error("LLM error: {message}")]
    Llm {
        /// Error message
        message: String,
    },

    /// Network communication errors
    #[error("Network error: {message}")]
    Network {
        /// Error message
        message: String,
    },

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {message}")]
    Serialization {
        /// Error message
        message: String,
    },

    /// Workflow execution errors
    #[error("Workflow execution error: {message}")]
    WorkflowExecution {
        /// Error message
        message: String,
    },

    /// Graph structure errors
    #[error("Graph error: {message}")]
    Graph {
        /// Error message
        message: String,
    },

    /// Agent-related errors
    #[error("Agent error: {agent_id} - {message}")]
    Agent {
        /// Agent identifier
        agent_id: String,
        /// Error message
        message: String,
    },

    /// Agent not found errors
    #[error("Agent not found: {agent_id}")]
    AgentNotFound {
        /// Agent identifier
        agent_id: String,
    },

    /// Type validation errors
    #[error("Validation error: {field} - {message}")]
    Validation {
        /// Field name that failed validation
        field: String,
        /// Error message
        message: String,
    },

    /// Authentication and authorization errors
    #[error("Authentication error: {provider} - {message}")]
    Authentication {
        /// Provider name
        provider: String,
        /// Error message
        message: String,
    },

    /// Rate limiting errors
    #[error("Rate limit exceeded: {provider} - retry after {retry_after_seconds}s")]
    RateLimit {
        /// Provider name
        provider: String,
        /// Seconds to wait before retrying
        retry_after_seconds: u64,
    },

    /// Generic internal errors
    #[error("Internal error: {message}")]
    Internal {
        /// Error message
        message: String,
    },

    /// IO errors
    #[error("IO error: {message}")]
    Io {
        /// Error message
        message: String,
    },

    /// Concurrency control errors
    #[error("Concurrency error: {message}")]
    Concurrency {
        /// Error message
        message: String,
    },
}

impl GraphBitError {
    /// Create a new configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a new LLM provider error
    pub fn llm_provider(provider: impl Into<String>, message: impl Into<String>) -> Self {
        Self::LlmProvider {
            provider: provider.into(),
            message: message.into(),
        }
    }

    /// Create a new generic LLM error
    pub fn llm(message: impl Into<String>) -> Self {
        Self::Llm {
            message: message.into(),
        }
    }

    /// Create a new workflow execution error
    pub fn workflow_execution(message: impl Into<String>) -> Self {
        Self::WorkflowExecution {
            message: message.into(),
        }
    }

    /// Create a new graph error
    pub fn graph(message: impl Into<String>) -> Self {
        Self::Graph {
            message: message.into(),
        }
    }

    /// Create a new agent error
    pub fn agent(agent_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Agent {
            agent_id: agent_id.into(),
            message: message.into(),
        }
    }

    /// Create a new agent not found error
    pub fn agent_not_found(agent_id: impl Into<String>) -> Self {
        Self::AgentNotFound {
            agent_id: agent_id.into(),
        }
    }

    /// Create a new validation error
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new authentication error
    pub fn authentication(provider: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Authentication {
            provider: provider.into(),
            message: message.into(),
        }
    }

    /// Create a new rate limit error
    pub fn rate_limit(provider: impl Into<String>, retry_after_seconds: u64) -> Self {
        Self::RateLimit {
            provider: provider.into(),
            retry_after_seconds,
        }
    }

    /// Create a new concurrency error
    pub fn concurrency(message: impl Into<String>) -> Self {
        Self::Concurrency {
            message: message.into(),
        }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            GraphBitError::Network { .. }
                | GraphBitError::RateLimit { .. }
                | GraphBitError::LlmProvider { .. }
                | GraphBitError::Llm { .. }
        )
    }

    /// Get retry delay in seconds for retryable errors
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            GraphBitError::RateLimit {
                retry_after_seconds,
                ..
            } => Some(*retry_after_seconds),
            GraphBitError::Network { .. } => Some(1),
            GraphBitError::LlmProvider { .. } => Some(2),
            GraphBitError::Llm { .. } => Some(1),
            _ => None,
        }
    }
}

// Implement From traits for external error types
impl From<reqwest::Error> for GraphBitError {
    fn from(error: reqwest::Error) -> Self {
        Self::Network {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for GraphBitError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization {
            message: error.to_string(),
        }
    }
}

impl From<anyhow::Error> for GraphBitError {
    fn from(error: anyhow::Error) -> Self {
        Self::Internal {
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for GraphBitError {
    fn from(error: std::io::Error) -> Self {
        Self::Io {
            message: error.to_string(),
        }
    }
}
