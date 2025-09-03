//! Tool decorator for GraphBit Python bindings
//!
//! This module provides the @tool decorator that converts Python functions into
//! executable tool calls for node agents with comprehensive introspection and validation.

use crate::tools::registry::ToolRegistry;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::{Arc, Mutex, OnceLock};

/// Global tool registry instance
static GLOBAL_REGISTRY: OnceLock<Arc<Mutex<ToolRegistry>>> = OnceLock::new();

/// Get or create the global tool registry
pub fn get_global_registry() -> Arc<Mutex<ToolRegistry>> {
    GLOBAL_REGISTRY
        .get_or_init(|| Arc::new(Mutex::new(ToolRegistry::new())))
        .clone()
}

/// Tool decorator for converting Python functions to executable tools
#[pyclass]
pub struct ToolDecorator {
    registry: Arc<Mutex<ToolRegistry>>,
}

#[pymethods]
impl ToolDecorator {
    /// Create a new tool decorator
    #[new]
    #[pyo3(signature = (registry=None))]
    pub fn new(registry: Option<&ToolRegistry>) -> Self {
        let reg = if let Some(_r) = registry {
            // Clone the provided registry (this is a simplified approach)
            Arc::new(Mutex::new(ToolRegistry::new()))
        } else {
            get_global_registry()
        };

        Self { registry: reg }
    }

    /// Decorator function to convert a Python function to a tool
    #[pyo3(signature = (description=None, name=None, return_type=None))]
    pub fn __call__(
        &self,
        description: Option<String>,
        name: Option<String>,
        return_type: Option<String>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let desc = description.unwrap_or_else(|| "Tool function".to_string());
        let ret_type = return_type;

        // Create a simple decorator that registers the function
        // We'll use a Python lambda that calls our registration function
        let globals = py.import("builtins")?.dict();
        let locals = PyDict::new(py);

        // Add our registration function to locals
        locals.set_item("register_func", py.get_type::<ToolRegistry>())?;
        locals.set_item("desc", desc)?;
        locals.set_item("name_opt", name)?;
        locals.set_item("ret_type", ret_type)?;

        // Create a simple decorator function
        let decorator = py.eval(
            c"lambda func: func", // Simple identity function for now
            Some(&globals),
            Some(&locals),
        )?;

        Ok(decorator.to_object(py))
    }

    /// Get the tool registry
    pub fn get_registry(&self) -> PyResult<ToolRegistry> {
        let _registry_guard = self.registry.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire registry lock: {}",
                e
            ))
        })?;

        // Return a new instance (simplified for now)
        Ok(ToolRegistry::new())
    }

    /// Register a function as a tool manually
    pub fn register(
        &self,
        func: PyObject,
        name: Option<String>,
        description: Option<String>,
        return_type: Option<String>,
        py: Python<'_>,
    ) -> PyResult<()> {
        let func_name = name.unwrap_or_else(|| {
            func.bind(py)
                .getattr("__name__")
                .and_then(|n| n.extract::<String>())
                .unwrap_or_else(|_| "unknown_tool".to_string())
        });

        let desc = description.unwrap_or_else(|| "Manually registered tool".to_string());

        // Extract function schema
        let func_bound = func.bind(py);
        let params_schema = extract_function_schema(&func_bound, py)?;
        drop(func_bound); // Release the borrow

        let registry_guard = self.registry.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire registry lock: {}",
                e
            ))
        })?;

        registry_guard.register_tool(
            func_name,
            desc,
            func.clone_ref(py),
            &params_schema,
            return_type,
        )?;

        Ok(())
    }

    /// List all registered tools
    pub fn list_tools(&self) -> PyResult<Vec<String>> {
        let registry_guard = self.registry.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire registry lock: {}",
                e
            ))
        })?;

        registry_guard.list_tools()
    }

    /// Get tool metadata
    pub fn get_tool_info(&self, name: &str) -> PyResult<Option<String>> {
        let registry_guard = self.registry.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire registry lock: {}",
                e
            ))
        })?;

        registry_guard.get_tool_metadata(name)
    }

    /// String representation
    pub fn __repr__(&self) -> String {
        "ToolDecorator(registry=<ToolRegistry>)".to_string()
    }
}

impl Default for ToolDecorator {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Extract function parameter schema using introspection
fn extract_function_schema<'a>(
    func: &'a Bound<'a, PyAny>,
    py: Python<'a>,
) -> PyResult<Bound<'a, PyDict>> {
    let inspect = py.import("inspect")?;
    let signature = inspect.call_method1("signature", (func,))?;
    let parameters = signature.getattr("parameters")?;

    let schema = PyDict::new(py);
    schema.set_item("type", "object")?;

    let properties = PyDict::new(py);
    let required = pyo3::types::PyList::empty(py);

    // Iterate through function parameters
    for item in parameters.try_iter()? {
        let (param_name, param) = item?.extract::<(String, PyObject)>()?;
        let param_name_str = param_name;

        // Skip 'self' and 'cls' parameters
        if param_name_str == "self" || param_name_str == "cls" {
            continue;
        }

        let param_info = PyDict::new(py);

        // Get parameter annotation for type information
        let param_bound = param.bind(py);
        let annotation = param_bound.getattr("annotation")?;
        let param_type =
            if annotation.is_none() || annotation.to_string() == "<class 'inspect._empty'>" {
                "string" // Default type
            } else {
                map_python_type_to_json_schema(&annotation.to_string())
            };

        param_info.set_item("type", param_type)?;

        // Check if parameter has a default value
        let default = param_bound.getattr("default")?;
        if default.to_string() != "<class 'inspect._empty'>" {
            param_info.set_item("default", default)?;
        } else {
            // Parameter is required
            required.append(param_name_str.clone())?;
        }

        // Add description placeholder
        param_info.set_item("description", format!("Parameter {}", param_name_str))?;

        properties.set_item(param_name_str, param_info)?;
    }

    schema.set_item("properties", properties)?;
    schema.set_item("required", required)?;

    Ok(schema)
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

/// Convenience function to create a tool decorator
#[pyfunction]
#[pyo3(signature = (description=None, name=None, return_type=None))]
pub fn tool(
    description: Option<String>,
    name: Option<String>,
    return_type: Option<String>,
    py: Python<'_>,
) -> PyResult<PyObject> {
    // Create a simple decorator function that registers tools
    let desc = description.unwrap_or_else(|| "Tool function".to_string());
    let ret_type = return_type.unwrap_or_else(|| "Any".to_string());

    // Create a Python function that will act as the decorator
    let decorator_func = py.eval(
        c"lambda func: func", // For now, just return the function as-is
        None,
        None,
    )?;

    Ok(decorator_func.to_object(py))
}

/// Get the global tool registry
#[pyfunction]
pub fn get_tool_registry() -> PyResult<ToolRegistry> {
    // Create a proxy registry that delegates to the global registry
    Ok(ToolRegistry::new_global_proxy())
}

/// Clear all registered tools
#[pyfunction]
pub fn clear_tools() -> PyResult<()> {
    let registry = get_global_registry();
    let _registry_guard = registry.lock().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to acquire registry lock: {}",
            e
        ))
    })?;

    // Clear would need to be implemented in ToolRegistry
    Ok(())
}
