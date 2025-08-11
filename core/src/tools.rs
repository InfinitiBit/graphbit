//! Tool calling management for GraphBit
//!
//! This module provides a comprehensive tool calling system that allows:
//! - Tool registration and management
//! - Function schema validation
//! - Tool execution with proper error handling
//! - Tool discovery and introspection

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::{LlmTool, LlmToolCall};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn};

/// A function that can be called by the LLM
pub type ToolFunction = Box<dyn Fn(serde_json::Value) -> GraphBitResult<serde_json::Value> + Send + Sync>;

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Success status
    pub success: bool,
    /// Result data or error message
    pub data: serde_json::Value,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Tool name that was executed
    pub tool_name: String,
}

impl ToolResult {
    /// Create a successful tool result
    pub fn success(tool_name: impl Into<String>, data: serde_json::Value, execution_time_ms: u64) -> Self {
        Self {
            success: true,
            data,
            execution_time_ms,
            tool_name: tool_name.into(),
        }
    }

    /// Create a failed tool result
    pub fn failure(tool_name: impl Into<String>, error: impl Into<String>, execution_time_ms: u64) -> Self {
        Self {
            success: false,
            data: serde_json::Value::String(error.into()),
            execution_time_ms,
            tool_name: tool_name.into(),
        }
    }
}

/// Tool metadata for registration
#[derive(Clone)]
pub struct ToolMetadata {
    /// Tool definition for LLM
    pub definition: LlmTool,
    /// Function to execute
    pub function: Arc<ToolFunction>,
    /// Tool category for organization
    pub category: String,
    /// Tool version
    pub version: String,
    /// Whether the tool is enabled
    pub enabled: bool,
}

impl std::fmt::Debug for ToolMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolMetadata")
            .field("definition", &self.definition)
            .field("function", &"<function>")
            .field("category", &self.category)
            .field("version", &self.version)
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl ToolMetadata {
    /// Create new tool metadata
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: serde_json::Value,
        function: ToolFunction,
    ) -> Self {
        Self {
            definition: LlmTool::new(name, description, parameters),
            function: Arc::new(function),
            category: "general".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
        }
    }

    /// Set the tool category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// Set the tool version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Enable or disable the tool
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Tool calling manager that handles tool registration and execution
#[derive(Debug)]
pub struct ToolManager {
    /// Registered tools
    tools: Arc<RwLock<HashMap<String, ToolMetadata>>>,
    /// Execution statistics
    stats: Arc<RwLock<ToolExecutionStats>>,
}

impl Default for ToolManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone)]
pub struct ToolExecutionStats {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub total_execution_time_ms: u64,
    pub tool_call_counts: HashMap<String, u64>,
}

impl ToolManager {
    /// Create a new tool manager
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ToolExecutionStats::default())),
        }
    }

    /// Register a tool with the manager
    pub fn register_tool(&self, metadata: ToolMetadata) -> GraphBitResult<()> {
        let tool_name = metadata.definition.name.clone();

        // Validate tool parameters schema
        self.validate_tool_schema(&metadata.definition)?;

        let mut tools = self.tools.write().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire tools write lock: {}", e))
        })?;

        if tools.contains_key(&tool_name) {
            warn!("Tool '{}' is being replaced", tool_name);
        }

        tools.insert(tool_name.clone(), metadata);
        info!("Tool '{}' registered successfully", tool_name);

        Ok(())
    }

    /// Unregister a tool
    pub fn unregister_tool(&self, tool_name: &str) -> GraphBitResult<bool> {
        let mut tools = self.tools.write().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire tools write lock: {}", e))
        })?;

        let removed = tools.remove(tool_name).is_some();
        if removed {
            info!("Tool '{}' unregistered successfully", tool_name);
        } else {
            warn!("Tool '{}' was not found for unregistration", tool_name);
        }

        Ok(removed)
    }

    /// Get all registered tool definitions for LLM
    pub fn get_tool_definitions(&self) -> GraphBitResult<Vec<LlmTool>> {
        let tools = self.tools.read().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire tools read lock: {}", e))
        })?;

        let definitions = tools
            .values()
            .filter(|tool| tool.enabled)
            .map(|tool| tool.definition.clone())
            .collect();

        Ok(definitions)
    }

    /// Execute a tool call
    pub fn execute_tool(&self, tool_call: &LlmToolCall) -> GraphBitResult<ToolResult> {
        let start_time = std::time::Instant::now();
        let tool_name = &tool_call.name;

        debug!("Executing tool call: {}", tool_name);

        // Get the tool
        let tools = self.tools.read().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire tools read lock: {}", e))
        })?;

        let tool = match tools.get(tool_name) {
            Some(tool) if tool.enabled => tool,
            Some(_) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                self.update_stats(tool_name, false, execution_time)?;
                return Ok(ToolResult::failure(
                    tool_name,
                    "Tool is disabled",
                    execution_time,
                ));
            }
            None => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                self.update_stats(tool_name, false, execution_time)?;
                return Ok(ToolResult::failure(
                    tool_name,
                    format!("Tool '{}' not found", tool_name),
                    execution_time,
                ));
            }
        };

        let function = Arc::clone(&tool.function);
        drop(tools); // Release the lock early

        // Execute the tool function
        let result = match function(tool_call.parameters.clone()) {
            Ok(result) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                self.update_stats(tool_name, true, execution_time)?;
                ToolResult::success(tool_name, result, execution_time)
            }
            Err(e) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                error!("Tool '{}' execution failed: {}", tool_name, e);
                self.update_stats(tool_name, false, execution_time)?;
                ToolResult::failure(tool_name, e.to_string(), execution_time)
            }
        };

        debug!(
            "Tool '{}' executed in {}ms (success: {})",
            tool_name, result.execution_time_ms, result.success
        );

        Ok(result)
    }

    /// Execute multiple tool calls in parallel
    pub async fn execute_tools_parallel(&self, tool_calls: &[LlmToolCall]) -> GraphBitResult<Vec<ToolResult>> {
        if tool_calls.is_empty() {
            return Ok(Vec::new());
        }

        let mut tasks = Vec::with_capacity(tool_calls.len());

        for tool_call in tool_calls {
            let manager_clone = self.clone();
            let tool_call_clone = tool_call.clone();

            let task = tokio::spawn(async move {
                manager_clone.execute_tool(&tool_call_clone)
            });
            tasks.push(task);
        }

        let mut results = Vec::with_capacity(tool_calls.len());
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result?),
                Err(e) => {
                    error!("Tool execution task failed: {}", e);
                    return Err(GraphBitError::concurrency(format!(
                        "Tool execution task failed: {}",
                        e
                    )));
                }
            }
        }

        Ok(results)
    }

    /// List all registered tools
    pub fn list_tools(&self) -> GraphBitResult<Vec<String>> {
        let tools = self.tools.read().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire tools read lock: {}", e))
        })?;

        Ok(tools.keys().cloned().collect())
    }

    /// Get tool metadata by name
    pub fn get_tool_info(&self, tool_name: &str) -> GraphBitResult<Option<ToolInfo>> {
        let tools = self.tools.read().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire tools read lock: {}", e))
        })?;

        Ok(tools.get(tool_name).map(|tool| ToolInfo {
            name: tool.definition.name.clone(),
            description: tool.definition.description.clone(),
            parameters: tool.definition.parameters.clone(),
            category: tool.category.clone(),
            version: tool.version.clone(),
            enabled: tool.enabled,
        }))
    }

    /// Get execution statistics
    pub fn get_stats(&self) -> GraphBitResult<ToolExecutionStats> {
        let stats = self.stats.read().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire stats read lock: {}", e))
        })?;

        Ok(stats.clone())
    }

    /// Reset execution statistics
    pub fn reset_stats(&self) -> GraphBitResult<()> {
        let mut stats = self.stats.write().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire stats write lock: {}", e))
        })?;

        *stats = ToolExecutionStats::default();
        info!("Tool execution statistics reset");

        Ok(())
    }

    /// Enable or disable a tool
    pub fn set_tool_enabled(&self, tool_name: &str, enabled: bool) -> GraphBitResult<bool> {
        let mut tools = self.tools.write().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire tools write lock: {}", e))
        })?;

        if let Some(tool) = tools.get_mut(tool_name) {
            tool.enabled = enabled;
            info!("Tool '{}' {} ", tool_name, if enabled { "enabled" } else { "disabled" });
            Ok(true)
        } else {
            warn!("Tool '{}' not found for enable/disable", tool_name);
            Ok(false)
        }
    }

    // Private helper methods

    fn validate_tool_schema(&self, tool: &LlmTool) -> GraphBitResult<()> {
        // Basic validation
        if tool.name.is_empty() {
            return Err(GraphBitError::validation("name", "Tool name cannot be empty"));
        }

        if tool.description.is_empty() {
            return Err(GraphBitError::validation("description", "Tool description cannot be empty"));
        }

        // Validate that parameters is a valid JSON schema object
        if !tool.parameters.is_object() {
            return Err(GraphBitError::validation(
                "parameters",
                "Tool parameters must be a JSON object representing a schema",
            ));
        }

        Ok(())
    }

    fn update_stats(&self, tool_name: &str, success: bool, execution_time_ms: u64) -> GraphBitResult<()> {
        let mut stats = self.stats.write().map_err(|e| {
            GraphBitError::concurrency(format!("Failed to acquire stats write lock: {}", e))
        })?;

        stats.total_calls += 1;
        stats.total_execution_time_ms += execution_time_ms;

        if success {
            stats.successful_calls += 1;
        } else {
            stats.failed_calls += 1;
        }

        *stats.tool_call_counts.entry(tool_name.to_string()).or_insert(0) += 1;

        Ok(())
    }
}

// Implement Clone for ToolManager
impl Clone for ToolManager {
    fn clone(&self) -> Self {
        Self {
            tools: Arc::clone(&self.tools),
            stats: Arc::clone(&self.stats),
        }
    }
}

/// Tool information for inspection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub category: String,
    pub version: String,
    pub enabled: bool,
}

/// Global tool manager instance
static GLOBAL_TOOL_MANAGER: std::sync::OnceLock<ToolManager> = std::sync::OnceLock::new();

/// Get the global tool manager instance
pub fn get_global_tool_manager() -> &'static ToolManager {
    GLOBAL_TOOL_MANAGER.get_or_init(ToolManager::new)
}

/// Convenience function to register a tool globally
pub fn register_global_tool(metadata: ToolMetadata) -> GraphBitResult<()> {
    get_global_tool_manager().register_tool(metadata)
}

/// Convenience function to execute a tool globally
pub fn execute_global_tool(tool_call: &LlmToolCall) -> GraphBitResult<ToolResult> {
    get_global_tool_manager().execute_tool(tool_call)
}

/// Convenience function to get all tool definitions globally
pub fn get_global_tool_definitions() -> GraphBitResult<Vec<LlmTool>> {
    get_global_tool_manager().get_tool_definitions()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_registration() {
        let manager = ToolManager::new();

        let tool = ToolMetadata::new(
            "test_tool",
            "A test tool",
            json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                },
                "required": ["name"]
            }),
            Box::new(|params| {
                Ok(json!({"result": format!("Hello, {}!", params["name"])}))
            }),
        );

        assert!(manager.register_tool(tool).is_ok());
        assert!(manager.list_tools().unwrap().contains(&"test_tool".to_string()));
    }

    #[test]
    fn test_tool_execution() {
        let manager = ToolManager::new();

        let tool = ToolMetadata::new(
            "echo_tool",
            "Echo the input",
            json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                },
                "required": ["message"]
            }),
            Box::new(|params| Ok(params)),
        );

        manager.register_tool(tool).unwrap();

        let tool_call = LlmToolCall {
            id: "test_call".to_string(),
            name: "echo_tool".to_string(),
            parameters: json!({"message": "Hello, World!"}),
        };

        let result = manager.execute_tool(&tool_call).unwrap();
        assert!(result.success);
        assert_eq!(result.data["message"], "Hello, World!");
    }

    #[test]
    fn test_tool_not_found() {
        let manager = ToolManager::new();

        let tool_call = LlmToolCall {
            id: "test_call".to_string(),
            name: "nonexistent_tool".to_string(),
            parameters: json!({}),
        };

        let result = manager.execute_tool(&tool_call).unwrap();
        assert!(!result.success);
        assert!(result.data.as_str().unwrap().contains("not found"));
    }
}