"""Framework benchmarks package."""

__version__ = "0.1.0"

from .common import BenchmarkMetrics, BenchmarkScenario, FrameworkType

try:
    from .graphbit_benchmark import GraphBitBenchmark
except ImportError:
    GraphBitBenchmark = None  # type: ignore

try:
    from .langchain_benchmark import LangChainBenchmark
except ImportError:
    LangChainBenchmark = None  # type: ignore

try:
    from .pydantic_ai_benchmark import PydanticAIBenchmark
except ImportError:
    PydanticAIBenchmark = None  # type: ignore

try:
    from .llamaindex_benchmark import LlamaIndexBenchmark
except ImportError:
    LlamaIndexBenchmark = None  # type: ignore

try:
    from .crewai_benchmark import CrewAIBenchmark
except ImportError:
    CrewAIBenchmark = None  # type: ignore

try:
    from .langgraph_benchmark import LangGraphBenchmark
except ImportError:
    LangGraphBenchmark = None  # type: ignore


__all__ = [
    "BenchmarkMetrics",
    "BenchmarkScenario",
    "FrameworkType",
    "GraphBitBenchmark",
    "LangChainBenchmark",
    "LangGraphBenchmark",
    "PydanticAIBenchmark",
    "LlamaIndexBenchmark",
    "CrewAIBenchmark",
]
