#!/usr/bin/env python3
"""
Comprehensive Test Suite Runner for GraphBit Modular Workflows

This script orchestrates all testing activities for the modular workflow system,
providing a single entry point for comprehensive validation.
"""

import argparse
import json
import os
import subprocess
import sys
import time
from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


class TestSuite(Enum):
    """Available test suites."""

    VALIDATION = "validation"
    INTEGRATION = "integration"
    WORKFLOW = "workflow"
    LEGACY_MIGRATION = "legacy-migration"
    ALL = "all"


@dataclass
class TestSuiteResult:
    """Result of a test suite execution."""

    suite: TestSuite
    passed: bool
    duration: float
    details: Dict[str, Any]
    error_message: Optional[str] = None


class TestSuiteRunner:
    """Orchestrates comprehensive testing of the modular workflow system."""

    def __init__(self, root_path: Path, github_token: Optional[str] = None):
        self.root_path = root_path
        self.github_token = github_token
        self.results: List[TestSuiteResult] = []
        self.test_artifacts_dir = root_path / ".github" / "test-suite-artifacts"

        # Ensure test artifacts directory exists
        self.test_artifacts_dir.mkdir(parents=True, exist_ok=True)

        # Detect repository info
        self.repo_owner, self.repo_name = self._detect_repository_info()

    def run_test_suite(self, suite: TestSuite) -> bool:
        """Run specified test suite."""
        print(f"Starting Test Suite: {suite.value.upper()}")
        print("=" * 60)

        if suite == TestSuite.ALL:
            return self._run_all_test_suites()
        elif suite == TestSuite.VALIDATION:
            return self._run_validation_tests()
        elif suite == TestSuite.INTEGRATION:
            return self._run_integration_tests()
        elif suite == TestSuite.WORKFLOW:
            return self._run_workflow_tests()
        elif suite == TestSuite.LEGACY_MIGRATION:
            return self._run_legacy_migration_tests()
        else:
            print(f"[ERROR] Unknown test suite: {suite}")
            return False

    def _run_all_test_suites(self) -> bool:
        """Run all test suites in order."""
        suites_to_run = [TestSuite.VALIDATION, TestSuite.INTEGRATION, TestSuite.WORKFLOW, TestSuite.LEGACY_MIGRATION]

        all_passed = True

        for suite in suites_to_run:
            print(f"\n{'='*20} {suite.value.upper()} TESTS {'='*20}")

            start_time = time.time()
            passed = self._run_single_suite(suite)
            duration = time.time() - start_time

            result = TestSuiteResult(suite=suite, passed=passed, duration=duration, details={})

            self.results.append(result)

            status = "[PASS]" if passed else "[FAIL]"
            print(f"\n{status} - {suite.value} tests ({duration:.2f}s)")

            if not passed:
                all_passed = False
                print(f"[WARN] {suite.value} tests failed - continuing with remaining suites")

        self._generate_comprehensive_report()
        return all_passed

    def _run_single_suite(self, suite: TestSuite) -> bool:
        """Run a single test suite."""
        if suite == TestSuite.VALIDATION:
            return self._run_validation_tests()
        elif suite == TestSuite.INTEGRATION:
            return self._run_integration_tests()
        elif suite == TestSuite.WORKFLOW:
            return self._run_workflow_tests()
        elif suite == TestSuite.LEGACY_MIGRATION:
            return self._run_legacy_migration_tests()
        else:
            return False

    def _run_validation_tests(self) -> bool:
        """Run workflow system validation tests."""
        print("Running Workflow System Validation...")

        validator_script = self.root_path / "scripts" / "workflow-validator.py"

        if not validator_script.exists():
            print("[ERROR] Workflow validator script not found")
            return False

        try:
            result = subprocess.run([sys.executable, str(validator_script), "--root", str(self.root_path)], capture_output=True, text=True, cwd=self.root_path, timeout=300)

            print("Validation Output:")
            print(result.stdout)

            if result.stderr:
                print("Validation Errors:")
                print(result.stderr)

            return result.returncode == 0

        except subprocess.TimeoutExpired:
            print("[ERROR] Validation tests timed out")
            return False
        except Exception as e:
            print(f"[ERROR] Validation tests failed: {e}")
            return False

    def _run_integration_tests(self) -> bool:
        """Run integration tests."""
        print("Running Integration Tests...")

        integration_script = self.root_path / "scripts" / "integration-tester.py"

        if not integration_script.exists():
            print("[ERROR] Integration tester script not found")
            return False

        if not self.repo_owner or not self.repo_name:
            print("[WARN] Could not detect repository info - skipping integration tests")
            return True

        if not self.github_token:
            print("[WARN] No GitHub token provided - skipping API-dependent integration tests")
            return True

        try:
            result = subprocess.run(
                [sys.executable, str(integration_script), "--repo-owner", self.repo_owner, "--repo-name", self.repo_name, "--github-token", self.github_token, "--root", str(self.root_path)],
                capture_output=True,
                text=True,
                cwd=self.root_path,
                timeout=600,
            )

            print("Integration Test Output:")
            print(result.stdout)

            if result.stderr:
                print("Integration Test Errors:")
                print(result.stderr)

            return result.returncode == 0

        except subprocess.TimeoutExpired:
            print("[ERROR] Integration tests timed out")
            return False
        except Exception as e:
            print(f"[ERROR] Integration tests failed: {e}")
            return False

    def _run_workflow_tests(self) -> bool:
        """Run workflow-specific tests."""
        print("Running Workflow Tests...")

        workflow_tester_script = self.root_path / "scripts" / "workflow-tester.py"

        if not workflow_tester_script.exists():
            print("[ERROR] Workflow tester script not found")
            return False

        if not self.repo_owner or not self.repo_name:
            print("[WARN] Could not detect repository info - skipping workflow tests")
            return True

        if not self.github_token:
            print("[WARN] No GitHub token provided - skipping API-dependent workflow tests")
            return True

        try:
            result = subprocess.run(
                [sys.executable, str(workflow_tester_script), "--repo-owner", self.repo_owner, "--repo-name", self.repo_name, "--github-token", self.github_token, "--root", str(self.root_path)],
                capture_output=True,
                text=True,
                cwd=self.root_path,
                timeout=900,
            )

            print("Workflow Test Output:")
            print(result.stdout)

            if result.stderr:
                print("Workflow Test Errors:")
                print(result.stderr)

            return result.returncode == 0

        except subprocess.TimeoutExpired:
            print("[ERROR] Workflow tests timed out")
            return False
        except Exception as e:
            print(f"[ERROR] Workflow tests failed: {e}")
            return False

    def _run_legacy_migration_tests(self) -> bool:
        """Run legacy migration tests."""
        print("Running Legacy Migration Tests...")

        migration_script = self.root_path / "scripts" / "legacy-migrator.py"

        if not migration_script.exists():
            print("[ERROR] Legacy migrator script not found")
            return False

        try:
            # Run migration in dry-run mode
            result = subprocess.run([sys.executable, str(migration_script), "--root", str(self.root_path), "--dry-run"], capture_output=True, text=True, cwd=self.root_path, timeout=300)

            print("Migration Test Output:")
            print(result.stdout)

            if result.stderr:
                print("Migration Test Errors:")
                print(result.stderr)

            return result.returncode == 0

        except subprocess.TimeoutExpired:
            print("[ERROR] Migration tests timed out")
            return False
        except Exception as e:
            print(f"[ERROR] Migration tests failed: {e}")
            return False

    def _detect_repository_info(self) -> Tuple[Optional[str], Optional[str]]:
        """Detect repository owner and name from git remote."""
        try:
            result = subprocess.run(["git", "remote", "get-url", "origin"], capture_output=True, text=True, cwd=self.root_path, timeout=30)

            if result.returncode == 0:
                repo_url = result.stdout.strip()

                # Parse GitHub URL
                if "github.com" in repo_url:
                    # Handle both HTTPS and SSH URLs
                    if repo_url.startswith("https://github.com/"):
                        parts = repo_url.replace("https://github.com/", "").replace(".git", "").split("/")
                    elif repo_url.startswith("git@github.com:"):
                        parts = repo_url.replace("git@github.com:", "").replace(".git", "").split("/")
                    else:
                        return None, None

                    if len(parts) >= 2:
                        return parts[0], parts[1]

            return None, None

        except Exception:
            return None, None

    def _generate_comprehensive_report(self):
        """Generate comprehensive test report."""
        passed_count = sum(1 for r in self.results if r.passed)
        failed_count = sum(1 for r in self.results if not r.passed)
        total_count = len(self.results)
        total_duration = sum(r.duration for r in self.results)

        print("\n" + "=" * 80)
        print("COMPREHENSIVE TEST SUITE REPORT")
        print("=" * 80)
        print(f"Repository: {self.repo_owner}/{self.repo_name}" if self.repo_owner else "Repository: Unknown")
        print(f"Total Test Suites: {total_count}")
        print(f"Passed: {passed_count}")
        print(f"Failed: {failed_count}")
        print(f"Success Rate: {(passed_count/total_count)*100:.1f}%" if total_count > 0 else "N/A")
        print(f"Total Duration: {total_duration:.2f}s")

        # Show individual suite results
        print(f"\n[TEST SUITE RESULTS]:")
        for result in self.results:
            status = "[PASS]" if result.passed else "[FAIL]"
            print(f"  {status} {result.suite.value.upper()} ({result.duration:.2f}s)")
            if result.error_message:
                print(f"      Error: {result.error_message}")

        if failed_count > 0:
            print(f"\n[WARNING] {failed_count} test suites failed:")
            for result in self.results:
                if not result.passed:
                    print(f"  - {result.suite.value}")

        if passed_count == total_count:
            print("\n[SUCCESS] All test suites passed! The modular workflow system is ready for production.")
            print("\n[NEXT STEPS]:")
            print("1. Deploy the modular workflow system")
            print("2. Update branch protection rules")
            print("3. Monitor first workflow runs")
            print("4. Remove legacy workflow backups when confident")
        else:
            print(f"\n[WARNING] {failed_count} test suites failed. Please review and fix issues before deployment.")
            print("\n[REQUIRED ACTIONS]:")
            print("1. Review failed test suite outputs")
            print("2. Fix identified issues")
            print("3. Re-run test suites")
            print("4. Ensure all tests pass before deployment")

        # Save detailed report
        self._save_comprehensive_report()

    def _save_comprehensive_report(self):
        """Save detailed test suite report."""
        report_file = self.test_artifacts_dir / f"test-suite-report-{datetime.now().strftime('%Y%m%d-%H%M%S')}.json"

        report_data = {
            "timestamp": datetime.now().isoformat(),
            "repository": f"{self.repo_owner}/{self.repo_name}" if self.repo_owner else "unknown",
            "summary": {
                "total_suites": len(self.results),
                "passed_suites": sum(1 for r in self.results if r.passed),
                "failed_suites": sum(1 for r in self.results if not r.passed),
                "success_rate": (sum(1 for r in self.results if r.passed) / len(self.results)) * 100 if self.results else 0,
                "total_duration": sum(r.duration for r in self.results),
            },
            "results": [{"suite": r.suite.value, "passed": r.passed, "duration": r.duration, "details": r.details, "error_message": r.error_message} for r in self.results],
        }

        with open(report_file, "w") as f:
            json.dump(report_data, f, indent=2)

        print(f"\nDetailed test suite report saved to: {report_file}")


def main():
    """Main entry point for test suite runner."""
    parser = argparse.ArgumentParser(
        description="GraphBit Comprehensive Test Suite Runner",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Test Suites:
  validation      - Validate workflow system integrity
  integration     - Test integration between legacy and modular systems
  workflow        - Test individual workflows and orchestration
  legacy-migration - Test legacy workflow migration process
  all             - Run all test suites (default)

Examples:
  python test-suite-runner.py --suite all
  python test-suite-runner.py --suite validation --root /path/to/project
  python test-suite-runner.py --suite workflow --github-token $GITHUB_TOKEN
        """,
    )

    parser.add_argument("--suite", choices=[s.value for s in TestSuite], default="all", help="Test suite to run")

    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Root directory of the project")

    parser.add_argument("--github-token", help="GitHub API token (or set GITHUB_TOKEN env var)")

    args = parser.parse_args()

    # Get GitHub token
    github_token = args.github_token or os.environ.get("GITHUB_TOKEN")

    runner = TestSuiteRunner(args.root, github_token)
    suite = TestSuite(args.suite)
    success = runner.run_test_suite(suite)

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
