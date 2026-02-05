# Migration Guide: Python to JavaScript

**Version:** GraphBit 0.5.5  
**Last Updated:** 2025-12-06

## Overview

This guide helps Python developers transition to GraphBit's JavaScript bindings. While the core concepts remain the same, there are important syntax and API differences to understand.

---

## Quick Reference

| Category | Python | JavaScript |
|----------|--------|------------|
| **Async** | Optional (sync/async) | Required (everything is async) |
| **Imports** | `from graphbit import ...` | `const { ... } = require('@infinitibit_gmbh/graphbit')` |
| **Null handling** | `None` for optional | `undefined` (omit field) |
| **Enums** | Strings | Numbers |
| **Naming** | `snake_case` | `camelCase` |

---

## 1. Installation & Setup

### Python

```python
pip install graphbit
```

### JavaScript

```javascript
npm install @infinitibit_gmbh/graphbit
```

---

## 2. Initialization

### Python

```python
import graphbit

# Optional initialization
graphbit.init()

# Get version
version = graphbit.version()
```

### JavaScript

```javascript
const { init, version } = require('@infinitibit_gmbh/graphbit');

// Required initialization
init();

// Get version (async)
const ver = version();
```

**Key Difference:** JavaScript `init()` should always be called before using GraphBit.

---

## 3. LLM Configuration

### Python

```python
from graphbit import LlmConfig

# Positional arguments
config = LlmConfig.openai('your-api-key', 'gpt-4o-mini')

# Or keyword arguments
config = LlmConfig.openai(
    api_key='your-api-key',
    model='gpt-4o-mini'
)
```

### JavaScript

```javascript
const { LlmConfig } = require('@infinitibit_gmbh/graphbit');

// Object parameters (required)
const config = LlmConfig.openai({
  apiKey: 'your-api-key',
  model: 'gpt-4o-mini'
});
```

**Key Differences:**

- ✅ JS uses `apiKey` (camelCase) instead of `api_key`
- ✅ JS requires object parameter, not positional
- ✅ All parameters in a single object

---

## 4. Workflows

### Python

```python
from graphbit import WorkflowBuilder, Workflow

# Create workflow
builder = WorkflowBuilder('My Workflow')
builder.description('A test workflow')

workflow = builder.build()

# Add node
workflow.add_node({
    'id': 'agent1',
    'name': 'Research Agent',
    'node_type': 'Agent'
})

# Add edge
workflow.add_edge('agent1', 'agent2', {
    'from_node': 'agent1',
    'to_node': 'agent2'
})

# Validate
is_valid = workflow.validate()
```

### JavaScript

```javascript
const { WorkflowBuilder } = require('@infinitibit_gmbh/graphbit');

// Create workflow  
const builder = new WorkflowBuilder('My Workflow');
builder.description('A test workflow');

const workflow = builder.build();

// Add node (async!)
await workflow.addNode({
  id: 'agent1',
  name: 'Research Agent',
  nodeType: 'Agent'  // camelCase
});

// Add edge (async!)
await workflow.addEdge('agent1', 'agent2', {
  fromNode: 'agent1',  // Required despite method args
  toNode: 'agent2'      // Required despite method args
});

// Validate (async!)
const isValid = await workflow.validate();
```

**Key Differences:**

- ✅ JS uses `new` keyword for WorkflowBuilder
- ✅ All workflow methods are async (use `await`)
- ✅ Field names are camelCase
- ✅ Edge objects need redundant fromNode/toNode fields

---

## 5. Retry Configuration

### Python

```python
from graphbit import RetryConfig, RetryableErrorType

retry_config = RetryConfig(
    max_attempts=3,
    initial_delay_ms=100,
    backoff_multiplier=2.0,
    max_delay_ms=1000,
    jitter_factor=0.1,
    retryable_errors=['network_error']  # String list
)
```

### JavaScript

```javascript
const { RetryableErrorType } = require('@infinitibit_gmbh/graphbit');

const retryConfig = {
  maxAttempts: 3,
  initialDelayMs: 100,
  backoffMultiplier: 2.0,
  maxDelayMs: 1000,
  jitterFactor: 0.1,
  retryableErrors: [RetryableErrorType.NetworkError]  // Numeric enum!
};
```

**Critical Difference:**

```javascript
// ❌ WRONG - Will throw NumberExpected error
retryableErrors: ['NetworkError']

// ✅ CORRECT - Use enum number
retryableErrors: [RetryableErrorType.NetworkError]
```

---

## 6. Agents

### Python

```python
from graphbit import AgentBuilder, LlmConfig

llm_config = LlmConfig.openai('key', 'gpt-4o-mini')

agent = AgentBuilder('Research Agent', llm_config) \
    .description('Researches topics') \
    .system_prompt('You are a researcher') \
    .temperature(0.7) \
    .max_tokens(1000) \
    .build()

# Execute
response = agent.execute('What is Python?')
```

### JavaScript

```javascript
const { AgentBuilder, LlmConfig } = require('@infinitibit_gmbh/graphbit');

const llmConfig = LlmConfig.openai({
  apiKey: 'key',
  model: 'gpt-4o-mini'
});

const agent = await new AgentBuilder('Research Agent', llmConfig)
  .description('Researches topics')
  .systemPrompt('You are a researcher')  // camelCase
  .temperature(0.7)
  .maxTokens(1000)  // camelCase
  .build();  // async!

// Execute (async!)
const response = await agent.execute('What is Python?');
```

**Key Differences:**

- ✅ `build()` is async in JS
- ✅ `execute()` is async in JS
- ✅ Use camelCase for method names

---

## 7. Executors

### Python

```python
from graphbit import Executor, ExecutorConfig, LlmConfig

llm_config = LlmConfig.openai('key')

executor = Executor(
    llm_config,
    executor_config=ExecutorConfig(max_concurrent_nodes=5)
)

context = executor.execute(workflow)

if context.is_completed():
    print('Success!')
```

### JavaScript

```javascript
const { Executor, LlmConfig } = require('@infinitibit_gmbh/graphbit');

const llmConfig = LlmConfig.openai({ apiKey: 'key' });

const executor = new Executor(llmConfig, {
  maxConcurrentNodes: 5  // Optional config object
});

const context = await executor.execute(workflow);  // async!

if (await context.isCompleted()) {  // async!
  console.log('Success!');
}
```

**Key Differences:**

- ✅ Constructor uses `new`
- ✅ Config is plain object, not `ExecutorConfig` class
- ✅ All context methods are async

---

## 8. Document Loading

### Python

```python
from graphbit import DocumentLoader

loader = DocumentLoader()
doc = loader.load_document('file.txt', 'txt')

print(doc.content)
```

### JavaScript

```javascript
const { DocumentLoader } = require('@infinitibit_gmbh/graphbit');

const loader = new DocumentLoader();
const doc = await loader.loadFile('file.txt', 'txt');  // async!

console.log(doc.content);
```

**Key Differences:**

- ✅ Method is `loadFile` not `load_document`
- ✅ Async operation
- ✅ JS also has `loadText()` method (Python doesn't)

---

## 9. Text Splitting

### Python

```python
from graphbit import CharacterSplitter, RecursiveSplitter

# Separate classes
char_splitter = CharacterSplitter(chunk_size=100, chunk_overlap=20)
recursive_splitter = RecursiveSplitter(chunk_size=100, chunk_overlap=20)

chunks = char_splitter.split_text('long text...')
```

### JavaScript

```javascript
const { TextSplitter } = require('@infinitibit_gmbh/graphbit');

// Unified class with factory methods
const charSplitter = TextSplitter.character(100, 20);
const recursiveSplitter = TextSplitter.recursive(100, 20);

const chunks = charSplitter.split('long text...');  // 'split' not 'split_text'
```

**Key Differences:**

- ✅ JS uses single `TextSplitter` class with factories
- ✅ Method is `split()` not `split_text()`
- ✅ Cleaner API in JS

---

## 10. Embeddings

### Python

```python
from graphbit import EmbeddingConfig, EmbeddingClient

config = EmbeddingConfig.openai('key', 'text-embedding-3-small')
client = EmbeddingClient(config)

# Single text
embedding = client.embed('text')

# Multiple texts
embeddings = client.embed_many(['text1', 'text2'])
```

### JavaScript

```javascript
const { EmbeddingConfig, EmbeddingClient } = require('@infinitibit_gmbh/graphbit');

const config = EmbeddingConfig.openai('key', 'text-embedding-3-small');
const client = new EmbeddingClient(config);

// Only array method (async!)
const response = await client.embed(['text1', 'text2']);

// Access embeddings
const embeddings = response.embeddings;
```

**Key Differences:**

- ✅ JS only has `embed(array)`, no separate `embed_many()`
- ✅ Returns full response object with metadata
- ✅ Async operation

---

## 11. Tools

### Python

```python
from graphbit import ToolRegistry

registry = ToolRegistry()

# Register tool
@registry.tool('calculator')
def calculate(operation: str, a: float, b: float) -> float:
    if operation == 'add':
        return a + b
    return 0

# Execute
result = registry.execute_tool('calculator', {
    'operation': 'add',
    'a': 5,
    'b': 3
})
```

### JavaScript

```javascript
const { createToolRegistry } = require('@infinitibit_gmbh/graphbit');

const registry = createToolRegistry();

// Register tool
registry.register(
  'calculator',
  'Performs calculations',
  { operation: 'string', a: 'number', b: 'number' },
  (args) => {
    if (args.operation === 'add') {
      return args.a + args.b;
    }
    // ❌ DON'T throw errors - causes fatal crash
    return 'Error: Unknown operation';  // ✅ Return error string
  }
);

// Execute (async!)
const result = await registry.execute('calculator', {
  operation: 'add',
  a: 5,
  b: 3
});
```

**Critical Difference:**

```javascript
// ❌ FATAL - Causes NAPI crash
(args) => {
  throw new Error('Error!');
}

// ✅ SAFE - Return error values
(args) => {
  return 'Error: Something went wrong';
}
```

---

## 12. Common Gotchas

### Gotcha 1: Null vs Undefined

**Python:**

```python
# Optional fields can be None
edge = {
    'from_node': 'a',
    'to_node': 'b',
    'condition': None  # OK in Python
}
```

**JavaScript:**

```javascript
// ❌ WRONG - null causes errors
const edge = {
  fromNode: 'a',
  toNode: 'b',
  condition: null  // ❌ Causes StringExpected error
};

// ✅ CORRECT - omit or use undefined
const edge = {
  fromNode: 'a',
  toNode: 'b'
  // condition omitted entirely
};
```

---

### Gotcha 2: Enum Values

**Python:**

```python
# Strings work
retry_config = {
    'retryable_errors': ['network_error', 'timeout']
}
```

**JavaScript:**

```javascript
// ❌ WRONG - strings don't work
const retryConfig = {
  retryableErrors: ['network_error']  // NumberExpected error
};

// ✅ CORRECT - use enum numbers
const retryConfig = {
  retryableErrors: [
    RetryableErrorType.NetworkError,
    RetryableErrorType.Timeout
  ]
};
```

---

### Gotcha 3: Async Everywhere

**Python:**

```python
# Most methods are sync
name = workflow.name()
valid = workflow.validate()
```

**JavaScript:**

```javascript
// ❌ WRONG - forgetting await
const name = workflow.name();  // Returns Promise, not string!

// ✅ CORRECT - use await
const name = await workflow.name();
const valid = await workflow.validate();
```

---

## 13. Feature Availability

### Not Available in JavaScript

These Python features are not available in JS:

```python
# ❌ Not in JS
graphbit.get_system_info()
graphbit.health_check()
graphbit.configure_runtime()
graphbit.shutdown()

# ❌ Not in JS as standalone class
LlmClient  # Use Agent or Executor instead
```

### JS-Specific Features

These are available only in JavaScript:

```javascript
// ✅ JS-specific
DocumentLoader.loadText(text, source)  // Load from string
```

---

## 14. Complete Migration Example

### Python Version

```python
from graphbit import (
    WorkflowBuilder,
    AgentBuilder,
    Executor,
    LlmConfig,
    RetryConfig
)

# Config
llm_config = LlmConfig.openai('key', 'gpt-4o-mini')

# Workflow
builder = WorkflowBuilder('Research')
workflow = builder.build()

# Add node with retry
workflow.add_node({
    'id': 'agent1',
    'name': 'Researcher',
    'node_type': 'Agent',
    'retry_config': RetryConfig(
        max_attempts=3,
        retryable_errors=['network_error']
    )
})

# Execute
executor = Executor(llm_config)
context = executor.execute(workflow)

if context.is_completed():
    outputs = context.get_all_outputs()
    print(f'Done: {outputs}')
```

### JavaScript Version

```javascript
const {
  WorkflowBuilder,
  AgentBuilder,
  Executor,
  LlmConfig,
  RetryableErrorType
} = require('@infinitibit_gmbh/graphbit');

async function main() {
  // Config
  const llmConfig = LlmConfig.openai({
    apiKey: 'key',
    model: 'gpt-4o-mini'
  });

  // Workflow
  const builder = new WorkflowBuilder('Research');
  const workflow = builder.build();

  // Add node with retry
  await workflow.addNode({
    id: 'agent1',
    name: 'Researcher',
    nodeType: 'Agent',
    retryConfig: {
      maxAttempts: 3,
      retryableErrors: [RetryableErrorType.NetworkError]  // Enum!
    }
  });

  // Execute
  const executor = new Executor(llmConfig);
  const context = await executor.execute(workflow);

  if (await context.isCompleted()) {
    const outputs = await context.getAllOutputs();
    console.log(`Done: ${JSON.stringify(outputs)}`);
  }
}

main().catch(console.error);
```

---

## 15. Migration Checklist

When migrating from Python to JavaScript:

- [ ] Change imports to `require()` or `import`
- [ ] Add `async/await` to all GraphBit calls
- [ ] Change `snake_case` to `camelCase`
- [ ] Convert function arguments to object parameters
- [ ] Use enum numbers, not strings
- [ ] Remove `None`, use `undefined` (omit fields)
- [ ] Add `new` keyword to constructors
- [ ] Wrap main code in `async function main()`
- [ ] Replace Python classes with JS factories (e.g., `TextSplitter.character()`)
- [ ] Return errors from tool callbacks, don't throw
- [ ] Test enum usage carefully
- [ ] Test optional field handling

---

## 16. Testing Your Migration

```javascript
// Quick test script
const { init, version } = require('@infinitibit_gmbh/graphbit');

async function testMigration() {
  try {
    init();
    console.log('✅ GraphBit initialized');
    
    const ver = version();
    console.log(`✅ Version: ${ver}`);
    
    // Test your migrated code here
    
  } catch (error) {
    console.error('❌ Migration issue:', error.message);
    
    // Check for common issues
    if (error.message.includes('NumberExpected')) {
      console.log('Hint: Use enum values, not strings');
    }
    if (error.message.includes('StringExpected')) {
      console.log('Hint: Use undefined, not null');
    }
  }
}

testMigration();
```

---

## 17. Getting Help

- **Documentation:** See `docs/js/` for comprehensive references
- **Examples:** Check `docs/js/examples/` for working code
- **Python Docs:** `docs/api-reference/python-api.md` for comparison
- **Issues:** Report migration problems on GitHub

---

## Quick Tips

1. **Start Simple:** Migrate one feature at a time
2. **Test Early:** Run tests after each migration step
3. **Use TypeScript:** Consider TypeScript for better type safety
4. **Check Examples:** Refer to verified examples in `docs/js/examples/`
5. **Watch for Async:** Add `await` everywhere
6. **Trust the Errors:** NAPI errors are usually clear about what's wrong

---

**Last Updated:** 2025-12-06  
**GraphBit Version:** 0.5.5  
**Maintainer:** GraphBit Documentation Team
