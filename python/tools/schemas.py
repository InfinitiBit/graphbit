"""
Schema generation utilities for GraphBit tools

This module provides automatic JSON schema generation from Python function
signatures and type hints.
"""

import inspect
import logging
from typing import Any, Callable, Dict, List, Optional, Union, get_type_hints

logger = logging.getLogger(__name__)


class SchemaGenerator:
    """
    Generates JSON schemas from Python function signatures and type hints.

    This class analyzes function signatures, type hints, and docstrings to
    automatically generate OpenAPI-compatible JSON schemas for tool parameters.
    """

    # Type mapping from Python types to JSON schema types
    TYPE_MAPPING = {
        str: "string",
        int: "integer",
        float: "number",
        bool: "boolean",
        list: "array",
        dict: "object",
        List: "array",
        Dict: "object",
        Any: None,  # Will be handled specially
    }

    def __init__(self):
        self.logger = logging.getLogger(f"{__name__}.{self.__class__.__name__}")

    def generate_schema(self, func: Callable) -> Dict[str, Any]:
        """
        Generate a JSON schema from a function signature.

        Args:
            func: The function to analyze

        Returns:
            JSON schema dictionary
        """
        try:
            signature = inspect.signature(func)
            type_hints = get_type_hints(func)
            docstring = inspect.getdoc(func)

            properties = {}
            required = []

            for param_name, param in signature.parameters.items():
                if param_name == "self":
                    continue

                # Get type information
                param_type = type_hints.get(param_name, Any)

                # Generate property schema
                prop_schema = self._type_to_schema(param_type)

                # Add description from docstring if available
                description = self._extract_param_description(docstring, param_name)
                if description:
                    prop_schema["description"] = description

                # Handle default values
                if param.default is not inspect.Parameter.empty:
                    if param.default is not None:
                        prop_schema["default"] = param.default
                else:
                    required.append(param_name)

                properties[param_name] = prop_schema

            schema = {
                "type": "object",
                "properties": properties,
            }

            if required:
                schema["required"] = required

            # Add function description if available
            if docstring:
                # Extract the first line as description
                first_line = docstring.split('\n')[0].strip()
                if first_line:
                    schema["description"] = first_line

            self.logger.debug(f"Generated schema for {func.__name__}: {schema}")
            return schema

        except Exception as e:
            self.logger.error(f"Failed to generate schema for {func.__name__}: {e}")
            # Return minimal schema on error
            return {"type": "object", "properties": {}}

    def _type_to_schema(self, type_hint: Any) -> Dict[str, Any]:
        """Convert a type hint to JSON schema."""

        # Handle None/Optional types
        if type_hint is type(None):
            return {"type": "null"}

        # Handle Union types (including Optional)
        if hasattr(type_hint, "__origin__") and type_hint.__origin__ is Union:
            args = type_hint.__args__
            if len(args) == 2 and type(None) in args:
                # This is Optional[T]
                non_none_type = args[0] if args[1] is type(None) else args[1]
                schema = self._type_to_schema(non_none_type)
                schema["nullable"] = True
                return schema
            else:
                # General Union - use anyOf
                return {
                    "anyOf": [self._type_to_schema(arg) for arg in args]
                }

        # Handle List types
        if hasattr(type_hint, "__origin__") and type_hint.__origin__ is list:
            schema = {"type": "array"}
            if hasattr(type_hint, "__args__") and type_hint.__args__:
                item_type = type_hint.__args__[0]
                schema["items"] = self._type_to_schema(item_type)
            return schema

        # Handle Dict types
        if hasattr(type_hint, "__origin__") and type_hint.__origin__ is dict:
            schema = {"type": "object"}
            if hasattr(type_hint, "__args__") and len(type_hint.__args__) >= 2:
                value_type = type_hint.__args__[1]
                schema["additionalProperties"] = self._type_to_schema(value_type)
            return schema

        # Handle basic types
        if type_hint in self.TYPE_MAPPING:
            json_type = self.TYPE_MAPPING[type_hint]
            if json_type is None:
                # Any type - allow anything
                return {}
            return {"type": json_type}

        # Handle string literals (Literal types)
        if hasattr(type_hint, "__origin__") and str(type_hint.__origin__) == "typing.Literal":
            values = list(type_hint.__args__)
            return {
                "type": "string" if all(isinstance(v, str) for v in values) else "string",
                "enum": values
            }

        # Fallback for unknown types
        self.logger.warning(f"Unknown type hint: {type_hint}, treating as string")
        return {"type": "string"}

    def _extract_param_description(self, docstring: Optional[str], param_name: str) -> Optional[str]:
        """Extract parameter description from docstring."""
        if not docstring:
            return None

        lines = docstring.split('\n')
        in_args_section = False

        for line in lines:
            stripped = line.strip()

            # Look for Args: or Arguments: section
            if stripped.lower() in ['args:', 'arguments:', 'parameters:']:
                in_args_section = True
                continue

            # End of args section
            if in_args_section and stripped and not line.startswith(' ') and not line.startswith('\t'):
                if stripped.endswith(':'):
                    break

            # Look for parameter description
            if in_args_section and stripped:
                if stripped.startswith(f"{param_name}:") or stripped.startswith(f"{param_name} ("):
                    # Extract description after the colon
                    colon_idx = stripped.find(':')
                    if colon_idx > 0:
                        description = stripped[colon_idx + 1:].strip()
                        if description:
                            return description

        return None


# Global schema generator instance
_schema_generator = SchemaGenerator()


def generate_schema(func: Callable) -> Dict[str, Any]:
    """
    Generate a JSON schema from a function signature.

    This is a convenience function that uses the global schema generator.

    Args:
        func: The function to analyze

    Returns:
        JSON schema dictionary

    Example:
        def get_weather(location: str, unit: str = "fahrenheit") -> dict:
            '''Get weather for a location.

            Args:
                location: The city and state, e.g. "San Francisco, CA"
                unit: Temperature unit (celsius or fahrenheit)
            '''
            pass

        schema = generate_schema(get_weather)
        # Returns:
        # {
        #     "type": "object",
        #     "properties": {
        #         "location": {"type": "string", "description": "The city and state..."},
        #         "unit": {"type": "string", "default": "fahrenheit", "description": "Temperature unit..."}
        #     },
        #     "required": ["location"]
        # }
    """
    return _schema_generator.generate_schema(func)


def set_custom_type_mapping(type_class: type, json_type: str) -> None:
    """
    Add a custom type mapping for schema generation.

    Args:
        type_class: Python type class
        json_type: Corresponding JSON schema type
    """
    SchemaGenerator.TYPE_MAPPING[type_class] = json_type
    logger.info(f"Added custom type mapping: {type_class} -> {json_type}")


def get_schema_generator() -> SchemaGenerator:
    """Get the global schema generator instance."""
    return _schema_generator
