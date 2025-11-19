# GraphBit JavaScript Examples

This directory contains example code demonstrating how to use the GraphBit JavaScript bindings.

## Prerequisites

Before running the examples, make sure you have:

1. Node.js >= 16.0.0 installed
2. Built the GraphBit JavaScript bindings
3. Set up your API keys (if needed)

## Setup

### 1. Build the Bindings

```bash
cd javascript
npm install
npm run build
```

### 2. Set Up Environment Variables

Create a `.env` file in the `javascript` directory:

```bash
# OpenAI API Key (required for OpenAI examples)
OPENAI_API_KEY=your_openai_api_key_here

# Anthropic API Key (required for Anthropic examples)
ANTHROPIC_API_KEY=your_anthropic_api_key_here

# HuggingFace API Key (optional, for HuggingFace embeddings)
HUGGINGFACE_API_KEY=your_huggingface_api_key_here
```

### 3. Install TypeScript (if not already installed)

```bash
npm install -g tsx
```

## Running Examples

### Basic Workflow

Demonstrates creating and executing a simple workflow:

```bash
tsx examples/basic-workflow.ts
```

**What it shows:**

- Initializing the GraphBit library
- Configuring an LLM provider
- Creating a workflow with metadata
- Executing the workflow
- Checking execution results and statistics

### Text Processing

Demonstrates document loading and text splitting:

```bash
tsx examples/text-processing.ts
```

**What it shows:**

- Different text splitting strategies (character, recursive, sentence, token)
- Loading documents from text
- Loading documents from files
- Processing and chunking documents

### Embeddings

Demonstrates generating embeddings with different providers:

```bash
tsx examples/embeddings.ts
```

**What it shows:**

- Generating embeddings with OpenAI
- Generating embeddings with HuggingFace
- Batch processing of texts
- Calculating similarity between embeddings

**Note:** This example requires API keys to be set.

### Agent Configuration

Demonstrates creating and configuring agents:

```bash
tsx examples/agent-config.ts
```

**What it shows:**

- Creating agents with different capabilities
- Configuring agent parameters
- Using the builder pattern for agent creation

### Multi-Provider Workflow

Demonstrates using multiple LLM providers in one workflow:

```bash
tsx examples/multi-provider.ts
```

**What it shows:**

- Configuring multiple LLM providers
- Switching between providers
- Comparing results from different providers

## Example Structure

Each example follows this structure:

1. **Import** - Import required modules from `@graphbit/core`
2. **Initialize** - Call `init()` to initialize the library
3. **Configure** - Set up LLM providers, agents, or other components
4. **Execute** - Run the workflow or operation
5. **Handle Results** - Process and display results

## Common Patterns

### Error Handling

```typescript
try {
  const result = await executor.execute(workflow);

  if (await result.isFailed()) {
    const error = await result.error();
    console.error('Workflow failed:', error);
  }
} catch (error) {
  console.error('Execution error:', error);
}
```

### Async/Await

All I/O operations in the bindings are asynchronous:

```typescript
const workflow = builder.build();
const name = await workflow.name(); // Note the 'await'
const id = await workflow.id();
```

### Configuration Objects

Use configuration objects for better readability:

```typescript
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini',
  temperature: 0.7,
  maxTokens: 1000,
});
```

## Troubleshooting

### "Cannot find module '@graphbit/core'"

Make sure you've built the bindings:

```bash
cd javascript
npm run build
```

### "API key not found"

Set your API keys in the `.env` file or as environment variables.

### "Module not found" errors

Install dependencies:

```bash
npm install
```

## Next Steps

- Read the [API Documentation](../docs/api/README.md)
- Check the [Migration Guide](../docs/MIGRATION.md)
- Review the [Contributing Guide](../docs/CONTRIBUTING.md)
- Explore the [Test Suite](../tests/) for more examples
