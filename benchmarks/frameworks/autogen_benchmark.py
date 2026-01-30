"""AutoGen framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, Optional

from autogen_agentchat.agents import AssistantAgent
from autogen_agentchat.messages import TextMessage
from autogen_ext.models.anthropic import AnthropicChatCompletionClient
from autogen_ext.models.ollama import OllamaChatCompletionClient
from autogen_ext.models.openai import OpenAIChatCompletionClient

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


class AutogenBenchmark(BaseBenchmark):
    """AutoGen framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any], num_runs: Optional[int] = None):
        """Initialize AutoGen benchmark with configuration."""
        super().__init__(config, num_runs=num_runs)
        self.agent: Optional[AssistantAgent] = None

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")
        return llm_config_obj.max_tokens, llm_config_obj.temperature

    async def setup(self) -> None:
        """Set up AutoGen for benchmarking."""
        # Get LLM configuration from config
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")

        max_tokens, temperature = self._get_llm_params()

        if llm_config_obj.provider == LLMProvider.OPENAI:
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")
            if not api_key:
                raise ValueError("OpenAI API key not found in environment or config")

            self.model_client = OpenAIChatCompletionClient(
                model=self.model,
                api_key=api_key,
                temperature=temperature,
                max_tokens=max_tokens,
            )
            self.agent = AssistantAgent("assistant", model_client=self.model_client)

        elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
            api_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
            if not api_key:
                raise ValueError("Anthropic API key not found in environment or config")
            self.model_client = AnthropicChatCompletionClient(
                model=self.model,
                api_key=api_key,
                temperature=temperature,
                max_tokens=max_tokens,
            )
            self.agent = AssistantAgent("assistant", model_client=self.model_client)

        elif llm_config_obj.provider == LLMProvider.OLLAMA:
            base_url = llm_config_obj.base_url or "http://localhost:11434"
            model_client = OllamaChatCompletionClient(
                model=self.model,
                host=base_url,
                options={
                    "temperature": temperature,
                    "num_predict": max_tokens,
                },
            )
            self.agent = AssistantAgent("assistant", model_client=model_client)

        else:
            raise ValueError(f"Unsupported provider for AutoGen: {llm_config_obj.provider}")

    async def teardown(self) -> None:
        """Cleanup AutoGen resources."""
        self.agent = None

    async def _ask(self, prompt: str) -> str:
        assert self.agent is not None
        agent = self.agent
        response = await agent.on_messages(
            [TextMessage(content=prompt, source="user")],
            None,
        )
        return response.chat_message.content

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run a simple single-task benchmark using AutoGen."""
        self.monitor.start_monitoring()
        token_count: int = 0

        try:
            result_content = ""
            agent = self.agent
            result = await agent.run(task=SIMPLE_TASK_PROMPT)
            llm_response = []
            for message in result.messages:
                if message.source != "user":
                    llm_response.append(message.content)
            result_content = "".join(llm_response)

            self.log_output(
                scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
                task_name="Simple Task",
                output=result_content,
            )

            token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + result_content)

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
        """Run a sequential pipeline benchmark using AutoGen."""
        self.monitor.start_monitoring()
        total_tokens = 0
        previous_result = ""

        try:
            agent = self.agent
            for i, task in enumerate(SEQUENTIAL_TASKS):
                prompt = f"Previous result:\n{previous_result}\n\nNew task:\n{task}" if i > 0 else task

                response = await agent.on_messages(
                    [TextMessage(content=prompt, source="user")],
                    None,
                )

                result_content = response.chat_message.content
                previous_result = result_content
                total_tokens += count_tokens_estimate(task + result_content)

                self.log_output(
                    scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                    task_name=f"Task {i + 1}",
                    output=result_content,
                )

        except Exception as e:
            self.logger.error(f"Error in sequential pipeline benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(
            len(SEQUENTIAL_TASKS), metrics.execution_time_ms / 1000
        )
        return metrics

    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Run a parallel pipeline benchmark using AutoGen."""
        self.monitor.start_monitoring()
        total_tokens = 0
        concurrency = int(self.config.get("concurrency", len(PARALLEL_TASKS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_task(task: str) -> str:
            async with sem:
                return await self._ask(task)

        try:
            results = await asyncio.gather(*(run_task(t) for t in PARALLEL_TASKS))
            for i, output in enumerate(results):
                self.log_output(
                    BenchmarkScenario.PARALLEL_PIPELINE.value,
                    f"Task {i + 1}",
                    output,
                )

            total_tokens = sum(count_tokens_estimate(t + r) for t, r in zip(PARALLEL_TASKS, results))
        except Exception as e:
            self.logger.error(f"Error in parallel pipeline benchmark: {e}")
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
        """Run a complex workflow benchmark using AutoGen."""
        self.monitor.start_monitoring()
        total_tokens = 0
        results: Dict[str, str] = {}

        try:
            for step in COMPLEX_WORKFLOW_STEPS:
                context = (
                    " | ".join(
                        f"{dependency}: {results[dependency]}"
                        for dependency in step["depends_on"]
                        if dependency in results
                    )
                    or "None"
                )

                prompt = f"Context:\n{context}\n\nTask:\n{step['prompt']}"
                output = await self._ask(prompt)

                results[step["task"]] = output
                total_tokens += count_tokens_estimate(prompt + output)

                self.log_output(
                    BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    step["task"],
                    output,
                )

        except Exception as e:
            self.logger.error(f"Error in complex workflow benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(
            len(COMPLEX_WORKFLOW_STEPS), metrics.execution_time_ms / 1000
        )
        return metrics

    async def run_memory_intensive(self) -> BenchmarkMetrics:
        """Run a memory-intensive benchmark using AutoGen."""
        self.monitor.start_monitoring()
        token_count = 0

        try:
            large_data = ["data" * 1000] * 1000  # ~4MB
            result_content = await self._ask(MEMORY_INTENSIVE_PROMPT)
            del large_data

            self.log_output(
                scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
                task_name="Memory Intensive Task",
                output=result_content,
            )

            token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + result_content)

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
        """Run concurrent tasks benchmark using AutoGen."""
        self.monitor.start_monitoring()
        total_tokens = 0
        concurrency = int(self.config.get("concurrency", len(CONCURRENT_TASK_PROMPTS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_task(prompt: str) -> str:
            async with sem:
                return await self._ask(prompt)

        try:
            results = await asyncio.gather(*(run_task(p) for p in CONCURRENT_TASK_PROMPTS))
            for i, output in enumerate(results):
                self.log_output(
                    BenchmarkScenario.CONCURRENT_TASKS.value,
                    f"Task {i + 1}",
                    output,
                )

            total_tokens = sum(count_tokens_estimate(p + r) for p, r in zip(CONCURRENT_TASK_PROMPTS, results))

        except Exception as e:
            self.logger.error(f"Error in concurrent tasks benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            metrics.concurrent_tasks = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.throughput_tasks_per_sec = calculate_throughput(
            len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000
        )
        return metrics
