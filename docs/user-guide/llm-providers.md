# LLM Providers

GraphBit supports multiple Large Language Model providers through a unified client interface. This guide covers configuration, usage, and optimization for each supported provider.

## Supported Providers

GraphBit supports these LLM providers:
- **OpenAI** - GPT models including GPT-4o, GPT-4o-mini
- **Azure OpenAI** - GPT models hosted on Microsoft Azure with enterprise features
- **Anthropic** - Claude models including Claude-4-Sonnet
- **OpenRouter** - Unified access to 400+ models from multiple providers (GPT, Claude, Mistral, etc.)
- **Perplexity** - Real-time search-enabled models including Sonar models
- **DeepSeek** - High-performance models including DeepSeek-Chat, DeepSeek-Coder, and DeepSeek-Reasoner
- **TogetherAI** - Access to open-source models including GPT-OSS, Kimi, and Qwen with competitive pricing
- **Fireworks AI** - Fast inference for open-source models including Llama, Mixtral, and Qwen
- **Replicate** - Access to open-source models with function calling support including Glaive, Hermes, and Granite models
- **xAI** - Grok models with real-time information and advanced reasoning capabilities
- **Ollama** - Local model execution with various open-source models

## Configuration

### OpenAI Configuration

Configure OpenAI provider with API key and model selection:

```python
import os

from graphbit import LlmConfig

# Basic OpenAI configuration
config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"  # Optional - defaults to gpt-4o-mini
)

# Access configuration details
print(f"Provider: {config.provider()}")  # "OpenAI"
print(f"Model: {config.model()}")        # "gpt-4o-mini"
```

#### Available OpenAI Models

| Model | Best For | Context Length | Performance |
|-------|----------|----------------|-------------|
| `gpt-4o` | Complex reasoning, latest features | 128K | High quality, slower |
| `gpt-4o-mini` | Balanced performance and cost | 128K | Good quality, faster |

```python
# Model selection examples
creative_config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o"  # For creative and complex tasks
)

production_config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"  # Balanced for production
)

```

### Azure OpenAI Configuration

Azure OpenAI provides enterprise-grade access to OpenAI models through Microsoft Azure, offering enhanced security, compliance, and regional availability.

```python
import os

from graphbit import LlmConfig

# Basic Azure OpenAI configuration
config = LlmConfig.azure_openai(
    api_key=os.getenv("AZURE_OPENAI_API_KEY"),
    deployment_name="gpt-4o-mini",  # Your Azure deployment name
    endpoint="https://your-resource.openai.azure.com"  # Your Azure OpenAI endpoint
)

print(f"Provider: {config.provider()}")  # "azure_openai"
print(f"Model: {config.model()}")        # "gpt-4o-mini"
```

#### Azure OpenAI with Custom API Version

```python
# Configuration with specific API version
config = LlmConfig.azure_openai(
    api_key=os.getenv("AZURE_OPENAI_API_KEY"),
    deployment_name="gpt-4o",
    endpoint="https://your-resource.openai.azure.com",
    api_version="2024-10-21"  # Optional - defaults to "2024-10-21"
)
```

#### Azure OpenAI Setup Requirements

To use Azure OpenAI, you need:

1. **Azure OpenAI Resource**: Create an Azure OpenAI resource in the Azure portal
2. **Model Deployment**: Deploy a model (e.g., GPT-4o, GPT-4o-mini) in your resource
3. **API Key**: Get your API key from the Azure portal
4. **Endpoint URL**: Your resource endpoint (format: `https://{resource-name}.openai.azure.com`)

#### Environment Variables

Set these environment variables for Azure OpenAI:

```bash
export AZURE_OPENAI_API_KEY="your-azure-openai-api-key"
export AZURE_OPENAI_ENDPOINT="https://your-resource.openai.azure.com"
export AZURE_OPENAI_DEPLOYMENT="your-deployment-name"
export AZURE_OPENAI_API_VERSION="2024-10-21"  # Optional
```

```python
# Using environment variables
config = LlmConfig.azure_openai(
    api_key=os.getenv("AZURE_OPENAI_API_KEY"),
    deployment_name=os.getenv("AZURE_OPENAI_DEPLOYMENT"),
    endpoint=os.getenv("AZURE_OPENAI_ENDPOINT"),
    api_version=os.getenv("AZURE_OPENAI_API_VERSION", "2024-10-21")
)
```

#### Available Azure OpenAI Models

| Model | Best For | Context Length | Performance |
|-------|----------|----------------|-------------|
| `gpt-4o` | Complex reasoning, latest features | 128K | High quality, slower |
| `gpt-4o-mini` | Balanced performance and cost | 128K | Good quality, faster |
| `gpt-4-turbo` | Advanced tasks, function calling | 128K | High quality, moderate speed |
| `gpt-4` | Complex analysis, creative tasks | 8K | High quality, slower |
| `gpt-3.5-turbo` | General tasks, cost-effective | 16K | Good quality, fast |

```python
# Model selection examples for Azure OpenAI
premium_config = LlmConfig.azure_openai(
    api_key=os.getenv("AZURE_OPENAI_API_KEY"),
    deployment_name="gpt-4o",  # For complex reasoning and latest features
    endpoint=os.getenv("AZURE_OPENAI_ENDPOINT")
)

balanced_config = LlmConfig.azure_openai(
    api_key=os.getenv("AZURE_OPENAI_API_KEY"),
    deployment_name="gpt-4o-mini",  # Balanced performance and cost
    endpoint=os.getenv("AZURE_OPENAI_ENDPOINT")
)

cost_effective_config = LlmConfig.azure_openai(
    api_key=os.getenv("AZURE_OPENAI_API_KEY"),
    deployment_name="gpt-3.5-turbo",  # Cost-effective for general tasks
    endpoint=os.getenv("AZURE_OPENAI_ENDPOINT")
)
```

### Anthropic Configuration

Configure Anthropic provider for Claude models:

```python
# Basic Anthropic configuration
config = LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-sonnet-4-20250514"  # Optional - defaults to claude-sonnet-4-20250514
)

print(f"Provider: {config.provider()}")  # "Anthropic"
print(f"Model: {config.model()}")        # "claude-sonnet-4-20250514"
```

#### Available Anthropic Models

| Model | Best For | Context Length | Speed |
|-------|----------|----------------|-------|
| `claude-opus-4-1-20250805` | Most capable, complex analysis | 200K | Medium speed, highest quality |
| `claude-sonnet-4-20250514` | Balanced performance | 200K/1M | Slow speed and good quality |
| `claude-3-haiku-20240307` | Fast, cost-effective | 200K | Fastest, good quality |

```python
# Model selection for different use cases
complex_config = LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-opus-4-1-20250805"  # For complex analysis
)

balanced_config = LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-sonnet-4-20250514"  # For balanced workloads
)

fast_config = LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-3-haiku-20240307"  # For speed and efficiency
)
```

### OpenRouter Configuration

OpenRouter provides unified access to 400+ AI models through a single API, including models from OpenAI, Anthropic, Google, Meta, Mistral, and many others. This allows you to easily switch between different models and providers without changing your code.

```python
import os

from graphbit import LlmConfig

# Basic OpenRouter configuration
config = LlmConfig.openrouter(
    api_key=os.getenv("OPENROUTER_API_KEY"),
    model="openai/gpt-4o-mini"  # Optional - defaults to openai/gpt-4o-mini
)

print(f"Provider: {config.provider()}")  # "openrouter"
print(f"Model: {config.model()}")        # "openai/gpt-4o-mini"
```

#### Popular OpenRouter Models

| Model | Provider | Best For | Context Length |
|-------|----------|----------|----------------|
| `openai/gpt-4o` | OpenAI | Complex reasoning, latest features | 128K |
| `openai/gpt-4o-mini` | OpenAI | Balanced performance and cost | 128K |
| `anthropic/claude-3-5-sonnet` | Anthropic | Advanced reasoning, coding | 200K |
| `anthropic/claude-3-5-haiku` | Anthropic | Fast responses, simple tasks | 200K |
| `google/gemini-pro-1.5` | Google | Large context, multimodal | 1M |
| `meta-llama/llama-3.1-405b-instruct` | Meta | Open source, high performance | 131K |
| `mistralai/mistral-large` | Mistral | Multilingual, reasoning | 128K |

```python
# Model selection examples
openai_config = LlmConfig.openrouter(
    api_key=os.getenv("OPENROUTER_API_KEY"),
    model="openai/gpt-4o"  # Access OpenAI models through OpenRouter
)

claude_config = LlmConfig.openrouter(
    api_key=os.getenv("OPENROUTER_API_KEY"),
    model="anthropic/claude-3-5-sonnet"  # Access Claude models
)

llama_config = LlmConfig.openrouter(
    api_key=os.getenv("OPENROUTER_API_KEY"),
    model="meta-llama/llama-3.1-405b-instruct"  # Access open source models
)
```

#### OpenRouter with Site Information

For better rankings and analytics on OpenRouter, you can provide your site information:

```python
# Configuration with site information for OpenRouter rankings
config = LlmConfig.openrouter_with_site(
    api_key=os.getenv("OPENROUTER_API_KEY"),
    model="openai/gpt-4o-mini",
    site_url="https://graphbit.ai",  # Optional - your site URL
    site_name="GraphBit AI Framework"  # Optional - your site name
)
```

### Perplexity Configuration

Configure Perplexity provider to access real-time search-enabled models:

```python
# Basic Perplexity configuration
config = LlmConfig.perplexity(
    api_key=os.getenv("PERPLEXITY_API_KEY"),
    model="sonar"  # Optional - defaults to sonar
)

print(f"Provider: {config.provider()}")  # "perplexity"
print(f"Model: {config.model()}")        # "sonar"
```

#### Available Perplexity Models

| Model | Best For | Context Length | Special Features |
|-------|----------|----------------|------------------|
| `sonar` | General purpose with search | 128K | Real-time web search, citations |
| `sonar-reasoning` | Complex reasoning with search | 128K | Multi-step reasoning, web research |
| `sonar-deep-research` | Comprehensive research | 128K | Exhaustive research, detailed analysis |

```python
# Model selection for different use cases
research_config = LlmConfig.perplexity(
    api_key=os.getenv("PERPLEXITY_API_KEY"),
    model="sonar-deep-research"  # For comprehensive research
)

reasoning_config = LlmConfig.perplexity(
    api_key=os.getenv("PERPLEXITY_API_KEY"),
    model="sonar-reasoning"  # For complex problem solving
)
```

### DeepSeek Configuration

Configure DeepSeek provider for high-performance, cost-effective AI models:

```python
# Basic DeepSeek configuration
config = LlmConfig.deepseek(
    api_key=os.getenv("DEEPSEEK_API_KEY"),
    model="deepseek-chat"  # Optional - defaults to deepseek-chat
)

print(f"Provider: {config.provider()}")  # "deepseek"
print(f"Model: {config.model()}")        # "deepseek-chat"
```

#### Available DeepSeek Models

| Model | Best For | Context Length | Performance | Cost |
|-------|----------|----------------|-------------|------|
| `deepseek-chat` | General conversation, instruction following | 128K | High quality, fast | 
| `deepseek-coder` | Code generation, programming tasks | 128K | Specialized for code | 
| `deepseek-reasoner` | Complex reasoning, mathematics | 128K | Advanced reasoning | 

```python
# Model selection for different use cases
general_config = LlmConfig.deepseek(
    api_key=os.getenv("DEEPSEEK_API_KEY"),
    model="deepseek-chat"  # For general tasks and conversation
)

coding_config = LlmConfig.deepseek(
    api_key=os.getenv("DEEPSEEK_API_KEY"),
    model="deepseek-coder"  # For code generation and programming
)

reasoning_config = LlmConfig.deepseek(
    api_key=os.getenv("DEEPSEEK_API_KEY"),
    model="deepseek-reasoner"  # For complex reasoning tasks
)
```

### TogetherAI Configuration

Configure TogetherAI provider for access to open-source models with competitive pricing:

```python
# Basic TogetherAI configuration
config = LlmConfig.togetherai(
    api_key=os.getenv("TOGETHER_API_KEY"),
    model="openai/gpt-oss-20b"  # Optional - defaults to openai/gpt-oss-20b
)

# Access configuration details
print(f"Provider: {config.provider()}")  # "togetherai"
print(f"Model: {config.model()}")        # "openai/gpt-oss-20b"
```

#### Available TogetherAI Models

| Model | Best For | Context Length | Cost (per 1M tokens) |
|-------|----------|----------------|---------------------|
| `openai/gpt-oss-20b` | General tasks, fast inference | 8K | $0.50 / $0.50 |
| `moonshotai/Kimi-K2-Instruct-0905` | Long documents, high context | 200K | $1.00 / $1.00 |
| `Qwen/Qwen3-Next-80B-A3B-Instruct` | Complex reasoning, most capable | 32K | $2.00 / $2.00 |

```python
# Model selection examples
fast_config = LlmConfig.togetherai(
    api_key=os.getenv("TOGETHER_API_KEY"),
    model="openai/gpt-oss-20b"  # Fast and cost-effective
)

long_context_config = LlmConfig.togetherai(
    api_key=os.getenv("TOGETHER_API_KEY"),
    model="moonshotai/Kimi-K2-Instruct-0905"  # For long documents
)

capable_config = LlmConfig.togetherai(
    api_key=os.getenv("TOGETHER_API_KEY"),
    model="Qwen/Qwen3-Next-80B-A3B-Instruct"  # Most capable
)
```

#### TogetherAI Features

- ✅ **Function Calling**: All models support function/tool calling
- ✅ **Streaming**: Real-time response streaming
- ✅ **Cost Estimation**: Built-in cost tracking per token
- ✅ **Context Detection**: Automatic context length detection
- ✅ **Async Support**: Full async/await compatibility

#### Getting Started with TogetherAI

1. **Sign up**: Create an account at [TogetherAI](https://api.together.xyz/)
2. **Get API Key**: Generate your API key from the dashboard
3. **Set Environment Variable**: `export TOGETHER_API_KEY="your-key"`
4. **Start Building**: Use in your GraphBit workflows

```python
# Complete example
import os
from graphbit import LlmConfig, Executor, Workflow, Node

# Configure TogetherAI
config = LlmConfig.togetherai(
    api_key=os.getenv("TOGETHER_API_KEY"),
    model="openai/gpt-oss-20b"
)

# Create client and generate text
client = LlmClient(config)
response = client.complete(
    prompt="Explain quantum computing in simple terms",
    max_tokens=200,
    temperature=0.7
)

print(response)
```

### Fireworks AI Configuration

Configure Fireworks AI for fast inference with open-source models:

```python
import os

from graphbit import LlmConfig

# Basic Fireworks AI configuration
config = LlmConfig.fireworks(
    api_key=os.getenv("FIREWORKS_API_KEY"),
    model="accounts/fireworks/models/llama-v3p1-8b-instruct"  # Optional - defaults to llama-v3p1-8b-instruct
)

print(f"Provider: {config.provider()}")  # "fireworks"
print(f"Model: {config.model()}")        # "accounts/fireworks/models/llama-v3p1-8b-instruct"
```

#### Popular Fireworks AI Models

| Model | Best For | Context Length | Performance | Cost |
|-------|----------|----------------|-------------|------|
| `accounts/fireworks/models/llama-v3p1-8b-instruct` | General tasks, fast inference | 131K | Fast, efficient | Very low |
| `accounts/fireworks/models/llama-v3p1-70b-instruct` | Complex reasoning, high quality | 131K | High quality | Low |
| `accounts/fireworks/models/deepseek-v3p1` | Most complex tasks | 131K | Highest quality | Medium |
| `accounts/fireworks/models/kimi-k2-instruct-0905` | Multilingual, code generation | 32K | Balanced | Low |
| `accounts/fireworks/models/qwen3-coder-480b-a35b-instruct` | Reasoning, mathematics | 32K | High quality | Low |

```python
# Model selection for different use cases
fast_config = LlmConfig.fireworks(
    api_key=os.getenv("FIREWORKS_API_KEY"),
    model="accounts/fireworks/models/llama-v3p1-8b-instruct"  # For fast, efficient tasks
)

quality_config = LlmConfig.fireworks(
    api_key=os.getenv("FIREWORKS_API_KEY"),
    model="accounts/fireworks/models/llama-v3p1-70b-instruct"  # For high-quality responses
)

coding_config = LlmConfig.fireworks(
    api_key=os.getenv("FIREWORKS_API_KEY"),
    model="accounts/fireworks/models/mixtral-8x7b-instruct"  # For code generation
)
```

#### Getting Started with Fireworks AI

1. **Sign up** at [fireworks.ai](https://fireworks.ai)
2. **Get your API key** from the dashboard
3. **Set environment variable**: `export FIREWORKS_API_KEY="your-api-key"`
4. **Start using** with GraphBit

```python
import os
from graphbit import LlmClient, LlmConfig

# Create configuration
config = LlmConfig.fireworks(
    api_key=os.getenv("FIREWORKS_API_KEY"),
    model="accounts/fireworks/models/llama-v3p1-8b-instruct"
)

# Create client and generate text
client = LlmClient(config)
response = client.complete(
    prompt="Explain quantum computing in simple terms",
    max_tokens=200,
    temperature=0.7
)

print(response)
```

### Replicate Configuration

Configure Replicate for access to open-source models with function calling capabilities:

```python
import os

from graphbit import LlmConfig

# Basic Replicate configuration
config = LlmConfig.replicate(
    api_key=os.getenv("REPLICATE_API_KEY"),
    model="anthropic/claude-4-sonnet"  # openai/gpt-5
)

print(f"Provider: {config.provider()}")  # "replicate"
print(f"Model: {config.model()}")        # "anthropic/claude-4-sonnet"
```

#### Provide Model Version

You can also provide version separately:

```python

# Configuration with specific model version
config = LlmConfig.replicate(
    api_key=os.getenv("REPLICATE_API_KEY"),
    model="lucataco/dolphin-2.9-llama3-8b",
    version="ee173688d3b8d9e05a5b910f10fb9bab1e9348963ab224579bb90d9fce3fb00b"
)
```

#### Getting Started with Replicate

1. **Sign up** at [replicate.com](https://replicate.com)
2. **Get your API token** from your account settings
3. **Set environment variable**: `export REPLICATE_API_KEY="your-api-token"`
4. **Start using** with GraphBit

```python
import os
from graphbit import LlmClient, LlmConfig

# Create configuration
config = LlmConfig.replicate(
    api_key=os.getenv("REPLICATE_API_KEY"),
    model="lucataco/glaive-function-calling-v1"
)

# Create client and generate text
client = LlmClient(config)
response = client.complete(
    prompt="Explain the benefits of open-source AI models",
    max_tokens=300,
    temperature=0.7
)

print(response)
```

### xAI Configuration

Configure xAI for Grok models with real-time information and advanced reasoning:

```python
import os

from graphbit import LlmConfig

# Basic xAI configuration
config = LlmConfig.xai(
    api_key=os.getenv("XAI_API_KEY"),
    model="grok-4"  # Optional - defaults to grok-4
)

print(f"Provider: {config.provider()}")  # "xai"
print(f"Model: {config.model()}")        # "grok-4"
```

#### Popular xAI Grok Models

| Model | Best For | Context Length | Performance | Cost |
|-------|----------|----------------|-------------|------|
| `grok-4` | Complex reasoning, latest features | 256K | Highest quality | Medium |
| `grok-4-0709` | Stable version of Grok-4 | 256K | High quality | Medium |
| `grok-code-fast-1` | Code generation, fast inference | 256K | Fast, efficient | Very low |
| `grok-3` | General tasks, balanced performance | 131K | Good quality | Medium |
| `grok-3-mini` | Quick tasks, cost-effective | 131K | Fast, efficient | Very low |

```python
# Model selection for different use cases
reasoning_config = LlmConfig.xai(
    api_key=os.getenv("XAI_API_KEY"),
    model="grok-4"  # For complex reasoning and latest features
)

coding_config = LlmConfig.xai(
    api_key=os.getenv("XAI_API_KEY"),
    model="grok-code-fast-1"  # For fast code generation
)

efficient_config = LlmConfig.xai(
    api_key=os.getenv("XAI_API_KEY"),
    model="grok-3-mini"  # For cost-effective tasks
)
```

#### Getting Started with xAI

1. **Sign up** at [x.ai](https://x.ai)
2. **Get your API key** from the developer console
3. **Set environment variable**: `export XAI_API_KEY="your-api-key"`
4. **Start using** with GraphBit

```python
import os
from graphbit import LlmClient, LlmConfig

# Create configuration
config = LlmConfig.xai(
    api_key=os.getenv("XAI_API_KEY"),
    model="grok-4"
)

# Create client and generate text
client = LlmClient(config)
response = client.complete(
    prompt="Explain quantum computing with real-time examples",
    max_tokens=200,
    temperature=0.7
)

print(response)
```

### AI21 Labs Configuration

Configure AI21 Labs for Jamba models with real-time information and advanced reasoning:

```python
import os

from graphbit import LlmConfig

# Basic AI21 configuration
config = LlmConfig.ai21(
    api_key=os.getenv("AI21_API_KEY"),
    model="jamba-mini"  # Optional - defaults to jamba-mini
)

print(f"Provider: {config.provider()}")  # "ai21"
print(f"Model: {config.model()}")        # "jamba-mini"
```

#### AI21 Jamba Models

| Model | Best For | Context Length | Performance | Cost |
|-------|----------|----------------|-------------|------|
| `jamba-mini` | General tasks, cost-effective | 256K | Highest quality | Medium |
| `jamba-large` | General tasks, balanced performance | 256K | High quality | Medium |

```python
# Model selection for different use cases
config = LlmConfig.ai21(
    api_key=os.getenv("AI21_API_KEY"),
    model="jamba-mini"  
)

config = LlmConfig.ai21(
    api_key=os.getenv("AI21_API_KEY"),
    model="jamba-large"  
)
```

#### Getting Started with AI21 Labs

1. **Sign up** at [AI21 Labs](https://ai21labs.com)
2. **Get your API key** from the developer console
3. **Set environment variable**: `export AI21_API_KEY="your-api-key"`
4. **Start using** with GraphBit

```python
import os
from graphbit import LlmClient, LlmConfig

# Create configuration
config = LlmConfig.xai(
    api_key=os.getenv("AI21_API_KEY"),
    model="jamba-mini"
)

# Create client and generate text
client = LlmClient(config)
response = client.complete(
    prompt="Explain quantum computing with real-time examples",
    max_tokens=200,
    temperature=0.7
)

print(response)
```

### Ollama Configuration

Configure Ollama for local model execution:

```python
# Basic Ollama configuration (no API key required)
config = LlmConfig.ollama(
    model="llama3.2"  # Optional - defaults to llama3.2
)

print(f"Provider: {config.provider()}")  # "Ollama"
print(f"Model: {config.model()}")        # "llama3.2"

# Other popular models
mistral_config = LlmConfig.ollama(model="mistral")
codellama_config = LlmConfig.ollama(model="codellama")
phi_config = LlmConfig.ollama(model="phi")
```

## LLM Client Usage

### Creating and Using Clients

```python
from graphbit import LlmClient

# Create client with configuration
client = LlmClient(config, debug=False)

# Basic text completion
response = client.complete(
    prompt="Explain the concept of machine learning",
    max_tokens=500,     # Optional - controls response length
    temperature=0.7     # Optional - controls randomness (0.0-1.0)
)

print(f"Response: {response}")
```

### Asynchronous Operations

GraphBit provides async methods for non-blocking operations:

```python
import asyncio

async def async_completion():
    # Async completion
    response = await client.complete_async(
        prompt="Write a short story about AI",
        max_tokens=300,
        temperature=0.8
    )
    return response

# Run async operation
response = asyncio.run(async_completion())
```

### Batch Processing

Process multiple prompts efficiently:

```python
async def batch_processing():
    prompts = [
        "Summarize quantum computing",
        "Explain blockchain technology", 
        "Describe neural networks",
        "What is machine learning?"
    ]
    
    responses = await client.complete_batch(
        prompts=prompts,
        max_tokens=200,
        temperature=0.5,
        max_concurrency=3  # Process 3 at a time
    )
    
    for i, response in enumerate(responses):
        print(f"Response {i+1}: {response}")

asyncio.run(batch_processing())
```

### Chat-Style Interactions

Use chat-optimized methods for conversational interactions:

```python
async def chat_example():
    # Chat with message history
    response = await client.chat_optimized(
        messages=[
            ("user", "Hello, how are you?"),
            ("assistant", "I'm doing well, thank you!"),
            ("user", "Can you help me with Python programming?"),
            ("user", "Specifically, how do I handle exceptions?")
        ],
        max_tokens=400,
        temperature=0.3
    )
    
    print(f"Chat response: {response}")

asyncio.run(chat_example())
```

### Streaming Responses

Get real-time streaming responses:

```python
async def streaming_example():
    print("Streaming response:")
    
    async for chunk in client.complete_stream(
        prompt="Tell me a detailed story about space exploration",
        max_tokens=1000,
        temperature=0.7
    ):
        print(chunk, end="", flush=True)
    
    print("\n--- Stream complete ---")

asyncio.run(streaming_example())
```

## Client Management and Monitoring

### Client Statistics

Monitor client performance and usage:

```python
# Get comprehensive statistics
stats = client.get_stats()

print(f"Total requests: {stats['total_requests']}")
print(f"Successful requests: {stats['successful_requests']}")
print(f"Failed requests: {stats['failed_requests']}")
print(f"Average response time: {stats['average_response_time_ms']}ms")
print(f"Circuit breaker state: {stats['circuit_breaker_state']}")
print(f"Client uptime: {stats['uptime']}")

# Calculate success rate
if stats['total_requests'] > 0:
    success_rate = stats['successful_requests'] / stats['total_requests']
    print(f"Success rate: {success_rate:.2%}")
```

### Client Warmup

Pre-initialize connections for better performance:

```python
async def warmup_client():
    # Warmup client to reduce cold start latency
    await client.warmup()
    print("Client warmed up and ready")

# Warmup before production use
asyncio.run(warmup_client())
```

### Reset Statistics

Reset client statistics for monitoring periods:

```python
# Reset statistics
client.reset_stats()
print("Client statistics reset")
```

## Provider-Specific Examples

### OpenAI Workflow Example

```python
import os

from graphbit import LlmConfig, Workflow, Node, Executor

def create_openai_workflow():
    """Create workflow using OpenAI"""
    
    # Configure OpenAI
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Create workflow
    workflow = Workflow("OpenAI Analysis Pipeline")
    
    # Create analyzer node
    analyzer = Node.agent(
        name="GPT Content Analyzer",
        prompt=f"Analyze the following content for sentiment, key themes, and quality:\n\n{input}",
        agent_id="gpt_analyzer"
    )
    
    # Add to workflow
    analyzer_id = workflow.add_node(analyzer)
    workflow.validate()
    
    # Create executor and run
    executor = Executor(config, timeout_seconds=60)
    return workflow, executor

# Usage
workflow, executor = create_openai_workflow()
result = executor.execute(workflow)
```

### Anthropic Workflow Example

```python
import os

from graphbit import LlmConfig, Workflow, Node, Executor

def create_anthropic_workflow():
    """Create workflow using Anthropic Claude"""
    
    # Configure Anthropic
    config = LlmConfig.anthropic(
        api_key=os.getenv("ANTHROPIC_API_KEY"),
        model="claude-sonnet-4-20250514"
    )
    
    # Create workflow
    workflow = Workflow("Claude Analysis Pipeline")
    
    # Create analyzer with detailed prompt
    analyzer = Node.agent(
        name="Claude Content Analyzer",
        prompt=f"""
        Analyze the following content with attention to:
        - Factual accuracy and logical consistency
        - Potential biases or assumptions
        - Clarity and structure
        - Key insights and recommendations
        
        Content: {input}
        
        Provide your analysis in a structured format.
        """,
        agent_id="claude_analyzer"
    )
    
    workflow.add_node(analyzer)
    workflow.validate()
    
    # Create executor with longer timeout for Claude
    executor = Executor(config, timeout_seconds=120)
    return workflow, executor

# Usage
workflow, executor = create_anthropic_workflow()
```

### DeepSeek Workflow Example

```python
import os

from graphbit import LlmConfig, Workflow, Node, Executor

def create_deepseek_workflow():
    """Create workflow using DeepSeek models"""
    
    # Configure DeepSeek
    config = LlmConfig.deepseek(
        api_key=os.getenv("DEEPSEEK_API_KEY"),
        model="deepseek-chat"
    )
    
    # Create workflow
    workflow = Workflow("DeepSeek Analysis Pipeline")
    
    # Create analyzer optimized for DeepSeek's capabilities
    analyzer = Node.agent(
        name="DeepSeek Content Analyzer",
        prompt=f"""
        Analyze the following content efficiently and accurately:
        - Main topics and themes
        - Key insights and takeaways
        - Actionable recommendations
        - Potential concerns or limitations
        
        Content: {input}
        
        Provide a clear, structured analysis.
        """,
        agent_id="deepseek_analyzer"
    )
    
    workflow.add_node(analyzer)
    workflow.validate()
    
    # Create executor optimized for DeepSeek's fast inference
    executor = Executor(config, timeout_seconds=90)
    return workflow, executor

# Usage for different DeepSeek models
def create_deepseek_coding_workflow():
    """Create workflow for code analysis using DeepSeek Coder"""
    
    config = LlmConfig.deepseek(
        api_key=os.getenv("DEEPSEEK_API_KEY"),
        model="deepseek-coder"
    )
    
    workflow = Workflow("DeepSeek Code Analysis")
    
    code_analyzer = Node.agent(
        name="DeepSeek Code Reviewer",
        prompt=f"""
        Review this code for:
        - Code quality and best practices
        - Potential bugs or issues
        - Performance improvements
        - Security considerations
        
        Code: {input}
        """,
        agent_id="deepseek_code_analyzer"
    )
    
    workflow.add_node(code_analyzer)
    workflow.validate()
    
    executor = Executor(config, timeout_seconds=90)
    return workflow, executor

# Usage
workflow, executor = create_deepseek_workflow()
```

### OpenRouter Workflow Example

```python
from graphbit import LlmConfig, Workflow, Node, Executor
import os

def create_openrouter_workflow():
    """Create workflow using OpenRouter with multiple models"""

    # Configure OpenRouter with a high-performance model
    config = LlmConfig.openrouter(
        api_key=os.getenv("OPENROUTER_API_KEY"),
        model="anthropic/claude-3-5-sonnet"  # Use Claude through OpenRouter
    )

    workflow = Workflow("OpenRouter Multi-Model Pipeline")

    # Create analyzer using Claude for complex reasoning
    analyzer = Node.agent(
        name="Claude Content Analyzer",
        prompt=f"""
        Analyze this content comprehensively:
        - Main themes and topics
        - Sentiment and tone
        - Key insights and takeaways
        - Potential improvements

        Content: {input}
        """,
        agent_id="claude_analyzer"
    )

    # Create summarizer using a different model for comparison
    summarizer = Node.agent(
        name="GPT Summarizer",
        prompt=f"Create a concise summary of this analysis: {input}",
        agent_id="gpt_summarizer",
        llm_config=LlmConfig.openrouter(
            api_key=os.getenv("OPENROUTER_API_KEY"),
            model="openai/gpt-4o-mini"  # Use GPT for summarization
        )
    )

    workflow.add_node(analyzer)
    workflow.add_node(summarizer)
    workflow.add_edge(analyzer, summarizer)
    workflow.validate()

    executor = Executor(config, timeout_seconds=120)
    return workflow, executor

# Usage
workflow, executor = create_openrouter_workflow()
```

### Ollama Workflow Example

```python
from graphbit import LlmConfig, Workflow, Node, Executor

def create_ollama_workflow():
    """Create workflow using local Ollama models"""
    
    # Configure Ollama (no API key needed)
    config = LlmConfig.ollama(model="llama3.2")
    
    # Create workflow
    workflow = Workflow("Local LLM Pipeline")
    
    # Create analyzer optimized for local models
    analyzer = Node.agent(
        name="Local Model Analyzer",
        prompt=f"Analyze this text briefly: {input}",
        agent_id="local_analyzer"
    )
    
    workflow.add_node(analyzer)
    workflow.validate()
    
    # Create executor with longer timeout for local processing
    executor = Executor(config, timeout_seconds=180)
    return workflow, executor

# Usage
workflow, executor = create_ollama_workflow()
```

### Replicate Workflow Example

```python
from graphbit import LlmConfig, Workflow, Node, Executor
import os

def create_replicate_workflow():
    """Create workflow using Replicate models with function calling"""

    # Configure Replicate with function calling model
    config = LlmConfig.replicate(
        api_key=os.getenv("REPLICATE_API_KEY"),
        model="lucataco/dolphin-2.9-llama3-8b:version"
    )

    # Create workflow
    workflow = Workflow("Replicate Function Calling Pipeline")

    # Create analyzer with function calling capabilities
    analyzer = Node.agent(
        name="Replicate Function Analyzer",
        prompt=f"""
        Analyze the following content and use available tools when needed:
        - Identify key topics and themes
        - Extract actionable insights
        - Suggest relevant tools or functions to call
        - Provide structured recommendations

        Content: {input}

        Use function calling when appropriate to enhance your analysis.
        """,
        agent_id="replicate_analyzer"
    )

    # Create summarizer using a different Replicate model
    summarizer = Node.agent(
        name="Hermes Summarizer",
        prompt=f"Create a concise summary of this analysis: {input}",
        agent_id="hermes_summarizer",
        llm_config=LlmConfig.replicate(
            api_key=os.getenv("REPLICATE_API_KEY"),
            model="lucataco/dolphin-2.9-llama3-8b:version"
        )
    )

    workflow.add_node(analyzer)
    workflow.add_node(summarizer)
    workflow.add_edge(analyzer, summarizer)
    workflow.validate()

    # Create executor with appropriate timeout for Replicate
    executor = Executor(config, timeout_seconds=300)  # Longer timeout for prediction-based API
    return workflow, executor

def create_replicate_enterprise_workflow():
    """Create workflow using Replicate's enterprise-focused models"""

    # Configure with Granite model for enterprise use
    config = LlmConfig.replicate(
        api_key=os.getenv("REPLICATE_API_KEY"),
        model="lucataco/dolphin-2.9-llama3-8b:version"
    )

    workflow = Workflow("Enterprise Replicate Pipeline")

    # Enterprise-focused analyzer
    analyzer = Node.agent(
        name="Granite Enterprise Analyzer",
        prompt=f"""
        Perform enterprise-grade analysis of the following content:
        - Business impact assessment
        - Risk analysis and mitigation strategies
        - Compliance considerations
        - Strategic recommendations

        Content: {input}

        Provide analysis suitable for enterprise decision-making.
        """,
        agent_id="granite_analyzer"
    )

    workflow.add_node(analyzer)
    workflow.validate()

    executor = Executor(config, timeout_seconds=300)
    return workflow, executor

# Usage
workflow, executor = create_replicate_workflow()
enterprise_workflow, enterprise_executor = create_replicate_enterprise_workflow()
```

## Performance Optimization

### Timeout Configuration

Configure appropriate timeouts for different providers:

```python
# OpenAI - typically faster
openai_executor = Executor(
    openai_config, 
    timeout_seconds=60
)


anthropic_executor = Executor(
    anthropic_config, 
    timeout_seconds=120
)

deepseek_executor = Executor(
    deepseek_config,
    timeout_seconds=90
)

# Replicate - longer timeout for prediction-based API
replicate_executor = Executor(
    replicate_config,
    timeout_seconds=300
)

# Ollama - local processing
ollama_executor = Executor(
    ollama_config,
    timeout_seconds=180
)
```

### Executor Types for Different Providers

Choose appropriate executor types based on provider characteristics:

```python
# High-throughput for cloud providers
cloud_executor = Executor(
    llm_config=openai_config,
    timeout_seconds=60
)

# Low-latency for fast providers
realtime_executor = Executor(
    llm_config=anthropic_config,
    lightweight_mode=True,
    timeout_seconds=30
)
```

## Error Handling

### Provider-Specific Error Handling

```python
def robust_llm_usage():
    try:
        # Configure
        config = LlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY")
        )

        client = LlmClient(config)
        
        # Execute with error handling
        response = client.complete(
            prompt="Test prompt",
            max_tokens=100
        )
        
        return response
        
    except Exception as e:
        print(f"LLM operation failed: {e}")
        return None
```

### Workflow Error Handling

```python
def execute_with_error_handling(workflow, executor):
    try:
        result = executor.execute(workflow)
        
        if result.is_completed():
            return result.output()
        elif result.is_failed():
            error_msg = result.error()
            print(f"Workflow failed: {error_msg}")
            return None
            
    except Exception as e:
        print(f"Execution error: {e}")
        return None
```

## Best Practices

### 1. Provider Selection

Choose providers based on your requirements:

```python
def get_optimal_config(use_case):
    """Select optimal provider for use case"""
    if use_case == "creative":
        return LlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o"
        )
    elif use_case == "analytical":
        return LlmConfig.anthropic(
            api_key=os.getenv("ANTHROPIC_API_KEY"),
            model="claude-sonnet-4-20250514"
        )
    elif use_case == "cost_effective":
        return LlmConfig.deepseek(
            api_key=os.getenv("DEEPSEEK_API_KEY"),
            model="deepseek-chat"
        )
    elif use_case == "coding":
        return LlmConfig.deepseek(
            api_key=os.getenv("DEEPSEEK_API_KEY"),
            model="deepseek-coder"
        )
    elif use_case == "reasoning":
        return LlmConfig.deepseek(
            api_key=os.getenv("DEEPSEEK_API_KEY"),
            model="deepseek-reasoner"
        )
    elif use_case == "multi_model":
        return LlmConfig.openrouter(
            api_key=os.getenv("OPENROUTER_API_KEY"),
            model="anthropic/claude-3-5-sonnet"
        )
    elif use_case == "function_calling":
        return LlmConfig.replicate(
            api_key=os.getenv("REPLICATE_API_KEY"),
            model="lucataco/dolphin-2.9-llama3-8b:version"
        )
    elif use_case == "open_source":
        return LlmConfig.replicate(
            api_key=os.getenv("REPLICATE_API_KEY"),
            model="lucataco/dolphin-2.9-llama3-8b:version"
        )
    elif use_case == "local":
        return LlmConfig.ollama(model="llama3.2")
    else:
        return LlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o-mini"
        )
```

### 2. API Key Management

Securely manage API keys:

```python
import os
from pathlib import Path

def get_api_key(provider):
    """Securely retrieve API keys"""
    key_mapping = {
        "openai": "OPENAI_API_KEY",
        "azure_openai": "AZURE_OPENAI_API_KEY",
        "anthropic": "ANTHROPIC_API_KEY",
        "openrouter": "OPENROUTER_API_KEY",
        "perplexity": "PERPLEXITY_API_KEY",
        "deepseek": "DEEPSEEK_API_KEY",
        "replicate": "REPLICATE_API_KEY"
    }
    
    env_var = key_mapping.get(provider)
    if not env_var:
        raise ValueError(f"Unknown provider: {provider}")
    
    api_key = os.getenv(env_var)
    if not api_key:
        raise ValueError(f"Missing {env_var} environment variable")
    
    return api_key

# Usage
try:
    openai_config = LlmConfig.openai(
        api_key=get_api_key("openai")
    )
except ValueError as e:
    print(f"Configuration error: {e}")
```

### 3. Client Reuse

Reuse clients for better performance:

```python
class LLMManager:
    def __init__(self):
        self.clients = {}
    
    def get_client(self, provider, model=None):
        """Get or create client for provider"""
        key = f"{provider}_{model or 'default'}"
        
        if key not in self.clients:
            if provider == "openai":
                config = LlmConfig.openai(
                    api_key=get_api_key("openai"),
                    model=model
                )
            elif provider == "anthropic":
                config = LlmConfig.anthropic(
                    api_key=get_api_key("anthropic"),
                    model=model
                )
            elif provider == "deepseek":
                config = LlmConfig.deepseek(
                    api_key=get_api_key("deepseek"),
                    model=model
                )
            elif provider == "replicate":
                config = LlmConfig.replicate(
                    api_key=get_api_key("replicate"),
                    model=model
                )
            elif provider == "ollama":
                config = LlmConfig.ollama(model=model)
            else:
                raise ValueError(f"Unknown provider: {provider}")
            
            self.clients[key] = LlmClient(config)
        
        return self.clients[key]

# Usage
llm_manager = LLMManager()
openai_client = llm_manager.get_client("openai", "gpt-4o-mini")
```

### 4. Monitoring and Logging

Monitor LLM usage and performance:

```python
def monitor_llm_usage(client, operation_name):
    """Monitor LLM client usage"""
    stats_before = client.get_stats()
    
    # Perform operation here
    
    stats_after = client.get_stats()
    
    requests_made = stats_after['total_requests'] - stats_before['total_requests']
    print(f"{operation_name}: {requests_made} requests made")
    
    if stats_after['total_requests'] > 0:
        success_rate = stats_after['successful_requests'] / stats_after['total_requests']
        print(f"Overall success rate: {success_rate:.2%}")
```

## What's Next

- Learn about [Embeddings](embeddings.md) for vector operations
- Explore [Workflow Builder](workflow-builder.md) for complex workflows
- Check [Performance](performance.md) for optimization techniques
- See [Monitoring](monitoring.md) for production monitoring
