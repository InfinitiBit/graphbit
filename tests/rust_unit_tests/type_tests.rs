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
