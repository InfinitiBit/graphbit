# LLM Integration and Advanced Usage

This example demonstrates comprehensive LLM integration with GraphBit, showcasing various providers, execution modes, and advanced features.

## Overview

We'll explore:
1. **Multiple LLM Providers**: OpenAI, Anthropic, HuggingFace, Ollama
2. **Execution Modes**: Sync, async, batch, streaming
3. **Performance Optimization**: High-throughput, low-latency, memory-optimized
4. **Error Handling**: Resilience patterns and fallbacks
5. **Monitoring**: Performance metrics and health checks

## Complete LLM Client Example

```python
from graphbit import init, LlmConfig, LlmClient, health_check, get_system_info
import os
import asyncio
import time

class AdvancedLLMSystem:
    def __init__(self):
        """Initialize the advanced LLM system."""
        # Initialize GraphBit
        init(enable_tracing=True)
        
        # Store multiple provider clients
        self.clients = {}
        self.initialize_providers()
    
    def initialize_providers(self):
        """Initialize all available LLM providers."""
        print("Initializing LLM providers...")
        
        # OpenAI client
        if os.getenv("OPENAI_API_KEY"):
            openai_config = LlmConfig.openai(
                api_key=os.getenv("OPENAI_API_KEY"),
                model="gpt-4o-mini"
            )
            self.clients['openai'] = LlmClient(openai_config, debug=True)
            print("OpenAI client initialized")
        
        # Anthropic client
        if os.getenv("ANTHROPIC_API_KEY"):
            anthropic_config = LlmConfig.anthropic(
                api_key=os.getenv("ANTHROPIC_API_KEY"),
                model="claude-3-5-haiku-20241022"
            )
            self.clients['anthropic'] = LlmClient(anthropic_config, debug=True)
            print("Anthropic client initialized")
        
        # DeepSeek client
        if os.getenv("DEEPSEEK_API_KEY"):
            deepseek_config = LlmConfig.deepseek(
                api_key=os.getenv("DEEPSEEK_API_KEY"),
                model="deepseek-chat"
            )
            self.clients['deepseek'] = LlmClient(deepseek_config, debug=True)
            print("DeepSeek client initialized")
        
        # HuggingFace client
        if os.getenv("HUGGINGFACE_API_KEY"):
            huggingface_config = LlmConfig.huggingface(
                api_key=os.getenv("HUGGINGFACE_API_KEY"),
                model="microsoft/DialoGPT-medium"
            )
            self.clients['huggingface'] = LlmClient(huggingface_config, debug=True)
            print("HuggingFace client initialized")
        
        # Ollama client (no API key required)
        try:
            ollama_config = LlmConfig.ollama("llama3.2")
            self.clients['ollama'] = LlmClient(ollama_config, debug=True)
            print("Ollama client initialized")
        except Exception as e:
            print(f"Ollama client failed: {e}")
        
        if not self.clients:
            raise Exception("No LLM providers available. Please set API keys or install Ollama.")
    
    def test_basic_completion(self, provider: str = 'openai'):
        """Test basic text completion."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return None
        
        client = self.clients[provider]
        prompt = "Explain quantum computing in simple terms."
        
        print(f"\nTesting basic completion with {provider}...")
        print(f"Prompt: {prompt}")
        
        try:
            start_time = time.time()
            response = client.complete(
                prompt=prompt,
                max_tokens=200,
                temperature=0.7
            )
            duration = (time.time() - start_time) * 1000
            
            print(f"Completed in {duration:.2f}ms")
            print(f"Response: {response[:200]}...")
            
            return response
        except Exception as e:
            print(f"Completion failed: {e}")
            return None
    
    async def test_async_completion(self, provider: str = 'openai'):
        """Test asynchronous completion."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return None
        
        client = self.clients[provider]
        prompt = "Write a haiku about artificial intelligence."
        
        print(f"\nTesting async completion with {provider}...")
        print(f"Prompt: {prompt}")
        
        try:
            start_time = time.time()
            response = await client.complete_async(
                prompt=prompt,
                max_tokens=100,
                temperature=0.8
            )
            duration = (time.time() - start_time) * 1000
            
            print(f"Async completed in {duration:.2f}ms")
            print(f"Response: {response}")
            
            return response
        except Exception as e:
            print(f"Async completion failed: {e}")
            return None
    
    async def test_batch_completion(self, provider: str = 'openai'):
        """Test batch completion for multiple prompts."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return None
        
        client = self.clients[provider]
        prompts = [
            "What is machine learning?",
            "Explain neural networks briefly.",
            "What are the benefits of cloud computing?",
            "How does blockchain work?",
            "What is the future of AI?"
        ]
        
        print(f"\nTesting batch completion with {provider}...")
        print(f"Processing {len(prompts)} prompts...")
        
        try:
            start_time = time.time()
            responses = await client.complete_batch(
                prompts=prompts,
                max_tokens=100,
                temperature=0.6,
                max_concurrency=3
            )
            duration = (time.time() - start_time) * 1000
            
            print(f"Batch completed in {duration:.2f}ms")
            print(f"Average per prompt: {duration/len(prompts):.2f}ms")
            
            for i, (prompt, response) in enumerate(zip(prompts, responses)):
                print(f"\n{i+1}. {prompt}")
                print(f"   → {response[:100]}...")
            
            return responses
        except Exception as e:
            print(f"Batch completion failed: {e}")
            return None
    
    async def test_chat_optimized(self, provider: str = 'openai'):
        """Test optimized chat completion."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return None
        
        client = self.clients[provider]
        messages = [
            ("system", "You are a helpful AI assistant specialized in technology."),
            ("user", "What's the difference between AI and ML?"),
            ("assistant", "AI is the broader concept of machines being able to carry out tasks in a smart way, while ML is a specific subset of AI that involves training algorithms on data."),
            ("user", "Can you give me a practical example?")
        ]
        
        print(f"\nTesting chat-optimized completion with {provider}...")
        
        try:
            start_time = time.time()
            response = await client.chat_optimized(
                messages=messages,
                max_tokens=150,
                temperature=0.7
            )
            duration = (time.time() - start_time) * 1000
            
            print(f"Chat completed in {duration:.2f}ms")
            print(f"Response: {response}")
            
            return response
        except Exception as e:
            print(f"Chat completion failed: {e}")
            return None
    
    async def test_streaming_completion(self, provider: str = 'openai'):
        """Test streaming completion."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return None
        
        client = self.clients[provider]
        prompt = "Write a detailed explanation of how machine learning works, covering the key concepts step by step."
        
        print(f"\nTesting streaming completion with {provider}...")
        print(f"Prompt: {prompt}")
        print("Streaming response:")
        
        try:
            start_time = time.time()
            
            # Note: Streaming returns an async iterator
            stream = await client.complete_stream(
                prompt=prompt,
                max_tokens=300,
                temperature=0.7
            )
            
            full_response = ""
            async for chunk in stream:
                print(chunk, end='', flush=True)
                full_response += chunk
            
            duration = (time.time() - start_time) * 1000
            print(f"\n\nStreaming completed in {duration:.2f}ms")
            print(f"Total tokens: ~{len(full_response.split())}")
            
            return full_response
        except Exception as e:
            print(f"Streaming completion failed: {e}")
            return None
    
    def test_client_warmup(self, provider: str = 'openai'):
        """Test client warmup for improved performance."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return
        
        client = self.clients[provider]
        
        print(f"\nTesting client warmup with {provider}...")
        
        try:
            # Warmup the client
            start_time = time.time()
            asyncio.run(client.warmup())
            warmup_duration = (time.time() - start_time) * 1000
            
            print(f"Warmup completed in {warmup_duration:.2f}ms")
            
            # Test performance after warmup
            start_time = time.time()
            response = client.complete("Quick test after warmup", max_tokens=50)
            completion_duration = (time.time() - start_time) * 1000
            
            print(f"Post-warmup completion: {completion_duration:.2f}ms")
            
        except Exception as e:
            print(f"Warmup failed: {e}")
    
    def get_client_statistics(self, provider: str = 'openai'):
        """Get detailed client statistics."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return None
        
        client = self.clients[provider]
        
        print(f"\nGetting statistics for {provider}...")
        
        try:
            stats = client.get_stats()
            
            print(f"Client Statistics for {provider}:")
            for key, value in stats.items():
                if isinstance(value, float):
                    print(f"  {key}: {value:.3f}")
                else:
                    print(f"  {key}: {value}")
            
            return stats
        except Exception as e:
            print(f"Failed to get statistics: {e}")
            return None
    
    def reset_client_statistics(self, provider: str = 'openai'):
        """Reset client statistics."""
        if provider not in self.clients:
            print(f"Provider '{provider}' not available")
            return
        
        client = self.clients[provider]
        
        try:
            client.reset_stats()
            print(f"Statistics reset for {provider}")
        except Exception as e:
            print(f"Failed to reset statistics: {e}")
    
    def compare_providers(self, prompt: str = "Explain the concept of recursion in programming."):
        """Compare responses from all available providers."""
        print("\nComparing providers...")
        print(f"Prompt: {prompt}")
        
        results = {}
        
        for provider_name, client in self.clients.items():
            print(f"\n--- Testing {provider_name} ---")
            try:
                start_time = time.time()
                response = client.complete(
                    prompt=prompt,
                    max_tokens=150,
                    temperature=0.7
                )
                duration = (time.time() - start_time) * 1000
                
                results[provider_name] = {
                    'response': response,
                    'duration_ms': duration,
                    'success': True
                }
                
                print(f"{provider_name}: {duration:.2f}ms")
                print(f"Response: {response[:100]}...")
                
            except Exception as e:
                results[provider_name] = {
                    'error': str(e),
                    'success': False
                }
                print(f"{provider_name}: {e}")
        
        return results

# Performance-optimized clients
def create_performance_optimized_clients():
    """Create clients optimized for different performance characteristics."""
    
    init()
    
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("OpenAI API key required for performance tests")
        return None
    
    # High-throughput client
    high_throughput_config = LlmConfig.openai(api_key, "gpt-4o-mini")
    high_throughput_client = LlmClient(high_throughput_config, debug=False)
    
    # Low-latency client  
    low_latency_config = LlmConfig.openai(api_key, "gpt-4o-mini")
    low_latency_client = LlmClient(low_latency_config, debug=False)
    
    return {
        'high_throughput': high_throughput_client,
        'low_latency': low_latency_client
    }

async def performance_benchmark():
    """Benchmark different client configurations."""
    
    clients = create_performance_optimized_clients()
    if not clients:
        return
    
    test_prompt = "Summarize the benefits of renewable energy in one paragraph."
    
    print("\n🏃 Performance Benchmark")
    print("=" * 50)
    
    for config_name, client in clients.items():
        print(f"\nTesting {config_name} configuration...")
        
        # Single completion test
        start_time = time.time()
        try:
            response = client.complete(test_prompt, max_tokens=100)
            single_duration = (time.time() - start_time) * 1000
            print(f"Single completion: {single_duration:.2f}ms")
        except Exception as e:
            print(f"Single completion failed: {e}")
            continue
        
        # Batch test
        batch_prompts = [test_prompt] * 5
        start_time = time.time()
        try:
            batch_responses = await client.complete_batch(
                batch_prompts,
                max_tokens=100,
                max_concurrency=3
            )
            batch_duration = (time.time() - start_time) * 1000
            avg_per_prompt = batch_duration / len(batch_prompts)
            print(f"Batch completion (5 prompts): {batch_duration:.2f}ms total, {avg_per_prompt:.2f}ms avg")
        except Exception as e:
            print(f"Batch completion failed: {e}")
        
        # Get statistics
        try:
            stats = client.get_stats()
            print(f"Stats: {stats.get('total_requests', 0)} requests, "
                  f"{stats.get('average_response_time_ms', 0):.2f}ms avg response time")
        except:
            pass

# Error handling and resilience examples
async def test_error_handling():
    """Test error handling and resilience features."""
    
    init()
    
    # Test with invalid API key
    print("\nTesting Error Handling")
    print("=" * 40)
    
    try:
        invalid_config = LlmConfig.openai("invalid-key", "gpt-4o-mini")
        invalid_client = LlmClient(invalid_config)
        
        print("Testing with invalid API key...")
        response = invalid_client.complete("Test prompt", max_tokens=50)
        print("Expected error but got response")
    except Exception as e:
        print(f"Correctly handled invalid API key: {type(e).__name__}")
    
    # Test timeout handling
    if os.getenv("OPENAI_API_KEY"):
        try:
            config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
            client = LlmClient(config)
            
            print("\nTesting very long prompt (potential timeout)...")
            very_long_prompt = "Write a comprehensive essay about " + "technology " * 1000
            
            response = client.complete(very_long_prompt, max_tokens=2000)
            print("Long prompt handled successfully")
        except Exception as e:
            print(f"Timeout/limit handled: {type(e).__name__}")

# System health monitoring
def monitor_llm_system_health():
    """Monitor LLM system health and performance."""
    
    init()
    
    print("\nSystem Health Check")
    print("=" * 40)
    
    # Check GraphBit health
    health = health_check()
    print("GraphBit Health:")
    for key, value in health.items():
        status = "Ok!" if value else "Not Ok!"
        print(f"  {status} {key}: {value}")
    
    # Get system information
    info = get_system_info()
    print("\nSystem Information:")
    print(f"  Version: {info.get('version', 'unknown')}")
    print(f"  Runtime threads: {info.get('runtime_worker_threads', 'unknown')}")
    print(f"  Memory allocator: {info.get('memory_allocator', 'unknown')}")
    
    # Test provider connectivity
    print("\nProvider Connectivity:")
    
    providers_to_test = [
        ('OpenAI', lambda: LlmConfig.openai(os.getenv("OPENAI_API_KEY", "test"), "gpt-4o-mini")),
        ('Anthropic', lambda: LlmConfig.anthropic(os.getenv("ANTHROPIC_API_KEY", "test"), "claude-3-5-haiku-20241022")),
        ('Ollama', lambda: LlmConfig.ollama("llama3.2"))
    ]
    
    for provider_name, config_func in providers_to_test:
        try:
            config = config_func()
            client = LlmClient(config)
            print(f"  {provider_name}: Configuration valid")
        except Exception as e:
            print(f"  {provider_name}: {str(e)[:50]}...")

# Example usage
async def main():
    """Run comprehensive LLM system demonstration."""
    
    print("GraphBit LLM Integration Demo")
    print("=" * 60)
    
    try:
        # Initialize system
        llm_system = AdvancedLLMSystem()
        
        # Test basic functionality
        for provider in llm_system.clients.keys():
            llm_system.test_basic_completion(provider)
            await llm_system.test_async_completion(provider)
            break  # Just test first available provider for demo
        
        # Test advanced features with primary provider
        primary_provider = list(llm_system.clients.keys())[0]
        
        await llm_system.test_batch_completion(primary_provider)
        await llm_system.test_chat_optimized(primary_provider)
        
        # Test performance features
        llm_system.test_client_warmup(primary_provider)
        llm_system.get_client_statistics(primary_provider)
        
        # Compare providers if multiple available
        if len(llm_system.clients) > 1:
            llm_system.compare_providers()
        
        # Performance benchmark
        await performance_benchmark()
        
        # Error handling tests
        await test_error_handling()
        
        # System health check
        monitor_llm_system_health()
        
        print("\nDemo completed successfully!")
        
    except Exception as e:
        print(f"\nDemo failed: {e}")

if __name__ == "__main__":
    asyncio.run(main())
```

## Simplified Examples

### Quick OpenAI Integration

```python
from graphbit import init, LlmConfig, LlmClient
import os

def quick_openai_example():
    """Simple OpenAI integration example."""
    
    # Initialize
    init()
    
    # Configure and create client
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    client = LlmClient(config)
    
    # Simple completion
    response = client.complete(
        "Explain quantum computing in 3 sentences.",
        max_tokens=100,
        temperature=0.7
    )
    
    print(f"Response: {response}")
    
    # Get statistics
    stats = client.get_stats()
    print(f"Total requests: {stats.get('total_requests', 0)}")

# Usage
quick_openai_example()
```

### Local Ollama Integration

```python
from graphbit import init, LlmConfig, LlmClient

def quick_ollama_example():
    """Simple Ollama integration example."""
    
    # Initialize
    init()
    
    # Configure Ollama (no API key needed)
    config = LlmConfig.ollama("llama3.2")
    client = LlmClient(config, debug=True)
    
    # Test completion
    try:
        response = client.complete(
            "What are the benefits of local AI models?",
            max_tokens=150,
            temperature=0.8
        )
        print(f"Ollama response: {response}")
    except Exception as e:
        print(f"Ollama error (make sure Ollama is running): {e}")

# Usage
quick_ollama_example()
```

### Anthropic Claude Integration

```python
from graphbit import init, LlmConfig, LlmClient
import os

def quick_anthropic_example():
    """Simple Anthropic Claude integration example."""
    
    # Initialize
    init()
    
    # Configure Anthropic
    config = LlmConfig.anthropic(
        api_key=os.getenv("ANTHROPIC_API_KEY"),
        model="claude-3-5-haiku-20241022"
    )
    client = LlmClient(config)
    
    # Complex reasoning task
    response = client.complete(
        """Analyze the pros and cons of remote work from both 
        employee and employer perspectives. Be balanced and thorough.""",
        max_tokens=300,
        temperature=0.6
    )
    
    print(f"Claude's analysis: {response}")

# Usage (requires ANTHROPIC_API_KEY)
quick_anthropic_example()
```

## Best Practices

### Configuration Management

```python
from graphbit import init, LlmConfig, LlmClient
import time
import os

def setup_production_llm_config():
    """Set up production-ready LLM configuration."""
    
    init(log_level="warn", enable_tracing=False)
    
    # Primary provider with fallback
    providers = []
    
    if os.getenv("OPENAI_API_KEY"):
        providers.append(('openai', LlmConfig.openai(
            os.getenv("OPENAI_API_KEY"),
            "gpt-4o-mini"
        )))
    
    if os.getenv("ANTHROPIC_API_KEY"):
        providers.append(('anthropic', LlmConfig.anthropic(
            os.getenv("ANTHROPIC_API_KEY"),
            "claude-3-5-haiku-20241022"
        )))
    
    # Add local fallback
    try:
        providers.append(('ollama', LlmConfig.ollama("llama3.2")))
    except:
        pass
    
    if not providers:
        raise Exception("No LLM providers configured")
    
    return providers

def robust_completion(prompt: str, max_retries: int = 3):
    """Completion with provider fallback."""
    
    providers = setup_production_llm_config()
    
    for provider_name, config in providers:
        for attempt in range(max_retries):
            try:
                client = LlmClient(config, debug=False)
                return client.complete(prompt, max_tokens=200)
            except Exception as e:
                print(f"Attempt {attempt + 1} with {provider_name} failed: {e}")
                if attempt == max_retries - 1:
                    continue  # Try next provider
                time.sleep(2 ** attempt)  # Exponential backoff
    
    raise Exception("All providers failed")
```

## Key Features

### Provider Flexibility
- **Multiple Providers**: OpenAI, Anthropic, Ollama support
- **Easy Switching**: Consistent API across providers
- **Fallback Support**: Automatic provider failover

### Performance Optimization
- **Async Operations**: Non-blocking completions
- **Batch Processing**: Efficient multiple prompt handling
- **Streaming**: Real-time response streaming
- **Client Warmup**: Improved initial response times

### Monitoring and Reliability
- **Statistics Tracking**: Detailed performance metrics
- **Health Checks**: System health monitoring
- **Error Handling**: Comprehensive error management
- **Resilience Patterns**: Circuit breakers and retry logic

This example demonstrates GraphBit's comprehensive LLM integration capabilities for building production-ready AI applications.
