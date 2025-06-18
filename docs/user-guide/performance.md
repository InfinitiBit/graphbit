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

def create_parallel_workflow():
    """Create workflow optimized for parallel execution."""
    
    builder = graphbit.PyWorkflowBuilder("Parallel Processing Workflow")
    
    # Input processor
    input_processor = graphbit.PyWorkflowNode.agent_node(
        name="Input Processor",
        description="Processes input for parallel branches",
        agent_id="input_processor",
        prompt="Prepare data for parallel processing: {input}"
    )
    
    # Parallel processing branches
    branch_nodes = []
    for i in range(4):  # Create 4 parallel branches
        branch = graphbit.PyWorkflowNode.agent_node(
            name=f"Parallel Branch {i+1}",
            description=f"Processes branch {i+1} data",
            agent_id=f"branch_{i+1}",
            prompt=f"Process branch {i+1} data: {{prepared_data}}"
        )
        branch_nodes.append(branch)
    
    # Results aggregator
    aggregator = graphbit.PyWorkflowNode.agent_node(
        name="Results Aggregator",
        description="Combines parallel processing results",
        agent_id="aggregator",
        prompt="""
        Combine results from parallel branches:
        Branch 1: {branch_1_output}
        Branch 2: {branch_2_output}
        Branch 3: {branch_3_output}
        Branch 4: {branch_4_output}
        """
    )
    
    # Build parallel structure
    input_id = builder.add_node(input_processor)
    branch_ids = [builder.add_node(branch) for branch in branch_nodes]
    agg_id = builder.add_node(aggregator)
    
    # Connect input to all branches (fan-out)
    for branch_id in branch_ids:
        builder.connect(input_id, branch_id, graphbit.PyWorkflowEdge.data_flow())
    
    # Connect all branches to aggregator (fan-in)
    for branch_id in branch_ids:
        builder.connect(branch_id, agg_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def create_performance_optimized_executor():
    """Create executor optimized for performance."""
    
    # Use fast, cost-effective model
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-3.5-turbo"
    )
    
    # Create high-performance executor
    executor = graphbit.PyWorkflowExecutor(config)
    
    return executor
```

## Resource Management

### Memory Optimization

```python
def create_memory_optimized_workflow():
    """Create workflow optimized for memory usage."""
    
    builder = graphbit.PyWorkflowBuilder("Memory Optimized Workflow")
    
    # Use concise prompts to reduce memory usage
    data_processor = graphbit.PyWorkflowNode.agent_node(
        name="Memory Efficient Processor",
        description="Processes data with minimal memory footprint",
        agent_id="mem_processor",
        prompt="Analyze: {input}"  # Concise prompt
    )
    
    # Data compressor
    compressor = graphbit.PyWorkflowNode.transform_node(
        name="Data Compressor",
        description="Compresses output data",
        transformation="json_extract"  # Extract only essential data
    )
    
    # Build memory-efficient workflow
    proc_id = builder.add_node(data_processor)
    comp_id = builder.add_node(compressor)
    
    builder.connect(proc_id, comp_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def monitor_memory_usage():
    """Monitor workflow memory usage."""
    
    import psutil
    import os
    
    def get_memory_usage():
        """Get current memory usage."""
        process = psutil.Process(os.getpid())
        memory_info = process.memory_info()
        return {
            "rss": memory_info.rss / 1024 / 1024,  # MB
            "vms": memory_info.vms / 1024 / 1024,  # MB
            "percent": process.memory_percent()
        }
    
    # Baseline memory
    baseline = get_memory_usage()
    print(f"Baseline memory: {baseline['rss']:.2f} MB")
    
    return baseline
```

## Configuration Tuning

### Environment-Specific Optimization

```python
def get_optimized_config(environment="production", workload_type="balanced"):
    """Get optimized configuration for specific environment and workload."""
    
    configs = {
        "development": {
            "model": "gpt-3.5-turbo",
            "timeout": 30000
        },
        "production": {
            "model": "gpt-4o-mini",
            "timeout": 120000
        }
    }
    
    workload_adjustments = {
        "high_throughput": {
            "model": "gpt-3.5-turbo",  # Faster model
            "timeout": 15000
        },
        "high_quality": {
            "model": "gpt-4o",  # Best quality model
            "timeout": 180000
        },
        "low_latency": {
            "model": "gpt-3.5-turbo",
            "timeout": 10000
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
    llm_config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model=config_params["model"]
    )
    
    # Create executor
    executor = graphbit.PyWorkflowExecutor(llm_config)
    
    return executor
```

## Performance Monitoring

### Execution Metrics

```python
import time
from dataclasses import dataclass
from typing import List

@dataclass
class ExecutionMetrics:
    """Metrics for workflow execution."""
    workflow_name: str
    execution_time: float
    node_count: int
    success: bool
    error_message: str = None
    memory_usage: float = None
    
class PerformanceMonitor:
    """Monitor workflow performance."""
    
    def __init__(self):
        self.metrics: List[ExecutionMetrics] = []
    
    def execute_with_monitoring(self, executor, workflow, input_data):
        """Execute workflow with performance monitoring."""
        
        start_time = time.time()
        success = False
        error_message = None
        
        try:
            result = executor.execute_with_input(workflow, input_data)
            success = True
        except Exception as e:
            error_message = str(e)
            result = None
        
        end_time = time.time()
        execution_time = end_time - start_time
        
        # Record metrics
        metrics = ExecutionMetrics(
            workflow_name=getattr(workflow, 'name', 'Unknown'),
            execution_time=execution_time,
            node_count=workflow.node_count(),
            success=success,
            error_message=error_message,
            memory_usage=0.0  # Simplified for now
        )
        
        self.metrics.append(metrics)
        
        return result, metrics
    
    def get_performance_summary(self):
        """Get performance summary statistics."""
        
        if not self.metrics:
            return "No metrics available"
        
        successful_executions = [m for m in self.metrics if m.success]
        failed_executions = [m for m in self.metrics if not m.success]
        
        if successful_executions:
            avg_time = sum(m.execution_time for m in successful_executions) / len(successful_executions)
            min_time = min(m.execution_time for m in successful_executions)
            max_time = max(m.execution_time for m in successful_executions)
        else:
            avg_time = min_time = max_time = 0
        
        summary = f"""
Performance Summary
==================
Total Executions: {len(self.metrics)}
Successful: {len(successful_executions)}
Failed: {len(failed_executions)}
Success Rate: {len(successful_executions) / len(self.metrics) * 100:.1f}%

Execution Times (successful only):
- Average: {avg_time:.2f}s
- Minimum: {min_time:.2f}s  
- Maximum: {max_time:.2f}s
        """
        
        return summary
```

## Best Practices

### 1. Choose the Right Execution Strategy

```python
def recommend_execution_strategy(workflow_characteristics):
    """Recommend optimal execution strategy based on workflow characteristics."""
    
    node_count = workflow_characteristics.get("node_count", 1)
    has_parallel_branches = workflow_characteristics.get("has_parallel_branches", False)
    avg_node_duration = workflow_characteristics.get("avg_node_duration", 1.0)
    data_size = workflow_characteristics.get("data_size", "small")
    
    recommendations = []
    
    if has_parallel_branches and node_count > 3:
        recommendations.append("Use parallel execution for independent branches")
    
    if avg_node_duration > 5.0:
        recommendations.append("Consider breaking down long-running nodes")
    
    if data_size == "large":
        recommendations.append("Use memory-optimized executor")
        recommendations.append("Consider data streaming or chunking")
    
    if node_count > 10:
        recommendations.append("Consider workflow decomposition")
    
    return recommendations
```

### 2. Optimize Model Selection

```python
def choose_optimal_model(task_complexity, latency_requirements, quality_requirements):
    """Choose optimal model based on requirements."""
    
    if latency_requirements == "low" and quality_requirements == "medium":
        return "gpt-3.5-turbo"
    elif quality_requirements == "high":
        return "gpt-4o"
    elif task_complexity == "simple":
        return "gpt-3.5-turbo"
    else:
        return "gpt-4o-mini"
```

### 3. Batch Processing

```python
def create_batch_processing_workflow(batch_size=10):
    """Create workflow optimized for batch processing."""
    
    builder = graphbit.PyWorkflowBuilder("Batch Processing Workflow")
    
    # Batch aggregator
    aggregator = graphbit.PyWorkflowNode.agent_node(
        name="Batch Aggregator",
        description=f"Processes {batch_size} items at once",
        agent_id="batch_aggregator",
        prompt=f"Process this batch of {batch_size} items: {{batch_input}}"
    )
    
    # Results splitter
    splitter = graphbit.PyWorkflowNode.transform_node(
        name="Results Splitter",
        description="Splits batch results into individual items",
        transformation="split"
    )
    
    # Build batch workflow
    agg_id = builder.add_node(aggregator)
    split_id = builder.add_node(splitter)
    
    builder.connect(agg_id, split_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

Performance optimization in GraphBit requires a systematic approach combining execution strategies, resource management, and continuous monitoring. Use these techniques to build high-performance workflows that scale with your needs. 
