#!/usr/bin/env python3

"""GraphBit framework benchmark implementation.

FAIR COMPARISON VERSION: Uses GraphBit Workflow and Agent APIs to match
the abstraction level of other frameworks being benchmarked (LangChain, CrewAI, etc.).
"""

import asyncio
import os
import uuid
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
    get_cpu_affinity_or_count_fallback,
    get_standard_llm_config,
)


class GraphBitBenchmark(BaseBenchmark):
    """GraphBit benchmark using Workflow and Agent APIs for fair comparison.
    
    This implementation uses GraphBit's high-level Workflow, Node, and Executor
    APIs to match the abstraction level of other frameworks like LangChain,
    CrewAI, and LangGraph. This ensures an apples-to-apples comparison.
    """

    def __init__(self, config: Dict, num_runs: Optional[int] = None):
        """Initialize GraphBit benchmark with configuration."""
        super().__init__(config, num_runs=num_runs)
        self.llm_config: Optional[Any] = None
        self.executor: Optional[Any] = None
        # Lazy loaded classes
        self._Workflow: Optional[Any] = None
        self._Node: Optional[Any] = None
        self._Executor: Optional[Any] = None
        self._LlmConfig: Optional[Any] = None

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if llm_config_obj:
            return llm_config_obj.max_tokens, llm_config_obj.temperature
        else:
            llm_config = get_standard_llm_config(self.config)
            return llm_config["max_tokens"], llm_config["temperature"]

    async def setup(self) -> None:
        """Set up GraphBit using Workflow and Executor APIs."""
        # Lazy import GraphBit dependencies
        from graphbit import Executor, LlmConfig, Node, Workflow, configure_runtime, init
        
        self._Workflow = Workflow
        self._Node = Node
        self._Executor = Executor
        self._LlmConfig = LlmConfig
        
        # Configure runtime with appropriate worker threads
        configure_runtime(worker_threads=get_cpu_affinity_or_count_fallback())
        init(debug=False)

        # Get LLM configuration from config
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            # Fallback to old format for backward compatibility
            llm_config_dict = get_standard_llm_config(self.config)
            api_key = os.getenv("OPENAI_API_KEY") or llm_config_dict["api_key"]
            if not api_key:
                raise ValueError("API key not found in environment or config")
            self.llm_config = LlmConfig.openai(api_key, llm_config_dict["model"])
        else:
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")

            if llm_config_obj.provider == LLMProvider.OPENAI:
                if not api_key:
                    raise ValueError("OpenAI API key not found in environment or config")
                self.llm_config = LlmConfig.openai(api_key, llm_config_obj.model)

            elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
                anthropic_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
                if not anthropic_key:
                    raise ValueError("Anthropic API key not found in environment or config")
                self.llm_config = LlmConfig.anthropic(anthropic_key, llm_config_obj.model)

            elif llm_config_obj.provider == LLMProvider.OLLAMA:
                self.llm_config = LlmConfig.ollama(llm_config_obj.model)

            elif llm_config_obj.provider == LLMProvider.AZURE_OPENAI:
                api_key = llm_config_obj.api_key or os.getenv("AZURE_OPENAI_API_KEY")
                azure_endpoint = llm_config_obj.base_url or os.getenv("AZURE_OPENAI_ENDPOINT")
                api_version = llm_config_obj.api_version or os.getenv("AZURE_OPENAI_API_VERSION", "2024-02-15-preview")
                
                if not api_key:
                    raise ValueError("Azure OpenAI API key not found in environment or config")
                if not azure_endpoint:
                    raise ValueError("Azure OpenAI endpoint not found in environment or config")

                # GraphBit may have azure_openai() method, otherwise use custom config
                try:
                    # Try GraphBit's built-in Azure support if available
                    # Based on signature: (api_key, deployment_name, endpoint, api_version=None)
                    self.llm_config = LlmConfig.azure_openai(
                        api_key=api_key,
                        deployment_name=llm_config_obj.model,  # Correct argument name
                        endpoint=azure_endpoint,
                        api_version=api_version,
                    )
                except AttributeError:
                    # Fallback: Use OpenAI config with Azure parameters
                    # This may need adjustment based on GraphBit's actual Azure support
                    raise ValueError(
                        "GraphBit Azure OpenAI support not yet implemented. "
                        "Please check GraphBit documentation for Azure configuration."
                    )

            else:
                raise ValueError(f"Unsupported provider for GraphBit: {llm_config_obj.provider}")

        # Create Executor with timeout for workflow execution
        max_tokens, _ = self._get_llm_params()
        timeout = max(120, max_tokens // 10)  # Dynamic timeout based on token limit
        self.executor = Executor(self.llm_config, timeout_seconds=timeout)

    async def teardown(self) -> None:
        """Cleanup GraphBit resources."""
        self.llm_config = None
        self.executor = None

    def _extract_workflow_output(self, result: Any, fallback: str = "Task completed") -> str:
        """Extract output from workflow result."""
        try:
            variables = result.variables()
            if variables:
                for _key, value in variables:
                    value_str = str(value).strip()
                    if value_str and value_str.lower() not in ["null", "none", '""', ""]:
                        # Return the first meaningful value
                        if len(value_str) > 10:
                            return value_str
            return fallback
        except Exception:
            return fallback

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run simple task using GraphBit Workflow API."""
        if self.executor is None:
            raise RuntimeError("Executor not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            # Create workflow with single agent node
            workflow = self._Workflow("Simple Task Benchmark")
            agent_id = str(uuid.uuid4())
            node = self._Node.agent(
                name="simple_task",
                prompt=SIMPLE_TASK_PROMPT,
                agent_id=agent_id
            )
            workflow.add_node(node)
            workflow.validate()

            # Execute workflow synchronously (wrapping in async context)
            result = await asyncio.get_event_loop().run_in_executor(
                None, self.executor.execute, workflow
            )

            if result.is_failed():
                raise RuntimeError(f"Workflow failed: {result.state()}")

            output_content = self._extract_workflow_output(result, "Simple task completed")

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
        """Run sequential pipeline using GraphBit Workflow with connected nodes."""
        if self.executor is None:
            raise RuntimeError("Executor not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            # Create workflow with sequential nodes
            workflow = self._Workflow("Sequential Pipeline Benchmark")
            agent_id = str(uuid.uuid4())
            node_ids: List[Any] = []

            # Create nodes for each task
            for i, task in enumerate(SEQUENTIAL_TASKS):
                # For sequential tasks, include context from previous in prompt
                if i == 0:
                    prompt = task
                else:
                    prompt = f"Building on the previous analysis, now: {task}"
                
                node = self._Node.agent(
                    name=f"sequential_task_{i}",
                    prompt=prompt,
                    agent_id=str(uuid.uuid4())
                )
                node_id = workflow.add_node(node)
                node_ids.append(node_id)

            # Connect nodes sequentially
            for i in range(len(node_ids) - 1):
                workflow.connect(node_ids[i], node_ids[i + 1])

            workflow.validate()

            # Execute workflow
            result = await asyncio.get_event_loop().run_in_executor(
                None, self.executor.execute, workflow
            )

            if result.is_failed():
                raise RuntimeError(f"Workflow failed: {result.state()}")

            # Extract results from workflow variables
            total_tokens = 0
            variables = result.variables() if result else []
            
            for i, task in enumerate(SEQUENTIAL_TASKS):
                output = ""
                if i < len(variables):
                    _, value = variables[i]
                    output = str(value) if value else ""
                
                if not output:
                    output = f"Sequential task {i+1} completed"

                self.log_output(
                    scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                    task_name=f"Sequential Task {i+1}",
                    output=output[:500],
                )
                total_tokens += count_tokens_estimate(task + output)

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
        """Run parallel pipeline using GraphBit Workflow with independent nodes."""
        if self.executor is None:
            raise RuntimeError("Executor not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            # Create workflow with parallel (independent) nodes
            workflow = self._Workflow("Parallel Pipeline Benchmark")
            agent_id = str(uuid.uuid4())

            # Create independent nodes for each task (no connections = parallel)
            for i, task in enumerate(PARALLEL_TASKS):
                node = self._Node.agent(
                    name=f"parallel_task_{i}",
                    prompt=task,
                    agent_id=str(uuid.uuid4())
                )
                workflow.add_node(node)

            workflow.validate()

            # Execute workflow - independent nodes run in parallel
            result = await asyncio.get_event_loop().run_in_executor(
                None, self.executor.execute, workflow
            )

            if result.is_failed():
                raise RuntimeError(f"Workflow failed: {result.state()}")

            # Extract results
            total_tokens = 0
            variables = result.variables() if result else []
            
            for i, task in enumerate(PARALLEL_TASKS):
                output = ""
                if i < len(variables):
                    _, value = variables[i]
                    output = str(value) if value else ""
                
                if not output:
                    output = f"Parallel task {i+1} completed"

                self.log_output(
                    scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                    task_name=f"Parallel Task {i+1}",
                    output=output[:500],
                )
                total_tokens += count_tokens_estimate(task + output)

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
        """Run complex workflow with dependencies using GraphBit Workflow API."""
        if self.executor is None:
            raise RuntimeError("Executor not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            # Create workflow with dependency graph
            workflow = self._Workflow("Complex Workflow Benchmark")
            node_ids: Dict[str, Any] = {}

            # Create nodes for independent steps
            for step in COMPLEX_WORKFLOW_STEPS:
                agent_key = "analyst" if "analysis" in step["task"] else "technical"
                # Use unique agent ID for each node, even if conceptually same 'role'
                # Pass context manually as established by common.COMPLEX_WORKFLOW_STEPS logic
                node = self._Node.agent(
                    name=step["task"],  # Sanitize name if needed
                    prompt=step["prompt"],
                    agent_id=str(uuid.uuid4())
                )
                node_id = workflow.add_node(node)
                node_ids[step["task"]] = node_id

            # Connect nodes based on dependencies
            for step in COMPLEX_WORKFLOW_STEPS:
                for dep in step.get("depends_on", []):
                    if dep in node_ids:
                        workflow.connect(node_ids[dep], node_ids[step["task"]])

            workflow.validate()

            # Execute workflow
            result = await asyncio.get_event_loop().run_in_executor(
                None, self.executor.execute, workflow
            )

            if result.is_failed():
                raise RuntimeError(f"Workflow failed: {result.state()}")

            # Extract results
            total_tokens = 0
            variables = result.variables() if result else []
            
            for i, step in enumerate(COMPLEX_WORKFLOW_STEPS):
                output = ""
                if i < len(variables):
                    _, value = variables[i]
                    output = str(value) if value else ""
                
                if not output:
                    output = f"{step['task']} completed"

                self.log_output(
                    scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    task_name=step["task"],
                    output=output[:500],
                )
                total_tokens += count_tokens_estimate(step["prompt"] + output)

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
        """Run memory-intensive test using GraphBit Workflow."""
        if self.executor is None:
            raise RuntimeError("Executor not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            # Create workflow with single memory-intensive task
            workflow = self._Workflow("Memory Intensive Benchmark")
            agent_id = str(uuid.uuid4())
            node = self._Node.agent(
                name="memory_intensive",
                prompt=MEMORY_INTENSIVE_PROMPT,
                agent_id=agent_id
            )
            workflow.add_node(node)
            workflow.validate()

            # Simulate memory pressure with large data
            _large_data = ["data" * 1000] * 1000  # ~4MB

            # Execute workflow
            result = await asyncio.get_event_loop().run_in_executor(
                None, self.executor.execute, workflow
            )

            if result.is_failed():
                raise RuntimeError(f"Workflow failed: {result.state()}")

            output_content = self._extract_workflow_output(result, "Memory task completed")

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
        """Run concurrent tasks using GraphBit Workflow with parallel nodes."""
        if self.executor is None:
            raise RuntimeError("Executor not initialized. Call setup() first.")

        self.monitor.start_monitoring()

        try:
            # Create workflow with concurrent (independent) nodes
            workflow = self._Workflow("Concurrent Tasks Benchmark")
            agent_id = str(uuid.uuid4())

            # Create independent nodes for each task
            for i, prompt in enumerate(CONCURRENT_TASK_PROMPTS):
                node = self._Node.agent(
                    name=f"concurrent_task_{i}",
                    prompt=prompt,
                    agent_id=str(uuid.uuid4())
                )
                workflow.add_node(node)

            workflow.validate()

            # Execute workflow - independent nodes can run concurrently
            result = await asyncio.get_event_loop().run_in_executor(
                None, self.executor.execute, workflow
            )

            if result.is_failed():
                raise RuntimeError(f"Workflow failed: {result.state()}")

            # Extract results
            total_tokens = 0
            variables = result.variables() if result else []
            
            for i, prompt in enumerate(CONCURRENT_TASK_PROMPTS):
                output = ""
                if i < len(variables):
                    _, value = variables[i]
                    output = str(value) if value else ""
                
                if not output:
                    output = f"Concurrent task {i+1} completed"

                self.log_output(
                    scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                    task_name=f"Concurrent Task {i+1}",
                    output=output[:500],
                )
                total_tokens += count_tokens_estimate(prompt + output)

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
