# LLM Providers

GraphBit supports multiple Large Language Model providers through a unified client interface. This guide covers configuration, usage, and optimization for each supported provider.

## Supported Providers

GraphBit supports these LLM providers:
- **OpenAI** - GPT models including GPT-4o, GPT-4o-mini
- **Anthropic** - Claude models including Claude-4-Sonnet
- **OpenRouter** - Unified access to 400+ models from multiple providers (GPT, Claude, Mistral, etc.)
- **Perplexity** - Real-time search-enabled models including Sonar models
- **DeepSeek** - High-performance models including DeepSeek-Chat, DeepSeek-Coder, and DeepSeek-Reasoner
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
        "anthropic": "ANTHROPIC_API_KEY",
        "openrouter": "OPENROUTER_API_KEY",
        "perplexity": "PERPLEXITY_API_KEY",
        "deepseek": "DEEPSEEK_API_KEY"
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
