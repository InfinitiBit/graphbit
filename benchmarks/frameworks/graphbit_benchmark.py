#!/usr/bin/env python3

"""GraphBit framework benchmark implementation.

Updated to use current simplified Python bindings with direct LLM calls for most tasks.
"""

import os
import sys
import uuid
import asyncio
from typing import Any, Dict

try:
    import graphbit
except ImportError:
    print("GraphBit Python bindings not installed. " "Run 'maturin develop' in graphbit/")
    sys.exit(1)

from .common import (
    COMPLEX_WORKFLOW_STEPS,
    CONCURRENT_TASK_PROMPTS,
    MEMORY_INTENSIVE_PROMPT,
    PARALLEL_TASKS,
    SEQUENTIAL_TASKS,
    SIMPLE_TASK_PROMPT,
    BaseBenchmark,
    BenchmarkMetrics,
    BenchmarkScenario,
    calculate_throughput,
    count_tokens_estimate,
    get_standard_llm_config,
)


class GraphBitBenchmark(BaseBenchmark):
    """GraphBit framework benchmark implementation using simplified bindings with direct LLM calls."""

    def __init__(self, config: Dict[str, Any]):
        """Initialize GraphBit benchmark with configuration."""
        super().__init__(config)
        self.llm_config: Any = None
        self.llm_client: Any = None
        self.executor: Any = None  # Only for actual workflow tests

        if graphbit is None:
            raise ImportError("GraphBit Python bindings not installed. " "Please run 'maturin develop' in the graphbit/ directory.")

    async def setup(self) -> None:
        """Set up GraphBit with simplified configuration and direct LLM client."""
        # Initialize GraphBit
        graphbit.init()

        # Get standardized configuration
        llm_config = get_standard_llm_config(self.config)
        openai_key = os.getenv("OPENAI_API_KEY") or llm_config["api_key"]
        if not openai_key:
            raise ValueError("OpenAI API key not found in environment or config")

        # Create LLM configuration using the simplified API
        self.llm_config = graphbit.LlmConfig.openai(openai_key, llm_config["model"])

        # Create LLM client for direct calls (primary interface)
        self.llm_client = graphbit.LlmClient(self.llm_config)

        # Create executor only for actual workflow tests
        self.executor = graphbit.Executor(self.llm_config)
        self.executor.timeout(30)  # 30 seconds timeout
        self.executor.retries(3)   # 3 retries

    async def teardown(self) -> None:
        """Cleanup GraphBit resources."""
        self.llm_config = None
        self.llm_client = None
        self.executor = None

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run a simple single-task benchmark using direct LLM call."""
        self.monitor.start_monitoring()

        try:
            # Get standard config for consistent parameters
            llm_config = get_standard_llm_config(self.config)
            
            # Use direct LLM call instead of workflow for simple tasks
            output_content = self.llm_client.complete(
                prompt=SIMPLE_TASK_PROMPT,
                max_tokens=llm_config["max_tokens"],
                temperature=llm_config["temperature"]
            )
            
            # Log LLM output to file
            self.log_output(
                scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
                task_name="Simple Task",
                output=output_content,
            )

            # Count tokens
            token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + output_content)

        except Exception as e:
            self.logger.error(f"Error in simple task benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)

        return metrics

    async def run_sequential_pipeline(self) -> BenchmarkMetrics:
        """Run a sequential pipeline benchmark using direct LLM calls."""
        self.monitor.start_monitoring()

        try:
            # Get standard config for consistent parameters
            llm_config = get_standard_llm_config(self.config)
            total_tokens = 0
            
            # Process each task sequentially using direct LLM calls
            for i, task in enumerate(SEQUENTIAL_TASKS):
                task_name = f"Task {i+1}"
                
                # Direct LLM call for each task
                output_content = self.llm_client.complete(
                    prompt=task,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"]
                )

                # Log output
                self.log_output(
                    scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                    task_name=task_name,
                    output=output_content,
                )

                # Count tokens
                input_tokens = count_tokens_estimate(task)
                output_tokens = count_tokens_estimate(output_content)
                total_tokens += input_tokens + output_tokens

        except Exception as e:
            self.logger.error(f"Error in sequential pipeline benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(SEQUENTIAL_TASKS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Run a parallel pipeline benchmark using concurrent direct LLM calls."""
        self.monitor.start_monitoring()

        try:
            # Get standard config for consistent parameters
            llm_config = get_standard_llm_config(self.config)
            
            # Create async tasks for parallel execution
            async def process_task(i: int, task: str) -> tuple[int, str, str]:
                task_name = f"Parallel Task {i+1}"
                # Use async LLM call for true parallelism
                output_content = await self.llm_client.complete_async(
                    prompt=task,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"]
                )
                return i, task_name, output_content

            # Execute all tasks concurrently
            tasks = [process_task(i, task) for i, task in enumerate(PARALLEL_TASKS)]
            results = await asyncio.gather(*tasks, return_exceptions=True)

            # Process results
            total_tokens = 0
            successful_count = 0
            
            for i, result in enumerate(results):
                if isinstance(result, Exception):
                    self.logger.error(f"Parallel task {i+1} failed: {result}")
                    continue
                
                task_idx, task_name, output_content = result
                successful_count += 1

                # Log output
                self.log_output(
                    scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                    task_name=task_name,
                    output=output_content,
                )

                # Count tokens
                input_tokens = count_tokens_estimate(PARALLEL_TASKS[task_idx])
                output_tokens = count_tokens_estimate(output_content)
                total_tokens += input_tokens + output_tokens

            # Calculate error rate
            error_rate = (len(PARALLEL_TASKS) - successful_count) / len(PARALLEL_TASKS) if PARALLEL_TASKS else 0.0

        except Exception as e:
            self.logger.error(f"Error in parallel pipeline benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.error_rate = error_rate
        metrics.throughput_tasks_per_sec = calculate_throughput(len(PARALLEL_TASKS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_complex_workflow(self) -> BenchmarkMetrics:
        """Run a complex workflow benchmark using sequential direct LLM calls (consistent with other frameworks)."""
        self.monitor.start_monitoring()

        try:
            # Get standard config for consistent parameters
            llm_config = get_standard_llm_config(self.config)
            total_tokens = 0
            results: Dict[str, str] = {}

            # Process each step sequentially, maintaining dependencies
            for step in COMPLEX_WORKFLOW_STEPS:
                step_name = step["task"]
                
                # Build context from previous steps
                context_parts = []
                for dep in step["depends_on"]:
                    if dep in results:
                        context_parts.append(f"{dep}: {results[dep]}")
                
                context = " | ".join(context_parts) if context_parts else "None"
                
                # Create full prompt with context
                full_prompt = f"Context from previous steps: {context}\n\nTask: {step['prompt']}"
                
                # Execute using direct LLM call
                output_content = self.llm_client.complete(
                    prompt=full_prompt,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"]
                )
                
                # Store result for future dependencies
                results[step_name] = output_content
                
                # Log output
                self.log_output(
                    scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    task_name=step_name,
                    output=output_content,
                )

                # Count tokens
                input_tokens = count_tokens_estimate(full_prompt)
                output_tokens = count_tokens_estimate(output_content)
                total_tokens += input_tokens + output_tokens

        except Exception as e:
            self.logger.error(f"Error in complex workflow benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(COMPLEX_WORKFLOW_STEPS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_memory_intensive(self) -> BenchmarkMetrics:
        """Run a memory-intensive benchmark using direct LLM call."""
        self.monitor.start_monitoring()

        try:
            # Get standard config for consistent parameters
            llm_config = get_standard_llm_config(self.config)
            
            # Use direct LLM call for memory-intensive task
            output_content = self.llm_client.complete(
                prompt=MEMORY_INTENSIVE_PROMPT,
                max_tokens=llm_config["max_tokens"],
                temperature=llm_config["temperature"]
            )
            
            # Log LLM output to file
            self.log_output(
                scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
                task_name="Memory Intensive Task",
                output=output_content,
            )

            # Count tokens
            token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + output_content)

        except Exception as e:
            self.logger.error(f"Error in memory intensive benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)

        return metrics

    async def run_concurrent_tasks(self) -> BenchmarkMetrics:
        """Run concurrent tasks benchmark using direct async LLM calls."""
        self.monitor.start_monitoring()

        try:
            # Get standard config for consistent parameters
            llm_config = get_standard_llm_config(self.config)
            
            # Create async tasks for concurrent execution
            async def process_concurrent_task(i: int, prompt: str) -> tuple[int, str, str]:
                task_name = f"Concurrent Task {i+1}"
                # Use async LLM call for true concurrency
                output_content = await self.llm_client.complete_async(
                    prompt=prompt,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"]
                )
                return i, task_name, output_content

            # Execute all tasks concurrently
            tasks = [process_concurrent_task(i, prompt) for i, prompt in enumerate(CONCURRENT_TASK_PROMPTS)]
            results = await asyncio.gather(*tasks, return_exceptions=True)

            # Process results
            total_tokens = 0
            successful_count = 0

            for i, result in enumerate(results):
                task_name = f"Concurrent Task {i+1}"

                if isinstance(result, Exception):
                    self.logger.error(f"Concurrent task {i+1} failed: {result}")
                    # Failed task - only count input tokens
                    total_tokens += count_tokens_estimate(CONCURRENT_TASK_PROMPTS[i])
                    self.log_output(
                        scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                        task_name=task_name,
                        output=f"{task_name} failed to execute: {str(result)}",
                    )
                    continue
                
                task_idx, task_name, output_content = result
                successful_count += 1
                
                # Log output
                self.log_output(
                    scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                    task_name=task_name,
                    output=output_content,
                )

                # Count tokens
                input_tokens = count_tokens_estimate(CONCURRENT_TASK_PROMPTS[task_idx])
                output_tokens = count_tokens_estimate(output_content)
                total_tokens += input_tokens + output_tokens

            # Calculate error rate
            error_rate = (len(CONCURRENT_TASK_PROMPTS) - successful_count) / len(CONCURRENT_TASK_PROMPTS) if CONCURRENT_TASK_PROMPTS else 1.0

        except Exception as e:
            self.logger.error(f"Error in concurrent tasks benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.error_rate = error_rate
        metrics.throughput_tasks_per_sec = calculate_throughput(len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_high_performance_concurrent_tasks(self) -> BenchmarkMetrics:
        """Run high-performance concurrent tasks using standard parameters for fair comparison."""
        self.monitor.start_monitoring()

        try:
            # Get standard config for consistent parameters (same as other benchmarks)
            llm_config = get_standard_llm_config(self.config)
            
            # Use the same prompts and parameters as regular concurrent tasks for fair comparison
            async def process_hp_task(i: int, prompt: str) -> tuple[int, str, str]:
                task_name = f"HP Task {i+1}"
                # Use same parameters as other benchmarks for fair comparison
                output_content = await self.llm_client.complete_async(
                    prompt=prompt,
                    max_tokens=llm_config["max_tokens"],  # Same as other benchmarks
                    temperature=llm_config["temperature"]  # Same as other benchmarks
                )
                return i, task_name, output_content

            # Execute all tasks concurrently
            tasks = [process_hp_task(i, prompt) for i, prompt in enumerate(CONCURRENT_TASK_PROMPTS)]
            results = await asyncio.gather(*tasks, return_exceptions=True)

            # Process results
            total_tokens = 0
            successful_count = 0

            for i, result in enumerate(results):
                task_name = f"HP Task {i+1}"

                if isinstance(result, Exception):
                    self.logger.error(f"HP task {i+1} failed: {result}")
                    # Failed task - only count input tokens
                    total_tokens += count_tokens_estimate(CONCURRENT_TASK_PROMPTS[i])
                    self.log_output(
                        scenario_name=BenchmarkScenario.HIGH_PERFORMANCE_CONCURRENT.value,
                        task_name=task_name,
                        output=f"{task_name} failed to execute: {str(result)}",
                    )
                    continue
                
                task_idx, task_name, output_content = result
                successful_count += 1
                
                # Log output
                self.log_output(
                    scenario_name=BenchmarkScenario.HIGH_PERFORMANCE_CONCURRENT.value,
                    task_name=task_name,
                    output=output_content,
                )

                # Count tokens
                input_tokens = count_tokens_estimate(CONCURRENT_TASK_PROMPTS[task_idx])
                output_tokens = count_tokens_estimate(output_content)
                total_tokens += input_tokens + output_tokens

            # Calculate error rate
            error_rate = (len(CONCURRENT_TASK_PROMPTS) - successful_count) / len(CONCURRENT_TASK_PROMPTS) if CONCURRENT_TASK_PROMPTS else 1.0

        except Exception as e:
            self.logger.error(f"Error in high performance concurrent tasks benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.error_rate = error_rate
        metrics.throughput_tasks_per_sec = calculate_throughput(len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000)

        return metrics
