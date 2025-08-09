#[cfg(test)]
mod tests {
    use crate::types::{NodeId, WorkflowContext, WorkflowId};
    use crate::workflow::WorkflowExecutor;
    use serde_json::json;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_workflow_context_node_outputs() {
        let mut context = WorkflowContext::new(WorkflowId::new());
        let node_id = NodeId::from_string("test_node").unwrap();
        let output = json!({
            "result": "success",
            "data": {
                "value": 42,
                "nested": {
                    "key": "nested_value"
                }
            }
        });

        // Store output
        context.set_node_output(&node_id, output.clone());

        // Retrieve and verify
        let stored = context.get_node_output("test_node").unwrap();
        assert_eq!(stored, &output);

        // Test nested access
        let nested_value = context.get_nested_output("test_node.data.value").unwrap();
        assert_eq!(nested_value, &json!(42));

        let deeply_nested = context
            .get_nested_output("test_node.data.nested.key")
            .unwrap();
        assert_eq!(deeply_nested, &json!("nested_value"));
    }

    #[tokio::test]
    async fn test_template_resolution() {
        let mut context = WorkflowContext::new(WorkflowId::new());

        // Set up test data
        context
            .variables
            .insert("name".to_string(), json!("Test User"));
        context.node_outputs.insert(
            "node1".to_string(),
            json!({
                "result": "Success",
                "data": {
                    "value": 42,
                    "nested": {
                        "key": "nested_value"
                    }
                }
            }),
        );

        // Test simple variable substitution
        let template = "Hello, {name}!";
        let result = WorkflowExecutor::resolve_template_variables(template, &context);
        assert_eq!(result, "Hello, Test User!");

        // Test node reference
        let template = "Status: {{node.node1.result}}";
        let result = WorkflowExecutor::resolve_template_variables(template, &context);
        assert_eq!(result, "Status: Success");

        // Test nested property access
        let template = "Value: {{node.node1.data.value}}";
        let result = WorkflowExecutor::resolve_template_variables(template, &context);
        assert_eq!(result, "Value: 42");

        // Test deeply nested property
        let template = "Nested: {{node.node1.data.nested.key}}";
        let result = WorkflowExecutor::resolve_template_variables(template, &context);
        assert_eq!(result, "Nested: nested_value");

        // Test mixed variables
        let template =
            "Hello {name}, status: {{node.node1.result}}, value: {{node.node1.data.value}}";
        let result = WorkflowExecutor::resolve_template_variables(template, &context);
        assert_eq!(result, "Hello Test User, status: Success, value: 42");

        // Test non-existent references
        let template = "Missing: {{node.nonexistent}}";
        let result = WorkflowExecutor::resolve_template_variables(template, &context);
        assert_eq!(result, "Missing: {{node.nonexistent}}");
    }

    #[tokio::test]
    async fn test_template_resolution_edge_cases() {
        let context = WorkflowContext::new(WorkflowId::new());

        // Test empty template
        let result = WorkflowExecutor::resolve_template_variables("", &context);
        assert_eq!(result, "");

        // Test template with no variables
        let result = WorkflowExecutor::resolve_template_variables("No variables here", &context);
        assert_eq!(result, "No variables here");

        // Test malformed references
        let result = WorkflowExecutor::resolve_template_variables("{{node.}}", &context);
        assert_eq!(result, "{{node.}}");

        let result = WorkflowExecutor::resolve_template_variables("{{node}}", &context);
        assert_eq!(result, "{{node}}");

        // Test nested braces
        let result = WorkflowExecutor::resolve_template_variables("{{{node.test}}}", &context);
        assert_eq!(result, "{{{node.test}}}");
    }

    #[tokio::test]
    async fn test_node_output_storage_during_execution() {
        // This test would require a more complex setup with actual agents
        // For now, we'll test the storage mechanism directly
        let context = Arc::new(Mutex::new(WorkflowContext::new(WorkflowId::new())));
        let node_id = NodeId::from_string("test_node").unwrap();
        let output = json!({"result": "test_output"});

        // Simulate node execution storing output
        {
            let mut ctx = context.lock().await;
            ctx.set_node_output(&node_id, output.clone());
        }

        // Verify output was stored
        {
            let ctx = context.lock().await;
            let stored = ctx.get_node_output("test_node").unwrap();
            assert_eq!(stored, &output);
        }
    }

    #[tokio::test]
    async fn test_context_serialization() {
        let mut context = WorkflowContext::new(WorkflowId::new());
        context
            .variables
            .insert("test_var".to_string(), json!("test_value"));
        context
            .node_outputs
            .insert("test_node".to_string(), json!({"output": "test"}));

        // Test serialization
        let serialized = serde_json::to_string(&context).unwrap();
        let deserialized: WorkflowContext = serde_json::from_str(&serialized).unwrap();

        // Verify data is preserved
        assert_eq!(
            deserialized.variables.get("test_var"),
            Some(&json!("test_value"))
        );
        assert_eq!(
            deserialized.node_outputs.get("test_node"),
            Some(&json!({"output": "test"}))
        );
    }

    #[tokio::test]
    async fn test_large_node_outputs() {
        let mut context = WorkflowContext::new(WorkflowId::new());
        let node_id = NodeId::from_string("large_node").unwrap();

        // Create a large output (1MB string)
        let large_string = "x".repeat(1024 * 1024);
        let large_output = json!({"data": large_string});

        // Store and retrieve large output
        context.set_node_output(&node_id, large_output.clone());
        let retrieved = context.get_node_output("large_node").unwrap();
        assert_eq!(retrieved, &large_output);

        // Test nested access on large output
        let nested_data = context.get_nested_output("large_node.data").unwrap();
        assert_eq!(nested_data, &json!(large_string));
    }

    #[tokio::test]
    async fn test_concurrent_node_output_access() {
        let context = Arc::new(Mutex::new(WorkflowContext::new(WorkflowId::new())));
        let node_id = NodeId::from_string("concurrent_node").unwrap();
        let output = json!({"concurrent": "test"});

        // Store output from one task
        let context_clone = context.clone();
        let output_clone = output.clone();
        let store_task = tokio::spawn(async move {
            let mut ctx = context_clone.lock().await;
            ctx.set_node_output(&node_id, output_clone);
        });

        // Read output from another task
        let context_clone2 = context.clone();
        let read_task = tokio::spawn(async move {
            // Wait a bit to ensure store happens first
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            let ctx = context_clone2.lock().await;
            ctx.get_node_output("concurrent_node").cloned()
        });

        // Wait for both tasks
        store_task.await.unwrap();
        let result = read_task.await.unwrap();

        assert_eq!(result, Some(output));
    }
}
