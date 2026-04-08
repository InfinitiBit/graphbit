# Streaming Workflow Guide

This guide explains how to use **workflow streaming** in GraphBit from start to finish. It covers:

- what streaming is and when to use it
- all stream modes (`updates`, `messages`, `all`)
- how to inspect the official streaming event schema
- complete runnable examples:
  - workflow streaming **without tools**
  - workflow streaming **with tools**

---

## What Is Workflow Streaming?

`Executor.execute_streaming(...)` runs a workflow and returns an iterator of event dictionaries in real time.

Instead of waiting for a final result only, you can receive:

- workflow lifecycle events (started/completed/failed)
- node lifecycle events (started/completed/failed)
- LLM call lifecycle events
- tool call lifecycle events
- token chunks as they are generated

This is useful for:

- live UI updates
- progress bars and status tracking
- detailed debugging and observability
- showing partial model output as it arrives

---

## Prerequisites

Install and configure GraphBit, then initialize it:

```python
import graphbit
from graphbit import Executor, LlmConfig, Node, Workflow

graphbit.init()
```

You also need an LLM provider config, for example:

```python
import os
from graphbit import LlmConfig

config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini",
)
```

---

## Streaming API Basics

### `execute_streaming`

```python
iterator = executor.execute_streaming(workflow, stream_mode="updates")
for event in iterator:
    print(event)
```

`stream_mode` controls which events are emitted:

- `updates`: lifecycle updates (workflow/node/llm/tool), **no token chunks**
- `messages`: token chunks + terminal workflow event
- `all`: everything (`updates` + `messages`)

If `stream_mode` is not provided, GraphBit defaults to `updates`.

---

## Inspect the Official Event Schema

GraphBit exposes a built-in schema for streaming events. This is the best way to build robust consumers and avoid guessing event fields.

```python
from graphbit import Executor
import json

schema = Executor.get_stream_event_schema()
print(json.dumps(schema, indent=2))
```

The schema includes:

- stream modes and their semantics
- all event types
- per-event field names, field types, and descriptions
- event grouping metadata (`workflow`, `node`, `llm`, `tool`)

---

## Event Structure You Receive

Each streamed event is a Python `dict`.

Common keys:

- `event`: concrete event name (for example `node_started`, `token`)
- `time`: RFC3339 timestamp
- `category`: `workflow | node | llm | tool`
- `phase`: `started | completed | failed | token`

Event-specific keys are added depending on `event`. Examples:

- token event: `node_id`, `node_name`, `llm_call_id`, `content`
- tool completion: `tool_name`, `tool_call_id`, `output`, `duration_ms`
- workflow completion: `result`, `outputs`

---

## Complete Example: Streaming Without Tools

This example creates a two-node workflow and streams it in `all` mode so you can see lifecycle and token events together.

```python
import os
import graphbit
from graphbit import Executor, LlmConfig, Node, Workflow

graphbit.init()

api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    raise RuntimeError("OPENAI_API_KEY is not set")

config = LlmConfig.openai(api_key, "gpt-4o-mini")

# Build a simple no-tools workflow
workflow = Workflow("StreamingWithoutTools")

node1 = Node.agent(
    name="ExplainLangChain",
    prompt="What is LangChain? Explain in 3 concise lines.",
    llm_config=config,
)
node2 = Node.agent(
    name="ExplainCrewAI",
    prompt="What is CrewAI? Explain in 3 concise lines.",
    llm_config=config,
)

id1 = workflow.add_node(node1)
id2 = workflow.add_node(node2)
workflow.connect(id1, id2)
workflow.validate()

executor = Executor(config)

event_count = 0
token_count = 0
final_result = None

for event in executor.execute_streaming(workflow, stream_mode="all"):
    event_count += 1
    event_type = event.get("event")

    if event_type == "token":
        token_count += 1
        print(event.get("content", ""), end="", flush=True)
        continue

    if event_type == "workflow_started":
        print(f"\n[workflow_started] total_nodes={event.get('total_nodes')}")
    elif event_type == "node_started":
        print(f"\n[node_started] {event.get('node_name')}")
    elif event_type == "llm_call_started":
        print(
            f"\n[llm_call_started] node={event.get('node_name')} "
            f"iteration={event.get('iteration')} model={event.get('model')}"
        )
    elif event_type == "llm_call_completed":
        print(
            f"\n[llm_call_completed] node={event.get('node_name')} "
            f"finish_reason={event.get('finish_reason')} "
            f"duration_ms={event.get('duration_ms')}"
        )
    elif event_type == "node_completed":
        output = event.get("output", "")
        print(f"\n[node_completed] {event.get('node_name')} output_chars={len(output)}")
    elif event_type == "workflow_completed":
        final_result = event.get("result")
        print("\n[workflow_completed]")
    elif event_type == "workflow_failed":
        print(f"\n[workflow_failed] {event.get('error')} ({event.get('error_type')})")

print(f"\n\nTotal events: {event_count}, token events: {token_count}")
if final_result is not None:
    print("Node outputs:")
    print(final_result.get_all_node_outputs())
```

---

## Complete Example: Streaming With Tools

This example shows tool-call events (`tool_call_started`, `tool_call_completed`) plus token streaming.

```python
import os
import graphbit
from graphbit import Executor, LlmConfig, Node, Workflow, tool

graphbit.init()

api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    raise RuntimeError("OPENAI_API_KEY is not set")

config = LlmConfig.openai(api_key, "gpt-4o-mini")

@tool(_description="Add two integers")
def add(a: int, b: int) -> str:
    return f"The sum of {a} and {b} is {a + b}."

@tool(_description="Get weather by city")
def get_weather(city: str) -> str:
    return f"The weather in {city} is sunny, 25C."

workflow = Workflow("StreamingWithTools")

planner = Node.agent(
    name="Planner",
    prompt=(
        "Use tools to do two things:\n"
        "1) Find the sum of 10 and 20.\n"
        "2) Find the weather in Dhaka."
    ),
    llm_config=config,
    tools=[add, get_weather],
)

advisor = Node.agent(
    name="Advisor",
    prompt="Based on the weather in Dhaka, suggest what I should wear.",
    llm_config=config,
    tools=[get_weather],
)

planner_id = workflow.add_node(planner)
advisor_id = workflow.add_node(advisor)
workflow.connect(planner_id, advisor_id)
workflow.validate()

executor = Executor(config)

current_node = None
current_call = None
final_result = None

for event in executor.execute_streaming(workflow, stream_mode="all"):
    event_type = event.get("event")

    if event_type == "token":
        node_name = event.get("node_name")
        llm_call_id = event.get("llm_call_id")
        if node_name != current_node or llm_call_id != current_call:
            current_node = node_name
            current_call = llm_call_id
            print(f"\n--- streaming node={node_name} llm_call_id={llm_call_id} ---")
        print(event.get("content", ""), end="", flush=True)
        continue

    if event_type == "tool_call_started":
        print(
            f"\n[tool_call_started] node={event.get('node_name')} "
            f"tool={event.get('tool_name')} args={event.get('parameters')}"
        )
    elif event_type == "tool_call_completed":
        print(
            f"\n[tool_call_completed] node={event.get('node_name')} "
            f"tool={event.get('tool_name')} duration_ms={event.get('duration_ms')}"
        )
        print(f"output={event.get('output')}")
    elif event_type == "tool_call_failed":
        print(
            f"\n[tool_call_failed] node={event.get('node_name')} "
            f"tool={event.get('tool_name')} error={event.get('error')}"
        )
    elif event_type == "workflow_completed":
        final_result = event.get("result")
        print("\n[workflow_completed]")
    elif event_type == "workflow_failed":
        print(f"\n[workflow_failed] {event.get('error')} ({event.get('error_type')})")

print("\n")
if final_result is not None:
    print("Final node outputs:")
    print(final_result.get_all_node_outputs())
    print("Response metadata:")
    print(final_result.get_all_node_response_metadata())
```

---

## Choosing the Right `stream_mode`

- Use `updates` when you want state transitions, durations, and tool/LLM lifecycle without printing token-by-token output.
- Use `messages` when you mainly care about live token text and final terminal status.
- Use `all` for full observability and debugging.

---

## Error Handling Pattern

Always handle terminal outcomes explicitly:

```python
for event in executor.execute_streaming(workflow, stream_mode="all"):
    if event["event"] == "workflow_failed":
        print("Failed:", event["error"], event.get("error_type"))
        break
    if event["event"] == "workflow_completed":
        result = event["result"]
        print("Success:", result.get_all_node_outputs())
        break
```

---

## Get Node Outputs and Complete Workflow Metadata

When streaming finishes successfully, the `workflow_completed` event includes a `result` object (a `WorkflowResult`).

Use it to access:

- all final node outputs: `result.get_all_node_outputs()`
- full workflow/node response metadata: `result.get_all_node_response_metadata()`

```python
final_result = None

for event in executor.execute_streaming(workflow, stream_mode="all"):
    if event["event"] == "workflow_completed":
        final_result = event["result"]
        break
    if event["event"] == "workflow_failed":
        raise RuntimeError(event["error"])

if final_result is not None:
    node_outputs = final_result.get_all_node_outputs()
    workflow_metadata = final_result.get_all_node_response_metadata()

    print("Node outputs:")
    print(node_outputs)

    print("Workflow metadata:")
    print(workflow_metadata)
```

### What these contain

- `result.get_all_node_outputs()`
  - Returns a dictionary keyed by node name, with each node's final output text/value.
- `result.get_all_node_response_metadata()`
  - Returns detailed metadata for the full execution, including per-node/provider response details and workflow-level metadata (such as workflow name and execution-level response information).

---

## Practical Tips

- Validate workflow structure before execution: `workflow.validate()`
- Keep `stream_mode="all"` while developing, then reduce to `updates` or `messages` in production
- Group token streams by `llm_call_id` when rendering in UIs
- Use `Executor.get_stream_event_schema()` in tests to assert expected event contracts
- Log `time`, `event`, `category`, and `phase` for easier debugging

---

## Related Docs

- [Workflow Builder](workflow-builder.md)
- [Agent Configuration](agents.md)
- [Async vs Sync](async-vs-sync.md)
- [Python API Reference](../api-reference/python-api.md)
