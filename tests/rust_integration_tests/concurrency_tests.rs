//! Concurrency and Performance Integration Tests
//!
//! Tests for concurrency management, performance monitoring,
//! circuit breakers, retry logic, and stress testing.

use graphbit_core::types::*;
use graphbit_core::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::test]
async fn test_retry_config_creation_and_delay_calculation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let retry_config = RetryConfig::new(5)
        .with_exponential_backoff(100, 2.0, 5000)
        .with_jitter(0.1)
        .with_retryable_errors(vec![
            RetryableErrorType::NetworkError,
            RetryableErrorType::TimeoutError,
            RetryableErrorType::RateLimitError,
        ]);

    assert_eq!(retry_config.max_attempts, 5);
    assert_eq!(retry_config.initial_delay_ms, 100);
    assert_eq!(retry_config.backoff_multiplier, 2.0);
    assert_eq!(retry_config.max_delay_ms, 5000);
    assert_eq!(retry_config.jitter_factor, 0.1);

    // Test delay calculation
    let delay1 = retry_config.calculate_delay(1);
    let delay2 = retry_config.calculate_delay(2);
    let delay3 = retry_config.calculate_delay(3);

    // Delays should increase exponentially
    assert!(delay2 > delay1);
    assert!(delay3 > delay2);

    // But should be capped at max_delay_ms
    let large_delay = retry_config.calculate_delay(10);
    assert!(large_delay <= retry_config.max_delay_ms);
}

#[tokio::test]
async fn test_retry_error_type_classification() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test different error types
    let network_error = errors::GraphBitError::Network {
        message: "Connection failed".to_string(),
    };
    assert_eq!(
        RetryableErrorType::from_error(&network_error),
        RetryableErrorType::NetworkError
    );

    let llm_error = errors::GraphBitError::Llm {
        message: "API timeout".to_string(),
    };
    assert_eq!(
        RetryableErrorType::from_error(&llm_error),
        RetryableErrorType::TimeoutError
    );

    let rate_limit_error = errors::GraphBitError::RateLimit {
        provider: "openai".to_string(),
        retry_after_seconds: 60,
    };
    assert_eq!(
        RetryableErrorType::from_error(&rate_limit_error),
        RetryableErrorType::RateLimitError
    );
}

#[tokio::test]
async fn test_circuit_breaker_state_transitions() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        recovery_timeout_ms: 1000,
        success_threshold: 2,
        failure_window_ms: 5000,
    };

    let mut circuit_breaker = CircuitBreaker::new(config);

    // Initially closed
    assert!(matches!(circuit_breaker.state, CircuitBreakerState::Closed));
    assert!(circuit_breaker.should_allow_request());

    // Record failures
    circuit_breaker.record_failure();
    assert!(circuit_breaker.should_allow_request()); // Still closed

    circuit_breaker.record_failure();
    assert!(circuit_breaker.should_allow_request()); // Still closed

    circuit_breaker.record_failure();
    assert!(!circuit_breaker.should_allow_request()); // Now open

    // Verify it's open
    assert!(matches!(
        circuit_breaker.state,
        CircuitBreakerState::Open { .. }
    ));

    // Fast forward time (simulate wait)
    sleep(Duration::from_millis(1100)).await;

    // Should transition to half-open
    assert!(circuit_breaker.should_allow_request());
    assert!(matches!(
        circuit_breaker.state,
        CircuitBreakerState::HalfOpen
    ));

    // Record successes to close the circuit
    circuit_breaker.record_success();
    circuit_breaker.record_success();

    // Should be closed again
    assert!(matches!(circuit_breaker.state, CircuitBreakerState::Closed));
    assert!(circuit_breaker.should_allow_request());
}

#[tokio::test]
async fn test_concurrency_config_variants() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test high throughput config
    let high_throughput = ConcurrencyConfig::high_throughput();
    assert!(high_throughput.global_max_concurrency >= 100);
    assert!(high_throughput.get_node_type_limit("agent") >= 50);

    // Test low latency config
    let low_latency = ConcurrencyConfig::low_latency();
    assert!(low_latency.global_max_concurrency >= 20);
    assert!(low_latency.get_node_type_limit("agent") >= 10);

    // Test memory optimized config
    let memory_optimized = ConcurrencyConfig::memory_optimized();
    assert!(memory_optimized.global_max_concurrency <= 50);
    assert!(memory_optimized.get_node_type_limit("agent") <= 20);
}

#[tokio::test]
async fn test_concurrency_manager_permit_acquisition() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = ConcurrencyConfig::low_latency();
    let manager = ConcurrencyManager::new(config);

    // Test agent task permit acquisition
    let agent_task = TaskInfo::agent_task(AgentId::new(), NodeId::new());
    let permits_result = manager.acquire_permits(&agent_task).await;
    assert!(permits_result.is_ok());

    // Test HTTP task permit acquisition
    let http_task = TaskInfo::http_task(NodeId::new());
    let permits_result = manager.acquire_permits(&http_task).await;
    assert!(permits_result.is_ok());

    // Test transform task permit acquisition
    let transform_task = TaskInfo::transform_task(NodeId::new());
    let permits_result = manager.acquire_permits(&transform_task).await;
    assert!(permits_result.is_ok());
}

#[tokio::test]
async fn test_concurrency_stats_tracking() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = ConcurrencyConfig::default();
    let manager = ConcurrencyManager::new(config);

    let initial_stats = manager.get_stats().await;
    assert_eq!(initial_stats.total_permit_acquisitions, 0);
    assert_eq!(initial_stats.current_active_tasks, 0);

    // Acquire some permits
    let task1 = TaskInfo::agent_task(AgentId::new(), NodeId::new());
    let _permits1 = manager.acquire_permits(&task1).await.unwrap();

    let task2 = TaskInfo::http_task(NodeId::new());
    let _permits2 = manager.acquire_permits(&task2).await.unwrap();

    let stats_after = manager.get_stats().await;
    assert_eq!(stats_after.total_permit_acquisitions, 2);
    assert!(stats_after.current_active_tasks >= 2);

    // Test available permits
    let available = manager.get_available_permits().await;
    assert!(available.contains_key("agent"));
    assert!(available.contains_key("http_request"));
}

#[tokio::test]
async fn test_workflow_execution_stats() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let stats = WorkflowExecutionStats {
        total_nodes: 10,
        successful_nodes: 8,
        failed_nodes: 2,
        avg_execution_time_ms: 150.5,
        max_concurrent_nodes: 5,
        total_execution_time_ms: 3000,
        peak_memory_usage_mb: Some(256.8),
        semaphore_acquisitions: 25,
        avg_semaphore_wait_ms: 12.3,
    };

    assert_eq!(stats.total_nodes, 10);
    assert_eq!(stats.successful_nodes, 8);
    assert_eq!(stats.failed_nodes, 2);
    assert!(stats.avg_execution_time_ms > 0.0);
    assert!(stats.total_execution_time_ms > 0);

    // Test success rate calculation
    let success_rate = stats.successful_nodes as f64 / stats.total_nodes as f64;
    assert_eq!(success_rate, 0.8);
}

#[tokio::test]
async fn test_node_execution_result_tracking() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let start_time = chrono::Utc::now();

    let success_result = NodeExecutionResult::success(serde_json::json!({"output": "success"}))
        .with_metadata("node_type".to_string(), serde_json::json!("agent"))
        .with_duration(1500)
        .with_retry_count(0)
        .mark_completed();

    assert!(success_result.success);
    assert_eq!(
        success_result.output,
        serde_json::json!({"output": "success"})
    );
    assert_eq!(success_result.duration_ms, 1500);
    assert_eq!(success_result.retry_count, 0);
    assert!(success_result.completed_at.is_some());
    assert!(success_result.started_at >= start_time);

    let failure_result = NodeExecutionResult::failure("Processing failed".to_string())
        .with_metadata("error_type".to_string(), serde_json::json!("timeout"))
        .with_duration(5000)
        .with_retry_count(3);

    assert!(!failure_result.success);
    assert_eq!(failure_result.error, Some("Processing failed".to_string()));
    assert_eq!(failure_result.duration_ms, 5000);
    assert_eq!(failure_result.retry_count, 3);
}

#[tokio::test]
async fn test_workflow_context_variable_management() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let mut context = WorkflowContext::new(WorkflowId::new());

    // Test variable setting and getting
    context.set_variable("input_text".to_string(), serde_json::json!("Hello, world!"));
    context.set_variable("processing_mode".to_string(), serde_json::json!("async"));
    context.set_variable(
        "user_config".to_string(),
        serde_json::json!({
            "language": "en",
            "timeout": 30
        }),
    );

    assert_eq!(
        context.get_variable("input_text"),
        Some(&serde_json::json!("Hello, world!"))
    );
    assert_eq!(
        context.get_variable("processing_mode"),
        Some(&serde_json::json!("async"))
    );
    assert_eq!(context.get_variable("nonexistent"), None);

    // Test metadata
    context.set_metadata(
        "start_time".to_string(),
        serde_json::json!(chrono::Utc::now().to_rfc3339()),
    );
    context.set_metadata("version".to_string(), serde_json::json!("1.0.0"));

    assert!(context.metadata.contains_key("start_time"));
    assert_eq!(
        context.metadata.get("version"),
        Some(&serde_json::json!("1.0.0"))
    );

    // Test state transitions
    context.state = WorkflowState::Running {
        current_node: NodeId::new(),
    };
    assert!(context.state.is_running());
    assert!(!context.state.is_terminal());

    context.complete();
    assert!(context.state.is_terminal());
    assert!(!context.state.is_running());
}

#[tokio::test]
async fn test_agent_message_creation_and_metadata() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let sender = AgentId::new();
    let recipient = AgentId::new();

    let message = AgentMessage::new(
        sender.clone(),
        Some(recipient.clone()),
        MessageContent::Text("Test message".to_string()),
    )
    .with_metadata("priority".to_string(), serde_json::json!("high"))
    .with_metadata("message_type".to_string(), serde_json::json!("request"))
    .with_metadata(
        "timestamp".to_string(),
        serde_json::json!(chrono::Utc::now().to_rfc3339()),
    );

    assert_eq!(message.sender, sender);
    assert_eq!(message.recipient, Some(recipient));
    assert_eq!(message.metadata.len(), 3);
    assert_eq!(
        message.metadata.get("priority"),
        Some(&serde_json::json!("high"))
    );

    // Test different message content types
    let data_message = AgentMessage::new(
        sender.clone(),
        None,
        MessageContent::Data(serde_json::json!({
            "data": [1, 2, 3, 4, 5],
            "metadata": {"type": "numbers"}
        })),
    );

    match &data_message.content {
        MessageContent::Data(data) => {
            assert_eq!(data["data"], serde_json::json!([1, 2, 3, 4, 5]));
        }
        _ => panic!("Expected data content"),
    }

    // Test tool call message
    let tool_message = AgentMessage::new(
        sender,
        None,
        MessageContent::ToolCall {
            tool_name: "calculate".to_string(),
            parameters: serde_json::json!({"expression": "2 + 2"}),
        },
    );

    match &tool_message.content {
        MessageContent::ToolCall {
            tool_name,
            parameters,
        } => {
            assert_eq!(tool_name, "calculate");
            assert_eq!(parameters["expression"], "2 + 2");
        }
        _ => panic!("Expected tool call content"),
    }
}

#[tokio::test]
async fn test_agent_capabilities_management() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::ToolExecution,
        AgentCapability::DecisionMaking,
        AgentCapability::Custom("machine_learning".to_string()),
        AgentCapability::Custom("web_scraping".to_string()),
    ];

    let llm_config = llm::LlmConfig::ollama("llama3.2");
    let agent_config = agents::AgentConfig::new(
        "Multi-Capability Agent",
        "Agent with multiple capabilities",
        llm_config,
    )
    .with_capabilities(capabilities.clone());

    assert_eq!(agent_config.capabilities.len(), 6);
    assert!(agent_config
        .capabilities
        .contains(&AgentCapability::TextProcessing));
    assert!(agent_config
        .capabilities
        .contains(&AgentCapability::Custom("machine_learning".to_string())));

    // Test capability checking
    let agent_trait: &dyn agents::AgentTrait = &agents::Agent::new(agent_config.clone())
        .await
        .unwrap_or_else(|_| {
            panic!("Failed to create agent for capability testing");
        });

    assert!(agent_trait.has_capability(&AgentCapability::TextProcessing));
    assert!(agent_trait.has_capability(&AgentCapability::Custom("web_scraping".to_string())));
    assert!(!agent_trait.has_capability(&AgentCapability::Custom("unknown_capability".to_string())));
}

#[tokio::test]
async fn test_id_generation_and_uniqueness() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test AgentId uniqueness
    let mut agent_ids = std::collections::HashSet::new();
    for _ in 0..1000 {
        let id = AgentId::new();
        assert!(agent_ids.insert(id), "Agent ID should be unique");
    }

    // Test WorkflowId uniqueness
    let mut workflow_ids = std::collections::HashSet::new();
    for _ in 0..1000 {
        let id = WorkflowId::new();
        assert!(workflow_ids.insert(id), "Workflow ID should be unique");
    }

    // Test NodeId uniqueness
    let mut node_ids = std::collections::HashSet::new();
    for _ in 0..1000 {
        let id = NodeId::new();
        assert!(node_ids.insert(id), "Node ID should be unique");
    }
}

#[tokio::test]
async fn test_id_string_conversion() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test AgentId string conversion
    let agent_id = AgentId::new();
    let agent_id_str = agent_id.to_string();
    let parsed_agent_id = AgentId::from_string(&agent_id_str).expect("Failed to parse agent ID");
    assert_eq!(agent_id, parsed_agent_id);

    // Test deterministic ID generation from string
    let deterministic_id1 =
        AgentId::from_string("test-agent-1").expect("Failed to create deterministic ID");
    let deterministic_id2 =
        AgentId::from_string("test-agent-1").expect("Failed to create deterministic ID");
    assert_eq!(deterministic_id1, deterministic_id2);

    // Test different strings produce different IDs
    let diff_id1 = AgentId::from_string("agent-a").expect("Failed to create ID");
    let diff_id2 = AgentId::from_string("agent-b").expect("Failed to create ID");
    assert_ne!(diff_id1, diff_id2);
}

#[tokio::test]
async fn test_stress_concurrent_operations() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = ConcurrencyConfig::high_throughput();
    let manager = Arc::new(ConcurrencyManager::new(config));

    let start_time = Instant::now();
    let num_tasks = 100;
    let num_concurrent = 10;

    // Create concurrent tasks
    let mut handles = Vec::new();

    for batch in 0..num_concurrent {
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            let mut local_results = Vec::new();

            for i in 0..(num_tasks / num_concurrent) {
                let task_id = NodeId::new();
                let task_info = TaskInfo::agent_task(AgentId::new(), task_id);

                let permit_result = manager_clone.acquire_permits(&task_info).await;
                match permit_result {
                    Ok(permits) => {
                        // Simulate some work
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        local_results.push((batch, i, true));
                        drop(permits); // Explicitly drop permits
                    }
                    Err(_) => {
                        local_results.push((batch, i, false));
                    }
                }
            }
            local_results
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut all_results = Vec::new();
    for handle in handles {
        let results = handle.await.expect("Task should complete");
        all_results.extend(results);
    }

    let duration = start_time.elapsed();

    // Verify results
    assert_eq!(all_results.len(), num_tasks);
    let successful_tasks = all_results
        .iter()
        .filter(|(_, _, success)| *success)
        .count();
    assert!(successful_tasks > 0, "At least some tasks should succeed");

    // Performance check - should complete within reasonable time
    assert!(
        duration.as_secs() < 10,
        "Stress test should complete within 10 seconds"
    );

    println!(
        "Stress test completed: {successful_tasks}/{num_tasks} tasks successful in {duration:?}"
    );
}

#[tokio::test]
async fn test_memory_usage_tracking() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

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

    // Simulate memory tracking
    stats.peak_memory_usage_mb = Some(128.5);
    assert_eq!(stats.peak_memory_usage_mb, Some(128.5));

    // Test memory optimization config
    let memory_config = ConcurrencyConfig::memory_optimized();
    assert!(
        memory_config.global_max_concurrency <= 50,
        "Memory optimized should have lower concurrency limits"
    );

    // Create memory-optimized manager
    let manager = ConcurrencyManager::new(memory_config);
    let available_permits = manager.get_available_permits().await;

    // Memory optimized should have conservative permit limits
    for (node_type, permits) in available_permits {
        assert!(
            permits <= 20,
            "Memory optimized permits for {node_type} should be <= 20, got {permits}"
        );
    }
}

// Advanced concurrency tests
#[tokio::test]
async fn test_concurrent_permit_contention() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Create a very limited concurrency config to force contention
    let mut limited_config = ConcurrencyConfig {
        global_max_concurrency: 2, // Very limited
        ..Default::default()
    };
    limited_config
        .node_type_limits
        .insert("agent".to_string(), 1);

    let manager = Arc::new(ConcurrencyManager::new(limited_config));
    let num_contenders = 10;
    let mut handles = Vec::new();

    let start_time = Instant::now();

    // Create many contending tasks
    for i in 0..num_contenders {
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            let task_info = TaskInfo::agent_task(AgentId::new(), NodeId::new());
            let start = Instant::now();

            let permit_result = manager_clone.acquire_permits(&task_info).await;
            let wait_time = start.elapsed();

            match permit_result {
                Ok(_permits) => {
                    // Hold the permit for a bit to create contention
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    (i, true, wait_time)
                }
                Err(_) => (i, false, wait_time),
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        results.push(result);
    }

    let total_duration = start_time.elapsed();

    // Verify contention behavior
    let successful_tasks = results.iter().filter(|(_, success, _)| *success).count();
    assert!(
        successful_tasks > 0,
        "At least some tasks should succeed despite contention"
    );

    // Some tasks should have had to wait due to contention
    let tasks_with_wait = results
        .iter()
        .filter(|(_, _, wait_time)| wait_time.as_millis() > 50)
        .count();
    assert!(
        tasks_with_wait > 0,
        "Some tasks should have experienced wait time due to contention"
    );

    println!(
        "Contention test: {successful_tasks}/{num_contenders} successful, {tasks_with_wait} with wait time, total duration: {total_duration:?}"
    );
}

#[tokio::test]
async fn test_circuit_breaker_recovery_behavior() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        recovery_timeout_ms: 500, // Short recovery time for testing
        success_threshold: 1,
        failure_window_ms: 2000,
    };

    let mut circuit_breaker = CircuitBreaker::new(config);

    // Trigger circuit breaker to open
    circuit_breaker.record_failure();
    circuit_breaker.record_failure();
    assert!(!circuit_breaker.should_allow_request());

    // Wait for recovery timeout
    tokio::time::sleep(Duration::from_millis(600)).await;

    // Should be half-open now
    assert!(circuit_breaker.should_allow_request());
    assert!(matches!(
        circuit_breaker.state,
        CircuitBreakerState::HalfOpen
    ));

    // Test failure during half-open (should go back to open)
    circuit_breaker.record_failure();
    assert!(!circuit_breaker.should_allow_request());
    assert!(matches!(
        circuit_breaker.state,
        CircuitBreakerState::Open { .. }
    ));

    // Wait again and test successful recovery
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert!(circuit_breaker.should_allow_request());

    // Record success to close the circuit
    circuit_breaker.record_success();
    assert!(matches!(circuit_breaker.state, CircuitBreakerState::Closed));
}

#[tokio::test]
async fn test_retry_logic_with_different_error_types() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let retry_config = RetryConfig::new(3)
        .with_exponential_backoff(100, 2.0, 1000)
        .with_retryable_errors(vec![
            RetryableErrorType::NetworkError,
            RetryableErrorType::TimeoutError,
        ]);

    // Test retryable errors
    let network_error = errors::GraphBitError::Network {
        message: "Connection failed".to_string(),
    };
    assert!(retry_config.should_retry(&network_error, 1));
    assert!(retry_config.should_retry(&network_error, 2));
    assert!(!retry_config.should_retry(&network_error, 3)); // Exceeded max attempts

    // Test non-retryable errors
    let validation_error = errors::GraphBitError::Validation {
        field: "input".to_string(),
        message: "Invalid input".to_string(),
    };
    assert!(!retry_config.should_retry(&validation_error, 1));

    // Test delay calculation with jitter
    let delay1 = retry_config.calculate_delay(1);
    let delay2 = retry_config.calculate_delay(1);

    // With jitter, delays might be slightly different
    assert!((90..=110).contains(&delay1)); // 100ms ± 10% jitter
    assert!((90..=110).contains(&delay2));
}

#[tokio::test]
async fn test_workflow_executor_concurrent_execution() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let executor = workflow::WorkflowExecutor::new_high_throughput();

    // Test concurrent task execution capability
    let tasks = vec![
        "task1".to_string(),
        "task2".to_string(),
        "task3".to_string(),
        "task4".to_string(),
        "task5".to_string(),
    ];

    let task_fn = |task: String| {
        Box::pin(async move {
            // Simulate async work
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(format!("Processed {task}"))
        }) as futures::future::BoxFuture<'static, Result<String, errors::GraphBitError>>
    };

    let start_time = Instant::now();
    let results = executor.execute_concurrent_tasks(tasks, task_fn).await;
    let duration = start_time.elapsed();

    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 5);

    // Should complete faster than sequential execution
    assert!(
        duration.as_millis() < 600,
        "Concurrent execution should be faster than sequential"
    );

    // Check all results
    for result in results {
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.starts_with("Processed task"));
    }
}

#[tokio::test]
async fn test_deadlock_prevention() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Create a scenario that could potentially deadlock
    let config = ConcurrencyConfig::memory_optimized(); // Limited resources
    let manager = Arc::new(ConcurrencyManager::new(config));

    let mut handles = Vec::new();
    let num_tasks = 20;

    // Create many tasks that could potentially create resource contention
    for i in 0..num_tasks {
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            let task_info = if i % 2 == 0 {
                TaskInfo::agent_task(AgentId::new(), NodeId::new())
            } else {
                TaskInfo::http_task(NodeId::new())
            };

            // Add timeout to prevent hanging
            let permit_future = manager_clone.acquire_permits(&task_info);
            let timeout_duration = Duration::from_secs(5);

            match tokio::time::timeout(timeout_duration, permit_future).await {
                Ok(Ok(permits)) => {
                    // Simulate work
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    drop(permits);
                    true
                }
                Ok(Err(_)) => false,
                Err(_) => {
                    println!("Task {i} timed out - potential deadlock detected");
                    false
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks with overall timeout
    let overall_timeout = Duration::from_secs(10);
    let start = Instant::now();

    let mut completed_tasks = 0;
    for handle in handles {
        match tokio::time::timeout(Duration::from_secs(1), handle).await {
            Ok(Ok(success)) => {
                if success {
                    completed_tasks += 1;
                }
            }
            Ok(Err(_)) => {
                println!("Task panicked");
            }
            Err(_) => {
                println!("Task timed out");
            }
        }
    }

    let total_duration = start.elapsed();

    // Should complete within reasonable time (no deadlock)
    assert!(
        total_duration < overall_timeout,
        "Should not deadlock - completed in {total_duration:?}"
    );
    assert!(
        completed_tasks > 0,
        "At least some tasks should complete successfully"
    );

    println!(
        "Deadlock prevention test: {completed_tasks}/{num_tasks} tasks completed in {total_duration:?}"
    );
}
