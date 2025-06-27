# LLM Providers

GraphBit supports multiple Large Language Model providers through a unified client interface. This guide covers configuration, usage, and optimization for each supported provider.

## Supported Providers

GraphBit supports these LLM providers:
- **OpenAI** - GPT models including GPT-4o, GPT-4o-mini
- **Anthropic** - Claude models including Claude-3.5-Sonnet  
- **HuggingFace** - Access to thousands of models via HuggingFace Inference API
- **Ollama** - Local model execution with various open-source models

## Configuration

### OpenAI Configuration

Configure OpenAI provider with API key and model selection:

```python
import graphbit
import os

# Basic OpenAI configuration
config = graphbit.LlmConfig.openai(
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
| `gpt-4-turbo` | High-quality outputs | 128K | High quality |
| `gpt-3.5-turbo` | Fast, cost-effective tasks | 16K | Fast, economical |

```python
# Model selection examples
creative_config = graphbit.LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o"  # For creative and complex tasks
)

production_config = graphbit.LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"  # Balanced for production
)

fast_config = graphbit.LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-3.5-turbo"  # For high-volume, simple tasks
)
```

### Anthropic Configuration

Configure Anthropic provider for Claude models:

```python
# Basic Anthropic configuration
config = graphbit.LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-3-5-sonnet-20241022"  # Optional - defaults to claude-3-5-sonnet-20241022
)

print(f"Provider: {config.provider()}")  # "Anthropic"
print(f"Model: {config.model()}")        # "claude-3-5-sonnet-20241022"
```

#### Available Anthropic Models

| Model | Best For | Context Length | Speed |
|-------|----------|----------------|-------|
| `claude-3-opus-20240229` | Most capable, complex analysis | 200K | Slowest, highest quality |
| `claude-3-sonnet-20240229` | Balanced performance | 200K | Medium speed and quality |
| `claude-3-haiku-20240307` | Fast, cost-effective | 200K | Fastest, good quality |
| `claude-3-5-sonnet-20241022` | Latest, improved reasoning | 200K | Good speed, high quality |

```python
# Model selection for different use cases
complex_config = graphbit.LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-3-opus-20240229"  # For complex analysis
)

balanced_config = graphbit.LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-3-5-sonnet-20241022"  # For balanced workloads
)

fast_config = graphbit.LlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-3-haiku-20240307"  # For speed and efficiency
)
```

### HuggingFace Configuration

Configure HuggingFace provider to access thousands of models via the Inference API:

```python
# Basic HuggingFace configuration
config = graphbit.LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="microsoft/DialoGPT-medium"  # Optional - defaults to microsoft/DialoGPT-medium
)

print(f"Provider: {config.provider()}")  # "huggingface"
print(f"Model: {config.model()}")        # "microsoft/DialoGPT-medium"

# Custom endpoint configuration
custom_config = graphbit.LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="mistralai/Mistral-7B-Instruct-v0.1",
    base_url="https://my-custom-endpoint.huggingface.co"  # Optional custom endpoint
)
```

#### Popular HuggingFace Models

| Model | Best For | Size | Performance |
|-------|----------|------|-------------|
| `microsoft/DialoGPT-medium` | Conversational AI, chat | 345M | Fast, good dialogue |
| `mistralai/Mistral-7B-Instruct-v0.1` | General instruction following | 7B | High quality, versatile |
| `microsoft/CodeBERT-base` | Code understanding | 125M | Specialized for code |
| `facebook/blenderbot-400M-distill` | Conversational AI | 400M | Balanced dialogue |
| `huggingface/CodeBERTa-small-v1` | Code generation | 84M | Fast code tasks |
| `microsoft/DialoGPT-large` | Advanced dialogue | 762M | Higher quality chat |

```python
# Model selection for different use cases
dialogue_config = graphbit.LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="microsoft/DialoGPT-large"  # For high-quality dialogue
)

instruction_config = graphbit.LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="mistralai/Mistral-7B-Instruct-v0.1"  # For instruction following
)

code_config = graphbit.LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="microsoft/CodeBERT-base"  # For code-related tasks
)

# Fast and lightweight option
lightweight_config = graphbit.LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="microsoft/DialoGPT-medium"  # Balanced performance
)
```

#### HuggingFace API Key Setup

To use HuggingFace models, you need an API key:

1. Create an account at [HuggingFace](https://huggingface.co/)
2. Generate an API token in your [settings](https://huggingface.co/settings/tokens)
3. Set the environment variable:

```bash
export HUGGINGFACE_API_KEY="your-api-key-here"
```

#### Model Selection Tips

- **Free Tier**: Most models work with free HuggingFace accounts
- **Custom Models**: You can use any public model from the HuggingFace Hub
- **Private Models**: Use your own fine-tuned models with appropriate permissions
- **Performance**: Larger models (7B+) provide better quality but slower responses
- **Cost**: HuggingFace Inference API has competitive pricing for hosted inference

### Ollama Configuration

Configure Ollama for local model execution:

```python
# Basic Ollama configuration (no API key required)
config = graphbit.LlmConfig.ollama(
    model="llama3.2"  # Optional - defaults to llama3.2
)

print(f"Provider: {config.provider()}")  # "Ollama"
print(f"Model: {config.model()}")        # "llama3.2"

# Other popular models
mistral_config = graphbit.LlmConfig.ollama(model="mistral")
codellama_config = graphbit.LlmConfig.ollama(model="codellama")
phi_config = graphbit.LlmConfig.ollama(model="phi")
```

## LLM Client Usage

### Creating and Using Clients

```python
# Create client with configuration
client = graphbit.LlmClient(config, debug=False)

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
def create_openai_workflow():
    """Create workflow using OpenAI"""
    # Initialize GraphBit
    graphbit.init()
    
    # Configure OpenAI
    config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Create workflow
    workflow = graphbit.Workflow("OpenAI Analysis Pipeline")
    
    # Create analyzer node
    analyzer = graphbit.Node.agent(
        name="GPT Content Analyzer",
        prompt="Analyze the following content for sentiment, key themes, and quality:\n\n{input}",
        agent_id="gpt_analyzer"
    )
    
    # Add to workflow
    analyzer_id = workflow.add_node(analyzer)
    workflow.validate()
    
    # Create executor and run
    executor = graphbit.Executor(config, timeout_seconds=60)
    return workflow, executor

# Usage
workflow, executor = create_openai_workflow()
result = executor.execute(workflow)
```

### Anthropic Workflow Example

```python
def create_anthropic_workflow():
    """Create workflow using Anthropic Claude"""
    graphbit.init()
    
    # Configure Anthropic
    config = graphbit.LlmConfig.anthropic(
        api_key=os.getenv("ANTHROPIC_API_KEY"),
        model="claude-3-5-sonnet-20241022"
    )
    
    # Create workflow
    workflow = graphbit.Workflow("Claude Analysis Pipeline")
    
    # Create analyzer with detailed prompt
    analyzer = graphbit.Node.agent(
        name="Claude Content Analyzer",
        prompt="""
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
    executor = graphbit.Executor(config, timeout_seconds=120)
    return workflow, executor

# Usage
workflow, executor = create_anthropic_workflow()
```

### Ollama Workflow Example

```python
def create_ollama_workflow():
    """Create workflow using local Ollama models"""
    graphbit.init()
    
    # Configure Ollama (no API key needed)
    config = graphbit.LlmConfig.ollama(model="llama3.2")
    
    # Create workflow
    workflow = graphbit.Workflow("Local LLM Pipeline")
    
    # Create analyzer optimized for local models
    analyzer = graphbit.Node.agent(
        name="Local Model Analyzer",
        prompt="Analyze this text briefly: {input}",
        agent_id="local_analyzer"
    )
    
    workflow.add_node(analyzer)
    workflow.validate()
    
    # Create executor with longer timeout for local processing
    executor = graphbit.Executor(config, timeout_seconds=180)
    return workflow, executor

# Usage
workflow, executor = create_ollama_workflow()
```

## Performance Optimization

### Timeout Configuration

Configure appropriate timeouts for different providers:

```python
# OpenAI - typically faster
openai_executor = graphbit.Executor(
    openai_config, 
    timeout_seconds=60
)

# Anthropic - may need more time
anthropic_executor = graphbit.Executor(
    anthropic_config, 
    timeout_seconds=120
)

# Ollama - local processing may be slower
ollama_executor = graphbit.Executor(
    ollama_config, 
    timeout_seconds=180
)
```

### Executor Types for Different Providers

Choose appropriate executor types based on provider characteristics:

```python
# High-throughput for cloud providers
cloud_executor = graphbit.Executor.new_high_throughput(
    llm_config=openai_config,
    timeout_seconds=60
)

# Low-latency for fast providers
realtime_executor = graphbit.Executor.new_low_latency(
    llm_config=anthropic_config,
    timeout_seconds=30
)

# Memory-optimized for local models
local_executor = graphbit.Executor.new_memory_optimized(
    llm_config=ollama_config,
    timeout_seconds=180
)
```

## Error Handling

### Provider-Specific Error Handling

```python
def robust_llm_usage():
    try:
        # Initialize and configure
        graphbit.init()
        config = graphbit.LlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY")
        )
        client = graphbit.LlmClient(config)
        
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
        return graphbit.LlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o"
        )
    elif use_case == "analytical":
        return graphbit.LlmConfig.anthropic(
            api_key=os.getenv("ANTHROPIC_API_KEY"),
            model="claude-3-5-sonnet-20241022"
        )
    elif use_case == "local":
        return graphbit.LlmConfig.ollama(model="llama3.2")
    else:
        return graphbit.LlmConfig.openai(
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
        "anthropic": "ANTHROPIC_API_KEY"
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
    openai_config = graphbit.LlmConfig.openai(
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
                config = graphbit.LlmConfig.openai(
                    api_key=get_api_key("openai"),
                    model=model
                )
            elif provider == "anthropic":
                config = graphbit.LlmConfig.anthropic(
                    api_key=get_api_key("anthropic"),
                    model=model
                )
            elif provider == "ollama":
                config = graphbit.LlmConfig.ollama(model=model)
            else:
                raise ValueError(f"Unknown provider: {provider}")
            
            self.clients[key] = graphbit.LlmClient(config)
        
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
