//! Error types and conversions for JavaScript bindings

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::errors::GraphBitError as CoreGraphBitError;

/// Error types that can occur in GraphBit operations
#[napi]
pub enum ErrorKind {
    /// Configuration error
    Configuration,
    /// Validation error
    Validation,
    /// Execution error
    Execution,
    /// Network/IO error
    Network,
    /// LLM provider error
    LlmProvider,
    /// Agent error
    Agent,
    /// Workflow error
    Workflow,
    /// Graph error
    Graph,
    /// Document processing error
    DocumentProcessing,
    /// Serialization error
    Serialization,
    /// Authentication error
    Authentication,
    /// Rate limit error
    RateLimit,
    /// Internal error
    Internal,
    /// IO error
    Io,
    /// Concurrency error
    Concurrency,
    /// Unknown error
    Unknown,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ErrorKind::Configuration => "Configuration",
            ErrorKind::Validation => "Validation",
            ErrorKind::Execution => "Execution",
            ErrorKind::Network => "Network",
            ErrorKind::LlmProvider => "LlmProvider",
            ErrorKind::Agent => "Agent",
            ErrorKind::Workflow => "Workflow",
            ErrorKind::Graph => "Graph",
            ErrorKind::DocumentProcessing => "DocumentProcessing",
            ErrorKind::Serialization => "Serialization",
            ErrorKind::Authentication => "Authentication",
            ErrorKind::RateLimit => "RateLimit",
            ErrorKind::Internal => "Internal",
            ErrorKind::Io => "Io",
            ErrorKind::Concurrency => "Concurrency",
            ErrorKind::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// GraphBit error object exposed to JavaScript
#[napi(object)]
pub struct GraphBitError {
    /// Error kind/category
    pub kind: ErrorKind,
    /// Error message
    pub message: String,
    /// Optional error details
    pub details: Option<String>,
    /// Optional error code
    pub code: Option<String>,
}

impl From<CoreGraphBitError> for GraphBitError {
    fn from(err: CoreGraphBitError) -> Self {
        let (kind, message, details, code) = match &err {
            CoreGraphBitError::Configuration { message } => (ErrorKind::Configuration, message.clone(), None, None),
            CoreGraphBitError::Validation { field, message } => (ErrorKind::Validation, message.clone(), Some(format!("Field: {}", field)), None),
            CoreGraphBitError::WorkflowExecution { message } => (ErrorKind::Execution, message.clone(), None, None),
            CoreGraphBitError::Network { message } => (ErrorKind::Network, message.clone(), None, None),
            CoreGraphBitError::LlmProvider { provider, message } => (ErrorKind::LlmProvider, message.clone(), Some(format!("Provider: {}", provider)), None),
            CoreGraphBitError::Llm { message } => (ErrorKind::LlmProvider, message.clone(), None, None),
            CoreGraphBitError::Agent { agent_id, message } => (ErrorKind::Agent, message.clone(), Some(format!("Agent ID: {}", agent_id)), None),
            CoreGraphBitError::AgentNotFound { agent_id } => (ErrorKind::Agent, format!("Agent not found: {}", agent_id), Some(format!("Agent ID: {}", agent_id)), Some("AGENT_NOT_FOUND".to_string())),
            CoreGraphBitError::Graph { message } => (ErrorKind::Graph, message.clone(), None, None),
            CoreGraphBitError::Serialization { message } => (ErrorKind::Serialization, message.clone(), None, None),
            CoreGraphBitError::Authentication { provider, message } => (ErrorKind::Authentication, message.clone(), Some(format!("Provider: {}", provider)), None),
            CoreGraphBitError::RateLimit { provider, retry_after_seconds } => (ErrorKind::RateLimit, format!("Rate limit exceeded for {}", provider), Some(format!("Retry after: {}s", retry_after_seconds)), Some("RATE_LIMIT".to_string())),
            CoreGraphBitError::Internal { message } => (ErrorKind::Internal, message.clone(), None, None),
            CoreGraphBitError::Io { message } => (ErrorKind::Io, message.clone(), None, None),
            CoreGraphBitError::Concurrency { message } => (ErrorKind::Concurrency, message.clone(), None, None),
        };

        Self {
            kind,
            message,
            details,
            code,
        }
    }
}

impl GraphBitError {
    /// Create a new GraphBit error
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            details: None,
            code: None,
        }
    }

    /// Create a new error with details
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Create a new error with code
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

/// Convert core GraphBitError to JavaScript error
pub fn to_napi_error(err: CoreGraphBitError) -> Error {
    let message = err.to_string();
    let graphbit_error: GraphBitError = err.into();
    
    // Create a generic error but prepend the kind for clarity
    Error::new(
        Status::GenericFailure,
        format!("[{}] {}", graphbit_error.kind, message),
    )
}

/// Helper function to create a configuration error
pub fn configuration_error(msg: impl Into<String>) -> Error {
    Error::new(
        Status::InvalidArg,
        format!("[Configuration] {}", msg.into()),
    )
}

/// Helper function to create a validation error
pub fn validation_error(msg: impl Into<String>) -> Error {
    Error::new(
        Status::InvalidArg,
        format!("[Validation] {}", msg.into()),
    )
}

/// Helper function to create an execution error
pub fn execution_error(msg: impl Into<String>) -> Error {
    Error::new(
        Status::GenericFailure,
        format!("[Execution] {}", msg.into()),
    )
}

/// Helper function to create a network error
pub fn network_error(msg: impl Into<String>) -> Error {
    Error::new(
        Status::GenericFailure,
        format!("[Network] {}", msg.into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = GraphBitError::new(ErrorKind::Configuration, "test error");
        assert_eq!(err.message, "test error");
        assert!(err.details.is_none());
        assert!(err.code.is_none());
    }

    #[test]
    fn test_error_with_details() {
        let err = GraphBitError::new(ErrorKind::Validation, "test error")
            .with_details("additional details");
        assert_eq!(err.message, "test error");
        assert_eq!(err.details, Some("additional details".to_string()));
    }
}

