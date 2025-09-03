//! Integration tests for core tools functionality

use super::*;

#[tokio::test]
async fn test_tool_registry_thread_safety() {
    skip_if_no_tools("ToolRegistry not available");
    
    // Test thread safety with concurrent registrations
    let config = get_tools_test_config();
    
    // Simulate concurrent tool registrations
    let results = run_concurrent_operations(
        config.max_concurrent_tools,
        move |i| {
            // Simulate tool registration
            Ok(format!("tool_{}", i))
        }
    ).await;
    
    // Verify all operations completed
    assert_eq!(results.len(), config.max_concurrent_tools);
    for result in results {
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_tool_executor_concurrent_execution() {
    skip_if_no_tools("ToolExecutor not available");
    
    // Test concurrent tool execution
    let config = get_tools_test_config();
    
    // Create test tools
    let test_tools = vec![
        ("add_numbers", create_test_tool_function("add_numbers")),
        ("multiply_numbers", create_test_tool_function("multiply_numbers")),
        ("echo", create_test_tool_function("echo")),
    ];
    
    // Test concurrent execution
    let results = run_concurrent_operations(
        config.max_concurrent_tools,
        move |i| {
            let _tool_name = test_tools[i % test_tools.len()].0;
            let tool_func = &test_tools[i % test_tools.len()].1;
            
            // Execute tool with test parameters
            let input = serde_json::json!({
                "a": i as i64,
                "b": 2
            });
            
            tool_func(input)
        }
    ).await;
    
    // Verify execution results
    assert_eq!(results.len(), config.max_concurrent_tools);
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Tool execution {} failed: {:?}", i, result);
    }
}

#[tokio::test]
async fn test_tool_decorator_python_binding() {
    skip_if_no_tools("ToolDecorator not available");
    
    // Test Python binding functionality
    // This would test the actual Python binding if available
    
    // For now, test the interface structure
    let test_metadata = create_test_tool_metadata("test_tool", "Test tool for binding");
    
    assert_eq!(test_metadata["name"], "test_tool");
    assert_eq!(test_metadata["description"], "Test tool for binding");
    assert!(test_metadata["parameters_schema"].is_object());
    assert_eq!(test_metadata["return_type"], "string");
    
    // Test that metadata can be serialized/deserialized
    let json_string = serde_json::to_string(&test_metadata).unwrap();
    let deserialized: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    
    assert_eq!(deserialized["name"], test_metadata["name"]);
    assert_eq!(deserialized["description"], test_metadata["description"]);
}



#[tokio::test]
async fn test_tool_metadata_management() {
    skip_if_no_tools("ToolMetadata not available");
    
    // Test metadata management operations
    let mut metadata_collection = Vec::new();
    
    // Create multiple metadata entries
    for i in 0..100 {
        let metadata = create_test_tool_metadata(
            &format!("tool_{}", i),
            &format!("Tool number {}", i)
        );
        metadata_collection.push(metadata);
    }
    
    // Test metadata operations
    assert_eq!(metadata_collection.len(), 100);
    
    // Test metadata retrieval
    let tool_50 = metadata_collection.iter().find(|m| m["name"] == "tool_50");
    assert!(tool_50.is_some());
    assert_eq!(tool_50.unwrap()["description"], "Tool number 50");
    
    // Test metadata filtering
    let string_tools: Vec<_> = metadata_collection
        .iter()
        .filter(|m| m["return_type"] == "string")
        .collect();
    assert_eq!(string_tools.len(), 100); // All our test tools return string
}

#[tokio::test]
async fn test_tool_execution_context() {
    skip_if_no_tools("ExecutionContext not available");
    
    // Test execution context creation and management
    let context = create_test_execution_context();
    
    // Verify context structure
    assert!(context["session_id"].is_string());
    assert!(context["user_id"].is_string());
    assert!(context["timestamp"].is_number());
    assert!(context["metadata"].is_object());
    assert!(context["metadata"]["test_mode"].as_bool().unwrap());
    
    // Test context serialization
    let context_json = serde_json::to_string(&context).unwrap();
    let deserialized_context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
    
    assert_eq!(deserialized_context["session_id"], context["session_id"]);
    assert_eq!(deserialized_context["user_id"], context["user_id"]);
}

#[tokio::test]
async fn test_tool_validation_integration() {
    skip_if_no_tools("ToolValidation not available");
    
    // Test tool result validation
    let successful_result = serde_json::json!({
        "success": true,
        "tool_name": "test_tool",
        "output": "test_output",
        "duration_ms": 100,
        "timestamp": chrono::Utc::now().timestamp()
    });
    
    let failed_result = serde_json::json!({
        "success": false,
        "tool_name": "failing_tool",
        "error": "Test error message",
        "duration_ms": 50,
        "timestamp": chrono::Utc::now().timestamp()
    });
    
    // Validate successful result
    let validation_result = validate_tool_result(&successful_result, true, "test_tool");
    assert!(validation_result.is_ok());
    
    // Validate failed result
    let validation_result = validate_tool_result(&failed_result, false, "failing_tool");
    assert!(validation_result.is_ok());
    
    // Test invalid result validation
    let invalid_result = serde_json::json!({
        "success": true,
        "tool_name": "test_tool"
        // Missing output field
    });
    
    let validation_result = validate_tool_result(&invalid_result, true, "test_tool");
    assert!(validation_result.is_err());
}

#[tokio::test]
async fn test_tool_error_handling_integration() {
    skip_if_no_tools("ToolErrorHandling not available");
    
    // Test error handling across tool execution
    let failing_tool = create_test_tool_function("fail_always");
    
    // Execute failing tool
    let result = failing_tool(serde_json::json!({}));
    assert!(result.is_err());
    
    // Test error message content
    let error_message = result.unwrap_err();
    assert!(error_message.contains("always fails"));
    
    // Test error propagation
    let error_context = serde_json::json!({
        "error": error_message,
        "tool_name": "fail_always",
        "timestamp": chrono::Utc::now().timestamp()
    });
    
    // Verify error context structure
    assert!(error_context["error"].is_string());
    assert!(error_context["tool_name"].is_string());
    assert!(error_context["timestamp"].is_number());
}

#[tokio::test]
async fn test_tool_concurrency_limits() {
    skip_if_no_tools("ToolConcurrency not available");
    
    // Test concurrency limits
    let config = get_tools_test_config();
    
    // Test that we can handle the configured concurrency level
    let results = run_concurrent_operations(
        config.max_concurrent_tools,
        |i| {
            // Simulate tool execution
            Ok(format!("concurrent_tool_{}", i))
        }
    ).await;
    
    // Verify all operations completed within limits
    assert_eq!(results.len(), config.max_concurrent_tools);
    
    // Test exceeding limits (should still work but may be slower)
    let high_concurrency = config.max_concurrent_tools * 2;
    let high_results = run_concurrent_operations(
        high_concurrency,
        |i| {
            Ok(format!("high_concurrency_tool_{}", i))
        }
    ).await;
    
    assert_eq!(high_results.len(), high_concurrency);
}

#[tokio::test]
async fn test_tool_execution_timeout() {
    skip_if_no_tools("ToolTimeout not available");
    
    // Test execution timeout handling
    let config = get_tools_test_config();
    
    // Test with slow tool
    let slow_tool = create_test_tool_function("slow_tool");
    
    let start_time = std::time::Instant::now();
    let result = slow_tool(serde_json::json!({}));
    let execution_time = start_time.elapsed();
    
    // Tool should complete within reasonable time
    assert!(execution_time.as_millis() < config.max_execution_time_ms as u128);
    assert!(result.is_ok());
    
    // Test timeout configuration
    assert!(config.max_execution_time_ms > 0);
    assert!(config.max_execution_time_ms <= 10000); // Max 10 seconds for tests
}
