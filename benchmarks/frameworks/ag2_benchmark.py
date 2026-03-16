"""AG2 (formerly AutoGen) framework benchmark implementation.

Uses ag2>=0.11.0 — the community-maintained successor to Microsoft AutoGen.
Package: pip install "ag2[openai,anthropic,ollama]>=0.11.0"
GitHub: https://github.com/ag2ai/ag2
"""

import asyncio
import os
from typing import Any, Dict, Optional

from autogen import AssistantAgent, GroupChat, GroupChatManager, LLMConfig, UserProxyAgent

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
    LLMConfig as BenchLLMConfig,
    LLMProvider,
    calculate_throughput,
    count_tokens_estimate,
)


def _make_llm_config(bench_config: "BenchLLMConfig") -> LLMConfig:
    """Translate benchmark LLMConfig to AG2 LLMConfig."""
    entry: dict[str, Any] = {"model": bench_config.model}

    if bench_config.provider == LLMProvider.OPENAI:
        api_key = bench_config.api_key or os.getenv("OPENAI_API_KEY", "")
        entry["api_key"] = api_key
    elif bench_config.provider == LLMProvider.ANTHROPIC:
        api_key = bench_config.api_key or os.getenv("ANTHROPIC_API_KEY", "")
        entry["api_key"] = api_key
        entry["api_type"] = "anthropic"
    elif bench_config.provider == LLMProvider.OLLAMA:
        base_url = bench_config.base_url or "http://localhost:11434/v1"
        entry["api_key"] = "ollama"
        entry["base_url"] = base_url
        entry["api_type"] = "openai"

    return LLMConfig(
        entry,
        temperature=bench_config.temperature,
        max_tokens=bench_config.max_tokens,
    )


def _run_chat(initiator: UserProxyAgent, recipient: Any, message: str) -> str:
    """Run a synchronous chat and return the last assistant message."""
    chat_result = initiator.initiate_chat(recipient, message=message, max_turns=2)
    # Last message from the conversation
    for msg in reversed(chat_result.chat_history):
        if msg.get("role") != "user":
            return msg.get("content", "")
    return ""


class AG2Benchmark(BaseBenchmark):
    """AG2 (formerly AutoGen) framework benchmark.

    Uses ag2>=0.11.0 with the community-maintained package.
    Complex workflow scenario uses GroupChat with LLM-driven speaker selection
    to match the multi-agent sophistication of other benchmarked frameworks.
    """

    def __init__(self, config: Dict[str, Any], num_runs: Optional[int] = None):
        super().__init__(config, num_runs=num_runs)
        self._llm_config: Optional[LLMConfig] = None
        self._executor: Optional[UserProxyAgent] = None

    async def setup(self) -> None:
        """Set up AG2 config and reusable executor proxy."""
        bench_llm: BenchLLMConfig = self.config["llm_config"]
        self._llm_config = _make_llm_config(bench_llm)
        # Silent executor proxy — never asks for human input, never executes code
        self._executor = UserProxyAgent(
            name="executor",
            human_input_mode="NEVER",
            code_execution_config=False,
            max_consecutive_auto_reply=0,
        )

    async def teardown(self) -> None:
        self._llm_config = None
        self._executor = None

    def _single_agent(self, name: str = "assistant") -> AssistantAgent:
        return AssistantAgent(name=name, llm_config=self._llm_config)

    def _ask_sync(self, prompt: str, agent_name: str = "assistant") -> str:
        """Ask a single agent and return its response."""
        agent = self._single_agent(agent_name)
        result = _run_chat(self._executor, agent, prompt)
        return result

    async def _ask(self, prompt: str) -> str:
        """Async wrapper for _ask_sync (runs in executor thread)."""
        loop = asyncio.get_event_loop()
        return await loop.run_in_executor(None, self._ask_sync, prompt)

    # ------------------------------------------------------------------
    # Benchmark scenarios
    # ------------------------------------------------------------------

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Single agent, single prompt."""
        self.monitor.start_monitoring()
        result = ""
        try:
            result = await self._ask(SIMPLE_TASK_PROMPT)
            self.log_output(BenchmarkScenario.SIMPLE_TASK.value, "Simple Task", result)
        except Exception as e:
            self.logger.error(f"AG2 simple task error: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            return metrics
        metrics = self.monitor.stop_monitoring()
        metrics.token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + result)
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)
        return metrics

    async def run_sequential_pipeline(self) -> BenchmarkMetrics:
        """Chain of tasks with context passed between calls."""
        self.monitor.start_monitoring()
        total_tokens = 0
        previous = ""
        try:
            for i, task in enumerate(SEQUENTIAL_TASKS):
                prompt = f"Previous result:\n{previous}\n\nNew task:\n{task}" if i > 0 else task
                result = await self._ask(prompt)
                previous = result
                total_tokens += count_tokens_estimate(task + result)
                self.log_output(BenchmarkScenario.SEQUENTIAL_PIPELINE.value, f"Task {i + 1}", result)
        except Exception as e:
            self.logger.error(f"AG2 sequential pipeline error: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            return metrics
        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(len(SEQUENTIAL_TASKS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Concurrent independent tasks — each gets its own AssistantAgent instance."""
        self.monitor.start_monitoring()
        total_tokens = 0
        concurrency = int(self.config.get("concurrency", len(PARALLEL_TASKS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_one(task: str) -> str:
            async with sem:
                return await self._ask(task)

        try:
            results = await asyncio.gather(*(run_one(t) for t in PARALLEL_TASKS))
            for i, output in enumerate(results):
                self.log_output(BenchmarkScenario.PARALLEL_PIPELINE.value, f"Task {i + 1}", output)
            total_tokens = sum(count_tokens_estimate(t + r) for t, r in zip(PARALLEL_TASKS, results))
        except Exception as e:
            self.logger.error(f"AG2 parallel pipeline error: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            return metrics
        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(PARALLEL_TASKS)
        metrics.throughput_tasks_per_sec = calculate_throughput(len(PARALLEL_TASKS), metrics.execution_time_ms / 1000)
        return metrics

    async def run_complex_workflow(self) -> BenchmarkMetrics:
        """Multi-step workflow using GroupChat with three specialized agents.

        GroupChat with speaker_selection_method='auto' gives the LLM control over
        which specialist handles each step — equivalent to CrewAI's crew orchestration
        and LangGraph's conditional edge routing.
        """
        self.monitor.start_monitoring()
        total_tokens = 0

        # Build the task description from COMPLEX_WORKFLOW_STEPS
        workflow_description = "\n".join(
            f"Step {i + 1} ({step['task']}): {step['prompt']}"
            for i, step in enumerate(COMPLEX_WORKFLOW_STEPS)
        )
        full_task = (
            "Complete the following multi-step workflow. Each step may build on previous results.\n\n"
            + workflow_description
        )

        try:
            # Three specialized agents — LLM chooses which one speaks for each step
            analyst = AssistantAgent(
                name="analyst",
                system_message=(
                    "You are an analytical specialist. Handle analysis, evaluation, and reasoning tasks. "
                    "Say TERMINATE when the full workflow is complete."
                ),
                llm_config=self._llm_config,
            )
            planner = AssistantAgent(
                name="planner",
                system_message=(
                    "You are a planning specialist. Handle task decomposition, sequencing, and coordination."
                ),
                llm_config=self._llm_config,
            )
            executor_agent = AssistantAgent(
                name="executor_agent",
                system_message=(
                    "You are an execution specialist. Implement plans, produce concrete outputs and results."
                ),
                llm_config=self._llm_config,
            )

            groupchat = GroupChat(
                agents=[analyst, planner, executor_agent],
                messages=[],
                max_round=len(COMPLEX_WORKFLOW_STEPS) * 2 + 2,
                speaker_selection_method="auto",
            )
            manager = GroupChatManager(groupchat=groupchat, llm_config=self._llm_config)

            loop = asyncio.get_event_loop()

            def run_group_chat() -> str:
                result = self._executor.initiate_chat(manager, message=full_task)
                return "\n".join(
                    m.get("content", "")
                    for m in result.chat_history
                    if m.get("role") != "user"
                )

            combined_output = await loop.run_in_executor(None, run_group_chat)
            total_tokens = count_tokens_estimate(full_task + combined_output)
            self.log_output(BenchmarkScenario.COMPLEX_WORKFLOW.value, "GroupChat Workflow", combined_output)

        except Exception as e:
            self.logger.error(f"AG2 complex workflow error: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.throughput_tasks_per_sec = calculate_throughput(
            len(COMPLEX_WORKFLOW_STEPS), metrics.execution_time_ms / 1000
        )
        return metrics

    async def run_memory_intensive(self) -> BenchmarkMetrics:
        """Large context processing with single agent."""
        self.monitor.start_monitoring()
        result = ""
        try:
            large_data = ["data" * 1000] * 1000  # ~4MB allocated, then freed
            result = await self._ask(MEMORY_INTENSIVE_PROMPT)
            del large_data
            self.log_output(BenchmarkScenario.MEMORY_INTENSIVE.value, "Memory Intensive", result)
        except Exception as e:
            self.logger.error(f"AG2 memory intensive error: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            return metrics
        metrics = self.monitor.stop_monitoring()
        metrics.token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + result)
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)
        return metrics

    async def run_concurrent_tasks(self) -> BenchmarkMetrics:
        """Multiple simultaneous agent operations."""
        self.monitor.start_monitoring()
        total_tokens = 0
        concurrency = int(self.config.get("concurrency", len(CONCURRENT_TASK_PROMPTS)))
        sem = asyncio.Semaphore(concurrency)

        async def run_one(prompt: str) -> str:
            async with sem:
                return await self._ask(prompt)

        try:
            results = await asyncio.gather(*(run_one(p) for p in CONCURRENT_TASK_PROMPTS))
            for i, output in enumerate(results):
                self.log_output(BenchmarkScenario.CONCURRENT_TASKS.value, f"Task {i + 1}", output)
            total_tokens = sum(count_tokens_estimate(p + r) for p, r in zip(CONCURRENT_TASK_PROMPTS, results))
        except Exception as e:
            self.logger.error(f"AG2 concurrent tasks error: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.concurrent_tasks = 0
            return metrics
        metrics = self.monitor.stop_monitoring()
        metrics.token_count = total_tokens
        metrics.concurrent_tasks = len(CONCURRENT_TASK_PROMPTS)
        metrics.throughput_tasks_per_sec = calculate_throughput(
            len(CONCURRENT_TASK_PROMPTS), metrics.execution_time_ms / 1000
        )
        return metrics
