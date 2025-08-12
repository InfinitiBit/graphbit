"""
Decorator interface for GraphBit tool calling

This module provides the main @tool decorator and ToolRegistry for easy
tool registration and management.
"""

import functools
import inspect
import logging
from typing import Any, Callable, Dict, List, Optional

from .schemas import generate_schema
from .utils import validate_tool_function, ToolValidationError

# Set up logging
logger = logging.getLogger(__name__)


class ToolRegistry:
    """
    Registry for managing tool functions and their metadata.

    This class maintains a global registry of tools and provides methods
    for registration, validation, and retrieval.
    """

    def __init__(self):
        self._tools: Dict[str, Dict[str, Any]] = {}
        self._categories: Dict[str, List[str]] = {}

    def register(
        self,
        name: str,
        func: Callable,
        description: str,
        parameters: Dict[str, Any],
        category: str = "general",
        version: str = "1.0.0",
        enabled: bool = True,
        auto_register: bool = True,
    ) -> None:
        """Register a tool function with metadata."""

        # Validate the function
        try:
            validate_tool_function(func)
        except ToolValidationError as e:
            logger.error(f"Tool validation failed for '{name}': {e}")
            raise

        # Store tool metadata
        try:
            source_file = inspect.getfile(func)
        except (OSError, TypeError):
            source_file = "<unknown>"

        try:
            source_line = inspect.getsourcelines(func)[1]
        except (OSError, TypeError):
            source_line = 0

        tool_metadata = {
            "name": name,
            "function": func,
            "description": description,
            "parameters": parameters,
            "category": category,
            "version": version,
            "enabled": enabled,
            "signature": inspect.signature(func),
            "source_file": source_file,
            "source_line": source_line,
        }

        self._tools[name] = tool_metadata

        # Update category registry
        if category not in self._categories:
            self._categories[category] = []
        if name not in self._categories[category]:
            self._categories[category].append(name)

        logger.info(f"Tool '{name}' registered in category '{category}'")

        # Auto-register with GraphBit if available and requested
        if auto_register:
            self._auto_register_with_graphbit(tool_metadata)

    def _auto_register_with_graphbit(self, tool_metadata: Dict[str, Any]) -> None:
        """Automatically register with GraphBit if available."""
        try:
            import graphbit

            # Create a wrapper function that handles the GraphBit calling convention
            def wrapper(params: Dict[str, Any]) -> Any:
                func = tool_metadata["function"]
                signature = tool_metadata["signature"]

                # Map parameters to function arguments
                bound_args = signature.bind(**params)
                bound_args.apply_defaults()

                # Call the function
                result = func(*bound_args.args, **bound_args.kwargs)

                # Ensure result is JSON serializable
                if not isinstance(result, (dict, list, str, int, float, bool, type(None))):
                    # Convert to string if not already JSON serializable
                    result = str(result)

                return result

            # Register with GraphBit
            graphbit.register_tool(
                name=tool_metadata["name"],
                description=tool_metadata["description"],
                parameters=tool_metadata["parameters"],
                function=wrapper,
                category=tool_metadata["category"],
                version=tool_metadata["version"],
                enabled=tool_metadata["enabled"],
            )

            logger.info(f"Tool '{tool_metadata['name']}' auto-registered with GraphBit")

        except ImportError:
            logger.warning(
                f"GraphBit not available - tool '{tool_metadata['name']}' registered locally only"
            )
        except Exception as e:
            logger.error(f"Failed to auto-register tool '{tool_metadata['name']}' with GraphBit: {e}")

    def unregister(self, name: str) -> bool:
        """Unregister a tool by name."""
        if name in self._tools:
            tool_metadata = self._tools[name]
            category = tool_metadata["category"]

            # Remove from main registry
            del self._tools[name]

            # Remove from category
            if category in self._categories and name in self._categories[category]:
                self._categories[category].remove(name)
                if not self._categories[category]:
                    del self._categories[category]

            logger.info(f"Tool '{name}' unregistered")
            return True
        return False

    def get_tool(self, name: str) -> Optional[Dict[str, Any]]:
        """Get tool metadata by name."""
        return self._tools.get(name)

    def list_tools(self, category: Optional[str] = None) -> List[str]:
        """List all registered tools, optionally filtered by category."""
        if category:
            return self._categories.get(category, [])
        return list(self._tools.keys())

    def list_categories(self) -> List[str]:
        """List all tool categories."""
        return list(self._categories.keys())

    def get_tools_by_category(self, category: str) -> List[Dict[str, Any]]:
        """Get all tools in a specific category."""
        tool_names = self._categories.get(category, [])
        return [self._tools[name] for name in tool_names if name in self._tools]

    def enable_tool(self, name: str) -> bool:
        """Enable a tool."""
        if name in self._tools:
            self._tools[name]["enabled"] = True
            logger.info(f"Tool '{name}' enabled")
            return True
        return False

    def disable_tool(self, name: str) -> bool:
        """Disable a tool."""
        if name in self._tools:
            self._tools[name]["enabled"] = False
            logger.info(f"Tool '{name}' disabled")
            return True
        return False

    def get_tool_definitions(self) -> List[Dict[str, Any]]:
        """Get tool definitions in GraphBit format."""
        definitions = []
        for tool in self._tools.values():
            if tool["enabled"]:
                definitions.append({
                    "name": tool["name"],
                    "description": tool["description"],
                    "parameters": tool["parameters"],
                })
        return definitions

    def clear(self) -> None:
        """Clear all registered tools."""
        self._tools.clear()
        self._categories.clear()
        logger.info("All tools cleared from registry")


# Global tool registry instance
_global_registry = ToolRegistry()


def tool(
    description: str,
    parameters: Optional[Dict[str, Any]] = None,
    name: Optional[str] = None,
    category: str = "general",
    version: str = "1.0.0",
    enabled: bool = True,
    auto_schema: bool = True,
    auto_register: bool = True,
) -> Callable:
    """
    Decorator to register a Python function as a tool for LLM calling.

    Args:
        description: Human-readable description of what the tool does
        parameters: JSON schema for the tool parameters (auto-generated if None)
        name: Tool name (defaults to function name)
        category: Tool category for organization
        version: Tool version
        enabled: Whether the tool is enabled by default
        auto_schema: Whether to auto-generate schema from function signature
        auto_register: Whether to automatically register with GraphBit

    Returns:
        Decorated function with tool metadata attached

    Example:
        @tool(
            description="Get current weather for a location",
            parameters={
                "type": "object",
                "properties": {
                    "location": {"type": "string", "description": "City and state"},
                    "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]}
                },
                "required": ["location"]
            },
            category="weather"
        )
        def get_weather(location: str, unit: str = "fahrenheit") -> dict:
            return {"temperature": 72, "condition": "sunny", "unit": unit}
    """

    def decorator(func: Callable) -> Callable:
        # Determine tool name
        tool_name = name or func.__name__

        # Generate parameters schema if not provided and auto_schema is enabled
        tool_parameters = parameters
        if tool_parameters is None and auto_schema:
            try:
                tool_parameters = generate_schema(func)
                logger.info(f"Auto-generated schema for tool '{tool_name}'")
            except Exception as e:
                logger.warning(f"Failed to auto-generate schema for '{tool_name}': {e}")
                # Fallback to empty object schema
                tool_parameters = {"type": "object", "properties": {}}

        if tool_parameters is None:
            tool_parameters = {"type": "object", "properties": {}}

        # Register the tool
        _global_registry.register(
            name=tool_name,
            func=func,
            description=description,
            parameters=tool_parameters,
            category=category,
            version=version,
            enabled=enabled,
            auto_register=auto_register,
        )

        # Add metadata to the function
        func._tool_name = tool_name
        func._tool_description = description
        func._tool_parameters = tool_parameters
        func._tool_category = category
        func._tool_version = version
        func._tool_enabled = enabled

        # Create a wrapper that preserves the original function
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            return func(*args, **kwargs)

        # Copy tool metadata to wrapper
        wrapper._tool_name = tool_name
        wrapper._tool_description = description
        wrapper._tool_parameters = tool_parameters
        wrapper._tool_category = category
        wrapper._tool_version = version
        wrapper._tool_enabled = enabled

        return wrapper

    return decorator


def get_registry() -> ToolRegistry:
    """Get the global tool registry."""
    return _global_registry


def list_tools(category: Optional[str] = None) -> List[str]:
    """List all registered tools."""
    return _global_registry.list_tools(category)


def get_tool_definitions() -> List[Dict[str, Any]]:
    """Get all tool definitions for LLM use."""
    return _global_registry.get_tool_definitions()


def enable_tool(name: str) -> bool:
    """Enable a tool by name."""
    return _global_registry.enable_tool(name)


def disable_tool(name: str) -> bool:
    """Disable a tool by name."""
    return _global_registry.disable_tool(name)


def clear_tools() -> None:
    """Clear all registered tools."""
    _global_registry.clear()
