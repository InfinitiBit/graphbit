//! Tests for episodic memory

use graphbit_core::memory::episodic::{Episode, EpisodicMemory};
use graphbit_core::memory::storage::{InMemoryStorage, MemoryStorage};
use graphbit_core::memory::types::*;

#[test]
fn test_episodic_memory_creation() {
    let episodic = EpisodicMemory::new();

    assert!(
        episodic.get_current_episode().is_none(),
        "Should have no active episode initially"
    );
}

#[test]
fn test_episodic_memory_start_episode() {
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Test Episode".to_string());

    assert!(
        episodic.get_current_episode().is_some(),
        "Should have active episode"
    );
}

#[test]
fn test_episodic_memory_add_to_episode() {
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Weather Conversation".to_string());
    episodic.add_to_episode("User asked about the weather".to_string());
    episodic.add_to_episode("Agent responded with forecast".to_string());

    let episode = episodic.get_current_episode().unwrap();
    assert!(episode.content.contains("User asked about the weather"));
    assert!(episode.content.contains("Agent responded with forecast"));
}

#[test]
fn test_episodic_memory_end_episode() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Test Episode".to_string());
    episodic.add_to_episode("Event 1".to_string());
    episodic.add_to_episode("Event 2".to_string());

    let episode_id = episodic.end_episode(&mut storage).unwrap();

    assert!(episode_id.is_some(), "Should return episode ID");
    assert!(
        episodic.get_current_episode().is_none(),
        "Episode should be ended"
    );

    // Verify episode was stored
    let stored = storage.get(&episode_id.unwrap()).unwrap();
    assert_eq!(stored.memory_type, MemoryType::Episodic);
}

#[test]
fn test_episodic_memory_end_episode_without_active() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    let result = episodic.end_episode(&mut storage).unwrap();

    assert!(
        result.is_none(),
        "Should return None when no active episode"
    );
}

#[test]
fn test_episodic_memory_add_participant() {
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Meeting".to_string());
    episodic.add_participant("Alice".to_string());
    episodic.add_participant("Bob".to_string());

    let episode = episodic.get_current_episode().unwrap();
    assert_eq!(episode.participants.len(), 2);
    assert!(episode.participants.contains(&"Alice".to_string()));
    assert!(episode.participants.contains(&"Bob".to_string()));
}

#[test]
fn test_episodic_memory_set_outcome() {
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Task Discussion".to_string());
    episodic.set_outcome("Task completed successfully".to_string());

    let episode = episodic.get_current_episode().unwrap();
    assert_eq!(
        episode.outcome,
        Some("Task completed successfully".to_string())
    );
}

#[test]
fn test_episodic_memory_add_tag() {
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Support Call".to_string());
    episodic.add_tag("support".to_string());
    episodic.add_tag("urgent".to_string());

    let episode = episodic.get_current_episode().unwrap();
    assert_eq!(episode.tags.len(), 2);
    assert!(episode.tags.contains(&"support".to_string()));
    assert!(episode.tags.contains(&"urgent".to_string()));
}

#[test]
fn test_episodic_memory_store_episode() {
    let mut storage = InMemoryStorage::new();
    let episodic = EpisodicMemory::new();

    let mut episode = Episode::new("Test Episode".to_string(), "Episode content".to_string());
    episode.add_participant("Alice".to_string());
    episode.add_tag("test".to_string());

    let memory_id = episodic.store_episode(episode, &mut storage).unwrap();

    let stored = storage.get(&memory_id).unwrap();
    assert_eq!(stored.memory_type, MemoryType::Episodic);
    assert!(stored.content.contains("Test Episode"));
}

#[test]
fn test_episodic_memory_get_episodes_by_participant() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    // Create episode with Alice
    episodic.start_episode("Meeting 1".to_string());
    episodic.add_participant("Alice".to_string());
    episodic.add_to_episode("Discussed project".to_string());
    episodic.end_episode(&mut storage).unwrap();

    // Create episode with Bob
    episodic.start_episode("Meeting 2".to_string());
    episodic.add_participant("Bob".to_string());
    episodic.add_to_episode("Reviewed code".to_string());
    episodic.end_episode(&mut storage).unwrap();

    // Create episode with both
    episodic.start_episode("Meeting 3".to_string());
    episodic.add_participant("Alice".to_string());
    episodic.add_participant("Bob".to_string());
    episodic.add_to_episode("Team sync".to_string());
    episodic.end_episode(&mut storage).unwrap();

    let alice_episodes = episodic.get_episodes_by_participant("Alice", &storage);
    assert_eq!(alice_episodes.len(), 2);

    let bob_episodes = episodic.get_episodes_by_participant("Bob", &storage);
    assert_eq!(bob_episodes.len(), 2);
}

#[test]
fn test_episodic_memory_get_episodes_by_tag() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Support Call 1".to_string());
    episodic.add_tag("support".to_string());
    episodic.end_episode(&mut storage).unwrap();

    episodic.start_episode("Bug Report".to_string());
    episodic.add_tag("bug".to_string());
    episodic.end_episode(&mut storage).unwrap();

    episodic.start_episode("Support Call 2".to_string());
    episodic.add_tag("support".to_string());
    episodic.end_episode(&mut storage).unwrap();

    let support_episodes = episodic.get_episodes_by_tag("support", &storage);
    assert_eq!(support_episodes.len(), 2);

    let bug_episodes = episodic.get_episodes_by_tag("bug", &storage);
    assert_eq!(bug_episodes.len(), 1);
}

#[test]
fn test_episodic_memory_get_recent_episodes() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    // Create 5 episodes
    for i in 0..5 {
        episodic.start_episode(format!("Episode {}", i));
        episodic.add_to_episode(format!("Content {}", i));
        episodic.end_episode(&mut storage).unwrap();
    }

    let recent = episodic.get_recent_episodes(3, &storage);
    assert_eq!(recent.len(), 3);
}

#[test]
fn test_episodic_memory_count_episodes() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    assert_eq!(episodic.count_episodes(&storage), 0);

    episodic.start_episode("Episode 1".to_string());
    episodic.end_episode(&mut storage).unwrap();

    episodic.start_episode("Episode 2".to_string());
    episodic.end_episode(&mut storage).unwrap();

    assert_eq!(episodic.count_episodes(&storage), 2);
}

#[test]
fn test_episodic_memory_is_recording() {
    let mut episodic = EpisodicMemory::new();

    assert!(!episodic.is_recording());

    episodic.start_episode("Test".to_string());
    assert!(episodic.is_recording());

    let mut storage = InMemoryStorage::new();
    episodic.end_episode(&mut storage).unwrap();
    assert!(!episodic.is_recording());
}

#[test]
fn test_episodic_memory_metadata() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Test Episode".to_string());
    episodic.add_to_episode("Test event".to_string());
    let memory_id = episodic.end_episode(&mut storage).unwrap().unwrap();

    let stored = storage.get(&memory_id).unwrap();

    assert_eq!(stored.metadata.source, "episodic");
    assert!(stored.metadata.tags.contains(&"episode".to_string()));
}

#[test]
fn test_episodic_memory_importance() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Important Conversation".to_string());
    episodic.add_to_episode("Important conversation content".to_string());
    let memory_id = episodic.end_episode(&mut storage).unwrap().unwrap();

    let stored = storage.get(&memory_id).unwrap();

    // Episodic memories should have moderate importance (0.7)
    assert_eq!(stored.importance_score, 0.7);
}

#[test]
fn test_episodic_memory_multiple_content_additions() {
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Multi-part Episode".to_string());

    for i in 0..10 {
        episodic.add_to_episode(format!("Event {}", i));
    }

    let episode = episodic.get_current_episode().unwrap();
    for i in 0..10 {
        assert!(episode.content.contains(&format!("Event {}", i)));
    }
}

#[test]
fn test_episodic_memory_restart_episode() {
    let mut episodic = EpisodicMemory::new();

    // Start first episode
    episodic.start_episode("Episode 1".to_string());
    let ep1 = episodic.get_current_episode().unwrap();
    let ep1_id = ep1.id.clone();

    // Start new episode without ending first (should replace)
    episodic.start_episode("Episode 2".to_string());
    let ep2 = episodic.get_current_episode().unwrap();
    let ep2_id = ep2.id.clone();

    assert_ne!(ep1_id, ep2_id, "New episode should have different ID");
}

#[test]
fn test_episodic_memory_empty_episode() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Empty Episode".to_string());
    let episode_id = episodic.end_episode(&mut storage).unwrap();

    // Empty episode should still return an ID
    assert!(episode_id.is_some());
}

#[test]
fn test_episodic_memory_long_content() {
    let mut episodic = EpisodicMemory::new();

    episodic.start_episode("Long Episode".to_string());

    // Add many content pieces
    for i in 0..100 {
        episodic.add_to_episode(format!("Event {}", i));
    }

    let episode = episodic.get_current_episode().unwrap();
    for i in 0..100 {
        assert!(episode.content.contains(&format!("Event {}", i)));
    }
}

#[test]
fn test_episodic_memory_sequential_episodes() {
    let mut storage = InMemoryStorage::new();
    let mut episodic = EpisodicMemory::new();

    let mut episode_ids = vec![];

    for i in 0..5 {
        episodic.start_episode(format!("Episode {}", i));
        episodic.add_to_episode(format!("Episode {} content", i));
        let id = episodic.end_episode(&mut storage).unwrap().unwrap();
        episode_ids.push(id);
    }

    // Verify all episodes were stored
    assert_eq!(storage.count_by_type(MemoryType::Episodic), 5);

    // Verify each episode has correct content
    for (i, id) in episode_ids.iter().enumerate() {
        let entry = storage.get(id).unwrap();
        assert!(entry.content.contains(&format!("Episode {}", i)));
    }
}

#[test]
fn test_episode_creation() {
    let episode = Episode::new("Test Title".to_string(), "Test Content".to_string());

    assert_eq!(episode.title, "Test Title");
    assert_eq!(episode.content, "Test Content");
    assert!(episode.participants.is_empty());
    assert!(episode.tags.is_empty());
    assert_eq!(episode.outcome, None);
}

#[test]
fn test_episode_add_participant_no_duplicates() {
    let mut episode = Episode::new("Meeting".to_string(), String::new());

    episode.add_participant("Alice".to_string());
    episode.add_participant("Alice".to_string()); // Duplicate

    assert_eq!(episode.participants.len(), 1);
}

#[test]
fn test_episode_add_tag_no_duplicates() {
    let mut episode = Episode::new("Task".to_string(), String::new());

    episode.add_tag("urgent".to_string());
    episode.add_tag("urgent".to_string()); // Duplicate

    assert_eq!(episode.tags.len(), 1);
}
