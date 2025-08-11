"""
GraphBit Tools - Easy Python decorator interface for tool calling

This package provides a simple and powerful decorator interface for registering
Python functions as tools that can be called by LLMs through GraphBit.

Usage:
    from tools import tool
    
    @tool(
        description="Get the current weather for a location",
        parameters={
            "type": "object",
            "properties": {
                "location": {"type": "string", "description": "The city and state, e.g. San Francisco, CA"},
                "unit": {"type": "string", "enum": ["celsius", "fahrenheit"], "description": "Temperature unit"}
            },
            "required": ["location"]
        }
    )
    def get_weather(location: str, unit: str = "fahrenheit") -> dict:
        # Your implementation here
        return {"temperature": 72, "condition": "sunny", "unit": unit}
"""

from .decorators import tool, ToolRegistry
from .schemas import generate_schema, SchemaGenerator
from .utils import validate_tool_function, ToolValidationError

__version__ = "0.1.0"
__author__ = "GraphBit Team"

__all__ = [
    "tool",
    "ToolRegistry", 
    "generate_schema",
    "SchemaGenerator",
    "validate_tool_function",
    "ToolValidationError",
]