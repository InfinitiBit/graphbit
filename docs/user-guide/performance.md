# Performance Optimization

GraphBit provides multiple strategies for optimizing workflow performance, from execution patterns to resource management. This guide covers techniques to maximize throughput, minimize latency, and optimize resource usage.

## Overview

Performance optimization in GraphBit focuses on:
- **Execution Optimization**: Parallel processing and efficient node execution
- **Resource Management**: Memory, CPU, and network optimization  
- **Caching Strategies**: Reducing redundant computations
- **Configuration Tuning**: Optimal settings for different scenarios
- **Monitoring & Profiling**: Identifying and resolving bottlenecks

## Execution Optimization

### Parallel Processing

```python
import graphbit
import time
import os

# Initialize GraphBit
graphbit.init()

def create_parallel_workflow():
    """Create workflow optimized for parallel execution."""
    
    workflow = graphbit.Workflow("Parallel Processing Workflow")
    
    # Input processor
    input_processor = graphbit.Node.agent(
        name="Input Processor",
        prompt="Prepare data for parallel processing: {input}",
        agent_id="input_processor"
    )
    
    # Parallel processing branches
    branch_nodes = []
    for i in range(4):  # Create 4 parallel branches
        branch = graphbit.Node.agent(
            name=f"Parallel Branch {i+1}",
            prompt=f"Process branch {i+1} data: {{prepared_data}}",
            agent_id=f"branch_{i+1}"
        )
        branch_nodes.append(branch)
    
    # Results aggregator
    aggregator = graphbit.Node.agent(
        name="Results Aggregator",
        prompt="""
        Combine results from parallel branches:
        Branch 1: {branch_1_output}
        Branch 2: {branch_2_output}
        Branch 3: {branch_3_output}
        Branch 4: {branch_4_output}
        """,
        agent_id="aggregator"
    )
    
    # Build parallel structure
    input_id = workflow.add_node(input_processor)
    branch_ids = [workflow.add_node(branch) for branch in branch_nodes]
    agg_id = workflow.add_node(aggregator)
    
    # Connect input to all branches (fan-out)
    for branch_id in branch_ids:
        workflow.connect(input_id, branch_id)
    
    # Connect all branches to aggregator (fan-in)
    for branch_id in branch_ids:
        workflow.connect(branch_id, agg_id)
    
    return workflow

def create_performance_optimized_executor():
    """Create executor optimized for performance."""
    
    # Use fast, cost-effective model
    config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"  # Fast and efficient model
    )
    
    # Create high-performance executor
    executor = graphbit.Executor.new_high_throughput(
        llm_config=config,
        timeout_seconds=60,
        debug=False
    )
    
    return executor
```

## Executor Types for Performance

### Different Executor Configurations

```python
def create_optimized_executors():
    """Create different executor types for various performance needs."""
    
    # Base LLM configuration
    llm_config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    executors = {}
    
    # Standard executor - balanced performance
    executors["standard"] = graphbit.Executor(
        config=llm_config,
        timeout_seconds=60,
        debug=False
    )
    
    # High-throughput executor - maximize parallel processing
    executors["high_throughput"] = graphbit.Executor.new_high_throughput(
        llm_config=llm_config,
        timeout_seconds=120,
        debug=False
    )
    
    # Low-latency executor - minimize response time
    executors["low_latency"] = graphbit.Executor.new_low_latency(
        llm_config=llm_config,
        timeout_seconds=30,
        debug=False
    )
    
    # Memory-optimized executor - minimize memory usage
    executors["memory_optimized"] = graphbit.Executor.new_memory_optimized(
        llm_config=llm_config,
        timeout_seconds=90,
        debug=False
    )
    
    return executors

def benchmark_executor_types(workflow, test_input):
    """Benchmark different executor types."""
    
    executors = create_optimized_executors()
    results = {}
    
    for executor_type, executor in executors.items():
        print(f"Benchmarking {executor_type} executor...")
        
        start_time = time.time()
        
        try:
            result = executor.execute(workflow)
            duration_ms = (time.time() - start_time) * 1000
            
            results[executor_type] = {
                "duration_ms": duration_ms,
                "success": result.is_completed(),
                "output_length": len(result.output()) if result.is_completed() else 0
            }
            
            print(f"  ✅ {executor_type}: {duration_ms:.1f}ms")
            
        except Exception as e:
            duration_ms = (time.time() - start_time) * 1000
            results[executor_type] = {
                "duration_ms": duration_ms,
                "success": False,
                "error": str(e)
            }
            
            print(f"  ❌ {executor_type}: Failed after {duration_ms:.1f}ms - {e}")
    
    return results
```

## Resource Management

### Memory Optimization

```python
def create_memory_optimized_workflow():
    """Create workflow optimized for memory usage."""
    
    workflow = graphbit.Workflow("Memory Optimized Workflow")
    
    # Use concise prompts to reduce memory usage
    data_processor = graphbit.Node.agent(
        name="Memory Efficient Processor",
        prompt="Analyze: {input}",  # Concise prompt
        agent_id="mem_processor"
    )
    
    # Data compressor using transform node
    compressor = graphbit.Node.transform(
        name="Data Compressor",
        transformation="uppercase"  # Simple transformation to reduce data size
    )
    
    # Build memory-efficient workflow
    proc_id = workflow.add_node(data_processor)
    comp_id = workflow.add_node(compressor)
    
    workflow.connect(proc_id, comp_id)
    
    return workflow

def monitor_memory_usage():
    """Monitor workflow memory usage."""
    
    try:
        import psutil
        import os
        
        def get_memory_usage():
            """Get current memory usage."""
            process = psutil.Process(os.getpid())
            memory_info = process.memory_info()
            return {
                "rss_mb": memory_info.rss / 1024 / 1024,  # MB
                "vms_mb": memory_info.vms / 1024 / 1024,  # MB
                "percent": process.memory_percent()
            }
        
        # Baseline memory
        baseline = get_memory_usage()
        print(f"Baseline memory: {baseline['rss_mb']:.2f} MB")
        
        return baseline
        
    except ImportError:
        print("psutil not available for memory monitoring")
        return None

def execute_with_memory_monitoring(workflow, executor):
    """Execute workflow with memory monitoring."""
    
    baseline_memory = monitor_memory_usage()
    
    if baseline_memory:
        print(f"Starting execution with {baseline_memory['rss_mb']:.2f} MB")
    
    # Execute workflow
    start_time = time.time()
    result = executor.execute(workflow)
    duration_ms = (time.time() - start_time) * 1000
    
    # Check memory after execution
    final_memory = monitor_memory_usage()
    
    if baseline_memory and final_memory:
        memory_increase = final_memory['rss_mb'] - baseline_memory['rss_mb']
        print(f"Memory increase: {memory_increase:.2f} MB")
        print(f"Execution completed in {duration_ms:.1f}ms")
    
    return result
```

## Configuration Tuning

### Environment-Specific Optimization

```python
def get_optimized_config(environment="production", workload_type="balanced"):
    """Get optimized configuration for specific environment and workload."""
    
    configs = {
        "development": {
            "model": "gpt-4o-mini",
            "timeout": 30
        },
        "production": {
            "model": "gpt-4o-mini",
            "timeout": 120
        }
    }
    
    workload_adjustments = {
        "high_throughput": {
            "model": "gpt-4o-mini",  # Faster model
            "timeout": 15
        },
        "high_quality": {
            "model": "gpt-4o",  # Best quality model
            "timeout": 180
        },
        "low_latency": {
            "model": "gpt-4o-mini",
            "timeout": 10
        }
    }
    
    base_config = configs.get(environment, configs["production"])
    workload_config = workload_adjustments.get(workload_type, {})
    
    # Merge configurations
    final_config = {**base_config, **workload_config}
    
    return final_config

def create_optimized_executor(environment="production", workload_type="balanced"):
    """Create executor with optimized configuration."""
    
    config_params = get_optimized_config(environment, workload_type)
    
    # LLM configuration
    llm_config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model=config_params["model"]
    )
    
    # Create executor based on workload type
    if workload_type == "high_throughput":
        executor = graphbit.Executor.new_high_throughput(
            llm_config=llm_config,
            timeout_seconds=config_params["timeout"],
            debug=False
        )
    elif workload_type == "low_latency":
        executor = graphbit.Executor.new_low_latency(
            llm_config=llm_config,
            timeout_seconds=config_params["timeout"],
            debug=False
        )
    elif workload_type == "memory_optimized":
        executor = graphbit.Executor.new_memory_optimized(
            llm_config=llm_config,
            timeout_seconds=config_params["timeout"],
            debug=False
        )
    else:
        executor = graphbit.Executor(
            config=llm_config,
            timeout_seconds=config_params["timeout"],
            debug=False
        )
    
    return executor
```

## LLM Client Optimization

### Client Configuration and Performance

```python
def create_optimized_llm_client():
    """Create optimized LLM client for best performance."""
    
    # OpenAI configuration with performance settings
    openai_config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"  # Fast and cost-effective
    )
    
    # Create client
    client = graphbit.LlmClient(openai_config)
    
    # Warm up the client
    try:
        client.warmup()
        print("✅ Client warmed up successfully")
    except Exception as e:
        print(f"⚠️ Client warmup failed: {e}")
    
    return client

def benchmark_llm_providers():
    """Benchmark different LLM providers for performance."""
    
    providers = {
        "openai_gpt4o_mini": graphbit.LlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o-mini"
        ),
        "anthropic_claude": graphbit.LlmConfig.anthropic(
            api_key=os.getenv("ANTHROPIC_API_KEY"),
            model="claude-3-5-sonnet-20241022"
        ),
        "ollama_llama": graphbit.LlmConfig.ollama(
            model="llama3.2"
        )
    }
    
    test_prompt = "Summarize this text in one sentence: {input}"
    test_input = "The quick brown fox jumps over the lazy dog."
    
    results = {}
    
    for provider_name, config in providers.items():
        print(f"Testing {provider_name}...")
        
        try:
            client = graphbit.LlmClient(config)
            
            # Warm up
            client.warmup()
            
            # Time the completion
            start_time = time.time()
            
            response = client.complete(test_prompt, {"input": test_input})
            
            duration_ms = (time.time() - start_time) * 1000
            
            # Get client statistics
            stats = client.get_stats()
            
            results[provider_name] = {
                "duration_ms": duration_ms,
                "success": True,
                "response_length": len(response),
                "stats": stats
            }
            
            print(f"  ✅ {provider_name}: {duration_ms:.1f}ms")
            
        except Exception as e:
            results[provider_name] = {
                "success": False,
                "error": str(e)
            }
            print(f"  ❌ {provider_name}: Failed - {e}")
    
    return results

def optimize_client_batch_processing():
    """Demonstrate optimized batch processing."""
    
    config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    client = graphbit.LlmClient(config)
    client.warmup()
    
    # Test data
    test_prompts = [
        "Summarize: {text}",
        "Extract key points: {text}",
        "Analyze sentiment: {text}",
        "Generate title: {text}"
    ]
    
    test_texts = [
        "The weather is beautiful today.",
        "I'm frustrated with the slow service.",
        "This product exceeds all expectations.",
        "The meeting was productive and insightful."
    ]
    
    # Batch processing
    start_time = time.time()
    
    batch_inputs = []
    for prompt in test_prompts:
        for text in test_texts:
            batch_inputs.append({"prompt": prompt, "text": text})
    
    # Process batch
    batch_results = client.batch_complete(
        [(item["prompt"], {"text": item["text"]}) for item in batch_inputs]
    )
    
    batch_duration_ms = (time.time() - start_time) * 1000
    
    print(f"Batch processing ({len(batch_inputs)} items): {batch_duration_ms:.1f}ms")
    print(f"Average per item: {batch_duration_ms / len(batch_inputs):.1f}ms")
    
    return batch_results
```

## Workflow Design Patterns for Performance

### Efficient Workflow Patterns

```python
def create_high_performance_patterns():
    """Create various high-performance workflow patterns."""
    
    patterns = {}
    
    # 1. Fan-out/Fan-in Pattern
    patterns["fan_out_in"] = create_fan_out_fan_in_workflow()
    
    # 2. Pipeline Pattern
    patterns["pipeline"] = create_pipeline_workflow()
    
    # 3. Conditional Processing Pattern
    patterns["conditional"] = create_conditional_workflow()
    
    return patterns

def create_fan_out_fan_in_workflow():
    """Create efficient fan-out/fan-in workflow."""
    
    workflow = graphbit.Workflow("Fan-Out Fan-In Workflow")
    
    # Single input processor
    input_node = graphbit.Node.agent(
        name="Input Splitter",
        prompt="Split this input for parallel processing: {input}",
        agent_id="splitter"
    )
    
    # Multiple parallel processors
    processors = []
    for i in range(3):
        processor = graphbit.Node.agent(
            name=f"Processor {i+1}",
            prompt=f"Process part {i+1}: {{split_input}}",
            agent_id=f"processor_{i+1}"
        )
        processors.append(processor)
    
    # Single aggregator
    aggregator = graphbit.Node.agent(
        name="Result Aggregator",
        prompt="Combine results: {processor_1_output}, {processor_2_output}, {processor_3_output}",
        agent_id="aggregator"
    )
    
    # Build workflow
    input_id = workflow.add_node(input_node)
    processor_ids = [workflow.add_node(p) for p in processors]
    agg_id = workflow.add_node(aggregator)
    
    # Connect: input -> all processors -> aggregator
    for proc_id in processor_ids:
        workflow.connect(input_id, proc_id)
        workflow.connect(proc_id, agg_id)
    
    return workflow

def create_pipeline_workflow():
    """Create efficient pipeline workflow."""
    
    workflow = graphbit.Workflow("Pipeline Workflow")
    
    # Sequential processing stages
    stages = [
        ("Data Validator", "Validate input data: {input}"),
        ("Data Processor", "Process validated data: {validated_data}"),
        ("Data Formatter", "Format processed data: {processed_data}"),
        ("Quality Checker", "Check quality: {formatted_data}")
    ]
    
    nodes = []
    for i, (name, prompt) in enumerate(stages):
        node = graphbit.Node.agent(
            name=name,
            prompt=prompt,
            agent_id=f"stage_{i+1}"
        )
        nodes.append(node)
    
    # Build pipeline
    node_ids = [workflow.add_node(node) for node in nodes]
    
    # Connect sequentially
    for i in range(len(node_ids) - 1):
        workflow.connect(node_ids[i], node_ids[i + 1])
    
    return workflow

def create_conditional_workflow():
    """Create efficient conditional workflow."""
    
    workflow = graphbit.Workflow("Conditional Workflow")
    
    # Input analyzer
    analyzer = graphbit.Node.agent(
        name="Input Analyzer",
        prompt="Analyze input type and complexity: {input}",
        agent_id="analyzer"
    )
    
    # Condition node
    condition = graphbit.Node.condition(
        name="Complexity Check",
        expression="complexity == 'simple'"
    )
    
    # Simple processor
    simple_processor = graphbit.Node.agent(
        name="Simple Processor",
        prompt="Quick processing: {input}",
        agent_id="simple_proc"
    )
    
    # Complex processor
    complex_processor = graphbit.Node.agent(
        name="Complex Processor", 
        prompt="Detailed processing: {input}",
        agent_id="complex_proc"
    )
    
    # Build conditional workflow
    analyzer_id = workflow.add_node(analyzer)
    condition_id = workflow.add_node(condition)
    simple_id = workflow.add_node(simple_processor)
    complex_id = workflow.add_node(complex_processor)
    
    # Connect: analyzer -> condition -> appropriate processor
    workflow.connect(analyzer_id, condition_id)
    workflow.connect(condition_id, simple_id)   # true branch
    workflow.connect(condition_id, complex_id)  # false branch
    
    return workflow
```

## Performance Monitoring and Profiling

### Real-time Performance Tracking

```python
def create_performance_profiler():
    """Create performance profiler for workflows."""
    
    class WorkflowProfiler:
        def __init__(self):
            self.execution_times = []
            self.node_times = {}
            self.memory_usage = []
            self.throughput_data = []
        
        def profile_execution(self, workflow, executor, iterations=1):
            """Profile workflow execution."""
            
            results = {
                "total_iterations": iterations,
                "execution_times": [],
                "average_time": 0,
                "throughput_per_second": 0,
                "memory_peak": 0
            }
            
            print(f"Profiling workflow with {iterations} iterations...")
            
            start_overall = time.time()
            
            for i in range(iterations):
                # Monitor memory before execution
                baseline_memory = monitor_memory_usage()
                
                # Time the execution
                start_time = time.time()
                result = executor.execute(workflow)
                execution_time = (time.time() - start_time) * 1000
                
                results["execution_times"].append(execution_time)
                
                # Monitor memory after execution
                final_memory = monitor_memory_usage()
                
                if baseline_memory and final_memory:
                    memory_used = final_memory['rss_mb'] - baseline_memory['rss_mb']
                    results["memory_peak"] = max(results["memory_peak"], memory_used)
                
                print(f"  Iteration {i+1}: {execution_time:.1f}ms")
            
            # Calculate statistics
            total_time_seconds = time.time() - start_overall
            results["average_time"] = sum(results["execution_times"]) / len(results["execution_times"])
            results["throughput_per_second"] = iterations / total_time_seconds
            
            return results
        
        def compare_configurations(self, workflow, configs):
            """Compare different executor configurations."""
            
            comparison_results = {}
            
            for config_name, executor in configs.items():
                print(f"\nTesting configuration: {config_name}")
                
                profile_result = self.profile_execution(workflow, executor, iterations=3)
                comparison_results[config_name] = profile_result
            
            # Print comparison summary
            print("\n" + "="*50)
            print("Configuration Comparison Summary")
            print("="*50)
            
            for config_name, result in comparison_results.items():
                print(f"{config_name}:")
                print(f"  Average Time: {result['average_time']:.1f}ms")
                print(f"  Throughput: {result['throughput_per_second']:.2f}/sec")
                print(f"  Peak Memory: {result['memory_peak']:.2f}MB")
            
            return comparison_results
    
    return WorkflowProfiler()

def run_comprehensive_performance_test():
    """Run comprehensive performance test."""
    
    # Create test workflow
    workflow = create_parallel_workflow()
    
    # Create different executor configurations
    executors = create_optimized_executors()
    
    # Create profiler
    profiler = create_performance_profiler()
    
    # Run comparison
    results = profiler.compare_configurations(workflow, executors)
    
    # Find best performer
    best_config = min(results.keys(), key=lambda k: results[k]["average_time"])
    
    print(f"\n🏆 Best performing configuration: {best_config}")
    print(f"   Average time: {results[best_config]['average_time']:.1f}ms")
    print(f"   Throughput: {results[best_config]['throughput_per_second']:.2f}/sec")
    
    return results
```

## Caching and Optimization Strategies

### Response Caching

```python
def implement_response_caching():
    """Implement response caching for repeated queries."""
    
    import hashlib
    import json
    from typing import Dict, Any
    
    class CachedExecutor:
        def __init__(self, base_executor, cache_size=1000):
            self.base_executor = base_executor
            self.cache: Dict[str, Any] = {}
            self.cache_size = cache_size
            self.cache_hits = 0
            self.cache_misses = 0
        
        def _generate_cache_key(self, workflow, input_data=None):
            """Generate cache key for workflow and input."""
            
            # Create a hash of workflow structure and input
            workflow_str = str(workflow)
            input_str = json.dumps(input_data or {}, sort_keys=True)
            combined = workflow_str + input_str
            
            return hashlib.md5(combined.encode()).hexdigest()
        
        def execute(self, workflow, input_data=None):
            """Execute workflow with caching."""
            
            cache_key = self._generate_cache_key(workflow, input_data)
            
            # Check cache first
            if cache_key in self.cache:
                self.cache_hits += 1
                print(f"💾 Cache hit for key: {cache_key[:8]}...")
                return self.cache[cache_key]
            
            # Execute workflow
            self.cache_misses += 1
            print(f"🔄 Cache miss, executing workflow...")
            
            result = self.base_executor.execute(workflow)
            
            # Store in cache
            if len(self.cache) >= self.cache_size:
                # Remove oldest entry (simple FIFO)
                oldest_key = next(iter(self.cache))
                del self.cache[oldest_key]
            
            self.cache[cache_key] = result
            
            return result
        
        def get_cache_stats(self):
            """Get cache performance statistics."""
            
            total_requests = self.cache_hits + self.cache_misses
            hit_rate = (self.cache_hits / total_requests) * 100 if total_requests > 0 else 0
            
            return {
                "cache_hits": self.cache_hits,
                "cache_misses": self.cache_misses,
                "hit_rate_percent": hit_rate,
                "cache_size": len(self.cache)
            }
    
    return CachedExecutor

def test_caching_performance():
    """Test caching performance improvement."""
    
    # Create base executor
    base_executor = create_performance_optimized_executor()
    
    # Create cached executor
    CachedExecutorClass = implement_response_caching()
    cached_executor = CachedExecutorClass(base_executor)
    
    # Create simple test workflow
    workflow = graphbit.Workflow("Cache Test Workflow")
    
    test_agent = graphbit.Node.agent(
        name="Test Agent",
        prompt="Process this input: {input}",
        agent_id="test_agent"
    )
    
    workflow.add_node(test_agent)
    
    # Test with repeated executions
    test_inputs = ["test input 1", "test input 2", "test input 1"]  # Repeat first input
    
    print("Testing caching performance...")
    
    for i, test_input in enumerate(test_inputs):
        print(f"\nExecution {i+1} with input: '{test_input}'")
        
        start_time = time.time()
        result = cached_executor.execute(workflow)
        duration_ms = (time.time() - start_time) * 1000
        
        print(f"Execution time: {duration_ms:.1f}ms")
    
    # Print cache statistics
    stats = cached_executor.get_cache_stats()
    print(f"\nCache Statistics:")
    print(f"  Hits: {stats['cache_hits']}")
    print(f"  Misses: {stats['cache_misses']}")
    print(f"  Hit Rate: {stats['hit_rate_percent']:.1f}%")
    print(f"  Cache Size: {stats['cache_size']}")
    
    return stats
```

## Best Practices

### 1. Choose the Right Executor Type

```python
def select_optimal_executor(workload_characteristics):
    """Select optimal executor based on workload characteristics."""
    
    recommendations = {
        "high_volume_simple": "high_throughput",
        "real_time_interactive": "low_latency", 
        "resource_constrained": "memory_optimized",
        "balanced_workload": "standard"
    }
    
    workload_type = workload_characteristics.get("type", "balanced_workload")
    recommended_executor = recommendations.get(workload_type, "standard")
    
    print(f"Recommended executor for '{workload_type}': {recommended_executor}")
    
    return recommended_executor
```

### 2. Model Selection for Performance

```python
def select_optimal_model(use_case):
    """Select optimal model based on use case."""
    
    model_recommendations = {
        "simple_tasks": "gpt-4o-mini",
        "complex_analysis": "gpt-4o", 
        "cost_sensitive": "gpt-4o-mini",
        "highest_quality": "gpt-4o"
    }
    
    recommended_model = model_recommendations.get(use_case, "gpt-4o-mini")
    
    print(f"Recommended model for '{use_case}': {recommended_model}")
    
    return recommended_model
```

### 3. Workflow Design for Performance

```python
def optimize_workflow_design():
    """Guidelines for optimizing workflow design."""
    
    guidelines = {
        "minimize_sequential_dependencies": "Use parallel processing where possible",
        "optimize_prompt_length": "Keep prompts concise and focused",
        "batch_similar_operations": "Group similar operations together",
        "use_conditional_logic": "Skip unnecessary processing with conditions",
        "cache_common_operations": "Cache frequently used results"
    }
    
    for guideline, description in guidelines.items():
        print(f"✅ {guideline}: {description}")
    
    return guidelines
```

## Performance Testing Framework

### Automated Performance Testing

```python
def create_performance_test_suite():
    """Create comprehensive performance test suite."""
    
    class PerformanceTestSuite:
        def __init__(self):
            self.test_results = {}
        
        def run_latency_test(self, workflow, executor, iterations=10):
            """Test execution latency."""
            
            execution_times = []
            
            for i in range(iterations):
                start_time = time.time()
                result = executor.execute(workflow)
                duration_ms = (time.time() - start_time) * 1000
                execution_times.append(duration_ms)
            
            return {
                "average_latency_ms": sum(execution_times) / len(execution_times),
                "min_latency_ms": min(execution_times),
                "max_latency_ms": max(execution_times),
                "p95_latency_ms": sorted(execution_times)[int(len(execution_times) * 0.95)]
            }
        
        def run_throughput_test(self, workflow, executor, duration_seconds=60):
            """Test execution throughput."""
            
            start_time = time.time()
            executions = 0
            
            while (time.time() - start_time) < duration_seconds:
                try:
                    result = executor.execute(workflow)
                    executions += 1
                except Exception as e:
                    print(f"Execution failed: {e}")
            
            actual_duration = time.time() - start_time
            throughput = executions / actual_duration
            
            return {
                "executions_completed": executions,
                "test_duration_seconds": actual_duration,
                "throughput_per_second": throughput
            }
        
        def run_stress_test(self, workflow, executor, max_concurrent=10):
            """Test system under stress."""
            
            import threading
            import queue
            
            results_queue = queue.Queue()
            
            def execute_workflow():
                try:
                    start_time = time.time()
                    result = executor.execute(workflow)
                    duration_ms = (time.time() - start_time) * 1000
                    results_queue.put({"success": True, "duration_ms": duration_ms})
                except Exception as e:
                    results_queue.put({"success": False, "error": str(e)})
            
            # Launch concurrent executions
            threads = []
            for i in range(max_concurrent):
                thread = threading.Thread(target=execute_workflow)
                threads.append(thread)
                thread.start()
            
            # Wait for all threads to complete
            for thread in threads:
                thread.join()
            
            # Collect results
            results = []
            while not results_queue.empty():
                results.append(results_queue.get())
            
            successful_executions = [r for r in results if r["success"]]
            failed_executions = [r for r in results if not r["success"]]
            
            return {
                "total_executions": len(results),
                "successful_executions": len(successful_executions),
                "failed_executions": len(failed_executions),
                "success_rate_percent": (len(successful_executions) / len(results)) * 100,
                "average_duration_ms": sum(r["duration_ms"] for r in successful_executions) / len(successful_executions) if successful_executions else 0
            }
        
        def run_complete_test_suite(self, workflow, executor):
            """Run complete performance test suite."""
            
            print("🧪 Running Performance Test Suite")
            print("="*40)
            
            # Latency test
            print("1. Latency Test...")
            latency_results = self.run_latency_test(workflow, executor)
            print(f"   Average latency: {latency_results['average_latency_ms']:.1f}ms")
            
            # Throughput test
            print("2. Throughput Test...")
            throughput_results = self.run_throughput_test(workflow, executor, duration_seconds=30)
            print(f"   Throughput: {throughput_results['throughput_per_second']:.2f}/sec")
            
            # Stress test
            print("3. Stress Test...")
            stress_results = self.run_stress_test(workflow, executor, max_concurrent=5)
            print(f"   Success rate: {stress_results['success_rate_percent']:.1f}%")
            
            self.test_results = {
                "latency": latency_results,
                "throughput": throughput_results,
                "stress": stress_results
            }
            
            return self.test_results
    
    return PerformanceTestSuite()

# Usage example
if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    
    # Create test workflow and executor
    test_workflow = create_memory_optimized_workflow()
    test_executor = create_performance_optimized_executor()
    
    # Run performance tests
    test_suite = create_performance_test_suite()
    results = test_suite.run_complete_test_suite(test_workflow, test_executor)
    
    print("\n📊 Performance Test Results Summary:")
    print(f"Average Latency: {results['latency']['average_latency_ms']:.1f}ms")
    print(f"Throughput: {results['throughput']['throughput_per_second']:.2f}/sec")
    print(f"Stress Test Success Rate: {results['stress']['success_rate_percent']:.1f}%")
```

## What's Next

- Learn about [Monitoring](monitoring.md) for tracking performance in production
- Explore [Reliability](reliability.md) for robust production systems
- Check [Validation](validation.md) for ensuring performance requirements
- See [LLM Providers](llm-providers.md) for provider-specific optimizations 