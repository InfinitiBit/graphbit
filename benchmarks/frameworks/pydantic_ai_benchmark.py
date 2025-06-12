"""Pydantic AI framework benchmark implementation."""

import asyncio
import logging
import os
from typing import Any, Dict, List, Optional

try:
    from pydantic import BaseModel
    from pydantic_ai import Agent
    from pydantic_ai.models.openai import OpenAIModel

    _PYDANTIC_AI_AVAILABLE = True
except ImportError:
    Agent = None
    OpenAIModel = None
    BaseModel = None
    _PYDANTIC_AI_AVAILABLE = False

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

# Define response models for Pydantic AI
if _PYDANTIC_AI_AVAILABLE:

    class SimpleResponse(BaseModel):
        """Response model for simple tasks."""

        result: str

    class SequentialResponse(BaseModel):
        """Response model for sequential tasks."""

        step_result: str
        step_index: int

    class ComplexResponse(BaseModel):
        """Response model for complex workflow tasks."""

        task_name: str
        result: str
        depends_on: List[str]


class PydanticAIBenchmark(BaseBenchmark):
    """Pydantic AI framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any]):
        """Initialize PydanticAI benchmark with configuration."""
        super().__init__(config)
        self.openai_model: Optional[Any] = None
        self.agents: Dict[str, Any] = {}

        if Agent is None or OpenAIModel is None:
            raise ImportError("Pydantic AI not installed. Please install with: pip install pydantic-ai")

    async def setup(self) -> None:
        """Set up Pydantic AI for benchmarking."""
        # Get standardized configuration
        llm_config = get_standard_llm_config(self.config)
        api_key = os.getenv("OPENAI_API_KEY") or llm_config["api_key"]
        if not api_key:
            raise ValueError("OpenAI API key not found in environment or config")

        # Initialize Pydantic AI model
        self.openai_model = OpenAIModel(
            model_name=llm_config["model"],
        )

        # Pre-create common agents
        self._setup_agents()

    def _setup_agents(self) -> None:
        """Set up common Pydantic AI agents."""
        if not _PYDANTIC_AI_AVAILABLE or self.openai_model is None:
            raise ValueError("Pydantic AI components not available")

        # Simple task agent
        self.agents["simple"] = Agent(
            model=self.openai_model,
            result_type=SimpleResponse,
            system_prompt="You are a helpful AI assistant. Respond with analysis and insights.",
        )

        # Sequential task agent
        self.agents["sequential"] = Agent(
            model=self.openai_model,
            result_type=SequentialResponse,
            system_prompt="You are processing tasks in sequence. Use previous results to inform your response.",
        )

        # Complex workflow agent
        self.agents["complex"] = Agent(
            model=self.openai_model,
            result_type=ComplexResponse,
            system_prompt="You are part of a complex workflow. Consider dependencies and context.",
        )

        # General purpose agent for other scenarios
        self.agents["general"] = Agent(
            model=self.openai_model,
            result_type=str,
            system_prompt="You are a helpful AI assistant.",
        )

    async def teardown(self) -> None:
        """Cleanup Pydantic AI resources."""
        self.openai_model = None
        self.agents = {}

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Execute a simple single-task benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        try:
            agent = self.agents["simple"]

            # Execute simple task
            result = await agent.run(SIMPLE_TASK_PROMPT)

            # Log LLM output to file
            self.log_output(
                scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
                task_name="Simple Task",
                output=result.output.result,
            )

            # Count tokens
            token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + str(result.output.result))

        except Exception as e:
            logging.error(f"Error in simple task: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)

        return metrics

    async def run_sequential_pipeline(self) -> BenchmarkMetrics:
        """Execute a sequential pipeline benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        try:
            agent = self.agents["sequential"]
            previous_result = ""
            total_tokens = 0

            # Execute tasks sequentially
            for i, task in enumerate(SEQUENTIAL_TASKS):
                if i == 0:
                    prompt = f"Task {i+1}: {task}"
                else:
                    prompt = f"Previous result: {previous_result}\n\nTask {i+1}: {task}"

                result = await agent.run(prompt)
                previous_result = result.output.step_result
                total_tokens += count_tokens_estimate(prompt + previous_result)

                # Log LLM output to file
                self.log_output(
                    scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                    task_name=f"Task {i+1}",
                    output=result.output.step_result,
                )

        except Exception as e:
            logging.error(f"Error in sequential pipeline: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(SEQUENTIAL_TASKS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Execute a parallel pipeline benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        try:
            agent = self.agents["general"]

            # Create coroutines for parallel execution
            tasks = [agent.run(task) for task in PARALLEL_TASKS]

            # Execute tasks in parallel
            results = await asyncio.gather(*tasks)

            # Log LLM outputs to file
            for i, (_task, result) in enumerate(zip(PARALLEL_TASKS, results)):
                self.log_output(
                    scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                    task_name=f"Parallel Task {i+1}",
                    output=str(result.output),
                )

            # Count tokens
            total_tokens = sum(count_tokens_estimate(task + str(result.output)) for task, result in zip(PARALLEL_TASKS, results))

        except Exception as e:
            logging.error(f"Error in parallel pipeline: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(PARALLEL_TASKS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(PARALLEL_TASKS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_complex_workflow(self) -> BenchmarkMetrics:
        """Run a complex workflow benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        try:
            agent = self.agents["complex"]
            results: Dict[str, str] = {}
            total_tokens = 0

            # Execute workflow steps in dependency order
            for step in COMPLEX_WORKFLOW_STEPS:
                # Build context from dependencies
                context_parts = []
                for dep in step["depends_on"]:
                    if dep in results:
                        context_parts.append(f"{dep}: {results[dep]}")

                context = " | ".join(context_parts) if context_parts else "None"

                # Create prompt
                prompt = f"Context from dependencies: {context}\n\nTask: {step['prompt']}\nTask name: {step['task']}"

                # Execute step
                result = await agent.run(prompt)
                results[step["task"]] = result.output.result

                # Log LLM output to file
                self.log_output(
                    scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    task_name=step["task"],
                    output=result.output.result,
                )

                total_tokens += count_tokens_estimate(prompt + result.output.result)

        except Exception as e:
            logging.error(f"Error in complex workflow: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(COMPLEX_WORKFLOW_STEPS), metrics.execution_time_ms / 1000)

        return metrics

    async def run_memory_intensive(self) -> BenchmarkMetrics:
        """Run a memory-intensive benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        try:
            agent = self.agents["general"]

            # Create large data structures to test memory usage
            large_data = ["data" * 1000] * 1000  # ~4MB of string data

            # Execute memory-intensive task
            result = await agent.run(MEMORY_INTENSIVE_PROMPT)

            # Log LLM output to file
            self.log_output(
                scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
                task_name="Memory Intensive Task",
                output=str(result.output),
            )

            # Clean up
            del large_data

            token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + str(result.output))

        except Exception as e:
            logging.error(f"Error in memory-intensive task: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)

        return metrics

    async def run_concurrent_tasks(self) -> BenchmarkMetrics:
        """Run concurrent tasks benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        try:
            agent = self.agents["general"]

            # Create coroutines for concurrent execution
            tasks = [agent.run(prompt) for prompt in CONCURRENT_TASK_PROMPTS]

            # Execute all tasks concurrently
            results = await asyncio.gather(*tasks)

            # Log LLM outputs to file
            for i, (_prompt, result) in enumerate(zip(CONCURRENT_TASK_PROMPTS, results)):
                self.log_output(
                    scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                    task_name=f"Concurrent Task {i+1}",
                    output=str(result.output),
                )

            # Count tokens
            total_tokens = sum(count_tokens_estimate(prompt + str(result.output)) for prompt, result in zip(CONCURRENT_TASK_PROMPTS, results))

        except Exception as e:
            logging.error(f"Error in concurrent tasks: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            metrics.concurrent_tasks = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000)

        return metrics
