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
    /// Unknown error
    Unknown,
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

    // Determine error kind based on the error type
    let kind = match &err {
        CoreGraphBitError::Configuration { .. } => "Configuration",
        CoreGraphBitError::Validation { .. } => "Validation",
        CoreGraphBitError::WorkflowExecution { .. } => "Execution",
        CoreGraphBitError::Network { .. } => "Network",
        CoreGraphBitError::LlmProvider { .. } => "LlmProvider",
        CoreGraphBitError::Llm { .. } => "LlmProvider",
        CoreGraphBitError::Agent { .. } => "Agent",
        CoreGraphBitError::AgentNotFound { .. } => "Agent",
        CoreGraphBitError::Graph { .. } => "Graph",
        CoreGraphBitError::Serialization { .. } => "Serialization",
        _ => "Unknown",
    };

    Error::new(
        Status::GenericFailure,
        format!("[{}] {}", kind, message),
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

