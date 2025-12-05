# LLM Configuration

This document covers the `LlmConfig` class for configuring Large Language Model providers in GraphBit JavaScript bindings.

## Overview

`LlmConfig` provides a unified interface for configuring various LLM providers (OpenAI, Anthropic, Ollama, DeepSeek, etc.). Use the static factory methods to create provider-specific configurations.

## Class: `LlmConfig`

### Static Factory Methods

#### `LlmConfig.openai(options)`

Create OpenAI provider configuration.

**Signature:**

```typescript
static openai(options: OpenAiOptions): LlmConfig
```

**Parameters:**

- `options.apiKey` (string, required): Your OpenAI API key
- `options.model` (string, optional): Model name (default: `"gpt-4o-mini"`)

**Returns:** `LlmConfig` instance

### 🟢 Verified Example

```javascript
const { LlmConfig } = require('graphbit');

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});

// With default model
const configDefault = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY
}); // Uses gpt-4o-mini by default
```

**Differences from Python:**

- Python: `LlmConfig.openai(api_key, model=None)` - positional arguments
- JavaScript: `LlmConfig.openai({ apiKey, model })` - object parameter
- Both support default models

---

#### `LlmConfig.anthropic(options)`

Create Anthropic (Claude) provider configuration.

**Signature:**

```typescript
static anthropic(options: AnthropicOptions): LlmConfig
```

**Parameters:**

- `options.apiKey` (string, required): Your Anthropic API key
- `options.model` (string, optional): Model name (default: `"claude-3-5-sonnet-20241022"`)

**Returns:** `LlmConfig` instance

### 🟢 Verified Example

```javascript
const config = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
  model: 'claude-3-5-sonnet-20241022'
});
```

---

#### `LlmConfig.ollama(options)`

Create Ollama (local models) provider configuration.

**Signature:**

```typescript
static ollama(options: OllamaOptions): LlmConfig
```

**Parameters:**

- `options.model` (string, required): Model name (e.g., `"llama3.2"`, `"mistral"`)
- `options.baseUrl` (string, optional): Ollama server URL (default: `"http://localhost:11434"`)

**Returns:** `LlmConfig` instance

### 🟢 Verified Example

```javascript
const config = LlmConfig.ollama({
  model: 'llama3.2',
  baseUrl: 'http://localhost:11434'
});

// With default URL
const configLocal = LlmConfig.ollama({
  model: 'mistral'
}); // Uses localhost:11434
```

---

#### `LlmConfig.deepseek(options)`

Create DeepSeek provider configuration.

**Signature:**

```typescript
static deepseek(options: DeepSeekOptions): LlmConfig
```

**Parameters:**

- `options.apiKey` (string, required): Your DeepSeek API key
- `options.model` (string, required): Model name

**Available Models:**

- `deepseek-chat` - General conversation and instruction following
- `deepseek-coder` - Specialized for code generation and programming tasks
- `deepseek-reasoner` - Advanced reasoning and mathematical problem solving

**Returns:** `LlmConfig` instance

### 🟢 Verified Example

```javascript
const config = LlmConfig.deepseek({
  apiKey: process.env.DEEPSEEK_API_KEY,
  model: 'deepseek-chat'
});
```

---

## Additional Providers

All providers follow the same pattern. Here are the available factory methods:

| Provider | Method | Required Keys | Default Model |
|----------|--------|---------------|---------------|
| OpenAI | `LlmConfig.openai()` | `apiKey` | `gpt-4o-mini` |
| Anthropic | `LlmConfig.anthropic()` | `apiKey` | `claude-3-5-sonnet-20241022` |
| Ollama | `LlmConfig.ollama()` | `model` | N/A |
| Azure OpenAI | `LlmConfig.azureOpenai()` | `apiKey`, `deploymentName`, `endpoint` | N/A |
| DeepSeek | `LlmConfig.deepseek()` | `apiKey`, `model` | N/A |
| MistralAI | `LlmConfig.mistralai()` | `apiKey`, `model` | N/A |
| HuggingFace | `LlmConfig.huggingface()` | `apiKey`, `model` | N/A |
| Perplexity | `LlmConfig.perplexity()` | `apiKey`, `model` | N/A |
| OpenRouter | `LlmConfig.openrouter()` | `apiKey`, `model` | N/A |
| Fireworks | `LlmConfig.fireworks()` | `apiKey`, `model` | N/A |
| Replicate | `LlmConfig.replicate()` | `apiKey`, `model` | N/A |
| TogetherAI | `LlmConfig.togetherai()` | `apiKey`, `model` | N/A |
| xAI (Grok) | `LlmConfig.xai()` | `apiKey`, `model` | N/A |
| AI21 | `LlmConfig.ai21()` | `apiKey`, `model` | N/A |
| ByteDance | `LlmConfig.bytedance()` | `apiKey`, `model` | N/A |

---

## Usage Patterns

### Environment Variables

```javascript
const { LlmConfig } = require('graphbit');
require('dotenv').config();

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});
```

### Multiple Providers

```javascript
const openaiConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY
});

const claudeConfig = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY
});

// Use different configs for different workflows
const fastLlm = LlmConfig.ollama({ model: 'llama3.2' });
const powerfulLlm = LlmConfig.openai({ 
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o'
});
```

---

## Best Practices

1. **Store API keys securely**: Never hardcode API keys - use environment variables
2. **Choose appropriate models**: Balance cost, speed, and quality for your use case
3. **Local development**: Use Ollama for free local testing before deploying with cloud providers
4. **Model selection**: Smaller models (gpt-4o-mini, llama3.2) for simple tasks; larger models for complex reasoning

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Parameter style** | Positional: `LlmConfig.openai(api_key, model)` | Object: `LlmConfig.openai({ apiKey, model })` |
| **Naming** | Snake case: `api_key` | Camel case: `apiKey` |
| **Instance methods** | `config.provider()`, `config.model()` | Not exposed (use config directly) |

---

## Next Steps

After creating a config, use it with:

- [LlmClient](./llm-client.md) - For direct LLM interactions
- [Executor](./executor.md) - For workflow execution

---

## Related Documentation

- [Core Functions](./core-functions.md) - Initialize GraphBit
- [LLM Client](./llm-client.md) - Execute LLM requests
- [Workflow Management](./workflow.md) - Use LLMs in workflows
