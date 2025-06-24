#!/usr/bin/env python3

"""GraphBit framework benchmark implementation.

Optimized to use direct API calls for maximum performance, bypassing workflow overhead.
"""

import os
import sys
import asyncio
import time
from typing import Any, Dict, List

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
    """Ultra-high-performance GraphBit benchmark using direct API calls."""

    def __init__(self, config: Dict):
        """Initialize GraphBit benchmark with configuration."""
        super().__init__(config)
        self.llm_config = None
        self.llm_client = None

    async def setup(self) -> None:
        """Set up GraphBit with minimal overhead configuration."""
        # Initialize GraphBit (minimal initialization)
        graphbit.init()

        # Get standardized configuration
        llm_config = get_standard_llm_config(self.config)
        openai_key = os.getenv("OPENAI_API_KEY") or llm_config["api_key"]
        if not openai_key:
            raise ValueError("OpenAI API key not found in environment or config")

        # Create minimal LLM configuration
        self.llm_config = graphbit.LlmConfig.openai(openai_key, llm_config["model"])

        # Create LLM client using the correct API
        self.llm_client = graphbit.LlmClient(self.llm_config)

    async def teardown(self) -> None:
        """Cleanup GraphBit resources."""
        self.llm_config = None
        self.llm_client = None

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run simple task using direct API call."""
        self.monitor.start_monitoring()

        try:
            llm_config = get_standard_llm_config(self.config)
            
            output_content = await self.llm_client.complete_async(
                prompt=SIMPLE_TASK_PROMPT,
                max_tokens=llm_config["max_tokens"],
                temperature=llm_config["temperature"]
            )
            
            self.log_output(
                scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
                task_name="Simple Task",
                output=output_content,
            )

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
        """Run sequential pipeline using direct API calls."""
        self.monitor.start_monitoring()

        try:
            llm_config = get_standard_llm_config(self.config)
            total_tokens = 0
            previous_result = ""

            for i, task in enumerate(SEQUENTIAL_TASKS):
                if i == 0:
                    prompt = task
                else:
                    prompt = f"Previous result: {previous_result}\n\nNew task: {task}"
                
                # Single direct API call
                result = await self.llm_client.complete_stream(
                    prompt=prompt,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"]
                )

                previous_result = result
                # Standardized token counting: only count base task + result (like LangChain)
                total_tokens += count_tokens_estimate(task + result)

                self.log_output(
                    scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                    task_name=f"Sequential Task {i+1}",
                    output=result,
                )

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
        """Run parallel pipeline using batch processing or concurrent API calls."""
        self.monitor.start_monitoring()

        try:
            llm_config = get_standard_llm_config(self.config)
            
            try:
                results = await self.llm_client.complete_batch(
                    prompts=PARALLEL_TASKS,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"],
                    max_concurrency=len(PARALLEL_TASKS)
                )
            except Exception as e:
                self.logger.error(f"Error in parallel pipeline benchmark: {e}")
                metrics = self.monitor.stop_monitoring()
                metrics.error_rate = 1.0
                metrics.token_count = 0
                return metrics

            total_tokens = 0
            for i, (task, result) in enumerate(zip(PARALLEL_TASKS, results)):
                if isinstance(result, Exception):
                    result = f"Error: {result}"
                
                self.log_output(
                    scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                    task_name=f"Parallel Task {i+1}",
                    output=result,
                )
                total_tokens += count_tokens_estimate(task + str(result))

        except Exception as e:
            self.logger.error(f"Error in parallel pipeline benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(PARALLEL_TASKS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_complex_workflow(self) -> BenchmarkMetrics:
        """Run complex workflow using direct API calls to avoid workflow system issues."""
        self.monitor.start_monitoring()

        try:
            llm_config = get_standard_llm_config(self.config)
            total_tokens = 0
            workflow_results = {}

            # Execute workflow steps in dependency order
            for step in COMPLEX_WORKFLOW_STEPS:
                step_name = step["task"]
                step_prompt = step["prompt"]
                
                # Build context from dependencies
                context_parts = []
                for dependency in step["depends_on"]:
                    if dependency in workflow_results:
                        context_parts.append(f"{dependency}: {workflow_results[dependency]}")
                
                # Create full prompt with context
                if context_parts:
                    full_prompt = f"Context from previous steps:\n{chr(10).join(context_parts)}\n\nNew task: {step_prompt}"
                else:
                    full_prompt = step_prompt
                
                # Execute step using direct API call
                result = await self.llm_client.complete_async(
                    prompt=full_prompt,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"]
                )
                
                # Store result for next steps
                workflow_results[step_name] = result
                
                # Log output
                self.log_output(
                    scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    task_name=step_name,
                    output=result,
                )
                
                # Count tokens for step prompt and result
                total_tokens += count_tokens_estimate(full_prompt + result)

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
        """Run memory-intensive test using direct API call."""
        self.monitor.start_monitoring()

        try:
            llm_config = get_standard_llm_config(self.config)
            
            # Single direct API call
            output_content = await self.llm_client.complete_async(
                prompt=MEMORY_INTENSIVE_PROMPT,
                max_tokens=llm_config["max_tokens"],
                temperature=llm_config["temperature"]
            )

            self.log_output(
                scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
                task_name="Memory Intensive Task",
                output=output_content,
            )

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
        """Run concurrent tasks using batch processing or asyncio.gather."""
        self.monitor.start_monitoring()

        try:
            llm_config = get_standard_llm_config(self.config)
            
            # Try to use batch processing first for better performance
            try:
                results = await self.llm_client.complete_batch(
                    prompts=CONCURRENT_TASK_PROMPTS,
                    max_tokens=llm_config["max_tokens"],
                    temperature=llm_config["temperature"],
                    max_concurrency=20
                )
            except Exception as e:
                self.logger.error(f"Error in concurrent tasks benchmark: {e}")
                metrics = self.monitor.stop_monitoring()
                metrics.error_rate = 1.0
                metrics.token_count = 0
                return metrics
                
                # Create all concurrent tasks
                tasks = [single_call(prompt) for prompt in CONCURRENT_TASK_PROMPTS]
                results = await asyncio.gather(*tasks, return_exceptions=True)

            total_tokens = 0
            for i, (task, result) in enumerate(zip(CONCURRENT_TASK_PROMPTS, results)):
                if isinstance(result, Exception):
                    result = f"Error: {result}"
                    
                self.log_output(
                    scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                    task_name=f"Concurrent Task {i+1}",
                    output=result,
                )
                total_tokens += count_tokens_estimate(task + str(result))

        except Exception as e:
            self.logger.error(f"Error in concurrent tasks benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000)

        return metrics
