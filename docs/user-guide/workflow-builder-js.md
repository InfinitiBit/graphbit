# Workflow Builder - JavaScript

The Workflow Builder in GraphBit JavaScript provides an approach to creating AI agent workflows. Build workflows by creating agents, connecting them, and executing them with the executor.

## Overview

GraphBit workflows are built using:
- **WorkflowBuilder** - Creates workflow containers
- **AgentBuilder** - Creates AI agents as workflow nodes  
- **Workflow** - Container for nodes and connections
- **Executor** - Runs workflows and returns results

## Basic Usage

### Creating a Workflow

```typescript
import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

// Initialize GraphBit
init();

// Create a new workflow
const workflowBuilder = new WorkflowBuilder('My AI Pipeline')
  .description('A sample AI workflow');
  
const workflow = await workflowBuilder.build();
```

### Creating Agents

Agents are the primary processing units in workflows:

```typescript
// Configure LLM
const llmConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});

// Create an analyzer agent
const analyzer = await new AgentBuilder('Data Analyzer', llmConfig)
  .systemPrompt('Analyze this data for patterns')
  .description('Analyzes input data')
  .build();

// Create a summarizer agent
const summarizer = await new AgentBuilder('Content Summarizer', llmConfig)
  .systemPrompt('Summarize the following analyzed content')
  .description('Creates summaries')
  .build();

// Create a formatter agent
const formatter = await new AgentBuilder('Output Formatter', llmConfig)
  .systemPrompt('Transform the provided text to uppercase')
  .description('Formats output')
  .build();
```

### Building the Workflow

```typescript
import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

init();

const llmConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});

// Create workflow
const workflow = await new WorkflowBuilder('Data Pipeline')
  .description('Processes and analyzes data')
  .build();

// Create agents
const analyzer = await new AgentBuilder('Analyzer', llmConfig)
  .systemPrompt('Analyze the input data')
  .build();

const formatter = await new AgentBuilder('Formatter', llmConfig)
  .systemPrompt('Format the analyzed data')
  .build();

const summarizer = await new AgentBuilder('Summarizer', llmConfig)
  .systemPrompt('Create a summary')
  .build();

// Add agents as nodes to workflow
await workflow.addNode({
  id: 'analyzer',
  name: await analyzer.name(),
  description: await analyzer.description(),
  nodeType: 'Agent'
});

await workflow.addNode({
  id: 'formatter',
  name: await formatter.name(),
  description: await formatter.description(),
  nodeType: 'Agent'
});

await workflow.addNode({
  id: 'summarizer',
  name: await summarizer.name(),
  description: await summarizer.description(),
  nodeType: 'Agent'
});

// Connect nodes to define data flow
await workflow.addEdge('analyzer', 'formatter');
await workflow.addEdge('formatter', 'summarizer');
```

## Workflow Validation

Before execution, always validate your workflow:

```typescript
// Validate workflow structure
const errors = await workflow.validate();

if (errors.length > 0) {
  console.error('Workflow validation failed:', errors);
} else {
  console.log('Workflow is valid');
}
```

**Validation checks:**
- No circular dependencies
- All nodes are properly connected
- Node IDs are unique

## Setting Up Execution

### Basic Execution

```typescript
import { LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});

const executor = new Executor(config);

// Execute workflow
const result = await executor.execute(workflow, {
  input: 'Process this text'
});

// Check results
console.log('Workflow execution completed');
const outputs = result.getAllNodeOutputs();
console.log('Node outputs:', outputs);
```

## Accessing Workflow Results

### Get Node Outputs

```typescript
// Get all outputs
const allOutputs = result.getAllNodeOutputs();
console.log('All outputs:', allOutputs);

// Access specific outputs by analyzing the result structure
console.log('Result:', result);
```

## Complete Example

```typescript
import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

async function main() {
  // Initialize
  init();

  // Configure LLM
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  // Create workflow
  const workflow = await new WorkflowBuilder('Text Processing Pipeline')
    .description('Analyzes and summarizes text')
    .build();

  // Create agents
  const analyzer = await new AgentBuilder('Text Analyzer', config)
    .systemPrompt('Analyze the sentiment and key topics in the text')
    .build();

  const summarizer = await new AgentBuilder('Summarizer', config)
    .systemPrompt('Create a concise summary of the analyzed content')
    .build();

  // Add nodes
  await workflow.addNode({
    id: 'analyzer',
    name: await analyzer.name(),
    description: await analyzer.description(),
    nodeType: 'Agent'
  });

  await workflow.addNode({
    id: 'summarizer',
    name: await summarizer.name(),
    description: await summarizer.description(),
    nodeType: 'Agent'
  });

  // Connect nodes
  await workflow.addEdge('analyzer', 'summarizer');

  // Validate
  const errors = await workflow.validate();
  if (errors.length > 0) {
    console.error('Validation errors:', errors);
    return;
  }

  // Execute
  const executor = new Executor(config);
  const result = await executor.execute(workflow, {
    input: 'GraphBit is an amazing workflow automation framework for building AI agent pipelines...'
  });

  // Display results
  console.log('Pipeline completed successfully');
  const outputs = result.getAllNodeOutputs();
  console.log('All outputs:', outputs);
}

main().catch(console.error);
```

## Best Practices

### 1. Clear Agent Names
Use descriptive names for easy debugging:

```typescript
// ✅ Good
const emailValidator = await new AgentBuilder('Email Validator', config)
  .systemPrompt('Validate email format')
  .build();

// ❌ Poor
const agent1 = await new AgentBuilder('a1', config).build();
```

### 2. Meaningful Node IDs
Use consistent, meaningful IDs when adding to workflow:

```typescript
// ✅ Good
await workflow.addNode({
  id: 'text_analyzer_v1',
  name: await agent.name(),
  nodeType: 'Agent'
});

// ❌ Poor
await workflow.addNode({
  id: 'node1',
  name: 'n1',
  nodeType: 'Agent'
});
```

### 3. Always Validate
Validate before execution:

```typescript
const errors = await workflow.validate();
if (errors.length > 0) {
  throw new Error(`Validation failed: ${JSON.stringify(errors)}`);
}
```

### 4. Handle Errors
Always handle execution errors:

```typescript
try {
  const result = await executor.execute(workflow, { input: 'test' });
  console.log('Success:', result.getAllNodeOutputs());
} catch (error) {
  console.error('Execution error:', error);
}
```

### 5. Use Environment Variables
Store API keys securely:

```typescript
import 'dotenv/config';

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY
});
```

## Troubleshooting

### Workflow Won't Validate
- Check for circular connections
- Ensure all nodes are reachable
- Verify node IDs are unique

### Agents Not Creating
- Verify LLM config is correct
- Check API keys are set in environment
- Ensure `await` is used with `build()`

### Execution Fails
- Validate API keys are correct
- Check network connectivity
- Review error messages carefully

## See Also

- [Agents](./agents-js.md)
- [LLM Providers](./llm-providers-js.md)
- [Quick Start](../getting-started/quickstart-js.md)
