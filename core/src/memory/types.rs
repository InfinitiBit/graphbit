//! Core data types for the memory layer.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use crate::embeddings::EmbeddingConfig;
use crate::llm::LlmConfig;

/// Unique identifier for memories, following the `AgentId` pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub Uuid);

impl MemoryId {
    /// Create a new random memory ID.
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse a memory ID from a string.
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the underlying UUID.
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for MemoryId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MemoryId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Scoping information for memory isolation.
///
/// Memories can be scoped to a specific user, agent, or run. All fields are
/// optional; omitted fields act as wildcards when filtering.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryScope {
    /// User-level scope.
    pub user_id: Option<String>,
    /// Agent-level scope.
    pub agent_id: Option<String>,
    /// Run-level scope.
    pub run_id: Option<String>,
}

/// A single stored memory fact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier.
    pub id: MemoryId,
    /// The fact / content of the memory.
    pub content: String,
    /// Scoping information.
    pub scope: MemoryScope,
    /// Arbitrary key-value metadata.
    pub metadata: HashMap<String, serde_json::Value>,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
    /// Content hash for deduplication.
    pub hash: String,
}

/// A memory paired with its similarity score from a search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredMemory {
    /// The memory.
    pub memory: Memory,
    /// Cosine similarity score (0.0 .. 1.0).
    pub score: f64,
}

/// A historical record of a memory mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHistory {
    /// Which memory was affected.
    pub memory_id: MemoryId,
    /// Content before the change (empty for `Add`).
    pub old_content: String,
    /// Content after the change (empty for `Delete`).
    pub new_content: String,
    /// What kind of mutation.
    pub action: MemoryAction,
    /// When the mutation occurred.
    pub timestamp: DateTime<Utc>,
}

/// Describes the kind of memory mutation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemoryAction {
    /// A brand-new fact was added.
    Add,
    /// An existing fact was updated / refined.
    Update,
    /// A fact was removed.
    Delete,
    /// No change required (duplicate / irrelevant).
    Noop,
}

impl fmt::Display for MemoryAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "ADD"),
            Self::Update => write!(f, "UPDATE"),
            Self::Delete => write!(f, "DELETE"),
            Self::Noop => write!(f, "NOOP"),
        }
    }
}

impl MemoryAction {
    /// Parse an action from its string representation.
    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ADD" => Self::Add,
            "UPDATE" => Self::Update,
            "DELETE" => Self::Delete,
            _ => Self::Noop,
        }
    }
}

/// An LLM decision about what to do with an extracted fact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDecision {
    /// The extracted fact text.
    pub fact: String,
    /// The decided action.
    pub action: MemoryAction,
    /// If updating or deleting, the target memory's ID (as a string).
    pub target_memory_id: Option<String>,
}

/// Configuration for the memory subsystem.
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// LLM provider configuration for fact extraction.
    pub llm_config: LlmConfig,
    /// Embedding provider configuration for vector search.
    pub embedding_config: EmbeddingConfig,
    /// Path to the SQLite database file (`:memory:` for in-memory).
    pub db_path: String,
    /// Minimum cosine-similarity threshold for search results (0.0 .. 1.0).
    pub similarity_threshold: f64,
    /// Maximum tokens for the extraction LLM call.
    pub max_extraction_tokens: u32,
    /// Temperature for the extraction LLM call.
    pub extraction_temperature: f32,
}

impl MemoryConfig {
    /// Create a new config with sensible defaults.
    pub fn new(llm_config: LlmConfig, embedding_config: EmbeddingConfig) -> Self {
        Self {
            llm_config,
            embedding_config,
            db_path: "graphbit_memory.db".to_string(),
            similarity_threshold: 0.7,
            max_extraction_tokens: 1500,
            extraction_temperature: 0.1,
        }
    }

    /// Override the database path.
    pub fn with_db_path(mut self, db_path: impl Into<String>) -> Self {
        self.db_path = db_path.into();
        self
    }

    /// Override the similarity threshold.
    pub fn with_similarity_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Override the max extraction tokens.
    pub fn with_max_extraction_tokens(mut self, tokens: u32) -> Self {
        self.max_extraction_tokens = tokens;
        self
    }

    /// Override the extraction temperature.
    pub fn with_extraction_temperature(mut self, temperature: f32) -> Self {
        self.extraction_temperature = temperature.clamp(0.0, 1.0);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embeddings::EmbeddingConfig;
    use crate::llm::LlmConfig;

    #[test]
    fn test_memory_id_creation() {
        let id1 = MemoryId::new();
        let id2 = MemoryId::new();
        assert_ne!(id1, id2, "Two fresh IDs should be unique");

        let id_str = id1.to_string();
        let parsed = MemoryId::from_string(&id_str).expect("should parse valid UUID");
        assert_eq!(id1, parsed);
    }

    #[test]
    fn test_memory_id_from_invalid_string() {
        let result = MemoryId::from_string("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_scope_default() {
        let scope = MemoryScope::default();
        assert!(scope.user_id.is_none());
        assert!(scope.agent_id.is_none());
        assert!(scope.run_id.is_none());
    }

    #[test]
    fn test_memory_action_display() {
        assert_eq!(MemoryAction::Add.to_string(), "ADD");
        assert_eq!(MemoryAction::Update.to_string(), "UPDATE");
        assert_eq!(MemoryAction::Delete.to_string(), "DELETE");
        assert_eq!(MemoryAction::Noop.to_string(), "NOOP");
    }

    #[test]
    fn test_memory_action_from_str_lossy() {
        assert_eq!(MemoryAction::from_str_lossy("ADD"), MemoryAction::Add);
        assert_eq!(MemoryAction::from_str_lossy("add"), MemoryAction::Add);
        assert_eq!(MemoryAction::from_str_lossy("UPDATE"), MemoryAction::Update);
        assert_eq!(MemoryAction::from_str_lossy("DELETE"), MemoryAction::Delete);
        assert_eq!(MemoryAction::from_str_lossy("unknown"), MemoryAction::Noop);
    }

    #[test]
    fn test_memory_config_builder() {
        let llm_config = LlmConfig::openai("test-key", "gpt-4o-mini");
        let embedding_config = EmbeddingConfig {
            provider: crate::embeddings::EmbeddingProvider::OpenAI,
            api_key: "test-key".to_string(),
            model: "text-embedding-3-small".to_string(),
            base_url: None,
            timeout_seconds: None,
            max_batch_size: None,
            extra_params: HashMap::new(),
            #[cfg(feature = "python")]
            python_instance: None,
        };

        let config = MemoryConfig::new(llm_config, embedding_config)
            .with_db_path(":memory:")
            .with_similarity_threshold(0.8)
            .with_max_extraction_tokens(2000)
            .with_extraction_temperature(0.2);

        assert_eq!(config.db_path, ":memory:");
        assert!((config.similarity_threshold - 0.8).abs() < f64::EPSILON);
        assert_eq!(config.max_extraction_tokens, 2000);
        assert!((config.extraction_temperature - 0.2).abs() < f32::EPSILON);
    }
}
