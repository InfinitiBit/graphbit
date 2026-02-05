# LLM Providers (JavaScript/TypeScript)

GraphBit supports multiple Large Language Model providers through a unified configuration interface. This guide covers configuration for each supported provider.

## Supported Providers

GraphBit supports these LLM providers:
- **OpenAI** - GPT models including GPT-4o
- **Azure OpenAI** - Enterprise-grade OpenAI models
- **Anthropic** - Claude models
- **MistralAI** - Mistral models
- **ByteDance ModelArk** - ByteDance Seed models
- **OpenRouter** - Unified access to 400+ models
- **Perplexity** - Search-enabled models
- **DeepSeek** - High-performance models
- **TogetherAI** - Open-source models
- **Fireworks AI** - Fast inference
- **Replicate** - Open-source models
- **xAI** - Grok models
- **Ollama** - Local model execution

## Configuration

All configurations are created using the `LlmConfig` class.

### OpenAI Configuration

```typescript
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || '',
  model: 'gpt-4o-mini' // Optional - defaults to gpt-4o-mini
});
```

### Azure OpenAI Configuration

```typescript
const config = LlmConfig.azureOpenai({
  apiKey: process.env.AZURE_OPENAI_API_KEY || '',
  deploymentName: 'gpt-4o-mini',
  endpoint: 'https://your-resource.openai.azure.com',
  apiVersion: '2024-10-21' // Optional
});
```

### Anthropic Configuration

```typescript
const config = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY || '',
  model: 'claude-3-5-sonnet-20241022'
});
```

### MistralAI Configuration

> [!IMPORTANT]
> The `model` parameter is **required** for MistralAI.

```typescript
const config = LlmConfig.mistralai({
  apiKey: process.env.MISTRALAI_API_KEY || '',
  model: 'mistral-large-latest'
});
```

### Ollama Configuration (Local)

> [!IMPORTANT]
> The `model` parameter is **required** for Ollama. Make sure the model is pulled locally with `ollama pull <model-name>` before use.

```typescript
const config = LlmConfig.ollama({
  model: 'llama2',
  baseUrl: 'http://localhost:11434' // Optional
});
```

### OpenRouter Configuration

> [!IMPORTANT]
> The `model` parameter is **required** for OpenRouter. Model format is `provider/model-name`.

```typescript
const config = LlmConfig.openrouter({
  apiKey: process.env.OPENROUTER_API_KEY || '',
  model: 'openai/gpt-4o'
});
```

### Other Providers

#### ByteDance ModelArk
```typescript
const config = LlmConfig.bytedance({
  apiKey: process.env.BYTEDANCE_API_KEY || '',
  model: 'seed-1-6-250915'
});
```

#### DeepSeek

> [!IMPORTANT]
> The `model` parameter is **required** for DeepSeek.

```typescript
const config = LlmConfig.deepseek({
  apiKey: process.env.DEEPSEEK_API_KEY || '',
  model: 'deepseek-chat'
});
```

#### Perplexity

> [!IMPORTANT]
> The `model` parameter is **required** for Perplexity.

```typescript
const config = LlmConfig.perplexity({
  apiKey: process.env.PERPLEXITY_API_KEY || '',
  model: 'llama-3.1-sonar-small-128k-online'
});
```

#### TogetherAI
```typescript
const config = LlmConfig.togetherai({
  apiKey: process.env.TOGETHER_API_KEY || '',
  model: 'openai/gpt-oss-20b'
});
```

#### Fireworks AI
```typescript
const config = LlmConfig.fireworks({
  apiKey: process.env.FIREWORKS_API_KEY || '',
  model: 'accounts/fireworks/models/llama-v3p1-8b-instruct'
});
```

#### Replicate
```typescript
const config = LlmConfig.replicate({
  apiKey: process.env.REPLICATE_API_KEY || '',
  model: 'meta/llama-2-70b-chat'
});
```

#### xAI (Grok)
```typescript
const config = LlmConfig.xai({
  apiKey: process.env.XAI_API_KEY || '',
  model: 'grok-beta'
});
```

#### AI21
```typescript
const config = LlmConfig.ai21({
  apiKey: process.env.AI21_API_KEY || '',
  model: 'jamba-1.5-large'
});
```

#### HuggingFace
```typescript
const config = LlmConfig.huggingface({
  apiKey: process.env.HUGGINGFACE_API_KEY || '',
  model: 'meta-llama/Llama-2-7b-chat-hf'
});
```

## Usage

Once configured, pass the `config` object to `AgentBuilder` or `Executor`.

```typescript
import { AgentBuilder } from '@infinitibit_gmbh/graphbit';

const agent = await new AgentBuilder('My Agent', config)
  .build();
```
