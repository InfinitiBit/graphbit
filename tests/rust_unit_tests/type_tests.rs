use graphbit_core::errors::GraphBitError;
use graphbit_core::types::*;
use graphbit_core::{AgentId, MessageContent, NodeId, WorkflowId};

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
