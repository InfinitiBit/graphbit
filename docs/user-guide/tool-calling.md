# Tool Calling Guide

This comprehensive guide covers GraphBit's powerful tool calling system, which allows you to create Python functions that can be executed by LLM agents within workflows.


## Overview

GraphBit's tool calling system enables LLM agents to execute Python functions during workflow execution. This allows agents to:

- Perform calculations and data processing
- Access external APIs and services
- Interact with databases and file systems
- Execute custom business logic
- Chain multiple operations together

### Key Components

- **@tool decorator**: Converts Python functions into callable tools
- **ToolRegistry**: Manages registered tools and metadata
- **ToolExecutor**: Executes tools with configuration and error handling
- **ToolResult**: Contains execution results and metadata
- **ExecutorConfig**: Configures tool execution behavior

## Quick Start

Here's a simple example to get you started with tool calling:

```python
import os
from graphbit import LlmConfig, Executor, Workflow, Node, tool

# Configure LLM
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
executor = Executor(config)

# Create tools with the @tool decorator
@tool(description="Get current weather information for any city")
def get_weather(location: str) -> dict:
    """Get weather information for a specific location."""
    return {
        "location": location,
        "temperature": 22,
        "condition": "sunny",
        "humidity": 65
    }

@tool(description="Perform mathematical calculations and return results")
def calculate(expression: str) -> str:
    """Perform mathematical calculations safely."""
    try:
        result = eval(expression)  # Note: Use safe evaluation in production
        return f"Result: {result}"
    except Exception as e:
        return f"Error: {str(e)}"

# Create workflow with tool-enabled agent
workflow = Workflow("Tool Calling Example")

agent = Node.agent(
    name="Smart Agent",
    prompt="What's the weather in Paris and what is 15 + 27?",
    agent_id="smart_agent_001",
    tools=[get_weather, calculate]  # Provide tools to the agent
)

workflow.add_node(agent)
workflow.validate()

# Execute workflow
result = executor.execute(workflow)

if result.is_success():
    print("Agent Output:", result.get_node_output("Smart Agent"))
else:
    print("Workflow failed:", result.error())
```

## Tool Registration

### Using the @tool Decorator

The `@tool` decorator is the primary way to register functions as tools:

```python
from graphbit import tool

# Basic tool registration
@tool(description="Add two numbers together")
def add_numbers(a: int, b: int) -> int:
    """Add two numbers and return the result."""
    return a + b

# Tool with custom name and return type
@tool(
    name="text_processor",
    description="Process text with various operations",
    return_type="str"
)
def process_text(text: str, operation: str = "uppercase") -> str:
    """Process text with the specified operation."""
    if operation == "uppercase":
        return text.upper()
    elif operation == "lowercase":
        return text.lower()
    elif operation == "reverse":
        return text[::-1]
    else:
        return text
```

### Manual Tool Registration

You can also register tools manually using the ToolRegistry:

```python
from graphbit import ToolRegistry, get_tool_registry

# Get the global tool registry
registry = get_tool_registry()

# Define a function
def multiply_numbers(x: float, y: float) -> float:
    """Multiply two numbers."""
    return x * y

# Register manually
registry.register_tool(
    name="multiply",
    description="Multiply two numbers together",
    function=multiply_numbers,
    parameters_schema={
        "type": "object",
        "properties": {
            "x": {"type": "number", "description": "First number"},
            "y": {"type": "number", "description": "Second number"}
        },
        "required": ["x", "y"]
    },
    return_type="float"
)
```

### Tool Metadata and Schema

GraphBit automatically generates JSON schemas for your tools based on type hints:

```python
from typing import List, Optional, Dict, Any

@tool(description="Search and filter data")
def search_data(
    query: str,
    filters: Optional[Dict[str, Any]] = None,
    limit: int = 10,
    sort_by: str = "relevance"
) -> List[Dict[str, Any]]:
    """Search data with optional filters and sorting."""
    # Implementation here
    results = [
        {"id": 1, "title": "Sample Result", "score": 0.95},
        {"id": 2, "title": "Another Result", "score": 0.87}
    ]
    
    # Apply limit
    return results[:limit]
```

## Tool Execution

### Execution in Workflows

Tools are automatically executed when agents call them during workflow execution:

```python
# Create agent with tools
agent = Node.agent(
    name="Data Analyst",
    prompt="Search for 'machine learning' and process the top 5 results",
    agent_id="analyst_001",
    tools=[search_data, process_text]
)

# The agent will automatically call tools as needed
workflow = Workflow("Data Analysis")
workflow.add_node(agent)
result = executor.execute(workflow)
```

### Manual Tool Execution

You can also execute tools manually for testing or direct use:

```python
from graphbit import ToolExecutor, ExecutorConfig

# Create tool executor with configuration
config = ExecutorConfig(
    max_execution_time_ms=30000,  # 30 seconds timeout
    max_tool_calls=10,            # Maximum 10 sequential calls
    continue_on_error=False,      # Stop on first error
    store_results=True,           # Store execution results
    enable_logging=True           # Enable detailed logging
)

executor = ToolExecutor(config=config)

# Execute tools manually
tool_calls = [
    {
        "name": "add_numbers",
        "parameters": {"a": 10, "b": 20}
    },
    {
        "name": "process_text",
        "parameters": {"text": "hello world", "operation": "uppercase"}
    }
]

try:
    results = executor.execute_tools(tool_calls)
    for result in results.get_all():
        print(f"Tool: {result.tool_name}")
        print(f"Success: {result.success}")
        print(f"Output: {result.output}")
        print(f"Duration: {result.duration_ms}ms")
except Exception as e:
    print(f"Manual execution failed: {e}")
```

## Configuration

### ExecutorConfig Parameters

Configure tool execution behavior with ExecutorConfig:

```python
from graphbit import ExecutorConfig

# Development configuration
dev_config = ExecutorConfig(
    max_execution_time_ms=10000,  # 10 seconds per tool
    max_tool_calls=5,             # Limit tool calls
    continue_on_error=True,       # Continue on errors for debugging
    store_results=True,           # Store all results
    enable_logging=True           # Verbose logging
)

# Production configuration
prod_config = ExecutorConfig.production()  # Optimized defaults

# Custom configuration
custom_config = ExecutorConfig(
    max_execution_time_ms=60000,  # 1 minute timeout
    max_tool_calls=20,            # Allow more tool calls
    continue_on_error=False,      # Fail fast in production
    store_results=True,           # Keep results for analysis
    enable_logging=False          # Minimal logging for performance
)
```

### Tool Registry Management

Manage your tool registry:

```python
from graphbit import get_tool_registry, clear_tools

# Register a test tool first
@tool(description="Test tool for registry")
def test_tool() -> str:
    return "test"

# Get registry
registry = get_tool_registry()

# List all registered tools
tools = registry.list_tools()
print("Registered tools:", [tool.name for tool in tools])

# Get tool metadata
for tool in tools:
    print(f"Tool: {tool.name}")
    print(f"Description: {tool.description}")
    print(f"Parameters: {tool.parameters_schema}")
    print(f"Call count: {tool.call_count}")

# Clear all tools (useful for testing)
clear_tools()
```


### Async Tool Support

While tools themselves are synchronous, you can handle async operations:

```python
import asyncio
from concurrent.futures import ThreadPoolExecutor

@tool(description="Perform async operation synchronously")
def async_operation(url: str, timeout: int = 30) -> Dict[str, Any]:
    """Perform an async operation in a sync context."""
    import aiohttp

    async def fetch_async():
        async with aiohttp.ClientSession() as session:
            async with session.get(url, timeout=timeout) as response:
                return {
                    "status": response.status,
                    "data": await response.text(),
                    "headers": dict(response.headers)
                }

    # Run async operation in sync context
    try:
        loop = asyncio.get_event_loop()
    except RuntimeError:
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)

    return loop.run_until_complete(fetch_async())
```


This comprehensive guide covers various aspects of GraphBit's tool calling system. Use it as a reference for building powerful, tool-enabled workflows that can interact with external systems, process data, and perform complex operations through LLM agents.
