use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[napi(object)]
#[derive(Clone)]
pub struct ToolResult {
    pub success: bool,
    #[napi(ts_type = "any")]
    pub result: Value,
    pub error: Option<String>,
    pub execution_time_ms: f64,
}

use graphbit_core::llm::LlmTool;

struct RegisteredTool {
    definition: LlmTool,
    callback: ThreadsafeFunction<Value, ErrorStrategy::Fatal>,
}

#[napi]
pub struct ToolRegistry {
    tools: Arc<Mutex<HashMap<String, RegisteredTool>>>,
}

// Internal implementation for Rust usage
impl ToolRegistry {
    pub fn create() -> Self {
        Self {
            tools: Arc::new(Mutex::new(HashMap::new())),
        }
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

        let tool = LlmTool::new(name.clone(), description, parameters);
        
        let registered = RegisteredTool {
            definition: tool,
            callback: tsfn,
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
        
        let result: napi::Result<Value> = tool.call_async(args).await;
        
        let duration = start.elapsed().as_secs_f64() * 1000.0;

        match result {
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
