use graphbit_core::errors::GraphBitError;
use graphbit_core::types::*;
use graphbit_core::{AgentId, MessageContent, NodeId, WorkflowId};
use std::collections::HashMap;

// ID Types Tests
#[test]
fn test_agent_id() {
    let id = AgentId::new();
    assert!(!id.as_uuid().is_nil());

    let id_from_string = AgentId::from_string("test-agent").unwrap();
    assert!(!id_from_string.as_uuid().is_nil());

    // Test deterministic ID generation
    let id1 = AgentId::from_string("test-agent").unwrap();
    let id2 = AgentId::from_string("test-agent").unwrap();
    assert_eq!(id1, id2);
}

#[test]
fn test_workflow_id() {
    let id = WorkflowId::new();
    assert!(!id.as_uuid().is_nil());

    let id_str = id.to_string();
    assert!(!id_str.is_empty());
}

#[test]
fn test_node_id() {
    let id = NodeId::from_string("test-node").unwrap();
    assert!(!id.as_uuid().is_nil());

    let id_display = format!("{id}");
    assert!(!id_display.is_empty());
}

#[test]
fn test_message_content() {
    let text = "test message".to_string();
    let content = MessageContent::Text(text.clone());

    match content {
        MessageContent::Text(msg) => assert_eq!(msg, text),
        _ => panic!("Unexpected message content type"),
    }
}

// Retry Configuration Tests
#[test]
fn test_retry_config_calculate_delay_no_jitter() {
    let cfg = RetryConfig::default()
        .with_exponential_backoff(100, 2.0, 1000)
        .with_jitter(0.0);
    assert_eq!(cfg.calculate_delay(0), 0);
    assert_eq!(cfg.calculate_delay(1), 100);
    assert_eq!(cfg.calculate_delay(2), 200);
    assert_eq!(cfg.calculate_delay(3), 400);
    assert_eq!(cfg.calculate_delay(10), 1000); // capped at max
}

#[test]
fn test_retry_config_should_retry_classification() {
    // Create config with explicit retryable error types including RateLimitError
    let cfg = RetryConfig::new(3)
        .with_jitter(0.0)
        .with_retryable_errors(vec![
            RetryableErrorType::NetworkError,
            RetryableErrorType::RateLimitError,
            RetryableErrorType::TimeoutError,
        ]);

    let net = GraphBitError::Network {
        message: "x".into(),
    };
    assert!(cfg.should_retry(&net, 0));

    let rate = GraphBitError::rate_limit("p", 2);
    assert!(cfg.should_retry(&rate, 0));

    let cfg2 = RetryConfig {
        max_attempts: 1,
        ..RetryConfig::default()
    };
    assert!(!cfg2.should_retry(&net, 1)); // attempt >= max_attempts
}

// Circuit Breaker Tests
#[test]
fn test_circuit_breaker_transitions() {
    let mut cb = CircuitBreaker::new(CircuitBreakerConfig {
        failure_threshold: 2,
        recovery_timeout_ms: 100, // Increase timeout to ensure it passes
        success_threshold: 1,
        failure_window_ms: 1_000,
    });
    // Two failures -> open
    cb.record_failure();
    cb.record_failure();
    assert!(matches!(cb.state, CircuitBreakerState::Open { .. }));

    // Wait for recovery timeout
    std::thread::sleep(std::time::Duration::from_millis(150));

    // After timeout allow request -> half-open
    assert!(cb.should_allow_request());
    assert!(matches!(cb.state, CircuitBreakerState::HalfOpen));
    // Success closes
    cb.record_success();
    assert!(matches!(cb.state, CircuitBreakerState::Closed));
}

// Concurrency Stats Tests
#[test]
fn test_concurrency_stats_helpers() {
    let mut stats = ConcurrencyStats {
        total_permit_acquisitions: 4,
        total_wait_time_ms: 20,
        ..Default::default()
    };
    stats.calculate_avg_wait_time();
    assert_eq!(stats.avg_wait_time_ms, 5.0);
    assert!(stats.get_utilization(10) >= 0.0);
}

// Task Info Tests
#[test]
fn test_task_info_from_node_type() {
    use graphbit_core::graph::NodeType;
    let nid = NodeId::new();
    let info = TaskInfo::from_node_type(
        &NodeType::Transform {
            transformation: "x".into(),
        },
        &nid,
    );
    assert_eq!(info.node_type, "transform");
}

// Workflow Context Tests
#[test]
fn test_workflow_context_node_outputs_and_nested_get() {
    let mut ctx = WorkflowContext::new(WorkflowId::new());
    let nid = NodeId::new();
    ctx.set_node_output(&nid, serde_json::json!({"a": {"b": 1}}));
    let got = ctx.get_nested_output(&format!("{nid}.a.b")).unwrap();
    assert_eq!(got, &serde_json::json!(1));
}

// Additional comprehensive tests for 100% types coverage

#[test]
fn test_workflow_id_from_string() {
    // Test valid UUID string
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let workflow_id = WorkflowId::from_string(uuid_str).unwrap();
    assert_eq!(workflow_id.to_string(), uuid_str);

    // Test invalid UUID string
    let invalid_uuid = "invalid-uuid";
    let result = WorkflowId::from_string(invalid_uuid);
    assert!(result.is_err());
}

#[test]
fn test_agent_message_comprehensive() {
    let sender = AgentId::new();
    let recipient = AgentId::new();
    let content = MessageContent::Text("Test message".to_string());

    let message = AgentMessage::new(sender.clone(), Some(recipient.clone()), content.clone())
        .with_metadata("priority".to_string(), serde_json::Value::String("high".to_string()))
        .with_metadata("category".to_string(), serde_json::Value::String("test".to_string()));

    assert_eq!(message.sender, sender);
    assert_eq!(message.recipient, Some(recipient));
    assert!(matches!(message.content, MessageContent::Text(_)));
    assert_eq!(message.metadata.len(), 2);
    assert_eq!(message.metadata.get("priority"), Some(&serde_json::Value::String("high".to_string())));
    assert_eq!(message.metadata.get("category"), Some(&serde_json::Value::String("test".to_string())));
}

#[test]
fn test_workflow_context_comprehensive() {
    let workflow_id = WorkflowId::new();
    let mut context = WorkflowContext::new(workflow_id.clone());

    // Test variable operations
    context.set_variable("key1".to_string(), serde_json::Value::String("value1".to_string()));
    context.set_variable("key2".to_string(), serde_json::Value::Number(serde_json::Number::from(42)));

    assert_eq!(context.get_variable("key1"), Some(&serde_json::Value::String("value1".to_string())));
    assert_eq!(context.get_variable("key2"), Some(&serde_json::Value::Number(serde_json::Number::from(42))));
    assert_eq!(context.get_variable("nonexistent"), None);

    // Test metadata operations
    context.set_metadata("meta1".to_string(), serde_json::Value::String("metadata1".to_string()));
    assert_eq!(context.metadata.len(), 1);

    // Test state transitions
    assert!(matches!(context.state, WorkflowState::Pending));

    context.complete();
    assert!(matches!(context.state, WorkflowState::Completed));
    assert!(context.completed_at.is_some());

    // Test failure state
    let mut context2 = WorkflowContext::new(WorkflowId::new());
    context2.fail("Test error".to_string());
    assert!(matches!(context2.state, WorkflowState::Failed { .. }));
    assert!(context2.completed_at.is_some());

    // Test stats operations
    let stats = WorkflowExecutionStats {
        total_nodes: 5,
        successful_nodes: 4,
        failed_nodes: 1,
        avg_execution_time_ms: 200.0,
        max_concurrent_nodes: 3,
        total_execution_time_ms: 1000,
        peak_memory_usage_mb: Some(50.0),
        semaphore_acquisitions: 10,
        avg_semaphore_wait_ms: 5.0,
    };
    context.set_stats(stats.clone());
    assert!(context.get_stats().is_some());

    // Test execution duration
    let duration = context.execution_duration_ms();
    assert!(duration.is_some());
}

#[test]
fn test_workflow_context_node_outputs_comprehensive() {
    let mut context = WorkflowContext::new(WorkflowId::new());
    let node_id = NodeId::new();

    // Test node output operations
    let output = serde_json::json!({"result": "success", "count": 10});
    context.set_node_output(&node_id, output.clone());

    let retrieved = context.get_node_output(&node_id.to_string());
    assert_eq!(retrieved, Some(&output));

    // Test node output by name
    context.set_node_output_by_name("test_node", serde_json::json!({"data": "test"}));
    let retrieved_by_name = context.get_node_output("test_node");
    assert_eq!(retrieved_by_name, Some(&serde_json::json!({"data": "test"})));

    // Test nested output access
    let nested_output = serde_json::json!({
        "level1": {
            "level2": {
                "value": "nested_value"
            }
        }
    });
    context.set_node_output_by_name("nested_node", nested_output);

    let nested_value = context.get_nested_output("nested_node.level1.level2.value");
    assert_eq!(nested_value, Some(&serde_json::Value::String("nested_value".to_string())));

    // Test invalid nested path
    let invalid_nested = context.get_nested_output("nested_node.invalid.path");
    assert_eq!(invalid_nested, None);

    // Test empty path
    let empty_path = context.get_nested_output("");
    assert_eq!(empty_path, None);
}

#[test]
fn test_workflow_state_methods() {
    // Test terminal states
    let completed = WorkflowState::Completed;
    assert!(completed.is_terminal());
    assert!(!completed.is_running());
    assert!(!completed.is_paused());

    let failed = WorkflowState::Failed { error: "Test error".to_string() };
    assert!(failed.is_terminal());
    assert!(!failed.is_running());
    assert!(!failed.is_paused());

    let cancelled = WorkflowState::Cancelled;
    assert!(cancelled.is_terminal());
    assert!(!cancelled.is_running());
    assert!(!cancelled.is_paused());

    // Test running state
    let running = WorkflowState::Running { current_node: NodeId::new() };
    assert!(!running.is_terminal());
    assert!(running.is_running());
    assert!(!running.is_paused());

    // Test paused state
    let paused = WorkflowState::Paused {
        current_node: NodeId::new(),
        reason: "Test pause".to_string()
    };
    assert!(!paused.is_terminal());
    assert!(!paused.is_running());
    assert!(paused.is_paused());

    // Test pending state
    let pending = WorkflowState::Pending;
    assert!(!pending.is_terminal());
    assert!(!pending.is_running());
    assert!(!pending.is_paused());
}

#[test]
fn test_node_execution_result_comprehensive() {
    let node_id = NodeId::new();

    // Test successful result
    let success_result = NodeExecutionResult::success(
        serde_json::json!({"output": "test"}),
        node_id.clone()
    )
    .with_metadata("key1".to_string(), serde_json::Value::String("value1".to_string()))
    .with_duration(1500)
    .with_retry_count(2)
    .mark_completed();

    assert!(success_result.success);
    assert_eq!(success_result.output, serde_json::json!({"output": "test"}));
    assert_eq!(success_result.error, None);
    assert_eq!(success_result.duration_ms, 1500);
    assert_eq!(success_result.retry_count, 2);
    assert_eq!(success_result.metadata.len(), 1);
    assert!(success_result.completed_at.is_some());
    assert_eq!(success_result.node_id, node_id);

    // Test failure result
    let failure_result = NodeExecutionResult::failure(
        "Test error".to_string(),
        node_id.clone()
    )
    .with_metadata("error_type".to_string(), serde_json::Value::String("timeout".to_string()))
    .with_duration(5000)
    .with_retry_count(3)
    .mark_completed();

    assert!(!failure_result.success);
    assert_eq!(failure_result.output, serde_json::Value::Null);
    assert_eq!(failure_result.error, Some("Test error".to_string()));
    assert_eq!(failure_result.duration_ms, 5000);
    assert_eq!(failure_result.retry_count, 3);
    assert_eq!(failure_result.metadata.len(), 1);
    assert!(failure_result.completed_at.is_some());
    assert_eq!(failure_result.node_id, node_id);
}

#[test]
fn test_retry_config_comprehensive() {
    // Test basic retry config
    let retry_config = RetryConfig::new(5)
        .with_jitter(0.1)
        .with_retryable_errors(vec![
            RetryableErrorType::NetworkError,
            RetryableErrorType::TimeoutError,
        ]);

    assert_eq!(retry_config.max_attempts, 5);
    assert_eq!(retry_config.jitter_factor, 0.1);
    assert_eq!(retry_config.retryable_errors.len(), 2);

    // Test delay calculation
    let delay_0 = retry_config.calculate_delay(0);
    assert_eq!(delay_0, 0);

    let delay_1 = retry_config.calculate_delay(1);
    assert!(delay_1 > 0);

    let delay_2 = retry_config.calculate_delay(2);
    assert!(delay_2 > delay_1);

    // Test should_retry logic
    let network_error = GraphBitError::Network { message: "Connection failed".to_string() };
    assert!(retry_config.should_retry(&network_error, 1));
    assert!(retry_config.should_retry(&network_error, 4));
    assert!(!retry_config.should_retry(&network_error, 5)); // Max attempts reached

    let config_error = GraphBitError::config("Invalid config".to_string());
    assert!(!retry_config.should_retry(&config_error, 1)); // Not retryable
}

#[test]
fn test_retryable_error_type_classification() {
    // Test network error classification
    let network_error = GraphBitError::Network { message: "Connection failed".to_string() };
    let error_type = RetryableErrorType::from_error(&network_error);
    assert!(matches!(error_type, RetryableErrorType::NetworkError));

    // Test timeout error classification
    let timeout_error = GraphBitError::Network { message: "Request timed out".to_string() };
    let error_type = RetryableErrorType::from_error(&timeout_error);
    assert!(matches!(error_type, RetryableErrorType::TimeoutError));

    // Test rate limit error classification
    let rate_limit_error = GraphBitError::rate_limit("openai".to_string(), 60);
    let error_type = RetryableErrorType::from_error(&rate_limit_error);
    assert!(matches!(error_type, RetryableErrorType::RateLimitError));

    // Test other error classification
    let config_error = GraphBitError::config("Invalid config".to_string());
    let error_type = RetryableErrorType::from_error(&config_error);
    assert!(matches!(error_type, RetryableErrorType::Other));
}

#[test]
fn test_circuit_breaker_comprehensive() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        success_threshold: 2,
        recovery_timeout_ms: 5000,
        failure_window_ms: 60000,
    };

    let mut circuit_breaker = CircuitBreaker::new(config.clone());

    // Initially closed, should allow requests
    assert!(circuit_breaker.should_allow_request());

    // Record failures to trigger opening
    circuit_breaker.record_failure();
    assert!(circuit_breaker.should_allow_request()); // Still closed

    circuit_breaker.record_failure();
    assert!(circuit_breaker.should_allow_request()); // Still closed

    circuit_breaker.record_failure();
    assert!(!circuit_breaker.should_allow_request()); // Now open

    // Test success recording in closed state
    let mut cb2 = CircuitBreaker::new(config);
    cb2.record_success();
    assert!(cb2.should_allow_request());
}

#[test]
fn test_concurrency_config_variants() {
    // Test high throughput config
    let high_throughput = ConcurrencyConfig::high_throughput();
    assert_eq!(high_throughput.global_max_concurrency, 200);
    assert_eq!(high_throughput.get_node_type_limit("agent"), 50);
    assert_eq!(high_throughput.get_node_type_limit("http_request"), 32);
    assert_eq!(high_throughput.get_node_type_limit("transform"), 64);
    assert_eq!(high_throughput.get_node_type_limit("condition"), 128);

    // Test low latency config
    let low_latency = ConcurrencyConfig::low_latency();
    assert_eq!(low_latency.global_max_concurrency, 24);
    assert_eq!(low_latency.get_node_type_limit("agent"), 10);
    assert_eq!(low_latency.get_node_type_limit("http_request"), 8);

    // Test memory optimized config
    let memory_optimized = ConcurrencyConfig::memory_optimized();
    assert_eq!(memory_optimized.global_max_concurrency, 32);
    assert_eq!(memory_optimized.get_node_type_limit("agent"), 5);

    // Test unknown node type (should return default)
    let default_limit = high_throughput.get_node_type_limit("unknown_type");
    assert_eq!(default_limit, high_throughput.global_max_concurrency / 4);
}

#[test]
fn test_task_info_creation_comprehensive() {
    let task_id = NodeId::new();
    let agent_id = AgentId::new();

    // Test agent task
    let agent_task = TaskInfo::agent_task(agent_id, task_id.clone());
    assert_eq!(agent_task.node_type, "agent");
    assert_eq!(agent_task.task_id, task_id);

    // Test HTTP task
    let http_task = TaskInfo::http_task(task_id.clone());
    assert_eq!(http_task.node_type, "http_request");
    assert_eq!(http_task.task_id, task_id);

    // Test transform task
    let transform_task = TaskInfo::transform_task(task_id.clone());
    assert_eq!(transform_task.node_type, "transform");
    assert_eq!(transform_task.task_id, task_id);

    // Test condition task
    let condition_task = TaskInfo::condition_task(task_id.clone());
    assert_eq!(condition_task.node_type, "condition");
    assert_eq!(condition_task.task_id, task_id);

    // Test delay task
    let delay_task = TaskInfo::delay_task(task_id.clone(), 5000);
    assert_eq!(delay_task.node_type, "delay");
    assert_eq!(delay_task.task_id, task_id);
}

#[test]
fn test_concurrency_stats_operations() {
    let mut stats = ConcurrencyStats {
        total_permit_acquisitions: 100,
        total_wait_time_ms: 5000,
        peak_active_tasks: 25,
        permit_failures: 5,
        current_active_tasks: 10,
        avg_wait_time_ms: 0.0,
    };

    // Test average wait time calculation
    stats.calculate_avg_wait_time();
    assert_eq!(stats.avg_wait_time_ms, 50.0); // 5000 / 100

    // Test utilization calculation
    let utilization = stats.get_utilization(50);
    assert_eq!(utilization, 20.0); // (10 / 50) * 100

    // Test utilization with zero capacity
    let zero_utilization = stats.get_utilization(0);
    assert_eq!(zero_utilization, 0.0);

    // Test with zero acquisitions
    let mut zero_stats = ConcurrencyStats {
        total_permit_acquisitions: 0,
        total_wait_time_ms: 1000,
        peak_active_tasks: 0,
        permit_failures: 0,
        current_active_tasks: 0,
        avg_wait_time_ms: 0.0,
    };

    zero_stats.calculate_avg_wait_time();
    assert_eq!(zero_stats.avg_wait_time_ms, 0.0);
}

#[test]
fn test_concurrency_manager_creation() {
    let config = ConcurrencyConfig::high_throughput();
    let _manager = ConcurrencyManager::new(config.clone());

    // Verify manager was created (config is private, so just check it doesn't panic)
    // The manager should be created successfully with the provided config
}

// Additional comprehensive tests for 100% coverage

#[test]
fn test_default_implementations_comprehensive() {
    // Test all Default implementations
    let _agent_id = AgentId::default();
    let _workflow_id = WorkflowId::default();
    let _node_id = NodeId::default();
    let _agent_message = AgentMessage::default();
    let _workflow_context = WorkflowContext::default();
    let _node_result = NodeExecutionResult::default();
    let _retry_config = RetryConfig::default();
    let _circuit_breaker_config = CircuitBreakerConfig::default();
    let _concurrency_config = ConcurrencyConfig::default();

    // Verify default values are reasonable
    assert_eq!(RetryConfig::default().max_attempts, 3);
    assert_eq!(CircuitBreakerConfig::default().failure_threshold, 5);
    assert_eq!(ConcurrencyConfig::default().global_max_concurrency, 16);
}

#[test]
fn test_display_implementations() {
    // Test Display trait implementations
    let agent_id = AgentId::new();
    let workflow_id = WorkflowId::new();
    let node_id = NodeId::new();

    let agent_str = format!("{}", agent_id);
    let workflow_str = format!("{}", workflow_id);
    let node_str = format!("{}", node_id);

    // Verify they produce valid UUID strings
    assert!(!agent_str.is_empty());
    assert!(!workflow_str.is_empty());
    assert!(!node_str.is_empty());
    assert!(agent_str.contains('-'));
    assert!(workflow_str.contains('-'));
    assert!(node_str.contains('-'));
}

#[test]
fn test_workflow_context_default_implementation() {
    let context = WorkflowContext::default();

    // Verify default state
    assert!(matches!(context.state, WorkflowState::Pending));
    assert!(context.variables.is_empty());
    assert!(context.metadata.is_empty());
    assert!(context.node_outputs.is_empty());
    assert!(context.stats.is_none());
    assert!(context.completed_at.is_none());
}

#[test]
fn test_agent_message_default_implementation() {
    let message = AgentMessage::default();

    // Verify default values
    assert!(message.recipient.is_none());
    assert!(matches!(message.content, MessageContent::Text(_)));
    assert!(message.metadata.is_empty());
}

#[test]
fn test_node_execution_result_default_implementation() {
    let result = NodeExecutionResult::default();

    // Verify default values
    assert!(!result.success);
    assert_eq!(result.output, serde_json::Value::Null);
    assert!(result.error.is_none());
    assert!(result.metadata.is_empty());
    assert_eq!(result.duration_ms, 0);
    assert_eq!(result.retry_count, 0);
    assert!(result.completed_at.is_none());
}

#[test]
fn test_circuit_breaker_config_default() {
    let config = CircuitBreakerConfig::default();

    // Verify default values
    assert_eq!(config.failure_threshold, 5);
    assert_eq!(config.recovery_timeout_ms, 60000); // DEFAULT_RECOVERY_TIMEOUT_MS (1 minute)
    assert_eq!(config.success_threshold, 3);
    assert_eq!(config.failure_window_ms, 300000); // DEFAULT_FAILURE_WINDOW_MS (5 minutes)
}

#[test]
fn test_concurrency_config_default() {
    let config = ConcurrencyConfig::default();

    // Verify default values
    assert_eq!(config.global_max_concurrency, 16);
    assert_eq!(config.get_node_type_limit("agent"), 4);
    assert_eq!(config.get_node_type_limit("http_request"), 8);
    assert_eq!(config.get_node_type_limit("transform"), 16);
    assert_eq!(config.get_node_type_limit("condition"), 32);
    assert_eq!(config.get_node_type_limit("delay"), 1); // Actual default is 1
    // document_loader and custom are not in default config, so they get global_max/4
    assert_eq!(config.get_node_type_limit("document_loader"), 4); // 16/4 = 4
    assert_eq!(config.get_node_type_limit("custom"), 4); // 16/4 = 4
}

#[test]
fn test_workflow_state_paused_variant() {
    let node_id = NodeId::new();
    let paused_state = WorkflowState::Paused {
        current_node: node_id.clone(),
        reason: "User requested pause".to_string(),
    };

    // Test paused state methods
    assert!(!paused_state.is_terminal());
    assert!(!paused_state.is_running());
    assert!(paused_state.is_paused());
}

#[test]
fn test_workflow_context_execution_duration_edge_cases() {
    let mut context = WorkflowContext::new(WorkflowId::new());

    // Test duration when not completed - it still returns a duration (current time - start time)
    let duration_not_completed = context.execution_duration_ms();
    assert!(duration_not_completed.is_some());

    // Test duration when completed
    context.complete();
    let duration = context.execution_duration_ms();
    assert!(duration.is_some());
    // Remove the useless comparison warning
    assert!(duration.unwrap() < 10000); // Should be less than 10 seconds for this test
}

#[test]
fn test_retry_config_edge_cases() {
    // Test with zero max attempts
    let config = RetryConfig::new(0);
    assert_eq!(config.max_attempts, 0);

    // Test jitter clamping
    let config = RetryConfig::default()
        .with_jitter(2.0); // Should be clamped to 1.0
    assert_eq!(config.jitter_factor, 1.0);

    let config = RetryConfig::default()
        .with_jitter(-0.5); // Should be clamped to 0.0
    assert_eq!(config.jitter_factor, 0.0);

    // Test delay calculation with high attempt numbers
    let config = RetryConfig::default()
        .with_exponential_backoff(100, 2.0, 1000)
        .with_jitter(0.0); // Remove jitter for predictable results
    let delay = config.calculate_delay(100); // Very high attempt
    assert_eq!(delay, 1000); // Should be capped at max_delay_ms
}

#[test]
fn test_retryable_error_type_comprehensive() {
    // Test all error type classifications
    let network_error = GraphBitError::Network { message: "Connection failed".to_string() };
    assert!(matches!(RetryableErrorType::from_error(&network_error), RetryableErrorType::NetworkError));

    let timeout_error = GraphBitError::Network { message: "Request timed out".to_string() };
    assert!(matches!(RetryableErrorType::from_error(&timeout_error), RetryableErrorType::TimeoutError));

    let rate_limit_error = GraphBitError::rate_limit("api".to_string(), 60);
    assert!(matches!(RetryableErrorType::from_error(&rate_limit_error), RetryableErrorType::RateLimitError));

    let config_error = GraphBitError::config("Invalid config".to_string());
    assert!(matches!(RetryableErrorType::from_error(&config_error), RetryableErrorType::Other));

    // Test error message patterns
    let connection_error = GraphBitError::Network { message: "connection refused".to_string() };
    assert!(matches!(RetryableErrorType::from_error(&connection_error), RetryableErrorType::NetworkError));

    let dns_error = GraphBitError::Network { message: "dns resolution failed".to_string() };
    assert!(matches!(RetryableErrorType::from_error(&dns_error), RetryableErrorType::NetworkError));
}

#[test]
fn test_circuit_breaker_half_open_state_transitions() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        recovery_timeout_ms: 100,
        success_threshold: 2,
        failure_window_ms: 1000,
    };

    let mut breaker = CircuitBreaker::new(config);

    // Trigger failures to open the breaker
    breaker.record_failure();
    breaker.record_failure();
    assert!(matches!(breaker.state, CircuitBreakerState::Open { .. }));

    // Wait for recovery timeout
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Should transition to half-open on first request
    assert!(breaker.should_allow_request());
    assert!(matches!(breaker.state, CircuitBreakerState::HalfOpen));

    // Record one success (need 2 for success_threshold)
    breaker.record_success();
    assert!(matches!(breaker.state, CircuitBreakerState::HalfOpen));

    // Record second success - should close
    breaker.record_success();
    assert!(matches!(breaker.state, CircuitBreakerState::Closed));
}

#[test]
fn test_circuit_breaker_half_open_failure() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        recovery_timeout_ms: 100,
        success_threshold: 2,
        failure_window_ms: 1000,
    };

    let mut breaker = CircuitBreaker::new(config);

    // Open the breaker
    breaker.record_failure();
    breaker.record_failure();

    // Wait and transition to half-open
    std::thread::sleep(std::time::Duration::from_millis(150));
    assert!(breaker.should_allow_request());

    // Failure in half-open should reopen
    breaker.record_failure();
    assert!(matches!(breaker.state, CircuitBreakerState::Open { .. }));
}

#[test]
fn test_concurrency_manager_async_methods() {
    use tokio::runtime::Runtime;

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let config = ConcurrencyConfig::default();
        let manager = ConcurrencyManager::new(config);

        // Test get_stats
        let stats = manager.get_stats().await;
        assert_eq!(stats.current_active_tasks, 0);
        assert_eq!(stats.total_permit_acquisitions, 0);

        // Test get_available_permits
        let permits = manager.get_available_permits().await;
        assert!(permits.contains_key("agent"));
        assert!(permits.contains_key("http_request"));
        assert_eq!(permits.get("agent"), Some(&4)); // Default limit
    });
}

#[test]
fn test_concurrency_permits_drop_behavior() {
    // Test that ConcurrencyPermits implements Drop trait
    // We can't test the actual drop behavior directly due to private fields,
    // but we can verify the Drop trait is implemented by checking compilation
    use std::mem;

    // This test verifies that ConcurrencyPermits has a Drop implementation
    // The actual drop behavior is tested indirectly through the concurrency manager
    let drop_fn_exists = mem::needs_drop::<ConcurrencyPermits>();
    assert!(drop_fn_exists, "ConcurrencyPermits should implement Drop");
}

#[test]
fn test_workflow_context_nested_output_edge_cases() {
    let mut context = WorkflowContext::new(WorkflowId::new());
    let node_id = NodeId::new();

    // Test with complex nested structure
    let complex_output = serde_json::json!({
        "level1": {
            "level2": {
                "level3": {
                    "value": "deep_value",
                    "array": [1, 2, 3],
                    "null_value": null
                }
            }
        },
        "root_array": [
            {"item": "first"},
            {"item": "second"}
        ]
    });

    context.set_node_output(&node_id, complex_output);

    // Test deep nested access
    let deep_value = context.get_nested_output(&format!("{}.level1.level2.level3.value", node_id));
    assert_eq!(deep_value, Some(&serde_json::Value::String("deep_value".to_string())));

    // Test null value access
    let null_value = context.get_nested_output(&format!("{}.level1.level2.level3.null_value", node_id));
    assert_eq!(null_value, Some(&serde_json::Value::Null));

    // Test non-existent deep path
    let missing = context.get_nested_output(&format!("{}.level1.level2.level3.missing", node_id));
    assert_eq!(missing, None);

    // Test invalid path structure
    let invalid = context.get_nested_output(&format!("{}.level1.level2.level3.value.invalid", node_id));
    assert_eq!(invalid, None);

    // Test empty reference
    let empty = context.get_nested_output("");
    assert_eq!(empty, None);

    // Test single part reference (should work)
    let single = context.get_nested_output(&node_id.to_string());
    assert!(single.is_some());
}

#[test]
fn test_message_content_all_variants() {
    // Test all MessageContent variants for completeness
    let text_content = MessageContent::Text("Hello".to_string());
    let data_content = MessageContent::Data(serde_json::json!({"key": "value"}));
    let tool_call = MessageContent::ToolCall {
        tool_name: "test_tool".to_string(),
        parameters: serde_json::json!({"param": "value"}),
    };
    let tool_response = MessageContent::ToolResponse {
        tool_name: "test_tool".to_string(),
        result: serde_json::json!("success"),
        success: true,
    };
    let error_content = MessageContent::Error {
        error_code: "TEST_ERROR".to_string(),
        error_message: "Test error message".to_string(),
    };

    // Verify they can be created and matched
    assert!(matches!(text_content, MessageContent::Text(_)));
    assert!(matches!(data_content, MessageContent::Data(_)));
    assert!(matches!(tool_call, MessageContent::ToolCall { .. }));
    assert!(matches!(tool_response, MessageContent::ToolResponse { .. }));
    assert!(matches!(error_content, MessageContent::Error { .. }));
}

#[test]
fn test_agent_capability_all_variants() {
    // Test all AgentCapability variants
    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::ToolExecution,
        AgentCapability::DecisionMaking,
        AgentCapability::Custom("CustomCapability".to_string()),
    ];

    // Verify they can be created and are distinct
    assert_eq!(capabilities.len(), 5);
    assert_ne!(capabilities[0], capabilities[1]);
    assert_ne!(capabilities[3], capabilities[4]);

    // Test custom capability equality
    let custom1 = AgentCapability::Custom("test".to_string());
    let custom2 = AgentCapability::Custom("test".to_string());
    let custom3 = AgentCapability::Custom("different".to_string());

    assert_eq!(custom1, custom2);
    assert_ne!(custom1, custom3);
}

#[test]
fn test_workflow_execution_stats_comprehensive() {
    let stats = WorkflowExecutionStats {
        total_nodes: 10,
        successful_nodes: 8,
        failed_nodes: 2,
        avg_execution_time_ms: 150.5,
        max_concurrent_nodes: 5,
        total_execution_time_ms: 3000,
        peak_memory_usage_mb: Some(128.5),
        semaphore_acquisitions: 25,
        avg_semaphore_wait_ms: 12.3,
    };

    // Verify all fields are accessible
    assert_eq!(stats.total_nodes, 10);
    assert_eq!(stats.successful_nodes, 8);
    assert_eq!(stats.failed_nodes, 2);
    assert_eq!(stats.avg_execution_time_ms, 150.5);
    assert_eq!(stats.max_concurrent_nodes, 5);
    assert_eq!(stats.total_execution_time_ms, 3000);
    assert_eq!(stats.peak_memory_usage_mb, Some(128.5));
    assert_eq!(stats.semaphore_acquisitions, 25);
    assert_eq!(stats.avg_semaphore_wait_ms, 12.3);
}

#[test]
fn test_uuid_access_methods() {
    let agent_id = AgentId::new();
    let workflow_id = WorkflowId::new();
    let node_id = NodeId::new();

    // Test as_uuid methods
    let agent_uuid = agent_id.as_uuid();
    let workflow_uuid = workflow_id.as_uuid();
    let node_uuid = node_id.as_uuid();

    // Verify they return valid UUIDs
    assert!(!agent_uuid.is_nil());
    assert!(!workflow_uuid.is_nil());
    assert!(!node_uuid.is_nil());

    // Verify they're different
    assert_ne!(agent_uuid, workflow_uuid);
    assert_ne!(workflow_uuid, node_uuid);
    assert_ne!(agent_uuid, node_uuid);
}

#[test]
fn test_id_from_string_edge_cases() {
    // Test with valid UUID strings
    let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
    let agent_id = AgentId::from_string(valid_uuid).unwrap();
    let workflow_id = WorkflowId::from_string(valid_uuid).unwrap();
    let node_id = NodeId::from_string(valid_uuid).unwrap();

    assert_eq!(agent_id.to_string(), valid_uuid);
    assert_eq!(workflow_id.to_string(), valid_uuid);
    assert_eq!(node_id.to_string(), valid_uuid);

    // Test with invalid UUID strings (should generate deterministic UUIDs for Agent/Node)
    let invalid_uuid = "not-a-uuid";
    let agent_result = AgentId::from_string(invalid_uuid);
    let workflow_result = WorkflowId::from_string(invalid_uuid);
    let node_result = NodeId::from_string(invalid_uuid);

    // AgentId and NodeId should succeed with deterministic generation
    assert!(agent_result.is_ok());
    assert!(node_result.is_ok());

    // WorkflowId should fail for invalid UUID
    assert!(workflow_result.is_err());

    // Test deterministic generation
    let agent1 = AgentId::from_string("test").unwrap();
    let agent2 = AgentId::from_string("test").unwrap();
    assert_eq!(agent1, agent2);

    let node1 = NodeId::from_string("test").unwrap();
    let node2 = NodeId::from_string("test").unwrap();
    assert_eq!(node1, node2);
}

#[test]
fn test_retry_config_with_exponential_backoff_edge_cases() {
    // Test with zero initial delay
    let config = RetryConfig::new(3)
        .with_exponential_backoff(0, 2.0, 1000);

    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 0);
    assert_eq!(config.calculate_delay(2), 0);

    // Test with multiplier of 1.0 (no exponential growth)
    let config = RetryConfig::new(3)
        .with_exponential_backoff(100, 1.0, 1000)
        .with_jitter(0.0); // Remove jitter for predictable results

    assert_eq!(config.calculate_delay(1), 100);
    assert_eq!(config.calculate_delay(2), 100);
    assert_eq!(config.calculate_delay(3), 100);

    // Test with very high multiplier
    let config = RetryConfig::new(5)
        .with_exponential_backoff(10, 10.0, 1000)
        .with_jitter(0.0); // Remove jitter for predictable results

    assert_eq!(config.calculate_delay(1), 10);
    assert_eq!(config.calculate_delay(2), 100);
    assert_eq!(config.calculate_delay(3), 1000); // Capped at max
    assert_eq!(config.calculate_delay(4), 1000); // Still capped
}

#[test]
fn test_retry_config_with_jitter_comprehensive_coverage() {
    // Test with_jitter method with various values to ensure 100% coverage
    let config_default = RetryConfig::new(3);
    assert_eq!(config_default.jitter_factor, 0.1); // Default jitter

    // Test with_jitter with normal values
    let config_half = RetryConfig::new(3).with_jitter(0.5);
    assert_eq!(config_half.jitter_factor, 0.5);

    let config_quarter = RetryConfig::new(3).with_jitter(0.25);
    assert_eq!(config_quarter.jitter_factor, 0.25);

    let config_three_quarters = RetryConfig::new(3).with_jitter(0.75);
    assert_eq!(config_three_quarters.jitter_factor, 0.75);

    // Test boundary values
    let config_zero = RetryConfig::new(3).with_jitter(0.0);
    assert_eq!(config_zero.jitter_factor, 0.0);

    let config_one = RetryConfig::new(3).with_jitter(1.0);
    assert_eq!(config_one.jitter_factor, 1.0);

    // Test clamping - values above 1.0
    let config_above_one = RetryConfig::new(3).with_jitter(1.5);
    assert_eq!(config_above_one.jitter_factor, 1.0);

    let config_way_above = RetryConfig::new(3).with_jitter(10.0);
    assert_eq!(config_way_above.jitter_factor, 1.0);

    let config_max = RetryConfig::new(3).with_jitter(f64::MAX);
    assert_eq!(config_max.jitter_factor, 1.0);

    // Test clamping - values below 0.0
    let config_negative = RetryConfig::new(3).with_jitter(-0.5);
    assert_eq!(config_negative.jitter_factor, 0.0);

    let config_way_negative = RetryConfig::new(3).with_jitter(-10.0);
    assert_eq!(config_way_negative.jitter_factor, 0.0);

    let config_min = RetryConfig::new(3).with_jitter(f64::MIN);
    assert_eq!(config_min.jitter_factor, 0.0);

    // Test chaining with other methods
    let config_chained = RetryConfig::new(5)
        .with_jitter(0.0) // Use 0.0 jitter for predictable results
        .with_exponential_backoff(100, 2.0, 1000);
    assert_eq!(config_chained.jitter_factor, 0.0);
    assert_eq!(config_chained.max_attempts, 5);
    // Test that the delay calculation works correctly with the chained config
    assert_eq!(config_chained.calculate_delay(1), 100);
}

#[test]
fn test_should_retry_comprehensive_error_types() {
    let config = RetryConfig::new(3)
        .with_retryable_errors(vec![
            RetryableErrorType::NetworkError,
            RetryableErrorType::TimeoutError,
        ]);

    // Test retryable errors
    let network_error = GraphBitError::Network { message: "connection failed".to_string() };
    assert!(config.should_retry(&network_error, 0));
    assert!(config.should_retry(&network_error, 1));
    assert!(config.should_retry(&network_error, 2));
    assert!(!config.should_retry(&network_error, 3)); // Max attempts reached

    // Test non-retryable error
    let config_error = GraphBitError::config("invalid".to_string());
    assert!(!config.should_retry(&config_error, 0));
    assert!(!config.should_retry(&config_error, 1));

    // Test rate limit error (not in retryable list)
    let rate_limit = GraphBitError::rate_limit("api".to_string(), 60);
    assert!(!config.should_retry(&rate_limit, 0));
}

#[test]
fn test_concurrency_stats_edge_cases() {
    let mut stats = ConcurrencyStats {
        total_permit_acquisitions: 0,
        total_wait_time_ms: 1000,
        peak_active_tasks: 0,
        permit_failures: 0,
        current_active_tasks: 0,
        avg_wait_time_ms: 0.0,
    };

    // Test with zero acquisitions
    stats.calculate_avg_wait_time();
    assert_eq!(stats.avg_wait_time_ms, 0.0);

    // Test utilization with zero capacity
    let utilization = stats.get_utilization(0);
    assert_eq!(utilization, 0.0);

    // Test utilization with normal values
    stats.current_active_tasks = 5;
    let utilization = stats.get_utilization(10);
    assert_eq!(utilization, 50.0);

    // Test utilization at 100%
    stats.current_active_tasks = 10;
    let utilization = stats.get_utilization(10);
    assert_eq!(utilization, 100.0);
}

#[test]
fn test_task_info_from_node_type_comprehensive() {
    use graphbit_core::graph::NodeType;

    let task_id = NodeId::new();
    let agent_id = AgentId::new();

    // Test all node types
    let agent_node = NodeType::Agent {
        agent_id: agent_id.clone(),
        prompt_template: "test".to_string(),
    };
    let info = TaskInfo::from_node_type(&agent_node, &task_id);
    assert_eq!(info.node_type, "agent");
    assert_eq!(info.task_id, task_id);

    let transform_node = NodeType::Transform {
        transformation: "test".to_string(),
    };
    let info = TaskInfo::from_node_type(&transform_node, &task_id);
    assert_eq!(info.node_type, "transform");

    let condition_node = NodeType::Condition {
        expression: "test".to_string(),
    };
    let info = TaskInfo::from_node_type(&condition_node, &task_id);
    assert_eq!(info.node_type, "condition");

    let delay_node = NodeType::Delay {
        duration_seconds: 5,
    };
    let info = TaskInfo::from_node_type(&delay_node, &task_id);
    assert_eq!(info.node_type, "delay");

    let http_node = NodeType::HttpRequest {
        url: "https://example.com".to_string(),
        method: "GET".to_string(),
        headers: HashMap::new(),
    };
    let info = TaskInfo::from_node_type(&http_node, &task_id);
    assert_eq!(info.node_type, "http_request");

    let doc_loader_node = NodeType::DocumentLoader {
        document_type: "pdf".to_string(),
        source_path: "/path/to/doc.pdf".to_string(),
        encoding: Some("utf-8".to_string()),
    };
    let info = TaskInfo::from_node_type(&doc_loader_node, &task_id);
    assert_eq!(info.node_type, "document_loader");

    let custom_node = NodeType::Custom {
        function_name: "my_function".to_string(),
    };
    let info = TaskInfo::from_node_type(&custom_node, &task_id);
    assert_eq!(info.node_type, "generic"); // Custom maps to "generic" in the match

    let split_node = NodeType::Split;
    let info = TaskInfo::from_node_type(&split_node, &task_id);
    assert_eq!(info.node_type, "generic"); // Split maps to "generic" in the match

    let join_node = NodeType::Join;
    let info = TaskInfo::from_node_type(&join_node, &task_id);
    assert_eq!(info.node_type, "generic"); // Join maps to "generic" in the match
}
