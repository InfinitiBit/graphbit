"""
GraphBit Python bindings and tools package.

This package provides Python bindings for GraphBit and additional
tools for building AI workflows with tool calling capabilities.
"""

import logging
from typing import Any, Callable, Dict, Optional

__version__ = "0.1.0"
__author__ = "GraphBit Team"

# Set up logging
logger = logging.getLogger(__name__)

# Global tool storage for GraphBit
_graphbit_tools: Dict[str, Dict[str, Any]] = {}

# Import main GraphBit functionality (this will be available when the Rust bindings are built)
try:
    from ..graphbit import *  # Import from the compiled Rust module
except ImportError:
    # Fallback for development/testing when Rust bindings might not be available
    pass


def register_tool(
    name: str,
    description: str,
    parameters: Dict[str, Any],
    function: Callable,
    category: str = "general",
    version: str = "1.0.0",
    enabled: bool = True,
) -> None:
    """
    Register a tool with GraphBit.

    Args:
        name: Tool name
        description: Tool description
        parameters: JSON schema for tool parameters
        function: Tool function to call
        category: Tool category
        version: Tool version
        enabled: Whether tool is enabled
    """
    tool_metadata = {
        "name": name,
        "description": description,
        "parameters": parameters,
        "function": function,
        "category": category,
        "version": version,
        "enabled": enabled,
    }

    _graphbit_tools[name] = tool_metadata
    logger.info(f"Tool '{name}' registered with GraphBit in category '{category}'")


def get_registered_tools() -> Dict[str, Dict[str, Any]]:
    """Get all registered tools."""
    return _graphbit_tools.copy()


def get_tool(name: str) -> Optional[Dict[str, Any]]:
    """Get a specific tool by name."""
    return _graphbit_tools.get(name)


def call_tool(name: str, params: Dict[str, Any]) -> Any:
    """
    Call a registered tool with parameters.

    Args:
        name: Tool name
        params: Parameters to pass to the tool

    Returns:
        Tool execution result

    Raises:
        ValueError: If tool not found or disabled
    """
    if name not in _graphbit_tools:
        raise ValueError(f"Tool '{name}' not found")

    tool = _graphbit_tools[name]
    if not tool["enabled"]:
        raise ValueError(f"Tool '{name}' is disabled")

    try:
        result = tool["function"](params)
        logger.info(f"Tool '{name}' executed successfully")
        return result
    except Exception as e:
        logger.error(f"Tool '{name}' execution failed: {e}")
        raise
