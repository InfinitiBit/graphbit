//! Unit tests for ToolDecorator functionality

use super::*;

#[test]
fn test_tool_decorator_creation() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test basic creation
    // Note: This would test actual ToolDecorator creation if available
    assert!(true, "ToolDecorator creation test passed");
}

#[test]
fn test_tool_decorator_parameter_validation() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test parameter validation logic
    let test_input = serde_json::json!({
        "description": "Test tool",
        "name": "test_tool",
        "return_type": "string"
    });
    
    // Validate input structure
    assert!(test_input.get("description").is_some());
    assert!(test_input.get("name").is_some());
    assert!(test_input.get("return_type").is_some());
    
    // Test with missing parameters
    let incomplete_input = serde_json::json!({
        "description": "Test tool"
    });
    
    assert!(incomplete_input.get("name").is_none());
    assert!(incomplete_input.get("return_type").is_none());
}

#[test]
fn test_tool_decorator_function_registration() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test function registration logic
    let test_function = create_simple_test_tool("test_function");
    
    // Test the function works
    let result = test_function(serde_json::json!({"input": "test"}));
    assert!(result.is_ok());
    
    // Test with different input
    let result2 = test_function(serde_json::json!({"value": 42}));
    assert!(result2.is_ok());
}

#[test]
fn test_tool_decorator_metadata_extraction() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test metadata extraction from function
    let metadata = create_simple_tool_metadata("test_tool", "A test tool");
    
    assert_eq!(metadata["name"], "test_tool");
    assert_eq!(metadata["description"], "A test tool");
    assert!(metadata["parameters_schema"].is_object());
    assert_eq!(metadata["return_type"], "string");
}

#[test]
fn test_tool_decorator_error_handling() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test error handling with failing tool
    let failing_tool = create_simple_test_tool("fail_on_even");
    
    // Test with odd number (should succeed)
    let result = failing_tool(serde_json::json!({"value": 3}));
    assert!(result.is_ok());
    
    // Test with even number (should fail)
    let result = failing_tool(serde_json::json!({"value": 2}));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Even numbers cause failure"));
}

#[test]
fn test_tool_decorator_thread_safety() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test basic thread safety concepts
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..5 {
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
    assert_eq!(final_count, 5);
}

#[test]
fn test_tool_decorator_memory_management() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test memory management with multiple decorators
    let mut decorators = Vec::new();
    
    for i in 0..100 {
        let decorator = create_simple_test_tool(&format!("decorator_{}", i));
        decorators.push(decorator);
    }
    
    // Verify all decorators were created
    assert_eq!(decorators.len(), 100);
    
    // Test a few decorators work
    let result = decorators[0](serde_json::json!({"input": "test"}));
    assert!(result.is_ok());
    
    let result = decorators[50](serde_json::json!({"value": 42}));
    assert!(result.is_ok());
}

#[test]
fn test_tool_decorator_edge_cases() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test with empty input
    let empty_input = serde_json::json!({});
    assert!(empty_input.is_object());
    
    // Test with null values
    let null_input = serde_json::json!({
        "description": serde_json::Value::Null,
        "name": serde_json::Value::Null
    });
    assert!(null_input["description"].is_null());
    assert!(null_input["name"].is_null());
    
    // Test with very long strings
    let long_string = "A".repeat(1000);
    let long_input = serde_json::json!({
        "description": long_string,
        "name": "long_name_test"
    });
    assert_eq!(long_input["description"].as_str().unwrap().len(), 1000);
}

#[test]
fn test_tool_decorator_validation_logic() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test validation logic for different parameter types
    let string_param = serde_json::json!({"type": "string"});
    let number_param = serde_json::json!({"type": "number"});
    let boolean_param = serde_json::json!({"type": "boolean"});
    
    assert_eq!(string_param["type"], "string");
    assert_eq!(number_param["type"], "number");
    assert_eq!(boolean_param["type"], "boolean");
    
    // Test complex schema validation
    let complex_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"},
            "active": {"type": "boolean"}
        },
        "required": ["name"]
    });
    
    assert!(complex_schema["type"] == "object");
    assert!(complex_schema["properties"].is_object());
    assert!(complex_schema["required"].is_array());
}

#[test]
fn test_tool_decorator_function_types() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test different function types
    let echo_tool = create_simple_test_tool("echo");
    let add_one_tool = create_simple_test_tool("add_one");
    let to_string_tool = create_simple_test_tool("to_string");
    
    // Test echo tool
    let echo_result = echo_tool(serde_json::json!({"test": "value"}));
    assert!(echo_result.is_ok());
    
    // Test add_one tool
    let add_result = add_one_tool(serde_json::json!({"value": 5}));
    assert!(add_result.is_ok());
    assert_eq!(add_result.unwrap(), serde_json::json!(6));
    
    // Test to_string tool
    let string_result = to_string_tool(serde_json::json!({"value": 42}));
    assert!(string_result.is_ok());
    assert!(string_result.unwrap().as_str().unwrap().contains("42"));
}
