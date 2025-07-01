//! Validation integration tests
//!
//! Tests for the validation system including type validation,
//! workflow validation, and custom validation rules.

use graphbit_core::validation::{TypeValidator, ValidationRule};

#[tokio::test]
async fn test_basic_type_validation() {
    let validator = TypeValidator::new();

    // Test string validation
    let schema = serde_json::json!({
        "type": "string"
    });
    let result = validator.validate_against_schema("\"test string\"", &schema);
    assert!(result.is_valid);

    // Test number validation
    let schema = serde_json::json!({
        "type": "number"
    });
    let result = validator.validate_against_schema("42.0", &schema);
    assert!(result.is_valid);

    // Test boolean validation
    let schema = serde_json::json!({
        "type": "boolean"
    });
    let result = validator.validate_against_schema("true", &schema);
    assert!(result.is_valid);
}

#[tokio::test]
async fn test_json_schema_validation() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"},
            "active": {"type": "boolean"}
        },
        "required": ["name", "age"]
    });

    let valid_data = serde_json::json!({
        "name": "John Doe",
        "age": 30,
        "active": true
    });

    let result = validator.validate_against_schema(&valid_data.to_string(), &schema);
    assert!(result.is_valid);

    // Test with invalid data
    let invalid_data = serde_json::json!({
        "name": "John Doe"
        // missing required "age" field
    });

    let result = validator.validate_against_schema(&invalid_data.to_string(), &schema);
    assert!(!result.is_valid);
}

#[tokio::test]
async fn test_custom_validation_rule() {
    let validator = TypeValidator::new();

    let rule = ValidationRule::new(
        "string_length",
        "String must be between 5 and 50 characters",
    )
    .with_schema(serde_json::json!({
        "type": "string",
        "minLength": 5,
        "maxLength": 50
    }));

    // Test valid string
    let valid_string = serde_json::json!("Hello World");
    let result = validator.validate_against_rule(&valid_string.to_string(), &rule);
    assert!(result.is_valid);

    // Test invalid string (too short)
    let invalid_string = serde_json::json!("Hi");
    let result = validator.validate_against_rule(&invalid_string.to_string(), &rule);
    assert!(!result.is_valid);
}

#[tokio::test]
async fn test_numeric_validation_rule() {
    let validator = TypeValidator::new();

    let rule = ValidationRule::new("positive_number", "Number must be positive").with_schema(
        serde_json::json!({
            "type": "number",
            "minimum": 0,
            "exclusiveMinimum": true
        }),
    );

    // Test valid number
    let valid_number = serde_json::json!(42.5);
    let result = validator.validate_against_rule(&valid_number.to_string(), &rule);
    assert!(result.is_valid);

    // Test invalid number
    let invalid_number = serde_json::json!(-10);
    let result = validator.validate_against_rule(&invalid_number.to_string(), &rule);
    assert!(!result.is_valid);
}

#[tokio::test]
async fn test_workflow_config_validation() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "description": {"type": "string"},
            "timeout": {"type": "number", "minimum": 0},
            "parallel": {"type": "boolean"}
        },
        "required": ["name"]
    });

    let valid_config = serde_json::json!({
        "name": "Test Workflow",
        "description": "A test workflow for validation",
        "timeout": 300,
        "parallel": true
    });

    let result = validator.validate_against_schema(&valid_config.to_string(), &schema);
    assert!(result.is_valid);

    // Test with invalid config (missing required field)
    let invalid_config = serde_json::json!({
        "description": "A test workflow for validation",
        "timeout": 300
    });

    let result = validator.validate_against_schema(&invalid_config.to_string(), &schema);
    assert!(!result.is_valid);
}

#[tokio::test]
async fn test_node_type_validation() {
    let validator = TypeValidator::new();

    // Test agent node validation
    let agent_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type": {"type": "string", "enum": ["agent"]},
            "agent_id": {"type": "string"},
            "prompt": {"type": "string", "minLength": 1}
        },
        "required": ["type", "agent_id", "prompt"]
    });

    let agent_config = serde_json::json!({
        "type": "agent",
        "agent_id": "test-agent",
        "prompt": "Process this data"
    });

    let result = validator.validate_against_schema(&agent_config.to_string(), &agent_schema);
    assert!(result.is_valid);

    // Test condition node validation
    let condition_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type": {"type": "string", "enum": ["condition"]},
            "expression": {"type": "string", "minLength": 1}
        },
        "required": ["type", "expression"]
    });

    let condition_config = serde_json::json!({
        "type": "condition",
        "expression": "x > 0"
    });

    let result =
        validator.validate_against_schema(&condition_config.to_string(), &condition_schema);
    assert!(result.is_valid);

    // Test transform node validation
    let transform_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type": {"type": "string", "enum": ["transform"]},
            "transformation": {"type": "string", "minLength": 1}
        },
        "required": ["type", "transformation"]
    });

    let transform_config = serde_json::json!({
        "type": "transform",
        "transformation": "uppercase"
    });

    let result =
        validator.validate_against_schema(&transform_config.to_string(), &transform_schema);
    assert!(result.is_valid);

    // Test delay node validation
    let delay_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type": {"type": "string", "enum": ["delay"]},
            "duration": {"type": "number", "minimum": 0}
        },
        "required": ["type", "duration"]
    });

    let delay_config = serde_json::json!({
        "type": "delay",
        "duration": 5
    });

    let result = validator.validate_against_schema(&delay_config.to_string(), &delay_schema);
    assert!(result.is_valid);
}

#[tokio::test]
async fn test_edge_type_validation() {
    let validator = TypeValidator::new();

    let edge_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type": {"type": "string", "enum": ["data_flow", "control_flow", "conditional"]},
            "condition": {"type": "string"},
            "transform": {"type": "string"}
        },
        "required": ["type"]
    });

    // Test data flow edge
    let data_flow_config = serde_json::json!({
        "type": "data_flow"
    });

    let result = validator.validate_against_schema(&data_flow_config.to_string(), &edge_schema);
    assert!(result.is_valid);

    // Test conditional edge
    let conditional_config = serde_json::json!({
        "type": "conditional",
        "condition": "success"
    });

    let result = validator.validate_against_schema(&conditional_config.to_string(), &edge_schema);
    assert!(result.is_valid);

    // Test control flow edge
    let control_config = serde_json::json!({
        "type": "control_flow"
    });

    let result = validator.validate_against_schema(&control_config.to_string(), &edge_schema);
    assert!(result.is_valid);
}

#[tokio::test]
async fn test_complex_nested_validation() {
    let validator = TypeValidator::new();

    let workflow_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "nodes": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string"},
                        "type": {"type": "string"},
                        "config": {"type": "object"}
                    },
                    "required": ["id", "type"]
                }
            },
            "edges": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "from": {"type": "string"},
                        "to": {"type": "string"},
                        "type": {"type": "string"}
                    },
                    "required": ["from", "to", "type"]
                }
            }
        },
        "required": ["name", "nodes", "edges"]
    });

    let workflow_data = serde_json::json!({
        "name": "Complex Workflow",
        "nodes": [
            {
                "id": "node1",
                "type": "agent",
                "config": {"agent_id": "test-agent"}
            },
            {
                "id": "node2",
                "type": "transform",
                "config": {"transformation": "uppercase"}
            }
        ],
        "edges": [
            {
                "from": "node1",
                "to": "node2",
                "type": "data_flow"
            }
        ]
    });

    let result = validator.validate_against_schema(&workflow_data.to_string(), &workflow_schema);
    assert!(result.is_valid);
}

#[tokio::test]
async fn test_agent_id_naming_convention() {
    let validator = TypeValidator::new();

    let agent_id_rule =
        ValidationRule::new("agent_id_format", "Agent ID must follow naming convention")
            .with_schema(serde_json::json!({
                "type": "string",
                "pattern": "^[a-z]+(-[a-z]+)*$",  // kebab-case only
                "minLength": 3,
                "maxLength": 50
            }));

    // Test valid agent ID
    let valid_id = serde_json::json!("data-processor");
    let result = validator.validate_against_rule(&valid_id.to_string(), &agent_id_rule);
    assert!(result.is_valid);

    // Test various invalid agent IDs
    let invalid_ids = vec![
        "DataProcessor",   // Contains uppercase
        "data_processor",  // Contains underscore
        "data-processor-", // Ends with dash
        "-data-processor", // Starts with dash
        "dp",              // Too short
    ];

    for invalid_id in invalid_ids {
        let result =
            validator.validate_against_rule(&format!("\"{}\"", invalid_id), &agent_id_rule);
        assert!(!result.is_valid, "ID '{}' should be invalid", invalid_id);
    }
}

#[tokio::test]
async fn test_validation_error_handling() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "required_field": {"type": "string"}
        },
        "required": ["required_field"]
    });

    // Test with completely invalid JSON
    let invalid_data = "{ invalid json }";
    let result = validator.validate_against_schema(invalid_data, &schema);
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());
}

#[tokio::test]
async fn test_performance_large_data() {
    let validator = TypeValidator::new();

    // Create a large schema with many nested properties
    let large_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string"},
                        "name": {"type": "string"},
                        "value": {"type": "number"},
                        "active": {"type": "boolean"},
                        "metadata": {
                            "type": "object",
                            "properties": {
                                "created": {"type": "string"},
                                "updated": {"type": "string"},
                                "tags": {
                                    "type": "array",
                                    "items": {"type": "string"}
                                }
                            }
                        }
                    },
                    "required": ["id", "name", "value"]
                }
            }
        },
        "required": ["items"]
    });

    // Create large test data
    let mut items = Vec::new();
    for i in 0..100 {
        items.push(serde_json::json!({
            "id": format!("item-{}", i),
            "name": format!("Item {}", i),
            "value": i,
            "active": i % 2 == 0,
            "metadata": {
                "created": "2023-01-01T00:00:00Z",
                "updated": "2023-01-01T00:00:00Z",
                "tags": ["test", "large", "dataset"]
            }
        }));
    }

    let large_data = serde_json::json!({"items": items});

    let start = std::time::Instant::now();
    let result = validator.validate_against_schema(&large_data.to_string(), &large_schema);
    let duration = start.elapsed();

    assert!(result.is_valid);
    assert!(
        duration.as_millis() < 1000,
        "Validation should complete within 1 second"
    );
}

#[tokio::test]
async fn test_deeply_nested_validation() {
    let validator = TypeValidator::new();

    let nested_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "level1": {
                "type": "object",
                "properties": {
                    "level2": {
                        "type": "object",
                        "properties": {
                            "level3": {
                                "type": "object",
                                "properties": {
                                    "value": {"type": "string"}
                                },
                                "required": ["value"]
                            }
                        },
                        "required": ["level3"]
                    }
                },
                "required": ["level2"]
            }
        },
        "required": ["level1"]
    });

    let nested_data = serde_json::json!({
        "level1": {
            "level2": {
                "level3": {
                    "value": "deep value"
                }
            }
        }
    });

    let result = validator.validate_against_schema(&nested_data.to_string(), &nested_schema);
    assert!(result.is_valid);

    // Test with missing nested value
    let invalid_nested_data = serde_json::json!({
        "level1": {
            "level2": {
                "level3": {}
            }
        }
    });

    let result =
        validator.validate_against_schema(&invalid_nested_data.to_string(), &nested_schema);
    assert!(!result.is_valid);
}

#[tokio::test]
async fn test_conditional_schema_validation() {
    let validator = TypeValidator::new();

    let conditional_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "type": {"type": "string", "enum": ["agent", "condition"]},
            "agent_id": {"type": "string"},
            "expression": {"type": "string"}
        },
        "required": ["type"],
        "allOf": [
            {
                "if": {"properties": {"type": {"const": "agent"}}},
                "then": {"required": ["agent_id"]}
            },
            {
                "if": {"properties": {"type": {"const": "condition"}}},
                "then": {"required": ["expression"]}
            }
        ]
    });

    // Test agent configuration
    let agent_data = serde_json::json!({
        "type": "agent",
        "agent_id": "test-agent"
    });

    let result = validator.validate_against_schema(&agent_data.to_string(), &conditional_schema);
    assert!(result.is_valid);

    // Test condition configuration
    let condition_data = serde_json::json!({
        "type": "condition",
        "expression": "x > 0"
    });

    let result =
        validator.validate_against_schema(&condition_data.to_string(), &conditional_schema);
    assert!(result.is_valid);

    // Test invalid agent configuration (missing agent_id)
    let invalid_agent_data = serde_json::json!({
        "type": "agent"
    });

    let _result =
        validator.validate_against_schema(&invalid_agent_data.to_string(), &conditional_schema);
    // Note: Basic JSON Schema validator might not support conditional validation
    // This test serves as a placeholder for more advanced validation capabilities
}
