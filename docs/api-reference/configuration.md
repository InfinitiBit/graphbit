# Configuration Options

GraphBit provides extensive configuration options to customize workflow execution, LLM providers, reliability features, and performance settings.

## LLM Configuration

### OpenAI Configuration

```python
import graphbit

# Basic OpenAI configuration
config = graphbit.PyLlmConfig.openai(
    api_key="your-api-key",
    model="gpt-4o-mini"
)

# With additional parameters
config = graphbit.PyLlmConfig.openai(
    api_key="your-api-key",
    model="gpt-4o-mini",
    base_url="https://api.openai.com/v1",  # Custom endpoint
    organization="your-org-id"  # Optional organization
)
```

### Anthropic Configuration

```python
# Basic Anthropic configuration
config = graphbit.PyLlmConfig.anthropic(
    api_key="your-anthropic-key",
    model="claude-3-haiku-20240307"
)

# With custom endpoint
config = graphbit.PyLlmConfig.anthropic(
    api_key="your-anthropic-key",
    model="claude-3-sonnet-20240229",
    base_url="https://api.anthropic.com"
)
```

### HuggingFace Configuration

```python
# HuggingFace configuration
config = graphbit.PyLlmConfig.huggingface(
    api_key="your-hf-token",
    model="microsoft/DialoGPT-medium"
)
```

## Executor Configuration

### Basic Executor

```python
# Simple executor
executor = graphbit.PyWorkflowExecutor(config)
```

### Executor with Retry Policy

```python
# Configure retry behavior
retry_config = graphbit.PyRetryConfig.default() \
    .with_max_attempts(5) \
    .with_exponential_backoff(1000, 2.0, 30000) \
    .with_jitter(0.2)

executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(retry_config)
```

### Executor with Circuit Breaker

```python
# Configure circuit breaker
circuit_breaker_config = graphbit.PyCircuitBreakerConfig(
    failure_threshold=5,
    timeout_duration=60000  # 60 seconds
)

executor = graphbit.PyWorkflowExecutor(config) \
    .with_circuit_breaker_config(circuit_breaker_config)
```

### Complete Executor Configuration

```python
# Full configuration
executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(retry_config) \
    .with_circuit_breaker_config(circuit_breaker_config) \
    .with_timeout(120000)  # 2 minutes timeout
```

## Retry Configuration

### Default Retry Policy

```python
# Use default retry settings
retry_config = graphbit.PyRetryConfig.default()
```

### Custom Retry Policy

```python
# Custom retry configuration
retry_config = graphbit.PyRetryConfig.default() \
    .with_max_attempts(3) \
    .with_exponential_backoff(
        initial_delay=500,    # 500ms initial delay
        multiplier=2.0,       # Double delay each time
        max_delay=10000       # Cap at 10 seconds
    ) \
    .with_jitter(0.1)        # Add 10% random jitter
```

### Retry Policy Options

| Method | Description | Default |
|--------|-------------|---------|
| `with_max_attempts(n)` | Maximum retry attempts | 3 |  
| `with_exponential_backoff(initial, multiplier, max)` | Backoff strategy | 1000ms, 2.0, 30000ms |
| `with_jitter(factor)` | Random delay factor | 0.1 |
| `with_fixed_delay(ms)` | Fixed delay between retries | Not used |

## Circuit Breaker Configuration

### Basic Circuit Breaker

```python
# Default circuit breaker
circuit_config = graphbit.PyCircuitBreakerConfig(
    failure_threshold=5,      # Trip after 5 failures
    timeout_duration=30000    # 30 second timeout
)
```

### Advanced Circuit Breaker

```python
# Advanced configuration
circuit_config = graphbit.PyCircuitBreakerConfig(
    failure_threshold=10,     # Higher threshold
    timeout_duration=60000,   # 1 minute timeout
    half_open_max_calls=3     # Test calls in half-open state
)
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

### Optional Environment Variables

```bash
# Custom endpoints
export OPENAI_BASE_URL="https://your-custom-endpoint.com"
export ANTHROPIC_BASE_URL="https://your-anthropic-endpoint.com"

# Organization settings
export OPENAI_ORGANIZATION="your-org-id"

# Logging
export GRAPHBIT_LOG_LEVEL="INFO"
export GRAPHBIT_LOG_FORMAT="json"
```

## Execution Context Configuration

### Basic Context

```python
# Create execution context
context = graphbit.PyExecutionContext()
context.set_variable("input", "Hello World")
context.set_variable("user_id", "12345")
```

### Context with Metadata

```python
# Context with execution metadata
context = graphbit.PyExecutionContext()
context.set_variable("input", data)
context.set_metadata("execution_id", "exec_001")
context.set_metadata("user_session", session_id)
context.set_metadata("timestamp", datetime.now().isoformat())
```

## Performance Configuration

### Low Latency Executor

```python
# Optimized for speed
executor = graphbit.PyWorkflowExecutor.new_low_latency(config)
```

### High Throughput Executor

```python
# Optimized for throughput
executor = graphbit.PyWorkflowExecutor.new_high_throughput(config)
```

### Memory Optimized Executor

```python
# Optimized for memory usage
executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config)
```

## Logging Configuration

### Enable Logging

```python
import logging

# Configure Python logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

# GraphBit will use the configured logger
graphbit.init()
```

### Structured Logging

```python
import json
import sys

# JSON logging format
class JsonFormatter(logging.Formatter):
    def format(self, record):
        log_entry = {
            'timestamp': self.formatTime(record),
            'level': record.levelname,
            'message': record.getMessage(),
            'module': record.module
        }
        return json.dumps(log_entry)

handler = logging.StreamHandler(sys.stdout)
handler.setFormatter(JsonFormatter())

logger = logging.getLogger()
logger.addHandler(handler)
logger.setLevel(logging.INFO)
```

## Configuration Examples

### Development Configuration

```python
def create_dev_config():
    """Configuration for development environment."""
    
    # Faster model for development
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-3.5-turbo"  # Faster and cheaper
    )
    
    # Aggressive retry for development
    retry_config = graphbit.PyRetryConfig.default() \
        .with_max_attempts(2) \
        .with_fixed_delay(1000)
    
    # Relaxed circuit breaker
    circuit_config = graphbit.PyCircuitBreakerConfig(
        failure_threshold=10,
        timeout_duration=120000
    )
    
    executor = graphbit.PyWorkflowExecutor(config) \
        .with_retry_config(retry_config) \
        .with_circuit_breaker_config(circuit_config)
    
    return executor
```

### Production Configuration

```python
def create_prod_config():
    """Configuration for production environment."""
    
    # High-quality model for production
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Conservative retry policy
    retry_config = graphbit.PyRetryConfig.default() \
        .with_max_attempts(5) \
        .with_exponential_backoff(2000, 2.0, 60000) \
        .with_jitter(0.3)
    
    # Strict circuit breaker
    circuit_config = graphbit.PyCircuitBreakerConfig(
        failure_threshold=3,
        timeout_duration=30000
    )
    
    executor = graphbit.PyWorkflowExecutor(config) \
        .with_retry_config(retry_config) \
        .with_circuit_breaker_config(circuit_config) \
        .with_timeout(180000)  # 3 minutes
    
    return executor
```

### High-Volume Configuration

```python
def create_high_volume_config():
    """Configuration for high-volume processing."""
    
    # Fast, cost-effective model
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-3.5-turbo"
    )
    
    # Minimal retry to avoid delays
    retry_config = graphbit.PyRetryConfig.default() \
        .with_max_attempts(2) \
        .with_fixed_delay(500)
    
    # Quick circuit breaker recovery
    circuit_config = graphbit.PyCircuitBreakerConfig(
        failure_threshold=5,
        timeout_duration=15000  # 15 seconds
    )
    
    # Use high-throughput executor
    executor = graphbit.PyWorkflowExecutor.new_high_throughput(config) \
        .with_retry_config(retry_config) \
        .with_circuit_breaker_config(circuit_config)
    
    return executor
```

## Configuration Validation

### Validate Configuration

```python
def validate_config():
    """Validate configuration before use."""
    
    # Check environment variables
    required_vars = ["OPENAI_API_KEY"]
    for var in required_vars:
        if not os.getenv(var):
            raise ValueError(f"Required environment variable {var} not set")
    
    # Test configuration
    try:
        config = graphbit.PyLlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o-mini"
        )
        
        executor = graphbit.PyWorkflowExecutor(config)
        print("✅ Configuration valid")
        return executor
        
    except Exception as e:
        print(f"❌ Configuration invalid: {e}")
        raise
```

### Configuration Health Check

```python
def health_check(executor):
    """Check if executor is healthy."""
    
    try:
        # Create simple test workflow
        builder = graphbit.PyWorkflowBuilder("Health Check")
        
        test_node = graphbit.PyWorkflowNode.agent_node(
            "Test Node", "Health check", "test", "Say 'OK'"
        )
        
        builder.add_node(test_node)
        workflow = builder.build()
        
        # Execute with timeout
        result = executor.execute(workflow)
        
        if result.is_completed():
            print("✅ Executor healthy")
            return True
        else:
            print("❌ Executor unhealthy")
            return False
            
    except Exception as e:
        print(f"❌ Health check failed: {e}")
        return False
```

## Best Practices

### 1. Environment-Specific Configuration

```python
def get_config_for_environment():
    """Get configuration based on environment."""
    
    env = os.getenv("ENVIRONMENT", "development")
    
    if env == "production":
        return create_prod_config()
    elif env == "staging":
        return create_staging_config()
    else:
        return create_dev_config()
```

### 2. Secure API Key Management

```python
# Use environment variables
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    raise ValueError("OPENAI_API_KEY environment variable required")

config = graphbit.PyLlmConfig.openai(api_key=api_key, model="gpt-4o-mini")
```

### 3. Configuration Monitoring

```python
def monitor_config_performance(executor):
    """Monitor configuration performance."""
    
    import time
    
    start_time = time.time()
    
    # Execute test workflow
    result = executor.execute(test_workflow)
    
    end_time = time.time()
    execution_time = end_time - start_time
    
    # Log performance metrics
    logging.info(f"Execution time: {execution_time:.2f}s")
    
    if execution_time > 30:  # 30 second threshold
        logging.warning("Slow execution detected - consider tuning configuration")
    
    return result
```

### 4. Graceful Degradation

```python
def create_fallback_config():
    """Create fallback configuration for degraded service."""
    
    # Use faster, simpler model as fallback
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-3.5-turbo"
    )
    
    # More aggressive retry for fallback
    retry_config = graphbit.PyRetryConfig.default() \
        .with_max_attempts(1) \
        .with_fixed_delay(100)
    
    executor = graphbit.PyWorkflowExecutor(config) \
        .with_retry_config(retry_config) \
        .with_timeout(30000)  # 30 seconds
    
    return executor
```

Proper configuration is essential for optimal GraphBit performance. Choose settings that match your use case, environment, and performance requirements. 