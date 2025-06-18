"""Pydantic AI framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, List, Optional

from pydantic import BaseModel
from pydantic_ai import Agent
from pydantic_ai.models.openai import OpenAIModel

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
        self.openai_model: Optional[OpenAIModel] = None
        self.agents: Dict[str, Agent] = {}

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
        if self.openai_model is None:
            raise ValueError("OpenAIModel not initialized")

        self.agents["simple"] = Agent(
            model=self.openai_model,
            result_type=SimpleResponse,
            system_prompt="You are a helpful AI assistant. Respond with analysis and insights.",
        )

        self.agents["sequential"] = Agent(
            model=self.openai_model,
            result_type=SequentialResponse,
            system_prompt="You are processing tasks in sequence. Use previous results to inform your response.",
        )

        self.agents["complex"] = Agent(
            model=self.openai_model,
            result_type=ComplexResponse,
            system_prompt="You are part of a complex workflow. Consider dependencies and context.",
        )

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

        agent: Agent = self.agents["simple"]
        result = await agent.run(SIMPLE_TASK_PROMPT)

        self.log_output(
            scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
            task_name="Simple Task",
            output=result.output.result,
        )

        token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + result.output.result)
        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)
        return metrics

    async def run_sequential_pipeline(self) -> BenchmarkMetrics:
        """Execute a sequential pipeline benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        agent: Agent = self.agents["sequential"]
        previous_result: str = ""
        total_tokens = 0

        for i, task in enumerate(SEQUENTIAL_TASKS):
            if i == 0:
                prompt = f"Task {i+1}: {task}"
            else:
                prompt = f"Previous result: {previous_result}\n\nTask {i+1}: {task}"

            result = await agent.run(prompt)
            previous_result = result.output.step_result
            total_tokens += count_tokens_estimate(prompt + previous_result)

            self.log_output(
                scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                task_name=f"Task {i+1}",
                output=previous_result,
            )

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(SEQUENTIAL_TASKS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Execute a parallel pipeline benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        agent: Agent = self.agents["general"]
        tasks = [agent.run(task) for task in PARALLEL_TASKS]
        results = await asyncio.gather(*tasks)

        for i, (_, result) in enumerate(zip(PARALLEL_TASKS, results)):
            self.log_output(
                scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                task_name=f"Parallel Task {i+1}",
                output=str(result.output),
            )

        total_tokens = sum(count_tokens_estimate(task + str(result.output)) for task, result in zip(PARALLEL_TASKS, results))

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(PARALLEL_TASKS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(PARALLEL_TASKS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_complex_workflow(self) -> BenchmarkMetrics:
        """Run a complex workflow benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        agent: Agent = self.agents["complex"]
        results: Dict[str, str] = {}
        total_tokens = 0

        for step in COMPLEX_WORKFLOW_STEPS:
            context_parts = [f"{dep}: {results[dep]}" for dep in step["depends_on"] if dep in results]
            context = " | ".join(context_parts) if context_parts else "None"

            prompt = f"Context from dependencies: {context}\n\n" f"Task: {step['prompt']}\nTask name: {step['task']}"

            result = await agent.run(prompt)
            results[step["task"]] = result.output.result

            self.log_output(
                scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                task_name=step["task"],
                output=result.output.result,
            )

            total_tokens += count_tokens_estimate(prompt + result.output.result)

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(COMPLEX_WORKFLOW_STEPS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_memory_intensive(self) -> BenchmarkMetrics:
        """Run a memory-intensive benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        agent: Agent = self.agents["general"]

        # Simulate large memory usage (no real processing)
        large_data: List[str] = ["data" * 1000] * 1000  # ~4MB of string data

        result = await agent.run(MEMORY_INTENSIVE_PROMPT)

        self.log_output(
            scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
            task_name="Memory Intensive Task",
            output=str(result.output),
        )

        token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + str(result.output))
        del large_data

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)
        return metrics

    async def run_concurrent_tasks(self) -> BenchmarkMetrics:
        """Run concurrent tasks benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        agent: Agent = self.agents["general"]
        tasks = [agent.run(prompt) for prompt in CONCURRENT_TASK_PROMPTS]
        results = await asyncio.gather(*tasks)

        for i, (_, result) in enumerate(zip(CONCURRENT_TASK_PROMPTS, results)):
            self.log_output(
                scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                task_name=f"Concurrent Task {i+1}",
                output=str(result.output),
            )

        total_tokens = sum(count_tokens_estimate(prompt + str(result.output)) for prompt, result in zip(CONCURRENT_TASK_PROMPTS, results))

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000)
        return metrics
