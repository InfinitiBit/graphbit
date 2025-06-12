"""LangGraph framework benchmark implementation."""

import asyncio
import os
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Union, cast

from typing_extensions import TypedDict

try:
    from langchain.schema import AIMessage, HumanMessage
    from langchain_openai import ChatOpenAI
    from langgraph.graph import END, StateGraph

    # ToolExecutor has been moved or deprecated, removing the import

    _LANGGRAPH_AVAILABLE = True

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

except ImportError:

    StateGraph = cast(Any, None)
    END = cast(Any, None)
    ChatOpenAI = cast(Any, None)
    HumanMessage = cast(Any, None)
    AIMessage = cast(Any, None)
    TypedDict = cast(Any, None)
    _LANGGRAPH_AVAILABLE = False


if TYPE_CHECKING:
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


class LangGraphBenchmark(BaseBenchmark):
    """LangGraph framework benchmark implementation."""

    def __init__(self, config: Dict[str, Any]):
        """Initialize LangGraph benchmark with configuration."""
        super().__init__(config)
        self.llm: Optional[Any] = None
        self.graphs: Dict[str, Any] = {}
        self._dependencies_available = StateGraph is not None

        # Don't raise error immediately - defer until actual usage
        if not self._dependencies_available:
            self._import_error_msg = "LangGraph not installed. Please install with: pip install langgraph"

    def _check_dependencies(self) -> None:
        """Check if dependencies are available and raise error if not."""
        if not self._dependencies_available:
            raise ImportError(self._import_error_msg)

    async def setup(self) -> None:
        """Set up LangGraph for benchmarking."""
        self._check_dependencies()

        # Get standardized configuration
        llm_config = get_standard_llm_config(self.config)
        api_key = os.getenv("OPENAI_API_KEY") or llm_config["api_key"]
        if not api_key:
            raise ValueError("OpenAI API key not found in environment or config")

        # Initialize LangGraph LLM
        from pydantic import SecretStr

        self.llm = ChatOpenAI(
            model=llm_config["model"],
            api_key=SecretStr(api_key),  # Convert to SecretStr
            temperature=llm_config["temperature"],
        )

        # Pre-create common graphs
        self._setup_graphs()

    def _setup_graphs(self) -> None:
        """Set up common LangGraph workflows."""
        # Simple task graph
        self._create_simple_graph()

        # Sequential pipeline graph
        self._create_sequential_graph()

        # Parallel pipeline graph
        self._create_parallel_graph()

        # Complex workflow graph
        self._create_complex_graph()

    def _create_simple_graph(self) -> None:
        """Create a simple task execution graph."""

        def execute_task(state: SimpleState) -> SimpleState:
            messages = state["messages"]
            task = state["task"]

            # Add task message
            messages.append(HumanMessage(content=task))

            # Get LLM response
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

            # Get current task
            task = step_data.get(current_step, "")

            # Build context from previous results
            context_parts = [f"{k}: {v}" for k, v in results.items()]
            context = " | ".join(context_parts) if context_parts else "None"

            # Create prompt
            prompt = f"Previous results: {context}\n\nNew task: {task}"
            messages.append(HumanMessage(content=prompt))

            # Get LLM response
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke(messages)
            messages.append(response)

            # Store result
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

        def parallel_task_1(state: WorkflowState) -> WorkflowState:
            return self._execute_parallel_task(state, 0)

        def parallel_task_2(state: WorkflowState) -> WorkflowState:
            return self._execute_parallel_task(state, 1)

        def parallel_task_3(state: WorkflowState) -> WorkflowState:
            return self._execute_parallel_task(state, 2)

        def parallel_task_4(state: WorkflowState) -> WorkflowState:
            return self._execute_parallel_task(state, 3)

        def collect_results(state: WorkflowState) -> WorkflowState:
            # Just pass through the state with all collected results
            return state

        graph = StateGraph(WorkflowState)
        graph.add_node("task1", parallel_task_1)
        graph.add_node("task2", parallel_task_2)
        graph.add_node("task3", parallel_task_3)
        graph.add_node("task4", parallel_task_4)
        graph.add_node("collect", collect_results)

        graph.set_entry_point("task1")
        graph.set_entry_point("task2")
        graph.set_entry_point("task3")
        graph.set_entry_point("task4")

        graph.add_edge("task1", "collect")
        graph.add_edge("task2", "collect")
        graph.add_edge("task3", "collect")
        graph.add_edge("task4", "collect")
        graph.add_edge("collect", END)

        self.graphs["parallel"] = graph.compile()

    def _execute_parallel_task(self, state: WorkflowState, task_index: int) -> WorkflowState:
        """Execute a single parallel task."""
        if task_index < len(PARALLEL_TASKS):
            task = PARALLEL_TASKS[task_index]

            # Create a new message list for this task
            messages = [HumanMessage(content=task)]
            if self.llm is None:
                raise ValueError("LLM not initialized")
            response = self.llm.invoke(messages)

            # Store result
            state["results"][f"task_{task_index}"] = response.content

        return state

    def _create_complex_graph(self) -> None:
        """Create a complex workflow graph with dependencies."""

        def content_analysis(state: WorkflowState) -> WorkflowState:
            return self._execute_workflow_step(state, "content_analysis")

        def solution_generation(state: WorkflowState) -> WorkflowState:
            return self._execute_workflow_step(state, "solution_generation")

        def cost_analysis(state: WorkflowState) -> WorkflowState:
            return self._execute_workflow_step(state, "cost_analysis")

        def risk_assessment(state: WorkflowState) -> WorkflowState:
            return self._execute_workflow_step(state, "risk_assessment")

        def recommendation(state: WorkflowState) -> WorkflowState:
            return self._execute_workflow_step(state, "recommendation")

        graph = StateGraph(WorkflowState)
        graph.add_node("content_analysis", content_analysis)
        graph.add_node("solution_generation", solution_generation)
        graph.add_node("cost_analysis", cost_analysis)
        graph.add_node("risk_assessment", risk_assessment)
        graph.add_node("recommendation", recommendation)

        graph.set_entry_point("content_analysis")
        graph.add_edge("content_analysis", "solution_generation")
        graph.add_edge("solution_generation", "cost_analysis")
        graph.add_edge("solution_generation", "risk_assessment")
        graph.add_edge("cost_analysis", "recommendation")
        graph.add_edge("risk_assessment", "recommendation")
        graph.add_edge("recommendation", END)

        self.graphs["complex"] = graph.compile()

    def _execute_workflow_step(self, state: WorkflowState, step_name: str) -> WorkflowState:
        """Execute a single workflow step."""
        # Find step definition
        step_def = None
        for step in COMPLEX_WORKFLOW_STEPS:
            if step["task"] == step_name:
                step_def = step
                break

        if not step_def:
            return state

        # Build context from dependencies
        context_parts = []
        for dep in step_def["depends_on"]:
            if dep in state["results"]:
                context_parts.append(f"{dep}: {state['results'][dep]}")

        context = " | ".join(context_parts) if context_parts else "None"

        # Create prompt
        prompt = f"Context: {context}\n\nTask: {step_def['prompt']}"

        # Execute
        messages = [HumanMessage(content=prompt)]
        if self.llm is None:
            raise ValueError("LLM not initialized")
        response = self.llm.invoke(messages)

        # Store result
        state["results"][step_name] = response.content

        return state

    async def teardown(self) -> None:
        """Cleanup LangGraph resources."""
        self.llm = None
        self.graphs = {}

    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run a simple single-task benchmark using LangGraph."""
        self.monitor.start_monitoring()

        try:
            graph = self.graphs["simple"]

            initial_state = {"messages": [], "task": SIMPLE_TASK_PROMPT, "result": None}

            # Execute graph
            result = await graph.ainvoke(initial_state)

            # Count tokens
            token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + str(result.get("result", "")))

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
        """Run a sequential pipeline benchmark using LangGraph."""
        self.monitor.start_monitoring()

        try:
            graph = self.graphs["sequential"]

            # Prepare step data
            step_data = {f"step_{i}": task for i, task in enumerate(SEQUENTIAL_TASKS)}

            initial_state = {
                "messages": [],
                "current_step": "step_0",
                "results": {},
                "context": "",
                "step_data": step_data,
            }

            # Execute graph
            result = await graph.ainvoke(initial_state)

            # Count tokens
            total_tokens = sum(count_tokens_estimate(task + str(result["results"].get(f"step_{i}", ""))) for i, task in enumerate(SEQUENTIAL_TASKS))

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
        """Run a parallel pipeline benchmark using LangGraph."""
        self.monitor.start_monitoring()

        try:
            graph = self.graphs["parallel"]

            initial_state = {
                "messages": [],
                "current_step": "",
                "results": {},
                "context": "",
                "step_data": {},
            }

            # Execute graph
            result = await graph.ainvoke(initial_state)

            # Count tokens
            total_tokens = sum(count_tokens_estimate(task + str(result["results"].get(f"task_{i}", ""))) for i, task in enumerate(PARALLEL_TASKS))

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
        """Run a complex workflow benchmark using LangGraph."""
        self.monitor.start_monitoring()

        try:
            graph = self.graphs["complex"]

            initial_state = {
                "messages": [],
                "current_step": "",
                "results": {},
                "context": "",
                "step_data": {},
            }

            # Execute graph
            result = await graph.ainvoke(initial_state)

            # Count tokens
            total_tokens = sum(count_tokens_estimate(step["prompt"] + str(result["results"].get(step["task"], ""))) for step in COMPLEX_WORKFLOW_STEPS)

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
        """Run a memory-intensive benchmark using LangGraph."""
        self.monitor.start_monitoring()

        try:
            graph = self.graphs["simple"]

            # Create large data structures to test memory usage
            large_data = ["data" * 1000] * 1000  # ~4MB of string data

            initial_state = {
                "messages": [],
                "task": MEMORY_INTENSIVE_PROMPT,
                "result": None,
            }

            # Execute graph
            result = await graph.ainvoke(initial_state)

            # Clean up
            del large_data

            token_count = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + str(result.get("result", "")))

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
        """Run concurrent tasks benchmark using LangGraph."""
        self.monitor.start_monitoring()

        try:
            graph = self.graphs["simple"]

            # Create tasks for concurrent execution
            tasks = []
            for prompt in CONCURRENT_TASK_PROMPTS:
                initial_state = {"messages": [], "task": prompt, "result": None}
                tasks.append(graph.ainvoke(initial_state))

            # Execute all tasks concurrently
            results = await asyncio.gather(*tasks)

            # Count tokens
            total_tokens = sum(count_tokens_estimate(prompt + str(result.get("result", ""))) for prompt, result in zip(CONCURRENT_TASK_PROMPTS, results))

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
