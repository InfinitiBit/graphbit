# Quick Start Guide

Get started with GraphBit JavaScript bindings in 5 minutes.

## Installation

### Prerequisites

- Node.js >= 16.0.0
- npm or yarn

### Install from npm (when published)

```bash
npm install graphbit
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/javascript

# Install dependencies
npm install

# Build the bindings
npm run build

# Run tests
npm test
```

## Basic Usage

### 1. Initialize the Library

```typescript
import { init } from 'graphbit';

// Initialize GraphBit
init();
```

### 2. Configure an LLM Provider

```typescript
import { LlmConfig } from 'graphbit';

// OpenAI
const llmConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini',
  temperature: 0.7,
});

// Or Anthropic
const anthropicConfig = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
  model: 'claude-3-5-sonnet-20241022',
});

// Or Ollama (local)
const ollamaConfig = LlmConfig.ollama({
  model: 'llama2',
  baseUrl: 'http://localhost:11434',
});
```

### 3. Create a Workflow

```typescript
import { WorkflowBuilder } from 'graphbit';

const workflow = new WorkflowBuilder('My First Workflow')
  .description('A simple workflow to get started')
  .addMetadata('version', JSON.stringify('1.0'))
  .build();

console.log('Workflow created:', await workflow.name());
```

### 4. Execute the Workflow

```typescript
import { Executor } from 'graphbit';

const executor = new Executor(llmConfig, {
  timeoutSeconds: 60,
  debug: true,
});

const result = await executor.execute(workflow);

if (await result.isCompleted()) {
  console.log('✓ Workflow completed successfully');
  const outputs = await result.getAllOutputs();
  console.log('Outputs:', outputs);
} else if (await result.isFailed()) {
  console.error('✗ Workflow failed');
  const error = await result.error();
  console.error('Error:', error);
}
```

## Common Use Cases

### Text Processing

```typescript
import { DocumentLoader, TextSplitter } from 'graphbit';

// Load a document
const loader = new DocumentLoader();
const doc = await loader.loadFile('./document.pdf');

// Split into chunks
const splitter = TextSplitter.recursive(1000, 200);
const chunks = await splitter.split(doc.content);

console.log(`Split into ${chunks.length} chunks`);
```

### Generate Embeddings

```typescript
import { EmbeddingConfig, EmbeddingClient } from 'graphbit';

const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY, 'text-embedding-3-small');

const client = new EmbeddingClient(config);
const response = await client.embed(['First text to embed', 'Second text to embed']);

console.log(`Generated ${response.embeddings.length} embeddings`);
console.log(`Dimensions: ${response.embeddings[0].length}`);
```

### Create an Agent

```typescript
import { AgentBuilder } from 'graphbit';

const agent = new AgentBuilder('Code Assistant')
  .description('An agent that helps with coding tasks')
  .systemPrompt('You are an expert programmer. Help users with their code.')
  .llmConfig(llmConfig)
  .addCapability('CodeGeneration')
  .addCapability('TextGeneration')
  .temperature(0.3)
  .maxTokens(2000)
  .build();

console.log('Agent created:', await agent.id());
```

## Complete Example

```typescript
import {
  init,
  LlmConfig,
  WorkflowBuilder,
  Executor,
  TextSplitter,
  DocumentLoader,
} from 'graphbit';

async function main() {
  // Initialize
  init();

  // Configure LLM
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini',
  });

  // Load and process document
  const loader = new DocumentLoader();
  const doc = await loader.loadText('Sample document content');

  const splitter = TextSplitter.sentence(3);
  const chunks = await splitter.split(doc.content);

  console.log(`Processed ${chunks.length} chunks`);

  // Create and execute workflow
  const workflow = new WorkflowBuilder('Document Processing')
    .description('Process and analyze documents')
    .build();

  const executor = new Executor(llmConfig);
  const result = await executor.execute(workflow);

  if (await result.isCompleted()) {
    console.log('✓ Success!');
  }
}

main().catch(console.error);
```

## Environment Variables

Create a `.env` file:

```bash
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
HUGGINGFACE_API_KEY=hf_...
```

## Next Steps

- 📖 Read the [API Reference](./docs/API.md)
- 💡 Explore [Examples](./examples/README.md)
- 🧪 Check out the [Tests](./tests/) for more usage patterns
- 🤝 Read the [Contributing Guide](./docs/CONTRIBUTING.md)

## Getting Help

- 📚 [Documentation](https://docs.graphbit.ai)
- 💬 [Discord Community](https://discord.gg/graphbit)
- 🐛 [GitHub Issues](https://github.com/InfinitiBit/graphbit/issues)

## Common Issues

### "Cannot find module 'graphbit'"

Make sure you've built the bindings:

```bash
npm run build
```

### "API key not found"

Set your API keys as environment variables or in a `.env` file.

### TypeScript errors

Run type checking:

```bash
npm run typecheck
```

## Performance Tips

1. **Reuse LLM configs** - Create once, use multiple times
2. **Batch embeddings** - Process multiple texts in one call
3. **Use appropriate chunk sizes** - Balance between context and performance
4. **Enable debug mode** - Only during development
5. **Set timeouts** - Prevent hanging workflows

Happy coding! 🚀
