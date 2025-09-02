"""LangChain framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, Optional

from langchain.prompts import PromptTemplate
from pydantic import SecretStr
from langchain_openai import ChatOpenAI
from langchain_anthropic import ChatAnthropic
from langchain_ollama import ChatOllama

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


class LangChainBenchmark(BaseBenchmark):
    """LangChain framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any], num_runs: Optional[int] = None):
        """Initialize LangChain benchmark with configuration."""
        super().__init__(config, num_runs=num_runs)
        self.llm: Optional[Any] = None
        self.chains: Dict[str, PromptTemplate | Any] = {}

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")
        return llm_config_obj.max_tokens, llm_config_obj.temperature

    async def setup(self) -> None:
        """Set up LangChain for benchmarking."""

        # Get LLM configuration from config
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")

        max_tokens, temperature = self._get_llm_params()

        if llm_config_obj.provider == LLMProvider.OPENAI:
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")
            if not api_key:
                raise ValueError("OpenAI API key not found in environment or config")

            self.llm = ChatOpenAI(
                model=llm_config_obj.model,
                api_key=SecretStr(api_key),
                temperature=temperature,
            )

        elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
            api_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
            if not api_key:
                raise ValueError("Anthropic API key not found in environment or config")

            self.llm = ChatAnthropic(
                model=llm_config_obj.model,
                api_key=SecretStr(api_key),
                temperature=temperature,
                max_tokens=max_tokens,
            )

        elif llm_config_obj.provider == LLMProvider.OLLAMA:
            base_url = llm_config_obj.base_url or "http://localhost:11434"

            self.llm = ChatOllama(
                model=llm_config_obj.model,
                base_url=base_url,
                temperature=temperature,
                num_predict=max_tokens,
            )

        else:
            raise ValueError(f"Unsupported provider for LangChain: {llm_config_obj.provider}")

        self._setup_chains()

    def _setup_chains(self) -> None:
        """Set up common LangChain chains using RunnableSequence."""
        assert self.llm is not None, "LLM not initialized"

        simple_prompt = PromptTemplate(input_variables=["task"], template="{task}")
        self.chains["simple"] = simple_prompt | self.llm

        sequential_prompt = PromptTemplate(
            input_variables=["task", "previous_result"],
            template="Previous result: {previous_result}\n\nNew task: {task}",
        )
        self.chains["sequential"] = sequential_prompt | self.llm

        complex_prompt = PromptTemplate(
            input_variables=["task", "context"],
            template="Context: {context}\n\nTask: {task}",
        )
        self.chains["complex"] = complex_prompt | self.llm

    async def teardown(self) -> None:
        """Cleanup LangChain resources."""
        self.llm = None
        self.chains.clear()

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run a simple single-task benchmark using LangChain."""
        self.monitor.start_monitoring()
        token_count: int = 0

        try:
            chain = self.chains["simple"]
            result = await chain.ainvoke({"task": SIMPLE_TASK_PROMPT})
            result_content = result.content if hasattr(result, "content") else str(result)

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
        """Run a sequential pipeline benchmark using LangChain."""
        self.monitor.start_monitoring()
        total_tokens = 0
        previous_result = ""

        try:
            chain = self.chains["sequential"]
            for i, task in enumerate(SEQUENTIAL_TASKS):
                result = await chain.ainvoke({"task": task, "previous_result": previous_result if i > 0 else "None"})
                result_content = result.content if hasattr(result, "content") else str(result)
                previous_result = result_content
                total_tokens += count_tokens_estimate(task + result_content)

                self.log_output(
                    scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                    task_name=f"Task {i+1}",
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
        metrics.throughput_tasks_per_sec = calculate_throughput(len(SEQUENTIAL_TASKS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Run a parallel pipeline benchmark using LangChain."""
        self.monitor.start_monitoring()

        try:
            chain = self.chains["simple"]
            concurrency: int = int(self.config.get("concurrency", len(PARALLEL_TASKS)))
            sem = asyncio.Semaphore(concurrency)

            async def run_with_sem(t: str) -> Any:
                async with sem:
                    return await chain.ainvoke({"task": t})

            tasks = [run_with_sem(task) for task in PARALLEL_TASKS]
            results = await asyncio.gather(*tasks)

            result_contents = [result.content if hasattr(result, "content") else str(result) for result in results]

            for i, result_content in enumerate(result_contents):
                self.log_output(
                    scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                    task_name=f"Task {i+1}",
                    output=result_content,
                )

            total_tokens = sum(count_tokens_estimate(task + str(result)) for task, result in zip(PARALLEL_TASKS, result_contents))

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
        """Run a complex workflow benchmark using LangChain."""
        self.monitor.start_monitoring()
        total_tokens = 0
        results: Dict[str, str] = {}

        try:
            chain = self.chains["complex"]

            for step in COMPLEX_WORKFLOW_STEPS:
                context_parts = [f"{dep}: {results[dep]}" for dep in step["depends_on"] if dep in results]
                context = " | ".join(context_parts) if context_parts else "None"

                result = await chain.ainvoke(
                    {
                        "task": step["prompt"],
                        "context": context,
                    }
                )

                result_content = result.content if hasattr(result, "content") else str(result)
                results[step["task"]] = result_content
                total_tokens += count_tokens_estimate(step["prompt"] + context + result_content)

                self.log_output(
                    scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    task_name=step["task"],
                    output=result_content,
                )

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
        """Run a memory-intensive benchmark using LangChain."""
        self.monitor.start_monitoring()
        token_count = 0

        try:
            chain = self.chains["simple"]
            large_data = ["data" * 1000] * 1000  # ~4MB of string data
            result = await chain.ainvoke({"task": MEMORY_INTENSIVE_PROMPT})
            result_content = result.content if hasattr(result, "content") else str(result)
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
        """Run concurrent tasks benchmark using LangChain."""
        self.monitor.start_monitoring()
        total_tokens = 0

        try:
            chain = self.chains["simple"]
            concurrency: int = int(self.config.get("concurrency", len(CONCURRENT_TASK_PROMPTS)))
            sem = asyncio.Semaphore(concurrency)

            async def run_with_sem(p: str) -> Any:
                async with sem:
                    return await chain.ainvoke({"task": p})

            tasks = [run_with_sem(prompt) for prompt in CONCURRENT_TASK_PROMPTS]
            results = await asyncio.gather(*tasks)

            result_contents = [result.content if hasattr(result, "content") else str(result) for result in results]

            for i, result_content in enumerate(result_contents):
                self.log_output(
                    scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                    task_name=f"Task {i+1}",
                    output=result_content,
                )

            total_tokens = sum(count_tokens_estimate(prompt + result) for prompt, result in zip(CONCURRENT_TASK_PROMPTS, result_contents))

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
        metrics.throughput_tasks_per_sec = calculate_throughput(len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000)
        return metrics
