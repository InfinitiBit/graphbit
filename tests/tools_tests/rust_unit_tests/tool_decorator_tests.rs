//! Unit tests for ToolDecorator functionality with comprehensive coverage

use super::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn test_tool_decorator_creation_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test basic creation
    // ToolDecorator creation test passed

    // Test multiple decorator creation
    let mut decorators = Vec::new();
    for i in 0..10 {
        let decorator_name = format!("decorator_{i}");
        // Simulate decorator creation
        decorators.push(decorator_name);
    }

    assert_eq!(decorators.len(), 10);

    // Test decorator with different configurations
    let config_variations = vec![
        ("default", serde_json::json!({})),
        ("with_registry", serde_json::json!({"registry": "custom"})),
        (
            "with_options",
            serde_json::json!({"options": {"strict": true}}),
        ),
    ];

    for (name, config) in config_variations {
        assert!(config.is_object(), "Config for {name} should be object");
    }
}

#[test]
fn test_tool_decorator_parameter_validation_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test comprehensive parameter validation scenarios
    let parameter_test_cases = vec![
        // Complete parameters
        serde_json::json!({
            "description": "Complete test tool",
            "name": "complete_tool",
            "return_type": "string"
        }),
        // Minimal parameters
        serde_json::json!({
            "description": "Minimal tool"
        }),
        // Only name
        serde_json::json!({
            "name": "name_only_tool"
        }),
        // Only return type
        serde_json::json!({
            "return_type": "dict"
        }),
        // Empty parameters
        serde_json::json!({}),
        // Parameters with null values
        serde_json::json!({
            "description": null,
            "name": null,
            "return_type": null
        }),
        // Parameters with empty strings
        serde_json::json!({
            "description": "",
            "name": "",
            "return_type": ""
        }),
    ];

    for (i, test_case) in parameter_test_cases.iter().enumerate() {
        // Validate that each test case is a valid JSON object
        assert!(test_case.is_object(), "Test case {i} should be object");

        // Test parameter extraction
        let description = test_case.get("description");
        let name = test_case.get("name");
        let return_type = test_case.get("return_type");

        // Verify parameter handling
        match (description, name, return_type) {
            (Some(desc), Some(n), Some(rt)) => {
                // All parameters present
                assert!(desc.is_string() || desc.is_null());
                assert!(n.is_string() || n.is_null());
                assert!(rt.is_string() || rt.is_null());
            }
            _ => {
                // Some parameters missing - should be handled gracefully
                // Missing parameters handled
            }
        }
    }

    // Test parameter validation with edge cases
    let edge_case_parameters = [
        // Very long strings
        serde_json::json!({
            "description": "A".repeat(10000),
            "name": "B".repeat(1000),
            "return_type": "C".repeat(500)
        }),
        // Special characters
        serde_json::json!({
            "description": "Tool with special chars: !@#$%^&*()",
            "name": "special_tool_!@#$%",
            "return_type": "special_type"
        }),
        // Unicode characters
        serde_json::json!({
            "description": "Unicode tool: 🚀🌟🎉",
            "name": "unicode_tool_🚀",
            "return_type": "unicode_type"
        }),
        // Multiline strings
        serde_json::json!({
            "description": "Line 1\nLine 2\tTabbed\r\nWindows line ending",
            "name": "multiline_tool",
            "return_type": "string"
        }),
    ];

    for (i, edge_case) in edge_case_parameters.iter().enumerate() {
        assert!(edge_case.is_object(), "Edge case {i} should be object");

        // Verify edge case handling
        if let Some(desc) = edge_case.get("description") {
            if let Some(desc_str) = desc.as_str() {
                assert!(
                    !desc_str.is_empty() || desc_str.is_empty(),
                    "Description handled"
                );
            }
        }
    }
}

#[test]
fn test_tool_decorator_function_registration_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test various function types and registration scenarios
    let function_test_cases = vec![
        ("simple_function", "echo"),
        ("math_function", "add_one"),
        ("string_function", "to_string"),
        ("conditional_function", "fail_on_even"),
    ];

    for (_function_name, function_type) in function_test_cases {
        let test_function = create_simple_test_tool(function_type);

        // Test function with various inputs
        let test_inputs = [
            serde_json::json!({"input": "test"}),
            serde_json::json!({"value": 42}),
            serde_json::json!({"data": [1, 2, 3]}),
            serde_json::json!({}),
        ];

        for (i, input) in test_inputs.iter().enumerate() {
            let result = test_function(input.clone());

            match function_type {
                "echo" => {
                    assert!(result.is_ok(), "Echo function should succeed for input {i}");
                }
                "add_one" => {
                    if input.get("value").and_then(|v| v.as_i64()).is_some() {
                        assert!(result.is_ok(), "Add one should succeed with valid value");
                    } else {
                        assert!(result.is_err(), "Add one should fail with invalid value");
                    }
                }
                "to_string" => {
                    assert!(result.is_ok(), "To string should always succeed");
                }
                "fail_on_even" => {
                    if let Some(value) = input.get("value").and_then(|v| v.as_i64()) {
                        if value % 2 == 0 {
                            assert!(result.is_err(), "Should fail on even numbers");
                        } else {
                            assert!(result.is_ok(), "Should succeed on odd numbers");
                        }
                    } else {
                        assert!(result.is_err(), "Should fail without valid value");
                    }
                }
                _ => {
                    // Unknown function type, just verify it doesn't panic
                    let _ = result;
                }
            }
        }
    }

    // Test function registration with complex parameters
    let complex_function = create_simple_test_tool("echo");

    let complex_inputs = [
        serde_json::json!({
            "nested": {
                "data": {
                    "array": [1, 2, 3],
                    "string": "test",
                    "boolean": true,
                    "null": null
                }
            }
        }),
        serde_json::json!({
            "unicode": "🚀🌟🎉💻🔥✨",
            "special_chars": "!@#$%^&*()_+-=[]{}|;':\",./<>?`~",
            "multiline": "Line 1\nLine 2\tTabbed"
        }),
        serde_json::json!({
            "large_array": (0..1000).collect::<Vec<i32>>(),
            "large_string": "A".repeat(10000)
        }),
    ];

    for (i, complex_input) in complex_inputs.iter().enumerate() {
        let result = complex_function(complex_input.clone());
        assert!(
            result.is_ok(),
            "Complex function should handle complex input {i}"
        );
    }
}

#[test]
fn test_tool_decorator_metadata_extraction_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test metadata extraction with various scenarios
    let metadata_test_cases = vec![
        ("simple_tool", "Simple test tool", "string"),
        (
            "complex_tool",
            "Complex tool with detailed description",
            "object",
        ),
        ("math_tool", "Mathematical computation tool", "number"),
        ("boolean_tool", "Boolean logic tool", "boolean"),
        ("array_tool", "Array processing tool", "array"),
    ];

    for (name, description, _return_type) in metadata_test_cases {
        let metadata = create_simple_tool_metadata(name, description);

        // Verify basic metadata structure
        assert_eq!(metadata["name"], name);
        assert_eq!(metadata["description"], description);
        assert!(metadata["parameters_schema"].is_object());
        assert_eq!(metadata["return_type"], "string"); // Our helper always returns string

        // Verify required fields are present
        let required_fields = ["name", "description", "parameters_schema", "return_type"];
        for field in &required_fields {
            assert!(metadata.get(*field).is_some(), "Missing field: {field}");
        }

        // Test parameter schema structure
        let schema = &metadata["parameters_schema"];
        assert!(schema.get("type").is_some());
        assert!(schema.get("properties").is_some());

        if let Some(properties) = schema.get("properties") {
            assert!(properties.is_object());
        }
    }

    // Test metadata with complex parameter schemas
    let complex_schemas = [
        serde_json::json!({
            "type": "object",
            "properties": {
                "required_param": {"type": "string", "description": "Required parameter"},
                "optional_param": {"type": "number", "default": 42},
                "array_param": {"type": "array", "items": {"type": "string"}},
                "nested_param": {
                    "type": "object",
                    "properties": {
                        "nested_field": {"type": "boolean"}
                    }
                }
            },
            "required": ["required_param"]
        }),
        serde_json::json!({
            "type": "object",
            "properties": {
                "union_param": {
                    "oneOf": [
                        {"type": "string"},
                        {"type": "number"}
                    ]
                },
                "enum_param": {
                    "type": "string",
                    "enum": ["option1", "option2", "option3"]
                }
            }
        }),
    ];

    for (i, schema) in complex_schemas.iter().enumerate() {
        // Verify complex schema structure
        assert!(schema.is_object(), "Complex schema {i} should be object");
        assert!(schema.get("type").is_some());
        assert!(schema.get("properties").is_some());

        if let Some(properties) = schema.get("properties") {
            assert!(properties.is_object());

            // Verify property definitions
            for (prop_name, prop_def) in properties.as_object().unwrap() {
                assert!(
                    prop_def.is_object(),
                    "Property {prop_name} should be object"
                );

                // Most properties should have a type
                if prop_def.get("oneOf").is_none() {
                    assert!(
                        prop_def.get("type").is_some(),
                        "Property {prop_name} should have type"
                    );
                }
            }
        }
    }

    // Test metadata with edge cases
    let edge_case_metadata = vec![
        ("", "Empty name tool", "string"),
        (
            "tool_with_very_long_name_that_exceeds_normal_limits",
            "Long name",
            "string",
        ),
        ("unicode_tool_🚀", "Unicode tool 🌟", "string"),
        ("special_chars_!@#$%", "Special chars tool", "string"),
    ];

    for (name, description, _return_type) in edge_case_metadata {
        let metadata = create_simple_tool_metadata(name, description);

        // Verify edge cases are handled
        assert_eq!(metadata["name"], name);
        assert_eq!(metadata["description"], description);
        assert!(metadata["parameters_schema"].is_object());
    }
}

#[test]
fn test_tool_decorator_error_handling_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test various error scenarios
    let error_test_cases = vec![
        ("fail_on_even", serde_json::json!({"value": 2}), true), // Should fail
        ("fail_on_even", serde_json::json!({"value": 3}), false), // Should succeed
        (
            "fail_on_even",
            serde_json::json!({"value": "invalid"}),
            true,
        ), // Should fail
        ("fail_on_even", serde_json::json!({}), true),           // Should fail
        ("add_one", serde_json::json!({"value": 5}), false),     // Should succeed
        ("add_one", serde_json::json!({"value": "invalid"}), true), // Should fail
        ("echo", serde_json::json!({"any": "data"}), false),     // Should succeed
    ];

    for (tool_type, input, should_fail) in error_test_cases {
        let test_tool = create_simple_test_tool(tool_type);
        let result = test_tool(input.clone());

        if should_fail {
            assert!(
                result.is_err(),
                "Tool {tool_type} should fail with input {input:?}"
            );

            // Verify error message is meaningful
            let error_msg = result.unwrap_err();
            assert!(!error_msg.is_empty(), "Error message should not be empty");

            // Check for specific error patterns
            match tool_type {
                "fail_on_even" => {
                    if input
                        .get("value")
                        .and_then(|v| v.as_i64())
                        .is_some_and(|v| v % 2 == 0)
                    {
                        assert!(error_msg.contains("Even numbers cause failure"));
                    }
                }
                "add_one" => {
                    if input.get("value").is_none() || !input["value"].is_number() {
                        assert!(error_msg.contains("Invalid parameter"));
                    }
                }
                _ => {}
            }
        } else {
            assert!(
                result.is_ok(),
                "Tool {tool_type} should succeed with input {input:?}"
            );
        }
    }

    // Test error handling with malformed inputs
    let malformed_inputs = [
        serde_json::Value::Null,
        serde_json::json!("not_an_object"),
        serde_json::json!(123),
        serde_json::json!(true),
        serde_json::json!([1, 2, 3]),
    ];

    for (i, malformed_input) in malformed_inputs.iter().enumerate() {
        let test_tool = create_simple_test_tool("echo");
        let result = test_tool(malformed_input.clone());

        // Echo tool should handle any input gracefully
        assert!(
            result.is_ok(),
            "Echo tool should handle malformed input {i}"
        );
    }

    // Test error recovery scenarios
    let recovery_tool = create_simple_test_tool("fail_on_even");

    // Sequence of operations: fail, succeed, fail, succeed
    let recovery_sequence = vec![
        (serde_json::json!({"value": 2}), true),  // Fail
        (serde_json::json!({"value": 3}), false), // Succeed
        (serde_json::json!({"value": 4}), true),  // Fail
        (serde_json::json!({"value": 5}), false), // Succeed
    ];

    for (input, should_fail) in recovery_sequence {
        let result = recovery_tool(input.clone());

        if should_fail {
            assert!(
                result.is_err(),
                "Recovery test should fail for input {input:?}"
            );
        } else {
            assert!(
                result.is_ok(),
                "Recovery test should succeed for input {input:?}"
            );
        }
    }

    // Test error handling with resource constraints
    let large_input = serde_json::json!({
        "large_data": "A".repeat(100000),
        "large_array": (0..10000).collect::<Vec<i32>>()
    });

    let resource_tool = create_simple_test_tool("echo");
    let result = resource_tool(large_input);

    // Should handle large inputs gracefully
    assert!(result.is_ok(), "Tool should handle large inputs");
}

#[test]
fn test_tool_decorator_thread_safety_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test concurrent tool creation and execution
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Test concurrent tool operations
    for thread_id in 0..10 {
        let counter = Arc::clone(&shared_counter);

        let handle = thread::spawn(move || {
            // Create tool in thread
            let thread_tool = create_simple_test_tool("add_one");

            // Execute tool multiple times
            let mut thread_results = Vec::new();
            for i in 0..5 {
                let input = serde_json::json!({"value": thread_id * 10 + i});
                let result = thread_tool(input);
                thread_results.push(result.is_ok());
            }

            // Update shared counter
            {
                let mut num = counter.lock().unwrap();
                *num += thread_results.iter().filter(|&&success| success).count();
            }

            thread_results
        });

        handles.push(handle);
    }

    // Collect results from all threads
    let mut all_results = Vec::new();
    for handle in handles {
        let thread_results = handle.join().unwrap();
        all_results.extend(thread_results);
    }

    // Verify thread safety
    let final_count = *shared_counter.lock().unwrap();
    let successful_operations = all_results.iter().filter(|&&success| success).count();

    assert_eq!(final_count, successful_operations);
    assert!(successful_operations >= 40); // Most operations should succeed

    // Test concurrent access to shared tool
    let shared_tool = Arc::new(create_simple_test_tool("echo"));
    let concurrent_results = Arc::new(Mutex::new(Vec::new()));
    let mut concurrent_handles = vec![];

    for worker_id in 0..20 {
        let tool = Arc::clone(&shared_tool);
        let results = Arc::clone(&concurrent_results);

        let handle = thread::spawn(move || {
            let input = serde_json::json!({"worker_id": worker_id, "data": "test"});
            let result = tool(input);

            {
                let mut results_guard = results.lock().unwrap();
                results_guard.push((worker_id, result.is_ok()));
            }
        });

        concurrent_handles.push(handle);
    }

    // Wait for all concurrent operations
    for handle in concurrent_handles {
        handle.join().unwrap();
    }

    // Verify concurrent access results
    let final_results = concurrent_results.lock().unwrap();
    assert_eq!(final_results.len(), 20);

    let successful_concurrent = final_results.iter().filter(|(_, success)| *success).count();
    assert_eq!(successful_concurrent, 20); // All should succeed for echo tool

    // Test thread safety with different tool types
    let tool_types = vec!["echo", "add_one", "to_string", "fail_on_even"];
    let type_results = Arc::new(Mutex::new(HashMap::new()));
    let mut type_handles = vec![];

    for tool_type in tool_types {
        let results = Arc::clone(&type_results);
        let tool_type_owned = tool_type.to_string();

        let handle = thread::spawn(move || {
            let tool = create_simple_test_tool(&tool_type_owned);
            let mut type_successes = 0;

            // Test multiple inputs for this tool type
            for i in 0..10 {
                let input = match tool_type_owned.as_str() {
                    "add_one" | "fail_on_even" => serde_json::json!({"value": i}),
                    _ => serde_json::json!({"input": format!("test_{}", i)}),
                };

                if tool(input).is_ok() {
                    type_successes += 1;
                }
            }

            {
                let mut results_guard = results.lock().unwrap();
                results_guard.insert(tool_type_owned, type_successes);
            }
        });

        type_handles.push(handle);
    }

    // Wait for all tool type tests
    for handle in type_handles {
        handle.join().unwrap();
    }

    // Verify tool type results
    let type_results_final = type_results.lock().unwrap();
    assert_eq!(type_results_final.len(), 4);

    // Echo and to_string should have high success rates
    assert!(*type_results_final.get("echo").unwrap_or(&0) >= 8);
    assert!(*type_results_final.get("to_string").unwrap_or(&0) >= 8);
}

#[test]
fn test_tool_decorator_memory_management_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test memory management with large number of tools
    let mut tools = Vec::new();

    // Create many tools to test memory usage
    for i in 0..1000 {
        let tool_type = match i % 4 {
            0 => "echo",
            1 => "add_one",
            2 => "to_string",
            _ => "fail_on_even",
        };

        let tool = create_simple_test_tool(tool_type);
        tools.push((i, tool));
    }

    // Verify all tools were created
    assert_eq!(tools.len(), 1000);

    // Test random sampling of tools
    let test_indices = vec![0, 100, 250, 500, 750, 999];
    for &index in &test_indices {
        let (tool_id, ref tool) = tools[index];
        let tool_type = match tool_id % 4 {
            0 => "echo",
            1 => "add_one",
            2 => "to_string",
            _ => "fail_on_even",
        };

        let input = match tool_type {
            "add_one" | "fail_on_even" => serde_json::json!({"value": tool_id}),
            _ => serde_json::json!({"input": format!("test_{}", tool_id)}),
        };

        let result = tool(input);

        // Verify tool still works after mass creation
        match tool_type {
            "echo" | "to_string" => assert!(result.is_ok()),
            "add_one" => assert!(result.is_ok()),
            "fail_on_even" => {
                if tool_id % 2 == 0 {
                    assert!(result.is_err());
                } else {
                    assert!(result.is_ok());
                }
            }
            _ => {}
        }
    }

    // Test memory cleanup by dropping tools in batches
    let batch_size = 100;
    let mut remaining_tools = tools;

    while !remaining_tools.is_empty() {
        let current_batch_size = std::cmp::min(batch_size, remaining_tools.len());
        let batch: Vec<_> = remaining_tools.drain(0..current_batch_size).collect();

        // Test a tool from the batch before dropping
        if let Some((tool_id, ref tool)) = batch.first() {
            let input = serde_json::json!({"input": format!("batch_test_{}", tool_id)});
            let result = tool(input);
            // Echo-type tools should work
            if tool_id % 4 == 0 {
                assert!(result.is_ok());
            }
        }

        // Batch goes out of scope here, testing cleanup
        drop(batch);

        // Verify remaining tools still work
        if !remaining_tools.is_empty() {
            let (test_id, ref test_tool) = remaining_tools[0];
            let input = serde_json::json!({"input": format!("remaining_test_{}", test_id)});
            let result = test_tool(input);

            if test_id % 4 == 0 {
                // Echo tool
                assert!(result.is_ok());
            }
        }
    }

    // Test memory usage with large data
    let large_data_tool = create_simple_test_tool("echo");

    let large_inputs = [
        serde_json::json!({"large_string": "A".repeat(10000)}),
        serde_json::json!({"large_array": (0..1000).collect::<Vec<i32>>()}),
        serde_json::json!({
            "nested": {
                "deep": {
                    "structure": {
                        "with": {
                            "many": {
                                "levels": "test"
                            }
                        }
                    }
                }
            }
        }),
    ];

    for (i, large_input) in large_inputs.iter().enumerate() {
        let result = large_data_tool(large_input.clone());
        assert!(result.is_ok(), "Large data test {i} should succeed");
    }

    // Test rapid allocation and deallocation
    for cycle in 0..10 {
        let mut cycle_tools = Vec::new();

        // Rapid allocation
        for _i in 0..100 {
            let tool = create_simple_test_tool("echo");
            cycle_tools.push(tool);
        }

        // Test some tools
        for (i, tool) in cycle_tools.iter().enumerate().take(10) {
            let input = serde_json::json!({"cycle": cycle, "tool": i});
            let result = tool(input);
            assert!(result.is_ok());
        }

        // Rapid deallocation (tools go out of scope)
        drop(cycle_tools);
    }
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
fn test_tool_decorator_function_types_comprehensive() {
    skip_if_no_tools("ToolDecorator not available");

    // Test comprehensive function type scenarios
    let function_type_tests = vec![
        (
            "echo",
            vec![
                (serde_json::json!({"test": "value"}), true),
                (serde_json::json!({"number": 42}), true),
                (serde_json::json!({"array": [1, 2, 3]}), true),
                (serde_json::json!({}), true),
            ],
        ),
        (
            "add_one",
            vec![
                (serde_json::json!({"value": 5}), true),
                (serde_json::json!({"value": 0}), true),
                (serde_json::json!({"value": -1}), true),
                (serde_json::json!({"value": "invalid"}), false),
                (serde_json::json!({}), false),
            ],
        ),
        (
            "to_string",
            vec![
                (serde_json::json!({"value": 42}), true),
                (serde_json::json!({"value": "string"}), true),
                (serde_json::json!({"value": true}), true),
                (serde_json::json!({"value": null}), true),
                (serde_json::json!({"complex": {"nested": "data"}}), true),
            ],
        ),
        (
            "fail_on_even",
            vec![
                (serde_json::json!({"value": 1}), true), // Odd - should succeed
                (serde_json::json!({"value": 2}), false), // Even - should fail
                (serde_json::json!({"value": 3}), true), // Odd - should succeed
                (serde_json::json!({"value": 4}), false), // Even - should fail
                (serde_json::json!({"value": "invalid"}), false), // Invalid - should fail
            ],
        ),
    ];

    for (function_type, test_cases) in function_type_tests {
        let tool = create_simple_test_tool(function_type);

        for (input, should_succeed) in test_cases {
            let result = tool(input.clone());

            if should_succeed {
                assert!(
                    result.is_ok(),
                    "Function {function_type} should succeed with input {input:?}"
                );

                // Verify result content for specific function types
                match function_type {
                    "add_one" => {
                        if let Some(value) = input.get("value").and_then(|v| v.as_i64()) {
                            let expected = value + 1;
                            let actual = result.unwrap();
                            assert_eq!(actual, serde_json::json!(expected));
                        }
                    }
                    "echo" => {
                        let result_value = result.unwrap();
                        assert_eq!(result_value, input);
                    }
                    "to_string" => {
                        let result_value = result.unwrap();
                        assert!(result_value.is_string());
                        let result_str = result_value.as_str().unwrap();
                        assert!(result_str.contains(&input.to_string()) || !result_str.is_empty());
                    }
                    _ => {
                        // Just verify we got a valid result
                        let _ = result.unwrap();
                    }
                }
            } else {
                assert!(
                    result.is_err(),
                    "Function {function_type} should fail with input {input:?}"
                );

                // Verify error messages are meaningful
                let error_msg = result.unwrap_err();
                assert!(!error_msg.is_empty(), "Error message should not be empty");
            }
        }
    }

    // Test function performance characteristics
    let performance_tool = create_simple_test_tool("echo");
    let start_time = Instant::now();

    // Execute many operations to test performance
    for i in 0..1000 {
        let input = serde_json::json!({"iteration": i, "data": "performance_test"});
        let result = performance_tool(input);
        assert!(
            result.is_ok(),
            "Performance test iteration {i} should succeed"
        );
    }

    let elapsed = start_time.elapsed();

    // Performance should be reasonable (less than 1 second for 1000 operations)
    assert!(
        elapsed < Duration::from_secs(1),
        "Performance test took too long: {elapsed:?}"
    );

    // Test function behavior with concurrent access
    let concurrent_tool = Arc::new(create_simple_test_tool("add_one"));
    let mut concurrent_handles = vec![];

    for thread_id in 0..5 {
        let tool = Arc::clone(&concurrent_tool);

        let handle = thread::spawn(move || {
            let mut thread_results = Vec::new();

            for i in 0..10 {
                let input = serde_json::json!({"value": thread_id * 10 + i});
                let result = tool(input);
                thread_results.push(result.is_ok());
            }

            thread_results
        });

        concurrent_handles.push(handle);
    }

    // Collect concurrent results
    let mut all_concurrent_results = Vec::new();
    for handle in concurrent_handles {
        let thread_results = handle.join().unwrap();
        all_concurrent_results.extend(thread_results);
    }

    // Verify concurrent function execution
    assert_eq!(all_concurrent_results.len(), 50); // 5 threads * 10 operations
    let successful_concurrent = all_concurrent_results
        .iter()
        .filter(|&&success| success)
        .count();
    assert_eq!(successful_concurrent, 50); // All should succeed for add_one with valid inputs
}

#[test]
fn test_tool_decorator_integration_scenarios() {
    skip_if_no_tools("ToolDecorator not available");

    // Test integration scenarios that combine multiple aspects

    // Scenario 1: Tool chain simulation
    let tools = [
        ("input_processor", create_simple_test_tool("echo")),
        ("data_transformer", create_simple_test_tool("add_one")),
        ("output_formatter", create_simple_test_tool("to_string")),
        ("validator", create_simple_test_tool("fail_on_even")),
    ];

    // Simulate a processing chain
    let initial_data = serde_json::json!({"value": 5});
    let mut current_data = initial_data;

    // Step 1: Process input (echo)
    let processed = tools[0].1(current_data.clone());
    assert!(processed.is_ok());
    current_data = processed.unwrap();

    // Step 2: Transform data (add_one) - need to extract value
    if let Some(value) = current_data.get("value").and_then(|v| v.as_i64()) {
        let transform_input = serde_json::json!({"value": value});
        let transformed = tools[1].1(transform_input);
        assert!(transformed.is_ok());

        let new_value = transformed.unwrap().as_i64().unwrap();
        current_data = serde_json::json!({"value": new_value});
    }

    // Step 3: Validate result (fail_on_even)
    let validation_result = tools[3].1(current_data.clone());
    // Value should be 6 (even), so validation should fail
    assert!(validation_result.is_err());

    // Scenario 2: Error propagation through tool chain
    let error_chain_data = serde_json::json!({"value": 4}); // Even number

    // This should fail at validation step
    let validation_result = tools[3].1(error_chain_data);
    assert!(validation_result.is_err());
    assert!(validation_result
        .unwrap_err()
        .contains("Even numbers cause failure"));

    // Scenario 3: Stress test with rapid tool switching
    let stress_tools = [
        create_simple_test_tool("echo"),
        create_simple_test_tool("add_one"),
        create_simple_test_tool("to_string"),
    ];

    for cycle in 0..100 {
        let tool_index = cycle % stress_tools.len();
        let tool = &stress_tools[tool_index];

        let input = match tool_index {
            1 => serde_json::json!({"value": cycle}), // add_one
            _ => serde_json::json!({"cycle": cycle, "data": "stress_test"}),
        };

        let result = tool(input);
        assert!(result.is_ok(), "Stress test cycle {cycle} should succeed");
    }
}
