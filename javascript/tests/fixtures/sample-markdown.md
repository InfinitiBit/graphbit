# GraphBit Documentation

## Introduction

GraphBit is a **powerful workflow orchestration framework** for building AI applications.

## Features

### Core Capabilities

- Multi-agent coordination
- Workflow management
- LLM provider integration
- Document processing
- Text chunking strategies

### Supported Providers

1. **OpenAI** - GPT-4, GPT-3.5
2. **Anthropic** - Claude 3 family
3. **Ollama** - Local models
4. **Azure OpenAI** - Enterprise deployment
5. **Google AI** - Gemini models

## Quick Start

```typescript
import { init, WorkflowBuilder, LlmConfig } from '@graphbit/core';

// Initialize the framework
init();

// Create LLM configuration
const llmConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4'
});

// Build a workflow
const workflow = new WorkflowBuilder('My Workflow')
  .description('A simple workflow')
  .build();
```

## Advanced Usage

For advanced scenarios, refer to the comprehensive API documentation.

## License

MIT License - See LICENSE file for details.

