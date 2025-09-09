#!/usr/bin/env python3
"""Testing Runner for GraphBit CI/CD Pipeline.

This script handles comprehensive testing including unit tests,
integration tests, and coverage reporting.
"""

import argparse
import os
import shutil
import subprocess  # nosec B404: import of 'subprocess'
import sys
import time
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import List, Optional, Tuple


@dataclass
class TestResult:
    """Result of a test execution."""

    name: str
    passed: bool
    duration: float
    output: str
    coverage: Optional[float] = None
    error: Optional[str] = None


class TestingRunner:
    """Runs comprehensive test suites."""

    def __init__(self, root_path: Path):
        """Initialize the testing runner.

        Args:
            root_path: Root path of the project
        """
        self.root_path = root_path
        self.results: List[TestResult] = []
        self.artifacts_dir = root_path / ".github" / "artifacts"
        self.artifacts_dir.mkdir(parents=True, exist_ok=True)

    def run_all_tests(self, platform: str = "ubuntu-latest") -> bool:
        """Run all test suites."""
        print(f"Starting comprehensive testing on {platform}...")

        tests = [
            ("Rust Unit Tests", self._run_rust_tests),
            ("Rust Integration Tests", self._run_rust_integration_tests),
            ("Python Unit Tests", self._run_python_tests),
            ("Python Integration Tests", self._run_python_integration_tests),
            ("Coverage Analysis", self._run_coverage_analysis),
            ("Performance Tests", self._run_performance_tests),
        ]

        all_passed = True

        for name, test_func in tests:
            print(f"\nRunning {name}...")
            start_time = time.time()

            try:
                passed, output, error, coverage = test_func()
                duration = time.time() - start_time

                result = TestResult(name=name, passed=passed, duration=duration, output=output, coverage=coverage, error=error)

                self.results.append(result)

                status = "✅ PASSED" if passed else "❌ FAILED"
                coverage_text = f" (Coverage: {coverage:.1f}%)" if coverage else ""
                print(f"{status} - {name} ({duration:.2f}s){coverage_text}")

                if not passed:
                    all_passed = False
                    if error:
                        print(f"Error: {error}")

            except Exception as e:
                duration = time.time() - start_time
                result = TestResult(name=name, passed=False, duration=duration, output="", error=str(e))

                self.results.append(result)
                all_passed = False
                print(f"❌ FAILED - {name} ({duration:.2f}s)")
                print(f"Exception: {e}")

        self._generate_report()
        return all_passed

    def _run_rust_tests(self) -> Tuple[bool, str, Optional[str], Optional[float]]:
        """Run Rust unit tests."""
        try:
            cargo_path = shutil.which("cargo")
            if not cargo_path:
                raise FileNotFoundError("cargo executable not found in PATH")
            result = subprocess.run(
                [cargo_path, "test", "--workspace", "--lib", "--bins", "--verbose"],
                capture_output=True,
                text=True,
                cwd=self.root_path,
                timeout=600,
                env={**os.environ, "RUST_BACKTRACE": "1"},
                shell=False,
            )  # nosec

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None, None

        except subprocess.TimeoutExpired:
            return False, "", "Rust unit tests timed out", None
        except Exception as e:
            return False, "", str(e), None

    def _run_rust_integration_tests(self) -> Tuple[bool, str, Optional[str], Optional[float]]:
        """Run Rust integration tests."""
        try:
            cargo_path = shutil.which("cargo")
            if not cargo_path:
                raise FileNotFoundError("cargo executable not found in PATH")
            result = subprocess.run(
                [cargo_path, "test", "--workspace", "--tests", "--verbose"], capture_output=True, text=True, cwd=self.root_path, timeout=900, env={**os.environ, "RUST_BACKTRACE": "1"}, shell=False
            )  # nosec

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None, None

        except subprocess.TimeoutExpired:
            return False, "", "Rust integration tests timed out", None
        except Exception as e:
            return False, "", str(e), None

    def _run_python_tests(self) -> Tuple[bool, str, Optional[str], Optional[float]]:
        """Run Python unit tests."""
        try:
            # First, try to build the Python extension
            python_path = shutil.which("python")
            if not python_path:
                raise FileNotFoundError("python executable not found in PATH")
            build_result = subprocess.run([python_path, "-m", "pip", "install", "-e", "python/"], capture_output=True, text=True, cwd=self.root_path, timeout=300, shell=False)  # nosec

            if build_result.returncode != 0:
                return False, build_result.stdout, f"Failed to build Python extension: {build_result.stderr}", None

            # Run Python tests
            test_paths = []
            if (self.root_path / "tests").exists():
                test_paths.append("tests/")
            if (self.root_path / "python" / "tests").exists():
                test_paths.append("python/tests/")

            if not test_paths:
                return True, "No Python tests found, skipping", None, None

            result = subprocess.run(["python", "-m", "pytest"] + test_paths + ["-v", "--tb=short"], capture_output=True, text=True, cwd=self.root_path, timeout=600, shell=False)  # nosec

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None, None

        except subprocess.TimeoutExpired:
            return False, "", "Python unit tests timed out", None
        except Exception as e:
            return False, "", str(e), None

    def _run_python_integration_tests(self) -> Tuple[bool, str, Optional[str], Optional[float]]:
        """Run Python integration tests."""
        try:
            # Look for integration test directories
            integration_paths = []
            if (self.root_path / "tests" / "integration").exists():
                integration_paths.append("tests/integration/")
            if (self.root_path / "python" / "tests" / "integration").exists():
                integration_paths.append("python/tests/integration/")

            if not integration_paths:
                return True, "No Python integration tests found, skipping", None, None

            result = subprocess.run(
                ["python", "-m", "pytest"] + integration_paths + ["-v", "--tb=short"],
                capture_output=True,
                text=True,
                cwd=self.root_path,
                timeout=900,
                env={**os.environ, "TEST_REMOTE_URLS": "true"},
                shell=False,
            )  # nosec

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None, None

        except subprocess.TimeoutExpired:
            return False, "", "Python integration tests timed out", None
        except Exception as e:
            return False, "", str(e), None

    def _run_coverage_analysis(self) -> Tuple[bool, str, Optional[str], Optional[float]]:
        """Run coverage analysis."""
        try:
            # Run Rust coverage with tarpaulin
            rust_coverage = None
            try:
                cargo_path = shutil.which("cargo")
                if not cargo_path:
                    raise FileNotFoundError("cargo executable not found in PATH")
                result = subprocess.run(
                    [cargo_path, "tarpaulin", "--workspace", "--out", "Xml", "--output-dir", "target/coverage"], capture_output=True, text=True, cwd=self.root_path, timeout=900, shell=False
                )  # nosec

                if result.returncode == 0:
                    # Try to extract coverage percentage
                    import re

                    coverage_match = re.search(r"(\d+\.?\d*)%", result.stdout)
                    if coverage_match:
                        rust_coverage = float(coverage_match.group(1))

            except (subprocess.TimeoutExpired, FileNotFoundError):
                pass  # tarpaulin might not be available

            # Run Python coverage
            python_coverage = None
            try:
                test_paths = []
                if (self.root_path / "tests").exists():
                    test_paths.append("tests/")
                if (self.root_path / "python" / "tests").exists():
                    test_paths.append("python/tests/")

                if test_paths:
                    result = subprocess.run(
                        ["python", "-m", "pytest"] + test_paths + ["--cov=graphbit", "--cov-report=xml", "--cov-report=term-missing"],
                        capture_output=True,
                        text=True,
                        cwd=self.root_path,
                        timeout=600,
                        shell=False,
                    )  # nosec

                    if result.returncode == 0:
                        # Try to extract coverage percentage
                        import re

                        coverage_match = re.search(r"TOTAL.*?(\d+)%", result.stdout)
                        if coverage_match:
                            python_coverage = float(coverage_match.group(1))

            except (subprocess.TimeoutExpired, FileNotFoundError):
                pass  # pytest-cov might not be available

            # Determine overall result
            if rust_coverage is not None or python_coverage is not None:
                overall_coverage = max(filter(None, [rust_coverage, python_coverage]))
                coverage_text = f"Rust: {rust_coverage or 'N/A'}%, Python: {python_coverage or 'N/A'}%"
                return True, coverage_text, None, overall_coverage
            else:
                return True, "Coverage tools not available, skipping coverage analysis", None, None

        except Exception as e:
            return False, "", str(e), None

    def _run_performance_tests(self) -> Tuple[bool, str, Optional[str], Optional[float]]:
        """Run performance tests."""
        try:
            # Check if benchmarks exist
            if not (self.root_path / "benches").exists():
                return True, "No benchmarks found, skipping performance tests", None, None

            cargo_path = shutil.which("cargo")
            if not cargo_path:
                raise FileNotFoundError("cargo executable not found in PATH")
            result = subprocess.run([cargo_path, "bench", "--workspace"], capture_output=True, text=True, cwd=self.root_path, timeout=1200, shell=False)  # nosec

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None, None

        except subprocess.TimeoutExpired:
            return False, "", "Performance tests timed out", None
        except Exception as e:
            return False, "", str(e), None

    def _generate_report(self):
        """Generate testing report."""
        passed_count = sum(1 for r in self.results if r.passed)
        total_count = len(self.results)
        total_duration = sum(r.duration for r in self.results)

        # Calculate average coverage
        coverage_results = [r.coverage for r in self.results if r.coverage is not None]
        avg_coverage = sum(coverage_results) / len(coverage_results) if coverage_results else None

        print("\n" + "=" * 60)
        print("TESTING REPORT")
        print("=" * 60)
        print(f"Total Test Suites: {total_count}")
        print(f"Passed: {passed_count}")
        print(f"Failed: {total_count - passed_count}")
        print(f"Total Duration: {total_duration:.2f}s")

        if avg_coverage:
            print(f"Average Coverage: {avg_coverage:.1f}%")

        if passed_count == total_count:
            print("\n🎉 All test suites passed!")
        else:
            print(f"\n⚠️  {total_count - passed_count} test suites failed:")
            for result in self.results:
                if not result.passed:
                    print(f"  ❌ {result.name}")
                    if result.error:
                        print(f"     Error: {result.error}")

        # Save detailed report
        self._save_detailed_report()

    def _save_detailed_report(self):
        """Save detailed testing report to file."""
        report_file = self.artifacts_dir / f"testing-report-{datetime.now().strftime('%Y%m%d-%H%M%S')}.json"

        coverage_results = [r.coverage for r in self.results if r.coverage is not None]
        avg_coverage = sum(coverage_results) / len(coverage_results) if coverage_results else None

        report_data = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total_suites": len(self.results),
                "passed_suites": sum(1 for r in self.results if r.passed),
                "failed_suites": sum(1 for r in self.results if not r.passed),
                "total_duration": sum(r.duration for r in self.results),
                "average_coverage": avg_coverage,
            },
            "results": [{"name": r.name, "passed": r.passed, "duration": r.duration, "coverage": r.coverage, "output": r.output, "error": r.error} for r in self.results],
        }

        with open(report_file, "w") as f:
            import json

            json.dump(report_data, f, indent=2)

        print(f"\nDetailed report saved to: {report_file}")


def main():
    """Run the testing runner."""
    parser = argparse.ArgumentParser(description="GraphBit Testing Runner", formatter_class=argparse.RawDescriptionHelpFormatter)

    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Root directory of the project")

    parser.add_argument("--platform", default="ubuntu-latest", help="Platform identifier for testing")

    parser.add_argument("--suite", choices=["rust-unit", "rust-integration", "python-unit", "python-integration", "coverage", "performance", "all"], default="all", help="Specific test suite to run")

    args = parser.parse_args()

    runner = TestingRunner(args.root)

    if args.suite == "all":
        success = runner.run_all_tests(args.platform)
    else:
        # Run specific suite (implementation would be added here)
        print(f"Running specific test suite: {args.suite}")
        success = runner.run_all_tests(args.platform)  # For now, run all

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
