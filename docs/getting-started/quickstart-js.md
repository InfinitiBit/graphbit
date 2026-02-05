# Quick Start Tutorial (JavaScript/TypeScript)

Welcome to GraphBit for JavaScript! This tutorial will guide you through creating your first AI agent in just 5 minutes.

## Prerequisites

Before starting, ensure you have:
- Node.js 16+ installed
- GraphBit installed (`npm install @infinitibit_gmbh/graphbit`)
- An OpenAI API key set in your environment

---

## Your First Agent

Let's create a simple agent that can answer questions.

### Step 1: Basic Setup

Create a new TypeScript file `quickstart.ts`:

```typescript
import 'dotenv/config';
import { init, LlmConfig, AgentBuilder } from '@infinitibit_gmbh/graphbit';

// Initialize the library
init();

// Configure LLM (using OpenAI)
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || '',
  model: 'gpt-4o-mini',
});
```

### Step 2: Create and Build the Agent

```typescript
// Create an agent builder
const builder = new AgentBuilder('Quickstart Agent', config)
  .description('A helpful assistant')
  .systemPrompt('You are a helpful AI assistant. Answer concisely.')
  .temperature(0.7);

// Build the agent
// Note: build() is async
const agent = await builder.build();
```

### Step 3: Execute the Agent

```typescript
async function main() {
  const input = 'What is the capital of France?';
  console.log(`User: ${input}`);

  try {
    // Execute the agent
    const response = await agent.execute(input);
    console.log(`Agent: ${response}`);
  } catch (error) {
    console.error('Execution failed:', error);
  }
}

main().catch(console.error);
```

### Complete Example

Here's the complete working example:

```typescript
import { init, LlmConfig, AgentBuilder } from '@infinitibit_gmbh/graphbit';

async function main() {
  // Initialize
  init();

  // Configure LLM
  const apiKey = process.env.OPENAI_API_KEY;
  if (!apiKey) {
    console.error('Please set OPENAI_API_KEY environment variable');
    return;
  }

  const config = LlmConfig.openai({
    apiKey,
    model: 'gpt-4o-mini',
  });

  // Build Agent
  console.log('Building agent...');
  const agent = await new AgentBuilder('Quickstart Agent', config)
    .description('A helpful assistant')
    .systemPrompt('You are a helpful AI assistant.')
    .build();

  // Execute
  const input = 'Explain quantum computing in one sentence.';
  console.log(`\nUser: ${input}`);
  
  const response = await agent.execute(input);
  console.log(`Agent: ${response}`);
}

main().catch(console.error);
```

### Run Your Agent

```bash
export OPENAI_API_KEY="your-api-key-here"
npx ts-node quickstart.ts
```

Expected output:
```
Building agent...

User: Explain quantum computing in one sentence.
Agent: Quantum computing uses the principles of quantum mechanics to process information in ways that classical computers cannot, enabling faster solutions to complex problems.
```

---

## Working with Different LLM Providers

GraphBit supports multiple LLM providers. You can easily switch between them:

```typescript
// Anthropic
const anthropicConfig = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY || '',
  model: 'claude-3-5-sonnet-20241022',
});

// Ollama (Local)
const ollamaConfig = LlmConfig.ollama({
  model: 'llama2',
  baseUrl: 'http://localhost:11434',
});

// Use the config when building your agent
const agent = await new AgentBuilder('Local Agent', ollamaConfig).build();
```

---

## Next Steps

- Explore [Workflows](../user-guide/workflows-js.md) to connect multiple agents (Coming Soon)
- Learn about [Document Processing](../user-guide/documents-js.md)
