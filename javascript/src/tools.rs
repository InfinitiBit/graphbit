use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot;
use std::sync::atomic::{AtomicU64, Ordering};

/// Tool execution result
#[napi(object)]
#[derive(Clone)]
pub struct ToolResult {
    pub success: bool,
    pub result: Value,
    pub error: Option<String>,
    pub execution_time_ms: f64,
}

/// Tool metadata for monitoring and management
#[napi(object)]
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    #[napi(ts_type = "any")]
    pub parameters_schema: Value,
    pub created_at: f64,
    pub call_count: u32,
    pub total_duration_ms: f64,
    pub avg_duration_ms: f64,
    pub last_called_at: Option<f64>,
}

/// Tool execution record for history tracking
#[napi(object)]
#[derive(Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    pub tool_name: String,
    pub success: bool,
    pub duration_ms: f64,
    pub timestamp: f64,
    pub error: Option<String>,
}

/// Tool statistics for monitoring
#[napi(object)]
pub struct ToolStats {
    pub total_tools: u32,
    pub total_executions: u32,
    pub successful_executions: u32,
    pub failed_executions: u32,
    pub avg_execution_time_ms: f64,
    pub total_execution_time_ms: f64,
}

use graphbit_core::llm::LlmTool;

/// Pending async result for callback ID pattern
#[derive(Clone)]
struct AsyncPendingResult {
    result: Option<Value>,
    error: Option<String>,
    completed: bool,
}

/// Global counter for generating unique pending IDs
static PENDING_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

struct RegisteredTool {
    definition: LlmTool,
    callback: ThreadsafeFunction<Value, ErrorStrategy::Fatal>,
    metadata: ToolMetadata,
}

#[napi]
pub struct ToolRegistry {
    tools: Arc<Mutex<HashMap<String, RegisteredTool>>>,
    execution_history: Arc<Mutex<Vec<ToolExecution>>>,
    /// Pending results for async callbacks - stores resolved values by pending_id
    pending_results: Arc<Mutex<HashMap<String, AsyncPendingResult>>>,
    /// Notifiers for waiting tasks - oneshot senders by pending_id
    pending_notifiers: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
}

// Internal implementation for Rust usage
impl ToolRegistry {
    pub fn create() -> Self {
        Self {
            tools: Arc::new(Mutex::new(HashMap::new())),
            execution_history: Arc::new(Mutex::new(Vec::new())),
            pending_results: Arc::new(Mutex::new(HashMap::new())),
            pending_notifiers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_timestamp() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
    }
}

// NAPI implementation for JavaScript usage
#[napi]
impl ToolRegistry {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::create()
    }

    #[napi]
    pub fn register(
        &self,
        name: String,
        description: String,
        #[napi(ts_arg_type = "any")]
        parameters: Value,
        callback: JsFunction,
    ) -> napi::Result<()> {
        let tsfn = callback.create_threadsafe_function(
            0,
            |ctx: napi::threadsafe_function::ThreadSafeCallContext<Value>| {
                Ok(vec![ctx.value])
            },
        )?;

        let tool = LlmTool::new(name.clone(), description.clone(), parameters.clone());
        
        let metadata = ToolMetadata {
            name: name.clone(),
            description,
            parameters_schema: parameters,
            created_at: Self::get_timestamp(),
            call_count: 0,
            total_duration_ms: 0.0,
            avg_duration_ms: 0.0,
            last_called_at: None,
        };
        
        let registered = RegisteredTool {
            definition: tool,
            callback: tsfn,
            metadata,
        };

        let mut tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        tools.insert(name, registered);
        
        Ok(())
    }

    #[napi]
    pub async fn execute(&self, name: String, #[napi(ts_arg_type = "any")] args: Value) -> napi::Result<ToolResult> {
        let tool = {
            let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
            tools.get(&name).ok_or_else(|| napi::Error::from_reason(format!("Tool not found: {}", name)))?
                .callback
                .clone()
        };

        let start = std::time::Instant::now();
        
        // Generate unique pending ID for callback identification
        let pending_id = format!("pending_{}", PENDING_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
        
        // Create oneshot channel for async notification
        let (tx, rx) = oneshot::channel::<()>();
        
        // Store the notifier
        {
            let mut notifiers = self.pending_notifiers.lock()
                .map_err(|_| napi::Error::from_reason("Failed to lock notifiers"))?;
            notifiers.insert(pending_id.clone(), tx);
        }
        
        // Wrap args with pending_id so JS knows how to call back
        let wrapper_args = serde_json::json!({
            "__pendingId": pending_id,
            "__originalArgs": args
        });
        
        // Call the JavaScript function
        let immediate_result: napi::Result<Value> = tool.call_async(wrapper_args).await;
        
        // Check if the result indicates a pending async operation
        let (final_result, is_pending) = match &immediate_result {
            Ok(val) => {
                // Check if this is a pending marker from the JS wrapper
                if let Some(obj) = val.as_object() {
                    if obj.get("__pending").and_then(|v| v.as_bool()) == Some(true) {
                        (None, true)
                    } else {
                        // Direct result (sync callback)
                        (Some(val.clone()), false)
                    }
                } else {
                    // Primitive result (sync callback)
                    (Some(val.clone()), false)
                }
            }
            Err(_) => (None, false),
        };
        
        let resolved_result = if is_pending {
            // Wait for JS to call set_pending_result
            let _ = rx.await;
            
            // Retrieve the pending result
            let mut pending = self.pending_results.lock()
                .map_err(|_| napi::Error::from_reason("Failed to lock pending results"))?;
            
            if let Some(result) = pending.remove(&pending_id) {
                if let Some(err) = result.error {
                    Err(napi::Error::from_reason(err))
                } else {
                    Ok(result.result.unwrap_or(Value::Null))
                }
            } else {
                Err(napi::Error::from_reason("Pending result not found"))
            }
        } else {
            // Clean up notifier for sync path
            {
                let mut notifiers = self.pending_notifiers.lock()
                    .map_err(|_| napi::Error::from_reason("Failed to lock notifiers"))?;
                notifiers.remove(&pending_id);
            }
            
            // Use immediate result
            match (final_result, immediate_result) {
                (Some(val), _) => Ok(val),
                (None, Ok(val)) => Ok(val),
                (None, Err(e)) => Err(e),
            }
        };
        
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        let success = resolved_result.is_ok();

        // Update metadata
        {
            let mut tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
            if let Some(tool) = tools.get_mut(&name) {
                tool.metadata.call_count += 1;
                tool.metadata.total_duration_ms += duration;
                tool.metadata.avg_duration_ms = tool.metadata.total_duration_ms / tool.metadata.call_count as f64;
                tool.metadata.last_called_at = Some(Self::get_timestamp());
            }
        }

        // Record execution in history
        let execution = ToolExecution {
            tool_name: name.clone(),
            success,
            duration_ms: duration,
            timestamp: Self::get_timestamp(),
            error: if success { None } else { Some("Execution failed".to_string()) },
        };
        
        {
            let mut history = self.execution_history.lock().map_err(|_| napi::Error::from_reason("Failed to lock history"))?;
            history.push(execution);
        }

        match resolved_result {
            Ok(val) => Ok(ToolResult {
                success: true,
                result: val,
                error: None,
                execution_time_ms: duration,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                result: Value::Null,
                error: Some(e.to_string()),
                execution_time_ms: duration,
            }),
        }
    }

    /// Set the result for a pending async callback
    /// 
    /// Called from JavaScript when an async callback's Promise resolves.
    /// This stores the result and wakes the waiting Rust task.
    ///
    /// # Arguments
    /// * `pending_id` - The pending ID that was passed to the callback
    /// * `result` - The resolved result value (null if error)
    /// * `error` - Optional error message if the Promise rejected
    #[napi]
    pub fn set_pending_result(
        &self,
        pending_id: String,
        #[napi(ts_arg_type = "any")]
        result: Option<Value>,
        error: Option<String>,
    ) -> napi::Result<()> {
        // Store the result
        {
            let mut pending = self.pending_results.lock()
                .map_err(|_| napi::Error::from_reason("Failed to lock pending results"))?;
            pending.insert(pending_id.clone(), AsyncPendingResult {
                result,
                error,
                completed: true,
            });
        }
        
        // Notify the waiting task
        {
            let mut notifiers = self.pending_notifiers.lock()
                .map_err(|_| napi::Error::from_reason("Failed to lock notifiers"))?;
            if let Some(tx) = notifiers.remove(&pending_id) {
                let _ = tx.send(()); // Ignore send errors (task may have timed out)
            }
        }
        
        Ok(())
    }

    #[napi]
    pub fn get_tool_definition(&self, name: String) -> napi::Result<Option<serde_json::Value>> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        if let Some(tool) = tools.get(&name) {
            Ok(Some(serde_json::to_value(&tool.definition).map_err(|e| napi::Error::from_reason(e.to_string()))?))
        } else {
            Ok(None)
        }
    }

    #[napi]
    pub fn get_tool(&self, name: String) -> napi::Result<Option<serde_json::Value>> {
        self.get_tool_definition(name)
    }

    #[napi]
    pub fn has_tool(&self, name: String) -> napi::Result<bool> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        Ok(tools.contains_key(&name))
    }
    
    #[napi]
    pub fn get_registered_tools(&self) -> napi::Result<Vec<String>> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        Ok(tools.keys().cloned().collect())
    }

    // ========== NEW ENHANCED METHODS ==========

    /// Unregister a tool by name
    ///
    /// # Arguments
    /// * `name` - Name of the tool to unregister
    ///
    /// # Returns
    /// true if tool was found and removed, false if not found
    ///
    /// # Example
    ///
    /// ```javascript
    /// const removed = registry.unregisterTool('search_web');
    /// if (removed) {
    ///   console.log('Tool removed successfully');
    /// }
    /// ```
    #[napi]
    pub fn unregister_tool(&self, name: String) -> napi::Result<bool> {
        let mut tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        Ok(tools.remove(&name).is_some())
    }

    /// Get metadata for a specific tool
    ///
    /// Returns metadata including call count, durations, and timestamps.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const metadata = registry.getToolMetadata('search_web');
    /// if (metadata) {
    ///   console.log(`Calls: ${metadata.callCount}`);
    ///   console.log(`Avg duration: ${metadata.avgDurationMs}ms`);
    /// }
    /// ```
    #[napi]
    pub fn get_tool_metadata(&self, name: String) -> napi::Result<Option<ToolMetadata>> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        Ok(tools.get(&name).map(|tool| tool.metadata.clone()))
    }

    /// Get metadata for all registered tools
    ///
    /// # Example
    ///
    /// ```javascript
    /// const allMetadata = registry.getAllMetadata();
    /// allMetadata.forEach(meta => {
    ///   console.log(`${meta.name}: ${meta.callCount} calls`);
    /// });
    /// ```
    #[napi]
    pub fn get_all_metadata(&self) -> napi::Result<Vec<ToolMetadata>> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        Ok(tools.values().map(|tool| tool.metadata.clone()).collect())
    }

    /// Get execution history
    ///
    /// Returns a list of all tool executions with timestamps and durations.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const history = registry.getExecutionHistory();
    /// history.forEach(exec => {
    ///   console.log(`${exec.toolName}: ${exec.durationMs}ms at ${new Date(exec.timestamp * 1000)}`);
    /// });
    /// ```
    #[napi]
    pub fn get_execution_history(&self) -> napi::Result<Vec<ToolExecution>> {
        let history = self.execution_history.lock().map_err(|_| napi::Error::from_reason("Failed to lock history"))?;
        Ok(history.clone())
    }

    /// Clear execution history
    ///
    /// Removes all execution records but preserves tool metadata.
    ///
    /// # Example
    ///
    /// ```javascript
    /// registry.clearHistory();
    /// console.log('Execution history cleared');
    /// ```
    #[napi]
    pub fn clear_history(&self) -> napi::Result<()> {
        let mut history = self.execution_history.lock().map_err(|_| napi::Error::from_reason("Failed to lock history"))?;
        history.clear();
        Ok(())
    }

    /// Get comprehensive statistics across all tools
    ///
    /// Returns aggregated statistics including total executions,
    /// success rates, and performance metrics.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const stats = registry.getStats();
    /// console.log(`Total tools: ${stats.totalTools}`);
    /// console.log(`Total executions: ${stats.totalExecutions}`);
    /// console.log(`Success rate: ${(stats.successfulExecutions / stats.totalExecutions * 100).toFixed(2)}%`);
    /// console.log(`Avg execution time: ${stats.avgExecutionTimeMs.toFixed(2)}ms`);
    /// ```
    #[napi]
    pub fn get_stats(&self) -> napi::Result<ToolStats> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        let history = self.execution_history.lock().map_err(|_| napi::Error::from_reason("Failed to lock history"))?;

        let total_tools = tools.len() as u32;
        let total_executions = history.len() as u32;
        let successful_executions = history.iter().filter(|e| e.success).count() as u32;
        let failed_executions = total_executions - successful_executions;

        let total_execution_time_ms: f64 = history.iter().map(|e| e.duration_ms).sum();
        let avg_execution_time_ms = if total_executions > 0 {
            total_execution_time_ms / total_executions as f64
        } else {
            0.0
        };

        Ok(ToolStats {
            total_tools,
            total_executions,
            successful_executions,
            failed_executions,
            avg_execution_time_ms,
            total_execution_time_ms,
        })
    }

    /// Clear all tools from the registry
    ///
    /// Removes all registered tools and clears history.
    ///
    /// # Example
    ///
    /// ```javascript
    /// registry.clearAll();
    /// console.log('All tools cleared');
    /// ```
    #[napi]
    pub fn clear_all(&self) -> napi::Result<()> {
        let mut tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        let mut history = self.execution_history.lock().map_err(|_| napi::Error::from_reason("Failed to lock history"))?;
        
        tools.clear();
        history.clear();
        
        Ok(())
    }

    /// Get tools in LLM-compatible format
    ///
    /// Returns tool definitions formatted for LLM tool calling.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const llmTools = registry.getLlmTools();
    /// // Pass to LLM that supports tool calling
    /// ```
    #[napi]
    pub fn get_llm_tools(&self) -> napi::Result<Vec<Value>> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        
        let mut llm_tools = Vec::new();
        for tool in tools.values() {
            let tool_json = serde_json::to_value(&tool.definition)
                .map_err(|e| napi::Error::from_reason(format!("Failed to serialize tool: {}", e)))?;
            llm_tools.push(tool_json);
        }
        
        Ok(llm_tools)
    }

    /// Get count of registered tools
    ///
    /// # Example
    ///
    /// ```javascript
    /// const count = registry.getToolCount();
    /// console.log(`${count} tools registered`);
    /// ```
    #[napi]
    pub fn get_tool_count(&self) -> napi::Result<u32> {
        let tools = self.tools.lock().map_err(|_| napi::Error::from_reason("Failed to lock registry"))?;
        Ok(tools.len() as u32)
    }
}

// Helper to create a registry instance
#[napi]
pub fn create_tool_registry() -> ToolRegistry {
    ToolRegistry::create()
}

#[napi]
pub fn tool(
    name: String,
    description: String,
    #[napi(ts_arg_type = "any")]
    parameters: Value,
    callback: JsFunction,
) -> napi::Result<RegisteredToolWrapper> {
    Ok(RegisteredToolWrapper {
        name,
        description,
        parameters,
        callback,
    })
}

#[napi(object)]
pub struct RegisteredToolWrapper {
    pub name: String,
    pub description: String,
    #[napi(ts_type = "any")]
    pub parameters: Value,
    #[napi(ts_type = "(...args: any[]) => any")]
    pub callback: JsFunction,
}
