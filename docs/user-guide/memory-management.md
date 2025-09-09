# Memory Management

GraphBit provides comprehensive memory management features to optimize resource usage, monitor memory consumption, and configure memory-efficient execution environments. This guide covers all memory-related functionality available in GraphBit.

## Overview

GraphBit's memory management system includes:

- **Memory Allocators**: Optimized memory allocation strategies
- **Runtime Configuration**: Thread stack size and memory-efficient settings
- **Memory Monitoring**: Real-time memory usage tracking and health checks
- **Memory-Optimized Executors**: Executor configuration for resource-constrained environments
- **Concurrency Control**: Memory-aware concurrency limits
- **Caching Systems**: Intelligent caching to reduce memory pressure

## Memory Allocators

GraphBit uses different memory allocators depending on the platform for optimal performance:

### Checking Memory Allocator

```python
from graphbit import get_system_info

# Get system information including memory allocator
info = get_system_info()
print(f"Memory allocator: {info['memory_allocator']}")
print(f"CPU count: {info['cpu_count']}")
print(f"Runtime worker threads: {info['runtime_worker_threads']}")
```

### Platform-Specific Allocators

- **Linux/Unix**: Uses `jemalloc` for better memory efficiency and reduced fragmentation
- **Windows/Other**: Uses system default allocator
- **Python Bindings**: Uses system allocator to avoid TLS block allocation issues

## Runtime Memory Configuration

Configure memory-related runtime settings before initializing GraphBit:

### Basic Runtime Configuration

```python
from graphbit import configure_runtime, init

# Configure memory-optimized runtime settings
configure_runtime(
    worker_threads=2,           # Lower thread count for memory efficiency
    max_blocking_threads=4,     # Reduced blocking thread pool
    thread_stack_size_mb=1      # Smaller stack size (1MB per thread)
)
```

### Advanced Runtime Configuration

```python
from graphbit import configure_runtime, init, get_system_info

def setup_memory_optimized_runtime():
    """Configure runtime for memory-constrained environments."""
    
    # Get system capabilities
    info = get_system_info()
    cpu_count = info['cpu_count']
    
    # Configure conservative settings
    configure_runtime(
        worker_threads=max(2, cpu_count // 2),  # Half of available CPUs
        max_blocking_threads=max(4, cpu_count), # Equal to CPU count
        thread_stack_size_mb=1                  # Minimal stack size
    )
    
    init(debug=False)  # Disable debug mode to save memory
    
    print(f"Configured memory-optimized runtime:")
    print(f"  Worker threads: {max(2, cpu_count // 2)}")
    print(f"  Blocking threads: {max(4, cpu_count)}")
    print(f"  Stack size: 1MB per thread")

# Apply configuration
setup_memory_optimized_runtime()
```

## Memory Health Monitoring

GraphBit provides comprehensive memory health monitoring capabilities:

### Health Check API

```python
from graphbit import health_check

def check_memory_health():
    """Perform comprehensive memory health check."""
    
    health = health_check()
    
    print("Memory Health Status:")
    print(f"  Overall healthy: {health['overall_healthy']}")
    print(f"  Memory healthy: {health['memory_healthy']}")
    print(f"  Available memory: {health['available_memory_mb']}MB")
    print(f"  Total memory: {health.get('total_memory_mb', 'Unknown')}MB")
    
    # Check for memory warnings
    if not health['memory_healthy']:
        available = health['available_memory_mb']
        print(f"⚠️  Low memory warning: Only {available}MB available")
        print("   Consider using memory-optimized executor")
        return False
    
    print("✅ Memory status OK")
    return True

# Check memory before starting intensive operations
if check_memory_health():
    print("Safe to proceed with memory-intensive operations")
else:
    print("Consider memory optimization strategies")
```

### Continuous Memory Monitoring

```python
import time
from graphbit import health_check

def monitor_memory_with_graphbit():
    """Monitor memory usage using GraphBit's built-in health check."""

    def get_memory_info():
        """Get memory information using GraphBit's health check."""
        health = health_check()
        return {
            "available_memory_mb": health.get('available_memory_mb', 0),
            "total_memory_mb": health.get('total_memory_mb', 0),
            "memory_healthy": health.get('memory_healthy', True),
            "overall_healthy": health.get('overall_healthy', True)
        }

    return get_memory_info

def execute_with_memory_monitoring(executor, workflow):
    """Execute workflow with GraphBit's native memory monitoring."""

    get_memory_info = monitor_memory_with_graphbit()

    # Baseline measurement
    baseline = get_memory_info()
    print(f"Baseline memory status:")
    print(f"  Available: {baseline['available_memory_mb']}MB")
    print(f"  Memory healthy: {baseline['memory_healthy']}")

    # Execute with timing
    start_time = time.time()
    result = executor.execute(workflow)
    execution_time = (time.time() - start_time) * 1000

    # Final measurement
    final = get_memory_info()
    print(f"Final memory status:")
    print(f"  Available: {final['available_memory_mb']}MB")
    print(f"  Memory healthy: {final['memory_healthy']}")
    print(f"  Execution time: {execution_time:.1f}ms")

    # Calculate memory change
    if baseline['available_memory_mb'] and final['available_memory_mb']:
        memory_change = baseline['available_memory_mb'] - final['available_memory_mb']
        print(f"  Memory consumed: {memory_change}MB")

    return result
```

## Memory-Optimized Executors

### Executor Configuration Comparison

```python
from graphbit import LlmConfig, Executor
import os

def compare_executor_configurations():
    """Compare different executor configurations for memory usage."""

    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

    # Standard executor with default settings
    standard_executor = Executor(config, timeout_seconds=120)

    # Memory-optimized executor with efficient settings
    memory_executor = Executor(
        config,
        timeout_seconds=300,
        debug=False
    )

    # Configure memory executor for additional efficiency
    memory_executor.configure(
        timeout_seconds=300,
        max_retries=2,        # Fewer retries to save memory
        enable_metrics=False, # Disable metrics collection
        debug=False           # Disable debug mode
    )

    print("Executor Configurations:")
    print("1. Standard: Default settings")
    print("2. Memory-optimized: Configured for memory efficiency")

    return standard_executor, memory_executor
```

## Concurrency and Memory Management

GraphBit provides memory-aware concurrency configurations that balance performance with memory usage:

### Memory-Optimized Concurrency

```python
from graphbit import Workflow, Node, Executor, LlmConfig
import os

def create_memory_efficient_workflow():
    """Create workflow with memory-efficient concurrency settings."""

    # Configure for memory efficiency
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    executor = Executor(config, timeout_seconds=180, debug=False)

    # Create workflow with conservative node limits
    workflow = Workflow("Memory Efficient Workflow")

    # Add nodes with memory-conscious design
    processor = Node.agent(
        name="Memory Efficient Processor",
        prompt="Process this data efficiently: {input}",  # Concise prompt
        agent_id="mem_processor"
    )

    analyzer = Node.agent(
        name="Lightweight Analyzer",
        prompt="Analyze: {input}",  # Short prompt to reduce memory
        agent_id="analyzer"
    )

    # Build workflow
    proc_id = workflow.add_node(processor)
    analyzer_id = workflow.add_node(analyzer)
    workflow.connect(proc_id, analyzer_id)

    return workflow, executor

def demonstrate_concurrency_impact():
    """Demonstrate the impact of different concurrency settings on memory."""

    workflow, executor = create_memory_efficient_workflow()

    print("Memory-Optimized Concurrency Settings:")
    print("- Lower concurrent task limits")
    print("- Conservative resource allocation")
    print("- Reduced memory footprint per operation")

    # Execute with memory monitoring
    result = execute_with_memory_monitoring(executor, workflow)
    return result
```

### Concurrency Configuration Details

The memory-optimized configuration uses conservative limits:

- **Agent nodes**: Maximum 5 concurrent executions
- **HTTP requests**: Maximum 8 concurrent requests
- **Global concurrency**: Maximum 32 total concurrent operations

## Memory Optimization with GraphBit

### Executor Configuration Optimization

```python
from graphbit import LlmConfig, Executor, configure_runtime, init
import os

def optimize_executor_for_memory():
    """Configure GraphBit executor for optimal memory usage."""

    # Configure runtime for memory efficiency
    configure_runtime(
        worker_threads=2,           # Reduced thread count
        thread_stack_size_mb=1,     # Minimal stack size
        max_blocking_threads=4      # Limited blocking threads
    )

    init(debug=False)  # Disable debug mode to save memory

    # Create executor with memory-efficient settings
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    executor = Executor(
        config,
        timeout_seconds=300,
        debug=False
    )

    print("Memory-Optimized Executor Configuration:")
    print("- Reduced worker threads")
    print("- Minimal thread stack size")
    print("- Memory-efficient execution settings")
    print("- Debug mode disabled")

    return executor
```

### Workflow Design for Memory Efficiency

```python
from graphbit import Workflow, Node

def create_memory_efficient_workflow():
    """Create workflow optimized for memory usage with GraphBit."""

    workflow = Workflow("Memory Efficient Workflow")

    # Use concise node configurations
    processor = Node.agent(
        name="Efficient Processor",
        prompt="Process: {input}",  # Concise prompt
        agent_id="processor"
    )

    analyzer = Node.agent(
        name="Efficient Analyzer",
        prompt="Analyze: {input}",  # Short prompt
        agent_id="analyzer"
    )

    # Add nodes to workflow
    proc_id = workflow.add_node(processor)
    analyzer_id = workflow.add_node(analyzer)

    # Sequential connection for predictable memory usage
    workflow.connect(proc_id, analyzer_id)

    print("Memory-Efficient Workflow Features:")
    print("- Concise node prompts")
    print("- Sequential execution pattern")
    print("- Minimal node configuration")

    return workflow
```

## Performance Analysis and Memory Monitoring

### Memory Analysis with GraphBit's Native Tools

```python
import time
from graphbit import Executor, LlmConfig, Workflow, Node, health_check
import os

def profile_memory_usage():
    """Profile memory usage during GraphBit operations using native tools."""

    # Setup GraphBit components
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    executor = Executor(config, timeout_seconds=120, debug=False)

    workflow = Workflow("Memory Profile Test")
    node = Node.agent(
        name="Test Agent",
        prompt="Analyze this briefly: {input}",
        agent_id="test_agent"
    )
    workflow.add_node(node)

    # Take baseline measurement using GraphBit's health check
    baseline_health = health_check()
    baseline_available = baseline_health.get('available_memory_mb', 0)
    print(f"Baseline available memory: {baseline_available} MB")

    # Execute workflow
    start_time = time.time()
    result = executor.execute(workflow)
    execution_time = time.time() - start_time

    # Take final measurement
    final_health = health_check()
    final_available = final_health.get('available_memory_mb', 0)

    # Calculate memory usage
    memory_consumed = baseline_available - final_available

    print(f"Memory Analysis Results:")
    print(f"  Execution time: {execution_time:.2f}s")
    print(f"  Memory consumed: {memory_consumed} MB")
    print(f"  Final available memory: {final_available} MB")
    print(f"  Memory healthy: {final_health.get('memory_healthy', False)}")

    return {
        "execution_time": execution_time,
        "memory_consumed_mb": memory_consumed,
        "final_available_mb": final_available,
        "memory_healthy": final_health.get('memory_healthy', False),
        "result": result
    }

# Run memory profiling
profile_results = profile_memory_usage()
```


## Conclusion

GraphBit's memory management system provides comprehensive tools for optimizing memory usage across different environments and use cases. By utilizing GraphBit's built-in memory management features, you can effectively monitor memory health, optimize resource usage, and ensure efficient execution of your workflows.
