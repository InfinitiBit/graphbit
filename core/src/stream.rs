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
