"""Framework benchmarks package."""

__version__ = "0.1.0"

from .common import BenchmarkMetrics, BenchmarkScenario, FrameworkType

# Specific benchmark classes are imported lazily by ``run_benchmark.py`` to
# avoid hard dependencies when the package is imported simply to inspect CLI
# options. This keeps ``benchmarks.run_benchmark`` usable even if optional
# frameworks are not installed.

__getattr__ = None  # placate static analyzers

__all__ = [
    "BenchmarkMetrics",
    "BenchmarkScenario",
    "FrameworkType",
]

