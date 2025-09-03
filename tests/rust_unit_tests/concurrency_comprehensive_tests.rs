//! Comprehensive concurrency unit tests
//!
//! Tests for thread safety, concurrent access patterns,
//! synchronization primitives, and race condition prevention.

use graphbit_core::{
    errors::GraphBitError,
    graph::{NodeType, WorkflowGraph, WorkflowNode},
    types::{
        CircuitBreaker, CircuitBreakerConfig, ConcurrencyStats, NodeId, WorkflowContext, WorkflowId,
    },
};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use tokio::sync::{RwLock as TokioRwLock, Semaphore};

#[test]
fn test_atomic_operations_and_counters() {
    let counter = Arc::new(AtomicU32::new(0));
    let total_operations = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    // Spawn multiple threads to increment counters
    for _ in 0..10 {
        let counter = counter.clone();
        let total_operations = total_operations.clone();

        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
                total_operations.fetch_add(1, Ordering::Relaxed);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify final counts
    assert_eq!(counter.load(Ordering::SeqCst), 10_000);
    assert_eq!(total_operations.load(Ordering::Relaxed), 10_000);
}

#[test]
fn test_concurrent_graph_access() {
    let graph = Arc::new(RwLock::new(WorkflowGraph::new()));
    let mut handles = vec![];

    // Add initial nodes
    {
        let mut g = graph.write().unwrap();
        for i in 0..5 {
            let node = WorkflowNode::new(
                format!("Node{i}"),
                format!("Description {i}"),
                NodeType::Split,
            );
            g.add_node(node).unwrap();
        }
    }

    // Spawn readers
    for i in 0..5 {
        let graph = graph.clone();
        let handle = thread::spawn(move || {
            let g = graph.read().unwrap();
            let count = g.node_count();
            assert!(count >= 5);

            // Verify we can read node information
            let nodes = g.get_nodes();
            assert!(!nodes.is_empty());

            // Simulate some processing time
            thread::sleep(Duration::from_millis(10));

            format!("Reader {i} processed {count} nodes")
        });
        handles.push(handle);
    }

    // Spawn a writer (should wait for readers to finish)
    let graph_writer = graph.clone();
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Let readers start first

        let mut g = graph_writer.write().unwrap();
        let new_node = WorkflowNode::new("NewNode", "Added by writer", NodeType::Join);
        g.add_node(new_node).unwrap();

        "Writer completed".to_string()
    });
    handles.push(writer_handle);

    // Wait for all operations to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert_eq!(results.len(), 6); // 5 readers + 1 writer

    // Verify final state
    let final_count = graph.read().unwrap().node_count();
    assert_eq!(final_count, 6); // 5 initial + 1 added by writer
}

#[test]
fn test_concurrent_workflow_context_access() {
    let workflow_id = WorkflowId::new();
    let context = Arc::new(Mutex::new(WorkflowContext::new(workflow_id)));
    let mut handles = vec![];

    // Spawn multiple threads to update context
    for i in 0..10 {
        let context = context.clone();
        let handle = thread::spawn(move || {
            let mut ctx = context.lock().unwrap();

            // Add metadata
            ctx.set_metadata(
                format!("key_{i}"),
                serde_json::json!(format!("value_{}", i)),
            );

            // Add node output
            let node_id = NodeId::new();
            ctx.set_node_output(&node_id, serde_json::json!({"result": i, "thread": i}));

            i
        });
        handles.push(handle);
    }

    // Wait for all updates
    let thread_ids: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify all updates were applied
    let ctx = context.lock().unwrap();

    // Check metadata
    for i in 0..10 {
        let key = format!("key_{i}");
        let value = ctx.metadata.get(&key).unwrap();
        assert_eq!(value, &serde_json::json!(format!("value_{}", i)));
    }

    assert_eq!(thread_ids.len(), 10);
}

#[tokio::test]
async fn test_async_semaphore_concurrency_control() {
    let semaphore = Arc::new(Semaphore::new(3)); // Allow max 3 concurrent operations
    let active_count = Arc::new(AtomicU32::new(0));
    let max_concurrent = Arc::new(AtomicU32::new(0));

    let mut handles = vec![];

    // Spawn 10 tasks, but only 3 should run concurrently
    for i in 0..10 {
        let semaphore = semaphore.clone();
        let active_count = active_count.clone();
        let max_concurrent = max_concurrent.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();

            // Track concurrent operations
            let current_active = active_count.fetch_add(1, Ordering::SeqCst) + 1;

            // Update max concurrent if needed
            let mut max = max_concurrent.load(Ordering::SeqCst);
            while current_active > max {
                match max_concurrent.compare_exchange_weak(
                    max,
                    current_active,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(new_max) => max = new_max,
                }
            }

            // Simulate work
            tokio::time::sleep(Duration::from_millis(100)).await;

            active_count.fetch_sub(1, Ordering::SeqCst);
            i
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    assert_eq!(results.len(), 10);
    assert_eq!(max_concurrent.load(Ordering::SeqCst), 3); // Should never exceed semaphore limit
    assert_eq!(active_count.load(Ordering::SeqCst), 0); // All should be done
}

#[tokio::test]
async fn test_async_rwlock_concurrent_access() {
    let data = Arc::new(TokioRwLock::new(vec![0u32; 100]));
    let mut handles = vec![];

    // Spawn readers
    for i in 0..5 {
        let data = data.clone();
        let handle = tokio::spawn(async move {
            let guard = data.read().await;
            let sum: u32 = guard.iter().sum();

            // Simulate processing time
            tokio::time::sleep(Duration::from_millis(50)).await;

            (i, sum)
        });
        handles.push(handle);
    }

    // Spawn writers
    for i in 0..3 {
        let data = data.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(25)).await; // Let some readers start

            let mut guard = data.write().await;
            for j in 0..guard.len() {
                guard[j] += 1;
            }

            (i + 1000, 0u32) // Return same type as readers
        });
        handles.push(handle);
    }

    // Wait for all operations
    let results = futures::future::join_all(handles).await;

    // Verify all operations completed successfully
    assert_eq!(results.len(), 8); // 5 readers + 3 writers

    // Check final state
    let final_data = data.read().await;
    let final_sum: u32 = final_data.iter().sum();
    assert_eq!(final_sum, 300); // 100 elements * 3 increments each
}

#[test]
fn test_circuit_breaker_thread_safety() {
    let config = CircuitBreakerConfig {
        failure_threshold: 5,
        recovery_timeout_ms: 100,
        success_threshold: 2,
        failure_window_ms: 1000,
    };

    let breaker = Arc::new(Mutex::new(CircuitBreaker::new(config)));
    let mut handles = vec![];

    // Spawn threads to trigger failures
    for i in 0..10 {
        let breaker = breaker.clone();
        let handle = thread::spawn(move || {
            let mut b = breaker.lock().unwrap();

            if i < 5 {
                // First 5 threads record failures
                b.record_failure();
                "failure"
            } else {
                // Later threads try to record success
                if b.should_allow_request() {
                    b.record_success();
                    "success"
                } else {
                    "blocked"
                }
            }
        });
        handles.push(handle);
    }

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Should have some failures and some blocked/success results
    let failures = results.iter().filter(|&&r| r == "failure").count();
    let others = results.iter().filter(|&&r| r != "failure").count();

    assert_eq!(failures, 5);
    assert_eq!(others, 5);
}

#[test]
fn test_concurrency_stats_thread_safety() {
    let stats = Arc::new(Mutex::new(ConcurrencyStats::default()));
    let mut handles = vec![];

    // Spawn threads to update stats
    for i in 0..100 {
        let stats = stats.clone();
        let handle = thread::spawn(move || {
            let mut s = stats.lock().unwrap();

            s.total_permit_acquisitions += 1;
            s.total_wait_time_ms += i as u64;
            s.current_active_tasks += 1;

            if i % 10 == 0 {
                s.calculate_avg_wait_time();
            }

            // Simulate task completion
            thread::sleep(Duration::from_millis(1));
            s.current_active_tasks -= 1;
        });
        handles.push(handle);
    }

    // Wait for all updates
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify final stats
    let final_stats = stats.lock().unwrap();
    assert_eq!(final_stats.total_permit_acquisitions, 100);
    assert_eq!(final_stats.total_wait_time_ms, (0..100).sum::<u64>());
    assert_eq!(final_stats.current_active_tasks, 0); // All tasks completed
    assert!(final_stats.avg_wait_time_ms > 0.0);
}

#[tokio::test]
async fn test_concurrent_error_handling() {
    let error_count = Arc::new(AtomicU32::new(0));
    let success_count = Arc::new(AtomicU32::new(0));

    let mut handles = vec![];

    // Spawn tasks that may succeed or fail
    for i in 0..20 {
        let error_count = error_count.clone();
        let success_count = success_count.clone();

        let handle = tokio::spawn(async move {
            // Simulate random failures
            if i % 3 == 0 {
                error_count.fetch_add(1, Ordering::SeqCst);
                Err(GraphBitError::Network {
                    message: format!("Simulated error {i}"),
                })
            } else {
                success_count.fetch_add(1, Ordering::SeqCst);
                Ok(format!("Success {i}"))
            }
        });

        handles.push(handle);
    }

    // Collect results
    let results = futures::future::join_all(handles).await;

    let errors: Vec<_> = results
        .iter()
        .filter_map(|r| r.as_ref().ok().and_then(|inner| inner.as_ref().err()))
        .collect();

    let successes: Vec<_> = results
        .iter()
        .filter_map(|r| r.as_ref().ok().and_then(|inner| inner.as_ref().ok()))
        .collect();

    // Verify counts match atomic counters
    assert_eq!(errors.len() as u32, error_count.load(Ordering::SeqCst));
    assert_eq!(successes.len() as u32, success_count.load(Ordering::SeqCst));
    assert_eq!(errors.len() + successes.len(), 20);
}

#[test]
fn test_deadlock_prevention_with_ordered_locking() {
    let resource1 = Arc::new(Mutex::new(1u32));
    let resource2 = Arc::new(Mutex::new(2u32));

    let mut handles = vec![];

    // Thread 1: Lock resource1 then resource2
    let r1 = resource1.clone();
    let r2 = resource2.clone();
    let handle1 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(10)); // Give other thread a chance
        let _guard2 = r2.lock().unwrap();
        "Thread 1 completed"
    });
    handles.push(handle1);

    // Thread 2: Also lock resource1 then resource2 (same order to prevent deadlock)
    let r1 = resource1.clone();
    let r2 = resource2.clone();
    let handle2 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _guard2 = r2.lock().unwrap();
        "Thread 2 completed"
    });
    handles.push(handle2);

    // Both threads should complete without deadlock
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert_eq!(results.len(), 2);
    assert!(results.contains(&"Thread 1 completed"));
    assert!(results.contains(&"Thread 2 completed"));
}
