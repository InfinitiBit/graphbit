#!/usr/bin/env python3

"""GraphBit framework benchmark implementation.

Optimized to use direct API calls for maximum performance, bypassing workflow overhead.
"""

import os
import sys
from typing import Dict

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
    LLMConfig,
    LLMProvider,
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

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if llm_config_obj:
            return llm_config_obj.max_tokens, llm_config_obj.temperature
        else:
            llm_config = get_standard_llm_config(self.config)
            return llm_config["max_tokens"], llm_config["temperature"]

    async def setup(self) -> None:
        """Set up GraphBit with minimal overhead configuration."""
        # Initialize GraphBit (minimal initialization)
        graphbit.init()

        # Get LLM configuration from config
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            # Fallback to old format for backward compatibility
            llm_config_dict = get_standard_llm_config(self.config)
            api_key = os.getenv("OPENAI_API_KEY") or llm_config_dict["api_key"]
            if not api_key:
                raise ValueError("API key not found in environment or config")

            # Default to OpenAI for backward compatibility
            self.llm_config = graphbit.LlmConfig.openai(api_key, llm_config_dict["model"])
        else:
            # Use new LLMConfig structure
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")

            if llm_config_obj.provider == LLMProvider.OPENAI:
                if not api_key:
                    raise ValueError("OpenAI API key not found in environment or config")
                self.llm_config = graphbit.LlmConfig.openai(api_key, llm_config_obj.model)

            elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
                anthropic_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
                if not anthropic_key:
                    raise ValueError("Anthropic API key not found in environment or config")
                self.llm_config = graphbit.LlmConfig.anthropic(anthropic_key, llm_config_obj.model)

            elif llm_config_obj.provider == LLMProvider.OLLAMA:
                # GraphBit LlmConfig.ollama() only takes model parameter
                self.llm_config = graphbit.LlmConfig.ollama(llm_config_obj.model)

            else:
                raise ValueError(f"Unsupported provider for GraphBit: {llm_config_obj.provider}")

        # Create LLM client using the correct API
        self.llm_client = graphbit.LlmClient(self.llm_config)

    async def teardown(self) -> None:
        """Cleanup GraphBit resources."""
        self.llm_config = None
        self.llm_client = None

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run simple task using direct API call."""
        if self.llm_client is None:
            raise RuntimeError("LLM client not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            max_tokens, temperature = self._get_llm_params()

            output_content = await self.llm_client.complete_async(prompt=SIMPLE_TASK_PROMPT, max_tokens=max_tokens, temperature=temperature)

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
        if self.llm_client is None:
            raise RuntimeError("LLM client not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            max_tokens, temperature = self._get_llm_params()
            total_tokens = 0
            previous_result = ""

            for i, task in enumerate(SEQUENTIAL_TASKS):
                if i == 0:
                    prompt = task
                else:
                    prompt = f"Previous result: {previous_result}\n\nNew task: {task}"

                # Single direct API call
                result = await self.llm_client.complete_stream(prompt=prompt, max_tokens=max_tokens, temperature=temperature)

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
        if self.llm_client is None:
            raise RuntimeError("LLM client not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            max_tokens, temperature = self._get_llm_params()

            try:
                results = await self.llm_client.complete_batch(prompts=PARALLEL_TASKS, max_tokens=max_tokens, temperature=temperature, max_concurrency=len(PARALLEL_TASKS))
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
        if self.llm_client is None:
            raise RuntimeError("LLM client not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            max_tokens, temperature = self._get_llm_params()
            total_tokens = 0
            workflow_results: Dict[str, str] = {}

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
                result = await self.llm_client.complete_async(prompt=full_prompt, max_tokens=max_tokens, temperature=temperature)

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
        if self.llm_client is None:
            raise RuntimeError("LLM client not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            max_tokens, temperature = self._get_llm_params()

            # Single direct API call
            output_content = await self.llm_client.complete_async(prompt=MEMORY_INTENSIVE_PROMPT, max_tokens=max_tokens, temperature=temperature)

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
        if self.llm_client is None:
            raise RuntimeError("LLM client not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            max_tokens, temperature = self._get_llm_params()

            # Try to use batch processing first for better performance
            try:
                results = await self.llm_client.complete_batch(prompts=CONCURRENT_TASK_PROMPTS, max_tokens=max_tokens, temperature=temperature, max_concurrency=20)
            except Exception as e:
                self.logger.error(f"Error in concurrent tasks benchmark: {e}")
                metrics = self.monitor.stop_monitoring()
                metrics.error_rate = 1.0
                metrics.token_count = 0
                return metrics

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
