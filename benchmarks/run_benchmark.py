#!/usr/bin/env python3

"""
Comprehensive Framework Benchmark Runner.

This script runs benchmarks for GraphBit, LangChain, and PydanticAI frameworks,
measuring execution speed, CPU usage, memory usage, and throughput.
Each framework's LLM outputs are logged to individual .log files.
"""

import asyncio
import json
import os
import sys
import time
import traceback
from dataclasses import asdict
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, Optional, Tuple

import click
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

# API key will be checked later when actually running benchmarks

# Add the benchmarks directory to Python path
benchmarks_path = Path(__file__).parent / "frameworks"
sys.path.insert(0, str(benchmarks_path))

try:
    from frameworks.common import (
        DEFAULT_MAX_TOKENS,
        DEFAULT_MODEL,
        DEFAULT_TEMPERATURE,
        BaseBenchmark,
        BenchmarkMetrics,
        BenchmarkScenario,
        FrameworkType,
        LLMConfig,
        LLMProvider,
        create_llm_config_from_args,
        get_provider_models,
    )
    from frameworks.crewai_benchmark import CrewAIBenchmark
    from frameworks.graphbit_benchmark import GraphBitBenchmark
    from frameworks.langchain_benchmark import LangChainBenchmark
    from frameworks.langgraph_benchmark import LangGraphBenchmark
    from frameworks.llamaindex_benchmark import LlamaIndexBenchmark
    from frameworks.pydantic_ai_benchmark import PydanticAIBenchmark
except ImportError as e:
    print(f"ERROR: Failed to import benchmark modules: {e}")
    print("Make sure you're running this from the GraphBit workspace root directory.")
    print("Also ensure all required dependencies are installed:")
    print("  - pip install langchain langchain-openai")
    print("  - pip install langgraph")
    print("  - pip install pydantic-ai")
    print("  - pip install llama-index")
    print("  - pip install crewai")
    print("  - pip install matplotlib seaborn click")
    sys.exit(1)


class ComprehensiveBenchmarkRunner:
    """Runner for comprehensive framework benchmarks."""

    def __init__(self, llm_config: LLMConfig, verbose: bool = False):
        """Initialize the benchmark runner with configuration."""
        self.verbose = verbose
        self.llm_config = llm_config
        self.config: Dict[str, Any] = {
            "llm_config": llm_config,
            "provider": llm_config.provider.value,
            "model": llm_config.model,
            "temperature": llm_config.temperature,
            "max_tokens": llm_config.max_tokens,
            "api_key": llm_config.api_key or os.environ.get("OPENAI_API_KEY"),
            "base_url": llm_config.base_url,
            "log_dir": "logs",
            "results_dir": "results",
        }

        # Create logs and results directories
        log_dir = str(self.config.get("log_dir", "logs"))
        results_dir = str(self.config.get("results_dir", "results"))
        Path(log_dir).mkdir(exist_ok=True)
        Path(results_dir).mkdir(exist_ok=True)

        # Set up plotting style
        plt.style.use("seaborn-v0_8-darkgrid")
        sns.set_palette("husl")

        # Initialize framework benchmarks
        self.frameworks: Dict[FrameworkType, Dict[str, Any]] = {
            FrameworkType.GRAPHBIT: {
                "name": "GraphBit",
                "benchmark": GraphBitBenchmark(self.config),
                "results": {},
                "errors": {},
                "color": "#2E86AB",
            },
            FrameworkType.LANGCHAIN: {
                "name": "LangChain",
                "benchmark": LangChainBenchmark(self.config),
                "results": {},
                "errors": {},
                "color": "#A23B72",
            },
            FrameworkType.LANGGRAPH: {
                "name": "LangGraph",
                "benchmark": LangGraphBenchmark(self.config),
                "results": {},
                "errors": {},
                "color": "#062505",
            },
            FrameworkType.PYDANTIC_AI: {
                "name": "PydanticAI",
                "benchmark": PydanticAIBenchmark(self.config),
                "results": {},
                "errors": {},
                "color": "#F18F01",
            },
            FrameworkType.LLAMAINDEX: {
                "name": "LlamaIndex",
                "benchmark": LlamaIndexBenchmark(self.config),
                "results": {},
                "errors": {},
                "color": "#C73E1D",
            },
            FrameworkType.CREWAI: {
                "name": "CrewAI",
                "benchmark": CrewAIBenchmark(self.config),
                "results": {},
                "errors": {},
                "color": "#592E83",
            },
        }

        # Define scenarios to run
        self.scenarios = [
            (BenchmarkScenario.SIMPLE_TASK, "Simple Task"),
            (BenchmarkScenario.SEQUENTIAL_PIPELINE, "Sequential Pipeline"),
            (BenchmarkScenario.PARALLEL_PIPELINE, "Parallel Pipeline"),
            (BenchmarkScenario.COMPLEX_WORKFLOW, "Complex Workflow"),
            (BenchmarkScenario.MEMORY_INTENSIVE, "Memory Intensive"),
            (BenchmarkScenario.CONCURRENT_TASKS, "Concurrent Tasks"),
        ]

    def log(self, message: str, level: str = "INFO") -> None:
        """Log message with timestamp."""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        click.echo(f"[{timestamp}] {level}: {message}")

    def log_verbose(self, message: str) -> None:
        """Log verbose message only if verbose mode is enabled."""
        if self.verbose:
            self.log(message, "DEBUG")

    def print_framework_header(self, framework_name: str) -> None:
        """Print a header for the framework being tested."""
        if self.verbose:
            click.echo(f"\n{'=' * 60}")
            click.echo(f"Running {framework_name} Benchmark")
            click.echo(f"{'=' * 60}")
        else:
            self.log(f"Starting {framework_name} benchmark")

    def print_scenario_metrics(self, scenario_name: str, metrics: BenchmarkMetrics) -> None:
        """Print benchmark metrics in a formatted way."""
        if self.verbose:
            click.echo(f"\n{scenario_name} Results:")
            click.echo(f"  Execution Time: {metrics.execution_time_ms:.2f} ms")
            click.echo(f"  Memory Usage: {metrics.memory_usage_mb:.2f} MB")
            click.echo(f"  CPU Usage: {metrics.cpu_usage_percent:.2f}%")
            click.echo(f"  Token Count: {metrics.token_count}")
            click.echo(f"  Throughput: {metrics.throughput_tasks_per_sec:.2f} tasks/sec")
            click.echo(f"  Error Rate: {metrics.error_rate:.2%}")
            if metrics.concurrent_tasks > 0:
                click.echo(f"  Concurrent Tasks: {metrics.concurrent_tasks}")
            click.echo(f"  Setup Time: {metrics.setup_time_ms:.2f} ms")
            click.echo(f"  Teardown Time: {metrics.teardown_time_ms:.2f} ms")

            # Additional metadata if available
            if metrics.metadata:
                click.echo("  Metadata:")
                for key, value in metrics.metadata.items():
                    if isinstance(value, float):
                        click.echo(f"    {key}: {value:.2f}")
                    else:
                        click.echo(f"    {key}: {value}")
        else:
            self.log(f"{scenario_name}: {metrics.execution_time_ms:.0f}ms, {metrics.memory_usage_mb:.1f}MB, {metrics.cpu_usage_percent:.1f}% CPU, {metrics.token_count} tokens")

    async def run_framework_scenario(
        self,
        framework_type: FrameworkType,
        scenario: BenchmarkScenario,
        scenario_name: str,
    ) -> Tuple[Optional[BenchmarkMetrics], Optional[Exception]]:
        """Run a specific benchmark scenario for a framework."""
        framework_info = self.frameworks[framework_type]
        framework_name: str = framework_info["name"]
        benchmark: BaseBenchmark = framework_info["benchmark"]

        try:
            self.log_verbose(f"Running {framework_name} - {scenario_name}")
            metrics = await benchmark.run_scenario(scenario)
            self.print_scenario_metrics(scenario_name, metrics)
            return metrics, None
        except Exception as e:
            self.log(f"{framework_name} - {scenario_name} failed: {e}", "ERROR")
            if self.verbose:
                print(f"   Error details: {str(e)}")
            return None, e

    async def run_framework_benchmarks(self, framework_type: FrameworkType) -> None:
        """Run all benchmarks for a specific framework."""
        framework_info = self.frameworks[framework_type]
        framework_name = framework_info["name"]

        self.print_framework_header(framework_name)

        # Check if framework is available
        try:
            # Get benchmark class for initialization check
            framework_info["benchmark"]

            # Special check for GraphBit
            if framework_type == FrameworkType.GRAPHBIT:
                try:
                    import graphbit  # noqa: F401

                    self.log_verbose("GraphBit Python bindings are available")
                except ImportError:
                    self.log("GraphBit Python bindings not found!", "ERROR")
                    if self.verbose:
                        print("Please run the following commands first:")
                        print("  1. conda activate graphbit")
                        print("  2. cargo build --release")
                        print("  3. maturin develop --release")
                    framework_info["errors"]["import"] = "GraphBit not available"
                    return

        except Exception as e:
            self.log(f"Failed to initialize {framework_name}: {e}", "ERROR")
            framework_info["errors"]["init"] = str(e)
            return

        # Run all scenarios
        framework_start_time = time.time()

        for scenario, scenario_name in self.scenarios:
            metrics, error = await self.run_framework_scenario(framework_type, scenario, scenario_name)

            if metrics:
                framework_info["results"][scenario_name] = metrics
            if error:
                framework_info["errors"][scenario_name] = error

        framework_total_time = time.time() - framework_start_time

        # Framework summary
        successful_scenarios = len(framework_info["results"])

        self.log(f"{framework_name} completed: {successful_scenarios}/{len(self.scenarios)} scenarios successful, runtime: {framework_total_time:.2f}s")

        if self.verbose and framework_info["results"]:
            print(f"\n{framework_name} Performance Overview:")
            for scenario_name, metrics in framework_info["results"].items():
                print(f"  {scenario_name}: {metrics.execution_time_ms:.0f}ms, " f"{metrics.memory_usage_mb:.1f}MB, " f"{metrics.cpu_usage_percent:.1f}% CPU, " f"{metrics.token_count} tokens")

    def generate_comparison_report(self) -> None:
        """Generate a comparison report across all frameworks."""
        self.log("Generating comparison report")

        print(f"\n{'=' * 80}")
        print("COMPREHENSIVE BENCHMARK COMPARISON REPORT")
        print(f"{'=' * 80}")

        if self.verbose:
            # Summary table (only in verbose mode)
            print("\nFramework Performance Summary:")
            print(f"{'Framework':<15} {'Scenarios':<12} {'Avg Time (ms)':<15} {'Avg Memory (MB)':<16} {'Avg CPU (%)':<12} {'Avg Tokens':<12}")
            print(f"{'-' * 95}")

            for _framework_type, framework_info in self.frameworks.items():
                framework_name = framework_info["name"]
                results = framework_info["results"]

                if results:
                    scenario_count = len(results)
                    avg_time = sum(m.execution_time_ms for m in results.values()) / scenario_count
                    avg_memory = sum(m.memory_usage_mb for m in results.values()) / scenario_count
                    avg_cpu = sum(m.cpu_usage_percent for m in results.values()) / scenario_count
                    avg_tokens = sum(m.token_count for m in results.values()) / scenario_count

                    print(f"{framework_name:<15} {scenario_count:<12} {avg_time:<15.1f} {avg_memory:<16.1f} {avg_cpu:<12.1f} {avg_tokens:<12.0f}")
                else:
                    print(f"{framework_name:<15} {'0':<12} {'N/A':<15} {'N/A':<16} {'N/A':<12} {'N/A':<12}")

        # Detailed scenario comparison (always shown)
        print("\nDetailed Scenario Comparison:")
        for _scenario, scenario_name in self.scenarios:
            print(f"\n{scenario_name}:")
            print(f"{'Framework':<15} {'Time (ms)':<12} {'Memory (MB)':<13} {'CPU (%)':<10} {'Tokens':<10} {'Throughput':<12}")
            print(f"{'-' * 80}")

            for _framework_type, framework_info in self.frameworks.items():
                framework_name = framework_info["name"]
                results = framework_info["results"]

                if scenario_name in results:
                    metrics = results[scenario_name]
                    print(
                        f"{framework_name:<15} {metrics.execution_time_ms:<12.1f} "
                        f"{metrics.memory_usage_mb:<13.1f} {metrics.cpu_usage_percent:<10.1f} "
                        f"{metrics.token_count:<10} "
                        f"{metrics.throughput_tasks_per_sec:<12.2f}"
                    )
                else:
                    print(f"{framework_name:<15} {'FAILED':<12} {'FAILED':<13} {'FAILED':<10} {'FAILED':<10} {'FAILED':<12}")

        # Error summary
        has_errors = False
        for _framework_type, framework_info in self.frameworks.items():
            framework_name = framework_info["name"]
            errors = framework_info["errors"]

            if errors:
                has_errors = True
                self.log(f"{framework_name} had {len(errors)} errors", "WARNING")
                if self.verbose:
                    for scenario_name, error in errors.items():
                        error_msg = str(error)[:100] + "..." if len(str(error)) > 100 else str(error)
                        print(f"  {scenario_name}: {error_msg}")

        if not has_errors:
            self.log("No errors reported across all frameworks")

    def save_results_to_json(self) -> None:
        """Save benchmark results to JSON file for further analysis."""
        results_file = Path(str(self.config["log_dir"])) / "benchmark_results.json"

        # Convert results to JSON-serializable format
        json_results: Dict[str, Any] = {}
        for _framework_type, framework_info in self.frameworks.items():
            framework_name = framework_info["name"]
            json_results[framework_name] = {"results": {}, "errors": {}}

            # Convert metrics to dict
            for scenario_name, metrics in framework_info["results"].items():
                json_results[framework_name]["results"][scenario_name] = asdict(metrics)

            # Convert errors to string
            for scenario_name, error in framework_info["errors"].items():
                json_results[framework_name]["errors"][scenario_name] = str(error)

        # Add metadata
        json_results["_metadata"] = {
            "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
            "model": self.config["model"],
            "total_scenarios": len(self.scenarios),
            "frameworks_tested": len(self.frameworks),
        }

        try:
            with open(results_file, "w") as f:
                json.dump(json_results, f, indent=2)
            self.log(f"Results saved to: {results_file.absolute()}")
        except Exception as e:
            self.log(f"Failed to save results to JSON: {e}", "ERROR")

    def create_visualizations(self) -> None:
        """Create comprehensive visualizations of benchmark results."""
        self.log("Generating benchmark visualizations")

        # Create a simplified dashboard with only execution time and comparison table
        self.create_simple_dashboard()

        self.log(f"Visualization saved to: {Path(str(self.config['results_dir'])).absolute()}")

    def create_simple_dashboard(self) -> None:
        """Create a simple dashboard with execution time comparison and performance table."""
        fig, axes = plt.subplots(1, 2, figsize=(20, 8))
        fig.suptitle("Framework Benchmark Results", fontsize=20, fontweight="bold")

        # Prepare data
        scenario_names = [scenario_name for _, scenario_name in self.scenarios]
        x_pos = np.arange(len(scenario_names))
        width = 0.15

        framework_names = []
        colors_list = []

        for _framework_type, framework_info in self.frameworks.items():
            if framework_info["results"]:  # Only include frameworks with results
                framework_names.append(framework_info["name"])
                colors_list.append(framework_info["color"])

        # 1. Execution Time Comparison (Left side)
        ax1 = axes[0]
        for i, fw_name in enumerate(framework_names):
            fw_exec_times = []
            for scenario_name in scenario_names:
                found_time = None
                for _framework_type, framework_info in self.frameworks.items():
                    if framework_info["name"] == fw_name and scenario_name in framework_info["results"]:
                        found_time = framework_info["results"][scenario_name].execution_time_ms
                        break
                fw_exec_times.append(found_time if found_time is not None else 0)

            ax1.bar(
                x_pos + i * width,
                fw_exec_times,
                width,
                label=fw_name,
                color=colors_list[i],
                alpha=0.8,
            )

        ax1.set_xlabel("Benchmark Scenarios", fontsize=12)
        ax1.set_ylabel("Execution Time (milliseconds)", fontsize=12)
        ax1.set_title("Execution Time Comparison", fontsize=16, fontweight="bold")
        ax1.set_xticks(x_pos + width * (len(framework_names) - 1) / 2)
        ax1.set_xticklabels(scenario_names, rotation=45, ha="right")
        ax1.legend()
        ax1.grid(True, alpha=0.3)

        # 2. Performance Summary Table (Right side)
        ax2 = axes[1]
        ax2.axis("off")

        # Calculate average performance scores
        table_data = []
        headers = [
            "Framework",
            "Avg Exec Time (ms)",
            "Avg Memory (MB)",
            "Avg CPU (%)",
            "Avg Throughput (tasks/s)",
            "Success Rate (%)",
        ]

        for _fw_name in framework_names:
            found_framework_info: Optional[Dict[str, Any]] = None
            for _framework_type, fw_info in self.frameworks.items():
                if fw_info["name"] == _fw_name:
                    found_framework_info = fw_info
                    break

            if found_framework_info and found_framework_info["results"]:
                results = found_framework_info["results"]
                avg_exec = np.mean([m.execution_time_ms for m in results.values()])
                avg_memory = np.mean([m.memory_usage_mb for m in results.values()])
                avg_cpu = np.mean([m.cpu_usage_percent for m in results.values()])
                avg_throughput = np.mean([m.throughput_tasks_per_sec for m in results.values()])

                # Calculate success rate
                total_scenarios = len(found_framework_info["results"])
                total_errors = len(found_framework_info["errors"])
                success_rate = (total_scenarios / (total_scenarios + total_errors)) * 100 if total_scenarios + total_errors > 0 else 100

                table_data.append(
                    [
                        _fw_name,
                        f"{avg_exec:.0f}",
                        f"{avg_memory:.1f}",
                        f"{avg_cpu:.1f}",
                        f"{avg_throughput:.2f}",
                        f"{success_rate:.1f}%",
                    ]
                )

        # Create and style the table
        table = ax2.table(cellText=table_data, colLabels=headers, cellLoc="center", loc="center")
        table.auto_set_font_size(False)
        table.set_fontsize(11)
        table.scale(1.3, 2.0)

        # Style the table headers
        for i in range(len(headers)):
            table[(0, i)].set_facecolor("#4CAF50")
            table[(0, i)].set_text_props(weight="bold", color="white")

        # Color code the framework rows
        for i, _fw_name in enumerate(framework_names):
            row_idx = i + 1
            for j in range(len(headers)):
                if j == 0:  # Framework name column
                    table[(row_idx, j)].set_facecolor(colors_list[i])
                    table[(row_idx, j)].set_text_props(weight="bold", color="white")
                else:
                    table[(row_idx, j)].set_facecolor("#f0f0f0")

        ax2.set_title("Performance Summary Table", fontsize=16, fontweight="bold", pad=30)

        plt.tight_layout()

        # Save the dashboard
        dashboard_path = Path(str(self.config["results_dir"])) / "benchmark_results.png"
        plt.savefig(dashboard_path, dpi=300, bbox_inches="tight", facecolor="white")
        plt.close()

        self.log_verbose(f"Benchmark results saved: {dashboard_path}")

    async def run_all_benchmarks(self) -> None:
        """Run benchmarks for all frameworks."""
        self.log("Starting comprehensive framework benchmark")
        self.log(f"Testing {len(self.frameworks)} frameworks with {len(self.scenarios)} scenarios each")
        self.log(f"Model: {self.config['model']}")

        # Verify API key
        if not self.config["api_key"]:
            self.log("OPENAI_API_KEY not found in environment!", "ERROR")
            sys.exit(1)
        else:
            self.log_verbose("OPENAI_API_KEY configured")

        overall_start_time = time.time()

        # Run benchmarks for each framework
        for framework_type in self.frameworks.keys():
            try:
                await self.run_framework_benchmarks(framework_type)
            except Exception as e:
                framework_name = self.frameworks[framework_type]["name"]
                self.log(f"Catastrophic failure in {framework_name}: {e}", "ERROR")
                if self.verbose:
                    traceback.print_exc()
                self.frameworks[framework_type]["errors"]["catastrophic"] = str(e)

        overall_time = time.time() - overall_start_time

        # Generate comprehensive report
        self.generate_comparison_report()

        # Save results
        self.save_results_to_json()

        # Create visualizations
        self.create_visualizations()

        self.log(f"Benchmark completed in {overall_time:.2f} seconds")
        self.log(f"Results saved to: {Path(str(self.config['log_dir'])).absolute()}")
        self.log(f"Visualizations saved to: {Path(str(self.config['results_dir'])).absolute()}")


@click.command()
@click.option("--provider", type=click.Choice([p.value for p in LLMProvider], case_sensitive=False), default=LLMProvider.OPENAI.value, help="LLM provider to use for benchmarking", show_default=True)
@click.option("--model", type=str, default=DEFAULT_MODEL, help="Model name to use for benchmarking", show_default=True)
@click.option("--temperature", type=float, default=DEFAULT_TEMPERATURE, help="Temperature parameter for LLM", show_default=True)
@click.option("--max-tokens", type=int, default=DEFAULT_MAX_TOKENS, help="Maximum tokens for LLM responses", show_default=True)
@click.option("--api-key", type=str, help="API key for the LLM provider (overrides environment variable)")
@click.option("--base-url", type=str, help="Base URL for custom endpoints (e.g., Ollama)")
@click.option("--frameworks", type=str, help="Comma-separated list of frameworks to test (e.g., 'graphbit,langchain')")
@click.option("--scenarios", type=str, help="Comma-separated list of scenarios to run (e.g., 'simple_task,parallel_pipeline')")
@click.option("--verbose", "-v", is_flag=True, help="Enable verbose output")
@click.option("--list-models", is_flag=True, help="List available models for the selected provider and exit")
def main(
    provider: str,
    model: str,
    temperature: float,
    max_tokens: int,
    api_key: Optional[str],
    base_url: Optional[str],
    frameworks: Optional[str],
    scenarios: Optional[str],
    verbose: bool,
    list_models: bool,
) -> None:
    """Run comprehensive benchmarks for AI frameworks.

    This tool benchmarks GraphBit against other popular AI frameworks
    using configurable LLM providers and models.
    """
    if list_models:
        available_models = get_provider_models(provider)
        if available_models:
            click.echo(f"\nAvailable models for {provider}:")
            for model_name in available_models:
                click.echo(f"  • {model_name}")
        else:
            click.echo(f"No predefined models available for {provider}")
            click.echo("You can still specify a custom model name.")
        return

    # Check for API key only when actually running benchmarks
    if provider in ["openai"] and not (api_key or os.environ.get("OPENAI_API_KEY")):
        click.echo("Error: OpenAI API key is required", err=True)
        click.echo("Set it using: export OPENAI_API_KEY=your_api_key_here")
        click.echo("Or use: --api-key your_api_key_here")
        return
    elif provider == "anthropic" and not (api_key or os.environ.get("ANTHROPIC_API_KEY")):
        click.echo("Error: Anthropic API key is required", err=True)
        click.echo("Set it using: export ANTHROPIC_API_KEY=your_api_key_here")
        click.echo("Or use: --api-key your_api_key_here")
        return

    # Create configuration header
    click.echo("🚀 " + click.style("GraphBit Framework Benchmark", fg="bright_blue", bold=True))
    click.echo("=" * 50)
    click.echo(f"Provider: {click.style(provider, fg='green', bold=True)}")
    click.echo(f"Model: {click.style(model, fg='green', bold=True)}")
    click.echo(f"Temperature: {click.style(str(temperature), fg='yellow')}")
    click.echo(f"Max Tokens: {click.style(str(max_tokens), fg='yellow')}")
    if base_url:
        click.echo(f"Base URL: {click.style(base_url, fg='cyan')}")
    click.echo("=" * 50)

    # Validate model for provider
    available_models = get_provider_models(provider)
    if available_models and model not in available_models:
        click.echo(f" Warning: '{model}' is not in the predefined models for {provider}")
        click.echo(f"Available models: {', '.join(available_models)}")
        click.confirm("Continue anyway?", abort=True)

    # Create LLM configuration
    try:
        llm_config = create_llm_config_from_args(provider=provider, model=model, temperature=temperature, max_tokens=max_tokens, api_key=api_key, base_url=base_url)
    except ValueError as e:
        click.echo(f"Configuration error: {e}", err=True)
        return

    # Parse framework selection
    selected_frameworks = None
    if frameworks:
        framework_names = [name.strip().lower() for name in frameworks.split(",")]
        selected_frameworks = []
        for name in framework_names:
            try:
                # Map common names to FrameworkType
                name_mapping = {
                    "graphbit": FrameworkType.GRAPHBIT,
                    "langchain": FrameworkType.LANGCHAIN,
                    "langgraph": FrameworkType.LANGGRAPH,
                    "pydantic_ai": FrameworkType.PYDANTIC_AI,
                    "pydanticai": FrameworkType.PYDANTIC_AI,
                    "llamaindex": FrameworkType.LLAMAINDEX,
                    "llama_index": FrameworkType.LLAMAINDEX,
                    "crewai": FrameworkType.CREWAI,
                    "crew_ai": FrameworkType.CREWAI,
                }
                if name in name_mapping:
                    selected_frameworks.append(name_mapping[name])
                else:
                    framework_type = FrameworkType(name.upper())
                    selected_frameworks.append(framework_type)
            except ValueError:
                click.echo(f"Unknown framework: {name}", err=True)
                click.echo(f"Available frameworks: {', '.join([f.value.lower() for f in FrameworkType])}")
                return

    # Parse scenario selection
    selected_scenarios = None
    if scenarios:
        scenario_names = [name.strip().lower() for name in scenarios.split(",")]
        selected_scenarios = []
        for name in scenario_names:
            try:
                scenario = BenchmarkScenario(name)
                selected_scenarios.append(scenario)
            except ValueError:
                click.echo(f"Unknown scenario: {name}", err=True)
                click.echo(f"Available scenarios: {', '.join([s.value for s in BenchmarkScenario])}")
                return

    # Run the benchmark
    async def run_benchmark():
        runner = ComprehensiveBenchmarkRunner(llm_config, verbose=verbose)

        # Filter frameworks if specified
        if selected_frameworks:
            original_frameworks = runner.frameworks.copy()
            runner.frameworks = {fw_type: fw_info for fw_type, fw_info in original_frameworks.items() if fw_type in selected_frameworks}

        # Filter scenarios if specified
        if selected_scenarios:
            original_scenarios = runner.scenarios.copy()
            runner.scenarios = [(scenario, name) for scenario, name in original_scenarios if scenario in selected_scenarios]

        await runner.run_all_benchmarks()

    try:
        asyncio.run(run_benchmark())
    except KeyboardInterrupt:
        click.echo("\nBenchmark interrupted by user")
    except Exception as e:
        click.echo(f"Benchmark failed: {e}", err=True)
        if verbose:
            import traceback

            traceback.print_exc()


if __name__ == "__main__":
    main()
