# Migration Guide

This guide helps you migrate between different versions of the GraphBit JavaScript bindings.

## Table of Contents

- [Migrating to v1.0](#migrating-to-v10)
- [Breaking Changes by Version](#breaking-changes-by-version)
- [Common Migration Patterns](#common-migration-patterns)

## Migrating to v1.0

### From Python Bindings

If you're migrating from the Python bindings to JavaScript:

#### Workflow Creation

**Python:**

```python
from graphbit import Workflow, Executor, LlmConfig

workflow = Workflow("My Workflow")
workflow.add_node(...)
```

**JavaScript:**

```typescript
import { WorkflowBuilder, Executor, LlmConfig } from '@graphbit/core';

const workflow = new WorkflowBuilder('My Workflow')
  .addNode(...)
  .build();
```

#### LLM Configuration

**Python:**

```python
llm_config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"
)
```

**JavaScript:**

```typescript
const llmConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini',
});
```

#### Workflow Execution

**Python:**

```python
executor = Executor(config=llm_config)
result = executor.execute(workflow)

if result.is_completed():
    print(result.get_all_nodes_outputs())
```

**JavaScript:**

```typescript
const executor = new Executor(llmConfig);
const result = await executor.execute(workflow);

if (await result.isCompleted()) {
  console.log(await result.getAllOutputs());
}
```

### Key Differences

1. **Async/Await**: JavaScript bindings use async/await for all I/O operations
2. **Naming Convention**: JavaScript uses camelCase instead of snake_case
3. **Builder Pattern**: JavaScript uses builder pattern more extensively
4. **Type Safety**: TypeScript provides compile-time type checking

## Breaking Changes by Version

### v1.0.0 (Initial Release)

- Initial stable release
- Full API surface defined
- TypeScript definitions included

### v0.9.0 (Beta)

- Pre-release version
- API subject to change
- Not recommended for production

## Common Migration Patterns

### Error Handling

**Old Pattern (v0.x):**

```typescript
try {
  const result = executor.execute(workflow);
} catch (error) {
  console.error(error);
}
```

**New Pattern (v1.x):**

```typescript
try {
  const result = await executor.execute(workflow);
  if (await result.isFailed()) {
    const error = await result.error();
    console.error(error);
  }
} catch (error) {
  console.error('Execution error:', error);
}
```

### Configuration Options

**Old Pattern:**

```typescript
const config = LlmConfig.openai('api-key', 'gpt-4');
```

**New Pattern:**

```typescript
const config = LlmConfig.openai({
  apiKey: 'api-key',
  model: 'gpt-4',
  temperature: 0.7,
  maxTokens: 1000,
});
```

### Text Splitting

**Old Pattern:**

```typescript
const splitter = new TextSplitter('character', { chunkSize: 100 });
```

**New Pattern:**

```typescript
const splitter = TextSplitter.character(100, 20);
```

## Automated Migration

For large codebases, consider using automated migration tools:

```bash
# Install migration tool (when available)
npm install -g @graphbit/migrate

# Run migration
graphbit-migrate --from 0.9 --to 1.0 ./src
```

## Getting Help

If you encounter issues during migration:

1. Check the [API Reference](./api/README.md)
2. Review [Examples](./examples/README.md)
3. Search [GitHub Issues](https://github.com/InfinitiBit/graphbit/issues)
4. Ask in [Discord Community](https://discord.gg/graphbit)
