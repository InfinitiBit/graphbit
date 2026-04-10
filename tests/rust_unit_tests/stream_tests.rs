use graphbit_core::errors::GraphBitError;
use graphbit_core::stream::{
    StreamEvent, StreamMode, error_type_from_graphbit_error, error_type_from_string,
};

// ── StreamMode tests ─────────────────────────────────────────────────

#[test]
fn test_stream_mode_from_str() {
    assert_eq!(
        StreamMode::from_str_opt("updates"),
        Some(StreamMode::Updates)
    );
    assert_eq!(
        StreamMode::from_str_opt("messages"),
        Some(StreamMode::Messages)
    );
    assert_eq!(StreamMode::from_str_opt("all"), Some(StreamMode::All));
    assert_eq!(
        StreamMode::from_str_opt("UPDATES"),
        Some(StreamMode::Updates)
    );
    assert_eq!(
        StreamMode::from_str_opt("Messages"),
        Some(StreamMode::Messages)
    );
    assert_eq!(StreamMode::from_str_opt("invalid"), None);
    assert_eq!(StreamMode::from_str_opt(""), None);
}

#[test]
fn test_stream_mode_display() {
    assert_eq!(StreamMode::Updates.to_string(), "updates");
    assert_eq!(StreamMode::Messages.to_string(), "messages");
    assert_eq!(StreamMode::All.to_string(), "all");
}

#[test]
fn test_stream_mode_emits_tokens() {
    assert!(!StreamMode::Updates.emits_tokens());
    assert!(StreamMode::Messages.emits_tokens());
    assert!(StreamMode::All.emits_tokens());
}

#[test]
fn test_stream_mode_emits_tool_events() {
    assert!(StreamMode::Updates.emits_tool_events());
    assert!(!StreamMode::Messages.emits_tool_events());
    assert!(StreamMode::All.emits_tool_events());
}

// ── StreamEvent construction tests ───────────────────────────────────

#[test]
fn test_workflow_started_event() {
    let event = StreamEvent::WorkflowStarted {
        workflow_id: "wf-123".to_string(),
        workflow_name: "test_workflow".to_string(),
        total_nodes: 3,
    };
    // Verify serialization produces the correct tag
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "workflow_started");
    assert_eq!(json["workflow_id"], "wf-123");
    assert_eq!(json["workflow_name"], "test_workflow");
    assert_eq!(json["total_nodes"], 3);
}

#[test]
fn test_node_started_event() {
    let event = StreamEvent::NodeStarted {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "node_started");
    assert_eq!(json["node_id"], "node-1");
    assert_eq!(json["node_name"], "researcher");
}

#[test]
fn test_node_completed_event() {
    let event = StreamEvent::NodeCompleted {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        output: serde_json::json!("The research shows..."),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "node_completed");
    assert_eq!(json["output"], "The research shows...");
}

#[test]
fn test_node_failed_event() {
    let event = StreamEvent::NodeFailed {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        error: "Connection refused".to_string(),
        error_type: "connection_error".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "node_failed");
    assert_eq!(json["error_type"], "connection_error");
}

#[test]
fn test_workflow_failed_event() {
    let event = StreamEvent::WorkflowFailed {
        error: "Timed out after 120s".to_string(),
        error_type: "timeout_error".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "workflow_failed");
    assert_eq!(json["error_type"], "timeout_error");
}

#[test]
fn test_token_event() {
    let event = StreamEvent::Token {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        llm_call_id: "call-1".to_string(),
        content: "The".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "token");
    assert_eq!(json["llm_call_id"], "call-1");
    assert_eq!(json["content"], "The");
}

#[test]
fn test_llm_call_started_event() {
    let event = StreamEvent::LlmCallStarted {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        llm_call_id: "llm-call-1".to_string(),
        iteration: 1,
        model: "gpt-4o-mini".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "llm_call_started");
    assert_eq!(json["llm_call_id"], "llm-call-1");
    assert_eq!(json["iteration"], 1);
    assert_eq!(json["model"], "gpt-4o-mini");
}

#[test]
fn test_llm_call_completed_event() {
    let event = StreamEvent::LlmCallCompleted {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        llm_call_id: "llm-call-1".to_string(),
        iteration: 2,
        finish_reason: "tool_calls".to_string(),
        output: "Need a tool call".to_string(),
        duration_ms: 332.2,
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "llm_call_completed");
    assert_eq!(json["finish_reason"], "tool_calls");
    assert_eq!(json["iteration"], 2);
    assert_eq!(json["output"], "Need a tool call");
}

#[test]
fn test_tool_call_started_event() {
    let event = StreamEvent::ToolCallStarted {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        tool_name: "web_search".to_string(),
        tool_call_id: "call_abc123".to_string(),
        parameters: serde_json::json!({"query": "latest AI research"}),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "tool_call_started");
    assert_eq!(json["tool_name"], "web_search");
    assert_eq!(json["parameters"]["query"], "latest AI research");
}

#[test]
fn test_tool_call_completed_event() {
    let event = StreamEvent::ToolCallCompleted {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        tool_name: "web_search".to_string(),
        tool_call_id: "call_abc123".to_string(),
        output: serde_json::json!("Search results: ..."),
        duration_ms: 1200.5,
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "tool_call_completed");
    assert_eq!(json["duration_ms"], 1200.5);
}

#[test]
fn test_tool_call_failed_event() {
    let event = StreamEvent::ToolCallFailed {
        node_id: "node-1".to_string(),
        node_name: "researcher".to_string(),
        tool_name: "web_search".to_string(),
        tool_call_id: "call_abc123".to_string(),
        error: "Timeout connecting to API".to_string(),
        error_type: "timeout_error".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event"], "tool_call_failed");
    assert_eq!(json["error_type"], "timeout_error");
}

// ── Serialization roundtrip test ─────────────────────────────────────

#[test]
fn test_stream_event_roundtrip() {
    let events = vec![
        StreamEvent::WorkflowStarted {
            workflow_id: "wf-1".to_string(),
            workflow_name: "test".to_string(),
            total_nodes: 2,
        },
        StreamEvent::NodeStarted {
            node_id: "n1".to_string(),
            node_name: "step1".to_string(),
        },
        StreamEvent::Token {
            node_id: "n1".to_string(),
            node_name: "step1".to_string(),
            llm_call_id: "n1-llm-1".to_string(),
            content: "Hello".to_string(),
        },
        StreamEvent::NodeCompleted {
            node_id: "n1".to_string(),
            node_name: "step1".to_string(),
            output: serde_json::json!("Hello world"),
        },
        StreamEvent::WorkflowFailed {
            error: "test error".to_string(),
            error_type: "runtime_error".to_string(),
        },
    ];

    for event in events {
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: StreamEvent = serde_json::from_str(&json).unwrap();
        // Verify the round-trip produces valid JSON
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }
}

// ── Error type classification tests ──────────────────────────────────

#[test]
fn test_error_type_from_graphbit_error() {
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::Network {
            message: "conn refused".to_string()
        }),
        "connection_error"
    );
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::Authentication {
            provider: "openai".to_string(),
            message: "bad key".to_string()
        }),
        "permission_error"
    );
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::Validation {
            field: "model".to_string(),
            message: "invalid".to_string()
        }),
        "value_error"
    );
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::Configuration {
            message: "missing".to_string()
        }),
        "value_error"
    );
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::RateLimit {
            provider: "openai".to_string(),
            retry_after_seconds: 30
        }),
        "runtime_error"
    );
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::Internal {
            message: "unknown".to_string()
        }),
        "runtime_error"
    );
}

#[test]
fn test_error_type_from_graphbit_error_fallback() {
    // Errors without a direct mapping use message-based heuristics
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::WorkflowExecution {
            message: "Workflow execution timed out".to_string()
        }),
        "timeout_error"
    );
    assert_eq!(
        error_type_from_graphbit_error(&GraphBitError::WorkflowExecution {
            message: "Connection lost during execution".to_string()
        }),
        "connection_error"
    );
}

#[test]
fn test_error_type_from_string() {
    assert_eq!(
        error_type_from_string("Connection refused"),
        "connection_error"
    );
    assert_eq!(error_type_from_string("Request timed out"), "timeout_error");
    assert_eq!(
        error_type_from_string("Permission denied"),
        "permission_error"
    );
    assert_eq!(error_type_from_string("Invalid input"), "value_error");
    assert_eq!(
        error_type_from_string("Something went wrong"),
        "runtime_error"
    );
}
