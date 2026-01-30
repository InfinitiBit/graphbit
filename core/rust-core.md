# Rust Core (`graphbit-core`)

This document describes the **Rust core engine** of GraphBit (this `core/` crate). It’s written for contributors who want to understand (and extend) the workflow engine, agent system, and provider integrations.

## What lives in this crate

`graphbit-core` provides:

- **Workflow definition & validation**: graph nodes/edges + integrity checks
- **Workflow execution**: dependency-aware parallel execution, shared context
- **Agent abstraction**: a standard agent interface with an LLM-backed implementation
- **LLM provider layer**: a unified provider trait + provider implementations
- **Reliability & performance primitives**: retries, circuit breaker, concurrency controls
- **Utilities**: document loading, text splitting, embeddings, validation

> If you’re looking for the Python-facing API/bindings, those live under `../python/`. The Rust core is used by those bindings (and can also be used directly from Rust).

## Module map (where to look)

Core entry points and exports:

- `src/lib.rs`
  - Declares modules and **re-exports** the main public API (`Workflow`, `WorkflowExecutor`, `Agent`, etc.)
  - Sets a global allocator (`jemalloc`) on Unix when **not** building the `python` feature

Workflow data structures:

- `src/graph.rs`
  - `WorkflowGraph` (petgraph-backed DAG with serialization-friendly `nodes`/`edges`)
  - `WorkflowNode`, `WorkflowEdge`, `NodeType`
  - Graph validation: cycle detection, endpoint existence, node validation, duplicate agent IDs

Workflow execution engine:

- `src/workflow.rs`
  - `Workflow` and `WorkflowBuilder`
  - `WorkflowExecutor` with dependency-aware batch execution
  - Implicit context passing from parent nodes → child agent prompts
  - Template resolution: `{{node.<key>}}` and `{{node.<key>.<field>}}`

Agent system:

- `src/agents.rs`
  - `AgentTrait` (interface)
  - `Agent` (LLM-backed implementation)
  - `AgentConfig`, `AgentBuilder`

Core types and reliability primitives:

- `src/types.rs`
  - `AgentId`, `NodeId`, `WorkflowId` (UUID-based; `AgentId`/`NodeId` support deterministic IDs via `from_string`)
  - `WorkflowContext` (variables, node outputs, metadata, timing, stats)
  - `RetryConfig`, `CircuitBreakerConfig`, `CircuitBreaker`
  - `ConcurrencyConfig`, `ConcurrencyManager`, `ConcurrencyStats`

LLM provider abstraction:

- `src/llm/mod.rs`
  - `LlmRequest`, `LlmMessage`, tool calling types (`LlmTool`, `LlmToolCall`)
  - `LlmProviderFactory` (maps `LlmConfig` → concrete provider)
- `src/llm/providers.rs`
  - `LlmConfig` enum (provider config variants)
  - `LlmProviderTrait` (the provider interface), `LlmProvider` wrapper
  - (feature `python`) a registry to keep Python LLM instances alive for `LlmConfig::PythonBridge`

Errors and validation:

- `src/errors.rs` (`GraphBitError`, `GraphBitResult<T>`)
- `src/validation.rs` (JSON-schema-style validation helpers)

Supporting capabilities:

- `src/document_loader.rs` (PDF/DOCX/TXT/… loaders)
- `src/text_splitter.rs` (chunking strategies)
- `src/embeddings.rs` (embedding providers)

## Public API “mental model”

Most Rust users interact with core through types re-exported from `graphbit_core`:

- `Workflow`: a graph + metadata container
- `WorkflowNode` / `NodeType`: nodes that do work
- `WorkflowExecutor`: executes a `Workflow` and returns a `WorkflowContext`
- `WorkflowContext`: shared state, node outputs, metadata, stats
- `Agent` / `AgentTrait`: LLM-backed compute unit
- `LlmConfig` / `LlmProvider`: provider configuration and request execution

## Workflow data flow & prompt templating

### 1) Node outputs and shared variables

During execution, the executor stores results in:

- `WorkflowContext.node_outputs`: **structured JSON** keyed by:
  - node UUID (`NodeId.to_string()`), and
  - node name (`WorkflowNode.name`)
- `WorkflowContext.variables`: legacy/back-compat stringified outputs (also keyed by name and id)

Prefer `node_outputs` for new code and templates.

### 2) Prompt template resolution

The executor supports:

- `{{node.<key>}}` to substitute a full output value
- `{{node.<key>.<field>}}` to substitute nested JSON fields (dot notation)

Where `<key>` can be a node id string or a node name (because outputs are stored under both).

### 3) Implicit parent context (“preamble”)

For agent nodes, the executor builds an **implicit preamble** from _direct parent node outputs_ and prepends it to the agent’s prompt template. This enables “natural” chaining without requiring explicit template references everywhere.

## Execution model (dependencies + concurrency)

### Dependency-aware batching

`WorkflowExecutor::execute()` schedules nodes in **layers**:

- a batch contains all nodes whose dependencies have completed
- a batch is executed concurrently using `tokio::spawn`

This preserves correctness (parents first) while allowing parallelism where possible.

### Concurrency limits

`ConcurrencyManager` provides per-node-type limits (e.g. `agent`, `http_request`) plus a global budget via `ConcurrencyConfig`.

Provided presets:

- `WorkflowExecutor::new()` → `ConcurrencyConfig::default()`
- `WorkflowExecutor::new_high_throughput()`
- `WorkflowExecutor::new_low_latency()` (fail-fast, no retries by default)
- `WorkflowExecutor::new_memory_optimized()`

## Reliability model (retries + circuit breaker)

### Retry configuration

`RetryConfig` is used to decide:

- whether an error should be retried (`should_retry`)
- how long to wait (`calculate_delay`) with exponential backoff + jitter

The executor applies a default retry configuration unless disabled via `WorkflowExecutor::without_retries()`.

### Circuit breaker

Agent nodes are guarded by a per-agent `CircuitBreaker`:

- **Closed**: requests allowed
- **Open**: requests rejected until recovery timeout
- **HalfOpen**: allows probing until success threshold is met

This prevents cascading failures when a provider is unhealthy.

## LLM layer and tool calling

### Provider interface

LLM integrations implement `LlmProviderTrait`:

- `async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse>`
- optional streaming (`stream`)
- feature checks (`supports_function_calling`, etc.)

### Tool calling

`LlmRequest` can carry `tools: Vec<LlmTool>`.

In `WorkflowExecutor`, if an agent node has `tool_schemas` in `WorkflowNode.config`, the executor will:

- attach tools to the LLM request
- call the LLM provider
- if tool calls are returned, emit a structured JSON payload like:
  - `"type": "tool_calls_required"`
  - `"tool_calls": [...]`

**Important**: the Rust core does not execute tools itself; the Python layer is expected to execute tool calls (so tools can be normal Python callables with a registry).

## Feature flags and platform notes

### `python` feature

`graphbit-core` defines:

- `python`: enables PyO3 integration points (e.g. `LlmConfig::PythonBridge`)

On Unix, `jemalloc` is used as the global allocator **only when not** building with `python` (to avoid TLS allocation issues across the Python boundary).

## Contributing to `graphbit-core`

### Build and test (core only)

From the repository root:

```bash
cargo test -p graphbit-core
cargo fmt --check
cargo clippy -p graphbit-core -- -D warnings
```

Generate Rust docs locally:

```bash
cargo doc -p graphbit-core --open
```

### Add a new LLM provider (checklist)

1. **Implement the provider**
   - Add `src/llm/<provider>.rs`
   - Implement `LlmProviderTrait` for your provider type
2. **Add config surface**
   - Add a new variant to `LlmConfig` in `src/llm/providers.rs`
   - Add constructor helpers (e.g. `LlmConfig::<provider>()`) and `provider_name()` mapping
3. **Wire the factory**
   - Extend `LlmProviderFactory::create_provider()` in `src/llm/mod.rs`
4. **Add tests**
   - Prefer unit tests that validate request shaping and error handling
   - Avoid hard-coding secrets; use mocks where possible
5. **Document it**
   - Update the user-facing provider docs: `../docs/user-guide/llm-providers.md`

### Add a new node type (checklist)

1. **Define the node**
   - Add a new variant to `NodeType` in `src/graph.rs`
2. **Validate inputs**
   - Extend `WorkflowNode::validate()` for required fields and supported values
3. **Execute it**
   - Extend the match in `WorkflowExecutor::execute_node_with_retry()` (or a dedicated execution function)
4. **Make it usable from Python**
   - If the Python API exposes node constructors, add corresponding logic in `../python/`
5. **Document it**
   - Update `../docs/api-reference/node-types.md`

## Related project docs (outside this crate)

- `../docs/development/architecture.md` (3-tier overview)
- `../docs/development/python-bindings.md` (PyO3 layer details)
- `../docs/user-guide/workflow-builder.md` (Python-centric workflow usage)
