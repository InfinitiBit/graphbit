// Comprehensive types unit tests
//
// Tests for all core types, ID generation, message handling,
// workflow context, and type serialization.

use graphbit_core::types::*;
use serde_json::json;
// use uuid::Uuid; // Not needed, using graphbit_core types

#[test]
fn test_agent_id_creation_and_conversion() {
    // Test new random ID
    let id1 = AgentId::new();
    let id2 = AgentId::new();
    assert_ne!(id1, id2);

    // Test from valid UUID string
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let id_from_uuid = AgentId::from_string(uuid_str).unwrap();
    assert_eq!(id_from_uuid.to_string(), uuid_str);

    // Test from non-UUID string (should generate deterministic UUID)
    let id_from_str1 = AgentId::from_string("test-agent").unwrap();
    let id_from_str2 = AgentId::from_string("test-agent").unwrap();
    assert_eq!(id_from_str1, id_from_str2); // Should be deterministic

    let id_from_different = AgentId::from_string("different-agent").unwrap();
    assert_ne!(id_from_str1, id_from_different);
}

#[test]
fn test_node_id_and_workflow_id() {
    let node_id1 = NodeId::new();
    let node_id2 = NodeId::new();
    assert_ne!(node_id1, node_id2);

    let workflow_id1 = WorkflowId::new();
    let workflow_id2 = WorkflowId::new();
    assert_ne!(workflow_id1, workflow_id2);

    // Test serialization
    let serialized = serde_json::to_string(&node_id1).unwrap();
    let deserialized: NodeId = serde_json::from_str(&serialized).unwrap();
    assert_eq!(node_id1, deserialized);
}

#[test]
fn test_agent_message_creation_and_metadata() {
    let sender = AgentId::new();
    let recipient = AgentId::new();
    let content = MessageContent::Text("Hello, world!".to_string());

    let message = AgentMessage::new(sender.clone(), Some(recipient.clone()), content.clone());

    assert_eq!(message.sender, sender);
    assert_eq!(message.recipient, Some(recipient));
    assert!(matches!(message.content, MessageContent::Text(_)));
    assert!(message.metadata.is_empty());

    // Test with metadata
    let message_with_metadata = message
        .with_metadata("priority".to_string(), json!("high"))
        .with_metadata("timestamp".to_string(), json!(1234567890));

    assert_eq!(message_with_metadata.metadata.len(), 2);
    assert_eq!(
        message_with_metadata.metadata.get("priority").unwrap(),
        &json!("high")
    );
}

#[test]
fn test_message_content_variants() {
    // Test Text variant
    let text_content = MessageContent::Text("Hello".to_string());
    let serialized = serde_json::to_string(&text_content).unwrap();
    let deserialized: MessageContent = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(deserialized, MessageContent::Text(_)));

    // Test Data variant
    let data_content = MessageContent::Data(json!({"key": "value", "number": 42}));
    let serialized = serde_json::to_string(&data_content).unwrap();
    let deserialized: MessageContent = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(deserialized, MessageContent::Data(_)));

    // Test ToolCall variant
    let tool_call = MessageContent::ToolCall {
        tool_name: "calculator".to_string(),
        parameters: json!({"operation": "add", "a": 1, "b": 2}),
    };
    let serialized = serde_json::to_string(&tool_call).unwrap();
    let deserialized: MessageContent = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(deserialized, MessageContent::ToolCall { .. }));

    // Test ToolResponse variant
    let tool_response = MessageContent::ToolResponse {
        tool_name: "calculator".to_string(),
        result: json!(3),
        success: true,
    };
    let serialized = serde_json::to_string(&tool_response).unwrap();
    let deserialized: MessageContent = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(deserialized, MessageContent::ToolResponse { .. }));

    // Test Error variant
    let error_content = MessageContent::Error {
        error_code: "INVALID_INPUT".to_string(),
        error_message: "The input is invalid".to_string(),
    };
    let serialized = serde_json::to_string(&error_content).unwrap();
    let deserialized: MessageContent = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(deserialized, MessageContent::Error { .. }));
}

#[test]
fn test_agent_capabilities() {
    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::ToolExecution,
        AgentCapability::DecisionMaking,
        AgentCapability::Custom("CustomCapability".to_string()),
    ];

    for capability in capabilities {
        // Test serialization/deserialization
        let serialized = serde_json::to_string(&capability).unwrap();
        let deserialized: AgentCapability = serde_json::from_str(&serialized).unwrap();
        assert_eq!(capability, deserialized);
    }

    // Test equality
    let custom1 = AgentCapability::Custom("test".to_string());
    let custom2 = AgentCapability::Custom("test".to_string());
    let custom3 = AgentCapability::Custom("different".to_string());

    assert_eq!(custom1, custom2);
    assert_ne!(custom1, custom3);
    assert_ne!(
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis
    );
}

#[test]
fn test_workflow_context_operations() {
    let workflow_id = WorkflowId::new();
    let mut context = WorkflowContext::new(workflow_id.clone());

    assert_eq!(context.workflow_id, workflow_id);
    assert!(matches!(context.state, WorkflowState::Pending));

    // Test metadata operations
    context.set_metadata("key1".to_string(), json!("value1"));
    context.set_metadata("key2".to_string(), json!(42));

    assert_eq!(context.metadata.get("key1").unwrap(), &json!("value1"));
    assert_eq!(context.metadata.get("key2").unwrap(), &json!(42));
    assert!(!context.metadata.contains_key("nonexistent"));

    // Test node output operations
    let node_id = NodeId::new();
    let output_data = json!({"result": "success", "value": 123});
    context.set_node_output(&node_id, output_data.clone());

    assert_eq!(
        context.node_outputs.get(&node_id.to_string()).unwrap(),
        &output_data
    );

    // Test nested output access
    let nested_value = context
        .get_nested_output(&format!("{node_id}.result"))
        .unwrap();
    assert_eq!(nested_value, &json!("success"));

    let nested_number = context
        .get_nested_output(&format!("{node_id}.value"))
        .unwrap();
    assert_eq!(nested_number, &json!(123));

    // Test non-existent nested path
    assert!(context
        .get_nested_output(&format!("{node_id}.nonexistent"))
        .is_none());
}

#[test]
fn test_workflow_state_variants() {
    let node_id = NodeId::new();

    let states = vec![
        WorkflowState::Pending,
        WorkflowState::Running {
            current_node: node_id.clone(),
        },
        WorkflowState::Completed,
        WorkflowState::Failed {
            error: "Test error".to_string(),
        },
        WorkflowState::Cancelled,
    ];

    for state in states {
        // Test serialization/deserialization
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: WorkflowState = serde_json::from_str(&serialized).unwrap();

        // Compare discriminants since states contain different data
        match (&state, &deserialized) {
            (WorkflowState::Pending, WorkflowState::Pending) => {}
            (WorkflowState::Running { .. }, WorkflowState::Running { .. }) => {}
            (WorkflowState::Completed, WorkflowState::Completed) => {}
            (WorkflowState::Failed { .. }, WorkflowState::Failed { .. }) => {}
            (WorkflowState::Cancelled, WorkflowState::Cancelled) => {}
            _ => panic!("State serialization/deserialization mismatch"),
        }
    }
}

#[test]
fn test_retry_config_builder_pattern() {
    let config = RetryConfig::new(5)
        .with_exponential_backoff(100, 2.0, 10000)
        .with_jitter(0.2);

    assert_eq!(config.max_attempts, 5);
    assert_eq!(config.initial_delay_ms, 100);
    assert_eq!(config.backoff_multiplier, 2.0);
    assert_eq!(config.max_delay_ms, 10000);
    assert_eq!(config.jitter_factor, 0.2);

    // Test delay calculation
    assert_eq!(config.calculate_delay(0), 0); // No delay for first attempt

    let delay1 = config.calculate_delay(1);
    assert!((80..=120).contains(&delay1)); // 100 ± 20% jitter

    let delay2 = config.calculate_delay(2);
    assert!((160..=240).contains(&delay2)); // 200 ± 20% jitter
}

#[test]
fn test_circuit_breaker_state_machine() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        recovery_timeout_ms: 1000,
        success_threshold: 2,
        failure_window_ms: 5000,
    };

    let mut breaker = CircuitBreaker::new(config);

    // Initially closed
    assert!(matches!(breaker.state, CircuitBreakerState::Closed));
    assert!(breaker.should_allow_request());

    // Record failures to open the breaker
    breaker.record_failure();
    assert!(matches!(breaker.state, CircuitBreakerState::Closed));

    breaker.record_failure();
    assert!(matches!(breaker.state, CircuitBreakerState::Closed));

    breaker.record_failure();
    assert!(matches!(breaker.state, CircuitBreakerState::Open { .. }));
    assert!(!breaker.should_allow_request());

    // After timeout, should transition to half-open
    std::thread::sleep(std::time::Duration::from_millis(1001));
    assert!(breaker.should_allow_request());
    assert!(matches!(breaker.state, CircuitBreakerState::HalfOpen));

    // Success should close the breaker
    breaker.record_success();
    breaker.record_success();
    assert!(matches!(breaker.state, CircuitBreakerState::Closed));
}

#[test]
fn test_task_info_creation() {
    let node_id = NodeId::new();

    // Test different task info creation methods
    let agent_id = AgentId::new();
    let agent_task = TaskInfo::agent_task(agent_id, node_id.clone());
    assert_eq!(agent_task.node_type, "agent");
    assert_eq!(agent_task.task_id, node_id);

    let http_task = TaskInfo::http_task(node_id.clone());
    assert_eq!(http_task.node_type, "http_request");

    let transform_task = TaskInfo::transform_task(node_id.clone());
    assert_eq!(transform_task.node_type, "transform");

    let condition_task = TaskInfo::condition_task(node_id.clone());
    assert_eq!(condition_task.node_type, "condition");
}

#[test]
fn test_workflow_execution_stats() {
    let mut stats = WorkflowExecutionStats {
        total_nodes: 0,
        successful_nodes: 0,
        failed_nodes: 0,
        avg_execution_time_ms: 0.0,
        max_concurrent_nodes: 0,
        total_execution_time_ms: 0,
        peak_memory_usage_mb: None,
        semaphore_acquisitions: 0,
        avg_semaphore_wait_ms: 0.0,
    };

    // Test timing operations
    let start = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(10));
    stats.total_execution_time_ms = start.elapsed().as_millis() as u64;

    assert!(stats.total_execution_time_ms >= 10);

    // Test node execution tracking
    stats.total_nodes = 8;
    stats.successful_nodes = 5;
    stats.failed_nodes = 1;

    assert_eq!(stats.total_nodes, 8);
    assert_eq!(stats.successful_nodes, 5);
    assert_eq!(stats.failed_nodes, 1);
}

#[test]
fn test_node_execution_result() {
    let node_id = NodeId::new();
    let output = json!({"result": "success"});

    let result = NodeExecutionResult {
        success: true,
        output: output.clone(),
        error: None,
        metadata: std::collections::HashMap::new(),
        duration_ms: 150,
        started_at: chrono::Utc::now(),
        completed_at: Some(chrono::Utc::now()),
        retry_count: 2,
        node_id: node_id.clone(),
    };

    assert_eq!(result.node_id, node_id);
    assert_eq!(result.output, output);
    assert_eq!(result.duration_ms, 150);
    assert_eq!(result.retry_count, 2);
    assert!(result.error.is_none());

    // Test with error
    let error_result = NodeExecutionResult {
        success: false,
        output: json!(null),
        error: Some("Execution failed".to_string()),
        metadata: std::collections::HashMap::new(),
        duration_ms: 50,
        started_at: chrono::Utc::now(),
        completed_at: Some(chrono::Utc::now()),
        retry_count: 0,
        node_id: node_id.clone(),
    };

    assert!(error_result.error.is_some());
    assert_eq!(error_result.error.as_ref().unwrap(), "Execution failed");
}
