//! File I/O integration tests
//!
//! Tests for file reading, writing, and configuration parsing
//! with real file operations and error handling.

use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_json_config_parsing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.json");

    let config_content = serde_json::json!({
        "llm": {
            "provider": "openai",
            "model": "gpt-3.5-turbo",
            "api_key": "test-key",
            "base_url": null,
            "organization": null
        },
        "agents": {
            "test-agent": {
                "max_tokens": 1000,
                "temperature": 0.7,
                "system_prompt": "You are a helpful assistant.",
                "capabilities": ["text-generation"]
            }
        }
    });

    fs::write(
        &config_path,
        serde_json::to_string_pretty(&config_content).unwrap(),
    )
    .expect("Failed to write config file");

    // Test reading and parsing the config
    let content = fs::read_to_string(&config_path).expect("Failed to read config file");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");

    assert_eq!(parsed["llm"]["provider"], "openai");
    assert_eq!(parsed["agents"]["test-agent"]["max_tokens"], 1000);
}

#[tokio::test]
async fn test_json_workflow_parsing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let workflow_path = temp_dir.path().join("workflow.json");

    let workflow_content = serde_json::json!({
        "name": "Test Workflow",
        "description": "A test workflow",
        "nodes": [
            {
                "id": "start",
                "type": "agent",
                "name": "Start Agent",
                "description": "Starting agent",
                "config": {
                    "agent_id": "test-agent",
                    "prompt": "Hello world"
                }
            }
        ],
        "edges": [],
        "metadata": {}
    });

    fs::write(
        &workflow_path,
        serde_json::to_string_pretty(&workflow_content).unwrap(),
    )
    .expect("Failed to write workflow file");

    // Test reading and parsing the workflow
    let content = fs::read_to_string(&workflow_path).expect("Failed to read workflow file");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");

    assert_eq!(parsed["name"], "Test Workflow");
    assert_eq!(parsed["nodes"][0]["type"], "agent");
}

#[tokio::test]
async fn test_malformed_json_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let malformed_path = temp_dir.path().join("malformed.json");

    // Write malformed JSON
    fs::write(&malformed_path, "{invalid json content}").expect("Failed to write malformed file");

    // Test that parsing fails gracefully
    let content = fs::read_to_string(&malformed_path).expect("Failed to read file");
    let result = serde_json::from_str::<serde_json::Value>(&content);

    assert!(result.is_err(), "Should fail to parse malformed JSON");
}

#[tokio::test]
async fn test_empty_file_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let empty_path = temp_dir.path().join("empty.json");

    // Create empty file
    fs::write(&empty_path, "").expect("Failed to create empty file");

    // Test reading empty file
    let content = fs::read_to_string(&empty_path).expect("Failed to read empty file");
    assert!(content.is_empty(), "File should be empty");

    // Test parsing empty content
    let result = serde_json::from_str::<serde_json::Value>(&content);
    assert!(result.is_err(), "Should fail to parse empty JSON");
}

#[tokio::test]
async fn test_file_permissions() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_path = temp_dir.path().join("test.json");

    let test_content = serde_json::json!({"test": "data"});
    fs::write(&test_path, serde_json::to_string(&test_content).unwrap())
        .expect("Failed to write test file");

    // Test that file exists and is readable
    assert!(test_path.exists(), "File should exist");
    assert!(test_path.is_file(), "Should be a file");

    let metadata = fs::metadata(&test_path).expect("Failed to get metadata");
    assert!(metadata.len() > 0, "File should have content");
}

#[tokio::test]
async fn test_nested_directory_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let nested_path = temp_dir
        .path()
        .join("level1")
        .join("level2")
        .join("config.json");

    // Create parent directories
    if let Some(parent) = nested_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    }

    let config_content = serde_json::json!({"nested": true});
    fs::write(
        &nested_path,
        serde_json::to_string(&config_content).unwrap(),
    )
    .expect("Failed to write nested file");

    assert!(nested_path.exists(), "Nested file should exist");

    let content = fs::read_to_string(&nested_path).expect("Failed to read nested file");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse");
    assert_eq!(parsed["nested"], true);
}

#[tokio::test]
async fn test_unicode_file_content() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let unicode_path = temp_dir.path().join("unicode.json");

    let unicode_content = serde_json::json!({
        "message": "Hello, 世界! 🌍 Здравствуй мир! مرحبا بالعالم!",
        "emoji": "🚀✨🎉",
        "chinese": "你好世界",
        "russian": "Привет мир",
        "arabic": "مرحبا بالعالم"
    });

    fs::write(
        &unicode_path,
        serde_json::to_string(&unicode_content).unwrap(),
    )
    .expect("Failed to write unicode file");

    let content = fs::read_to_string(&unicode_path).expect("Failed to read unicode file");
    let parsed: serde_json::Value =
        serde_json::from_str(&content).expect("Failed to parse unicode JSON");

    assert!(parsed["message"].as_str().unwrap().contains("世界"));
    assert!(parsed["emoji"].as_str().unwrap().contains("🚀"));
}

#[tokio::test]
async fn test_large_file_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let large_path = temp_dir.path().join("large.json");

    // Create a reasonably large JSON structure
    let mut large_data = std::collections::HashMap::with_capacity(1000); // Pre-allocate for large data
    for i in 0..1000 {
        large_data.insert(
            format!("key_{}", i),
            format!(
                "This is value number {} with some additional text to make it larger",
                i
            ),
        );
    }

    let large_content = serde_json::json!(large_data);
    fs::write(&large_path, serde_json::to_string(&large_content).unwrap())
        .expect("Failed to write large file");

    let content = fs::read_to_string(&large_path).expect("Failed to read large file");
    let parsed: serde_json::Value =
        serde_json::from_str(&content).expect("Failed to parse large JSON");

    assert!(
        parsed.as_object().unwrap().len() == 1000,
        "Should have 1000 entries"
    );
    assert!(parsed["key_999"].as_str().unwrap().contains("999"));
}

#[tokio::test]
async fn test_file_validation_before_parsing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let valid_path = temp_dir.path().join("valid.json");
    let invalid_path = temp_dir.path().join("invalid.txt");

    // Create valid JSON file
    let valid_content = serde_json::json!({"valid": true});
    fs::write(&valid_path, serde_json::to_string(&valid_content).unwrap())
        .expect("Failed to write valid file");

    // Create invalid file (not JSON)
    fs::write(&invalid_path, "This is not JSON content").expect("Failed to write invalid file");

    // Test valid file
    assert!(valid_path.exists());
    if valid_path.extension().and_then(|s| s.to_str()) == Some("json") {
        let content = fs::read_to_string(&valid_path).expect("Failed to read valid file");
        let result = serde_json::from_str::<serde_json::Value>(&content);
        assert!(result.is_ok(), "Valid JSON should parse successfully");
    }

    // Test invalid file extension
    assert!(invalid_path.exists());
    let is_json_file = invalid_path.extension().and_then(|s| s.to_str()) == Some("json");
    assert!(!is_json_file, "Should detect non-JSON file extension");
}

#[tokio::test]
async fn test_environment_variable_substitution() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("env_config.json");

    // Set test environment variable
    std::env::set_var("TEST_API_KEY", "test-key-from-env");

    let config_content = serde_json::json!({
        "llm": {
            "provider": "openai",
            "api_key": "${TEST_API_KEY}",
            "model": "gpt-3.5-turbo"
        }
    });

    fs::write(
        &config_path,
        serde_json::to_string(&config_content).unwrap(),
    )
    .expect("Failed to write config file");

    let content = fs::read_to_string(&config_path).expect("Failed to read config file");

    // Manual environment variable substitution for testing
    let env_value = std::env::var("TEST_API_KEY").unwrap_or_default();
    let substituted_content = content.replace("${TEST_API_KEY}", &env_value);

    let parsed: serde_json::Value =
        serde_json::from_str(&substituted_content).expect("Failed to parse substituted JSON");

    assert_eq!(parsed["llm"]["api_key"], "test-key-from-env");

    // Clean up
    std::env::remove_var("TEST_API_KEY");
}

#[tokio::test]
async fn test_concurrent_file_access() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let shared_path = temp_dir.path().join("shared.json");

    let initial_content = serde_json::json!({"counter": 0});
    fs::write(
        &shared_path,
        serde_json::to_string(&initial_content).unwrap(),
    )
    .expect("Failed to write initial file");

    let mut handles = vec![];

    for i in 0..5 {
        let path = shared_path.clone();
        let handle = tokio::spawn(async move {
            // Read the file
            let content = fs::read_to_string(&path).expect("Failed to read file");
            let mut parsed: serde_json::Value =
                serde_json::from_str(&content).expect("Failed to parse JSON");

            // Modify the content
            parsed["counter"] = serde_json::json!(i);

            // Write back (this might cause race conditions, but that's what we're testing)
            fs::write(&path, serde_json::to_string(&parsed).unwrap())
                .expect("Failed to write file");
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task failed");
    }

    // Verify final state
    let final_content = fs::read_to_string(&shared_path).expect("Failed to read final file");
    let final_parsed: serde_json::Value =
        serde_json::from_str(&final_content).expect("Failed to parse final JSON");

    // The final counter value should be one of 0-4
    let counter = final_parsed["counter"].as_i64().unwrap();
    assert!(
        (0..5).contains(&counter),
        "Counter should be within expected range"
    );
}

#[tokio::test]
async fn test_backup_and_restore() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let original_path = temp_dir.path().join("original.json");
    let backup_path = temp_dir.path().join("original.json.backup");

    let original_content = serde_json::json!({
        "important_data": "This should not be lost",
        "version": 1
    });

    // Write original file
    fs::write(
        &original_path,
        serde_json::to_string_pretty(&original_content).unwrap(),
    )
    .expect("Failed to write original file");

    // Create backup
    fs::copy(&original_path, &backup_path).expect("Failed to create backup");

    // Modify original file
    let modified_content = serde_json::json!({
        "important_data": "This has been modified",
        "version": 2
    });
    fs::write(
        &original_path,
        serde_json::to_string_pretty(&modified_content).unwrap(),
    )
    .expect("Failed to write modified file");

    // Restore from backup
    fs::copy(&backup_path, &original_path).expect("Failed to restore from backup");

    // Verify restoration
    let restored_content =
        fs::read_to_string(&original_path).expect("Failed to read restored file");
    let restored_parsed: serde_json::Value =
        serde_json::from_str(&restored_content).expect("Failed to parse restored JSON");

    assert_eq!(restored_parsed["important_data"], "This should not be lost");
    assert_eq!(restored_parsed["version"], 1);
}
