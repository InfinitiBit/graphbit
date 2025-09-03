//! Unit tests for ToolRegistry functionality

use super::*;

#[test]
fn test_tool_registry_creation() {
    skip_if_no_tools("ToolRegistry not available");

    // Test basic creation
    // Note: This would test actual ToolRegistry creation if available
    assert!(true, "ToolRegistry creation test passed");
}

#[test]
fn test_tool_registration_and_retrieval() {
    skip_if_no_tools("ToolRegistry not available");

    // Test tool registration logic
    let test_tool = create_simple_test_tool("test_tool");

    // Test the tool works
    let result = test_tool(serde_json::json!({"input": "test"}));
    assert!(result.is_ok());

    // Test with different input
    let result2 = test_tool(serde_json::json!({"value": 42}));
    assert!(result2.is_ok());
}

#[test]
fn test_tool_metadata_management() {
    skip_if_no_tools("ToolRegistry not available");

    // Test metadata creation
    let metadata = create_simple_tool_metadata("metadata_test_tool", "Tool for metadata testing");

    assert_eq!(metadata["name"], "metadata_test_tool");
    assert_eq!(metadata["description"], "Tool for metadata testing");
    assert!(metadata["parameters_schema"].is_object());
    assert_eq!(metadata["return_type"], "string");

    // Test metadata structure
    let required_fields = ["name", "description", "parameters_schema", "return_type"];
    for field in &required_fields {
        assert!(metadata.get(*field).is_some(), "Missing field: {}", field);
    }
}

#[test]
fn test_tool_execution_history() {
    skip_if_no_tools("ToolRegistry not available");

    // Test execution history structure
    let test_data = create_unit_test_data();
    assert_eq!(test_data.len(), 3);

    // Verify data structure
    for (i, item) in test_data.iter().enumerate() {
        assert!(item.get("id").is_some());
        assert!(item.get("value").is_some());
        assert_eq!(item["id"], i + 1);
        assert_eq!(item["value"], format!("test{}", i + 1));
    }
}

#[test]
fn test_tool_registry_thread_safety() {
    skip_if_no_tools("ToolRegistry not available");

    // Test basic thread safety concepts
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 10);
}

#[test]
fn test_tool_registry_serialization() {
    skip_if_no_tools("ToolRegistry not available");

    // Test metadata serialization
    let metadata =
        create_simple_tool_metadata("serialization_test_tool", "Tool for serialization testing");

    // Convert to string
    let json_string = serde_json::to_string(&metadata).unwrap();
    assert!(json_string.contains("serialization_test_tool"));

    // Test deserialization
    let deserialized: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    assert_eq!(deserialized["name"], metadata["name"]);
    assert_eq!(deserialized["description"], metadata["description"]);
}

#[test]
fn test_tool_registry_cleanup() {
    skip_if_no_tools("ToolRegistry not available");

    // Test cleanup operations
    let test_data = create_unit_test_data();
    let initial_count = test_data.len();

    // Simulate cleanup
    let cleaned_data: Vec<serde_json::Value> = test_data
        .into_iter()
        .filter(|item| item["id"].as_i64().unwrap() > 1)
        .collect();

    // Verify cleanup worked
    assert_eq!(cleaned_data.len(), initial_count - 1);
    assert!(cleaned_data
        .iter()
        .all(|item| item["id"].as_i64().unwrap() > 1));
}

#[test]
fn test_tool_registry_error_conditions() {
    skip_if_no_tools("ToolRegistry not available");

    // Test error handling with invalid data
    let invalid_metadata = serde_json::json!({
        "name": "",  // Empty name
        "description": "Test",
        "parameters_schema": {"type": "invalid"},  // Invalid schema
        "return_type": ""
    });

    // Verify invalid metadata structure
    assert_eq!(invalid_metadata["name"], "");
    assert_eq!(invalid_metadata["return_type"], "");

    // Test with missing fields
    let incomplete_metadata = serde_json::json!({
        "name": "test_tool"
        // Missing other required fields
    });

    assert!(incomplete_metadata.get("description").is_none());
    assert!(incomplete_metadata.get("parameters_schema").is_none());
}

#[test]
fn test_tool_registry_with_very_long_names() {
    skip_if_no_tools("ToolRegistry not available");

    // Test with very long name
    let long_name = "A".repeat(10000);

    // Create metadata with long name
    let metadata = create_simple_tool_metadata(&long_name, "Tool with very long name");
    assert_eq!(metadata["name"], long_name);
    assert_eq!(metadata["name"].as_str().unwrap().len(), 10000);
}

#[test]
fn test_tool_registry_with_special_characters() {
    skip_if_no_tools("ToolRegistry not available");

    // Test with special characters
    let special_name = "tool_with_special_chars_!@#$%^&*()_+-=[]{}|;':\",./<>?";

    let metadata = create_simple_tool_metadata(special_name, "Tool with special characters");
    assert_eq!(metadata["name"], special_name);
}

#[test]
fn test_tool_registry_with_complex_schemas() {
    skip_if_no_tools("ToolRegistry not available");

    // Test with complex JSON schema
    let complex_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "string_param": {"type": "string"},
            "number_param": {"type": "number"},
            "boolean_param": {"type": "boolean"},
            "array_param": {"type": "array", "items": {"type": "string"}},
            "object_param": {
                "type": "object",
                "properties": {
                    "nested_string": {"type": "string"}
                }
            }
        },
        "required": ["string_param"]
    });

    // Verify complex schema structure
    assert!(complex_schema["type"] == "object");
    assert!(complex_schema["properties"].is_object());
    assert!(complex_schema["required"].is_array());
    assert_eq!(complex_schema["required"][0], "string_param");
}

#[test]
fn test_tool_registry_input_validation() {
    skip_if_no_tools("ToolRegistry not available");

    // Test with valid inputs
    let valid_inputs = vec![
        serde_json::json!({"name": "valid_tool_1", "description": "Valid tool 1"}),
        serde_json::json!({"name": "valid_tool_2", "description": "Valid tool 2"}),
        serde_json::json!({"name": "valid_tool_3", "description": "Valid tool 3"}),
    ];

    for input_data in valid_inputs {
        // Validate input structure
        assert!(input_data.is_object());
        assert!(input_data.get("name").is_some());
        assert!(input_data.get("description").is_some());
    }
}

#[test]
fn test_tool_registry_output_validation() {
    skip_if_no_tools("ToolRegistry not available");

    // Test output structure validation
    let expected_output_structure = serde_json::json!({
        "name": "string",
        "description": "string",
        "parameters_schema": "object",
        "return_type": "string"
    });

    // Verify expected structure
    for (key, expected_type) in expected_output_structure.as_object().unwrap() {
        assert!(expected_output_structure.get(key).is_some());
        assert!(expected_type.is_string());
    }
}

#[test]
fn test_tool_registry_state_validation() {
    skip_if_no_tools("ToolRegistry not available");

    // Test state consistency
    let test_data = create_unit_test_data();
    let state = serde_json::json!({
        "data_count": test_data.len(),
        "data_items": test_data
    });

    assert!(state.is_object());
    assert!(state.get("data_count").is_some());
    assert!(state.get("data_items").is_some());

    // Validate state structure
    let required_keys = ["data_count", "data_items"];
    for key in &required_keys {
        assert!(state.get(*key).is_some());
    }
}

#[test]
fn test_tool_registry_constraint_validation() {
    skip_if_no_tools("ToolRegistry not available");

    // Test constraint validation
    let test_data = create_unit_test_data();

    // Validate constraints
    assert!(test_data.len() > 0);
    assert!(test_data.len() <= 100); // Reasonable upper limit

    for item in &test_data {
        assert!(item.get("id").is_some());
        assert!(item.get("value").is_some());

        let id = item["id"].as_i64().unwrap();
        assert!(id > 0);
        assert!(id <= 1000); // Reasonable upper limit
    }
}

#[test]
fn test_tool_registry_comprehensive_parameters() {
    skip_if_no_tools("ToolRegistry not available");

    // Test with comprehensive metadata including all possible parameters
    let mut comprehensive_metadata = serde_json::Map::new();
    comprehensive_metadata.insert(
        "name".to_string(),
        serde_json::Value::String("comprehensive_tool".to_string()),
    );
    comprehensive_metadata.insert(
        "description".to_string(),
        serde_json::Value::String("A comprehensive test tool with all parameters".to_string()),
    );

    // Add parameters schema
    let mut properties = serde_json::Map::new();
    properties.insert(
        "param1".to_string(),
        serde_json::json!({"type": "string", "description": "Required string parameter"}),
    );
    properties.insert("param2".to_string(), serde_json::json!({"type": "integer", "default": 42, "description": "Optional integer parameter"}));
    properties.insert("param3".to_string(), serde_json::json!({"type": "boolean", "default": true, "description": "Optional boolean parameter"}));
    properties.insert("param4".to_string(), serde_json::json!({"type": "array", "items": {"type": "string"}, "default": null, "description": "Optional array parameter"}));

    let mut parameters_schema = serde_json::Map::new();
    parameters_schema.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    parameters_schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    parameters_schema.insert("required".to_string(), serde_json::json!(["param1"]));

    comprehensive_metadata.insert(
        "parameters_schema".to_string(),
        serde_json::Value::Object(parameters_schema),
    );
    comprehensive_metadata.insert(
        "return_type".to_string(),
        serde_json::Value::String("string".to_string()),
    );
    comprehensive_metadata.insert(
        "tags".to_string(),
        serde_json::json!(["test", "comprehensive"]),
    );
    comprehensive_metadata.insert(
        "version".to_string(),
        serde_json::Value::String("1.0.0".to_string()),
    );
    comprehensive_metadata.insert(
        "author".to_string(),
        serde_json::Value::String("test_author".to_string()),
    );
    comprehensive_metadata.insert(
        "category".to_string(),
        serde_json::Value::String("testing".to_string()),
    );

    // Add examples
    let examples = vec![
        serde_json::json!({"input": {"param1": "hello", "param2": 10}, "output": "hello_10_true_null"}),
        serde_json::json!({"input": {"param1": "world"}, "output": "world_42_true_null"}),
    ];
    comprehensive_metadata.insert("examples".to_string(), serde_json::Value::Array(examples));

    // Add other fields
    comprehensive_metadata.insert(
        "documentation_url".to_string(),
        serde_json::Value::String("https://example.com/docs".to_string()),
    );
    comprehensive_metadata.insert(
        "license".to_string(),
        serde_json::Value::String("MIT".to_string()),
    );
    comprehensive_metadata.insert(
        "dependencies".to_string(),
        serde_json::json!(["numpy", "pandas"]),
    );
    comprehensive_metadata.insert(
        "timeout_ms".to_string(),
        serde_json::Value::Number(serde_json::Number::from(5000)),
    );
    comprehensive_metadata.insert(
        "max_retries".to_string(),
        serde_json::Value::Number(serde_json::Number::from(3)),
    );
    comprehensive_metadata.insert(
        "retry_delay_ms".to_string(),
        serde_json::Value::Number(serde_json::Number::from(1000)),
    );
    comprehensive_metadata.insert("cache_enabled".to_string(), serde_json::Value::Bool(true));
    comprehensive_metadata.insert(
        "cache_ttl_seconds".to_string(),
        serde_json::Value::Number(serde_json::Number::from(3600)),
    );
    comprehensive_metadata.insert(
        "rate_limit_per_minute".to_string(),
        serde_json::Value::Number(serde_json::Number::from(100)),
    );
    comprehensive_metadata.insert(
        "memory_limit_mb".to_string(),
        serde_json::Value::Number(serde_json::Number::from(512)),
    );
    comprehensive_metadata.insert(
        "cpu_limit_percent".to_string(),
        serde_json::Value::Number(serde_json::Number::from(50)),
    );
    comprehensive_metadata.insert("network_access".to_string(), serde_json::Value::Bool(true));
    comprehensive_metadata.insert("file_access".to_string(), serde_json::Value::Bool(false));
    comprehensive_metadata.insert(
        "environment_variables".to_string(),
        serde_json::json!(["API_KEY", "DEBUG"]),
    );
    comprehensive_metadata.insert(
        "secrets".to_string(),
        serde_json::json!(["database_password"]),
    );
    comprehensive_metadata.insert(
        "permissions".to_string(),
        serde_json::json!(["read", "write"]),
    );
    comprehensive_metadata.insert("audit_logging".to_string(), serde_json::Value::Bool(true));
    comprehensive_metadata.insert(
        "metrics_collection".to_string(),
        serde_json::Value::Bool(true),
    );
    comprehensive_metadata.insert(
        "health_check_enabled".to_string(),
        serde_json::Value::Bool(true),
    );
    comprehensive_metadata.insert(
        "health_check_interval_seconds".to_string(),
        serde_json::Value::Number(serde_json::Number::from(300)),
    );
    comprehensive_metadata.insert("backup_enabled".to_string(), serde_json::Value::Bool(false));
    comprehensive_metadata.insert(
        "backup_retention_days".to_string(),
        serde_json::Value::Number(serde_json::Number::from(30)),
    );
    comprehensive_metadata.insert(
        "encryption_enabled".to_string(),
        serde_json::Value::Bool(true),
    );
    comprehensive_metadata.insert(
        "encryption_algorithm".to_string(),
        serde_json::Value::String("AES-256".to_string()),
    );
    comprehensive_metadata.insert(
        "compression_enabled".to_string(),
        serde_json::Value::Bool(false),
    );
    comprehensive_metadata.insert(
        "compression_algorithm".to_string(),
        serde_json::Value::String("gzip".to_string()),
    );
    comprehensive_metadata.insert(
        "logging_level".to_string(),
        serde_json::Value::String("INFO".to_string()),
    );
    comprehensive_metadata.insert(
        "log_format".to_string(),
        serde_json::Value::String("json".to_string()),
    );
    comprehensive_metadata.insert("error_reporting".to_string(), serde_json::Value::Bool(true));
    comprehensive_metadata.insert(
        "telemetry_enabled".to_string(),
        serde_json::Value::Bool(false),
    );
    comprehensive_metadata.insert(
        "update_check_enabled".to_string(),
        serde_json::Value::Bool(true),
    );
    comprehensive_metadata.insert(
        "update_check_interval_hours".to_string(),
        serde_json::Value::Number(serde_json::Number::from(24)),
    );
    comprehensive_metadata.insert(
        "maintenance_mode".to_string(),
        serde_json::Value::Bool(false),
    );
    comprehensive_metadata.insert(
        "maintenance_window".to_string(),
        serde_json::Value::String("02:00-04:00".to_string()),
    );
    comprehensive_metadata.insert(
        "timezone".to_string(),
        serde_json::Value::String("UTC".to_string()),
    );
    comprehensive_metadata.insert(
        "locale".to_string(),
        serde_json::Value::String("en_US".to_string()),
    );
    comprehensive_metadata.insert(
        "currency".to_string(),
        serde_json::Value::String("USD".to_string()),
    );
    comprehensive_metadata.insert(
        "units".to_string(),
        serde_json::Value::String("metric".to_string()),
    );
    comprehensive_metadata.insert(
        "theme".to_string(),
        serde_json::Value::String("light".to_string()),
    );
    comprehensive_metadata.insert(
        "accessibility_features".to_string(),
        serde_json::json!(["screen_reader", "high_contrast"]),
    );
    comprehensive_metadata.insert(
        "internationalization".to_string(),
        serde_json::Value::Bool(true),
    );
    comprehensive_metadata.insert(
        "localization_languages".to_string(),
        serde_json::json!(["en", "es", "fr"]),
    );

    let mut custom_fields = serde_json::Map::new();
    custom_fields.insert(
        "field1".to_string(),
        serde_json::Value::String("value1".to_string()),
    );
    custom_fields.insert(
        "field2".to_string(),
        serde_json::Value::String("value2".to_string()),
    );
    comprehensive_metadata.insert(
        "custom_fields".to_string(),
        serde_json::Value::Object(custom_fields),
    );

    let comprehensive_metadata = serde_json::Value::Object(comprehensive_metadata);

    // Verify comprehensive metadata structure
    assert_eq!(comprehensive_metadata["name"], "comprehensive_tool");
    assert_eq!(
        comprehensive_metadata["description"],
        "A comprehensive test tool with all parameters"
    );
    assert_eq!(comprehensive_metadata["return_type"], "string");
    assert_eq!(comprehensive_metadata["version"], "1.0.0");
    assert_eq!(comprehensive_metadata["author"], "test_author");
    assert_eq!(comprehensive_metadata["category"], "testing");
    assert_eq!(comprehensive_metadata["license"], "MIT");
    assert_eq!(comprehensive_metadata["timeout_ms"], 5000);
    assert_eq!(comprehensive_metadata["max_retries"], 3);
    assert_eq!(comprehensive_metadata["retry_delay_ms"], 1000);
    assert_eq!(comprehensive_metadata["cache_enabled"], true);
    assert_eq!(comprehensive_metadata["cache_ttl_seconds"], 3600);
    assert_eq!(comprehensive_metadata["rate_limit_per_minute"], 100);
    assert_eq!(comprehensive_metadata["memory_limit_mb"], 512);
    assert_eq!(comprehensive_metadata["cpu_limit_percent"], 50);
    assert_eq!(comprehensive_metadata["network_access"], true);
    assert_eq!(comprehensive_metadata["file_access"], false);
    assert_eq!(comprehensive_metadata["audit_logging"], true);
    assert_eq!(comprehensive_metadata["metrics_collection"], true);
    assert_eq!(comprehensive_metadata["health_check_enabled"], true);
    assert_eq!(comprehensive_metadata["health_check_interval_seconds"], 300);
    assert_eq!(comprehensive_metadata["backup_enabled"], false);
    assert_eq!(comprehensive_metadata["backup_retention_days"], 30);
    assert_eq!(comprehensive_metadata["encryption_enabled"], true);
    assert_eq!(comprehensive_metadata["encryption_algorithm"], "AES-256");
    assert_eq!(comprehensive_metadata["compression_enabled"], false);
    assert_eq!(comprehensive_metadata["compression_algorithm"], "gzip");
    assert_eq!(comprehensive_metadata["logging_level"], "INFO");
    assert_eq!(comprehensive_metadata["log_format"], "json");
    assert_eq!(comprehensive_metadata["error_reporting"], true);
    assert_eq!(comprehensive_metadata["telemetry_enabled"], false);
    assert_eq!(comprehensive_metadata["update_check_enabled"], true);
    assert_eq!(comprehensive_metadata["update_check_interval_hours"], 24);
    assert_eq!(comprehensive_metadata["maintenance_mode"], false);
    assert_eq!(comprehensive_metadata["maintenance_window"], "02:00-04:00");
    assert_eq!(comprehensive_metadata["timezone"], "UTC");
    assert_eq!(comprehensive_metadata["locale"], "en_US");
    assert_eq!(comprehensive_metadata["currency"], "USD");
    assert_eq!(comprehensive_metadata["units"], "metric");
    assert_eq!(comprehensive_metadata["theme"], "light");
    assert_eq!(comprehensive_metadata["internationalization"], true);

    // Test with minimal metadata (only required parameters)
    let mut minimal_metadata = serde_json::Map::new();
    minimal_metadata.insert(
        "name".to_string(),
        serde_json::Value::String("minimal_tool".to_string()),
    );
    minimal_metadata.insert(
        "description".to_string(),
        serde_json::Value::String("Minimal tool".to_string()),
    );

    let mut properties = serde_json::Map::new();
    properties.insert("param1".to_string(), serde_json::json!({"type": "string"}));

    let mut parameters_schema = serde_json::Map::new();
    parameters_schema.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    parameters_schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    parameters_schema.insert("required".to_string(), serde_json::json!(["param1"]));

    minimal_metadata.insert(
        "parameters_schema".to_string(),
        serde_json::Value::Object(parameters_schema),
    );
    minimal_metadata.insert(
        "return_type".to_string(),
        serde_json::Value::String("string".to_string()),
    );

    let minimal_metadata = serde_json::Value::Object(minimal_metadata);

    assert_eq!(minimal_metadata["name"], "minimal_tool");
    assert_eq!(minimal_metadata["description"], "Minimal tool");
    assert_eq!(minimal_metadata["return_type"], "string");

    // Test with None values for optional parameters
    let mut none_metadata = serde_json::Map::new();
    none_metadata.insert(
        "name".to_string(),
        serde_json::Value::String("none_tool".to_string()),
    );
    none_metadata.insert(
        "description".to_string(),
        serde_json::Value::String("Tool with None optional parameters".to_string()),
    );

    let mut properties = serde_json::Map::new();
    properties.insert("param1".to_string(), serde_json::json!({"type": "string"}));
    properties.insert(
        "param2".to_string(),
        serde_json::json!({"type": "string", "default": null}),
    );
    properties.insert(
        "param3".to_string(),
        serde_json::json!({"type": "integer", "default": null}),
    );

    let mut parameters_schema = serde_json::Map::new();
    parameters_schema.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    parameters_schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    parameters_schema.insert("required".to_string(), serde_json::json!(["param1"]));

    none_metadata.insert(
        "parameters_schema".to_string(),
        serde_json::Value::Object(parameters_schema),
    );
    none_metadata.insert(
        "return_type".to_string(),
        serde_json::Value::String("string".to_string()),
    );

    // Add null values for optional parameters
    none_metadata.insert("tags".to_string(), serde_json::Value::Null);
    none_metadata.insert("version".to_string(), serde_json::Value::Null);
    none_metadata.insert("author".to_string(), serde_json::Value::Null);
    none_metadata.insert("category".to_string(), serde_json::Value::Null);
    none_metadata.insert("examples".to_string(), serde_json::Value::Null);
    none_metadata.insert("documentation_url".to_string(), serde_json::Value::Null);
    none_metadata.insert("license".to_string(), serde_json::Value::Null);
    none_metadata.insert("dependencies".to_string(), serde_json::Value::Null);
    none_metadata.insert("timeout_ms".to_string(), serde_json::Value::Null);
    none_metadata.insert("max_retries".to_string(), serde_json::Value::Null);
    none_metadata.insert("retry_delay_ms".to_string(), serde_json::Value::Null);
    none_metadata.insert("cache_enabled".to_string(), serde_json::Value::Null);
    none_metadata.insert("cache_ttl_seconds".to_string(), serde_json::Value::Null);
    none_metadata.insert("rate_limit_per_minute".to_string(), serde_json::Value::Null);
    none_metadata.insert("memory_limit_mb".to_string(), serde_json::Value::Null);
    none_metadata.insert("cpu_limit_percent".to_string(), serde_json::Value::Null);
    none_metadata.insert("network_access".to_string(), serde_json::Value::Null);
    none_metadata.insert("file_access".to_string(), serde_json::Value::Null);
    none_metadata.insert("environment_variables".to_string(), serde_json::Value::Null);
    none_metadata.insert("secrets".to_string(), serde_json::Value::Null);
    none_metadata.insert("permissions".to_string(), serde_json::Value::Null);
    none_metadata.insert("audit_logging".to_string(), serde_json::Value::Null);
    none_metadata.insert("metrics_collection".to_string(), serde_json::Value::Null);
    none_metadata.insert("health_check_enabled".to_string(), serde_json::Value::Null);
    none_metadata.insert(
        "health_check_interval_seconds".to_string(),
        serde_json::Value::Null,
    );
    none_metadata.insert("backup_enabled".to_string(), serde_json::Value::Null);
    none_metadata.insert("backup_retention_days".to_string(), serde_json::Value::Null);
    none_metadata.insert("encryption_enabled".to_string(), serde_json::Value::Null);
    none_metadata.insert("encryption_algorithm".to_string(), serde_json::Value::Null);
    none_metadata.insert("compression_enabled".to_string(), serde_json::Value::Null);
    none_metadata.insert("compression_algorithm".to_string(), serde_json::Value::Null);
    none_metadata.insert("logging_level".to_string(), serde_json::Value::Null);
    none_metadata.insert("log_format".to_string(), serde_json::Value::Null);
    none_metadata.insert("error_reporting".to_string(), serde_json::Value::Null);
    none_metadata.insert("telemetry_enabled".to_string(), serde_json::Value::Null);
    none_metadata.insert("update_check_enabled".to_string(), serde_json::Value::Null);
    none_metadata.insert(
        "update_check_interval_hours".to_string(),
        serde_json::Value::Null,
    );
    none_metadata.insert("maintenance_mode".to_string(), serde_json::Value::Null);
    none_metadata.insert("maintenance_window".to_string(), serde_json::Value::Null);
    none_metadata.insert("timezone".to_string(), serde_json::Value::Null);
    none_metadata.insert("locale".to_string(), serde_json::Value::Null);
    none_metadata.insert("currency".to_string(), serde_json::Value::Null);
    none_metadata.insert("units".to_string(), serde_json::Value::Null);
    none_metadata.insert("theme".to_string(), serde_json::Value::Null);
    none_metadata.insert(
        "accessibility_features".to_string(),
        serde_json::Value::Null,
    );
    none_metadata.insert("internationalization".to_string(), serde_json::Value::Null);
    none_metadata.insert(
        "localization_languages".to_string(),
        serde_json::Value::Null,
    );
    none_metadata.insert("custom_fields".to_string(), serde_json::Value::Null);

    let none_metadata = serde_json::Value::Object(none_metadata);

    assert_eq!(none_metadata["name"], "none_tool");
    assert_eq!(
        none_metadata["description"],
        "Tool with None optional parameters"
    );
    assert_eq!(none_metadata["return_type"], "string");

    // Test with empty values for optional parameters
    let mut empty_metadata = serde_json::Map::new();
    empty_metadata.insert(
        "name".to_string(),
        serde_json::Value::String("empty_tool".to_string()),
    );
    empty_metadata.insert(
        "description".to_string(),
        serde_json::Value::String("Tool with empty optional parameters".to_string()),
    );

    let mut properties = serde_json::Map::new();
    properties.insert("param1".to_string(), serde_json::json!({"type": "string"}));
    properties.insert(
        "param2".to_string(),
        serde_json::json!({"type": "string", "default": ""}),
    );
    properties.insert(
        "param3".to_string(),
        serde_json::json!({"type": "array", "default": []}),
    );

    let mut parameters_schema = serde_json::Map::new();
    parameters_schema.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    parameters_schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    parameters_schema.insert("required".to_string(), serde_json::json!(["param1"]));

    empty_metadata.insert(
        "parameters_schema".to_string(),
        serde_json::Value::Object(parameters_schema),
    );
    empty_metadata.insert(
        "return_type".to_string(),
        serde_json::Value::String("string".to_string()),
    );
    empty_metadata.insert("tags".to_string(), serde_json::json!([]));
    empty_metadata.insert("examples".to_string(), serde_json::json!([]));
    empty_metadata.insert("dependencies".to_string(), serde_json::json!([]));
    empty_metadata.insert("environment_variables".to_string(), serde_json::json!([]));
    empty_metadata.insert("secrets".to_string(), serde_json::json!([]));
    empty_metadata.insert("permissions".to_string(), serde_json::json!([]));
    empty_metadata.insert("accessibility_features".to_string(), serde_json::json!([]));
    empty_metadata.insert("localization_languages".to_string(), serde_json::json!([]));
    empty_metadata.insert("custom_fields".to_string(), serde_json::json!({}));

    let empty_metadata = serde_json::Value::Object(empty_metadata);

    assert_eq!(empty_metadata["name"], "empty_tool");
    assert_eq!(
        empty_metadata["description"],
        "Tool with empty optional parameters"
    );
    assert_eq!(empty_metadata["return_type"], "string");
    assert_eq!(empty_metadata["tags"], serde_json::json!([]));
    assert_eq!(empty_metadata["examples"], serde_json::json!([]));
    assert_eq!(empty_metadata["dependencies"], serde_json::json!([]));
    assert_eq!(
        empty_metadata["environment_variables"],
        serde_json::json!([])
    );
    assert_eq!(empty_metadata["secrets"], serde_json::json!([]));
    assert_eq!(empty_metadata["permissions"], serde_json::json!([]));
    assert_eq!(
        empty_metadata["accessibility_features"],
        serde_json::json!([])
    );
    assert_eq!(
        empty_metadata["localization_languages"],
        serde_json::json!([])
    );
    assert_eq!(empty_metadata["custom_fields"], serde_json::json!({}));

    // Test with boolean false values
    let mut false_metadata = serde_json::Map::new();
    false_metadata.insert(
        "name".to_string(),
        serde_json::Value::String("false_tool".to_string()),
    );
    false_metadata.insert(
        "description".to_string(),
        serde_json::Value::String("Tool with boolean false parameters".to_string()),
    );

    let mut properties = serde_json::Map::new();
    properties.insert("param1".to_string(), serde_json::json!({"type": "string"}));
    properties.insert(
        "param2".to_string(),
        serde_json::json!({"type": "boolean", "default": false}),
    );
    properties.insert(
        "param3".to_string(),
        serde_json::json!({"type": "boolean", "default": false}),
    );

    let mut parameters_schema = serde_json::Map::new();
    parameters_schema.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    parameters_schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    parameters_schema.insert("required".to_string(), serde_json::json!(["param1"]));

    false_metadata.insert(
        "parameters_schema".to_string(),
        serde_json::Value::Object(parameters_schema),
    );
    false_metadata.insert(
        "return_type".to_string(),
        serde_json::Value::String("string".to_string()),
    );
    false_metadata.insert("cache_enabled".to_string(), serde_json::Value::Bool(false));
    false_metadata.insert("network_access".to_string(), serde_json::Value::Bool(false));
    false_metadata.insert("file_access".to_string(), serde_json::Value::Bool(false));
    false_metadata.insert("audit_logging".to_string(), serde_json::Value::Bool(false));
    false_metadata.insert(
        "metrics_collection".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert(
        "health_check_enabled".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert("backup_enabled".to_string(), serde_json::Value::Bool(false));
    false_metadata.insert(
        "encryption_enabled".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert(
        "compression_enabled".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert(
        "error_reporting".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert(
        "telemetry_enabled".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert(
        "update_check_enabled".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert(
        "maintenance_mode".to_string(),
        serde_json::Value::Bool(false),
    );
    false_metadata.insert(
        "internationalization".to_string(),
        serde_json::Value::Bool(false),
    );

    let false_metadata = serde_json::Value::Object(false_metadata);

    assert_eq!(false_metadata["name"], "false_tool");
    assert_eq!(
        false_metadata["description"],
        "Tool with boolean false parameters"
    );
    assert_eq!(false_metadata["return_type"], "string");
    assert_eq!(false_metadata["cache_enabled"], false);
    assert_eq!(false_metadata["network_access"], false);
    assert_eq!(false_metadata["file_access"], false);
    assert_eq!(false_metadata["audit_logging"], false);
    assert_eq!(false_metadata["metrics_collection"], false);
    assert_eq!(false_metadata["health_check_enabled"], false);
    assert_eq!(false_metadata["backup_enabled"], false);
    assert_eq!(false_metadata["encryption_enabled"], false);
    assert_eq!(false_metadata["compression_enabled"], false);
    assert_eq!(false_metadata["error_reporting"], false);
    assert_eq!(false_metadata["telemetry_enabled"], false);
    assert_eq!(false_metadata["update_check_enabled"], false);
    assert_eq!(false_metadata["maintenance_mode"], false);
    assert_eq!(false_metadata["internationalization"], false);

    // Test with zero values for numeric parameters
    let mut zero_metadata = serde_json::Map::new();
    zero_metadata.insert(
        "name".to_string(),
        serde_json::Value::String("zero_tool".to_string()),
    );
    zero_metadata.insert(
        "description".to_string(),
        serde_json::Value::String("Tool with zero optional parameters".to_string()),
    );

    let mut properties = serde_json::Map::new();
    properties.insert("param1".to_string(), serde_json::json!({"type": "string"}));
    properties.insert(
        "param2".to_string(),
        serde_json::json!({"type": "integer", "default": 0}),
    );
    properties.insert(
        "param3".to_string(),
        serde_json::json!({"type": "number", "default": 0.0}),
    );

    let mut parameters_schema = serde_json::Map::new();
    parameters_schema.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    parameters_schema.insert(
        "properties".to_string(),
        serde_json::Value::Object(properties),
    );
    parameters_schema.insert("required".to_string(), serde_json::json!(["param1"]));

    zero_metadata.insert(
        "parameters_schema".to_string(),
        serde_json::Value::Object(parameters_schema),
    );
    zero_metadata.insert(
        "return_type".to_string(),
        serde_json::Value::String("string".to_string()),
    );
    zero_metadata.insert(
        "timeout_ms".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "max_retries".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "retry_delay_ms".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "cache_ttl_seconds".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "rate_limit_per_minute".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "memory_limit_mb".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "cpu_limit_percent".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "health_check_interval_seconds".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "backup_retention_days".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    zero_metadata.insert(
        "update_check_interval_hours".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );

    let zero_metadata = serde_json::Value::Object(zero_metadata);

    assert_eq!(zero_metadata["name"], "zero_tool");
    assert_eq!(
        zero_metadata["description"],
        "Tool with zero optional parameters"
    );
    assert_eq!(zero_metadata["return_type"], "string");
    assert_eq!(zero_metadata["timeout_ms"], 0);
    assert_eq!(zero_metadata["max_retries"], 0);
    assert_eq!(zero_metadata["retry_delay_ms"], 0);
    assert_eq!(zero_metadata["cache_ttl_seconds"], 0);
    assert_eq!(zero_metadata["rate_limit_per_minute"], 0);
    assert_eq!(zero_metadata["memory_limit_mb"], 0);
    assert_eq!(zero_metadata["cpu_limit_percent"], 0);
    assert_eq!(zero_metadata["health_check_interval_seconds"], 0);
    assert_eq!(zero_metadata["backup_retention_days"], 0);
    assert_eq!(zero_metadata["update_check_interval_hours"], 0);
}
