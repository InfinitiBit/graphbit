# LLM Providers

GraphBit supports multiple Large Language Model providers, allowing you to choose the best model for your specific use case. This guide covers configuration, usage, and optimization for each supported provider.

## Supported Providers

- **OpenAI** - GPT-4, GPT-3.5 Turbo, and other OpenAI models
- **Anthropic** - Claude 3 family models  
- **HuggingFace** - Open-source models via HuggingFace API
- **Ollama** - Local model execution (coming soon)

## OpenAI Configuration

### Basic Setup

```python
import graphbit
import os

# Configure OpenAI provider
config = graphbit.PyLlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"
)
```

### Available Models

| Model | Best For | Context Length | Cost |
|-------|----------|----------------|------|
| `gpt-4o` | Complex reasoning, latest features | 128K | High |
| `gpt-4o-mini` | Balance of performance and cost | 128K | Medium |
| `gpt-4-turbo` | High-quality outputs | 128K | High |
| `gpt-3.5-turbo` | Fast, cost-effective | 16K | Low |

### Model Selection Guidelines

```python
# For creative tasks
creative_config = graphbit.PyLlmConfig.openai(
    os.getenv("OPENAI_API_KEY"),
    "gpt-4o"  # Best creativity and reasoning
)

# For production/cost-sensitive
production_config = graphbit.PyLlmConfig.openai(
    os.getenv("OPENAI_API_KEY"), 
    "gpt-4o-mini"  # Good balance
)

# For high-volume/simple tasks
volume_config = graphbit.PyLlmConfig.openai(
    os.getenv("OPENAI_API_KEY"),
    "gpt-3.5-turbo"  # Most economical
)
```

### OpenAI Example Workflow

```python
def create_openai_workflow():
    graphbit.init()
    config = graphbit.PyLlmConfig.openai(
        os.getenv("OPENAI_API_KEY"),
        "gpt-4o-mini"
    )
    
    builder = graphbit.PyWorkflowBuilder("OpenAI Analysis")
    
    analyzer = graphbit.PyWorkflowNode.agent_node_with_config(
        name="GPT Analyzer",
        description="Analyzes content using GPT",
        agent_id="gpt_analyzer",
        prompt="Provide detailed analysis of: {input}",
        max_tokens=1500,
        temperature=0.3
    )
    
    builder.add_node(analyzer)
    workflow = builder.build()
    
    executor = graphbit.PyWorkflowExecutor(config)
    return executor.execute(workflow)
```

## Anthropic Configuration

### Basic Setup

```python
# Configure Anthropic provider
config = graphbit.PyLlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-3-haiku-20240307"
)
```

### Available Models

| Model | Best For | Context Length | Speed |
|-------|----------|----------------|-------|
| `claude-3-opus-20240229` | Most capable, complex tasks | 200K | Slow |
| `claude-3-sonnet-20240229` | Balanced performance | 200K | Medium |
| `claude-3-haiku-20240307` | Fast, cost-effective | 200K | Fast |

### Model Selection for Anthropic

```python
# For complex analysis
complex_config = graphbit.PyLlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"),
    "claude-3-opus-20240229"
)

# For balanced workloads
balanced_config = graphbit.PyLlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"),
    "claude-3-sonnet-20240229"
)

# For speed and cost efficiency
fast_config = graphbit.PyLlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"),
    "claude-3-haiku-20240307"
)
```

### Anthropic Example Workflow

```python
def create_anthropic_workflow():
    graphbit.init()
    config = graphbit.PyLlmConfig.anthropic(
        os.getenv("ANTHROPIC_API_KEY"),
        "claude-3-sonnet-20240229"
    )
    
    builder = graphbit.PyWorkflowBuilder("Claude Analysis")
    
    analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Claude Analyzer",
        description="Analyzes content using Claude",
        agent_id="claude_analyzer",
        prompt="""
        Analyze the following content with attention to:
        - Factual accuracy
        - Logical structure  
        - Potential biases
        - Recommendations
        
        Content: {input}
        """
    )
    
    builder.add_node(analyzer)
    workflow = builder.build()
    
    executor = graphbit.PyWorkflowExecutor(config)
    return executor.execute(workflow)
```

## HuggingFace Configuration

### Basic Setup

```python
# Configure HuggingFace provider
config = graphbit.PyLlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="microsoft/DialoGPT-medium"
)
```

### Popular Models

| Model | Type | Best For |
|-------|------|----------|
| `microsoft/DialoGPT-medium` | Conversational | Dialog systems |
| `bigscience/bloom-560m` | General | General text generation |
| `google/flan-t5-base` | Instruction-following | Task-specific prompts |
| `EleutherAI/gpt-j-6B` | General | Open-source GPT alternative |

### HuggingFace Example

```python
def create_huggingface_workflow():
    graphbit.init()
    config = graphbit.PyLlmConfig.huggingface(
        os.getenv("HUGGINGFACE_API_KEY"),
        "microsoft/DialoGPT-medium"
    )
    
    builder = graphbit.PyWorkflowBuilder("HuggingFace Chat")
    
    chatbot = graphbit.PyWorkflowNode.agent_node(
        name="HF Chatbot",
        description="Conversational agent using HuggingFace",
        agent_id="hf_chatbot",
        prompt="Respond conversationally to: {input}"
    )
    
    builder.add_node(chatbot)
    workflow = builder.build()
    
    executor = graphbit.PyWorkflowExecutor(config)
    return executor.execute(workflow)
```

## Multi-Provider Workflows

### Provider Comparison

```python
def compare_providers():
    graphbit.init()
    
    # Configure multiple providers
    openai_config = graphbit.PyLlmConfig.openai(
        os.getenv("OPENAI_API_KEY"), "gpt-4o-mini"
    )
    
    anthropic_config = graphbit.PyLlmConfig.anthropic(
        os.getenv("ANTHROPIC_API_KEY"), "claude-3-haiku-20240307"
    )
    
    # Create executors for each provider
    openai_executor = graphbit.PyWorkflowExecutor(openai_config)
    anthropic_executor = graphbit.PyWorkflowExecutor(anthropic_config)
    
    # Create simple analysis workflow
    def create_analysis_workflow():
        builder = graphbit.PyWorkflowBuilder("Provider Comparison")
        analyzer = graphbit.PyWorkflowNode.agent_node(
            "Analyzer", "Analyzes input", "analyzer", 
            "Analyze and provide 3 key insights about: {input}"
        )
        builder.add_node(analyzer)
        return builder.build()
    
    workflow = create_analysis_workflow()
    
    # Execute with different providers
    openai_result = openai_executor.execute(workflow)
    anthropic_result = anthropic_executor.execute(workflow)
    
    print(f"OpenAI result: {openai_result.get_variable('output')}")
    print(f"Anthropic result: {anthropic_result.get_variable('output')}")
    
    return {
        "openai": openai_result,
        "anthropic": anthropic_result
    }
```

### Provider-Specific Workflows

```python
def create_specialized_workflows():
    """Create workflows optimized for different providers."""
    
    # OpenAI - Good for creative tasks
    def openai_creative_workflow():
        config = graphbit.PyLlmConfig.openai(
            os.getenv("OPENAI_API_KEY"), "gpt-4o"
        )
        builder = graphbit.PyWorkflowBuilder("Creative Writing")
        
        writer = graphbit.PyWorkflowNode.agent_node_with_config(
            name="Creative Writer",
            description="Writes creative content",
            agent_id="creative_writer",
            prompt="Write a creative story about: {topic}",
            max_tokens=2000,
            temperature=0.8
        )
        
        builder.add_node(writer)
        return graphbit.PyWorkflowExecutor(config), builder.build()
    
    # Anthropic - Good for analysis
    def anthropic_analysis_workflow():
        config = graphbit.PyLlmConfig.anthropic(
            os.getenv("ANTHROPIC_API_KEY"), "claude-3-sonnet-20240229"
        )
        builder = graphbit.PyWorkflowBuilder("Detailed Analysis")
        
        analyzer = graphbit.PyWorkflowNode.agent_node(
            name="Deep Analyzer",
            description="Provides detailed analysis",
            agent_id="deep_analyzer",
            prompt="""
            Provide a comprehensive analysis of: {input}
            
            Consider:
            - Multiple perspectives
            - Potential implications
            - Evidence and reasoning
            - Limitations and uncertainties
            """
        )
        
        builder.add_node(analyzer)
        return graphbit.PyWorkflowExecutor(config), builder.build()
    
    return {
        "creative": openai_creative_workflow(),
        "analysis": anthropic_analysis_workflow()
    }
```

## Provider-Specific Optimizations

### OpenAI Optimizations

```python
# Optimize for OpenAI characteristics
def optimize_for_openai():
    config = graphbit.PyLlmConfig.openai(
        os.getenv("OPENAI_API_KEY"), "gpt-4o-mini"
    )
    
    # Use structured prompts for better results
    structured_agent = graphbit.PyWorkflowNode.agent_node(
        name="Structured Processor",
        description="Uses structured prompts for OpenAI",
        agent_id="structured_processor",
        prompt="""
        Task: {task}
        
        Instructions:
        1. Analyze the input carefully
        2. Consider multiple angles
        3. Provide specific examples
        4. Conclude with actionable insights
        
        Input: {input}
        
        Response:
        """
    )
    
    return structured_agent
```

### Anthropic Optimizations

```python
# Optimize for Anthropic characteristics  
def optimize_for_anthropic():
    config = graphbit.PyLlmConfig.anthropic(
        os.getenv("ANTHROPIC_API_KEY"), "claude-3-sonnet-20240229"
    )
    
    # Use conversational style for better results
    conversational_agent = graphbit.PyWorkflowNode.agent_node(
        name="Conversational Processor",
        description="Uses conversational style for Anthropic",
        agent_id="conversational_processor",
        prompt="""
        I'd like you to help me understand this topic: {input}
        
        Please explain it clearly, considering:
        - What are the key concepts?
        - How do they relate to each other?
        - What are the practical implications?
        - Are there any important nuances or caveats?
        
        Feel free to use examples to illustrate your points.
        """
    )
    
    return conversational_agent
```

## Error Handling and Fallbacks

### Provider Fallback Strategy

```python
class ProviderFallback:
    def __init__(self):
        self.providers = [
            ("OpenAI", graphbit.PyLlmConfig.openai(
                os.getenv("OPENAI_API_KEY"), "gpt-4o-mini"
            )),
            ("Anthropic", graphbit.PyLlmConfig.anthropic(
                os.getenv("ANTHROPIC_API_KEY"), "claude-3-haiku-20240307"
            )),
        ]
    
    def execute_with_fallback(self, workflow):
        """Try providers in order until one succeeds."""
        for provider_name, config in self.providers:
            try:
                executor = graphbit.PyWorkflowExecutor(config)
                result = executor.execute(workflow)
                
                if result.is_completed():
                    print(f"✅ Success with {provider_name}")
                    return result
                    
            except Exception as e:
                print(f"❌ {provider_name} failed: {e}")
                continue
        
        raise Exception("All providers failed")

# Usage
fallback = ProviderFallback()
result = fallback.execute_with_fallback(workflow)
```

### Rate Limiting Handling

```python
def create_rate_limited_executor(config):
    """Create executor with rate limiting considerations."""
    
    return graphbit.PyWorkflowExecutor(config) \
        .with_retry_config(
            graphbit.PyRetryConfig.default()
            .with_exponential_backoff(2000, 2.0, 60000)  # Longer delays
            .with_jitter(0.3)  # More jitter for rate limits
        ) \
        .with_circuit_breaker_config(
            graphbit.PyCircuitBreakerConfig(10, 120000)  # Higher threshold
        )
```

## Best Practices

### 1. Provider Selection

Choose providers based on your needs:

```python
# For creative tasks
creative_config = graphbit.PyLlmConfig.openai(
    os.getenv("OPENAI_API_KEY"), "gpt-4o"
)

# For analytical tasks
analytical_config = graphbit.PyLlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"), "claude-3-sonnet-20240229"
)

# For cost-effective tasks
economical_config = graphbit.PyLlmConfig.openai(
    os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo"
)
```

### 2. Cost Optimization

```python
# Use shorter prompts for expensive models
cost_optimized_agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Cost Optimized",
    description="Optimized for cost",
    agent_id="cost_optimized",
    prompt="Summarize: {input}",  # Short, direct prompt
    max_tokens=500,  # Limit output tokens
    temperature=0.1  # Low temperature for consistency
)
```

### 3. Performance Optimization

```python
# Use faster models for real-time applications
realtime_config = graphbit.PyLlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"), "claude-3-haiku-20240307"  # Fastest Claude
)

# Optimize executor for speed
speed_executor = graphbit.PyWorkflowExecutor.new_low_latency(realtime_config)
```

### 4. Quality vs Speed Trade-offs

```python
def choose_config_for_use_case(use_case):
    """Choose optimal config based on use case."""
    
    configs = {
        "research": graphbit.PyLlmConfig.anthropic(
            os.getenv("ANTHROPIC_API_KEY"), "claude-3-opus-20240229"
        ),
        "content": graphbit.PyLlmConfig.openai(
            os.getenv("OPENAI_API_KEY"), "gpt-4o"
        ),
        "chat": graphbit.PyLlmConfig.openai(
            os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo"
        ),
        "analysis": graphbit.PyLlmConfig.anthropic(
            os.getenv("ANTHROPIC_API_KEY"), "claude-3-sonnet-20240229"
        )
    }
    
    return configs.get(use_case, configs["chat"])
```

## Monitoring and Costs

### Usage Tracking

```python
def track_provider_usage():
    """Track usage across different providers."""
    
    import time
    
    usage_stats = {
        "openai": {"requests": 0, "total_time": 0},
        "anthropic": {"requests": 0, "total_time": 0}
    }
    
    def track_execution(provider_name, executor, workflow):
        start_time = time.time()
        result = executor.execute(workflow)
        end_time = time.time()
        
        usage_stats[provider_name]["requests"] += 1
        usage_stats[provider_name]["total_time"] += (end_time - start_time)
        
        return result
    
    return track_execution, usage_stats
```

## Troubleshooting

### Common Issues

1. **API Key Issues**
```python
# Verify API keys are set
import os
if not os.getenv("OPENAI_API_KEY"):
    print("❌ OPENAI_API_KEY not set")
if not os.getenv("ANTHROPIC_API_KEY"):
    print("❌ ANTHROPIC_API_KEY not set")
```

2. **Rate Limiting**
```python
# Handle rate limits with proper retry configuration
rate_limit_config = graphbit.PyRetryConfig.default() \
    .with_exponential_backoff(5000, 2.0, 120000)
```

3. **Model Availability**
```python
# Use fallback models if primary model is unavailable
def create_fallback_config():
    try:
        return graphbit.PyLlmConfig.openai(
            os.getenv("OPENAI_API_KEY"), "gpt-4o"
        )
    except:
        return graphbit.PyLlmConfig.openai(
            os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo"
        )
```

GraphBit's multi-provider support gives you flexibility to choose the best model for each task while maintaining consistent workflow patterns across all providers. 
