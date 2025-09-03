//! Rust integration tests for GraphBit tools module
//!
//! This module contains integration tests that test the complete functionality
//! of the GraphBit tools system, including:
//! - Tool decorator functionality
//! - Tool registry management
//! - Tool execution engine
//! - Tool result handling
//! - Cross-language integration tests
//! - Core functionality testing

pub mod tools_core_tests;

/// Check if tools module is available for testing
pub fn has_tools_module() -> bool {
    // This would check if the tools module is compiled and available
    // For now, we'll assume it's available
    true
}

/// Skip test if tools module is not available
pub fn skip_if_no_tools(reason: &str) {
    if !has_tools_module() {
        println!("Skipping test - tools module not available: {}", reason);
        panic!("TEST_SKIP");
    }
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

impl Default for ToolsTestConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tools: 10,
            max_execution_time_ms: 5000,
            max_tool_calls: 100,
                }
    }
}

/// Helper function to create test tool functions
pub fn create_test_tool_function(name: &str) -> Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, String> + Send + Sync> {
    let name = name.to_string();
    Box::new(move |input: serde_json::Value| {
        match name.as_str() {
            "add_numbers" => {
                if let (Some(a), Some(b)) = (input.get("a").and_then(|v| v.as_i64()), 
                                             input.get("b").and_then(|v| v.as_i64())) {
                    Ok(serde_json::json!(a + b))
                } else {
                    Err("Invalid parameters for add_numbers".to_string())
                }
            }
            "multiply_numbers" => {
                if let (Some(a), Some(b)) = (input.get("a").and_then(|v| v.as_i64()), 
                                             input.get("b").and_then(|v| v.as_i64())) {
                    Ok(serde_json::json!(a * b))
                } else {
                    Err("Invalid parameters for multiply_numbers".to_string())
                }
            }
            "echo" => {
                Ok(input)
            }
            "fail_always" => {
                Err("This tool always fails".to_string())
            }
            "slow_tool" => {
                // Simulate a slow tool
                std::thread::sleep(std::time::Duration::from_millis(100));
                Ok(serde_json::json!("slow_result"))
            }
            _ => {
                Err(format!("Unknown test tool: {}", name))
            }
        }
    })
}



/// Helper function to create test tool metadata
pub fn create_test_tool_metadata(name: &str, description: &str) -> serde_json::Value {
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

/// Helper function to measure execution time
pub fn measure_execution_time<F, T>(f: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Helper function to run concurrent operations for testing
pub async fn run_concurrent_operations<F, T>(
    count: usize,
    operation: F,
) -> Vec<Result<T, String>>
where
    F: Fn(usize) -> Result<T, String> + Send + Sync + 'static,
    T: Send + Sync + Clone + 'static,
{
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    // Create a single Arc to the operation function
    let operation = Arc::new(operation);
    
    for i in 0..count {
        let results = Arc::clone(&results);
        let operation = Arc::clone(&operation);
        
        let handle = tokio::spawn(async move {
            let result = operation(i);
            let mut results = results.lock().await;
            results.push(result);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let results_guard = results.lock().await;
    results_guard.clone()
}

/// Helper function to create test execution context
pub fn create_test_execution_context() -> serde_json::Value {
    serde_json::json!({
        "session_id": "test_session_123",
        "user_id": "test_user",
        "timestamp": chrono::Utc::now().timestamp(),
        "metadata": {
            "test_mode": true,
            "version": "1.0.0"
        }
    })
}

/// Helper function to validate tool result
pub fn validate_tool_result(
    result: &serde_json::Value,
    expected_success: bool,
    expected_tool_name: &str,
) -> Result<(), String> {
    if let Some(success) = result.get("success").and_then(|v| v.as_bool()) {
        if success != expected_success {
            return Err(format!("Expected success={}, got {}", expected_success, success));
        }
    } else {
        return Err("Missing 'success' field in result".to_string());
    }
    
    if let Some(tool_name) = result.get("tool_name").and_then(|v| v.as_str()) {
        if tool_name != expected_tool_name {
            return Err(format!("Expected tool_name={}, got {}", expected_tool_name, tool_name));
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






