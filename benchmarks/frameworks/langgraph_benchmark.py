"""LangGraph framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, List, Optional

from langchain.schema import HumanMessage
from langgraph.graph import END, StateGraph
from typing_extensions import TypedDict

from .common import (
    COMPLEX_WORKFLOW_STEPS,
    CONCURRENT_TASK_PROMPTS,
    MEMORY_INTENSIVE_PROMPT,
    PARALLEL_TASKS,
    SEQUENTIAL_TASKS,
    SIMPLE_TASK_PROMPT,
    BaseBenchmark,
    BenchmarkMetrics,
    LLMConfig,
    LLMProvider,
    calculate_throughput,
    count_tokens_estimate,
)


class SimpleState(TypedDict):
    """State for simple task execution."""

    messages: List[Any]
    task: str
    result: Optional[str]


class WorkflowState(TypedDict):
    """State for complex workflow execution."""

    current_step: str
    results: Dict[str, str]
    context: str
    step_data: Dict[str, Any]


class LangGraphBenchmark(BaseBenchmark):
    """LangGraph framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any], num_runs: Optional[int] = None):
        """Initialize LangGraph benchmark with configuration."""
        super().__init__(config, num_runs=num_runs)
        self.llm: Optional[Any] = None
        self.graphs: Dict[str, Any] = {}

    def _get_llm_params(self) -> tuple[int, float]:
        """Get max_tokens and temperature from configuration."""
        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")
        return llm_config_obj.max_tokens, llm_config_obj.temperature

    async def setup(self) -> None:
        """Set up LangGraph for benchmarking."""
        from pydantic import SecretStr

        llm_config_obj: LLMConfig | None = self.config.get("llm_config")
        if not llm_config_obj:
            raise ValueError("LLMConfig not found in configuration")

        max_tokens, temperature = self._get_llm_params()

        if llm_config_obj.provider == LLMProvider.OPENAI:
            api_key = llm_config_obj.api_key or os.getenv("OPENAI_API_KEY")
            if not api_key:
                raise ValueError("OpenAI API key not found in environment or config")

            from langchain_openai import ChatOpenAI

            self.llm = ChatOpenAI(
                model=llm_config_obj.model,
                api_key=SecretStr(api_key),
                temperature=temperature,
            )

        elif llm_config_obj.provider == LLMProvider.ANTHROPIC:
            api_key = llm_config_obj.api_key or os.getenv("ANTHROPIC_API_KEY")
            if not api_key:
                raise ValueError("Anthropic API key not found in environment or config")

            from langchain_anthropic import ChatAnthropic

            self.llm = ChatAnthropic(
                model=llm_config_obj.model,
                api_key=SecretStr(api_key),
                temperature=temperature,
                max_tokens=max_tokens,
            )

        elif llm_config_obj.provider == LLMProvider.OLLAMA:
            base_url = llm_config_obj.base_url or "http://localhost:11434"

            from langchain_ollama import ChatOllama

            self.llm = ChatOllama(
                model=llm_config_obj.model,
                base_url=base_url,
                temperature=temperature,
                num_predict=max_tokens,
            )

        else:
            raise ValueError(f"Unsupported provider for LangGraph: {llm_config_obj.provider}")

        self._setup_graphs()

    def _setup_graphs(self) -> None:
        """Set up common LangGraph workflows."""
        self._create_simple_graph()
        self._create_sequential_graph()
        self._create_parallel_graph()
        self._create_complex_graph()

    def _create_simple_graph(self) -> None:
        """Create a simple task execution graph."""

        def execute_task(state: SimpleState) -> SimpleState:
            messages = state["messages"]
            task = state["task"]
            messages.append(HumanMessage(content=task))

            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke(messages)
            messages.append(response)

            return {"messages": messages, "task": task, "result": response.content}

        graph = StateGraph(SimpleState)
        graph.add_node("execute", execute_task)
        graph.set_entry_point("execute")
        graph.add_edge("execute", END)

        self.graphs["simple"] = graph.compile()

    def _create_sequential_graph(self) -> None:
        """Create a sequential pipeline graph."""

        def process_step(state: WorkflowState) -> WorkflowState:
            current_step = state["current_step"]
            results = state["results"].copy()  # Make a copy to avoid mutation
            step_data = state["step_data"]

            task = step_data.get(current_step, "")
            context_parts = [f"{k}: {v}" for k, v in results.items()]
            context = " | ".join(context_parts) if context_parts else "None"

            prompt = f"Previous results: {context}\n\nNew task: {task}"

            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=prompt)])
            results[current_step] = response.content

            return {
                "current_step": current_step,
                "results": results,
                "context": context,
                "step_data": step_data,
            }

        def route_next_step(state: WorkflowState) -> WorkflowState:
            steps = list(state["step_data"].keys())
            current_step = state["current_step"]
            try:
                current_index = steps.index(current_step)
                if current_index + 1 < len(steps):
                    next_step = steps[current_index + 1]
                    return {
                        "current_step": next_step,
                        "results": state["results"],
                        "context": state["context"],
                        "step_data": state["step_data"],
                    }
                else:
                    return state
            except ValueError:
                return state

        graph = StateGraph(WorkflowState)
        graph.add_node("process", process_step)
        graph.add_node("route", route_next_step)
        graph.set_entry_point("process")
        graph.add_edge("process", "route")

        def should_continue(state: WorkflowState) -> str:
            steps = list(state["step_data"].keys())
            current_step = state["current_step"]
            try:
                current_index = steps.index(current_step)
                if current_index + 1 < len(steps):
                    return "process"
                else:
                    return END
            except ValueError:
                return END

        graph.add_conditional_edges("route", should_continue)

        self.graphs["sequential"] = graph.compile()

    def _create_parallel_graph(self) -> None:
        """Create a parallel pipeline graph."""

        def task1_node(state: WorkflowState) -> WorkflowState:
            task = PARALLEL_TASKS[0]
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=task)])
            state["results"]["task_0"] = response.content
            return state

        def task2_node(state: WorkflowState) -> WorkflowState:
            task = PARALLEL_TASKS[1]
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=task)])
            state["results"]["task_1"] = response.content
            return state

        def task3_node(state: WorkflowState) -> WorkflowState:
            task = PARALLEL_TASKS[2]
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=task)])
            state["results"]["task_2"] = response.content
            return state

        def task4_node(state: WorkflowState) -> WorkflowState:
            task = PARALLEL_TASKS[3]
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=task)])
            state["results"]["task_3"] = response.content
            return state

        def collect_results(state: WorkflowState) -> WorkflowState:
            return state

        graph = StateGraph(WorkflowState)
        graph.add_node("task1", task1_node)
        graph.add_node("task2", task2_node)
        graph.add_node("task3", task3_node)
        graph.add_node("task4", task4_node)
        graph.add_node("collect", collect_results)

        def start_parallel(state: WorkflowState) -> WorkflowState:
            return state

        graph.add_node("start", start_parallel)
        graph.set_entry_point("start")

        graph.add_edge("start", "task1")
        graph.add_edge("start", "task2")
        graph.add_edge("start", "task3")
        graph.add_edge("start", "task4")

        graph.add_edge("task1", "collect")
        graph.add_edge("task2", "collect")
        graph.add_edge("task3", "collect")
        graph.add_edge("task4", "collect")
        graph.add_edge("collect", END)

        self.graphs["parallel"] = graph.compile()

    def _create_complex_graph(self) -> None:
        """Create a complex workflow graph with dependencies."""

        def content_analysis_node(state: WorkflowState) -> WorkflowState:
            step = COMPLEX_WORKFLOW_STEPS[0]
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=step["prompt"])])
            state["results"]["content_analysis"] = response.content
            return state

        def solution_generation_node(state: WorkflowState) -> WorkflowState:
            step = COMPLEX_WORKFLOW_STEPS[1]
            context = state["results"].get("content_analysis", "")
            prompt = f"Context: {context}\n\nTask: {step['prompt']}"
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=prompt)])
            state["results"]["solution_generation"] = response.content
            return state

        def cost_analysis_node(state: WorkflowState) -> WorkflowState:
            step = COMPLEX_WORKFLOW_STEPS[2]
            context = state["results"].get("solution_generation", "")
            prompt = f"Context: {context}\n\nTask: {step['prompt']}"
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=prompt)])
            state["results"]["cost_analysis"] = response.content
            return state

        def risk_assessment_node(state: WorkflowState) -> WorkflowState:
            step = COMPLEX_WORKFLOW_STEPS[3]
            context = state["results"].get("solution_generation", "")
            prompt = f"Context: {context}\n\nTask: {step['prompt']}"
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=prompt)])
            state["results"]["risk_assessment"] = response.content
            return state

        def recommendation_node(state: WorkflowState) -> WorkflowState:
            step = COMPLEX_WORKFLOW_STEPS[4]
            cost_context = state["results"].get("cost_analysis", "")
            risk_context = state["results"].get("risk_assessment", "")
            prompt = f"Cost Analysis: {cost_context}\n\nRisk Assessment: {risk_context}\n\nTask: {step['prompt']}"
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke([HumanMessage(content=prompt)])
            state["results"]["recommendation"] = response.content
            return state

        graph = StateGraph(WorkflowState)
        graph.add_node("content_analysis", content_analysis_node)
        graph.add_node("solution_generation", solution_generation_node)
        graph.add_node("cost_analysis", cost_analysis_node)
        graph.add_node("risk_assessment", risk_assessment_node)
        graph.add_node("recommendation", recommendation_node)

        graph.set_entry_point("content_analysis")
        graph.add_edge("content_analysis", "solution_generation")
        graph.add_edge("solution_generation", "cost_analysis")
        graph.add_edge("solution_generation", "risk_assessment")
        graph.add_edge("cost_analysis", "recommendation")
        graph.add_edge("risk_assessment", "recommendation")
        graph.add_edge("recommendation", END)

        self.graphs["complex"] = graph.compile()

    def _execute_parallel_task(self, state: WorkflowState, task_index: int) -> WorkflowState:
        """Execute a single parallel task."""
        if task_index < len(PARALLEL_TASKS):
            task = PARALLEL_TASKS[task_index]
            messages = [HumanMessage(content=task)]

            if self.llm is None:
                raise ValueError("LLM not initialized")

            response = self.llm.invoke(messages)
            state["results"][f"task_{task_index}"] = response.content

        return state

    def _execute_workflow_step(self, state: WorkflowState, step_name: str) -> WorkflowState:
        """Execute a single workflow step."""
        step_def = next((step for step in COMPLEX_WORKFLOW_STEPS if step["task"] == step_name), None)
        if not step_def:
            return state

        context_parts = [f"{dep}: {state['results'][dep]}" for dep in step_def["depends_on"] if dep in state["results"]]
        context = " | ".join(context_parts) if context_parts else "None"
        prompt = f"Context: {context}\n\nTask: {step_def['prompt']}"

        if self.llm is None:
            raise ValueError("LLM not initialized")

        messages = [HumanMessage(content=prompt)]
        response = self.llm.invoke(messages)
        state["results"][step_name] = response.content

        return state

    async def teardown(self) -> None:
        """Cleanup LangGraph resources."""
        self.llm = None
        self.graphs = {}

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run the simple task benchmark."""
        self.monitor.start_monitoring()
        try:
            graph = self.graphs["simple"]
            initial_state: SimpleState = {"messages": [], "task": SIMPLE_TASK_PROMPT, "result": None}
            result: SimpleState = await graph.ainvoke(initial_state)
            token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + (result["result"] or ""))
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
        """Run the sequential pipeline benchmark."""
        self.monitor.start_monitoring()
        try:
            graph = self.graphs["sequential"]
            step_data = {f"step_{i}": task for i, task in enumerate(SEQUENTIAL_TASKS)}
            state: WorkflowState = {
                "current_step": "step_0",
                "results": {},
                "context": "",
                "step_data": step_data,
            }
            result: WorkflowState = await graph.ainvoke(state)
            total_tokens = sum(count_tokens_estimate(task + result["results"].get(f"step_{i}", "")) for i, task in enumerate(SEQUENTIAL_TASKS))
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
        """Run the parallel pipeline benchmark."""
        self.monitor.start_monitoring()
        try:
            graph = self.graphs["parallel"]
            state: WorkflowState = {
                "current_step": "",
                "results": {},
                "context": "",
                "step_data": {},
            }
            result: WorkflowState = await graph.ainvoke(state)
            total_tokens = sum(count_tokens_estimate(task + result["results"].get(f"task_{i}", "")) for i, task in enumerate(PARALLEL_TASKS))
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
        """Run the complex workflow benchmark."""
        self.monitor.start_monitoring()
        try:
            graph = self.graphs["complex"]
            state: WorkflowState = {
                "current_step": "",
                "results": {},
                "context": "",
                "step_data": {},
            }
            result: WorkflowState = await graph.ainvoke(state)
            total_tokens = sum(count_tokens_estimate(step["prompt"] + result["results"].get(step["task"], "")) for step in COMPLEX_WORKFLOW_STEPS)
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
        """Run the memory-intensive benchmark."""
        self.monitor.start_monitoring()
        try:
            graph = self.graphs["simple"]
            _ = ["data" * 1000] * 1000
            state: SimpleState = {"messages": [], "task": MEMORY_INTENSIVE_PROMPT, "result": None}
            result: SimpleState = await graph.ainvoke(state)
            token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + (result["result"] or ""))
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
        """Run the concurrent tasks benchmark."""
        self.monitor.start_monitoring()
        try:
            graph = self.graphs["simple"]
            concurrency: int = int(self.config.get("concurrency", len(CONCURRENT_TASK_PROMPTS)))
            sem = asyncio.Semaphore(concurrency)

            async def run_with_sem(p: str) -> SimpleState:
                async with sem:
                    return await graph.ainvoke({"messages": [], "task": p, "result": None})

            tasks = [run_with_sem(prompt) for prompt in CONCURRENT_TASK_PROMPTS]
            results: List[SimpleState] = await asyncio.gather(*tasks)
            total_tokens = sum(count_tokens_estimate(prompt + (res["result"] or "")) for prompt, res in zip(CONCURRENT_TASK_PROMPTS, results))
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
