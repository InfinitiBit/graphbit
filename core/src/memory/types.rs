//! Core memory types for the GraphBit memory system
//!
//! This module defines the fundamental types used across all memory implementations,
//! including memory entries, identifiers, metadata, and configuration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for memory entries
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub Uuid);

impl MemoryId {
    /// Create a new random memory ID
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a memory ID from a string UUID
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the underlying UUID
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

impl std::fmt::Display for MemoryId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Types of memory in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryType {
    /// Short-term session-based memory for current conversation context
    Working,
    /// Long-term structured knowledge (facts, preferences, settings)
    Factual,
    /// Records of specific past conversations and interactions
    Episodic,
    /// General knowledge built from patterns and insights over time
    Semantic,
}

impl MemoryType {
    /// Get all memory types
    pub fn all() -> Vec<MemoryType> {
        vec![
            MemoryType::Working,
            MemoryType::Factual,
            MemoryType::Episodic,
            MemoryType::Semantic,
        ]
    }

    /// Get the default capacity for this memory type
    pub fn default_capacity(&self) -> usize {
        match self {
            MemoryType::Working => 100,
            MemoryType::Factual => 1000,
            MemoryType::Episodic => 500,
            MemoryType::Semantic => 200,
        }
    }
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryType::Working => write!(f, "working"),
            MemoryType::Factual => write!(f, "factual"),
            MemoryType::Episodic => write!(f, "episodic"),
            MemoryType::Semantic => write!(f, "semantic"),
        }
    }
}

/// A single memory entry in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique identifier for this memory
    pub id: MemoryId,
    /// The actual content of the memory
    pub content: String,
    /// Type of memory
    pub memory_type: MemoryType,
    /// Vector embedding for semantic search (optional)
    pub embedding: Option<Vec<f32>>,
    /// Additional metadata
    pub metadata: MemoryMetadata,
    /// When this memory was created
    pub created_at: DateTime<Utc>,
    /// When this memory was last accessed
    pub last_accessed: DateTime<Utc>,
    /// Number of times this memory has been accessed
    pub access_count: u32,
    /// Importance score (0.0-1.0) for decay calculations
    pub importance_score: f32,
    /// Session ID for working memory (optional)
    pub session_id: Option<String>,
    /// IDs of related memories for graph connections
    pub related_memories: Vec<MemoryId>,
}

impl MemoryEntry {
    /// Create a new memory entry
    pub fn new(content: String, memory_type: MemoryType, session_id: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: MemoryId::new(),
            content,
            memory_type,
            embedding: None,
            metadata: MemoryMetadata::new(),
            created_at: now,
            last_accessed: now,
            access_count: 0,
            importance_score: 0.5, // Default medium importance
            session_id,
            related_memories: Vec::with_capacity(4), // Pre-allocate for typical connections
        }
    }

    /// Create a new memory entry with custom importance
    pub fn with_importance(
        content: String,
        memory_type: MemoryType,
        importance: f32,
        session_id: Option<String>,
    ) -> Self {
        let mut entry = Self::new(content, memory_type, session_id);
        entry.importance_score = importance.clamp(0.0, 1.0);
        entry
    }

    /// Record an access to this memory
    pub fn record_access(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count = self.access_count.saturating_add(1);

        // Boost importance slightly on access (up to 0.1 increase)
        let boost = 0.01 * (1.0 - self.importance_score);
        self.importance_score = (self.importance_score + boost).min(1.0);
    }

    /// Add a related memory connection
    pub fn add_relation(&mut self, memory_id: MemoryId) {
        if !self.related_memories.contains(&memory_id) {
            self.related_memories.push(memory_id);
        }
    }

    /// Remove a related memory connection
    pub fn remove_relation(&mut self, memory_id: &MemoryId) {
        self.related_memories.retain(|id| id != memory_id);
    }

    /// Calculate decay factor based on age and access patterns
    pub fn calculate_decay(&self, now: DateTime<Utc>) -> f32 {
        let age_seconds = (now - self.created_at).num_seconds() as f32;
        let recency_seconds = (now - self.last_accessed).num_seconds() as f32;

        // Decay factors
        let age_decay = (-age_seconds / 86400.0 / 30.0).exp(); // 30-day half-life
        let recency_decay = (-recency_seconds / 86400.0 / 7.0).exp(); // 7-day recency half-life
        let access_boost = (self.access_count as f32).ln().max(0.0) / 10.0;

        // Combined decay score
        let decay_score =
            (age_decay * 0.3 + recency_decay * 0.5 + access_boost * 0.2) * self.importance_score;

        decay_score.clamp(0.0, 1.0)
    }

    /// Check if this memory should be forgotten based on decay threshold
    pub fn should_forget(&self, threshold: f32, now: DateTime<Utc>) -> bool {
        self.calculate_decay(now) < threshold
    }
}

/// Metadata associated with a memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Source of the memory (e.g., "conversation", "user_input", "system")
    pub source: String,
    /// Additional custom fields
    pub custom: HashMap<String, serde_json::Value>,
    /// Image data for multimodal memories (base64 encoded)
    pub image_data: Option<String>,
    /// Image description for multimodal memories
    pub image_description: Option<String>,
}

impl MemoryMetadata {
    /// Create new empty metadata
    pub fn new() -> Self {
        Self {
            tags: Vec::with_capacity(4),
            source: String::from("unknown"),
            custom: HashMap::with_capacity(4),
            image_data: None,
            image_description: None,
        }
    }

    /// Create metadata with tags
    pub fn with_tags(tags: Vec<String>) -> Self {
        Self {
            tags,
            source: String::from("unknown"),
            custom: HashMap::with_capacity(4),
            image_data: None,
            image_description: None,
        }
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Set the source
    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    /// Add custom field
    pub fn add_custom(&mut self, key: String, value: serde_json::Value) {
        self.custom.insert(key, value);
    }

    /// Set image data for multimodal memory
    pub fn set_image(&mut self, data: String, description: Option<String>) {
        self.image_data = Some(data);
        self.image_description = description;
    }
}

impl Default for MemoryMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory query for retrieval operations
#[derive(Debug, Clone)]
pub struct MemoryQuery {
    /// Query text for semantic search
    pub query: String,
    /// Memory types to search (None = all types)
    pub memory_types: Option<Vec<MemoryType>>,
    /// Maximum number of results
    pub limit: usize,
    /// Minimum similarity threshold (0.0-1.0)
    pub min_similarity: f32,
    /// Filter by session ID
    pub session_id: Option<String>,
    /// Filter by tags
    pub tags: Option<Vec<String>>,
    /// Include related memories in results
    pub include_related: bool,
}

impl MemoryQuery {
    /// Create a new memory query
    pub fn new(query: String) -> Self {
        Self {
            query,
            memory_types: None,
            limit: 10,
            min_similarity: 0.5,
            session_id: None,
            tags: None,
            include_related: false,
        }
    }

    /// Set memory types filter
    pub fn with_types(mut self, types: Vec<MemoryType>) -> Self {
        self.memory_types = Some(types);
        self
    }

    /// Set a single memory type filter
    pub fn with_memory_type(mut self, memory_type: MemoryType) -> Self {
        self.memory_types = Some(vec![memory_type]);
        self
    }

    /// Set session ID filter (alias for with_session)
    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set minimum similarity threshold
    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set session filter
    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Set tag filter
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Include related memories
    pub fn with_related(mut self) -> Self {
        self.include_related = true;
        self
    }
}
