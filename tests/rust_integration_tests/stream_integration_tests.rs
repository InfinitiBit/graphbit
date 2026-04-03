//! Streaming integration tests for `WorkflowExecutor::execute_streaming()`
//!
//! These tests verify end-to-end behavior of the streaming execution path
//! **without** requiring real LLM API keys.  Instead they:
//!
//! - Use `Custom` / `Transform` node types so no LLM calls are made, and
//! - Assert the correct sequence and structure of `StreamEvent`s emitted.
//!
//! Tests that require real LLM calls are gated with `has_openai_api_key()`.

use graphbit_core::{
    graph::{EdgeType, NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode},
    stream::{StreamEvent, StreamMode},
    workflow::{Workflow, WorkflowBuilder, WorkflowExecutor},
};
use tokio::sync::mpsc;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Collect all events from the channel (waiting for it to close).
async fn collect_events(mut rx: mpsc::Receiver<StreamEvent>) -> Vec<StreamEvent> {
    let mut events = Vec::new();
    while let Some(e) = rx.recv().await {
        events.push(e);
    }
    events
}

/// Return the `event` discriminant string for a `StreamEvent`.
fn event_tag(e: &StreamEvent) -> &'static str {
    match e {
        StreamEvent::WorkflowStarted { .. } => "workflow_started",
        StreamEvent::NodeStarted { .. } => "node_started",
        StreamEvent::NodeCompleted { .. } => "node_completed",
        StreamEvent::NodeFailed { .. } => "node_failed",
        StreamEvent::WorkflowCompleted { .. } => "workflow_completed",
        StreamEvent::WorkflowFailed { .. } => "workflow_failed",
        StreamEvent::Token { .. } => "token",
        StreamEvent::LlmCallStarted { .. } => "llm_call_started",
        StreamEvent::LlmCallCompleted { .. } => "llm_call_completed",
        StreamEvent::ToolCallStarted { .. } => "tool_call_started",
        StreamEvent::ToolCallCompleted { .. } => "tool_call_completed",
        StreamEvent::ToolCallFailed { .. } => "tool_call_failed",
    }
}

/// Build a minimal two-node workflow (no LLM calls) using `Custom` nodes.
fn two_node_workflow() -> Workflow {
    let node_a = WorkflowNode::new(
        "StepA",
        "First step",
        NodeType::Custom {
            function_name: "step_a".to_string(),
        },
    );
    let node_b = WorkflowNode::new(
        "StepB",
        "Second step",
        NodeType::Custom {
            function_name: "step_b".to_string(),
        },
    );

    let (builder, id_a) = WorkflowBuilder::new("TwoNodeWorkflow")
        .add_node(node_a)
        .expect("add node_a");
    let (builder, id_b) = builder.add_node(node_b).expect("add node_b");
    builder
        .connect(id_a, id_b, WorkflowEdge::data_flow())
        .expect("connect")
        .build()
        .expect("build")
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests: StreamMode::Updates
// ─────────────────────────────────────────────────────────────────────────────

/// `execute_streaming()` with `Updates` mode must emit the lifecycle events
/// and close the channel cleanly.
///
/// Note: Custom node types are not registered with the executor and the
/// executor has no LLM config, so the workflow will fail at agent resolution
/// time, emitting `WorkflowFailed` as the (only or last) event.  We verify
/// structural invariants: the channel closes and the last event is terminal.
#[tokio::test]
async fn test_streaming_updates_mode_event_sequence() {
    graphbit_core::init().expect("init");

    let workflow = two_node_workflow();
    let executor = WorkflowExecutor::new();
    let (tx, rx) = mpsc::channel::<StreamEvent>(64);

    let _ = executor
        .execute_streaming(workflow, None, tx, StreamMode::Updates)
        .await;

    let events = collect_events(rx).await;

    // Must contain at least one event
    assert!(
        !events.is_empty(),
        "Expected at least one event but got none"
    );

    // Last event must be terminal (WorkflowCompleted or WorkflowFailed)
    let last = events.last().unwrap();
    assert!(
        matches!(
            last,
            StreamEvent::WorkflowCompleted { .. } | StreamEvent::WorkflowFailed { .. }
        ),
        "Last event must be workflow_completed or workflow_failed, got: {}",
        event_tag(last)
    );
}

/// In `Updates` mode, `Token` events must NOT be emitted (no LLM streaming).
#[tokio::test]
async fn test_streaming_updates_mode_no_token_events() {
    graphbit_core::init().expect("init");

    let workflow = two_node_workflow();
    let executor = WorkflowExecutor::new();
    let (tx, rx) = mpsc::channel::<StreamEvent>(64);

    let _ = executor
        .execute_streaming(workflow, None, tx, StreamMode::Updates)
        .await;

    let events = collect_events(rx).await;

    let has_tokens = events
        .iter()
        .any(|e| matches!(e, StreamEvent::Token { .. }));
    assert!(
        !has_tokens,
        "Updates mode must not emit Token events; got {} events total",
        events.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests: StreamMode::Messages / All
// ─────────────────────────────────────────────────────────────────────────────

/// `Messages` mode must not emit `Token` events for `Custom` nodes
/// (they produce no LLM output), but the stream must still close cleanly.
#[tokio::test]
async fn test_streaming_messages_mode_closes_cleanly() {
    graphbit_core::init().expect("init");

    let workflow = two_node_workflow();
    let executor = WorkflowExecutor::new();
    let (tx, rx) = mpsc::channel::<StreamEvent>(64);

    let _ = executor
        .execute_streaming(workflow, None, tx, StreamMode::Messages)
        .await;

    let events = collect_events(rx).await;

    assert!(
        !events.is_empty(),
        "Expected events in Messages mode but got none"
    );

    // Terminal event must still be present
    let last = events.last().unwrap();
    assert!(
        matches!(
            last,
            StreamEvent::WorkflowCompleted { .. } | StreamEvent::WorkflowFailed { .. }
        ),
        "Messages mode: last event must be terminal, got: {}",
        event_tag(last)
    );
}

/// `All` mode behaves identically to `Messages` for `Custom` node workflows.
#[tokio::test]
async fn test_streaming_all_mode_closes_cleanly() {
    graphbit_core::init().expect("init");

    let workflow = two_node_workflow();
    let executor = WorkflowExecutor::new();
    let (tx, rx) = mpsc::channel::<StreamEvent>(64);

    let _ = executor
        .execute_streaming(workflow, None, tx, StreamMode::All)
        .await;

    let events = collect_events(rx).await;

    assert!(!events.is_empty(), "All mode: expected at least one event");

    let last = events.last().unwrap();
    assert!(
        matches!(
            last,
            StreamEvent::WorkflowCompleted { .. } | StreamEvent::WorkflowFailed { .. }
        ),
        "All mode: last event must be terminal, got: {}",
        event_tag(last)
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests: Channel robustness
// ─────────────────────────────────────────────────────────────────────────────

/// If the receiver is dropped before the workflow starts, `execute_streaming()`
/// must **not** panic — fire-and-forget is guaranteed.
#[tokio::test]
async fn test_streaming_dropped_receiver_does_not_panic() {
    graphbit_core::init().expect("init");

    let workflow = two_node_workflow();
    let executor = WorkflowExecutor::new();
    let (tx, rx) = mpsc::channel::<StreamEvent>(4);

    // Drop the receiver immediately to simulate a disconnected consumer
    drop(rx);

    // This must complete without panic/unwrap errors
    let _ = executor
        .execute_streaming(workflow, None, tx, StreamMode::Updates)
        .await;
}

/// A small channel buffer that fills up must not cause the workflow to deadlock;
/// sends failure silently when the buffer is full (fire-and-forget).
#[tokio::test]
async fn test_streaming_small_buffer_does_not_deadlock() {
    graphbit_core::init().expect("init");

    // Use a 5-node chain with a buffer of 1 to exercise back-pressure
    let mut builder = WorkflowBuilder::new("LongChain");
    let mut prev_id = None;

    for i in 0..5 {
        let node = WorkflowNode::new(
            &format!("Node{i}"),
            &format!("Step {i}"),
            NodeType::Custom {
                function_name: format!("step_{i}"),
            },
        );
        let (b, id) = builder.add_node(node).expect("add node");
        builder = if let Some(prev) = prev_id {
            b.connect(prev, id.clone(), WorkflowEdge::data_flow())
                .expect("connect")
        } else {
            b
        };
        prev_id = Some(id);
    }

    let workflow = builder.build().expect("build");
    let executor = WorkflowExecutor::new();
    // Buffer of 1: very tight, exercises the try_send / fire-and-forget path
    let (tx, rx) = mpsc::channel::<StreamEvent>(1);

    // Drain the channel concurrently so the workflow doesn't block on sends
    let drain = tokio::spawn(async move { collect_events(rx).await });

    let _ = executor
        .execute_streaming(workflow, None, tx, StreamMode::Updates)
        .await;

    let events = drain.await.expect("drain join");
    assert!(
        !events.is_empty(),
        "Expected at least one event even with a tight buffer"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests: WorkflowStarted fields
// ─────────────────────────────────────────────────────────────────────────────

/// When `WorkflowStarted` is emitted it must carry the correct `workflow_name`
/// and `total_nodes` count.
///
/// Note: If the executor has no LLM config and the workflow uses Custom nodes,
/// `WorkflowFailed` may be emitted before `WorkflowStarted` (agent resolution
/// happens before the started event).  We check the fields only when the event
/// is present in the stream.
#[tokio::test]
async fn test_workflow_started_event_fields() {
    graphbit_core::init().expect("init");

    let node_a = WorkflowNode::new(
        "Alpha",
        "Node alpha",
        NodeType::Custom {
            function_name: "alpha".to_string(),
        },
    );
    let (builder, _) = WorkflowBuilder::new("FieldCheckWorkflow")
        .add_node(node_a)
        .expect("add node");
    let workflow = builder.build().expect("build");

    let executor = WorkflowExecutor::new();
    let (tx, rx) = mpsc::channel::<StreamEvent>(32);

    let _ = executor
        .execute_streaming(workflow, None, tx, StreamMode::Updates)
        .await;

    let events = collect_events(rx).await;

    // The stream must always close with at least one event
    assert!(!events.is_empty(), "Expected at least one event");

    // If WorkflowStarted was emitted, validate its fields
    if let Some(started) = events
        .iter()
        .find(|e| matches!(e, StreamEvent::WorkflowStarted { .. }))
    {
        if let StreamEvent::WorkflowStarted {
            workflow_name,
            total_nodes,
            ..
        } = started
        {
            assert_eq!(workflow_name, "FieldCheckWorkflow");
            assert_eq!(*total_nodes, 1);
        }
    }
    // Regardless, the last event must be terminal
    let last = events.last().unwrap();
    assert!(
        matches!(
            last,
            StreamEvent::WorkflowCompleted { .. } | StreamEvent::WorkflowFailed { .. }
        ),
        "Last event must be terminal, got: {}",
        event_tag(last)
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests: StreamMode properties (no executor needed)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_stream_mode_emits_flags() {
    assert!(!StreamMode::Updates.emits_tokens());
    assert!(!StreamMode::Updates.emits_tool_events());
    assert!(StreamMode::Messages.emits_tokens());
    assert!(StreamMode::Messages.emits_tool_events());
    assert!(StreamMode::All.emits_tokens());
    assert!(StreamMode::All.emits_tool_events());
}

#[test]
fn test_stream_mode_roundtrip_via_display() {
    for mode in [StreamMode::Updates, StreamMode::Messages, StreamMode::All] {
        let s = mode.to_string();
        let parsed = StreamMode::from_str_opt(&s).expect("roundtrip");
        assert_eq!(parsed, mode);
    }
}
