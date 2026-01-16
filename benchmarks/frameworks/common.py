"""Common utilities and data structures for benchmarking frameworks."""

# GAIA Benchmark Questions
# Source: Curated from GAIA validation set (public examples)
# Paper: https://arxiv.org/abs/2311.12983
# Dataset: https://huggingface.co/datasets/gaia-benchmark/GAIA
#
# Questions are text-only (no tool/file requirements) and mapped to scenarios:
# - Simple Task: Level 1 factual question
# - Sequential: Level 1-2 multi-step questions
# - Parallel: Level 1 independent questions
# - Complex: Level 2-3 questions with dependencies
# - Memory: Level 2 long-context question
# - Concurrent: Level 1 high-volume questions
#

import gc
import logging
import os
import platform
import sys
import time
import tracemalloc
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum
from functools import lru_cache
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

import psutil

# Configure logging
# Standard LLM Configuration Constants
DEFAULT_TEMPERATURE = 0.1
DEFAULT_MAX_TOKENS = 2000
DEFAULT_MODEL = "gpt-4o-mini"


class LLMProvider(Enum):
    """Supported LLM providers for benchmarking."""

    OPENAI = "openai"
    ANTHROPIC = "anthropic"
    OLLAMA = "ollama"


@dataclass
class LLMConfig:
    """Configuration for LLM providers."""

    provider: LLMProvider
    model: str
    temperature: float = DEFAULT_TEMPERATURE
    max_tokens: int = DEFAULT_MAX_TOKENS
    api_key: Optional[str] = None
    base_url: Optional[str] = None  # For custom endpoints like Ollama
    api_version: Optional[str] = None  # For Azure OpenAI

    # Provider-specific configurations
    extra_params: Dict[str, Any] = field(default_factory=dict)


# Provider-specific model presets
PROVIDER_MODELS = {
    LLMProvider.OPENAI: ["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo", "o1-preview", "o1-mini"],
    LLMProvider.ANTHROPIC: ["claude-3-5-haiku-20241022", "claude-3-opus-20240229", "claude-3-sonnet-20240229", "claude-3-haiku-20240307"],
    LLMProvider.OLLAMA: ["llama3.2", "llama3.1", "codellama", "mistral", "phi3", "qwen2.5"],
}


class BenchmarkLogger:
    """Utility class for logging benchmark LLM outputs to files."""

    def __init__(self, framework_name: str, log_dir: str = "logs", num_runs: Optional[int] = None):
        """Initialize the benchmark logger with framework name and log directory."""
        self.framework_name = framework_name
        self.log_dir = Path(log_dir)
        self.log_dir.mkdir(exist_ok=True)
        self.log_file = self.log_dir / f"{framework_name.lower()}.log"

        # Set up logger
        self.logger = logging.getLogger(f"benchmark.{framework_name}")
        self.logger.setLevel(logging.INFO)

        # Create file handler
        file_handler = logging.FileHandler(self.log_file, mode="w")
        file_handler.setLevel(logging.INFO)

        # Create formatter
        formatter = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
        file_handler.setFormatter(formatter)

        # Add handler to logger
        self.logger.addHandler(file_handler)

        # Write header if averaging is used
        if num_runs is not None and num_runs > 1:
            with open(self.log_file, "r+") as f:
                content = f.read()
                f.seek(0, 0)
                f.write(f"# Results in this file are averaged over {num_runs} runs per scenario\n" + content)

    def log_llm_output(self, scenario_name: str, task_name: str, llm_output: str) -> None:
        """Log LLM output to the framework-specific log file."""
        self.logger.info(f"Scenario: {scenario_name}")
        self.logger.info(f"Task: {task_name}")
        self.logger.info(f"Output:\n{llm_output}\n{'-' * 80}")

    def error(self, message: str) -> None:
        """Log error messages."""
        self.logger.error(message)


class FrameworkType(Enum):
    """Supported frameworks for benchmarking."""

    GRAPHBIT = "GraphBit"
    LANGCHAIN = "LangChain"
    LANGGRAPH = "LangGraph"
    PYDANTIC_AI = "PydanticAI"
    LLAMAINDEX = "LlamaIndex"
    CREWAI = "CrewAI"


class BenchmarkScenario(Enum):
    """Different benchmark scenarios to test."""

    SIMPLE_TASK = "simple_task"
    SEQUENTIAL_PIPELINE = "sequential_pipeline"
    PARALLEL_PIPELINE = "parallel_pipeline"
    COMPLEX_WORKFLOW = "complex_workflow"
    MEMORY_INTENSIVE = "memory_intensive"
    CONCURRENT_TASKS = "concurrent_tasks"


@dataclass
class BenchmarkMetrics:
    """Performance metrics collected during benchmarking."""

    execution_time_ms: float = 0.0
    memory_usage_mb: float = 0.0
    cpu_usage_percent: float = 0.0
    token_count: int = 0
    latency_ms: float = 0.0
    throughput_tasks_per_sec: float = 0.0
    error_rate: float = 0.0
    concurrent_tasks: int = 0
    setup_time_ms: float = 0.0
    teardown_time_ms: float = 0.0

    # Additional metadata
    metadata: Dict[str, Any] = field(default_factory=dict)


class PerformanceMonitor:
    """Utility class for monitoring performance metrics."""

    def __init__(self) -> None:
        """Initialize the performance monitor."""
        self.process = psutil.Process()
        self.start_time: float = 0.0
        self.start_memory: float = 0.0
        self.start_cpu_times = self.process.cpu_times()  # type: ignore
        self.initial_memory_trace: int = 0

    def start_monitoring(self) -> None:
        """Start performance monitoring."""
        gc.collect()  # Clean up before starting
        tracemalloc.start()

        self.start_time = time.perf_counter()
        self.start_memory = self.process.memory_info().rss / 1024 / 1024  # MB
        self.start_cpu_times = self.process.cpu_times()
        self.initial_memory_trace = tracemalloc.get_traced_memory()[0]

    def stop_monitoring(self) -> BenchmarkMetrics:
        """Stop monitoring and return collected metrics."""
        end_time = time.perf_counter()
        end_memory = self.process.memory_info().rss / 1024 / 1024  # MB
        end_cpu_times = self.process.cpu_times()

        current_memory, peak_memory = tracemalloc.get_traced_memory()
        tracemalloc.stop()

        # Calculate metrics
        execution_time_ms = (end_time - self.start_time) * 1000
        memory_usage_mb = end_memory - self.start_memory

        # Calculate CPU usage (approximate)
        cpu_time_used = (end_cpu_times.user - self.start_cpu_times.user) + (end_cpu_times.system - self.start_cpu_times.system)
        wall_time = end_time - self.start_time
        cpu_usage_percent = (cpu_time_used / wall_time) * 100 if wall_time > 0 else 0

        return BenchmarkMetrics(
            execution_time_ms=execution_time_ms,
            memory_usage_mb=memory_usage_mb,
            cpu_usage_percent=cpu_usage_percent,
            latency_ms=execution_time_ms,  # Same as execution time for single tasks
            metadata={
                "peak_memory_mb": peak_memory / 1024 / 1024,
                "memory_delta_mb": (current_memory - self.initial_memory_trace) / 1024 / 1024,
            },
        )


class BaseBenchmark(ABC):
    """Base class for framework-specific benchmarks."""

    def __init__(self, config: Dict[str, Any], num_runs: Optional[int] = None):
        """Initialize the benchmark with configuration."""
        self.config = config
        self.api_key = config.get("api_key")
        self.model = config.get("model", "gpt-4o-mini")
        self.monitor = PerformanceMonitor()

        # Initialize logger
        framework_name = self.__class__.__name__.replace("Benchmark", "")
        log_dir = config.get("log_dir", "logs")
        self.logger = BenchmarkLogger(framework_name, log_dir, num_runs=num_runs if num_runs is not None else 10)

    def log_output(self, scenario_name: str, task_name: str, output: str) -> None:
        """Log LLM output to framework-specific log file."""
        self.logger.log_llm_output(scenario_name, task_name, output)

    @abstractmethod
    async def setup(self) -> None:
        """Set up the framework for benchmarking."""
        pass

    @abstractmethod
    async def teardown(self) -> None:
        """Cleanup after benchmarking."""
        pass

    @abstractmethod
    async def run_simple_task(self) -> BenchmarkMetrics:
        """Run a simple single-task benchmark."""
        pass

    @abstractmethod
    async def run_sequential_pipeline(self) -> BenchmarkMetrics:
        """Run a sequential pipeline benchmark."""
        pass

    @abstractmethod
    async def run_parallel_pipeline(self) -> BenchmarkMetrics:
        """Run a parallel pipeline benchmark."""
        pass

    @abstractmethod
    async def run_complex_workflow(self) -> BenchmarkMetrics:
        """Run a complex workflow benchmark."""
        pass

    @abstractmethod
    async def run_memory_intensive(self) -> BenchmarkMetrics:
        """Run a memory-intensive benchmark."""
        pass

    @abstractmethod
    async def run_concurrent_tasks(self) -> BenchmarkMetrics:
        """Run concurrent tasks benchmark."""
        pass

    async def run_scenario(self, scenario: BenchmarkScenario) -> BenchmarkMetrics:
        """Run a specific benchmark scenario.
        
        NOTE: execution_time_ms includes setup() overhead for fair total cost comparison.
        setup_time_ms is also stored separately for analysis purposes.
        """
        setup_start = time.perf_counter()
        await self.setup()
        setup_time = (time.perf_counter() - setup_start) * 1000

        metrics = None
        try:
            if scenario == BenchmarkScenario.SIMPLE_TASK:
                metrics = await self.run_simple_task()
            elif scenario == BenchmarkScenario.SEQUENTIAL_PIPELINE:
                metrics = await self.run_sequential_pipeline()
            elif scenario == BenchmarkScenario.PARALLEL_PIPELINE:
                metrics = await self.run_parallel_pipeline()
            elif scenario == BenchmarkScenario.COMPLEX_WORKFLOW:
                metrics = await self.run_complex_workflow()
            elif scenario == BenchmarkScenario.MEMORY_INTENSIVE:
                metrics = await self.run_memory_intensive()
            elif scenario == BenchmarkScenario.CONCURRENT_TASKS:
                metrics = await self.run_concurrent_tasks()
            else:
                raise ValueError(f"Unknown scenario: {scenario}")

            if metrics is not None:
                # Include setup time in total execution time for fair comparison
                # execution_time_ms now represents TOTAL scenario cost (setup + execution)
                metrics.execution_time_ms += setup_time
                # Keep setup_time_ms separately for metadata/debugging purposes
                metrics.setup_time_ms = setup_time

        except Exception as e:
            # Create failure metrics if scenario fails
            metrics = BenchmarkMetrics()
            metrics.error_rate = 1.0
            metrics.setup_time_ms = setup_time
            # Re-raise the exception so the caller can handle it
            raise e

        finally:
            teardown_start = time.perf_counter()
            await self.teardown()
            teardown_time = (time.perf_counter() - teardown_start) * 1000
            if metrics is not None:
                metrics.teardown_time_ms = teardown_time

        return metrics


# Common test scenarios data
# GAIA Benchmark Questions - Level 1 (Simple factual question)
SIMPLE_TASK_PROMPT = "What was the actual enrollment count of the clinical trial on H. pylori in acne vulgaris patients from Jan-May 2018 as listed on the NIH website?"

SEQUENTIAL_TASKS = [
    "In the 2020 Summer Olympics, which country won the most gold medals in swimming events?",
    "If a pint of ice cream contains 473 grams total weight and has 18g of fat per 1/2 cup serving (with 2 cups per pint), what is the fat percentage by weight? According to US federal standards (21 CFR 135.110), ice cream must contain at least 10% milkfat. Calculate how many percentage points above or below this standard the ice cream is, rounded to one decimal place. Answer as +X.X or -X.X",
    "A study published in 2019 found that the median age of Nobel Prize winners in Physics was 55 years at the time of award between 2000-2018. If the youngest winner in that period was 35 and the oldest was 96, and exactly 19 prizes were awarded (one per year), what would be the average age rounded to the nearest year if we assume a normal distribution?",
]

# GAIA Benchmark - Level 1 (Independent questions)
PARALLEL_TASKS = [
    "What is the total number of Prime Ministers that the United Kingdom has had from 1945 to 2020?",
    "According to the periodic table, what is the atomic number of the element with symbol 'Fe'?",
    "In computer science, what does the acronym 'LIFO' stand for?",
    "What year did the Soviet Union officially dissolve?"
]

# GAIA Benchmark - Level 2-3 (Complex reasoning with dependencies)
COMPLEX_WORKFLOW_STEPS: List[Dict[str, Any]] = [
    {
        "task": "step_1",
        "prompt": "According to a 2020 report, the world's proven oil reserves were approximately 1.73 trillion barrels. If global oil consumption in 2020 was about 88.4 million barrels per day, how many years would these reserves last at that consumption rate? Round to the nearest whole number.",
        "depends_on": [],
    },
    {
        "task": "step_2",
        "prompt": "The Fibonacci sequence starts with 0, 1, and each subsequent number is the sum of the previous two. What is the 15th number in the Fibonacci sequence?",
        "depends_on": ["step_1"],
    },
    {
        "task": "step_3",
        "prompt": "If a rectangle has a length that is 3 times its width, and its perimeter is 48 units, what is the area of the rectangle in square units?",
        "depends_on": ["step_1"],
    },
    {
        "task": "step_4",
        "prompt": "A researcher is analyzing citation patterns. Paper A was published in 2015 and has been cited 150 times. Paper B was published in 2018 and has been cited 120 times. If citation rates are assumed to be linear over time, and both papers continue to be cited at their current annual rates, in what year would Paper B's total citations equal Paper A's total citations? Assume current year is 2024.",
        "depends_on": ["step_2", "step_3"],
    },
    {
        "task": "step_5",
        "prompt": "What is the chemical formula for table salt?",
        "depends_on": ["step_4"],
    },
]

# GAIA Benchmark - Level 2 (Long context question)
MEMORY_INTENSIVE_PROMPT = "If a pint of ice cream contains 473 grams total weight and has 18g of fat per 1/2 cup serving (with 2 cups per pint), what is the fat percentage by weight? According to US federal standards (21 CFR 135.110), ice cream must contain at least 10% milkfat. Calculate how many percentage points above or below this standard the ice cream is, rounded to one decimal place. Answer as +X.X or -X.X"

# GAIA Benchmark - Level 1 (High-volume independent questions)
CONCURRENT_TASK_PROMPTS = [
    "How many stars are on the flag of the European Union?",
    "What is the chemical formula for table salt?",
    "In which year did the first iPhone launch?",
    "What is the capital city of Australia?",
    "What was the actual enrollment count of the clinical trial on H. pylori in acne vulgaris patients from Jan-May 2018 as listed on the NIH website?",
    "In the 2020 Summer Olympics, which country won the most gold medals in swimming events?",
    "What is the total number of Prime Ministers that the United Kingdom has had from 1945 to 2020?",
    "According to the periodic table, what is the atomic number of the element with symbol 'Fe'?",
]


def count_tokens_estimate(text: str) -> int:
    """Estimate token count for a text string."""
    # Rough approximation: ~4 characters per token
    return len(text) // 4


async def measure_latency(coro: Any) -> Tuple[Any, float]:
    """Measure latency of an async operation."""
    start = time.perf_counter()
    result = await coro
    latency = (time.perf_counter() - start) * 1000
    return result, latency


def calculate_throughput(task_count: int, total_time_seconds: float) -> float:
    """Calculate throughput in tasks per second."""
    if total_time_seconds <= 0:
        return 0.0
    return task_count / total_time_seconds


def handle_benchmark_errors(monitor_instance: Any) -> Any:
    """Provide standardized error handling in benchmark methods."""

    def decorator(func: Any) -> Any:
        async def wrapper(self: Any, *args: Any, **kwargs: Any) -> Any:
            try:
                return await func(self, *args, **kwargs)
            except Exception as e:
                # Stop monitoring and create error metrics
                metrics = monitor_instance.stop_monitoring() if hasattr(monitor_instance, "stop_monitoring") else self.monitor.stop_monitoring()
                metrics.error_rate = 1.0
                metrics.token_count = 0
                # Log the error
                print(f"Error in {func.__name__}: {str(e)}")
                return metrics

        return wrapper

    return decorator


# Common configuration helper
def get_standard_llm_config(config: Dict[str, Any]) -> Dict[str, Any]:
    """Get standardized LLM configuration with defaults."""
    return {
        "model": config.get("model", DEFAULT_MODEL),
        "temperature": config.get("temperature", DEFAULT_TEMPERATURE),
        "max_tokens": config.get("max_tokens", DEFAULT_MAX_TOKENS),
        "api_key": config.get("api_key"),
    }


def create_llm_config_from_args(provider: str, model: str, temperature: float, max_tokens: int, api_key: Optional[str] = None, base_url: Optional[str] = None, **kwargs: Any) -> LLMConfig:
    """Create LLMConfig from command line arguments."""
    try:
        provider_enum = LLMProvider(provider.lower())
    except ValueError:
        raise ValueError(f"Unsupported provider: {provider}. Supported: {[p.value for p in LLMProvider]}")

    return LLMConfig(provider=provider_enum, model=model, temperature=temperature, max_tokens=max_tokens, api_key=api_key, base_url=base_url, extra_params=kwargs)


def get_provider_models(provider: str) -> List[str]:
    """Get available models for a provider."""
    try:
        provider_enum = LLMProvider(provider.lower())
        return PROVIDER_MODELS.get(provider_enum, [])
    except ValueError:
        return []


@lru_cache(maxsize=None)
def select_least_busy_cores(count: int) -> List[int]:
    """Return ``count`` least busy CPU cores using a short sampling interval."""
    if psutil is None:
        # Fallback: just return the first ``count`` cores
        return list(range(count))

    usage = psutil.cpu_percent(interval=0.1, percpu=True)
    indices = sorted(range(len(usage)), key=lambda i: usage[i])
    return indices[:count]


def parse_core_list(spec: Optional[str]) -> Optional[List[int]]:
    """Parse a core list specification.

    ``spec`` may be ``None``, a comma separated list like ``"0,1"``,
    or ``"auto:N"`` to automatically select ``N`` least busy cores.
    """
    if spec is None:
        return None

    spec = spec.strip()
    if not spec:
        return None

    if spec.startswith("auto:"):
        try:
            count = int(spec.split(":", 1)[1])
        except ValueError as exc:
            raise ValueError("Invalid core specification") from exc
        return select_least_busy_cores(count)

    try:
        return [int(part) for part in spec.split(",") if part.strip()]
    except ValueError as exc:
        raise ValueError("Invalid core specification") from exc


def set_process_affinity(cores: Optional[List[int]]) -> None:
    """Apply CPU affinity to the current process if ``cores`` provided."""
    if cores is not None:
        psutil.Process().cpu_affinity(cores)


def get_cpu_affinity_or_count_fallback() -> int:
    """Get the number of CPU cores available or the current CPU affinity."""
    if platform.system() in ("Linux", "Windows"):
        try:
            return len(psutil.Process().cpu_affinity())
        except Exception:
            return os.cpu_count() or 4
    return os.cpu_count() or 4

