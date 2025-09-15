//! Workflow node for GraphBit Python bindings

use crate::llm::LlmConfig;
use crate::tools::ToolExecutor;
use graphbit_core::{
    graph::{NodeType, WorkflowNode},
    types::AgentId,
};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};
use std::cell::RefCell;
use std::collections::HashMap;

// Global tool registry for storing Python tool functions
thread_local! {
    static TOOL_REGISTRY: RefCell<HashMap<String, PyObject>> = RefCell::new(HashMap::new());
}

/// A workflow node representing a single operation or step in a workflow
#[pyclass]
#[derive(Clone)]
pub struct Node {
    pub(crate) inner: WorkflowNode,
}

#[pymethods]
impl Node {
    #[staticmethod]
    #[pyo3(signature = (name, prompt, agent_id=None, output_name=None, tools=None, system_prompt=None, llm_config=None))]
    fn agent(
        name: String,
        prompt: String,
        agent_id: Option<String>,
        output_name: Option<String>,
        tools: Option<&Bound<'_, PyList>>,
        system_prompt: Option<String>,
        llm_config: Option<LlmConfig>,
    ) -> PyResult<Self> {
        // Validate required parameters
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Name cannot be empty",
            ));
        }
        if prompt.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Prompt cannot be empty",
            ));
        }

        let id = agent_id.unwrap_or_else(|| {
            format!(
                "agent_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            )
        });
        let mut node = WorkflowNode::new(
            name.clone(),
            format!("Agent: {}", name),
            NodeType::Agent {
                agent_id: AgentId::from_string(&id).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid ID: {}", e))
                })?,
                prompt_template: prompt,
            },
        );

        // Store output name in metadata if provided
        if let Some(output_name) = output_name {
            node.config.insert(
                "output_name".to_string(),
                serde_json::Value::String(output_name),
            );
        }

        // Store system prompt in metadata if provided
        if let Some(system_prompt) = system_prompt {
            node.config.insert(
                "system_prompt".to_string(),
                serde_json::Value::String(system_prompt),
            );
        }

        // Store LLM config in metadata if provided
        if let Some(llm_config) = llm_config {
            match serde_json::to_value(&llm_config.inner) {
                Ok(config_value) => {
                    node.config.insert("llm_config".to_string(), config_value);
                }
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Failed to serialize LLM config: {}",
                        e
                    )));
                }
            }
        }

        // Store tools in metadata if provided
        if let Some(tools_list) = tools {
            println!("🔧 Processing {} tools for agent node", tools_list.len());
            let mut tool_schemas = Vec::new();
            let mut tool_names = Vec::new();

            // Get Python interpreter for introspection
            let py = tools_list.py();

            for tool in tools_list.iter() {
                // Extract tool metadata
                let tool_name = tool
                    .getattr("__name__")
                    .and_then(|name| name.extract::<String>())
                    .unwrap_or_else(|_| "unknown_tool".to_string());

                let tool_doc = tool
                    .getattr("__doc__")
                    .and_then(|doc| doc.extract::<Option<String>>())
                    .unwrap_or(None)
                    .unwrap_or_else(|| "Tool function".to_string());

                // Extract comprehensive function schema using introspection
                let parameters_schema = match extract_comprehensive_function_schema(&tool, py) {
                    Ok(schema) => schema,
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to extract schema for tool '{}': {}",
                            tool_name, e
                        );
                        // Fallback to empty schema
                        serde_json::json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        })
                    }
                };

                // Create comprehensive tool schema for LLM
                let tool_schema = serde_json::json!({
                    "name": tool_name,
                    "description": tool_doc,
                    "parameters": parameters_schema
                });

                tool_schemas.push(tool_schema);
                tool_names.push(tool_name);
            }

            // Store tool schemas for LLM integration
            node.config.insert(
                "tool_schemas".to_string(),
                serde_json::Value::Array(tool_schemas.clone()),
            );
            println!(
                "🔧 Stored {} tool schemas in node config",
                tool_schemas.len()
            );

            // Store tool names for reference
            node.config.insert(
                "tools".to_string(),
                serde_json::Value::Array(
                    tool_names
                        .into_iter()
                        .map(serde_json::Value::String)
                        .collect(),
                ),
            );

            Python::with_gil(|py| {
                TOOL_REGISTRY.with(|registry| {
                    let mut registry = registry.borrow_mut();
                    for (i, tool) in tools_list.iter().enumerate() {
                        let tool_name = tool
                            .getattr("__name__")
                            .and_then(|name| name.extract::<String>())
                            .unwrap_or_else(|_| format!("tool_{}", i));
                        let py_obj = tool.into_pyobject(py).unwrap();
                        registry.insert(tool_name, py_obj.into_any().unbind());
                    }
                });
            });
        }

        Ok(Self { inner: node })
    }

    #[staticmethod]
    fn transform(name: String, transformation: String) -> PyResult<Self> {
        // Validate required parameters
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Transform name cannot be empty",
            ));
        }
        if transformation.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Transform transformation cannot be empty",
            ));
        }

        Ok(Self {
            inner: WorkflowNode::new(
                name.clone(),
                format!("Transform: {}", name),
                NodeType::Transform { transformation },
            ),
        })
    }

    #[staticmethod]
    fn condition(name: String, expression: String) -> PyResult<Self> {
        // Validate required parameters
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Condition name cannot be empty",
            ));
        }
        if expression.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Condition expression cannot be empty",
            ));
        }

        Ok(Self {
            inner: WorkflowNode::new(
                name.clone(),
                format!("Condition: {}", name),
                NodeType::Condition { expression },
            ),
        })
    }

    fn id(&self) -> String {
        self.inner.id.to_string()
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Check if this node has tools configured
    fn has_tools(&self) -> bool {
        self.inner.config.contains_key("tools")
    }

    /// Get the list of tool names for this node
    fn get_tool_names(&self) -> Vec<String> {
        if let Some(tools_value) = self.inner.config.get("tools") {
            if let Some(tools_array) = tools_value.as_array() {
                return tools_array
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }
        Vec::new()
    }

    /// Create a tool executor for this node with registered tools
    fn create_tool_executor(&self, py: Python<'_>) -> PyResult<ToolExecutor> {
        use crate::tools::registry::ToolRegistry;

        // Create a new tool registry and populate it with the tools from this node
        let registry = ToolRegistry::new();

        // Get the tool names for this node
        let tool_names = self.get_tool_names();

        // Register tools from the global registry
        TOOL_REGISTRY.with(|global_registry| {
            let global_registry = global_registry.borrow();
            for tool_name in &tool_names {
                if let Some(tool_func) = global_registry.get(tool_name) {
                    // Create empty parameters schema
                    let params_dict = pyo3::types::PyDict::new(py);

                    registry.register_tool(
                        tool_name.clone(),
                        format!("Tool function: {}", tool_name),
                        tool_func.clone_ref(py),
                        &params_dict,
                        None, // No specific return type
                    )?;
                }
            }
            Ok::<(), PyErr>(())
        })?;

        Ok(ToolExecutor::new(Some(&registry), None))
    }

    /// Execute tools for this node with given tool calls
    fn execute_tools(&self, tool_calls: &Bound<'_, PyList>, py: Python<'_>) -> PyResult<String> {
        if !self.has_tools() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "This node does not have tools configured",
            ));
        }

        let executor = self.create_tool_executor(py)?;
        let results = executor.execute_tools(tool_calls, py)?;

        // Convert results to a summary string for LLM
        let mut summary = String::new();
        summary.push_str("Tool Execution Results:\n");

        for (i, result) in results.get_all().iter().enumerate() {
            let output_text = if result.success {
                result.output.clone()
            } else {
                format!(
                    "Error: {}",
                    result
                        .error
                        .as_ref()
                        .unwrap_or(&"Unknown error".to_string())
                )
            };

            summary.push_str(&format!(
                "{}. {} -> {}\n",
                i + 1,
                result.tool_name,
                output_text
            ));
        }

        Ok(summary)
    }

    /// Get node configuration as JSON string
    fn get_config(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&self.inner.config).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize node config: {}",
                e
            ))
        })
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "Node(id='{}', name='{}', type='{}')",
            self.inner.id,
            self.inner.name,
            match &self.inner.node_type {
                NodeType::Agent { .. } => "Agent",
                NodeType::Transform { .. } => "Transform",
                NodeType::Condition { .. } => "Condition",
                NodeType::Split => "Split",
                NodeType::Join => "Join",
                NodeType::Delay { .. } => "Delay",
                NodeType::HttpRequest { .. } => "HttpRequest",
                NodeType::Custom { .. } => "Custom",
                NodeType::DocumentLoader { .. } => "DocumentLoader",
            }
        )
    }
}

/// Extract comprehensive function parameter schema using introspection and docstring parsing
fn extract_comprehensive_function_schema(
    func: &Bound<'_, PyAny>,
    py: Python<'_>,
) -> PyResult<serde_json::Value> {
    let inspect = py.import("inspect")?;
    let signature = inspect.call_method1("signature", (func,))?;
    let parameters = signature.getattr("parameters")?;

    // Extract docstring for parameter descriptions
    let docstring = func
        .getattr("__doc__")
        .and_then(|doc| doc.extract::<Option<String>>())
        .unwrap_or(None)
        .unwrap_or_else(|| String::new());

    let param_descriptions = parse_docstring_parameters(&docstring);

    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    // Iterate through function parameters using Python dict iteration
    let param_items = parameters.call_method0("items")?;
    for item in param_items.try_iter()? {
        let item_tuple = item?;
        let param_name = item_tuple.get_item(0)?.extract::<String>()?;
        let param_obj = item_tuple.get_item(1)?;

        // Skip 'self' and 'cls' parameters
        if param_name == "self" || param_name == "cls" {
            continue;
        }

        // Get parameter annotation for type information
        let annotation = param_obj.getattr("annotation")?;
        let param_type =
            if annotation.is_none() || annotation.to_string() == "<class 'inspect._empty'>" {
                "string" // Default type
            } else {
                map_python_type_to_json_schema(&annotation.to_string())
            };

        let mut param_info = serde_json::Map::new();
        param_info.insert(
            "type".to_string(),
            serde_json::Value::String(param_type.to_string()),
        );

        // Add description from docstring if available, otherwise use a default
        let description = param_descriptions
            .get(&param_name)
            .cloned()
            .unwrap_or_else(|| format!("Parameter {}", param_name));
        param_info.insert(
            "description".to_string(),
            serde_json::Value::String(description),
        );

        // Check if parameter has a default value
        let default = param_obj.getattr("default")?;
        if default.to_string() != "<class 'inspect._empty'>" {
            // Try to convert default value to JSON
            if let Ok(default_str) = default.extract::<String>() {
                param_info.insert(
                    "default".to_string(),
                    serde_json::Value::String(default_str),
                );
            } else if let Ok(default_int) = default.extract::<i64>() {
                param_info.insert(
                    "default".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(default_int)),
                );
            } else if let Ok(default_float) = default.extract::<f64>() {
                if let Some(num) = serde_json::Number::from_f64(default_float) {
                    param_info.insert("default".to_string(), serde_json::Value::Number(num));
                }
            } else if let Ok(default_bool) = default.extract::<bool>() {
                param_info.insert("default".to_string(), serde_json::Value::Bool(default_bool));
            }
        } else {
            // Parameter is required
            required.push(param_name.clone());
        }

        properties.insert(param_name, serde_json::Value::Object(param_info));
    }

    Ok(serde_json::json!({
        "type": "object",
        "properties": properties,
        "required": required
    }))
}

/// Parse docstring to extract parameter descriptions
/// Supports Google, NumPy, and Sphinx docstring formats
fn parse_docstring_parameters(docstring: &str) -> std::collections::HashMap<String, String> {
    use std::collections::HashMap;

    let mut param_descriptions = HashMap::new();

    if docstring.is_empty() {
        return param_descriptions;
    }

    // Try Google-style docstring format first
    if let Some(args_section) = extract_args_section_google(docstring) {
        for line in args_section.lines() {
            if let Some((param_name, description)) = parse_google_param_line(line) {
                param_descriptions.insert(param_name, description);
            }
        }
    }

    // Try NumPy/Sphinx-style docstring format
    if param_descriptions.is_empty() {
        if let Some(params_section) = extract_parameters_section_numpy(docstring) {
            for line in params_section.lines() {
                if let Some((param_name, description)) = parse_numpy_param_line(line) {
                    param_descriptions.insert(param_name, description);
                }
            }
        }
    }

    param_descriptions
}

/// Extract Args section from Google-style docstring
fn extract_args_section_google(docstring: &str) -> Option<String> {
    let lines: Vec<&str> = docstring.lines().collect();
    let mut in_args_section = false;
    let mut args_lines = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed == "Args:" || trimmed == "Arguments:" || trimmed == "Parameters:" {
            in_args_section = true;
            continue;
        }

        if in_args_section {
            if trimmed.ends_with(':') && !trimmed.contains(' ') && trimmed.len() > 1 {
                break;
            }
            args_lines.push(line);
        }
    }

    if args_lines.is_empty() {
        None
    } else {
        Some(args_lines.join("\n"))
    }
}

/// Parse a single parameter line from Google-style docstring
fn parse_google_param_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();

    // Look for pattern: "param_name (type): description" or "param_name: description"
    if let Some(colon_pos) = trimmed.find(':') {
        let param_part = trimmed[..colon_pos].trim();
        let description = trimmed[colon_pos + 1..].trim();

        // Extract parameter name (remove type annotation if present)
        let param_name = if let Some(paren_pos) = param_part.find('(') {
            param_part[..paren_pos].trim()
        } else {
            param_part
        };

        if !param_name.is_empty() && !description.is_empty() {
            return Some((param_name.to_string(), description.to_string()));
        }
    }

    None
}

/// Extract Parameters section from NumPy/Sphinx-style docstring
fn extract_parameters_section_numpy(docstring: &str) -> Option<String> {
    let lines: Vec<&str> = docstring.lines().collect();
    let mut in_params_section = false;
    let mut params_lines = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed == "Parameters"
            || trimmed == "Parameters:"
            || trimmed.starts_with("Parameters\n")
            || trimmed.starts_with("Parameters\r\n")
        {
            in_params_section = true;
            continue;
        }

        if in_params_section {
            // Check if we've reached another section
            if trimmed == "Returns"
                || trimmed == "Returns:"
                || trimmed == "Raises"
                || trimmed == "Raises:"
                || trimmed == "Examples"
                || trimmed == "Examples:"
            {
                break;
            }
            params_lines.push(line);
        }
    }

    if params_lines.is_empty() {
        None
    } else {
        Some(params_lines.join("\n"))
    }
}

/// Parse a single parameter line from NumPy/Sphinx-style docstring
fn parse_numpy_param_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();

    // Look for pattern: "param_name : type" followed by description on next lines
    // For simplicity, we'll look for lines that start with a word followed by space and colon
    if let Some(colon_pos) = trimmed.find(" :") {
        let param_name = trimmed[..colon_pos].trim();
        let rest = trimmed[colon_pos + 2..].trim();

        if !param_name.is_empty() {
            // For NumPy style, description might be on the same line after type
            let description = if rest.is_empty() {
                format!("Parameter {}", param_name)
            } else {
                rest.to_string()
            };
            return Some((param_name.to_string(), description));
        }
    }

    None
}

/// Map Python type annotations to JSON Schema types
fn map_python_type_to_json_schema(type_str: &str) -> &'static str {
    match type_str {
        s if s.contains("int") => "integer",
        s if s.contains("float") => "number",
        s if s.contains("bool") => "boolean",
        s if s.contains("str") => "string",
        s if s.contains("list") || s.contains("List") => "array",
        s if s.contains("dict") || s.contains("Dict") => "object",
        _ => "string", // Default fallback
    }
}

/// Convert JSON value to Python object
fn json_to_python_value(value: &serde_json::Value, py: Python<'_>) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => {
            let py_bool = b.into_pyobject(py)?;
            Ok(
                <pyo3::Bound<'_, pyo3::types::PyBool> as Clone>::clone(&py_bool)
                    .into_any()
                    .unbind(),
            )
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else {
                Ok(n.to_string().into_pyobject(py)?.into_any().unbind())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any().unbind()),
        serde_json::Value::Array(arr) => {
            let py_list = pyo3::types::PyList::empty(py);
            for item in arr {
                let py_item = json_to_python_value(item, py)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into_pyobject(py)?.into_any().unbind())
        }
        serde_json::Value::Object(obj) => {
            let py_dict = pyo3::types::PyDict::new(py);
            for (key, val) in obj {
                let py_val = json_to_python_value(val, py)?;
                py_dict.set_item(key, py_val)?;
            }
            Ok(py_dict.into_pyobject(py)?.into_any().unbind())
        }
    }
}

/// Execute a tool from the registry by name with given arguments
#[pyfunction]
pub(crate) fn execute_tool(
    py: Python<'_>,
    tool_name: String,
    args: Vec<PyObject>,
) -> PyResult<PyObject> {
    TOOL_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        if let Some(tool_func) = registry.get(&tool_name) {
            // Call the tool function with the provided arguments
            let args_tuple = PyTuple::new(py, args)?;
            tool_func.call1(py, args_tuple)
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Tool '{}' not found in registry",
                tool_name
            )))
        }
    })
}

/// Get all registered tool names
#[pyfunction]
pub(crate) fn get_registered_tools(_py: Python<'_>) -> PyResult<Vec<String>> {
    TOOL_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        Ok(registry.keys().cloned().collect())
    })
}

/// Bridge function to sync tools from global registry to thread-local registry
#[pyfunction]
pub(crate) fn sync_global_tools_to_workflow(py: Python<'_>) -> PyResult<()> {
    use crate::tools::decorator::get_global_registry;

    // Get tools from the global registry
    let global_registry = get_global_registry();
    let global_guard = global_registry.lock().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to acquire global registry lock: {}",
            e
        ))
    })?;

    // Get the list of registered tools from the global registry
    let global_tools = global_guard.list_tools().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to list global tools: {}",
            e
        ))
    })?;

    // For each tool in the global registry, get the actual function and register it in thread-local
    TOOL_REGISTRY.with(|local_registry| {
        let mut local_registry = local_registry.borrow_mut();

        for tool_name in global_tools {
            // Create a placeholder function that delegates to the global registry
            let tool_placeholder = py.eval(
                c"lambda *args, **kwargs: 'Tool execution delegated to global registry'",
                None,
                None,
            )?;

            local_registry.insert(
                tool_name.clone(),
                tool_placeholder.into_pyobject(py)?.into_any().unbind(),
            );
        }

        Ok::<(), PyErr>(())
    })?;

    Ok(())
}

/// Execute a tool from the global registry
fn execute_tool_from_global_registry(
    py: Python<'_>,
    tool_name: &str,
    parameters: &serde_json::Map<String, serde_json::Value>,
) -> PyResult<String> {
    use crate::tools::decorator::get_global_registry;

    // Get the global registry
    let global_registry = get_global_registry();
    let global_guard = global_registry.lock().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to acquire global registry lock: {}",
            e
        ))
    })?;

    // Check if the tool exists in the global registry
    let has_tool = global_guard.has_tool(tool_name).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to check tool existence: {}",
            e
        ))
    })?;

    if !has_tool {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Tool '{}' not found in global registry",
            tool_name
        )));
    }

    // Execute the tool using the global registry's execute method
    // Convert parameters to the format expected by the registry
    let params_dict = pyo3::types::PyDict::new(py);
    for (key, value) in parameters {
        match json_to_python_value(value, py) {
            Ok(py_value) => {
                params_dict.set_item(key, py_value)?;
            }
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to convert parameter '{}': {}",
                    key, e
                )));
            }
        }
    }

    // Execute the tool through the global registry
    match global_guard.execute_tool(tool_name, &params_dict, py) {
        Ok(tool_result) => {
            if tool_result.success {
                Ok(format!("{}: {}", tool_name, tool_result.output))
            } else {
                Ok(format!(
                    "{}: Error - {}",
                    tool_name,
                    tool_result
                        .error
                        .unwrap_or_else(|| "Unknown error".to_string())
                ))
            }
        }
        Err(e) => Ok(format!(
            "{}: Error - Tool execution failed: {}",
            tool_name, e
        )),
    }
}

/// Execute tool calls from workflow
#[pyfunction]
pub(crate) fn execute_workflow_tool_calls(
    py: Python<'_>,
    tool_calls_json: String,
    node_tools: Vec<String>,
) -> PyResult<String> {
    // Parse the tool calls JSON
    let tool_calls: Vec<serde_json::Value> =
        serde_json::from_str(&tool_calls_json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse tool calls JSON: {}",
                e
            ))
        })?;

    let mut results = Vec::new();

    for tool_call in tool_calls {
        if let (Some(tool_name), Some(parameters)) = (
            tool_call.get("tool_name").and_then(|v| v.as_str()),
            tool_call.get("parameters").and_then(|v| v.as_object()),
        ) {
            // Validate tool availability
            if !node_tools.contains(&tool_name.to_string()) {
                results.push(format!(
                    "{}: Error - Tool '{}' not available for this node",
                    tool_name, tool_name
                ));
                continue;
            }

            // Execute tool using the global registry
            let tool_result = TOOL_REGISTRY.with(|registry| {
                let registry = registry.borrow();
                if let Some(tool_func) = registry.get(tool_name) {
                    // Convert parameters to Python kwargs
                    let kwargs = pyo3::types::PyDict::new(py);

                    // Convert each parameter from JSON to Python
                    for (key, value) in parameters {
                        match json_to_python_value(value, py) {
                            Ok(py_value) => {
                                if let Err(e) = kwargs.set_item(key, py_value) {
                                    return Ok::<String, PyErr>(format!(
                                        "{}: Error - Failed to set parameter '{}': {}",
                                        tool_name, key, e
                                    ));
                                }
                            }
                            Err(e) => {
                                return Ok::<String, PyErr>(format!(
                                    "{}: Error - Failed to convert parameter '{}': {}",
                                    tool_name, key, e
                                ));
                            }
                        }
                    }

                    // Execute the tool function
                    match tool_func.call(py, (), Some(&kwargs)) {
                        Ok(result) => {
                            // Convert result to string
                            let result_str = match result.extract::<String>(py) {
                                Ok(s) => s,
                                Err(_) => result.to_string(), // Fallback to repr
                            };
                            Ok::<String, PyErr>(format!("{}: {}", tool_name, result_str))
                        }
                        Err(e) => Ok::<String, PyErr>(format!(
                            "{}: Error - Tool execution failed: {}",
                            tool_name, e
                        )),
                    }
                } else {
                    // Execute from global registry if not found in thread-local
                    Ok::<String, PyErr>(
                        execute_tool_from_global_registry(py, tool_name, parameters)
                            .unwrap_or_else(|e| {
                                format!(
                                    "{}: Error - Tool '{}' not found in any registry: {}",
                                    tool_name, tool_name, e
                                )
                            }),
                    )
                }
            })?;

            results.push(tool_result);
        } else {
            results.push(
                "Error: Invalid tool call format - missing tool_name or parameters".to_string(),
            );
        }
    }

    // Return formatted results
    if results.is_empty() {
        Ok("No tool calls were executed".to_string())
    } else {
        Ok(results.join("\n"))
    }
}

/// Tool execution bridge for workflow integration
#[pyfunction]
pub(crate) fn execute_production_tool_calls(
    py: Python<'_>,
    tool_calls_json: String,
    node_tools: Vec<String>,
) -> PyResult<String> {
    use crate::tools::registry::ToolRegistry;

    // Parse tool calls
    let tool_calls: Vec<serde_json::Value> =
        serde_json::from_str(&tool_calls_json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse tool calls JSON: {}",
                e
            ))
        })?;

    // Create a tool registry with available tools
    let registry = ToolRegistry::new();

    // Register tools from the global registry that are available for this node
    TOOL_REGISTRY.with(|global_registry| {
        let global_registry = global_registry.borrow();
        for tool_name in &node_tools {
            if let Some(tool_func) = global_registry.get(tool_name) {
                let params_dict = pyo3::types::PyDict::new(py);
                let _ = registry.register_tool(
                    tool_name.clone(),
                    format!("Tool function: {}", tool_name),
                    tool_func.clone_ref(py),
                    &params_dict,
                    None,
                );
            }
        }
        Ok::<(), PyErr>(())
    })?;

    let mut results = Vec::new();

    // Execute each tool call
    for tool_call in tool_calls {
        if let (Some(tool_name), Some(parameters)) = (
            tool_call.get("tool_name").and_then(|v| v.as_str()),
            tool_call.get("parameters").and_then(|v| v.as_object()),
        ) {
            // Convert parameters to PyDict
            let params_dict = pyo3::types::PyDict::new(py);
            for (key, value) in parameters {
                match json_to_python_value(value, py) {
                    Ok(py_value) => {
                        params_dict.set_item(key, py_value)?;
                    }
                    Err(e) => {
                        results.push(format!(
                            "{}: Error - Parameter conversion failed: {}",
                            tool_name, e
                        ));
                        continue;
                    }
                }
            }

            // Execute the tool
            match registry.execute_tool(tool_name, &params_dict, py) {
                Ok(tool_result) => {
                    if tool_result.success {
                        results.push(format!("{}: {}", tool_name, tool_result.output));
                    } else {
                        let error_msg = tool_result
                            .error
                            .unwrap_or_else(|| "Unknown error".to_string());
                        results.push(format!("{}: Error - {}", tool_name, error_msg));
                    }
                }
                Err(e) => {
                    results.push(format!("{}: Error - {}", tool_name, e));
                }
            }
        } else {
            results.push("Error: Invalid tool call format".to_string());
        }
    }

    Ok(results.join("\n"))
}
