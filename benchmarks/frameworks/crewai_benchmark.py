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
        """Run a simple single-task benchmark using CrewAI."""
        self.monitor.start_monitoring()

        try:
            task = Task(
                description=SIMPLE_TASK_PROMPT,
                agent=self.agents["general"],
                expected_output="A comprehensive analysis and summary",
            )
            crew = Crew(agents=[self.agents["general"]], tasks=[task], verbose=False)
            result = await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)

            self.log_output(
                scenario_name=BenchmarkScenario.SIMPLE_TASK.value,
                task_name="Simple Task",
                output=str(result),
            )

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

            async def execute_task(task_desc: str, agent_key: str) -> str:
                agent = self.agents[agent_key]
                task = Task(
                    description=task_desc,
                    agent=agent,
                    expected_output="Clear and concise response",
                )
                crew = Crew(agents=[agent], tasks=[task], verbose=False)
                return await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)

            agent_keys = ["general", "technical", "creator", "analyst"]
            tasks = [execute_task(task_desc, agent_keys[i % len(agent_keys)]) for i, task_desc in enumerate(PARALLEL_TASKS)]

            results = await asyncio.gather(*tasks)

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
        """Run a complex workflow benchmark using CrewAI."""
        self.monitor.start_monitoring()

        try:
            results: Dict[str, str] = {}
            total_tokens = 0

            for step in COMPLEX_WORKFLOW_STEPS:
                if "analysis" in step["task"]:
                    agent = self.agents["analyst"]
                elif "solution" in step["task"] or "recommendation" in step["task"]:
                    agent = self.agents["technical"]
                elif "cost" in step["task"] or "risk" in step["task"]:
                    agent = self.agents["analyst"]
                else:
                    agent = self.agents["general"]

                context_parts = [f"{dep}: {results[dep]}" for dep in step["depends_on"] if dep in results]
                context = " | ".join(context_parts) if context_parts else "None"
                full_prompt = f"Context: {context}\n\nTask: {step['prompt']}"

                task = Task(
                    description=full_prompt,
                    agent=agent,
                    expected_output="Comprehensive analysis and recommendations",
                )
                crew = Crew(agents=[agent], tasks=[task], verbose=False)
                result = await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)
                result_str = str(result)
                results[step["task"]] = result_str

                self.log_output(
                    scenario_name=BenchmarkScenario.COMPLEX_WORKFLOW.value,
                    task_name=step["task"],
                    output=result_str,
                )

                # Count tokens based on original step prompt only for fair comparison
                total_tokens += count_tokens_estimate(step["prompt"] + result_str)

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
        """Run a memory-intensive benchmark using CrewAI."""
        self.monitor.start_monitoring()

        try:
            large_data = ["data" * 1000] * 1000  # ~4MB of string data

            task = Task(
                description=MEMORY_INTENSIVE_PROMPT,
                agent=self.agents["analyst"],
                expected_output="Detailed data analysis with insights and recommendations",
            )
            crew = Crew(agents=[self.agents["analyst"]], tasks=[task], verbose=False)
            result = await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)
            del large_data

            self.log_output(
                scenario_name=BenchmarkScenario.MEMORY_INTENSIVE.value,
                task_name="Memory Intensive",
                output=str(result),
            )

            token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + str(result))

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
        """Run concurrent tasks benchmark using CrewAI."""
        self.monitor.start_monitoring()

        try:

            async def execute_concurrent_task(task_desc: str, agent_key: str) -> str:
                agent = self.agents[agent_key]
                task = Task(
                    description=task_desc,
                    agent=agent,
                    expected_output="Clear and informative response",
                )
                crew = Crew(agents=[agent], tasks=[task], verbose=False)
                return await asyncio.get_event_loop().run_in_executor(None, crew.kickoff)

            agent_keys = ["general", "technical", "creator", "analyst"]
            tasks = [execute_concurrent_task(task_desc, agent_keys[i % len(agent_keys)]) for i, task_desc in enumerate(CONCURRENT_TASK_PROMPTS)]
            results = await asyncio.gather(*tasks)

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
