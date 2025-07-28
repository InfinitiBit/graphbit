# Configuration Options

GraphBit provides extensive configuration options to customize workflow execution, LLM providers, reliability features, and performance settings.

## Library Initialization

### Basic Initialization

```python
import graphbit

# Basic initialization
graphbit.init()
```

### Advanced Initialization

```python
# With debugging and logging
graphbit.init(
    log_level="info",          # Log level: trace, debug, info, warn, error
    enable_tracing=True,       # Enable detailed tracing
    debug=True                 # Enable debug mode (alias for enable_tracing)
)
```

### Runtime Configuration

Configure the runtime before initialization for advanced control:

```python
# Configure runtime (call before init)
graphbit.configure_runtime(
    worker_threads=8,          # Number of worker threads
    max_blocking_threads=16,   # Maximum blocking threads
    thread_stack_size_mb=8     # Thread stack size in MB
)

# Then initialize
graphbit.init()
```

## LLM Configuration

### OpenAI Configuration

```python
# Basic OpenAI configuration
config = graphbit.LlmConfig.openai(
    api_key="your-api-key",
    model="gpt-4o-mini"        # Optional, defaults to gpt-4o-mini
)

# With default model
config = graphbit.LlmConfig.openai("your-api-key")
```

### Anthropic Configuration

```python
# Basic Anthropic configuration
config = graphbit.LlmConfig.anthropic(
    api_key="your-anthropic-key",
    model="claude-3-5-sonnet-20241022"  # Optional, defaults to claude-3-5-sonnet-20241022
)

# With default model
config = graphbit.LlmConfig.anthropic("your-anthropic-key")
```

### Perplexity Configuration

```python
# Basic Perplexity configuration
config = graphbit.LlmConfig.perplexity(
    api_key="your-perplexity-key",
    model="sonar"             # Optional, defaults to sonar
)

# With default model
config = graphbit.LlmConfig.perplexity("your-perplexity-key")
=======
### DeepSeek Configuration

```python
# Basic DeepSeek configuration
config = graphbit.LlmConfig.deepseek(
    api_key="your-deepseek-key",
    model="deepseek-chat"        # Optional, defaults to deepseek-chat
)

# With default model
config = graphbit.LlmConfig.deepseek("your-deepseek-key")

# Different models for specific use cases
coding_config = graphbit.LlmConfig.deepseek("your-deepseek-key", "deepseek-coder")
reasoning_config = graphbit.LlmConfig.deepseek("your-deepseek-key", "deepseek-reasoner")
```

### Ollama Configuration

```python
# Local Ollama configuration
config = graphbit.LlmConfig.ollama(
    model="llama3.2"          # Optional, defaults to llama3.2
)

# With default model
config = graphbit.LlmConfig.ollama()
```

### Configuration Properties

```python
# Access configuration properties
provider = config.provider()  # "openai", "anthropic", "perplexity", "ollama"
=======
provider = config.provider()  # "openai", "anthropic", "deepseek", "ollama"
model = config.model()        # Model name
```

## LLM Client Configuration

### Basic Client

```python
# Simple client
client = graphbit.LlmClient(config)
```

### Client with Debug Mode

```python
# Client with debugging enabled
client = graphbit.LlmClient(config, debug=True)
```

### Client Statistics and Monitoring

```python
# Get performance statistics
stats = client.get_stats()
print(f"Total requests: {stats['total_requests']}")
print(f"Success rate: {stats['success_rate']}")
print(f"Average response time: {stats['average_response_time_ms']}ms")

# Reset statistics
client.reset_stats()

# Warm up client for better performance
import asyncio
asyncio.run(client.warmup())
```

## Executor Configuration

### Basic Executor

```python
# Simple executor
executor = graphbit.Executor(llm_config)
```

### Executor with Options

```python
# Executor with configuration
executor = graphbit.Executor(
    config=llm_config,
    lightweight_mode=False,    # Enable lightweight/low-latency mode
    timeout_seconds=300,       # Execution timeout (1-3600 seconds)
    debug=True                 # Enable debug mode
)
```

### Specialized Executors

#### High Throughput Executor

```python
# Optimized for high throughput
executor = graphbit.Executor.new_high_throughput(
    llm_config,
    timeout_seconds=600,       # Optional timeout override
    debug=False                # Optional debug mode
)
```

#### Low Latency Executor

```python
# Optimized for low latency
executor = graphbit.Executor.new_low_latency(
    llm_config,
    timeout_seconds=30,        # Shorter timeout for low latency
    debug=False
)
```

#### Memory Optimized Executor

```python
# Optimized for memory usage
executor = graphbit.Executor.new_memory_optimized(
    llm_config,
    timeout_seconds=300,
    debug=False
)
```

### Runtime Configuration

```python
# Configure executor settings
executor.configure(
    timeout_seconds=600,       # Execution timeout (1-3600 seconds)
    max_retries=5,            # Maximum retries (0-10)
    enable_metrics=True,      # Enable performance metrics
    debug=False               # Debug mode
)

# Legacy configuration methods
executor.set_lightweight_mode(True)  # Enable lightweight mode
is_lightweight = executor.is_lightweight_mode()  # Check mode
```

### Executor Statistics

```python
# Get execution statistics
stats = executor.get_stats()
print(f"Total executions: {stats['total_executions']}")
print(f"Success rate: {stats['success_rate']}")
print(f"Average duration: {stats['average_duration_ms']}ms")
print(f"Execution mode: {stats['execution_mode']}")

# Reset statistics
executor.reset_stats()

# Get current execution mode
mode = executor.get_execution_mode()  # Returns: HighThroughput, LowLatency, etc.
```

## Embeddings Configuration

### OpenAI Embeddings

```python
# OpenAI embeddings configuration
embed_config = graphbit.EmbeddingConfig.openai(
    api_key="your-api-key",
    model="text-embedding-3-small"  # Optional, defaults to text-embedding-3-small
)

# With default model
embed_config = graphbit.EmbeddingConfig.openai("your-api-key")
```

### HuggingFace Embeddings

```python
# HuggingFace embeddings configuration
embed_config = graphbit.EmbeddingConfig.huggingface(
    api_key="your-hf-token",
    model="sentence-transformers/all-MiniLM-L6-v2"
)
```

### Embeddings Client

```python
# Create embeddings client
embed_client = graphbit.EmbeddingClient(embed_config)

# Generate embeddings
embedding = embed_client.embed("Hello world")
embeddings = embed_client.embed_many(["Text 1", "Text 2"])

# Calculate similarity
similarity = graphbit.EmbeddingClient.similarity(embedding1, embedding2)
```

## Environment Variables

### Required Environment Variables

```bash
# OpenAI
export OPENAI_API_KEY="your-openai-api-key"

# Anthropic
export ANTHROPIC_API_KEY="your-anthropic-api-key"

# HuggingFace
export HUGGINGFACE_API_KEY="your-huggingface-token"
```

### GraphBit-Specific Environment Variables

```bash
# Runtime configuration
export GRAPHBIT_WORKER_THREADS="8"
export GRAPHBIT_MAX_BLOCKING_THREADS="16"

# Logging
export GRAPHBIT_LOG_LEVEL="INFO"
export GRAPHBIT_DEBUG="true"
```

## System Information and Health

### System Information

```python
# Get comprehensive system information
info = graphbit.get_system_info()
print(f"Version: {info['version']}")
print(f"CPU count: {info['cpu_count']}")
print(f"Runtime initialized: {info['runtime_initialized']}")
print(f"Worker threads: {info['runtime_worker_threads']}")
print(f"Memory allocator: {info['memory_allocator']}")
```

### Health Checks

```python
# Perform health check
health = graphbit.health_check()
if health['overall_healthy']:
    print("✅ System is healthy")
    print(f"Memory healthy: {health['memory_healthy']}")
    print(f"Runtime healthy: {health['runtime_healthy']}")
else:
    print("❌ System has issues")
```

### Version Information

```python
# Get current version
version = graphbit.version()
print(f"GraphBit version: {version}")
```

## Configuration Examples

### Development Configuration

```python
def create_dev_config():
    """Configuration for development environment."""
    
    # Initialize with debugging
    graphbit.init(debug=True, log_level="info")
    
    # Use faster, cheaper model for development
    config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Low-latency executor for development
    executor = graphbit.Executor.new_low_latency(
        config, 
        timeout_seconds=60,
        debug=True
    )
    
    return executor
```

### Production Configuration

```python
def create_prod_config():
    """Configuration for production environment."""
    
    # Initialize without debugging
    graphbit.init(debug=False, log_level="warn")
    
    # High-quality model for production
    config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # High-throughput executor for production
    executor = graphbit.Executor.new_high_throughput(
        config,
        timeout_seconds=300,
        debug=False
    )
    
    # Configure for production reliability
    executor.configure(
        timeout_seconds=300,
        max_retries=3,
        enable_metrics=True,
        debug=False
    )
    
    return executor
```

### High-Volume Configuration

```python
def create_high_volume_config():
    """Configuration for high-volume processing."""
    
    # Configure runtime for high throughput
    graphbit.configure_runtime(
        worker_threads=16,
        max_blocking_threads=32
    )
    graphbit.init(debug=False)
    
    # Fast, cost-effective model
    config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Memory-optimized executor
    executor = graphbit.Executor.new_memory_optimized(
        config,
        timeout_seconds=180,
        debug=False
    )
    
    return executor
```

### Local Development Configuration

```python
def create_local_config():
    """Configuration for local development with Ollama."""
    
    graphbit.init(debug=True, log_level="debug")
    
    # Use local Ollama
    config = graphbit.LlmConfig.ollama("llama3.2")
    
    # Low-latency for quick iteration
    executor = graphbit.Executor.new_low_latency(
        config,
        timeout_seconds=180,  # Longer timeout for local inference
        debug=True
    )
    
    return executor
```

## Configuration Validation

### Environment Validation

```python
def validate_environment():
    """Validate environment setup."""
    
    import os
    
    # Check required environment variables
    required_vars = {
        "OPENAI_API_KEY": "OpenAI API key",
        # Add others as needed
    }
    
    missing_vars = []
    for var, description in required_vars.items():
        if not os.getenv(var):
            missing_vars.append(f"{var} ({description})")
    
    if missing_vars:
        raise ValueError(f"Missing environment variables: {', '.join(missing_vars)}")
    
    print("✅ Environment validation passed")
```

### Configuration Testing

```python
def test_configuration(config):
    """Test LLM configuration."""
    
    try:
        # Create client
        client = graphbit.LlmClient(config)
        
        # Test simple completion
        response = client.complete("Say 'Configuration test successful'")
        
        if "successful" in response.lower():
            print("Configuration test passed")
            return True
        else:
            print("Configuration test failed - unexpected response")
            return False
            
    except Exception as e:
        print(f"Configuration test failed: {e}")
        return False
```

### Health Check Function

```python
def comprehensive_health_check():
    """Comprehensive system health check."""
    
    # Check system health
    health = graphbit.health_check()
    if not health['overall_healthy']:
        print("System health check failed")
        return False
    
    # Check system info
    info = graphbit.get_system_info()
    if not info['runtime_initialized']:
        print("Runtime not initialized")
        return False
    
    # Test basic functionality
    try:
        config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"))
        client = graphbit.LlmClient(config)
        
        # Quick test
        response = client.complete("Test")
        if not response:
            print("LLM test failed")
            return False
            
    except Exception as e:
        print(f"❌ LLM test failed: {e}")
        return False
    
    print("✅ Comprehensive health check passed")
    return True
```

## Best Practices

### 1. Environment-Based Configuration

```python
def get_config_for_environment():
    """Get configuration based on environment."""
    
    env = os.getenv("ENVIRONMENT", "development")
    
    if env == "production":
        return create_prod_config()
    elif env == "staging":
        return create_staging_config()
    elif env == "local":
        return create_local_config()
    else:
        return create_dev_config()
```

### 2. Secure Configuration

```python
def secure_config_setup():
    """Set up configuration securely."""
    
    # Validate API key exists
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise ValueError("OPENAI_API_KEY environment variable required")
    
    # Validate API key format (basic check)
    if len(api_key) < 20:
        raise ValueError("Invalid API key format")
    
    config = graphbit.LlmConfig.openai(api_key)
    return config
```

### 3. Performance Monitoring

```python
def monitor_performance(executor):
    """Monitor executor performance."""
    
    import time
    
    # Get initial stats
    initial_stats = executor.get_stats()
    
    start_time = time.time()
    
    # Your workflow execution here
    # result = executor.execute(workflow)
    
    end_time = time.time()
    execution_time = end_time - start_time
    
    # Get final stats
    final_stats = executor.get_stats()
    
    # Log performance metrics
    print(f"Execution time: {execution_time:.2f}s")
    print(f"Total executions: {final_stats['total_executions']}")
    print(f"Success rate: {final_stats['success_rate']:.2%}")
    
    # Alert on performance issues
    if execution_time > 60:  # 60 second threshold
        print("Slow execution detected - consider tuning configuration")
    
    if final_stats['success_rate'] < 0.95:  # 95% success rate threshold
        print("Low success rate - check configuration and API health")
```

### 4. Graceful Error Handling

```python
def robust_config_creation():
    """Create configuration with fallback options."""
    
    try:
        # Primary configuration
        config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"))
        executor = graphbit.Executor.new_high_throughput(config)
        
        # Test configuration
        if test_configuration(config):
            return executor
        else:
            raise Exception("Configuration test failed")
            
    except Exception as e:
        print(f"Primary configuration failed: {e}")
        
        try:
            # Fallback to local Ollama
            print("Falling back to local Ollama...")
            fallback_config = graphbit.LlmConfig.ollama()
            fallback_executor = graphbit.Executor.new_low_latency(fallback_config)
            
            if test_configuration(fallback_config):
                return fallback_executor
            else:
                raise Exception("Fallback configuration failed")
                
        except Exception as fallback_error:
            print(f"Fallback configuration failed: {fallback_error}")
            raise Exception("All configuration options exhausted")
```

### 5. Resource Cleanup

```python
def cleanup_resources():
    """Clean up GraphBit resources."""
    
    try:
        # Shutdown GraphBit (for testing/cleanup)
        graphbit.shutdown()
        print("Resources cleaned up successfully")
    except Exception as e:
        print(f"Error during cleanup: {e}")
```

## Configuration Troubleshooting

### Common Issues and Solutions

#### 1. API Key Issues

```python
# Check API key validity
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    print("OPENAI_API_KEY not set")
elif len(api_key) < 20:
    print("API key appears invalid (too short)")
elif not api_key.startswith("sk-"):
    print("OpenAI API key should start with 'sk-'")
else:
    print("API key format looks correct")
```

#### 2. Runtime Issues

```python
# Check runtime status
info = graphbit.get_system_info()
if not info['runtime_initialized']:
    print("Runtime not initialized - call graphbit.init()")
else:
    print(f"Runtime initialized with {info['runtime_worker_threads']} workers")
```

#### 3. Memory Issues

```python
# Check memory status
health = graphbit.health_check()
if not health['memory_healthy']:
    print(f"Low memory: {health['available_memory_mb']}MB available")
    print("Consider using memory-optimized executor")
else:
    print("Memory status OK")
```

Proper configuration is essential for optimal GraphBit performance. Choose settings that match your use case, environment, and performance requirements.
