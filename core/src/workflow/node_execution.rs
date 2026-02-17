//! Node execution functions for different node types.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::agents::r#trait::AgentTrait;
use crate::document_loader::DocumentLoader;
use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::{LlmRequest, LlmTool};
use crate::types::{AgentId, NodeId, WorkflowContext};

use super::template::resolve_template_variables;

/// Execute an agent node
pub async fn execute_agent_node(
    current_node_id: &NodeId,
    agent_id: &AgentId,
    prompt_template: &str,
    node_config: &HashMap<String, serde_json::Value>,
    context: Arc<Mutex<WorkflowContext>>,
    agents: Arc<RwLock<HashMap<AgentId, Arc<dyn AgentTrait>>>>,
) -> GraphBitResult<serde_json::Value> {
    let agents_guard = agents.read().await;
    let agent = agents_guard
        .get(agent_id)
        .ok_or_else(|| GraphBitError::agent_not_found(agent_id.to_string()))?
        .clone();
    drop(agents_guard);

    let resolved_prompt = {
        let ctx = context.lock().await;

        let deps_map = ctx
            .metadata
            .get("node_dependencies")
            .cloned()
            .unwrap_or(serde_json::json!({}));
        let id_name_map = ctx
            .metadata
            .get("node_id_to_name")
            .cloned()
            .unwrap_or(serde_json::json!({}));

        let mut sections: Vec<String> = Vec::new();
        let mut parents_json: serde_json::Map<String, serde_json::Value> =
            serde_json::Map::new();

        let id_name_obj = id_name_map.as_object();
        let cur_id_str = current_node_id.to_string();
        let parent_ids: Vec<String> = deps_map
            .as_object()
            .and_then(|m| m.get(&cur_id_str))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        let available_keys: Vec<String> = ctx.node_outputs.keys().cloned().collect();
        tracing::debug!(
            current_node_id = %cur_id_str,
            parent_ids = ?parent_ids,
            available_output_keys = ?available_keys,
            "Implicit preamble: checking direct parents and available outputs"
        );

        for pid in &parent_ids {
            let val_opt = ctx.node_outputs.get(pid).or_else(|| {
                id_name_obj
                    .and_then(|m| m.get(pid))
                    .and_then(|v| v.as_str())
                    .and_then(|name| ctx.node_outputs.get(name))
            });

            if let Some(value) = val_opt {
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };

                sections.push(value_str.clone());
                parents_json.insert(pid.to_string(), value.clone());

                if let Some(parent_name) = id_name_obj
                    .and_then(|m| m.get(pid))
                    .and_then(|v| v.as_str())
                {
                    parents_json.insert(parent_name.to_string(), value.clone());
                }
            } else {
                let name_try = id_name_obj
                    .and_then(|m| m.get(pid))
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                tracing::debug!(
                    current_node_id = %cur_id_str,
                    parent_id = %pid,
                    parent_name = ?name_try,
                    "Implicit preamble: no output found for parent"
                );
            }
        }

        tracing::debug!(
            current_node_id = %cur_id_str,
            section_count = sections.len(),
            "Implicit preamble: built sections"
        );

        let implicit_preamble = if sections.is_empty() {
            "".to_string()
        } else {
            sections.join("\n\n") + "\n\n"
        };

        let combined = format!("{implicit_preamble}{prompt_template}");
        let resolved = resolve_template_variables(&combined, &ctx);
        let preview: String = resolved.chars().take(400).collect();
        tracing::debug!(
            current_node_id = %cur_id_str,
            parent_count = parent_ids.len(),
            preview = %preview,
            "Resolved prompt preview with implicit parent context"
        );
        resolved
    };

    let has_tools = node_config.contains_key("tool_schemas");

    tracing::info!(
        "Agent tool detection - has_tools: {has_tools}, config keys: {:?}",
        node_config.keys().collect::<Vec<_>>()
    );
    if let Some(tool_schemas) = node_config.get("tool_schemas") {
        tracing::info!("Tool schemas found: {tool_schemas}");
    }

    if has_tools {
        tracing::info!("Executing agent with tools - prompt: '{resolved_prompt}'");
        tracing::info!("ENTERING execute_agent_with_tools function");

        let node_name = {
            let ctx = context.lock().await;
            ctx.metadata
                .get("node_id_to_name")
                .and_then(|m| m.as_object())
                .and_then(|m| m.get(&current_node_id.to_string()))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string()
        };

        let result = execute_agent_with_tools(
            agent_id,
            &resolved_prompt,
            node_config,
            agent,
            current_node_id,
            &node_name,
            context.clone(),
        )
        .await;
        tracing::info!("Agent with tools execution result: {:?}", result);
        result
    } else {
        tracing::info!("NO TOOLS DETECTED - using standard agent execution");

        let mut request = LlmRequest::new(resolved_prompt.clone());

        if let Some(temp_value) = node_config.get("temperature") {
            if let Some(temp_num) = temp_value.as_f64() {
                request = request.with_temperature(temp_num as f32);
            }
        }

        if let Some(max_tokens_value) = node_config.get("max_tokens") {
            if let Some(max_tokens_num) = max_tokens_value.as_u64() {
                request = request.with_max_tokens(max_tokens_num as u32);
            }
        }

        let execution_timestamp = chrono::Utc::now();
        let llm_start = std::time::Instant::now();
        let llm_response = agent.llm_provider().complete(request).await?;
        let llm_duration_ms = llm_start.elapsed().as_secs_f64() * 1000.0;

        {
            let node_name = {
                let ctx = context.lock().await;
                ctx.metadata
                    .get("node_id_to_name")
                    .and_then(|m| m.as_object())
                    .and_then(|m| m.get(&current_node_id.to_string()))
                    .and_then(|v| v.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| "unknown".to_string())
            };

            let mut ctx = context.lock().await;
            if let Ok(mut response_metadata) = serde_json::to_value(&llm_response) {
                if let Some(obj) = response_metadata.as_object_mut() {
                    obj.insert(
                        "prompt".to_string(),
                        serde_json::Value::String(resolved_prompt.clone()),
                    );
                    obj.insert(
                        "duration_ms".to_string(),
                        serde_json::json!(llm_duration_ms),
                    );
                    obj.insert(
                        "execution_timestamp".to_string(),
                        serde_json::json!(execution_timestamp.to_rfc3339()),
                    );
                }

                ctx.metadata.insert(
                    format!("node_response_{current_node_id}"),
                    response_metadata.clone(),
                );
                ctx.metadata
                    .insert(format!("node_response_{node_name}"), response_metadata);
            }
        }

        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&llm_response.content) {
            Ok(json_value)
        } else {
            Ok(serde_json::Value::String(llm_response.content))
        }
    }
}

/// Execute an agent with tool calling orchestration
async fn execute_agent_with_tools(
    _agent_id: &AgentId,
    prompt: &str,
    node_config: &HashMap<String, serde_json::Value>,
    agent: Arc<dyn AgentTrait>,
    node_id: &NodeId,
    node_name: &str,
    context: Arc<Mutex<WorkflowContext>>,
) -> GraphBitResult<serde_json::Value> {
    tracing::info!("Starting execute_agent_with_tools for agent: {_agent_id}");

    let tool_schemas = node_config
        .get("tool_schemas")
        .and_then(|v| v.as_array())
        .ok_or_else(|| GraphBitError::validation("node_config", "Missing tool_schemas"))?;

    tracing::info!("Found {} tool schemas", tool_schemas.len());

    let mut tools = Vec::new();
    for schema in tool_schemas {
        if let (Some(name), Some(description), Some(parameters)) = (
            schema.get("name").and_then(|v| v.as_str()),
            schema.get("description").and_then(|v| v.as_str()),
            schema.get("parameters"),
        ) {
            tools.push(LlmTool::new(name, description, parameters.clone()));
        }
    }

    let mut request = LlmRequest::new(prompt);
    for tool in &tools {
        request = request.with_tool(tool.clone());
    }

    if let Some(temp_value) = node_config.get("temperature") {
        if let Some(temp_num) = temp_value.as_f64() {
            request = request.with_temperature(temp_num as f32);
            tracing::debug!("Applied temperature={} to tool selection request", temp_num);
        }
    }

    if let Some(max_tokens_value) = node_config.get("max_tokens") {
        if let Some(max_tokens_num) = max_tokens_value.as_u64() {
            request = request.with_max_tokens(max_tokens_num as u32);
            tracing::debug!(
                "Applied max_tokens={} to tool selection request",
                max_tokens_num
            );
        }
    }

    if let Some(top_p_value) = node_config.get("top_p") {
        if let Some(top_p_num) = top_p_value.as_f64() {
            request = request.with_top_p(top_p_num as f32);
            tracing::debug!("Applied top_p={} to tool selection request", top_p_num);
        }
    }

    tracing::info!("Created LLM request with {} tools", request.tools.len());
    for (i, tool) in request.tools.iter().enumerate() {
        tracing::info!("Tool {i}: {} - {}", tool.name, tool.description);
    }

    tracing::info!(
        "About to call LLM provider with {} tools",
        request.tools.len()
    );

    let execution_timestamp = chrono::Utc::now();
    let llm_start = std::time::Instant::now();
    let llm_response = agent.llm_provider().complete(request).await?;
    let llm_duration_ms = llm_start.elapsed().as_secs_f64() * 1000.0;

    {
        let mut ctx = context.lock().await;
        if let Ok(mut response_metadata) = serde_json::to_value(&llm_response) {
            if let Some(obj) = response_metadata.as_object_mut() {
                obj.insert(
                    "prompt".to_string(),
                    serde_json::Value::String(prompt.to_string()),
                );
                obj.insert(
                    "duration_ms".to_string(),
                    serde_json::json!(llm_duration_ms),
                );
                obj.insert(
                    "execution_timestamp".to_string(),
                    serde_json::json!(execution_timestamp.to_rfc3339()),
                );
            }

            ctx.metadata.insert(
                format!("node_response_{node_id}"),
                response_metadata.clone(),
            );
            ctx.metadata
                .insert(format!("node_response_{node_name}"), response_metadata);
        }
    }

    tracing::info!("LLM Response - Content: '{}'", llm_response.content);
    tracing::info!(
        "LLM Response - Tool calls count: {}",
        llm_response.tool_calls.len()
    );
    for (i, tool_call) in llm_response.tool_calls.iter().enumerate() {
        tracing::info!(
            "Tool call {i}: {} with params: {:?}",
            tool_call.name,
            tool_call.parameters
        );
    }

    if !llm_response.tool_calls.is_empty() {
        tracing::info!(
            "LLM made {} tool calls - these should be executed by the Python layer",
            llm_response.tool_calls.len()
        );

        let tool_calls_json = serde_json::to_value(&llm_response.tool_calls).map_err(|e| {
            GraphBitError::workflow_execution(format!("Failed to serialize tool calls: {e}"))
        })?;

        Ok(serde_json::json!({
            "type": "tool_calls_required",
            "content": llm_response.content,
            "tool_calls": tool_calls_json,
            "original_prompt": prompt,
            "initial_tokens_used": llm_response.usage.completion_tokens,
            "max_tokens_configured": node_config.get("max_tokens").and_then(|v| v.as_u64()),
            "message": "Tool execution should be handled by Python layer with proper tool registry"
        }))
    } else {
        tracing::info!(
            "No tool calls made by LLM, returning original response: {}",
            llm_response.content
        );
        Ok(serde_json::Value::String(llm_response.content))
    }
}

/// Execute a condition node
pub async fn execute_condition_node(_expression: &str) -> GraphBitResult<serde_json::Value> {
    Ok(serde_json::Value::Bool(true))
}

/// Execute a transform node
pub async fn execute_transform_node(
    _transformation: &str,
    _context: Arc<Mutex<WorkflowContext>>,
) -> GraphBitResult<serde_json::Value> {
    Ok(serde_json::Value::String("transformed".to_string()))
}

/// Execute a delay node
pub async fn execute_delay_node(duration_seconds: u64) -> GraphBitResult<serde_json::Value> {
    tokio::time::sleep(tokio::time::Duration::from_secs(duration_seconds)).await;
    Ok(serde_json::Value::String(format!(
        "Delayed for {duration_seconds} seconds",
    )))
}

/// Execute a document loader node
pub async fn execute_document_loader_node(
    document_type: &str,
    source_path: &str,
    _context: Arc<Mutex<WorkflowContext>>,
) -> GraphBitResult<serde_json::Value> {
    let loader = DocumentLoader::new();

    match loader.load_document(source_path, document_type).await {
        Ok(document_content) => {
            let content_json = serde_json::json!({
                "source": document_content.source,
                "document_type": document_content.document_type,
                "content": document_content.content,
                "metadata": document_content.metadata,
                "file_size": document_content.file_size,
                "extracted_at": document_content.extracted_at
            });
            Ok(content_json)
        }
        Err(e) => Err(GraphBitError::workflow_execution(format!(
            "Failed to load document: {e}",
        ))),
    }
}
