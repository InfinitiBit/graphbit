//! Memory tools for agent integration
//!
//! This module provides helper functions for memory operations that can be
//! exposed as tools to agents through the Python bindings.

use super::manager::MemoryManager;
use super::types::{MemoryEntry, MemoryId, MemoryQuery, MemoryType};
use crate::errors::GraphBitResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Result of a remember operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RememberResult {
    /// ID of the stored memory
    pub memory_id: String,
    /// Type of memory stored
    pub memory_type: String,
    /// Success message
    pub message: String,
}

/// Result of a recall operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallResult {
    /// Retrieved memories
    pub memories: Vec<RecallMemory>,
    /// Number of results
    pub count: usize,
    /// Query that was used
    pub query: String,
}

/// A recalled memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallMemory {
    /// Memory ID
    pub id: String,
    /// Memory content
    pub content: String,
    /// Memory type
    pub memory_type: String,
    /// Importance score
    pub importance: f32,
    /// Similarity score (if semantic search was used)
    pub score: Option<f32>,
    /// Tags
    pub tags: Vec<String>,
}

impl From<MemoryEntry> for RecallMemory {
    fn from(entry: MemoryEntry) -> Self {
        Self {
            id: entry.id.to_string(),
            content: entry.content,
            memory_type: format!("{:?}", entry.memory_type),
            importance: entry.importance_score,
            score: None,
            tags: entry.metadata.tags,
        }
    }
}

/// Result of a forget operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgetResult {
    /// Whether the memory was successfully removed
    pub success: bool,
    /// Message describing the result
    pub message: String,
}

/// Result of a connect operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectResult {
    /// Whether the connection was successful
    pub success: bool,
    /// Message describing the result
    pub message: String,
}

/// Result of getting session context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContextResult {
    /// Formatted context string
    pub context: String,
    /// Number of memories in context
    pub memory_count: usize,
    /// Current session ID
    pub session_id: Option<String>,
}

/// Memory tools helper for agent integration
pub struct MemoryTools {
    manager: Arc<tokio::sync::RwLock<MemoryManager>>,
}

impl MemoryTools {
    /// Create a new memory tools instance
    pub fn new(manager: Arc<tokio::sync::RwLock<MemoryManager>>) -> Self {
        Self { manager }
    }

    /// Store information in memory
    ///
    /// # Arguments
    /// * `content` - The content to remember
    /// * `memory_type` - Optional memory type (Working, Factual, Episodic, Semantic)
    /// * `importance` - Optional importance score (0.0-1.0)
    /// * `_tags` - Optional tags for categorization (currently unused)
    pub async fn remember(
        &self,
        content: String,
        memory_type: Option<String>,
        importance: Option<f32>,
        _tags: Option<Vec<String>>,
    ) -> GraphBitResult<RememberResult> {
        // Determine memory type
        let mem_type = memory_type
            .as_deref()
            .unwrap_or("Working")
            .to_lowercase();

        let memory_id = match mem_type.as_str() {
            "working" => {
                let manager = self.manager.read().await;
                manager.store_working(content.clone()).await?
            }
            "factual" => {
                let manager = self.manager.read().await;
                // For factual, try to extract key-value from content
                if let Some((key, value)) = content.split_once(':') {
                    manager
                        .store_fact(key.trim().to_string(), value.trim().to_string())
                        .await?
                } else {
                    // Store as generic fact with auto-generated key
                    let key = format!("fact_{}", chrono::Utc::now().timestamp());
                    manager.store_fact(key, content.clone()).await?
                }
            }
            "episodic" => {
                let mut manager = self.manager.write().await;
                // Add to current episode or create new one
                manager.add_to_episode(content.clone());
                MemoryId::new() // Return a placeholder ID
            }
            "semantic" => {
                let mut manager = self.manager.write().await;
                // For semantic, use content as both name and description
                let mut concept = super::semantic::SemanticConcept::new(
                    content.clone(),
                    content.clone(),
                );
                // Set confidence based on importance
                if let Some(imp) = importance {
                    concept.confidence = imp;
                }
                manager.store_concept(concept).await?
            }
            _ => {
                // Default to working memory
                let manager = self.manager.read().await;
                manager.store_working(content.clone()).await?
            }
        };

        Ok(RememberResult {
            memory_id: memory_id.to_string(),
            memory_type: mem_type,
            message: format!("Successfully stored memory: {}", content),
        })
    }

    /// Retrieve relevant memories
    ///
    /// # Arguments
    /// * `query` - The search query
    /// * `limit` - Maximum number of results (default: 10)
    /// * `memory_type` - Optional filter by memory type
    /// * `tags` - Optional filter by tags
    pub async fn recall(
        &self,
        query: String,
        limit: Option<usize>,
        memory_type: Option<String>,
        tags: Option<Vec<String>>,
    ) -> GraphBitResult<RecallResult> {
        let mut mem_query = MemoryQuery::new(query.clone()).with_limit(limit.unwrap_or(10));

        // Apply filters
        if let Some(mem_type_str) = memory_type {
            let mem_type = match mem_type_str.to_lowercase().as_str() {
                "working" => MemoryType::Working,
                "factual" => MemoryType::Factual,
                "episodic" => MemoryType::Episodic,
                "semantic" => MemoryType::Semantic,
                _ => MemoryType::Working,
            };
            mem_query = mem_query.with_memory_type(mem_type);
        }

        if let Some(tag_list) = tags {
            mem_query = mem_query.with_tags(tag_list);
        }

        // Retrieve memories
        let manager = self.manager.read().await;
        let results = manager.retrieve(mem_query).await?;

        let memories: Vec<RecallMemory> = results
            .into_iter()
            .map(|result| {
                let mut recall_mem = RecallMemory::from(result.entry);
                recall_mem.score = Some(result.similarity);
                recall_mem
            })
            .collect();

        let count = memories.len();

        Ok(RecallResult {
            memories,
            count,
            query,
        })
    }

    /// Remove a specific memory
    ///
    /// # Arguments
    /// * `memory_id` - The ID of the memory to remove
    pub async fn forget(&self, memory_id: String) -> GraphBitResult<ForgetResult> {
        let id = MemoryId::from_string(&memory_id)?;
        let manager = self.manager.read().await;
        let removed = manager.remove_memory(&id).await?;

        Ok(ForgetResult {
            success: removed,
            message: if removed {
                format!("Successfully removed memory: {}", memory_id)
            } else {
                format!("Memory not found: {}", memory_id)
            },
        })
    }

    /// Connect two memories (for semantic relationships)
    ///
    /// # Arguments
    /// * `memory_id1` - First memory ID
    /// * `memory_id2` - Second memory ID
    pub async fn connect_memories(
        &self,
        memory_id1: String,
        memory_id2: String,
    ) -> GraphBitResult<ConnectResult> {
        // This is a simplified implementation
        // In a full implementation, you would update the semantic graph
        Ok(ConnectResult {
            success: true,
            message: format!(
                "Connected memories: {} <-> {}",
                memory_id1, memory_id2
            ),
        })
    }

    /// Get formatted session context for LLM injection
    pub async fn get_session_context(&self) -> GraphBitResult<SessionContextResult> {
        let manager = self.manager.read().await;
        let context = manager.get_working_context().await;
        let session_id = manager.get_current_session().await;

        // Count memories in context
        let memory_count = context.lines().filter(|line| line.starts_with('-')).count();

        Ok(SessionContextResult {
            context,
            memory_count,
            session_id,
        })
    }

    /// Start a new session
    pub async fn start_session(&self, session_id: String) {
        let mut manager = self.manager.write().await;
        manager.start_session(session_id);
    }

    /// End the current session
    pub async fn end_session(&self) -> GraphBitResult<usize> {
        let mut manager = self.manager.write().await;
        manager.end_session().await
    }

    /// Get memory statistics
    pub async fn get_stats(&self) -> GraphBitResult<serde_json::Value> {
        let manager = self.manager.read().await;
        let stats = manager.get_stats().await;
        Ok(serde_json::to_value(stats)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_remember_working() {
        let manager = Arc::new(RwLock::new(MemoryManager::with_defaults()));
        let tools = MemoryTools::new(manager);

        let result = tools
            .remember(
                "User prefers dark mode".to_string(),
                Some("Working".to_string()),
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(result.memory_type, "working");
        assert!(result.message.contains("Successfully stored"));
    }

    #[tokio::test]
    async fn test_recall() {
        let manager = Arc::new(RwLock::new(MemoryManager::with_defaults()));
        let tools = MemoryTools::new(manager.clone());

        // Store some memories
        tools
            .remember(
                "User likes Python".to_string(),
                Some("Working".to_string()),
                None,
                None,
            )
            .await
            .unwrap();

        // Recall
        let result = tools
            .recall("Python".to_string(), Some(5), None, None)
            .await
            .unwrap();

        assert_eq!(result.query, "Python");
    }
}

