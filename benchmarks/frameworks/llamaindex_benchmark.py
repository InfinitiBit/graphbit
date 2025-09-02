"""LlamaIndex framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, List, Optional

from llama_index.core import Settings, VectorStoreIndex
from llama_index.core.schema import Document
from llama_index.llms.openai import OpenAI
from llama_index.llms.anthropic import Anthropic
from llama_index.llms.ollama import Ollama

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


class LlamaIndexBenchmark(BaseBenchmark):
    """LlamaIndex framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any], num_runs: Optional[int] = None):
        """Initialize LlamaIndex benchmark with configuration."""
        super().__init__(config, num_runs=num_runs)
        self.llm: Optional[Any] = None
        self.index: Optional[VectorStoreIndex] = None
        self.query_engine: Optional[Any] = None
        self.chat_engine: Optional[Any] = None

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")
        return llm_config_obj.max_tokens, llm_config_obj.temperature

    async def setup(self) -> None:
        """Set up LlamaIndex for benchmarking."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")

        max_tokens, temperature = self._get_llm_params()

        if llm_config_obj.provider == LLMProvider.OPENAI:
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")
            if not api_key:
                raise ValueError("OpenAI API key not found in environment or config")

            self.llm = OpenAI(
                model=llm_config_obj.model,
                api_key=api_key,
                temperature=temperature,
                max_tokens=max_tokens,
            )

        elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
            api_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
            if not api_key:
                raise ValueError("Anthropic API key not found in environment or config")

            self.llm = Anthropic(
                model=llm_config_obj.model,
                api_key=api_key,
                temperature=temperature,
                max_tokens=max_tokens,
            )

        elif llm_config_obj.provider == LLMProvider.OLLAMA:
            base_url = llm_config_obj.base_url or "http://localhost:11434"

            self.llm = Ollama(
                model=llm_config_obj.model,
                base_url=base_url,
                temperature=temperature,
                request_timeout=60.0,
            )

        else:
            raise ValueError(f"Unsupported provider for LlamaIndex: {llm_config_obj.provider}")

        Settings.llm = self.llm

        documents: List[Document] = [
            Document(text="This is a sample document for benchmarking purposes."),
            Document(text="Another document with different content to test retrieval."),
            Document(text="Complex workflow tasks require sophisticated reasoning capabilities."),
        ]

        self.index = VectorStoreIndex.from_documents(documents)
        self.query_engine = self.index.as_query_engine()
        self.chat_engine = self.index.as_chat_engine()

    async def teardown(self) -> None:
        """Cleanup LlamaIndex resources."""
        self.llm = None
        self.index = None
        self.query_engine = None
        self.chat_engine = None

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run a simple single-task benchmark using LlamaIndex."""
        self.monitor.start_monitoring()

        if self.llm is None:
            raise ValueError("LLM not initialized")
        response = await self.llm.acomplete(SIMPLE_TASK_PROMPT)
        result = str(response)

        self.log_output(
            scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
            task_name="Simple Task",
            output=result,
        )

        token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + result)

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)
        return metrics

    async def run_sequential_pipeline(self) -> BenchmarkMetrics:
        """Run a sequential pipeline benchmark using LlamaIndex."""
        self.monitor.start_monitoring()

        if self.llm is None:
            raise ValueError("LLM not initialized")

        previous_result = ""
        total_tokens = 0

        for i, task in enumerate(SEQUENTIAL_TASKS):
            prompt = task if i == 0 else f"Previous result: {previous_result}\n\nNew task: {task}"

            response = await self.llm.acomplete(prompt)
            result = str(response)
            previous_result = result
            total_tokens += count_tokens_estimate(prompt + result)

            self.log_output(
                scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                task_name=f"Task {i+1}",
                output=result,
            )

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(SEQUENTIAL_TASKS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Run a parallel pipeline benchmark using LlamaIndex."""
        self.monitor.start_monitoring()

        if self.llm is None:
            raise ValueError("LLM not initialized")

        llm = self.llm  # Capture for closure
        concurrency: int = int(self.config.get("concurrency", len(PARALLEL_TASKS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_with_sem(t: str) -> Any:
            async with sem:
                return await llm.acomplete(t)

        tasks = [run_with_sem(task) for task in PARALLEL_TASKS]
        responses = await asyncio.gather(*tasks)
        results = [str(response) for response in responses]

        total_tokens = sum(count_tokens_estimate(task + result) for task, result in zip(PARALLEL_TASKS, results))

        for i, result in enumerate(results):
            self.log_output(
                scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                task_name=f"Parallel Task {i+1}",
                output=result,
            )

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(PARALLEL_TASKS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(PARALLEL_TASKS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_complex_workflow(self) -> BenchmarkMetrics:
        """Run a complex workflow benchmark using LlamaIndex query engine."""
        self.monitor.start_monitoring()

        if self.query_engine is None or self.llm is None:
            raise ValueError("Query engine or LLM not initialized")

        total_tokens = 0
        results: List[str] = []

        for i, step in enumerate(COMPLEX_WORKFLOW_STEPS):
            response = self.query_engine.query(step["prompt"])
            result = str(response)
            results.append(result)
            total_tokens += count_tokens_estimate(step["prompt"] + result)

            self.log_output(
                scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                task_name=f"Workflow Step {i+1}",
                output=result,
            )

        synthesis_prompt = "Synthesize the following results into a comprehensive conclusion:\n\n" + "\n\n".join([f"Step {i+1}: {result}" for i, result in enumerate(results)])

        final_response = await self.llm.acomplete(synthesis_prompt)
        final_result = str(final_response)
        total_tokens += count_tokens_estimate(synthesis_prompt + final_result)

        self.log_output(
            scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
            task_name="Final Synthesis",
            output=final_result,
        )

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(COMPLEX_WORKFLOW_STEPS) + 1, metrics.execution_time_ms / 1000)
        return metrics

    async def run_memory_intensive(self) -> BenchmarkMetrics:
        """Run a memory-intensive benchmark using LlamaIndex."""
        self.monitor.start_monitoring()

        if self.llm is None:
            raise ValueError("LLM not initialized")

        # Create large documents to stress memory
        large_documents = [Document(text=MEMORY_INTENSIVE_PROMPT + f" Document {i+1}. " + "Lorem ipsum dolor sit amet. " * 100) for i in range(50)]

        # Create index but use LLM directly for consistent token counts
        VectorStoreIndex.from_documents(large_documents)

        # Use the LLM directly with the full MEMORY_INTENSIVE_PROMPT for consistent token counting
        response = await self.llm.acomplete(MEMORY_INTENSIVE_PROMPT)
        result = str(response)

        self.log_output(
            scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
            task_name="Memory Intensive Task",
            output=result,
        )

        token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + result)

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)
        return metrics

    async def run_concurrent_tasks(self) -> BenchmarkMetrics:
        """Run concurrent tasks benchmark using LlamaIndex."""
        self.monitor.start_monitoring()

        if self.llm is None:
            raise ValueError("LLM not initialized")

        llm = self.llm  # Capture for closure
        concurrency: int = int(self.config.get("concurrency", len(CONCURRENT_TASK_PROMPTS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_with_sem(text: str) -> Any:
            async with sem:
                return await llm.acomplete(text)

        llm_tasks = [run_with_sem(f"Task {i+1}: {prompt}") for i, prompt in enumerate(CONCURRENT_TASK_PROMPTS[: len(CONCURRENT_TASK_PROMPTS) // 2])]

        query_tasks = [
            run_with_sem(f"Query Task {i+1}: {prompt}")
            for i, prompt in enumerate(
                CONCURRENT_TASK_PROMPTS[len(CONCURRENT_TASK_PROMPTS) // 2 :],
                start=len(CONCURRENT_TASK_PROMPTS) // 2,
            )
        ]

        all_tasks = llm_tasks + query_tasks
        responses = await asyncio.gather(*all_tasks)
        results: List[str] = [str(response) for response in responses]

        total_tokens = sum(count_tokens_estimate(prompt + result) for prompt, result in zip(CONCURRENT_TASK_PROMPTS, results))

        for i, (_prompt, result) in enumerate(zip(CONCURRENT_TASK_PROMPTS, results)):
            self.log_output(
                scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                task_name=f"Concurrent Task {i+1}",
                output=result,
            )

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000)
        return metrics
