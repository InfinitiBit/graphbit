//! Episodic memory implementation for conversation history tracking
//!
//! Episodic memory stores records of specific past conversations and interactions
//! with timestamps, participants, and full context preservation.

use super::storage::MemoryStorage;
use super::types::{MemoryEntry, MemoryId, MemoryMetadata, MemoryType};
use crate::errors::GraphBitResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An episode representing a conversation or interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Unique episode ID
    pub id: String,
    /// Episode title or summary
    pub title: String,
    /// Participants in the episode
    pub participants: Vec<String>,
    /// When the episode occurred
    pub timestamp: DateTime<Utc>,
    /// Episode content/transcript
    pub content: String,
    /// Episode outcome or result
    pub outcome: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Episode {
    /// Create a new episode
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            participants: Vec::with_capacity(2),
            timestamp: Utc::now(),
            content,
            outcome: None,
            tags: Vec::with_capacity(4),
        }
    }

    /// Add a participant
    pub fn add_participant(&mut self, participant: String) {
        if !self.participants.contains(&participant) {
            self.participants.push(participant);
        }
    }

    /// Set the outcome
    pub fn set_outcome(&mut self, outcome: String) {
        self.outcome = Some(outcome);
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

/// Episodic memory manager for conversation history
#[derive(Debug)]
pub struct EpisodicMemory {
    /// Current episode being recorded (if any)
    current_episode: Option<Episode>,
}

impl EpisodicMemory {
    /// Create a new episodic memory instance
    pub fn new() -> Self {
        Self {
            current_episode: None,
        }
    }

    /// Start recording a new episode
    pub fn start_episode(&mut self, title: String) {
        self.current_episode = Some(Episode::new(title, String::new()));
    }

    /// Add content to the current episode
    pub fn add_to_episode(&mut self, content: String) {
        if let Some(ref mut episode) = self.current_episode {
            if !episode.content.is_empty() {
                episode.content.push_str("\n");
            }
            episode.content.push_str(&content);
        }
    }

    /// Add a participant to the current episode
    pub fn add_participant(&mut self, participant: String) {
        if let Some(ref mut episode) = self.current_episode {
            episode.add_participant(participant);
        }
    }

    /// Set the outcome of the current episode
    pub fn set_outcome(&mut self, outcome: String) {
        if let Some(ref mut episode) = self.current_episode {
            episode.set_outcome(outcome);
        }
    }

    /// Add a tag to the current episode
    pub fn add_tag(&mut self, tag: String) {
        if let Some(ref mut episode) = self.current_episode {
            episode.add_tag(tag);
        }
    }

    /// End the current episode and store it
    pub fn end_episode(
        &mut self,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<Option<MemoryId>> {
        if let Some(episode) = self.current_episode.take() {
            let memory_id = self.store_episode(episode, storage)?;
            Ok(Some(memory_id))
        } else {
            Ok(None)
        }
    }

    /// Store an episode as a memory
    pub fn store_episode(
        &self,
        episode: Episode,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<MemoryId> {
        let mut metadata = MemoryMetadata::new();
        metadata.set_source("episodic".to_string());
        metadata.add_tag("episode".to_string());

        for tag in &episode.tags {
            metadata.add_tag(tag.clone());
        }

        // Add participants as metadata
        for participant in &episode.participants {
            metadata.add_custom(
                format!("participant_{}", participant),
                serde_json::json!(true),
            );
        }

        // Add timestamp
        metadata.add_custom(
            "timestamp".to_string(),
            serde_json::json!(episode.timestamp.to_rfc3339()),
        );

        // Add outcome if present
        if let Some(ref outcome) = episode.outcome {
            metadata.add_custom("outcome".to_string(), serde_json::json!(outcome));
        }

        // Format content with title and details
        let content = format!(
            "Episode: {}\nTime: {}\nParticipants: {}\n\n{}{}",
            episode.title,
            episode.timestamp.format("%Y-%m-%d %H:%M:%S"),
            episode.participants.join(", "),
            episode.content,
            episode
                .outcome
                .as_ref()
                .map(|o| format!("\n\nOutcome: {}", o))
                .unwrap_or_default()
        );

        let mut entry = MemoryEntry::with_importance(
            content,
            MemoryType::Episodic,
            0.7, // Episodes are moderately important
            None,
        );
        entry.metadata = metadata;

        let id = entry.id.clone();
        storage.store(entry)?;
        Ok(id)
    }

    /// Retrieve episodes by participant
    pub fn get_episodes_by_participant(
        &self,
        participant: &str,
        storage: &dyn MemoryStorage,
    ) -> Vec<MemoryEntry> {
        let episodes = storage.list_by_type(MemoryType::Episodic);
        let participant_key = format!("participant_{}", participant);

        episodes
            .into_iter()
            .filter(|ep| ep.metadata.custom.contains_key(&participant_key))
            .cloned()
            .collect()
    }

    /// Retrieve episodes by tag
    pub fn get_episodes_by_tag(&self, tag: &str, storage: &dyn MemoryStorage) -> Vec<MemoryEntry> {
        let episodes = storage.list_by_type(MemoryType::Episodic);

        episodes
            .into_iter()
            .filter(|ep| ep.metadata.tags.contains(&tag.to_string()))
            .cloned()
            .collect()
    }

    /// Retrieve episodes within a time range
    pub fn get_episodes_by_timerange(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        storage: &dyn MemoryStorage,
    ) -> Vec<MemoryEntry> {
        let episodes = storage.list_by_type(MemoryType::Episodic);

        episodes
            .into_iter()
            .filter(|ep| ep.created_at >= start && ep.created_at <= end)
            .cloned()
            .collect()
    }

    /// Get recent episodes (last N)
    pub fn get_recent_episodes(
        &self,
        limit: usize,
        storage: &dyn MemoryStorage,
    ) -> Vec<MemoryEntry> {
        let mut episodes: Vec<MemoryEntry> = storage
            .list_by_type(MemoryType::Episodic)
            .into_iter()
            .cloned()
            .collect();

        // Sort by creation time (most recent first)
        episodes.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        episodes.truncate(limit);
        episodes
    }

    /// Get episode summary (title and timestamp)
    pub fn get_episode_summary(&self, episode: &MemoryEntry) -> String {
        // Extract title from content (first line after "Episode: ")
        let lines: Vec<&str> = episode.content.lines().collect();
        let title = lines
            .first()
            .and_then(|line| line.strip_prefix("Episode: "))
            .unwrap_or("Untitled Episode");

        format!(
            "{} ({})",
            title,
            episode.created_at.format("%Y-%m-%d %H:%M")
        )
    }

    /// Count episodes
    pub fn count_episodes(&self, storage: &dyn MemoryStorage) -> usize {
        storage.count_by_type(MemoryType::Episodic)
    }

    /// Check if currently recording an episode
    pub fn is_recording(&self) -> bool {
        self.current_episode.is_some()
    }

    /// Get current episode (if recording)
    pub fn get_current_episode(&self) -> Option<&Episode> {
        self.current_episode.as_ref()
    }
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::storage::InMemoryStorage;

    #[test]
    fn test_episode_creation() {
        let mut episode = Episode::new("Test Episode".to_string(), "Test content".to_string());

        episode.add_participant("Alice".to_string());
        episode.add_participant("Bob".to_string());
        episode.set_outcome("Success".to_string());
        episode.add_tag("test".to_string());

        assert_eq!(episode.participants.len(), 2);
        assert_eq!(episode.outcome, Some("Success".to_string()));
        assert_eq!(episode.tags.len(), 1);
    }

    #[test]
    fn test_episodic_memory_recording() {
        let mut episodic = EpisodicMemory::new();
        let mut storage = InMemoryStorage::new();

        assert!(!episodic.is_recording());

        episodic.start_episode("Test Conversation".to_string());
        assert!(episodic.is_recording());

        episodic.add_to_episode("User: Hello".to_string());
        episodic.add_to_episode("Agent: Hi there!".to_string());
        episodic.add_participant("User".to_string());
        episodic.add_participant("Agent".to_string());

        let memory_id = episodic.end_episode(&mut storage).unwrap();
        assert!(memory_id.is_some());
        assert!(!episodic.is_recording());

        assert_eq!(episodic.count_episodes(&storage), 1);
    }

    #[test]
    fn test_episode_retrieval() {
        let mut episodic = EpisodicMemory::new();
        let mut storage = InMemoryStorage::new();

        // Create and store an episode
        let mut episode = Episode::new("Test".to_string(), "Content".to_string());
        episode.add_participant("Alice".to_string());
        episode.add_tag("important".to_string());
        episodic.store_episode(episode, &mut storage).unwrap();

        // Retrieve by participant
        let episodes = episodic.get_episodes_by_participant("Alice", &storage);
        assert_eq!(episodes.len(), 1);

        // Retrieve by tag
        let episodes = episodic.get_episodes_by_tag("important", &storage);
        assert_eq!(episodes.len(), 1);

        // Get recent episodes
        let recent = episodic.get_recent_episodes(10, &storage);
        assert_eq!(recent.len(), 1);
    }
}
