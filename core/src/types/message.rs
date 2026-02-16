//! Agent message and content types.

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::ids::AgentId;

/// Message structure for agent communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Unique message ID
    pub id: Uuid,
    /// ID of the sending agent
    pub sender: AgentId,
    /// ID of the receiving agent (None for broadcast)
    pub recipient: Option<AgentId>,
    /// Message content
    pub content: MessageContent,
    /// Message metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Timestamp when message was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AgentMessage {
    /// Create a new agent message
    pub fn new(sender: AgentId, recipient: Option<AgentId>, content: MessageContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Add metadata to the message
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl Default for AgentMessage {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            sender: AgentId::new(),
            recipient: None,
            content: MessageContent::Text("".to_string()),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Different types of message content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    /// Plain text message
    Text(String),
    /// Structured data message
    Data(serde_json::Value),
    /// Tool call request
    ToolCall {
        /// Name of the tool to call
        tool_name: String,
        /// Parameters to pass to the tool
        parameters: serde_json::Value,
    },
    /// Tool call response
    ToolResponse {
        /// Name of the tool that was called
        tool_name: String,
        /// Result returned by the tool
        result: serde_json::Value,
        /// Whether the tool call was successful
        success: bool,
    },
    /// Error message
    Error {
        /// Error code identifier
        error_code: String,
        /// Human-readable error message
        error_message: String,
    },
}
