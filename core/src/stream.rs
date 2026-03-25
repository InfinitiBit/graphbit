//! Streaming types for workflow execution
//!
//! This module defines the event and mode types used by the streaming
//! workflow executor to deliver real-time progress updates.
//!
//! # Stream Modes
//!
//! - `Updates` — one event per node start/finish (default)
//! - `Messages` — real-time LLM tokens + tool call events + node events
//! - `All` — all event types (currently equivalent to `Messages`)
//!
//! # Event Types
//!
//! Events are divided into three categories:
//! 1. **Node-level** (all modes): `WorkflowStarted`, `NodeStarted`, `NodeCompleted`,
//!    `NodeFailed`, `WorkflowCompleted`, `WorkflowFailed`
//! 2. **Token-level** (`Messages`/`All`): `Token`
//! 3. **Tool call** (`Messages`/`All`): `ToolCallStarted`, `ToolCallCompleted`,
//!    `ToolCallFailed`

use crate::errors::GraphBitError;
use crate::types::WorkflowContext;
use serde::{Deserialize, Serialize};

// ──────────────────────────────────────────────────────────────────────────────
// StreamMode
// ──────────────────────────────────────────────────────────────────────────────

/// Controls the granularity of events emitted during streaming execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamMode {
    /// Node-level: one event per node start/finish (default).
    Updates,
    /// Token-level: real-time LLM tokens + tool call events + node events.
    Messages,
    /// All events — currently equivalent to `Messages`, but semantically
    /// distinct for forward compatibility when additional event categories
    /// are added.
    All,
}

impl StreamMode {
    /// Whether this mode emits `Token` events (real-time LLM output).
    #[inline]
    pub fn emits_tokens(&self) -> bool {
        matches!(self, Self::Messages | Self::All)
    }

    /// Whether this mode emits `ToolCall*` events.
    #[inline]
    pub fn emits_tool_events(&self) -> bool {
        matches!(self, Self::Messages | Self::All)
    }

    /// Parse a stream mode from a string (case-insensitive).
    ///
    /// Returns `None` for invalid values.
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "updates" => Some(Self::Updates),
            "messages" => Some(Self::Messages),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

impl std::fmt::Display for StreamMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Updates => write!(f, "updates"),
            Self::Messages => write!(f, "messages"),
            Self::All => write!(f, "all"),
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// StreamEvent
// ──────────────────────────────────────────────────────────────────────────────

/// Events emitted during streaming workflow execution.
///
/// These events are sent through a `tokio::sync::mpsc` channel from the
/// executor to the Python-side iterator. Each event is converted to a
/// Python dict before being yielded.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum StreamEvent {
    // ── Node-level events (all modes) ────────────────────────────────────
    /// Workflow execution started.
    #[serde(rename = "workflow_started")]
    WorkflowStarted {
        /// Workflow UUID
        workflow_id: String,
        /// Human-readable workflow name
        workflow_name: String,
        /// Total number of nodes in the workflow
        total_nodes: usize,
    },

    /// A node began execution.
    #[serde(rename = "node_started")]
    NodeStarted {
        /// Node UUID
        node_id: String,
        /// Human-readable node name
        node_name: String,
    },

    /// A node completed successfully.
    #[serde(rename = "node_completed")]
    NodeCompleted {
        /// Node UUID
        node_id: String,
        /// Human-readable node name
        node_name: String,
        /// Node output (typically a string or JSON value)
        output: serde_json::Value,
    },

    /// A node failed.
    #[serde(rename = "node_failed")]
    NodeFailed {
        /// Node UUID
        node_id: String,
        /// Human-readable node name
        node_name: String,
        /// Error description
        error: String,
        /// Python exception type hint (e.g., "connection_error", "timeout_error")
        error_type: String,
    },

    /// Workflow completed successfully.
    ///
    /// The `context` field contains the full `WorkflowContext` with identical
    /// metadata to what `execute()` produces.
    #[serde(rename = "workflow_completed")]
    WorkflowCompleted {
        /// Final workflow context (converted to `WorkflowResult` on the Python side)
        context: WorkflowContext,
    },

    /// Workflow failed.
    #[serde(rename = "workflow_failed")]
    WorkflowFailed {
        /// Error description
        error: String,
        /// Python exception type hint
        error_type: String,
    },

    // ── Token-level events (Messages / All) ──────────────────────────────
    /// A single LLM token generated during node execution.
    #[serde(rename = "token")]
    Token {
        /// Node UUID producing this token
        node_id: String,
        /// Human-readable node name
        node_name: String,
        /// Token content (may be a single character or word fragment)
        content: String,
    },

    // ── Tool call events (Messages / All) ────────────────────────────────
    /// A tool call started within an agent node's ReAct loop.
    #[serde(rename = "tool_call_started")]
    ToolCallStarted {
        /// Node UUID
        node_id: String,
        /// Human-readable node name
        node_name: String,
        /// Name of the tool being called
        tool_name: String,
        /// Unique identifier for this tool call
        tool_call_id: String,
        /// Tool call parameters (masked if guardrails are active)
        parameters: serde_json::Value,
    },

    /// A tool call completed successfully.
    #[serde(rename = "tool_call_completed")]
    ToolCallCompleted {
        /// Node UUID
        node_id: String,
        /// Human-readable node name
        node_name: String,
        /// Name of the tool that was called
        tool_name: String,
        /// Unique identifier for this tool call
        tool_call_id: String,
        /// Tool output
        output: serde_json::Value,
        /// Tool execution duration in milliseconds
        duration_ms: f64,
    },

    /// A tool call failed.
    ///
    /// This is an informational event — it does NOT cause the iterator to
    /// raise an exception. The node may retry or continue with other tools.
    #[serde(rename = "tool_call_failed")]
    ToolCallFailed {
        /// Node UUID
        node_id: String,
        /// Human-readable node name
        node_name: String,
        /// Name of the tool that failed
        tool_name: String,
        /// Unique identifier for this tool call
        tool_call_id: String,
        /// Error description
        error: String,
        /// Python exception type hint
        error_type: String,
    },
}

// ──────────────────────────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────────────────────────

/// Classify a `GraphBitError` into a Python-compatible error type string.
///
/// These strings map 1:1 with the exception types in
/// `python/src/errors.rs::to_py_error()`:
///
/// | Return value           | Python exception   |
/// |------------------------|--------------------|
/// | `"connection_error"`   | `ConnectionError`  |
/// | `"permission_error"`   | `PermissionError`  |
/// | `"value_error"`        | `ValueError`       |
/// | `"timeout_error"`      | `TimeoutError`     |
/// | `"runtime_error"`      | `RuntimeError`     |
pub fn error_type_from_graphbit_error(error: &GraphBitError) -> String {
    match error {
        GraphBitError::Network { .. } => "connection_error".to_string(),
        GraphBitError::Authentication { .. } => "permission_error".to_string(),
        GraphBitError::Validation { .. } | GraphBitError::Configuration { .. } => {
            "value_error".to_string()
        }
        GraphBitError::RateLimit { .. } => "runtime_error".to_string(),
        _ => {
            // Fallback: inspect the error message for common patterns
            let msg = error.to_string().to_lowercase();
            if msg.contains("timeout") || msg.contains("timed out") {
                "timeout_error".to_string()
            } else if msg.contains("connection") || msg.contains("network") {
                "connection_error".to_string()
            } else if msg.contains("permission") || msg.contains("unauthorized") {
                "permission_error".to_string()
            } else {
                "runtime_error".to_string()
            }
        }
    }
}

/// Classify a plain error string into a Python-compatible error type string.
///
/// Used when the original `GraphBitError` is not available (e.g., stringified
/// errors from external sources).
pub fn error_type_from_string(error_msg: &str) -> String {
    let msg = error_msg.to_lowercase();
    if msg.contains("timeout") || msg.contains("timed out") {
        "timeout_error".to_string()
    } else if msg.contains("connection") || msg.contains("network") {
        "connection_error".to_string()
    } else if msg.contains("permission") || msg.contains("unauthorized") || msg.contains("auth") {
        "permission_error".to_string()
    } else if msg.contains("invalid") || msg.contains("validation") {
        "value_error".to_string()
    } else {
        "runtime_error".to_string()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── StreamMode tests ─────────────────────────────────────────────────

    #[test]
    fn test_stream_mode_from_str() {
        assert_eq!(
            StreamMode::from_str_opt("updates"),
            Some(StreamMode::Updates)
        );
        assert_eq!(
            StreamMode::from_str_opt("messages"),
            Some(StreamMode::Messages)
        );
        assert_eq!(StreamMode::from_str_opt("all"), Some(StreamMode::All));
        assert_eq!(
            StreamMode::from_str_opt("UPDATES"),
            Some(StreamMode::Updates)
        );
        assert_eq!(
            StreamMode::from_str_opt("Messages"),
            Some(StreamMode::Messages)
        );
        assert_eq!(StreamMode::from_str_opt("invalid"), None);
        assert_eq!(StreamMode::from_str_opt(""), None);
    }

    #[test]
    fn test_stream_mode_display() {
        assert_eq!(StreamMode::Updates.to_string(), "updates");
        assert_eq!(StreamMode::Messages.to_string(), "messages");
        assert_eq!(StreamMode::All.to_string(), "all");
    }

    #[test]
    fn test_stream_mode_emits_tokens() {
        assert!(!StreamMode::Updates.emits_tokens());
        assert!(StreamMode::Messages.emits_tokens());
        assert!(StreamMode::All.emits_tokens());
    }

    #[test]
    fn test_stream_mode_emits_tool_events() {
        assert!(!StreamMode::Updates.emits_tool_events());
        assert!(StreamMode::Messages.emits_tool_events());
        assert!(StreamMode::All.emits_tool_events());
    }

    // ── StreamEvent construction tests ───────────────────────────────────

    #[test]
    fn test_workflow_started_event() {
        let event = StreamEvent::WorkflowStarted {
            workflow_id: "wf-123".to_string(),
            workflow_name: "test_workflow".to_string(),
            total_nodes: 3,
        };
        // Verify serialization produces the correct tag
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "workflow_started");
        assert_eq!(json["workflow_id"], "wf-123");
        assert_eq!(json["workflow_name"], "test_workflow");
        assert_eq!(json["total_nodes"], 3);
    }

    #[test]
    fn test_node_started_event() {
        let event = StreamEvent::NodeStarted {
            node_id: "node-1".to_string(),
            node_name: "researcher".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "node_started");
        assert_eq!(json["node_id"], "node-1");
        assert_eq!(json["node_name"], "researcher");
    }

    #[test]
    fn test_node_completed_event() {
        let event = StreamEvent::NodeCompleted {
            node_id: "node-1".to_string(),
            node_name: "researcher".to_string(),
            output: serde_json::json!("The research shows..."),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "node_completed");
        assert_eq!(json["output"], "The research shows...");
    }

    #[test]
    fn test_node_failed_event() {
        let event = StreamEvent::NodeFailed {
            node_id: "node-1".to_string(),
            node_name: "researcher".to_string(),
            error: "Connection refused".to_string(),
            error_type: "connection_error".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "node_failed");
        assert_eq!(json["error_type"], "connection_error");
    }

    #[test]
    fn test_workflow_failed_event() {
        let event = StreamEvent::WorkflowFailed {
            error: "Timed out after 120s".to_string(),
            error_type: "timeout_error".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "workflow_failed");
        assert_eq!(json["error_type"], "timeout_error");
    }

    #[test]
    fn test_token_event() {
        let event = StreamEvent::Token {
            node_id: "node-1".to_string(),
            node_name: "researcher".to_string(),
            content: "The".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "token");
        assert_eq!(json["content"], "The");
    }

    #[test]
    fn test_tool_call_started_event() {
        let event = StreamEvent::ToolCallStarted {
            node_id: "node-1".to_string(),
            node_name: "researcher".to_string(),
            tool_name: "web_search".to_string(),
            tool_call_id: "call_abc123".to_string(),
            parameters: serde_json::json!({"query": "latest AI research"}),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "tool_call_started");
        assert_eq!(json["tool_name"], "web_search");
        assert_eq!(json["parameters"]["query"], "latest AI research");
    }

    #[test]
    fn test_tool_call_completed_event() {
        let event = StreamEvent::ToolCallCompleted {
            node_id: "node-1".to_string(),
            node_name: "researcher".to_string(),
            tool_name: "web_search".to_string(),
            tool_call_id: "call_abc123".to_string(),
            output: serde_json::json!("Search results: ..."),
            duration_ms: 1200.5,
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "tool_call_completed");
        assert_eq!(json["duration_ms"], 1200.5);
    }

    #[test]
    fn test_tool_call_failed_event() {
        let event = StreamEvent::ToolCallFailed {
            node_id: "node-1".to_string(),
            node_name: "researcher".to_string(),
            tool_name: "web_search".to_string(),
            tool_call_id: "call_abc123".to_string(),
            error: "Timeout connecting to API".to_string(),
            error_type: "timeout_error".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["event"], "tool_call_failed");
        assert_eq!(json["error_type"], "timeout_error");
    }

    // ── Serialization roundtrip test ─────────────────────────────────────

    #[test]
    fn test_stream_event_roundtrip() {
        let events = vec![
            StreamEvent::WorkflowStarted {
                workflow_id: "wf-1".to_string(),
                workflow_name: "test".to_string(),
                total_nodes: 2,
            },
            StreamEvent::NodeStarted {
                node_id: "n1".to_string(),
                node_name: "step1".to_string(),
            },
            StreamEvent::Token {
                node_id: "n1".to_string(),
                node_name: "step1".to_string(),
                content: "Hello".to_string(),
            },
            StreamEvent::NodeCompleted {
                node_id: "n1".to_string(),
                node_name: "step1".to_string(),
                output: serde_json::json!("Hello world"),
            },
            StreamEvent::WorkflowFailed {
                error: "test error".to_string(),
                error_type: "runtime_error".to_string(),
            },
        ];

        for event in events {
            let json = serde_json::to_string(&event).unwrap();
            let deserialized: StreamEvent = serde_json::from_str(&json).unwrap();
            // Verify the round-trip produces valid JSON
            let json2 = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(json, json2);
        }
    }

    // ── Error type classification tests ──────────────────────────────────

    #[test]
    fn test_error_type_from_graphbit_error() {
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::Network {
                message: "conn refused".to_string()
            }),
            "connection_error"
        );
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::Authentication {
                provider: "openai".to_string(),
                message: "bad key".to_string()
            }),
            "permission_error"
        );
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::Validation {
                field: "model".to_string(),
                message: "invalid".to_string()
            }),
            "value_error"
        );
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::Configuration {
                message: "missing".to_string()
            }),
            "value_error"
        );
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::RateLimit {
                provider: "openai".to_string(),
                retry_after_seconds: 30
            }),
            "runtime_error"
        );
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::Internal {
                message: "unknown".to_string()
            }),
            "runtime_error"
        );
    }

    #[test]
    fn test_error_type_from_graphbit_error_fallback() {
        // Errors without a direct mapping use message-based heuristics
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::WorkflowExecution {
                message: "Workflow execution timed out".to_string()
            }),
            "timeout_error"
        );
        assert_eq!(
            error_type_from_graphbit_error(&GraphBitError::WorkflowExecution {
                message: "Connection lost during execution".to_string()
            }),
            "connection_error"
        );
    }

    #[test]
    fn test_error_type_from_string() {
        assert_eq!(
            error_type_from_string("Connection refused"),
            "connection_error"
        );
        assert_eq!(error_type_from_string("Request timed out"), "timeout_error");
        assert_eq!(
            error_type_from_string("Permission denied"),
            "permission_error"
        );
        assert_eq!(error_type_from_string("Invalid input"), "value_error");
        assert_eq!(
            error_type_from_string("Something went wrong"),
            "runtime_error"
        );
    }
}
