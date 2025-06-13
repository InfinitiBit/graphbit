"""LangGraph framework benchmark implementation."""

import asyncio
import os
from typing import Any, Dict, List, Optional, Union

from langchain.schema import HumanMessage
from langchain_openai import ChatOpenAI
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
    calculate_throughput,
    count_tokens_estimate,
    get_standard_llm_config,
)


class SimpleState(TypedDict):
    """State for simple task execution."""

    messages: List[Any]
    task: str
    result: Optional[str]


class WorkflowState(TypedDict):
    """State for complex workflow execution."""

    messages: List[Any]
    current_step: str
    results: Dict[str, str]
    context: str
    step_data: Dict[str, Any]


class LangGraphBenchmark(BaseBenchmark):
    """LangGraph framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any]):
        """Initialize LangGraph benchmark with configuration."""
        super().__init__(config)
        self.llm: Optional[ChatOpenAI] = None
        self.graphs: Dict[str, Any] = {}

    async def setup(self) -> None:
        """Set up LangGraph for benchmarking."""
        from pydantic import SecretStr

        llm_config = get_standard_llm_config(self.config)
        api_key = os.getenv("OPENAI_API_KEY") or llm_config["api_key"]
        if not api_key:
            raise ValueError("OpenAI API key not found in environment or config")

        self.llm = ChatOpenAI(
            model=llm_config["model"],
            api_key=SecretStr(api_key),
            temperature=llm_config["temperature"],
        )

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
            messages = state["messages"]
            current_step = state["current_step"]
            results = state["results"]
            step_data = state["step_data"]

            task = step_data.get(current_step, "")
            context_parts = [f"{k}: {v}" for k, v in results.items()]
            context = " | ".join(context_parts) if context_parts else "None"

            prompt = f"Previous results: {context}\n\nNew task: {task}"
            messages.append(HumanMessage(content=prompt))

            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke(messages)
            messages.append(response)
            results[current_step] = response.content

            return {
                "messages": messages,
                "current_step": current_step,
                "results": results,
                "context": context,
                "step_data": step_data,
            }

        def route_next_step(state: WorkflowState) -> Union[str, Any]:
            steps = list(state["step_data"].keys())
            current_step = state["current_step"]
            try:
                current_index = steps.index(current_step)
                if current_index + 1 < len(steps):
                    next_step = steps[current_index + 1]
                    state["current_step"] = next_step
                    return "process"
                else:
                    return END
            except ValueError:
                return END

        graph = StateGraph(WorkflowState)
        graph.add_node("process", process_step)
        graph.set_entry_point("process")
        graph.add_conditional_edges("process", route_next_step)

        self.graphs["sequential"] = graph.compile()

    def _create_parallel_graph(self) -> None:
        """Create a parallel pipeline graph."""

        def make_task_node(i: int):
            def run(state: WorkflowState) -> WorkflowState:
                return self._execute_parallel_task(state, i)

            return run

        graph = StateGraph(WorkflowState)

        for i in range(len(PARALLEL_TASKS)):
            node_name = f"task{i+1}"
            graph.add_node(node_name, make_task_node(i))
            graph.set_entry_point(node_name)
            graph.add_edge(node_name, "collect")

        def collect_results(state: WorkflowState) -> WorkflowState:
            return state

        graph.add_node("collect", collect_results)
        graph.add_edge("collect", END)

        self.graphs["parallel"] = graph.compile()

    def _create_complex_graph(self) -> None:
        """Create a complex workflow graph with dependencies."""

        def make_step_node(step_name: str):
            def run(state: WorkflowState) -> WorkflowState:
                return self._execute_workflow_step(state, step_name)

            return run

        graph = StateGraph(WorkflowState)

        for step in COMPLEX_WORKFLOW_STEPS:
            graph.add_node(step["task"], make_step_node(step["task"]))

        # Add edges based on dependency structure
        for step in COMPLEX_WORKFLOW_STEPS:
            for dep in step["depends_on"]:
                graph.add_edge(dep, step["task"])

        last_step = COMPLEX_WORKFLOW_STEPS[-1]["task"]
        graph.add_edge(last_step, END)

        # Set first task as entry point
        first_step = COMPLEX_WORKFLOW_STEPS[0]["task"]
        graph.set_entry_point(first_step)

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
                "messages": [],
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
                "messages": [],
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
                "messages": [],
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
            tasks = [graph.ainvoke({"messages": [], "task": prompt, "result": None}) for prompt in CONCURRENT_TASK_PROMPTS]
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
