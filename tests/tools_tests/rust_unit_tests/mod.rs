//! Rust unit tests for GraphBit tools module
//!
//! This module contains unit tests for individual components of the GraphBit tools system.
//! These tests focus on isolated functionality without external dependencies.

pub mod tool_decorator_tests;
pub mod tool_executor_tests;
pub mod tool_registry_tests;
pub mod tool_result_tests;

/// Check if tools module is available for unit testing
pub fn has_tools_module() -> bool {
    // This would check if the tools module is compiled and available
    // For now, we'll assume it's available
    true
}

/// Get test configuration for tools testing
pub fn get_tools_test_config() -> ToolsTestConfig {
    ToolsTestConfig {
        max_concurrent_tools: 10,
        max_execution_time_ms: 5000,
        max_tool_calls: 100,
    }
}

/// Configuration for tools testing
#[derive(Debug, Clone)]
pub struct ToolsTestConfig {
    pub max_concurrent_tools: usize,
    pub max_execution_time_ms: u64,
    pub max_tool_calls: usize,
}

/// Skip test if tools module is not available
pub fn skip_if_no_tools(reason: &str) {
    if !has_tools_module() {
        println!("Skipping test - tools module not available: {}", reason);
        panic!("TEST_SKIP");
    }
}

/// Helper function to create test tool functions for unit tests
pub fn create_simple_test_tool(
    name: &str,
) -> Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync> {
    let name = name.to_string();
    Box::new(move |input: serde_json::Value| match name.as_str() {
        "echo" => Ok(input),
        "add_one" => {
            if let Some(value) = input.get("value").and_then(|v| v.as_i64()) {
                Ok(serde_json::json!(value + 1))
            } else {
                Err("Invalid parameter for add_one".to_string())
            }
        }
        "to_string" => Ok(serde_json::json!(input.to_string())),
        "fail_on_even" => {
            if let Some(value) = input.get("value").and_then(|v| v.as_i64()) {
                if value % 2 == 0 {
                    Err("Even numbers cause failure".to_string())
                } else {
                    Ok(serde_json::json!(value))
                }
            } else {
                Err("Invalid parameter for fail_on_even".to_string())
            }
        }
        _ => Err(format!("Unknown test tool: {}", name)),
    })
}

/// Helper function to create simple test tool metadata
pub fn create_simple_tool_metadata(name: &str, description: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "description": description,
        "parameters_schema": {
            "type": "object",
            "properties": {
                "input": {"type": "string"}
            }
        },
        "return_type": "string"
    })
}

/// Helper function to validate basic tool result structure
pub fn validate_basic_tool_result(
    result: &serde_json::Value,
    expected_success: bool,
    expected_tool_name: &str,
) -> Result<(), String> {
    if let Some(success) = result.get("success").and_then(|v| v.as_bool()) {
        if success != expected_success {
            return Err(format!(
                "Expected success={}, got {}",
                expected_success, success
            ));
        }
    } else {
        return Err("Missing 'success' field in result".to_string());
    }

    if let Some(tool_name) = result.get("tool_name").and_then(|v| v.as_str()) {
        if tool_name != expected_tool_name {
            return Err(format!(
                "Expected tool_name={}, got {}",
                expected_tool_name, tool_name
            ));
        }
    } else {
        return Err("Missing 'tool_name' field in result".to_string());
    }

    if expected_success {
        if result.get("output").is_none() {
            return Err("Missing 'output' field in successful result".to_string());
        }
    } else {
        if result.get("error").is_none() {
            return Err("Missing 'error' field in failed result".to_string());
        }
    }

    Ok(())
}

/// Helper function to create test execution context for unit tests
pub fn create_simple_execution_context() -> serde_json::Value {
    serde_json::json!({
        "test_mode": true,
        "session_id": "unit_test_session",
        "timestamp": chrono::Utc::now().timestamp()
    })
}

/// Helper function to create test data for unit tests
pub fn create_unit_test_data() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({"id": 1, "value": "test1"}),
        serde_json::json!({"id": 2, "value": "test2"}),
        serde_json::json!({"id": 3, "value": "test3"}),
    ]
}
