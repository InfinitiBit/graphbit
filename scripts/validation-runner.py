#!/usr/bin/env python3
"""
Validation Runner for GraphBit CI/CD Pipeline

This script handles code quality validation, linting, security checks,
and other pre-testing validation steps.
"""

import argparse
import subprocess
import sys
import time
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple


@dataclass
class ValidationResult:
    """Result of a validation check."""

    name: str
    passed: bool
    duration: float
    output: str
    error: Optional[str] = None


class ValidationRunner:
    """Runs comprehensive validation checks."""

    def __init__(self, root_path: Path):
        self.root_path = root_path
        self.results: List[ValidationResult] = []

    def run_all_validations(self) -> bool:
        """Run all validation checks."""
        print("Starting comprehensive validation...")

        validations = [
            ("Code Formatting (Rust)", self._check_rust_formatting),
            ("Code Linting (Rust)", self._check_rust_linting),
            ("Code Formatting (Python)", self._check_python_formatting),
            ("Code Linting (Python)", self._check_python_linting),
            ("Security Scan", self._run_security_scan),
            ("Dependency Check", self._check_dependencies),
            ("Version Consistency", self._check_version_consistency),
            ("Documentation Build", self._check_documentation),
        ]

        all_passed = True

        for name, validation_func in validations:
            print(f"\nRunning {name}...")
            start_time = time.time()

            try:
                passed, output, error = validation_func()
                duration = time.time() - start_time

                result = ValidationResult(name=name, passed=passed, duration=duration, output=output, error=error)

                self.results.append(result)

                status = "✅ PASSED" if passed else "❌ FAILED"
                print(f"{status} - {name} ({duration:.2f}s)")

                if not passed:
                    all_passed = False
                    if error:
                        print(f"Error: {error}")

            except Exception as e:
                duration = time.time() - start_time
                result = ValidationResult(name=name, passed=False, duration=duration, output="", error=str(e))

                self.results.append(result)
                all_passed = False
                print(f"❌ FAILED - {name} ({duration:.2f}s)")
                print(f"Exception: {e}")

        self._generate_report()
        return all_passed

    def _check_rust_formatting(self) -> Tuple[bool, str, Optional[str]]:
        """Check Rust code formatting."""
        try:
            result = subprocess.run(["cargo", "fmt", "--all", "--", "--check"], capture_output=True, text=True, cwd=self.root_path, timeout=120)

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None

        except subprocess.TimeoutExpired:
            return False, "", "Rust formatting check timed out"
        except Exception as e:
            return False, "", str(e)

    def _check_rust_linting(self) -> Tuple[bool, str, Optional[str]]:
        """Check Rust code with clippy."""
        try:
            result = subprocess.run(["cargo", "clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"], capture_output=True, text=True, cwd=self.root_path, timeout=300)

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None

        except subprocess.TimeoutExpired:
            return False, "", "Rust clippy check timed out"
        except Exception as e:
            return False, "", str(e)

    def _check_python_formatting(self) -> Tuple[bool, str, Optional[str]]:
        """Check Python code formatting."""
        try:
            # Check if black is available
            result = subprocess.run(["python", "-m", "black", "--check", "--diff", "python/", "scripts/"], capture_output=True, text=True, cwd=self.root_path, timeout=120)

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None

        except subprocess.TimeoutExpired:
            return False, "", "Python formatting check timed out"
        except FileNotFoundError:
            # If black is not available, skip this check
            return True, "Black not available, skipping Python formatting check", None
        except Exception as e:
            return False, "", str(e)

    def _check_python_linting(self) -> Tuple[bool, str, Optional[str]]:
        """Check Python code with flake8 or similar."""
        try:
            # Try flake8 first
            result = subprocess.run(["python", "-m", "flake8", "python/", "scripts/"], capture_output=True, text=True, cwd=self.root_path, timeout=120)

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None

        except subprocess.TimeoutExpired:
            return False, "", "Python linting check timed out"
        except FileNotFoundError:
            # If flake8 is not available, try pylint or skip
            try:
                result = subprocess.run(["python", "-m", "pylint", "python/", "scripts/"], capture_output=True, text=True, cwd=self.root_path, timeout=180)

                # Pylint returns non-zero for warnings, so we're more lenient
                return result.returncode < 16, result.stdout, result.stderr if result.returncode >= 16 else None

            except FileNotFoundError:
                return True, "No Python linter available, skipping Python linting check", None
        except Exception as e:
            return False, "", str(e)

    def _run_security_scan(self) -> Tuple[bool, str, Optional[str]]:
        """Run security scans."""
        try:
            # Check for common security issues in dependencies
            result = subprocess.run(["cargo", "audit"], capture_output=True, text=True, cwd=self.root_path, timeout=180)

            # cargo audit returns non-zero for vulnerabilities
            if result.returncode != 0:
                return False, result.stdout, result.stderr

            return True, result.stdout, None

        except subprocess.TimeoutExpired:
            return False, "", "Security scan timed out"
        except FileNotFoundError:
            # If cargo audit is not available, skip this check
            return True, "cargo-audit not available, skipping security scan", None
        except Exception as e:
            return False, "", str(e)

    def _check_dependencies(self) -> Tuple[bool, str, Optional[str]]:
        """Check dependency consistency."""
        try:
            # Check Cargo.lock is up to date
            result = subprocess.run(["cargo", "check", "--locked"], capture_output=True, text=True, cwd=self.root_path, timeout=300)

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None

        except subprocess.TimeoutExpired:
            return False, "", "Dependency check timed out"
        except Exception as e:
            return False, "", str(e)

    def _check_version_consistency(self) -> Tuple[bool, str, Optional[str]]:
        """Check version consistency across files."""
        try:
            # Use existing version sync script if available
            version_script = self.root_path / "scripts" / "verify-version-sync.py"

            if version_script.exists():
                result = subprocess.run(["python", str(version_script)], capture_output=True, text=True, cwd=self.root_path, timeout=60)

                return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None
            else:
                return True, "Version sync script not found, skipping version consistency check", None

        except subprocess.TimeoutExpired:
            return False, "", "Version consistency check timed out"
        except Exception as e:
            return False, "", str(e)

    def _check_documentation(self) -> Tuple[bool, str, Optional[str]]:
        """Check documentation builds successfully."""
        try:
            # Try to build documentation
            result = subprocess.run(["cargo", "doc", "--no-deps", "--workspace"], capture_output=True, text=True, cwd=self.root_path, timeout=300)

            return result.returncode == 0, result.stdout, result.stderr if result.returncode != 0 else None

        except subprocess.TimeoutExpired:
            return False, "", "Documentation build timed out"
        except Exception as e:
            return False, "", str(e)

    def _generate_report(self):
        """Generate validation report."""
        passed_count = sum(1 for r in self.results if r.passed)
        total_count = len(self.results)
        total_duration = sum(r.duration for r in self.results)

        print("\n" + "=" * 60)
        print("VALIDATION REPORT")
        print("=" * 60)
        print(f"Total Checks: {total_count}")
        print(f"Passed: {passed_count}")
        print(f"Failed: {total_count - passed_count}")
        print(f"Total Duration: {total_duration:.2f}s")

        if passed_count == total_count:
            print("\n🎉 All validation checks passed!")
        else:
            print(f"\n⚠️  {total_count - passed_count} validation checks failed:")
            for result in self.results:
                if not result.passed:
                    print(f"  ❌ {result.name}")
                    if result.error:
                        print(f"     Error: {result.error}")

        # Save detailed report
        self._save_detailed_report()

    def _save_detailed_report(self):
        """Save detailed validation report to file."""
        report_dir = self.root_path / ".github" / "artifacts"
        report_dir.mkdir(parents=True, exist_ok=True)

        report_file = report_dir / f"validation-report-{datetime.now().strftime('%Y%m%d-%H%M%S')}.json"

        report_data = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total_checks": len(self.results),
                "passed_checks": sum(1 for r in self.results if r.passed),
                "failed_checks": sum(1 for r in self.results if not r.passed),
                "total_duration": sum(r.duration for r in self.results),
            },
            "results": [{"name": r.name, "passed": r.passed, "duration": r.duration, "output": r.output, "error": r.error} for r in self.results],
        }

        with open(report_file, "w") as f:
            import json

            json.dump(report_data, f, indent=2)

        print(f"\nDetailed report saved to: {report_file}")


def main():
    """Main entry point for validation runner."""
    parser = argparse.ArgumentParser(description="GraphBit Validation Runner", formatter_class=argparse.RawDescriptionHelpFormatter)

    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Root directory of the project")

    parser.add_argument("--check", choices=["formatting", "linting", "security", "dependencies", "versions", "docs", "all"], default="all", help="Specific validation check to run")

    args = parser.parse_args()

    runner = ValidationRunner(args.root)

    if args.check == "all":
        success = runner.run_all_validations()
    else:
        # Run specific check (implementation would be added here)
        print(f"Running specific check: {args.check}")
        success = runner.run_all_validations()  # For now, run all

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
