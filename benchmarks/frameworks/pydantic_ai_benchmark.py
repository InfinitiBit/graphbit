"""Pydantic AI framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, List, Optional

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
    LLMConfig,
    LLMProvider,
    calculate_throughput,
    count_tokens_estimate,
)


class PydanticAIBenchmark(BaseBenchmark):
    """Pydantic AI framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any], num_runs: Optional[int] = None):
        """Initialize PydanticAI benchmark with configuration."""
        super().__init__(config, num_runs=num_runs)
        self.model: Optional[Any] = None
        self.agents: Dict[str, Any] = {}
        # Lazy loaded classes
        self._Agent: Optional[Any] = None
        self._ModelSettings: Optional[Any] = None

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")
        return llm_config_obj.max_tokens, llm_config_obj.temperature

    async def setup(self) -> None:
        """Set up Pydantic AI for benchmarking."""
        # Lazy import PydanticAI dependencies
        from pydantic_ai import Agent
        from pydantic_ai.settings import ModelSettings
        
        self._Agent = Agent
        self._ModelSettings = ModelSettings
        
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")

        if llm_config_obj.provider == LLMProvider.OPENAI:
            from pydantic_ai.models.openai import OpenAIModel
            
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")
            if not api_key:
                raise ValueError("OpenAI API key not found in environment or config")

            self.model = OpenAIModel(model_name=llm_config_obj.model)

        elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
            from pydantic_ai.models.anthropic import AnthropicModel
            
            api_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
            if not api_key:
                raise ValueError("Anthropic API key not found in environment or config")

            self.model = AnthropicModel(model_name=llm_config_obj.model)

        elif llm_config_obj.provider == LLMProvider.OLLAMA:
            raise ValueError("PydanticAI does not support OLLAMA provider. Please use OpenAI or Anthropic.")

        elif llm_config_obj.provider == LLMProvider.AZURE_OPENAI:
            from pydantic_ai.models.openai import OpenAIModel
            
            api_key = llm_config_obj.api_key or os.getenv("AZURE_OPENAI_API_KEY")
            azure_endpoint = llm_config_obj.base_url or os.getenv("AZURE_OPENAI_ENDPOINT")
            api_version = llm_config_obj.api_version or os.getenv("AZURE_OPENAI_API_VERSION", "2024-02-15-preview")
            
            if not api_key:
                raise ValueError("Azure OpenAI API key not found in environment or config")
            if not azure_endpoint:
                raise ValueError("Azure OpenAI endpoint not found in environment or config")

            # PydanticAI uses OpenAIModel with Azure parameters
            self.model = OpenAIModel(
                model_name=llm_config_obj.model,  # deployment name
                base_url=f"{azure_endpoint}/openai/deployments/{llm_config_obj.model}",
                api_key=api_key,
                openai_client_params={"default_headers": {"api-version": api_version}},
            )

        else:
            raise ValueError(f"Unsupported provider for PydanticAI: {llm_config_obj.provider}")

        # Pre-create common agents
        self._setup_agents()

    def _setup_agents(self) -> None:
        """Set up common Pydantic AI agents."""
        if self.model is None:
            raise ValueError("Model not initialized")
        assert self._Agent is not None, "Agent class not loaded"
        assert self._ModelSettings is not None, "ModelSettings class not loaded"
        
        Agent = self._Agent
        ModelSettings = self._ModelSettings

        max_tokens, temperature = self._get_llm_params()
        model_settings = ModelSettings(
            temperature=temperature,
            max_tokens=max_tokens,
        )

        self.agents["simple"] = Agent(
            model=self.model,
            output_type=str,
            model_settings=model_settings,
        )

        self.agents["sequential"] = Agent(
            model=self.model,
            output_type=str,
            model_settings=model_settings,
        )

        self.agents["complex"] = Agent(
            model=self.model,
            output_type=str,
            model_settings=model_settings,
        )

        self.agents["general"] = Agent(
            model=self.model,
            output_type=str,
            model_settings=model_settings,
        )

    async def teardown(self) -> None:
        """Cleanup Pydantic AI resources."""
        self.model = None
        self.agents = {}

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Execute a simple single-task benchmark using Pydantic AI."""
        self.monitor.start_monitoring()

        agent: Agent = self.agents["simple"]
        result = await agent.run(SIMPLE_TASK_PROMPT)

        self.log_output(
            scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
            task_name="Simple Task",
            output=str(result.output),
        )

        token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + str(result.output))
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
            previous_result = str(result.output)
            # Count tokens based on original task only for fair comparison
            total_tokens += count_tokens_estimate(task + previous_result)

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
        concurrency: int = int(self.config.get("concurrency", len(PARALLEL_TASKS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_with_sem(t: str) -> Any:
            async with sem:
                return await agent.run(t)

        tasks = [run_with_sem(task) for task in PARALLEL_TASKS]
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
            results[step["task"]] = str(result.output)

            self.log_output(
                scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                task_name=step["task"],
                output=str(result.output),
            )

            # Count tokens based on original step prompt only for fair comparison
            total_tokens += count_tokens_estimate(step["prompt"] + str(result.output))

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
        concurrency: int = int(self.config.get("concurrency", len(CONCURRENT_TASK_PROMPTS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_with_sem(p: str) -> Any:
            async with sem:
                return await agent.run(p)

        tasks = [run_with_sem(prompt) for prompt in CONCURRENT_TASK_PROMPTS]
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
