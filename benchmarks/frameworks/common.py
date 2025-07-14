"""Common utilities and data structures for benchmarking frameworks."""

import gc
import logging
import os
import sys
import time
from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field
from enum import Enum
from functools import lru_cache
from pathlib import Path
from threading import Thread
from typing import Any, Dict, List, Optional, Tuple


def get_total_cpu_time(proc: Any) -> Tuple[float, float]:
    """Sum CPU time (user+system) for process and its children."""
    total_user = proc.cpu_times().user
    total_sys = proc.cpu_times().system
    for child in proc.children(recursive=True):
        try:
            ctimes = child.cpu_times()
            total_user += ctimes.user
            total_sys += ctimes.system
        except psutil.NoSuchProcess:
            pass
    return total_user, total_sys


def sample_total_memory(proc: Any, interval: float, running: List[bool], samples: List[int]) -> None:
    """Sample total RSS memory for process and its children."""
    while running[0]:
        total_rss = proc.memory_info().rss
        for child in proc.children(recursive=True):
            try:
                total_rss += child.memory_info().rss
            except psutil.NoSuchProcess:
                pass
        samples.append(total_rss)
        time.sleep(interval)


try:
    import psutil
except Exception as e:  # pragma: no cover - optional dependency
    psutil = None
    print(f"Warning: psutil not available ({e}). Benchmark metrics will be limited.")

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
    HUGGINGFACE = "huggingface"


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
    LLMProvider.ANTHROPIC: ["claude-3-5-sonnet-20241022", "claude-3-5-haiku-20241022", "claude-3-opus-20240229", "claude-3-sonnet-20240229", "claude-3-haiku-20240307"],
    LLMProvider.OLLAMA: ["llama3.2", "llama3.1", "codellama", "mistral", "phi3", "qwen2.5"],
    LLMProvider.HUGGINGFACE: ["microsoft/DialoGPT-medium", "microsoft/DialoGPT-large", "facebook/blenderbot-400M-distill", "google/flan-t5-large"],
}


class BenchmarkLogger:
    """Utility class for logging benchmark LLM outputs to files."""

    def __init__(self, framework_name: str, log_dir: str = "logs"):
        """Initialize the benchmark logger with framework name and log directory."""
        self.framework_name = framework_name
        self.log_dir = Path(log_dir)
        self.log_dir.mkdir(exist_ok=True)
        self.log_file = self.log_dir / f"{framework_name.lower()}.log"

        # Set up logger
        self.logger = logging.getLogger(f"benchmark.{framework_name}")
        self.logger.setLevel(logging.INFO)

        # Create file handler
        file_handler = logging.FileHandler(self.log_file)
        file_handler.setLevel(logging.INFO)

        # Create formatter
        formatter = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
        file_handler.setFormatter(formatter)

        # Add handler to logger
        self.logger.addHandler(file_handler)

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

    # LLM API time (seconds)
    llm_api_time_sec: float = 0.0

    # Detailed CPU and memory stats
    user_cpu_sec: float = 0.0
    sys_cpu_sec: float = 0.0
    avg_memory_mb: float = 0.0
    peak_memory_mb: float = 0.0
    mem_delta_mb: float = 0.0

    # Additional metadata
    metadata: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        """Return metrics as a serializable dictionary."""
        return asdict(self)


class PerformanceMonitor:
    """Utility class for monitoring performance metrics."""

    def __init__(self) -> None:
        """Initialize the performance monitor."""
        if psutil is None:
            raise RuntimeError("psutil is required for performance monitoring")

        self.process = psutil.Process(os.getpid())
        self.start_time: float = 0.0
        self.cpu_user_start: float = 0.0
        self.cpu_sys_start: float = 0.0
        self.mem_start: int = 0
        self.samples: List[int] = []
        self._running: List[bool] = [False]
        self._mem_thread: Optional[Thread] = None

    def start_monitoring(self) -> None:
        """Start performance monitoring."""
        gc.collect()  # Clean up before starting
        self.cpu_user_start, self.cpu_sys_start = get_total_cpu_time(self.process)
        self.mem_start = self.process.memory_info().rss + sum(c.memory_info().rss for c in self.process.children(recursive=True) if c.is_running())
        self.samples = []
        self._running[0] = True
        self._mem_thread = Thread(
            target=sample_total_memory,
            args=(self.process, 0.05, self._running, self.samples),
        )
        self._mem_thread.start()
        self.start_time = time.perf_counter()

    def stop_monitoring(self) -> BenchmarkMetrics:
        """Stop monitoring and return collected metrics."""
        end_time = time.perf_counter()
        self._running[0] = False
        if self._mem_thread is not None:
            self._mem_thread.join()

        user_cpu_end, sys_cpu_end = get_total_cpu_time(self.process)
        mem_end = self.process.memory_info().rss + sum(c.memory_info().rss for c in self.process.children(recursive=True) if c.is_running())

        exec_time_sec = end_time - self.start_time
        user_cpu = user_cpu_end - self.cpu_user_start
        sys_cpu = sys_cpu_end - self.cpu_sys_start
        mem_delta_mb = (mem_end - self.mem_start) / 1024 / 1024

        if self.samples:
            avg_mem_mb = sum(self.samples) / len(self.samples) / 1024 / 1024
            peak_mem_mb = max(self.samples) / 1024 / 1024
        else:
            avg_mem_mb = 0.0
            peak_mem_mb = 0.0

        cpu_usage_percent = ((user_cpu + sys_cpu) / exec_time_sec) * 100 if exec_time_sec > 0 else 0

        return BenchmarkMetrics(
            execution_time_ms=exec_time_sec * 1000,
            memory_usage_mb=avg_mem_mb,
            cpu_usage_percent=cpu_usage_percent,
            latency_ms=exec_time_sec * 1000,
            user_cpu_sec=user_cpu,
            sys_cpu_sec=sys_cpu,
            avg_memory_mb=avg_mem_mb,
            peak_memory_mb=peak_mem_mb,
            mem_delta_mb=mem_delta_mb,
        )


class BaseBenchmark(ABC):
    """Base class for framework-specific benchmarks."""

    def __init__(self, config: Dict[str, Any]):
        """Initialize the benchmark with configuration."""
        self.config = config
        self.api_key = config.get("api_key")
        self.model = config.get("model", "gpt-4o-mini")
        self.monitor = PerformanceMonitor()

        # Initialize logger
        framework_name = self.__class__.__name__.replace("Benchmark", "")
        log_dir = config.get("log_dir", "logs")
        self.logger = BenchmarkLogger(framework_name, log_dir)

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
        """Run a specific benchmark scenario."""
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
SIMPLE_TASK_PROMPT = (
    "Analyze the following text and provide a summary: "
    "'The quick brown fox jumps over the lazy dog. "
    "This sentence contains all letters of the English alphabet "
    "and is commonly used for typing practice.'"
)

SEQUENTIAL_TASKS = [
    "Generate a product description for a wireless headphone.",
    "Create a marketing slogan for the product described above.",
    "Write a customer review for this product.",
    "Summarize the key selling points based on the description and review.",
]

PARALLEL_TASKS = [
    "Translate 'Hello, world!' to Spanish.",
    "Calculate the square root of 144.",
    "Generate a haiku about technology.",
    "List three benefits of renewable energy.",
]

COMPLEX_WORKFLOW_STEPS: List[Dict[str, Any]] = [
    {
        "task": "content_analysis",
        "prompt": "Analyze this business problem: 'A startup needs to improve customer retention.'",
        "depends_on": [],
    },
    {
        "task": "solution_generation",
        "prompt": "Based on the analysis, generate 3 potential solutions for improving customer retention.",
        "depends_on": ["content_analysis"],
    },
    {
        "task": "cost_analysis",
        "prompt": "Estimate implementation costs for each solution.",
        "depends_on": ["solution_generation"],
    },
    {
        "task": "risk_assessment",
        "prompt": "Assess risks for each solution.",
        "depends_on": ["solution_generation"],
    },
    {
        "task": "recommendation",
        "prompt": "Provide a final recommendation considering costs and risks.",
        "depends_on": ["cost_analysis", "risk_assessment"],
    },
]

MEMORY_INTENSIVE_PROMPT = (
    """
Analyze the following large dataset and provide insights:

Data: """
    + "Sample data point, " * 1000
    + """

Please provide:
1. Key patterns in the data
2. Statistical summary
3. Recommendations based on the analysis
4. Potential data quality issues
5. Suggestions for further analysis
"""
)

CONCURRENT_TASK_PROMPTS = [
    "Summarize the benefits of cloud computing.",
    "Explain machine learning in simple terms.",
    "Describe the importance of cybersecurity.",
    "List advantages of remote work.",
    "Explain sustainable development goals.",
    "Describe artificial intelligence applications.",
    "Explain blockchain technology basics.",
    "Summarize digital transformation trends.",
    "Describe data science methodologies.",
    "Explain IoT and its applications.",
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
    if cores and psutil is not None:
        psutil.Process().cpu_affinity(cores)


def set_memory_binding(node: Optional[int]) -> None:
    """Bind process memory allocations to the given NUMA ``node`` if specified."""
    if node is None:
        return

    try:
        import ctypes

        libnuma = ctypes.CDLL("libnuma.so.1")
        libnuma.numa_available.restype = ctypes.c_int
        if libnuma.numa_available() < 0:
            raise RuntimeError("NUMA not available")

        libnuma.numa_allocate_nodemask.restype = ctypes.c_void_p
        mask = libnuma.numa_allocate_nodemask()
        libnuma.numa_bitmask_setbit.argtypes = [ctypes.c_void_p, ctypes.c_uint]
        libnuma.numa_bitmask_setbit(mask, ctypes.c_uint(node))
        libnuma.numa_set_membind.argtypes = [ctypes.c_void_p]
        libnuma.numa_set_membind(mask)
        libnuma.numa_bitmask_free.argtypes = [ctypes.c_void_p]
        libnuma.numa_bitmask_free(mask)
    except OSError as e:
        logging.warning("libnuma not available: %s", e)
        _reexec_with_numactl(node)
    except Exception as exc:  # pragma: no cover - optional fallback path
        logging.warning(
            "Failed to set memory binding via libnuma: %s. Falling back to numactl.",
            exc,
        )
        _reexec_with_numactl(node)


def _reexec_with_numactl(node: int) -> None:
    """Re-execute the current process under ``numactl`` for memory binding."""
    if os.environ.get("_NUMACTL_REEXEC") == "1":
        logging.error("numactl re-exec attempt already made and failed")
        return

    os.environ["_NUMACTL_REEXEC"] = "1"
    args = [
        "numactl",
        f"--membind={node}",
        "--localalloc",
        sys.executable,
        *sys.argv,
    ]
    logging.info("Re-executing under numactl: %s", " ".join(args))
    os.execvp("numactl", args)
