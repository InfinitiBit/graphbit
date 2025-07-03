"""CrewAI framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, List, Optional

from crewai import LLM, Agent, Crew, Task

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


class CrewAIBenchmark(BaseBenchmark):
    """CrewAI framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any]):
        """Initialize CrewAI benchmark with configuration."""
        super().__init__(config)
        self.llm: Optional[LLM] = None
        self.agents: Dict[str, Agent] = {}
        self.crews: Dict[str, Crew] = {}

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")
        return llm_config_obj.max_tokens, llm_config_obj.temperature

    async def setup(self) -> None:
        """Set up CrewAI for benchmarking."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")

        max_tokens, temperature = self._get_llm_params()

        if llm_config_obj.provider == LLMProvider.OPENAI:
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")
            if not api_key:
                raise ValueError("OpenAI API key not found in environment or config")

            self.llm = LLM(
                model=f"openai/{llm_config_obj.model}",
                api_key=api_key,
                temperature=temperature,
                max_tokens=max_tokens,
            )

        elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
            api_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
            if not api_key:
                raise ValueError("Anthropic API key not found in environment or config")

            self.llm = LLM(
                model=f"anthropic/{llm_config_obj.model}",
                api_key=api_key,
                temperature=temperature,
                max_tokens=max_tokens,
            )

        elif llm_config_obj.provider == LLMProvider.OLLAMA:
            base_url = llm_config_obj.base_url or "http://localhost:11434"

            self.llm = LLM(
                model=f"ollama/{llm_config_obj.model}",
                base_url=base_url,
                temperature=temperature,
                max_tokens=max_tokens,
            )

        else:
            raise ValueError(f"Unsupported provider for CrewAI: {llm_config_obj.provider}")

        self._setup_agents()

    def _setup_agents(self) -> None:
        """Set up common CrewAI agents."""
        assert self.llm is not None, "LLM must be initialized before setting up agents."

        def create_agent(role: str, goal: str, backstory: str) -> Agent:
            return Agent(
                role=role,
                goal=goal,
                backstory=backstory,
                verbose=False,
                allow_delegation=False,
                llm=self.llm,
            )

        self.agents["general"] = create_agent(
            "General Assistant",
            "Complete assigned tasks efficiently and accurately",
            "You are an experienced assistant capable of handling various tasks.",
        )

        self.agents["analyst"] = create_agent(
            "Data Analyst",
            "Analyze data and provide insights",
            "You are a skilled data analyst with expertise in pattern recognition.",
        )

        self.agents["creator"] = create_agent(
            "Content Creator",
            "Create engaging and informative content",
            "You are a creative writer with expertise in marketing and communication.",
        )

        self.agents["technical"] = create_agent(
            "Technical Expert",
            "Provide technical solutions and explanations",
            "You are a technical expert with deep knowledge across various domains.",
        )

    async def teardown(self) -> None:
        """Cleanup CrewAI resources."""
        self.llm = None
        self.agents.clear()
        self.crews.clear()

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run a simple task benchmark using CrewAI with retry mechanism."""
        self.monitor.start_monitoring()

        max_retries = 3  # Max retries for each task
        retry_delay = 2  # Delay between retries in seconds

        try:
            task = Task(
                description=SIMPLE_TASK_PROMPT,
                agent=self.agents["general"],
                expected_output="A comprehensive analysis and summary",
            )
            crew = Crew(agents=[self.agents["general"]], tasks=[task], verbose=False)

            # Retry mechanism
            for attempt in range(max_retries):
                try:
                    result = await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)
                    break  # If the task succeeds, exit the retry loop
                except Exception as e:
                    if attempt < max_retries - 1:  # Retry if we haven't reached the max attempts
                        self.logger.error(f"Error in simple task on attempt {attempt + 1}: {e}. Retrying...")
                        await asyncio.sleep(retry_delay)  # Retry delay
                    else:
                        self.logger.error(f"Error in simple task after {max_retries} attempts: {e}")
                        raise e  # If max retries are reached, raise the error

            # Log the output
            self.log_output(
                scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
                task_name="Simple Task",
                output=str(result),
            )

            # Estimate tokens based on task and result
            token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + str(result))

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
        """Run a sequential pipeline benchmark using CrewAI."""
        self.monitor.start_monitoring()

        try:
            total_tokens = 0
            results: List[str] = []

            for i, task_desc in enumerate(SEQUENTIAL_TASKS):
                agent = self.agents["creator"] if "product" in task_desc or "marketing" in task_desc else self.agents["analyst"]

                # For fair token comparison, use original task description
                # but still pass context for functional accuracy
                context = f"Previous results: {' | '.join(results)}\n\n" if results else ""
                full_prompt = f"{context}Task {i+1}: {task_desc}"

                task = Task(
                    description=full_prompt,
                    agent=agent,
                    expected_output="Detailed response addressing the task requirements",
                )
                crew = Crew(agents=[agent], tasks=[task], verbose=False)
                result = await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)
                result_str = str(result)
                results.append(result_str)

                self.log_output(
                    scenario_name=BenchmarkScenario.SEQUENTIAL_PIPELINE.value,
                    task_name=f"Task {i+1}",
                    output=result_str,
                )
                # Count tokens based on original task prompt only for fair comparison
                total_tokens += count_tokens_estimate(task_desc + result_str)

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
        """Run a parallel pipeline benchmark using CrewAI."""
        self.monitor.start_monitoring()

        try:
            # Optimize task execution using asyncio.gather to run tasks concurrently
            async def execute_task(task_desc: str, agent_key: str) -> str:
                agent = self.agents[agent_key]
                task = Task(
                    description=task_desc,
                    agent=agent,
                    expected_output="Clear and concise response",
                )
                crew = Crew(agents=[agent], tasks=[task], verbose=False)
                return await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)

            # Create tasks for execution
            agent_keys = ["general", "technical", "creator", "analyst"]
            concurrency: int = int(self.config.get("concurrency", len(PARALLEL_TASKS)))
            sem = asyncio.Semaphore(concurrency)

            async def run_with_sem(task_desc: str, agent_key: str) -> Any:
                async with sem:
                    return await execute_task(task_desc, agent_key)

            tasks = [run_with_sem(task_desc, agent_keys[i % len(agent_keys)]) for i, task_desc in enumerate(PARALLEL_TASKS)]

            # Run all tasks with concurrency limit
            results = await asyncio.gather(*tasks)

            # Log the output and calculate metrics
            for i, (_task_desc, result) in enumerate(zip(PARALLEL_TASKS, results)):
                self.log_output(
                    scenario_name=BenchmarkScenario.PARALLEL_PIPELINE.value,
                    task_name=f"Parallel Task {i+1}",
                    output=str(result),
                )

            total_tokens = sum(count_tokens_estimate(task_desc + str(result)) for task_desc, result in zip(PARALLEL_TASKS, results))

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
        """Run a complex workflow benchmark using CrewAI with optimized execution."""
        self.monitor.start_monitoring()

        try:
            results: Dict[str, str] = {}
            total_tokens = 0

            # Identify independent steps and run them concurrently
            async def execute_step(step: Dict[str, Any]) -> str:
                context_parts = [f"{dep}: {results[dep]}" for dep in step["depends_on"] if dep in results]
                context = " | ".join(context_parts) if context_parts else "None"
                full_prompt = f"Context: {context}\n\nTask: {step['prompt']}"

                agent_key = "analyst" if "analysis" in step["task"] else "technical"
                task = Task(
                    description=full_prompt,
                    agent=self.agents[agent_key],
                    expected_output="Comprehensive analysis and recommendations",
                )
                crew = Crew(agents=[self.agents[agent_key]], tasks=[task], verbose=False)

                # Execute task and ensure proper result handling
                result = await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)

                # Log the type of result to understand its structure
                # self.logger.info(f"Task result type: {type(result)}")

                # If result is a custom object (e.g., CrewOutput), handle it
                if isinstance(result, str):
                    return result
                else:
                    # Attempt to convert the result to string or access specific properties
                    try:
                        result_str = str(result)  # Attempt to convert to string
                        return result_str
                    except Exception as e:
                        # Handle the case where result cannot be directly converted to string
                        self.logger.error(f"Failed to convert result to string: {e}")
                        raise e

            # Identify dependent and independent steps
            steps_to_execute = []
            for step in COMPLEX_WORKFLOW_STEPS:
                # If the step has no dependencies or all dependencies are already in results, we can run it in parallel
                if not step["depends_on"] or all(dep in results for dep in step["depends_on"]):
                    steps_to_execute.append(execute_step(step))
                else:
                    # For steps that are dependent, execute sequentially
                    result = await execute_step(step)
                    results[step["task"]] = result
                    total_tokens += count_tokens_estimate(step["prompt"] + result)

            # Execute all steps concurrently when dependencies are satisfied
            step_results = await asyncio.gather(*steps_to_execute)

            # Add results to the workflow
            for i, result in enumerate(step_results):
                step = COMPLEX_WORKFLOW_STEPS[i]
                results[step["task"]] = result
                total_tokens += count_tokens_estimate(step["prompt"] + result)

            # Log the output and calculate metrics
            for task_name, result in results.items():
                self.log_output(
                    scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    task_name=task_name,
                    output=str(result),  # Ensure the result is a string before logging
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
        """Run a memory-intensive benchmark with optimized cleanup."""
        self.monitor.start_monitoring()

        try:
            # Simulate memory-intensive task
            large_data = ["data" * 1000] * 1000  # ~4MB of string data

            task = Task(
                description=MEMORY_INTENSIVE_PROMPT,
                agent=self.agents["analyst"],
                expected_output="Detailed data analysis with insights and recommendations",
            )
            crew = Crew(agents=[self.agents["analyst"]], tasks=[task], verbose=False)

            # Run the task asynchronously
            result = await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)

            # Efficient cleanup: delete large data as soon as it's no longer needed
            del large_data
            await asyncio.sleep(0)  # Yield control to allow cleanup to happen

            # Log the output
            self.log_output(
                scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
                task_name="Memory Intensive",
                output=str(result),
            )

            # Estimate tokens
            token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + str(result))

            # Cleanup resources asynchronously to reduce memory consumption
            await self.async_cleanup_resources()

        except Exception as e:
            self.logger.error(f"Error in memory-intensive benchmark: {e}")
            metrics = self.monitor.stop_monitoring()
            metrics.error_rate = 1.0
            metrics.token_count = 0
            return metrics

        metrics = self.monitor.stop_monitoring()
        metrics.token_count = token_count
        metrics.throughput_tasks_per_sec = calculate_throughput(1, metrics.execution_time_ms / 1000)
        return metrics

    async def async_cleanup_resources(self) -> None:
        """Asynchronously clean up any additional resources (e.g., files, data)."""
        # Example cleanup: freeing up additional resources, deleting temporary files, etc.
        await asyncio.sleep(1)  # Simulate async cleanup

    async def run_concurrent_tasks(self) -> BenchmarkMetrics:
        """Run concurrent tasks benchmark using CrewAI with retry mechanism."""
        self.monitor.start_monitoring()

        max_retries = 3  # Max retries for each task
        retry_delay = 2  # Delay between retries in seconds

        try:
            # Optimize task execution using asyncio.gather to run tasks concurrently
            async def execute_concurrent_task(task_desc: str, agent_key: str) -> str:
                agent = self.agents[agent_key]
                task = Task(
                    description=task_desc,
                    agent=agent,
                    expected_output="Clear and informative response",
                )
                crew = Crew(agents=[agent], tasks=[task], verbose=False)

                # Retry mechanism
                for attempt in range(max_retries):
                    try:
                        return await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)
                    except Exception as e:
                        if attempt < max_retries - 1:  # Retry if we haven't reached the max attempts
                            self.logger.error(f"Error in task '{task_desc}' on attempt {attempt + 1}: {e}. Retrying...")
                            await asyncio.sleep(retry_delay)  # Exponential backoff could be added here
                        else:
                            self.logger.error(f"Error in task '{task_desc}' after {max_retries} attempts: {e}")
                            raise e  # If max retries are reached, raise the error

                raise RuntimeError(f"Unexpected end of retry loop for task: {task_desc}")

            # Create tasks for execution
            agent_keys = ["general", "technical", "creator", "analyst"]
            concurrency: int = int(self.config.get("concurrency", len(CONCURRENT_TASK_PROMPTS)))
            sem = asyncio.Semaphore(concurrency)

            async def run_with_sem(task_desc: str, agent_key: str) -> Any:
                async with sem:
                    return await execute_concurrent_task(task_desc, agent_key)

            tasks = [run_with_sem(task_desc, agent_keys[i % len(agent_keys)]) for i, task_desc in enumerate(CONCURRENT_TASK_PROMPTS)]

            # Run all tasks in parallel with retries and concurrency limit
            results = await asyncio.gather(*tasks)

            # Log the output and calculate metrics
            for i, (_task_desc, result) in enumerate(zip(CONCURRENT_TASK_PROMPTS, results)):
                self.log_output(
                    scenario_name=BenchmarkScenario.CONCURRENT_TASKS.value,
                    task_name=f"Concurrent Task {i+1}",
                    output=str(result),
                )

            total_tokens = sum(count_tokens_estimate(task_desc + str(result)) for task_desc, result in zip(CONCURRENT_TASK_PROMPTS, results))

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
