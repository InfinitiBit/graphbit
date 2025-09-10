# Debugging Guide

This guide covers debugging techniques for GraphBit development, with special focus on Python bindings and common development issues.

## Quick Debugging Checklist

When experiencing issues, work through this checklist:

1. **Environment Setup**
   - [ ] API keys configured (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`)
   - [ ] `ARGV0` is unset (`unset ARGV0`)
   - [ ] Python bindings installed (`maturin develop`)
   - [ ] Dependencies up to date

2. **Basic Health Check**
   ```python
   from graphbit import init, health_check, get_system_info
   
   init(debug=True)
   health = health_check()
   print(f"System healthy: {health['overall_healthy']}")
   ```

3. **Runtime Verification**
   ```python
   info = get_system_info()
   print(f"Runtime initialized: {info['runtime_initialized']}")
   print(f"Worker threads: {info['runtime_worker_threads']}")
   ```

## Python Bindings Debugging

### 1. Import and Initialization Issues

#### Problem: Module Import Fails

```bash
# Error: ModuleNotFoundError: No module named 'graphbit'
python -c "import graphbit"
```

**Solution:**
```bash
# Verify installation
pip list | grep graphbit

# Reinstall if missing
cd python
maturin develop

# Check Python path
python -c "import sys; print(sys.path)"
```

#### Problem: Initialization Errors

```python
# Error during init()
from graphbit import init, get_system_info
try:
    init(debug=True, log_level="debug")
    print("Initialization successful")
except Exception as e:
    print(f"Initialization failed: {e}")
    
    # Get system info to diagnose
    try:
        info = get_system_info()
        print(f"System info: {info}")
    except:
        print("Cannot get system info - core issue")
```

**Common Causes:**
- Missing environment variables
- Insufficient system resources
- Runtime configuration conflicts

### 2. LLM Client Issues

#### Problem: Configuration Errors

```python
from graphbit import init, LlmConfig, LlmClient

# Ensure debug mode is enabled
init(debug=True)

try:
    # Test OpenAI configuration
    config = LlmConfig.openai(api_key="test-key")
    client = LlmClient(config, debug=True)
    print("Client created successfully")
except ValueError as e:
    print(f"Configuration error: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

#### Problem: API Request Failures

```python
import os
from graphbit import init, LlmConfig, LlmClient

# Ensure debug mode is enabled
init(debug=True, log_level="debug")

# Check API key
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    print("ERROR: OPENAI_API_KEY not set")
    exit(1)

config = LlmConfig.openai(api_key=api_key)
client = LlmClient(config, debug=True)

try:
    # Test basic completion
    response = client.complete("Hello", max_tokens=10)
    print(f"Success: {response}")
except ConnectionError as e:
    print(f"Network error: {e}")
except TimeoutError as e:
    print(f"Timeout error: {e}")
except PermissionError as e:
    print(f"Authentication error: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
    
    # Get client statistics for diagnosis
    stats = client.get_stats()
    print(f"Client stats: {stats}")
```

#### Problem: Circuit Breaker Activation

```python
# Check client statistics to see circuit breaker state
stats = client.get_stats()
print(f"Circuit breaker state: {stats['circuit_breaker_state']}")
print(f"Failed requests: {stats['failed_requests']}")
print(f"Total requests: {stats['total_requests']}")

# Reset client if needed
client.reset_stats()
```

### 3. Workflow Execution Issues

#### Problem: Executor Configuration

```python
from graphbit import init, LlmConfig, Executor

# Ensure debug mode is enabled
init(debug=True)

config = LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"))

try:
    # Test different executor modes
    executor = Executor(config, debug=True)
    print(f"Executor mode: {executor.get_execution_mode()}")
    
    # Check executor statistics
    stats = executor.get_stats()
    print(f"Executor stats: {stats}")
    
except Exception as e:
    print(f"Executor creation failed: {e}")
```

#### Problem: Execution Timeouts

```python
# Configure shorter timeout for testing
executor = Executor(
    config, 
    lightweight_mode=True,
    timeout_seconds=10,  # Very short timeout
    debug=True
)

try:
    result = executor.execute(workflow)
    print("Execution successful")
except TimeoutError as e:
    print(f"Execution timed out: {e}")
    
    # Check execution statistics
    stats = executor.get_stats()
    print(f"Average duration: {stats['average_duration_ms']}ms")
```

### 4. Runtime and Performance Issues

#### Problem: Memory Issues

```python
from graphbit import init, health_check

# Ensure debug mode is enabled
init(debug=True)

# Check memory health
health = health_check()
print(f"Memory healthy: {health['memory_healthy']}")
print(f"Available memory: {health['available_memory_mb']}MB")

# Monitor memory usage during execution
import psutil
import os

process = psutil.Process(os.getpid())
print(f"Memory usage: {process.memory_info().rss / 1024 / 1024:.1f}MB")
```

#### Problem: Thread Pool Issues

```python
# Check runtime configuration
info = get_system_info()
print(f"Worker threads: {info['runtime_worker_threads']}")
print(f"Max blocking threads: {info['runtime_max_blocking_threads']}")
print(f"CPU count: {info['cpu_count']}")

# Configure custom runtime if needed
configure_runtime(
    worker_threads=4,
    max_blocking_threads=16,
    thread_stack_size_mb=2
)
```

## Rust Core Debugging

### 1. Compilation Issues

#### Problem: Build Failures

```bash
# Clean build cache
cargo clean
rm -rf target/

# Verbose build to see detailed errors
RUST_BACKTRACE=full cargo build 2>&1 | tee build.log

# Check for common issues
grep -i error build.log
grep -i warning build.log
```

#### Problem: Dependency Conflicts

```bash
# Update dependencies
cargo update

# Check for security issues
cargo audit

# Resolve version conflicts
cargo tree --duplicates
```

### 2. Runtime Issues

#### Problem: Panic Debugging

```bash
# Enable full backtraces
export RUST_BACKTRACE=full
export RUST_LOG=debug

# Run with debugging
cargo run

# Or for tests
cargo test -- --nocapture
```

#### Problem: Performance Issues

```bash
# Install profiling tools
cargo install flamegraph

# Profile the application
cargo flamegraph --bin graphbit

# Or profile tests
cargo flamegraph --test integration_tests
```

## Environment Debugging

### 1. API Key Issues

```bash
# Check environment variables
env | grep -E "(OPENAI|ANTHROPIC)_API_KEY"

# Test API key validity
curl -H "Authorization: Bearer $OPENAI_API_KEY" \
     https://api.openai.com/v1/models \
     | jq '.data[0].id'

# Or for Anthropic
curl -H "x-api-key: $ANTHROPIC_API_KEY" \
     -H "anthropic-version: 2023-06-01" \
     https://api.anthropic.com/v1/messages \
     -d '{"model":"claude-sonnet-4-20250514","max_tokens":10,"messages":[{"role":"user","content":"test"}]}'
```

### 2. Network Issues

```python
# Test network connectivity
import requests

try:
    response = requests.get("https://api.openai.com/v1/models", timeout=10)
    print(f"OpenAI API accessible: {response.status_code}")
except Exception as e:
    print(f"Network issue: {e}")

# Test with GraphBit client
from graphbit import init, LlmConfig, LlmClient

# Ensure debug mode is enabled
init(debug=True)

config = LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"))
client = LlmClient(config, debug=True)

try:
    # Warmup to test connectivity
    await client.warmup()
    print("Warmup successful - network OK")
except Exception as e:
    print(f"Network/API issue: {e}")
```

## Logging and Tracing

### 1. Enable Comprehensive Logging

```python
# Maximum debugging information
from graphbit import init

init(
    debug=True,
    log_level="trace",  # Most verbose
    enable_tracing=True
)

# All operations will now have detailed logging
```

### 2. Rust-Level Logging

```bash
# Environment variable for Rust logging
export RUST_LOG=graphbit=trace,debug
export RUST_BACKTRACE=full

# Run your application
python your_script.py
```

### 3. Custom Logging

```python
import logging
from graphbit import init 

# Configure Python logging
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

# Initialize GraphBit with debug
init(debug=True, log_level="debug")

# Your operations will now have detailed logs
```

## Performance Debugging

### 1. Python Profiling

```bash
# Install profiling tools
pip install py-spy memory-profiler

# Profile CPU usage
py-spy record -o profile.svg -- python your_script.py

# Profile memory usage
mprof run your_script.py
mprof plot
```

### 2. Memory Leak Detection

```python
import tracemalloc
from graphbit import init

# Start memory tracing
tracemalloc.start()

# Your GraphBit operations
init()
# ... perform operations ...

# Check memory usage
current, peak = tracemalloc.get_traced_memory()
print(f"Current memory usage: {current / 1024 / 1024:.1f} MB")
print(f"Peak memory usage: {peak / 1024 / 1024:.1f} MB")
tracemalloc.stop()
```

### 3. Performance Monitoring

```python
import time
from graphbit import init, LlmConfig, LlmClient

# Ensure debug mode is enabled
init(debug=True)

config = LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"))
client = LlmClient(config, debug=True)

# Monitor operation timing
start_time = time.time()
try:
    response = client.complete("Hello", max_tokens=10)
    duration = time.time() - start_time
    print(f"Request completed in {duration:.2f}s")
    
    # Check client statistics
    stats = client.get_stats()
    print(f"Average response time: {stats['average_response_time_ms']}ms")
    
except Exception as e:
    duration = time.time() - start_time
    print(f"Request failed after {duration:.2f}s: {e}")
```

## Common Error Patterns

### 1. Authentication Errors

```
PermissionError: Authentication error with OpenAI: Invalid API key
```

**Solutions:**
- Verify API key is correct and active
- Check API key has proper permissions
- Ensure environment variable is set correctly

### 2. Network Errors

```
ConnectionError: Network error (attempt 1): Connection timeout
```

**Solutions:**
- Check internet connectivity
- Verify API endpoints are accessible
- Consider firewall/proxy issues
- Increase timeout settings

### 3. Resource Errors

```
MemoryError: Resource exhausted (memory): Out of memory
```

**Solutions:**
- Reduce concurrency settings
- Use memory-optimized execution mode
- Monitor memory usage
- Check for memory leaks

### 4. Timeout Errors

```
TimeoutError: Timeout in 'llm_completion' after 30000ms: Request timeout
```

**Solutions:**
- Increase timeout settings
- Use appropriate execution mode
- Check network latency
- Consider model/provider performance

## Getting Help

When debugging fails:

1. **Collect Information:**
   ```bash
   # System information
   python -c "from graphbit import get_system_info; print(get_system_info())"
   
   # Health check
   python -c "from graphbit import health_check; print(health_check())"
   
   # Environment
   env | grep -E "(RUST_|GRAPHBIT_|OPENAI_|ANTHROPIC_)"
   ```

2. **Create Minimal Reproduction:**
   ```python
   import os
   from graphbit import init, LlmConfig, LlmClient

   # Minimal failing example
   init(debug=True)
   
   config = LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"))
   client = LlmClient(config, debug=True)
   
   try:
       response = client.complete("test", max_tokens=5)
       print("Success:", response)
   except Exception as e:
       print("Error:", e)
       stats = client.get_stats()
       print("Stats:", stats)
   ```

3. **Submit Issue with:**
   - System information output
   - Full error traceback
   - Minimal reproduction code
   - Environment details
   - Steps attempted

This debugging guide should help identify and resolve most common issues in GraphBit development. 
