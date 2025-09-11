# Async vs Sync Usage in GraphBit Python API

GraphBit's Python bindings provide both synchronous (blocking) and asynchronous (non-blocking) interfaces for key operations, allowing you to choose the best fit for your application—whether it's a quick script, a web server, or a data pipeline.

---

## Overview

- **Synchronous (Sync) functions** block the current thread until the operation completes. They are simple to use in scripts and REPLs.
- **Asynchronous (Async) functions** return immediately and run in the background, allowing your program to do other work or handle many tasks concurrently. Async is ideal for web servers, pipelines, and high-throughput applications.

---

## Supported Sync and Async Functions

### Module 

| Function      | Type |
|---------------|------|
| [`init`](../api-reference/python-api.md#initlog_levelnone-enable_tracingnone-debugnone)              | Sync         |
| [`version`](../api-reference/python-api.md#version)              | Sync         |
| [`get_system_info`](../api-reference/python-api.md#get_system_info)              | Sync         |
| [`health_check`](../api-reference/python-api.md#health_check)              | Sync         |
| [`configure_runtime`](../api-reference/python-api.md#configure_runtimeworker_threadsnone-max_blocking_threadsnone-thread_stack_size_mbnone)              | Sync         |
| [`shutdown`](../api-reference/python-api.md#shutdown)              | Sync         |

### LLM Client 

| Function                | Type         |
|-------------------------|--------------|
| [`complete`](llm-providers.md#creating-and-using-clients)              | Sync         |
| [`get_stats`](llm-providers.md#client-statistics)              | Sync         |
| [`complete_async`](llm-providers.md#asynchronous-operations)        | Async        |
| [`complete_stream`](llm-providers.md#streaming-responses)       | Async        |
| [`complete_batch`](llm-providers.md#batch-processing)   | Async        |
| [`warmup`](llm-providers.md#client-warmup)   | Async        |

### Embedding Client 

| Function      | Type |
|---------------|------|
| [`embed`](../api-reference/configuration.md#embeddings-client)              | Sync         |
| [`embed_many`](../api-reference/configuration.md#embeddings-client)              | Sync         |
| [`similarity`](../api-reference/configuration.md#embeddings-client)              | Sync         |

### Workflow 

| Function      | Type |
|---------------|------|
| [`add_node`](../api-reference/python-api.md#add_nodenode)              | Sync         |
| [`connect`](../api-reference/python-api.md#connectfrom_id-to_id)              | Sync         |
| [`validate`](../api-reference/python-api.md#validate)              | Sync         |

### Workflow Executor 

| Function      | Type |
|---------------|------|
| [`configure`](concepts.md#executor-configuration)              | Sync         |
| [`get_stats`](../api-reference/python-api.md#get_stats)              | Sync         |
| [`reset_stats`](../api-reference/python-api.md#reset_stats)              | Sync         |
| [`get_execution_mode`](../api-reference/python-api.md#get_execution_mode)              | Sync         |
| [`execute`](workflow-builder.md#setting-up-execution)              | Async         |
| [`run_async`](workflow-builder.md#asynchronous-execution)              | Async         |

### Workflow Result 

| Function      | Type |
|---------------|------|
| [`is_success`](../api-reference/python-api.md#is_success)              | Sync         |
| [`is_failed`](../api-reference/python-api.md#is_failed)              | Sync         |
| [`state`](../api-reference/python-api.md#state)              | Sync         |
| [`execution_time_ms`](../api-reference/python-api.md#execution_time_ms)              | Sync         |
| [`get_variable`](../api-reference/python-api.md#get_variablekey)              | Sync         |
| [`get_all_variables`](../api-reference/python-api.md#get_all_variables)              | Sync         |
| [`variables`](../api-reference/python-api.md#variables)              | Sync         |
| [`get_node_output`](../api-reference/python-api.md#get_node_output)              | Sync         |
| [`get_all_node_outputs`](../api-reference/python-api.md#get_all_node_outputs)              | Sync         |

---

## Usage Examples

### Synchronous Usage

```python
import os

from graphbit import LlmConfig, LlmClient

config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"))

client = LlmClient(config)
result = client.complete("Hello, world!")
print(result)
```

### Asynchronous Usage

```python
import os
import asyncio

from graphbit import LlmConfig, LlmClient

config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"))

client = LlmClient(config)

async def main():
    result = await client.complete_async("Hello, async world!")
    print(result)

    # Streaming completion
    stream_result = await client.complete_stream("Stream this!")
    print(stream_result)

asyncio.run(main())
```

---

This documentation demonstrates sync and async functions supported by GraphBit’s Python API, enabling you to select the best approach for your use case, whether you need straightforward synchronous calls or high-performance asynchronous workflows.

---
