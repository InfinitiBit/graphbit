//! LLM response types and utilities

use crate::llm::LlmToolCall;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response from an LLM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// The generated content
    pub content: String,
    /// Tool calls made by the LLM
    pub tool_calls: Vec<LlmToolCall>,
    /// Usage statistics
    pub usage: LlmUsage,
    /// Response metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Finish reason
    pub finish_reason: FinishReason,
    /// Model used for generation
    pub model: String,
    /// Response ID (if provided by the API)
    pub id: Option<String>,
}

impl LlmResponse {
    /// Create a new LLM response
    pub fn new(content: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tool_calls: Vec::new(),
            usage: LlmUsage::default(),
            metadata: HashMap::with_capacity(4), // Pre-allocate for common metadata fields
            finish_reason: FinishReason::Stop,
            model: model.into(),
            id: None,
        }
    }

    /// Add tool calls to the response
    pub fn with_tool_calls(mut self, tool_calls: Vec<LlmToolCall>) -> Self {
        self.tool_calls = tool_calls;
        self
    }

    /// Set usage statistics
    pub fn with_usage(mut self, usage: LlmUsage) -> Self {
        self.usage = usage;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set finish reason
    pub fn with_finish_reason(mut self, finish_reason: FinishReason) -> Self {
        self.finish_reason = finish_reason;
        self
    }

    /// Set response ID
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Check if the response contains tool calls
    pub fn has_tool_calls(&self) -> bool {
        !self.tool_calls.is_empty()
    }

    /// Check if the response was truncated due to length
    pub fn is_truncated(&self) -> bool {
        matches!(self.finish_reason, FinishReason::Length)
    }

    /// Get the total token count
    pub fn total_tokens(&self) -> u32 {
        self.usage.total_tokens
    }

    /// Estimate the cost of this response (if cost per token is known)
    pub fn estimate_cost(&self, input_cost_per_token: f64, output_cost_per_token: f64) -> f64 {
        (self.usage.prompt_tokens as f64 * input_cost_per_token)
            + (self.usage.completion_tokens as f64 * output_cost_per_token)
    }
}

/// Token usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
}

impl LlmUsage {
    /// Create new usage statistics
    pub fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }

    /// Create empty usage statistics
    pub fn empty() -> Self {
        Self {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        }
    }

    /// Add usage statistics
    pub fn add(&mut self, other: &LlmUsage) {
        self.prompt_tokens += other.prompt_tokens;
        self.completion_tokens += other.completion_tokens;
        self.total_tokens += other.total_tokens;
    }
}

impl Default for LlmUsage {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::ops::Add for LlmUsage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            prompt_tokens: self.prompt_tokens + other.prompt_tokens,
            completion_tokens: self.completion_tokens + other.completion_tokens,
            total_tokens: self.total_tokens + other.total_tokens,
        }
    }
}

impl std::ops::AddAssign for LlmUsage {
    fn add_assign(&mut self, other: Self) {
        self.prompt_tokens += other.prompt_tokens;
        self.completion_tokens += other.completion_tokens;
        self.total_tokens += other.total_tokens;
    }
}

/// Reason why the LLM stopped generating
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural stopping point
    Stop,
    /// Reached maximum length
    Length,
    /// Tool/function call
    ToolCalls,
    /// Content filter triggered
    ContentFilter,
    /// Model error
    Error,
    /// Other reason
    Other(String),
}

impl FinishReason {
    /// Check if the response finished naturally
    pub fn is_natural_stop(&self) -> bool {
        matches!(self, FinishReason::Stop | FinishReason::ToolCalls)
    }

    /// Check if the response was truncated
    pub fn is_truncated(&self) -> bool {
        matches!(self, FinishReason::Length)
    }

    /// Check if there was an error
    pub fn is_error(&self) -> bool {
        matches!(self, FinishReason::Error | FinishReason::ContentFilter)
    }
}

impl std::fmt::Display for FinishReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinishReason::Stop => write!(f, "stop"),
            FinishReason::Length => write!(f, "length"),
            FinishReason::ToolCalls => write!(f, "tool_calls"),
            FinishReason::ContentFilter => write!(f, "content_filter"),
            FinishReason::Error => write!(f, "error"),
            FinishReason::Other(reason) => write!(f, "{reason}"),
        }
    }
}
