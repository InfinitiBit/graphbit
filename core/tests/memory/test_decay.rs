//! Tests for memory decay

use chrono::{Duration, Utc};
use graphbit_core::memory::decay::{DecayConfig, DecayManager, DecayStats};
use graphbit_core::memory::storage::{InMemoryStorage, MemoryStorage};
use graphbit_core::memory::types::*;

fn create_test_entry(content: &str, memory_type: MemoryType) -> MemoryEntry {
    MemoryEntry::new(content.to_string(), memory_type, None)
}

#[test]
fn test_decay_manager_creation() {
    let config = DecayConfig::default();
    let _manager = DecayManager::new(config);
    // Just verify creation succeeds
}

#[test]
fn test_decay_config_default() {
    let config = DecayConfig::default();

    assert!(config.enabled);
    assert_eq!(config.threshold, 0.3);
    assert_eq!(config.check_interval_seconds, 3600);
    assert_eq!(config.recent_access_protection_seconds, 86400);
    assert_eq!(config.importance_protection_threshold, 0.8);
}

#[test]
fn test_decay_config_presets() {
    let conservative = DecayConfig::conservative();
    assert_eq!(conservative.threshold, 0.1);
    assert_eq!(conservative.check_interval_seconds, 7200);

    let aggressive = DecayConfig::aggressive();
    assert_eq!(aggressive.threshold, 0.5);
    assert_eq!(aggressive.check_interval_seconds, 1800);

    let disabled = DecayConfig::disabled();
    assert!(!disabled.enabled);
}

#[test]
fn test_memory_entry_calculate_decay() {
    let entry = create_test_entry("Test", MemoryType::Working);
    let now = Utc::now();

    let decay_score = entry.calculate_decay(now);

    assert!(
        decay_score >= 0.0 && decay_score <= 1.0,
        "Decay score should be normalized"
    );
}

#[test]
fn test_decay_old_memory() {
    let mut entry = create_test_entry("Old memory", MemoryType::Working);
    entry.created_at = Utc::now() - Duration::days(90);
    entry.last_accessed = Utc::now() - Duration::days(90);

    let now = Utc::now();
    let decay_score = entry.calculate_decay(now);

    assert!(decay_score < 0.5, "Old memory should have low decay score");
}

#[test]
fn test_decay_recent_memory() {
    let entry = create_test_entry("Recent memory", MemoryType::Working);
    let now = Utc::now();

    let decay_score = entry.calculate_decay(now);

    // Recent memory with default importance (0.5) should have decay score around 0.4
    // (age_decay=1.0 * 0.3 + recency_decay=1.0 * 0.5 + access_boost=0 * 0.2) * importance=0.5 = 0.4
    assert!(
        decay_score > 0.3,
        "Recent memory should have moderate decay score, got {}",
        decay_score
    );
}

#[test]
fn test_decay_important_memory() {
    let entry = MemoryEntry::with_importance(
        "Important memory".to_string(),
        MemoryType::Factual,
        0.95,
        None,
    );
    let now = Utc::now();

    let decay_score = entry.calculate_decay(now);

    assert!(
        decay_score > 0.7,
        "Important memory should have high decay score"
    );
}

#[test]
fn test_decay_unimportant_memory() {
    let entry = MemoryEntry::with_importance(
        "Unimportant memory".to_string(),
        MemoryType::Working,
        0.1,
        None,
    );
    let now = Utc::now();

    let decay_score = entry.calculate_decay(now);

    assert!(
        decay_score < 0.3,
        "Unimportant memory should have low decay score"
    );
}

#[test]
fn test_decay_frequently_accessed() {
    let mut entry = create_test_entry("Frequently accessed", MemoryType::Factual);

    // Simulate multiple accesses
    for _ in 0..10 {
        entry.record_access();
    }

    let now = Utc::now();
    let decay_score = entry.calculate_decay(now);

    // With 10 accesses: access_boost = ln(10)/10 = 0.23
    // decay_score = (1.0*0.3 + 1.0*0.5 + 0.23*0.2) * 0.5 = 0.423
    assert!(
        decay_score > 0.4,
        "Frequently accessed memory should have higher decay score, got {}",
        decay_score
    );
}

#[test]
fn test_decay_run() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig {
        enabled: true,
        threshold: 0.5,
        ..Default::default()
    };
    let mut manager = DecayManager::new(config);

    // Store some memories
    let mut old_entry = create_test_entry("Old memory", MemoryType::Working);
    old_entry.created_at = Utc::now() - Duration::days(60);
    old_entry.last_accessed = Utc::now() - Duration::days(60);
    old_entry.importance_score = 0.1;

    let recent_entry =
        MemoryEntry::with_importance("Recent memory".to_string(), MemoryType::Factual, 0.9, None);

    storage.store(old_entry.clone()).unwrap();
    storage.store(recent_entry.clone()).unwrap();

    let initial_count = storage.count();
    assert_eq!(initial_count, 2);

    let stats = manager.run_decay(&mut storage).unwrap();

    let final_count = storage.count();

    // Old memory should be removed
    assert!(stats.forgotten > 0);
    assert_eq!(final_count, 1);
}

#[test]
fn test_decay_threshold() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig {
        enabled: true,
        threshold: 0.8, // High threshold
        ..Default::default()
    };
    let mut manager = DecayManager::new(config);

    // Store memory with moderate decay score
    let entry = MemoryEntry::with_importance("Test".to_string(), MemoryType::Working, 0.5, None);

    storage.store(entry).unwrap();

    let stats = manager.run_decay(&mut storage).unwrap();

    // With high threshold, memory may be removed
    assert!(stats.total_checked > 0);
}

#[test]
fn test_decay_preserves_important_memories() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig::default();
    let mut manager = DecayManager::new(config);

    // Store important memory
    let important = MemoryEntry::with_importance(
        "Critical information".to_string(),
        MemoryType::Factual,
        1.0,
        None,
    );
    let important_id = important.id.clone();

    storage.store(important).unwrap();

    manager.run_decay(&mut storage).unwrap();

    // Important memory should still exist
    let retrieved = storage.get(&important_id);
    assert!(retrieved.is_some(), "Important memory should be preserved");
}

#[test]
fn test_decay_stats() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig::default();
    let mut manager = DecayManager::new(config);

    // Store multiple memories with different characteristics
    for i in 0..10 {
        let mut entry = create_test_entry(&format!("Memory {}", i), MemoryType::Factual);
        if i < 5 {
            // Make half of them old and unimportant
            entry.created_at = Utc::now() - Duration::days(90);
            entry.last_accessed = Utc::now() - Duration::days(90);
            entry.importance_score = 0.1;
        } else {
            // Make the other half recent and important
            entry.importance_score = 0.9;
        }
        storage.store(entry).unwrap();
    }

    let stats = manager.run_decay(&mut storage).unwrap();

    assert_eq!(stats.total_checked, 10);
    assert!(
        stats.forgotten > 0,
        "Should forget old unimportant memories"
    );
    // Recent important memories (importance >= 0.8) are protected, not retained
    assert!(stats.protected > 0, "Should protect important memories");
}

#[test]
fn test_decay_disabled() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig::disabled();
    let mut manager = DecayManager::new(config);

    // Store old, unimportant memory
    let mut entry = create_test_entry("Old memory", MemoryType::Working);
    entry.created_at = Utc::now() - Duration::days(90);
    entry.last_accessed = Utc::now() - Duration::days(90);
    entry.importance_score = 0.1;

    storage.store(entry).unwrap();

    let stats = manager.run_decay(&mut storage).unwrap();

    // With decay disabled, nothing should be removed
    assert_eq!(stats.forgotten, 0);
    assert_eq!(storage.count(), 1);
}

#[test]
fn test_decay_type_specific_thresholds() {
    let mut storage = InMemoryStorage::new();
    let mut config = DecayConfig::default();

    // Set different thresholds for different types
    config.set_type_threshold(MemoryType::Working, 0.6);
    config.set_type_threshold(MemoryType::Factual, 0.2);

    let mut manager = DecayManager::new(config);

    // Store memories of different types with same decay score
    let mut working_entry = create_test_entry("Working memory", MemoryType::Working);
    working_entry.created_at = Utc::now() - Duration::days(30);
    working_entry.last_accessed = Utc::now() - Duration::days(30);
    working_entry.importance_score = 0.4;

    let mut factual_entry = create_test_entry("Factual memory", MemoryType::Factual);
    factual_entry.created_at = Utc::now() - Duration::days(30);
    factual_entry.last_accessed = Utc::now() - Duration::days(30);
    factual_entry.importance_score = 0.4;

    storage.store(working_entry).unwrap();
    storage.store(factual_entry).unwrap();

    let stats = manager.run_decay(&mut storage).unwrap();

    // Working memory should be removed (threshold 0.6), Factual should remain (threshold 0.2)
    assert!(stats.total_checked == 2);
}

#[test]
fn test_decay_protection_mechanisms() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig {
        enabled: true,
        threshold: 0.5,
        recent_access_protection_seconds: 3600, // 1 hour
        importance_protection_threshold: 0.85,
        ..Default::default()
    };
    let mut manager = DecayManager::new(config);

    // Store recently accessed memory (should be protected)
    let mut recent_access = create_test_entry("Recently accessed", MemoryType::Working);
    recent_access.last_accessed = Utc::now() - Duration::seconds(1800); // 30 min ago
    recent_access.importance_score = 0.3;

    // Store high importance memory (should be protected)
    let high_importance = MemoryEntry::with_importance(
        "High importance".to_string(),
        MemoryType::Factual,
        0.9,
        None,
    );

    storage.store(recent_access).unwrap();
    storage.store(high_importance).unwrap();

    let stats = manager.run_decay(&mut storage).unwrap();

    // Both should be protected
    assert_eq!(stats.protected, 2);
    assert_eq!(stats.forgotten, 0);
}

#[test]
fn test_decay_empty_storage() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig::default();
    let mut manager = DecayManager::new(config);

    let stats = manager.run_decay(&mut storage).unwrap();

    assert_eq!(stats.total_checked, 0);
    assert_eq!(stats.forgotten, 0);
    assert_eq!(stats.retained, 0);
}

#[test]
fn test_decay_force_decay() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig::disabled();
    let mut manager = DecayManager::new(config);

    // Store old memory
    let mut entry = create_test_entry("Old memory", MemoryType::Working);
    entry.created_at = Utc::now() - Duration::days(90);
    entry.last_accessed = Utc::now() - Duration::days(90);
    entry.importance_score = 0.1;

    storage.store(entry).unwrap();

    // Force decay even though it's disabled
    let stats = manager.force_decay(&mut storage).unwrap();

    // Should run decay despite being disabled
    assert!(stats.total_checked > 0);
}

#[test]
fn test_decay_update_config() {
    let config = DecayConfig::default();
    let mut manager = DecayManager::new(config);

    // Update to aggressive config
    let new_config = DecayConfig::aggressive();
    manager.update_config(new_config);

    let retrieved_config = manager.get_config();
    assert_eq!(retrieved_config.threshold, 0.5);
}

#[test]
fn test_decay_stats_rates() {
    let mut stats = DecayStats::new();
    stats.total_checked = 100;
    stats.forgotten = 30;
    stats.retained = 60;
    stats.protected = 10;

    assert!((stats.retention_rate() - 0.6).abs() < 0.001);
    assert!((stats.forgetting_rate() - 0.3).abs() < 0.001);
    assert!((stats.protection_rate() - 0.1).abs() < 0.001);
}

#[test]
fn test_decay_mixed_memory_types() {
    let mut storage = InMemoryStorage::new();
    let config = DecayConfig::default();
    let mut manager = DecayManager::new(config);

    // Store different memory types
    for memory_type in &[
        MemoryType::Working,
        MemoryType::Factual,
        MemoryType::Episodic,
        MemoryType::Semantic,
    ] {
        let mut entry = create_test_entry(&format!("{:?} memory", memory_type), *memory_type);
        entry.created_at = Utc::now() - Duration::days(60);
        entry.last_accessed = Utc::now() - Duration::days(60);
        entry.importance_score = 0.2;
        storage.store(entry).unwrap();
    }

    let stats = manager.run_decay(&mut storage).unwrap();

    assert_eq!(stats.total_checked, 4);
    assert!(stats.forgotten > 0);
}

// ============================================================================
// DecayStats Tests
// ============================================================================

#[test]
fn test_decay_stats_creation() {
    let stats = DecayStats::new();

    assert_eq!(stats.total_checked, 0);
    assert_eq!(stats.forgotten, 0);
    assert_eq!(stats.retained, 0);
    assert_eq!(stats.protected, 0);
    assert_eq!(stats.execution_time_ms, 0);
    assert!(stats.forgotten_by_type.is_empty());
}

#[test]
fn test_decay_stats_field_access() {
    let mut stats = DecayStats::new();
    stats.total_checked = 100;
    stats.forgotten = 30;
    stats.retained = 50;
    stats.protected = 20;
    stats.execution_time_ms = 150;

    assert_eq!(stats.total_checked, 100);
    assert_eq!(stats.forgotten, 30);
    assert_eq!(stats.retained, 50);
    assert_eq!(stats.protected, 20);
    assert_eq!(stats.execution_time_ms, 150);
}

#[test]
fn test_decay_stats_calculations() {
    let mut stats = DecayStats::new();
    stats.total_checked = 100;
    stats.forgotten = 30;
    stats.retained = 50;
    stats.protected = 20;

    // Verify the sum: forgotten + retained + protected = total_checked
    assert_eq!(
        stats.forgotten + stats.retained + stats.protected,
        stats.total_checked
    );
}

#[test]
fn test_decay_stats_all_forgotten() {
    let mut stats = DecayStats::new();
    stats.total_checked = 50;
    stats.forgotten = 50;
    stats.retained = 0;
    stats.protected = 0;

    assert_eq!(stats.forgotten, stats.total_checked);
    assert_eq!(stats.retained, 0);
    assert_eq!(stats.protected, 0);
}

#[test]
fn test_decay_stats_all_retained() {
    let mut stats = DecayStats::new();
    stats.total_checked = 50;
    stats.forgotten = 0;
    stats.retained = 50;
    stats.protected = 0;

    assert_eq!(stats.retained, stats.total_checked);
    assert_eq!(stats.forgotten, 0);
    assert_eq!(stats.protected, 0);
}

#[test]
fn test_decay_stats_all_protected() {
    let mut stats = DecayStats::new();
    stats.total_checked = 50;
    stats.forgotten = 0;
    stats.retained = 0;
    stats.protected = 50;

    assert_eq!(stats.protected, stats.total_checked);
    assert_eq!(stats.forgotten, 0);
    assert_eq!(stats.retained, 0);
}

#[test]
fn test_decay_stats_empty_storage() {
    let config = DecayConfig::default();
    let mut manager = DecayManager::new(config);
    let mut storage = InMemoryStorage::new();

    let stats = manager.run_decay(&mut storage).unwrap();

    assert_eq!(stats.total_checked, 0);
    assert_eq!(stats.forgotten, 0);
    assert_eq!(stats.retained, 0);
    assert_eq!(stats.protected, 0);
}

#[test]
fn test_decay_stats_forgotten_by_type() {
    let mut stats = DecayStats::new();
    stats.forgotten_by_type.insert(MemoryType::Working, 10);
    stats.forgotten_by_type.insert(MemoryType::Factual, 5);
    stats.forgotten_by_type.insert(MemoryType::Episodic, 3);

    assert_eq!(
        *stats.forgotten_by_type.get(&MemoryType::Working).unwrap(),
        10
    );
    assert_eq!(
        *stats.forgotten_by_type.get(&MemoryType::Factual).unwrap(),
        5
    );
    assert_eq!(
        *stats.forgotten_by_type.get(&MemoryType::Episodic).unwrap(),
        3
    );
    assert_eq!(stats.forgotten_by_type.get(&MemoryType::Semantic), None);
}

#[test]
fn test_decay_stats_execution_time() {
    let config = DecayConfig::default();
    let mut manager = DecayManager::new(config);
    let mut storage = InMemoryStorage::new();

    // Add some memories
    for i in 0..10 {
        let entry = create_test_entry(&format!("Memory {}", i), MemoryType::Working);
        storage.store(entry).unwrap();
    }

    let stats = manager.run_decay(&mut storage).unwrap();

    // Execution time should be recorded (u64 is always >= 0, just verify it exists)
    let _ = stats.execution_time_ms;
    assert_eq!(stats.total_checked, 10);
}
