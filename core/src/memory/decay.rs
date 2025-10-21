//! Memory decay and filtering mechanisms
//!
//! This module implements time-based and importance-based memory decay
//! to prevent memory bloat and maintain only relevant information.

use super::storage::MemoryStorage;
use super::types::{MemoryId, MemoryType};
use crate::errors::GraphBitResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for memory decay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayConfig {
    /// Enable decay mechanism
    pub enabled: bool,
    /// Minimum decay score threshold (memories below this are forgotten)
    pub threshold: f32,
    /// Decay check interval in seconds
    pub check_interval_seconds: u64,
    /// Per-type decay thresholds (overrides global threshold)
    pub type_thresholds: HashMap<MemoryType, f32>,
    /// Protect memories accessed within this many seconds
    pub recent_access_protection_seconds: u64,
    /// Minimum importance score to protect from decay
    pub importance_protection_threshold: f32,
}

impl DecayConfig {
    /// Create a new decay configuration with defaults
    pub fn new() -> Self {
        Self {
            enabled: true,
            threshold: 0.3,
            check_interval_seconds: 3600, // 1 hour
            type_thresholds: HashMap::with_capacity(4),
            recent_access_protection_seconds: 86400, // 24 hours
            importance_protection_threshold: 0.8,
        }
    }

    /// Create a conservative decay config (keeps more memories)
    pub fn conservative() -> Self {
        Self {
            enabled: true,
            threshold: 0.1,
            check_interval_seconds: 7200, // 2 hours
            type_thresholds: HashMap::with_capacity(4),
            recent_access_protection_seconds: 172800, // 48 hours
            importance_protection_threshold: 0.7,
        }
    }

    /// Create an aggressive decay config (forgets more aggressively)
    pub fn aggressive() -> Self {
        Self {
            enabled: true,
            threshold: 0.5,
            check_interval_seconds: 1800, // 30 minutes
            type_thresholds: HashMap::with_capacity(4),
            recent_access_protection_seconds: 43200, // 12 hours
            importance_protection_threshold: 0.9,
        }
    }

    /// Disable decay
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            threshold: 0.0,
            check_interval_seconds: 0,
            type_thresholds: HashMap::new(),
            recent_access_protection_seconds: 0,
            importance_protection_threshold: 1.0,
        }
    }

    /// Set threshold for a specific memory type
    pub fn set_type_threshold(&mut self, memory_type: MemoryType, threshold: f32) {
        self.type_thresholds
            .insert(memory_type, threshold.clamp(0.0, 1.0));
    }

    /// Get threshold for a specific memory type
    pub fn get_threshold(&self, memory_type: MemoryType) -> f32 {
        self.type_thresholds
            .get(&memory_type)
            .copied()
            .unwrap_or(self.threshold)
    }
}

impl Default for DecayConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory decay manager
#[derive(Debug)]
pub struct DecayManager {
    /// Decay configuration
    config: DecayConfig,
    /// Last decay check timestamp
    last_check: DateTime<Utc>,
}

impl DecayManager {
    /// Create a new decay manager
    pub fn new(config: DecayConfig) -> Self {
        Self {
            config,
            last_check: Utc::now(),
        }
    }

    /// Check if decay should run based on interval
    pub fn should_run_decay(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        let elapsed = (Utc::now() - self.last_check).num_seconds() as u64;
        elapsed >= self.config.check_interval_seconds
    }

    /// Run decay process on storage
    pub fn run_decay(&mut self, storage: &mut dyn MemoryStorage) -> GraphBitResult<DecayStats> {
        if !self.config.enabled {
            return Ok(DecayStats::default());
        }

        let now = Utc::now();
        let mut stats = DecayStats::new();

        // Get all memories
        let all_memories = storage.list_all();
        let mut to_delete = Vec::with_capacity(all_memories.len() / 10); // Estimate 10% deletion

        for entry in all_memories {
            stats.total_checked += 1;

            // Check if memory is protected
            if self.is_protected(entry, now) {
                stats.protected += 1;
                continue;
            }

            // Get threshold for this memory type
            let threshold = self.config.get_threshold(entry.memory_type);

            // Check if memory should be forgotten
            if entry.should_forget(threshold, now) {
                to_delete.push(entry.id.clone());
                stats.forgotten += 1;
                stats
                    .forgotten_by_type
                    .entry(entry.memory_type)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            } else {
                stats.retained += 1;
            }
        }

        // Delete memories marked for removal
        for id in to_delete {
            storage.delete(&id)?;
        }

        // Update last check time
        self.last_check = now;
        stats.execution_time_ms = (Utc::now() - now).num_milliseconds() as u64;

        Ok(stats)
    }

    /// Check if a memory is protected from decay
    fn is_protected(&self, entry: &super::types::MemoryEntry, now: DateTime<Utc>) -> bool {
        // Protect high-importance memories
        if entry.importance_score >= self.config.importance_protection_threshold {
            return true;
        }

        // Protect recently accessed memories
        let seconds_since_access = (now - entry.last_accessed).num_seconds() as u64;
        if seconds_since_access < self.config.recent_access_protection_seconds {
            return true;
        }

        false
    }

    /// Force decay run regardless of interval
    pub fn force_decay(&mut self, storage: &mut dyn MemoryStorage) -> GraphBitResult<DecayStats> {
        let original_enabled = self.config.enabled;
        self.config.enabled = true;
        let stats = self.run_decay(storage)?;
        self.config.enabled = original_enabled;
        Ok(stats)
    }

    /// Update decay configuration
    pub fn update_config(&mut self, config: DecayConfig) {
        self.config = config;
    }

    /// Get current decay configuration
    pub fn get_config(&self) -> &DecayConfig {
        &self.config
    }

    /// Manually forget specific memories by ID
    pub fn forget_memories(
        &self,
        storage: &mut dyn MemoryStorage,
        memory_ids: Vec<MemoryId>,
    ) -> GraphBitResult<usize> {
        let mut forgotten = 0;
        for id in memory_ids {
            if storage.delete(&id)? {
                forgotten += 1;
            }
        }
        Ok(forgotten)
    }

    /// Forget all memories of a specific type
    pub fn forget_type(
        &self,
        storage: &mut dyn MemoryStorage,
        memory_type: MemoryType,
    ) -> GraphBitResult<()> {
        storage.clear_type(memory_type);
        Ok(())
    }

    /// Forget all memories in a session
    pub fn forget_session(
        &self,
        storage: &mut dyn MemoryStorage,
        session_id: &str,
    ) -> GraphBitResult<()> {
        storage.clear_session(session_id);
        Ok(())
    }
}

/// Statistics from a decay run
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecayStats {
    /// Total memories checked
    pub total_checked: usize,
    /// Memories forgotten
    pub forgotten: usize,
    /// Memories retained
    pub retained: usize,
    /// Memories protected from decay
    pub protected: usize,
    /// Memories forgotten by type
    pub forgotten_by_type: HashMap<MemoryType, usize>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl DecayStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self {
            total_checked: 0,
            forgotten: 0,
            retained: 0,
            protected: 0,
            forgotten_by_type: HashMap::with_capacity(4),
            execution_time_ms: 0,
        }
    }

    /// Get retention rate (0.0-1.0)
    pub fn retention_rate(&self) -> f32 {
        if self.total_checked == 0 {
            return 0.0;
        }
        self.retained as f32 / self.total_checked as f32
    }

    /// Get forgetting rate (0.0-1.0)
    pub fn forgetting_rate(&self) -> f32 {
        if self.total_checked == 0 {
            return 0.0;
        }
        self.forgotten as f32 / self.total_checked as f32
    }

    /// Get protection rate (0.0-1.0)
    pub fn protection_rate(&self) -> f32 {
        if self.total_checked == 0 {
            return 0.0;
        }
        self.protected as f32 / self.total_checked as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_config_defaults() {
        let config = DecayConfig::new();
        assert!(config.enabled);
        assert_eq!(config.threshold, 0.3);
        assert_eq!(config.check_interval_seconds, 3600);
    }

    #[test]
    fn test_decay_config_presets() {
        let conservative = DecayConfig::conservative();
        assert_eq!(conservative.threshold, 0.1);

        let aggressive = DecayConfig::aggressive();
        assert_eq!(aggressive.threshold, 0.5);

        let disabled = DecayConfig::disabled();
        assert!(!disabled.enabled);
    }

    #[test]
    fn test_decay_stats() {
        let mut stats = DecayStats::new();
        stats.total_checked = 100;
        stats.forgotten = 30;
        stats.retained = 60;
        stats.protected = 10;

        assert!((stats.retention_rate() - 0.6).abs() < 0.001);
        assert!((stats.forgetting_rate() - 0.3).abs() < 0.001);
        assert!((stats.protection_rate() - 0.1).abs() < 0.001);
    }
}
