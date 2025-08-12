"""
Utility functions for GraphBit tool calling

This module provides validation, error handling, and other utility functions
for the tool calling system.
"""

import inspect
import logging
from typing import Any, Callable, Dict, List, Optional

logger = logging.getLogger(__name__)


class ToolValidationError(Exception):
    """Exception raised when tool validation fails."""
    pass


def validate_tool_function(func: Callable) -> None:
    """
    Validate that a function is suitable for use as a tool.

    Args:
        func: The function to validate

    Raises:
        ToolValidationError: If the function is not suitable for tool use
    """
    if not callable(func):
        raise ToolValidationError("Tool must be callable")

    # Check if function has reasonable signature
    try:
        signature = inspect.signature(func)
    except (ValueError, TypeError) as e:
        raise ToolValidationError(f"Cannot inspect function signature: {e}")

    # Check for *args - not supported in tool calling
    for param in signature.parameters.values():
        if param.kind == inspect.Parameter.VAR_POSITIONAL:
            raise ToolValidationError("Tools cannot use *args parameters")

        if param.kind == inspect.Parameter.VAR_KEYWORD:
            raise ToolValidationError("Tools cannot use **kwargs parameters")

    # Check that function is not a generator
    if inspect.isgeneratorfunction(func):
        raise ToolValidationError("Generator functions are not supported as tools")

    # Check that function is not async (for now)
    if inspect.iscoroutinefunction(func):
        raise ToolValidationError("Async functions are not yet supported as tools")

    logger.debug(f"Function '{func.__name__}' passed validation")


def validate_tool_parameters(parameters: Dict[str, Any]) -> None:
    """
    Validate tool parameters schema.

    Args:
        parameters: JSON schema for tool parameters

    Raises:
        ToolValidationError: If the schema is invalid
    """
    if not isinstance(parameters, dict):
        raise ToolValidationError("Tool parameters must be a dictionary")

    if "type" not in parameters:
        raise ToolValidationError("Tool parameters schema must have a 'type' field")

    if parameters["type"] != "object":
        raise ToolValidationError("Tool parameters schema must be of type 'object'")

    # Validate properties if present
    if "properties" in parameters:
        if not isinstance(parameters["properties"], dict):
            raise ToolValidationError("Tool parameters 'properties' must be a dictionary")

    # Validate required fields if present
    if "required" in parameters:
        if not isinstance(parameters["required"], list):
            raise ToolValidationError("Tool parameters 'required' must be a list")

        properties = parameters.get("properties", {})
        for required_field in parameters["required"]:
            if required_field not in properties:
                raise ToolValidationError(
                    f"Required field '{required_field}' not found in properties"
                )

    logger.debug("Tool parameters schema passed validation")


def safe_json_serialize(obj: Any) -> Any:
    """
    Safely serialize an object to JSON-compatible format.

    Args:
        obj: Object to serialize

    Returns:
        JSON-serializable representation of the object
    """
    if obj is None:
        return None

    if isinstance(obj, (str, int, float, bool)):
        return obj

    if isinstance(obj, (list, tuple)):
        return [safe_json_serialize(item) for item in obj]

    if isinstance(obj, dict):
        return {str(k): safe_json_serialize(v) for k, v in obj.items()}

    # For other types, convert to string
    try:
        return str(obj)
    except Exception:
        return "<non-serializable object>"


def extract_function_info(func: Callable) -> Dict[str, Any]:
    """
    Extract comprehensive information about a function.

    Args:
        func: Function to analyze

    Returns:
        Dictionary with function information
    """
    info = {
        "name": func.__name__,
        "module": getattr(func, "__module__", None),
        "qualname": getattr(func, "__qualname__", None),
        "doc": inspect.getdoc(func),
        "file": None,
        "line": None,
        "signature": None,
        "parameters": {},
        "annotations": {},
    }

    try:
        info["file"] = inspect.getfile(func)
    except (OSError, TypeError):
        pass

    try:
        lines, line_number = inspect.getsourcelines(func)
        info["line"] = line_number
    except (OSError, TypeError):
        pass

    try:
        signature = inspect.signature(func)
        info["signature"] = str(signature)

        for param_name, param in signature.parameters.items():
            info["parameters"][param_name] = {
                "kind": param.kind.name,
                "default": param.default if param.default is not inspect.Parameter.empty else None,
                "annotation": param.annotation if param.annotation is not inspect.Parameter.empty else None,
            }
    except (ValueError, TypeError):
        pass

    # Extract type annotations
    try:
        annotations = getattr(func, "__annotations__", {})
        info["annotations"] = {k: str(v) for k, v in annotations.items()}
    except Exception:
        pass

    return info


def format_tool_signature(func: Callable) -> str:
    """
    Format a function signature for display.

    Args:
        func: Function to format

    Returns:
        Formatted signature string
    """
    try:
        signature = inspect.signature(func)
        return f"{func.__name__}{signature}"
    except Exception:
        return f"{func.__name__}(...)"


def validate_tool_result(result: Any) -> Any:
    """
    Validate and normalize a tool execution result.

    Args:
        result: Tool execution result

    Returns:
        Validated and normalized result

    Raises:
        ToolValidationError: If the result is not valid
    """
    # Ensure result is JSON serializable
    try:
        normalized = safe_json_serialize(result)
        return normalized
    except Exception as e:
        raise ToolValidationError(f"Tool result is not JSON serializable: {e}")


class ToolMetrics:
    """
    Simple metrics collection for tool usage.
    """

    def __init__(self):
        self.call_counts: Dict[str, int] = {}
        self.success_counts: Dict[str, int] = {}
        self.failure_counts: Dict[str, int] = {}
        self.execution_times: Dict[str, List[float]] = {}

    def record_call(self, tool_name: str, success: bool, execution_time: float) -> None:
        """Record a tool call."""
        self.call_counts[tool_name] = self.call_counts.get(tool_name, 0) + 1

        if success:
            self.success_counts[tool_name] = self.success_counts.get(tool_name, 0) + 1
        else:
            self.failure_counts[tool_name] = self.failure_counts.get(tool_name, 0) + 1

        if tool_name not in self.execution_times:
            self.execution_times[tool_name] = []
        self.execution_times[tool_name].append(execution_time)

    def get_stats(self, tool_name: Optional[str] = None) -> Dict[str, Any]:
        """Get statistics for a tool or all tools."""
        if tool_name:
            return self._get_tool_stats(tool_name)
        else:
            return self._get_all_stats()

    def _get_tool_stats(self, tool_name: str) -> Dict[str, Any]:
        """Get statistics for a specific tool."""
        total_calls = self.call_counts.get(tool_name, 0)
        successes = self.success_counts.get(tool_name, 0)
        failures = self.failure_counts.get(tool_name, 0)
        times = self.execution_times.get(tool_name, [])

        return {
            "tool_name": tool_name,
            "total_calls": total_calls,
            "successes": successes,
            "failures": failures,
            "success_rate": successes / total_calls if total_calls > 0 else 0.0,
            "avg_execution_time": sum(times) / len(times) if times else 0.0,
            "min_execution_time": min(times) if times else 0.0,
            "max_execution_time": max(times) if times else 0.0,
        }

    def _get_all_stats(self) -> Dict[str, Any]:
        """Get statistics for all tools."""
        all_tools = set(self.call_counts.keys())

        return {
            "total_tools": len(all_tools),
            "total_calls": sum(self.call_counts.values()),
            "total_successes": sum(self.success_counts.values()),
            "total_failures": sum(self.failure_counts.values()),
            "tools": {tool: self._get_tool_stats(tool) for tool in all_tools},
        }

    def reset(self) -> None:
        """Reset all metrics."""
        self.call_counts.clear()
        self.success_counts.clear()
        self.failure_counts.clear()
        self.execution_times.clear()


# Global metrics instance
_global_metrics = ToolMetrics()


def get_tool_metrics() -> ToolMetrics:
    """Get the global tool metrics instance."""
    return _global_metrics
