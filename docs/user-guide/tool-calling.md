# Tool Calling

GraphBit provides a powerful and flexible tool calling system that allows Large Language Models (LLMs) to execute Python functions dynamically. This enables LLMs to interact with external systems, perform calculations, access databases, make API calls, and much more.

## Overview

Tool calling bridges the gap between LLM reasoning and real-world actions. With GraphBit's tool system, you can:

- **Register Python functions** as tools that LLMs can discover and call
- **Automatic schema generation** from Python type hints and docstrings  
- **Type-safe execution** with comprehensive validation
- **Performance monitoring** with built-in metrics and statistics
- **Category-based organization** for better tool management
- **Global and instance-level** tool registration patterns

## Quick Start

### 1. Basic Tool Registration

```python
import graphbit
from tools import tool

# Initialize GraphBit
graphbit.init()

# Register a simple tool
@tool(
    description="Get the current weather for a location",
    parameters={
        "type": "object",
        "properties": {
            "location": {"type": "string", "description": "City and state, e.g. San Francisco, CA"},
            "unit": {"type": "string", "enum": ["celsius", "fahrenheit"], "description": "Temperature unit"}
        },
        "required": ["location"]
    },
    category="weather"
)
def get_weather(location: str, unit: str = "fahrenheit") -> dict:
    """Get current weather information for a location."""
    # Your implementation here
    return {
        "location": location,
        "temperature": 72,
        "condition": "sunny",
        "unit": unit,
        "humidity": 65
    }
```

### 2. Using Tools with LLMs

```python
# Create LLM client
config = graphbit.LlmConfig.openai(api_key="your-api-key")
client = graphbit.LlmClient(config)

# The LLM can now use your registered tools
response = client.complete(
    prompt="What's the weather like in San Francisco?",
    max_tokens=150,
    tools=graphbit.get_tool_definitions()  # Provide available tools
)

print(response)
```

## Core Concepts

### Tool Definition

A tool in GraphBit consists of:

- **Name**: Unique identifier for the tool
- **Description**: Human-readable explanation of what the tool does
- **Parameters**: JSON schema defining the expected input parameters
- **Function**: Python callable that implements the tool logic
- **Metadata**: Category, version, and enabled status

### Tool Execution Flow

1. **Registration**: Tools are registered either globally or with specific managers
2. **Discovery**: LLMs receive tool definitions during conversation
3. **Invocation**: LLM decides to call a tool with specific parameters
4. **Validation**: Parameters are validated against the tool's schema
5. **Execution**: The Python function is called with validated parameters
6. **Response**: Results are returned to the LLM for further processing

## Registration Methods

### Method 1: Decorator (Recommended)

The `@tool` decorator provides the simplest and most feature-rich way to register tools:

```python
from tools import tool

@tool(
    description="Calculate the area of a rectangle",
    auto_schema=True,  # Generate schema from type hints
    category="math",
    version="1.0.0"
)
def calculate_area(length: float, width: float) -> dict:
    """
    Calculate the area of a rectangle.
    
    Args:
        length: Length of the rectangle in units
        width: Width of the rectangle in units
        
    Returns:
        Dictionary containing the calculated area
    """
    area = length * width
    return {
        "length": length,
        "width": width,
        "area": area,
        "unit": "square units"
    }
```

### Method 2: Manual Registration

For more control, you can register tools manually:

```python
import graphbit

def multiply_numbers(numbers: list) -> dict:
    """Multiply a list of numbers together."""
    result = 1
    for num in numbers:
        result *= num
    return {"result": result, "input": numbers}

# Register manually
graphbit.register_tool(
    name="multiply_numbers",
    description="Multiply a list of numbers together",
    parameters={
        "type": "object",
        "properties": {
            "numbers": {
                "type": "array",
                "items": {"type": "number"},
                "description": "List of numbers to multiply"
            }
        },
        "required": ["numbers"]
    },
    function=multiply_numbers,
    category="math",
    version="1.0.0"
)
```

### Method 3: Tool Manager Instance

For isolated tool collections:

```python
import graphbit

# Create dedicated tool manager
math_tools = graphbit.ToolManager()

def add_numbers(a: float, b: float) -> float:
    return a + b

math_tools.register_tool(
    name="add_numbers",
    description="Add two numbers together",
    parameters={
        "type": "object",
        "properties": {
            "a": {"type": "number"},
            "b": {"type": "number"}
        },
        "required": ["a", "b"]
    },
    function=add_numbers
)

# Execute with specific manager
result = math_tools.execute_tool("add_numbers", {"a": 5, "b": 3})
```

## Automatic Schema Generation

GraphBit can automatically generate JSON schemas from Python function signatures and type hints, making tool registration much simpler.

### Basic Type Support

```python
from tools import tool

@tool(description="Process user data", auto_schema=True)
def process_user(
    name: str,
    age: int,
    height: float,
    active: bool
) -> dict:
    """Process user information."""
    return {
        "name": name,
        "age": age,
        "height": height,
        "active": active,
        "processed": True
    }

# Automatically generates:
# {
#     "type": "object",
#     "properties": {
#         "name": {"type": "string"},
#         "age": {"type": "integer"},
#         "height": {"type": "number"},
#         "active": {"type": "boolean"}
#     },
#     "required": ["name", "age", "height", "active"]
# }
```

### Advanced Type Support

```python
from typing import List, Dict, Optional, Union, Literal
from tools import tool

@tool(description="Advanced type example", auto_schema=True)
def advanced_function(
    # Collections
    tags: List[str],
    metadata: Dict[str, str],
    
    # Optional types
    description: Optional[str] = None,
    
    # Union types
    priority: Union[int, str] = "normal",
    
    # Literal types (enums)
    status: Literal["draft", "published", "archived"] = "draft"
) -> dict:
    """
    Function with advanced type annotations.
    
    Args:
        tags: List of tag strings
        metadata: Key-value metadata pairs
        description: Optional description text
        priority: Priority level (number or string)
        status: Current status of the item
    """
    return {
        "tags": tags,
        "metadata": metadata,
        "description": description,
        "priority": priority,
        "status": status
    }
```

### Docstring Integration

Parameter descriptions are automatically extracted from docstrings:

```python
@tool(description="Calculate compound interest", auto_schema=True)
def compound_interest(
    principal: float,
    rate: float,
    time: int,
    compounds_per_year: int = 12
) -> dict:
    """
    Calculate compound interest on an investment.
    
    Args:
        principal: Initial amount of money invested
        rate: Annual interest rate as a decimal (e.g., 0.05 for 5%)
        time: Number of years the money is invested
        compounds_per_year: Number of times interest is compounded per year
        
    Returns:
        Dictionary with investment details and final amount
    """
    amount = principal * (1 + rate/compounds_per_year) ** (compounds_per_year * time)
    interest_earned = amount - principal
    
    return {
        "principal": principal,
        "rate": rate,
        "time": time,
        "compounds_per_year": compounds_per_year,
        "final_amount": round(amount, 2),
        "interest_earned": round(interest_earned, 2)
    }
```

## Tool Categories and Organization

Organize your tools using categories for better management:

```python
from tools import tool, get_registry

# Database tools
@tool(description="Execute SQL query", category="database")
def execute_query(sql: str, database: str = "default") -> dict:
    return {"result": "query_executed", "sql": sql, "database": database}

@tool(description="Get table schema", category="database")
def get_schema(table_name: str) -> dict:
    return {"table": table_name, "columns": ["id", "name", "created_at"]}

# File system tools
@tool(description="Read file contents", category="filesystem")
def read_file(file_path: str) -> str:
    with open(file_path, 'r') as f:
        return f.read()

@tool(description="List directory contents", category="filesystem")
def list_directory(path: str = ".") -> list:
    import os
    return os.listdir(path)

# Web tools
@tool(description="Make HTTP GET request", category="web")
def http_get(url: str, headers: dict = None) -> dict:
    import requests
    response = requests.get(url, headers=headers or {})
    return {
        "status_code": response.status_code,
        "content": response.text[:1000],  # Truncate for safety
        "headers": dict(response.headers)
    }

# List tools by category
registry = get_registry()
print("Database tools:", registry.list_tools(category="database"))
print("File system tools:", registry.list_tools(category="filesystem"))
print("Web tools:", registry.list_tools(category="web"))
```

## Error Handling and Validation

### Automatic Validation

Tools are automatically validated during registration:

```python
from tools import tool, ToolValidationError

# This will raise ToolValidationError
@tool(description="Invalid tool")
def invalid_tool(*args, **kwargs):  # Variable arguments not allowed
    pass

# This will also raise an error
@tool(description="Async tool")
async def async_tool(param: str):  # Async functions not yet supported
    return param
```

### Manual Validation

```python
from tools.utils import validate_tool_function, validate_tool_parameters

def my_function(param: str) -> str:
    return param.upper()

try:
    validate_tool_function(my_function)
    print("Function is valid for tool use")
except ToolValidationError as e:
    print(f"Validation failed: {e}")

# Validate parameters schema
parameters = {
    "type": "object",
    "properties": {
        "param": {"type": "string"}
    },
    "required": ["param"]
}

try:
    validate_tool_parameters(parameters)
    print("Parameters schema is valid")
except ToolValidationError as e:
    print(f"Schema validation failed: {e}")
```

### Runtime Error Handling

```python
@tool(description="Tool that might fail")
def risky_operation(value: int) -> dict:
    if value < 0:
        raise ValueError("Value must be positive")
    if value > 1000:
        raise ValueError("Value too large")
    
    return {"result": value * 2, "status": "success"}

# Execute with error handling
try:
    result = graphbit.execute_tool("risky_operation", {"value": -5})
    if result.success:
        print(f"Success: {result.data}")
    else:
        print(f"Tool failed: {result.data}")
except Exception as e:
    print(f"Execution error: {e}")
```

## Performance and Monitoring

### Tool Execution Metrics

```python
from tools.utils import get_tool_metrics

# Execute some tools
graphbit.execute_tool("get_weather", {"location": "San Francisco"})
graphbit.execute_tool("calculate_area", {"length": 10, "width": 5})

# Get performance metrics
metrics = get_tool_metrics()
stats = metrics.get_stats()

print(f"Total tools executed: {stats['total_calls']}")
print(f"Success rate: {stats['total_successes'] / stats['total_calls'] * 100:.1f}%")
print(f"Total failures: {stats['total_failures']}")

# Per-tool statistics
for tool_name, tool_stats in stats['tools'].items():
    print(f"\n{tool_name}:")
    print(f"  Calls: {tool_stats['total_calls']}")
    print(f"  Success rate: {tool_stats['success_rate'] * 100:.1f}%")
    print(f"  Avg execution time: {tool_stats['avg_execution_time']:.2f}ms")
    print(f"  Min/Max time: {tool_stats['min_execution_time']:.2f}ms / {tool_stats['max_execution_time']:.2f}ms")
```

### Tool Manager Statistics

```python
# Get tool execution statistics
executor = graphbit.Executor(llm_config)
result = executor.execute(workflow)

# Check execution stats
stats = executor.get_stats()
print(f"Total executions: {stats['total_executions']}")
print(f"Successful executions: {stats['successful_executions']}")
print(f"Average duration: {stats['average_duration_ms']}ms")
```

### Performance Best Practices

```python
@tool(description="Optimized tool example")
def optimized_tool(data: list) -> dict:
    """Example of performance-optimized tool."""
    # 1. Validate input early
    if not data:
        return {"error": "Empty data provided"}
    
    # 2. Limit processing for large inputs
    if len(data) > 1000:
        data = data[:1000]  # Truncate large inputs
    
    # 3. Use efficient algorithms
    result = sum(data)  # O(n) operation
    
    # 4. Return structured, JSON-serializable data
    return {
        "sum": result,
        "count": len(data),
        "average": result / len(data) if data else 0,
        "processed_at": "2024-01-01T00:00:00Z"
    }

# 5. Monitor execution time
import time

@tool(description="Timed operation")
def timed_operation(iterations: int = 100) -> dict:
    start_time = time.time()
    
    # Your operation here
    result = sum(range(iterations))
    
    execution_time = time.time() - start_time
    
    return {
        "result": result,
        "iterations": iterations,
        "execution_time_seconds": execution_time
    }
```

## Advanced Use Cases

### Dynamic Tool Loading

```python
import importlib
from tools import tool

def load_tools_from_module(module_name: str):
    """Dynamically load tools from a Python module."""
    try:
        module = importlib.import_module(module_name)
        
        # Find all functions decorated with @tool
        for attr_name in dir(module):
            attr = getattr(module, attr_name)
            if hasattr(attr, '_tool_name'):
                print(f"Found tool: {attr._tool_name}")
                
    except ImportError as e:
        print(f"Failed to load module {module_name}: {e}")

# Load tools from external modules
load_tools_from_module("my_custom_tools")
```

### Conditional Tool Registration

```python
import os
from tools import tool

# Only register database tools if DB is available
if os.getenv("DATABASE_URL"):
    @tool(description="Database query tool", category="database")
    def query_database(sql: str) -> dict:
        # Database implementation
        return {"result": "database_query_result"}

# Only register API tools if API key is available
if os.getenv("API_KEY"):
    @tool(description="External API call", category="api")
    def call_external_api(endpoint: str, params: dict = None) -> dict:
        # API implementation
        return {"response": "api_response"}
```

### Tool Composition

```python
@tool(description="Fetch and process data", category="data")
def fetch_and_process(url: str, process_type: str) -> dict:
    """Compose multiple operations into a single tool."""
    
    # Step 1: Fetch data
    fetch_result = graphbit.execute_tool("http_get", {"url": url})
    
    if not fetch_result.success:
        return {"error": "Failed to fetch data", "details": fetch_result.data}
    
    # Step 2: Process data based on type
    if process_type == "json":
        process_result = graphbit.execute_tool("parse_json", {"text": fetch_result.data["content"]})
    elif process_type == "csv":
        process_result = graphbit.execute_tool("parse_csv", {"text": fetch_result.data["content"]})
    else:
        return {"error": f"Unknown process type: {process_type}"}
    
    # Step 3: Return combined result
    return {
        "url": url,
        "process_type": process_type,
        "fetch_status": fetch_result.data["status_code"],
        "processed_data": process_result.data if process_result.success else None,
        "success": process_result.success
    }
```

## Integration Patterns

### With Workflow Builder

```python
import graphbit
from tools import tool

# Register tools
@tool(description="Data validation tool")
def validate_data(data: dict) -> dict:
    return {"valid": True, "data": data}

@tool(description="Data transformation tool")
def transform_data(data: dict, format: str) -> dict:
    return {"transformed": data, "format": format}

# Create workflow that uses tools
workflow = graphbit.Workflow()
workflow.add_node("validator", "tool", tool_name="validate_data")
workflow.add_node("transformer", "tool", tool_name="transform_data")
workflow.add_edge("validator", "transformer")

# Execute workflow
executor = graphbit.Executor(llm_config)
result = executor.execute(workflow, {"data": {"key": "value"}})
```

### With Agents

```python
import graphbit
from tools import tool

# Register agent tools
@tool(description="Research information online", category="research")
def research_topic(topic: str, depth: str = "basic") -> dict:
    # Research implementation
    return {"topic": topic, "findings": ["fact1", "fact2"], "depth": depth}

@tool(description="Generate report from research", category="content")
def generate_report(research_data: dict, format: str = "markdown") -> str:
    # Report generation implementation
    return f"# Report on {research_data['topic']}\n\nFindings: {', '.join(research_data['findings'])}"

# Create agent with access to tools
agent = graphbit.Agent(
    name="research_agent",
    llm_config=llm_config,
    tools=graphbit.get_tool_definitions()
)

# Agent can now use tools automatically
response = agent.process("Research the latest trends in AI and generate a brief report")
```

## API Reference

### Core Functions

#### `@tool(description, parameters=None, name=None, category="general", version="1.0.0", enabled=True, auto_schema=True, auto_register=True)`

Decorator to register a Python function as a tool.

**Parameters:**
- `description` (str): Human-readable description of the tool
- `parameters` (dict, optional): JSON schema for parameters (auto-generated if None)
- `name` (str, optional): Tool name (defaults to function name)
- `category` (str): Tool category for organization
- `version` (str): Tool version
- `enabled` (bool): Whether the tool is enabled
- `auto_schema` (bool): Whether to auto-generate schema from type hints
- `auto_register` (bool): Whether to auto-register with GraphBit

#### `graphbit.register_tool(name, description, parameters, function, category=None, version=None, enabled=None)`

Manually register a tool function.

#### `graphbit.execute_tool(tool_name, parameters)`

Execute a globally registered tool.

**Returns:** `ToolResult` object with `success`, `data`, and `execution_time_ms` properties.

#### `graphbit.get_tool_definitions()`

Get all tool definitions in LLM-compatible format.

**Returns:** List of tool definition dictionaries.

### Tool Management

#### `get_registry()`

Get the global tool registry for advanced operations.

#### `list_tools(category=None)`

List all registered tools, optionally filtered by category.

#### `enable_tool(name)` / `disable_tool(name)`

Enable or disable a tool by name.

#### `clear_tools()`

Clear all registered tools.

### Validation

#### `validate_tool_function(func)`

Validate that a function is suitable for tool use.

#### `validate_tool_parameters(parameters)`

Validate a tool parameters schema.

### Schema Generation

#### `generate_schema(func)`

Generate JSON schema from function signature and type hints.

#### `set_custom_type_mapping(type_class, json_type)`

Add custom type mapping for schema generation.

### Metrics

#### `get_tool_metrics()`

Get the global tool metrics instance for monitoring tool usage.

## Best Practices

### Tool Design

1. **Single Responsibility**: Each tool should have one clear purpose
2. **Descriptive Names**: Use clear, descriptive function and parameter names
3. **Type Annotations**: Always use type hints for automatic schema generation
4. **Documentation**: Include comprehensive docstrings with parameter descriptions
5. **Error Handling**: Implement proper error handling and validation
6. **JSON Serializable**: Ensure return values are JSON serializable

### Performance

1. **Lightweight Functions**: Keep tool functions fast and lightweight
2. **Input Validation**: Validate inputs early to avoid expensive operations
3. **Resource Limits**: Set reasonable limits on processing time and memory
4. **Caching**: Use caching for expensive or frequently called operations
5. **Monitoring**: Monitor tool execution metrics in production

### Security

1. **Input Sanitization**: Always sanitize and validate user inputs
2. **Least Privilege**: Tools should only have necessary permissions
3. **Rate Limiting**: Implement rate limiting for resource-intensive tools
4. **Audit Logging**: Log tool executions for security monitoring
5. **Sandboxing**: Consider sandboxing for tools that execute external code

### Organization

1. **Meaningful Categories**: Use descriptive categories to organize tools
2. **Semantic Versioning**: Use semantic versioning for tool versions
3. **Consistent Naming**: Follow consistent naming conventions
4. **Module Organization**: Organize related tools in separate modules
5. **Documentation**: Maintain up-to-date documentation for all tools

## Troubleshooting

### Common Issues

#### Schema Generation Failures

```python
# Problem: Complex types not supported
from typing import Any

@tool(description="Tool with Any type", auto_schema=True)
def problematic_tool(data: Any) -> dict:  # Any type can't be auto-converted
    return {"data": data}

# Solution: Provide explicit schema
@tool(
    description="Tool with explicit schema",
    parameters={
        "type": "object",
        "properties": {
            "data": {"description": "Input data of any type"}
        },
        "required": ["data"]
    }
)
def fixed_tool(data) -> dict:
    return {"data": data}
```

#### Tool Registration Errors

```python
# Problem: Function with unsupported signature
@tool(description="Invalid function")
def invalid_function(*args, **kwargs):  # Variable arguments not supported
    pass

# Solution: Use explicit parameters
@tool(description="Valid function")
def valid_function(param1: str, param2: int = 0) -> dict:
    return {"param1": param1, "param2": param2}
```

#### Import Errors

```python
# Problem: Missing GraphBit import
from tools import tool  # This works

@tool(description="My tool")
def my_tool():
    import graphbit  # This fails if not initialized
    return graphbit.get_tool_definitions()

# Solution: Initialize GraphBit first
import graphbit
graphbit.init()

from tools import tool

@tool(description="My tool")
def my_tool():
    return graphbit.get_tool_definitions()
```

### Debug Mode

Enable debug mode for detailed error information:

```python
import graphbit

# Enable debug mode
graphbit.init(debug=True, log_level="debug")

# Tool errors will now include detailed stack traces
result = graphbit.execute_tool("problematic_tool", {"param": "value"})
```

### Testing Tools

```python
import pytest
from tools import tool

@tool(description="Test tool")
def add_numbers(a: int, b: int) -> int:
    return a + b

def test_tool_registration():
    """Test that tool is properly registered."""
    from tools import get_registry
    registry = get_registry()
    assert "add_numbers" in registry.list_tools()

def test_tool_execution():
    """Test tool execution."""
    result = graphbit.execute_tool("add_numbers", {"a": 2, "b": 3})
    assert result.success
    assert result.data == 5

def test_tool_validation():
    """Test tool parameter validation."""
    # This should fail with invalid parameters
    result = graphbit.execute_tool("add_numbers", {"a": "not_a_number", "b": 3})
    assert not result.success
```

This comprehensive guide covers all aspects of GraphBit's tool calling system, from basic usage to advanced patterns and troubleshooting. The tool calling feature enables powerful LLM-driven applications that can interact with the real world through Python functions.
