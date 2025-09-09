#!/usr/bin/env python3
"""
Deployment Readiness Checker for GraphBit Modular Workflows

This script performs final validation before deploying the modular workflow system,
ensuring all components are ready for production use.
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


class ReadinessLevel(Enum):
    """Deployment readiness levels."""

    READY = "ready"
    WARNING = "warning"
    NOT_READY = "not_ready"
    CRITICAL = "critical"


@dataclass
class ReadinessCheck:
    """A deployment readiness check."""

    name: str
    description: str
    level: ReadinessLevel
    passed: bool
    details: str
    suggestion: Optional[str] = None


class DeploymentReadinessChecker:
    """Checks if the modular workflow system is ready for deployment."""

    def __init__(self, root_path: Path):
        self.root_path = root_path
        self.checks: List[ReadinessCheck] = []

        # Paths
        self.workflows_dir = root_path / ".github" / "workflows"
        self.scripts_dir = root_path / "scripts"
        self.backup_dir = root_path / ".github" / "workflows" / "legacy-backup"

    def check_deployment_readiness(self) -> bool:
        """Perform comprehensive deployment readiness check."""
        print("🚀 Checking Deployment Readiness")
        print("=" * 60)

        readiness_checks = [
            ("Modular Workflows", self._check_modular_workflows),
            ("Legacy Workflows", self._check_legacy_workflows),
            ("Script Dependencies", self._check_script_dependencies),
            ("Test Results", self._check_test_results),
            ("Configuration", self._check_configuration),
            ("Security", self._check_security),
            ("Documentation", self._check_documentation),
            ("Backup Status", self._check_backup_status),
        ]

        for check_name, check_func in readiness_checks:
            print(f"\n🔍 Checking {check_name}...")

            try:
                check_func()
                print(f"✅ {check_name} check completed")
            except Exception as e:
                self.checks.append(
                    ReadinessCheck(
                        name=check_name,
                        description=f"Failed to run {check_name} check",
                        level=ReadinessLevel.CRITICAL,
                        passed=False,
                        details=str(e),
                        suggestion="Fix the underlying issue and re-run the check",
                    )
                )
                print(f"❌ {check_name} check failed: {e}")

        return self._generate_readiness_report()

    def _check_modular_workflows(self):
        """Check that all modular workflows are present and valid."""
        required_workflows = ["01-validation.yml", "02-testing.yml", "03-build.yml", "04-release.yml", "05-deployment.yml", "pipeline-orchestrator.yml"]

        missing_workflows = []
        invalid_workflows = []

        for workflow in required_workflows:
            workflow_path = self.workflows_dir / workflow

            if not workflow_path.exists():
                missing_workflows.append(workflow)
                continue

            # Basic YAML validation
            try:
                import yaml

                with open(workflow_path, "r", encoding="utf-8") as f:
                    yaml.safe_load(f)
            except Exception as e:
                invalid_workflows.append(f"{workflow}: {str(e)}")

        if missing_workflows:
            self.checks.append(
                ReadinessCheck(
                    name="Missing Workflows",
                    description="Required modular workflow files are missing",
                    level=ReadinessLevel.CRITICAL,
                    passed=False,
                    details=f"Missing: {', '.join(missing_workflows)}",
                    suggestion="Ensure all modular workflow files are present",
                )
            )

        if invalid_workflows:
            self.checks.append(
                ReadinessCheck(
                    name="Invalid Workflows",
                    description="Some workflow files have syntax errors",
                    level=ReadinessLevel.CRITICAL,
                    passed=False,
                    details=f"Invalid: {', '.join(invalid_workflows)}",
                    suggestion="Fix YAML syntax errors in workflow files",
                )
            )

        if not missing_workflows and not invalid_workflows:
            self.checks.append(
                ReadinessCheck(
                    name="Modular Workflows",
                    description="All modular workflow files are present and valid",
                    level=ReadinessLevel.READY,
                    passed=True,
                    details=f"Found {len(required_workflows)} valid workflow files",
                )
            )

    def _check_legacy_workflows(self):
        """Check that legacy workflows are properly disabled."""
        legacy_workflows = ["ci.yml", "release.yml"]
        active_legacy = []
        disabled_legacy = []
        backed_up_legacy = []

        for legacy in legacy_workflows:
            active_path = self.workflows_dir / legacy
            disabled_path = self.workflows_dir / f"{legacy}.disabled"
            backup_path = self.backup_dir / f"{legacy}.backup"

            if active_path.exists():
                active_legacy.append(legacy)
            elif disabled_path.exists():
                disabled_legacy.append(legacy)

            if backup_path.exists() or (self.backup_dir.exists() and any(self.backup_dir.glob(f"{legacy}*"))):
                backed_up_legacy.append(legacy)

        if active_legacy:
            self.checks.append(
                ReadinessCheck(
                    name="Active Legacy Workflows",
                    description="Legacy workflows are still active and may conflict",
                    level=ReadinessLevel.CRITICAL,
                    passed=False,
                    details=f"Active legacy workflows: {', '.join(active_legacy)}",
                    suggestion="Disable legacy workflows by renaming them to .disabled",
                )
            )
        else:
            self.checks.append(
                ReadinessCheck(
                    name="Legacy Workflows Disabled", description="Legacy workflows are properly disabled", level=ReadinessLevel.READY, passed=True, details=f"Disabled: {', '.join(disabled_legacy)}"
                )
            )

        if not backed_up_legacy:
            self.checks.append(
                ReadinessCheck(
                    name="Legacy Backup Missing",
                    description="Legacy workflows are not backed up",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details="No backup files found for legacy workflows",
                    suggestion="Create backups of legacy workflows before deployment",
                )
            )
        else:
            self.checks.append(
                ReadinessCheck(
                    name="Legacy Workflows Backed Up", description="Legacy workflows are safely backed up", level=ReadinessLevel.READY, passed=True, details=f"Backed up: {', '.join(backed_up_legacy)}"
                )
            )

    def _check_script_dependencies(self):
        """Check that all required scripts are functional."""
        required_scripts = ["workflow-orchestrator.py", "validation-runner.py", "testing-runner.py", "build-runner.py"]

        missing_scripts = []
        broken_scripts = []
        working_scripts = []

        for script in required_scripts:
            script_path = self.scripts_dir / script

            if not script_path.exists():
                missing_scripts.append(script)
                continue

            # Test script execution
            try:
                result = subprocess.run([sys.executable, str(script_path), "--help"], capture_output=True, text=True, cwd=self.root_path, timeout=30)

                if result.returncode == 0:
                    working_scripts.append(script)
                else:
                    broken_scripts.append(script)

            except Exception:
                broken_scripts.append(script)

        if missing_scripts:
            self.checks.append(
                ReadinessCheck(
                    name="Missing Scripts",
                    description="Required scripts are missing",
                    level=ReadinessLevel.CRITICAL,
                    passed=False,
                    details=f"Missing: {', '.join(missing_scripts)}",
                    suggestion="Ensure all required scripts are present",
                )
            )

        if broken_scripts:
            self.checks.append(
                ReadinessCheck(
                    name="Broken Scripts",
                    description="Some scripts are not functional",
                    level=ReadinessLevel.CRITICAL,
                    passed=False,
                    details=f"Broken: {', '.join(broken_scripts)}",
                    suggestion="Fix script execution issues",
                )
            )

        if working_scripts and not missing_scripts and not broken_scripts:
            self.checks.append(
                ReadinessCheck(
                    name="Script Dependencies", description="All required scripts are functional", level=ReadinessLevel.READY, passed=True, details=f"Working scripts: {len(working_scripts)}"
                )
            )

    def _check_test_results(self):
        """Check if comprehensive tests have been run and passed."""
        test_artifacts_dir = self.root_path / ".github" / "test-suite-artifacts"

        if not test_artifacts_dir.exists():
            self.checks.append(
                ReadinessCheck(
                    name="No Test Results",
                    description="No test results found",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details="Test suite has not been run",
                    suggestion="Run comprehensive test suite before deployment",
                )
            )
            return

        # Look for recent test reports
        test_reports = list(test_artifacts_dir.glob("test-suite-report-*.json"))

        if not test_reports:
            self.checks.append(
                ReadinessCheck(
                    name="No Test Reports",
                    description="No test reports found",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details="No test suite reports available",
                    suggestion="Run comprehensive test suite and generate reports",
                )
            )
            return

        # Check the most recent test report
        latest_report = max(test_reports, key=lambda p: p.stat().st_mtime)

        try:
            with open(latest_report, "r") as f:
                report_data = json.load(f)

            summary = report_data.get("summary", {})
            success_rate = summary.get("success_rate", 0)
            failed_suites = summary.get("failed_suites", 0)

            if success_rate >= 100:
                self.checks.append(ReadinessCheck(name="Test Results", description="All tests passed successfully", level=ReadinessLevel.READY, passed=True, details=f"Success rate: {success_rate}%"))
            elif success_rate >= 80:
                self.checks.append(
                    ReadinessCheck(
                        name="Test Results",
                        description="Most tests passed with some warnings",
                        level=ReadinessLevel.WARNING,
                        passed=False,
                        details=f"Success rate: {success_rate}%, Failed suites: {failed_suites}",
                        suggestion="Review and fix failing test suites",
                    )
                )
            else:
                self.checks.append(
                    ReadinessCheck(
                        name="Test Results",
                        description="Significant test failures detected",
                        level=ReadinessLevel.NOT_READY,
                        passed=False,
                        details=f"Success rate: {success_rate}%, Failed suites: {failed_suites}",
                        suggestion="Fix failing tests before deployment",
                    )
                )

        except Exception as e:
            self.checks.append(
                ReadinessCheck(
                    name="Test Report Error",
                    description="Could not read test report",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details=str(e),
                    suggestion="Re-run test suite to generate valid report",
                )
            )

    def _check_configuration(self):
        """Check workflow configuration and settings."""
        # Check environment variables
        required_env_vars = ["GITHUB_TOKEN"]
        missing_env_vars = []

        for env_var in required_env_vars:
            if not os.environ.get(env_var):
                missing_env_vars.append(env_var)

        if missing_env_vars:
            self.checks.append(
                ReadinessCheck(
                    name="Environment Variables",
                    description="Required environment variables are missing",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details=f"Missing: {', '.join(missing_env_vars)}",
                    suggestion="Set required environment variables for workflow execution",
                )
            )
        else:
            self.checks.append(
                ReadinessCheck(
                    name="Environment Variables",
                    description="Required environment variables are configured",
                    level=ReadinessLevel.READY,
                    passed=True,
                    details="All required environment variables are set",
                )
            )

        # Check git configuration
        try:
            result = subprocess.run(["git", "remote", "get-url", "origin"], capture_output=True, text=True, cwd=self.root_path, timeout=30)

            if result.returncode == 0 and "github.com" in result.stdout:
                self.checks.append(
                    ReadinessCheck(name="Git Configuration", description="Git repository is properly configured", level=ReadinessLevel.READY, passed=True, details="GitHub repository detected")
                )
            else:
                self.checks.append(
                    ReadinessCheck(
                        name="Git Configuration",
                        description="Git repository configuration issue",
                        level=ReadinessLevel.WARNING,
                        passed=False,
                        details="Could not detect GitHub repository",
                        suggestion="Ensure repository is connected to GitHub",
                    )
                )

        except Exception:
            self.checks.append(
                ReadinessCheck(
                    name="Git Configuration",
                    description="Git is not available or configured",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details="Git command failed",
                    suggestion="Ensure Git is installed and repository is initialized",
                )
            )

    def _check_security(self):
        """Check security configurations."""
        # Check for hardcoded secrets (basic check)
        security_issues = []

        for workflow_file in ["01-validation.yml", "02-testing.yml", "03-build.yml", "04-release.yml", "05-deployment.yml", "pipeline-orchestrator.yml"]:
            workflow_path = self.workflows_dir / workflow_file

            if workflow_path.exists():
                try:
                    with open(workflow_path, "r", encoding="utf-8") as f:
                        content = f.read().lower()

                    # Look for potential hardcoded secrets
                    if "token:" in content and "secrets." not in content:
                        security_issues.append(f"Potential hardcoded token in {workflow_file}")

                except Exception:
                    pass

        if security_issues:
            self.checks.append(
                ReadinessCheck(
                    name="Security Issues",
                    description="Potential security issues detected",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details=f"Issues: {', '.join(security_issues)}",
                    suggestion="Review and fix potential security issues",
                )
            )
        else:
            self.checks.append(
                ReadinessCheck(name="Security Check", description="No obvious security issues detected", level=ReadinessLevel.READY, passed=True, details="Basic security checks passed")
            )

    def _check_documentation(self):
        """Check documentation completeness."""
        required_docs = [self.workflows_dir / "README.md", self.root_path / "WORKFLOW_MIGRATION_GUIDE.md"]

        missing_docs = []
        present_docs = []

        for doc_path in required_docs:
            if doc_path.exists():
                present_docs.append(doc_path.name)
            else:
                missing_docs.append(doc_path.name)

        if missing_docs:
            self.checks.append(
                ReadinessCheck(
                    name="Documentation",
                    description="Some documentation is missing",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details=f"Missing: {', '.join(missing_docs)}",
                    suggestion="Create missing documentation files",
                )
            )
        else:
            self.checks.append(
                ReadinessCheck(name="Documentation", description="Required documentation is present", level=ReadinessLevel.READY, passed=True, details=f"Present: {', '.join(present_docs)}")
            )

    def _check_backup_status(self):
        """Check backup and rollback capabilities."""
        if not self.backup_dir.exists():
            self.checks.append(
                ReadinessCheck(
                    name="Backup Directory",
                    description="Backup directory does not exist",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details="No backup directory found",
                    suggestion="Create backup directory and backup legacy workflows",
                )
            )
            return

        backup_files = list(self.backup_dir.glob("*.backup*"))

        if not backup_files:
            self.checks.append(
                ReadinessCheck(
                    name="Backup Files",
                    description="No backup files found",
                    level=ReadinessLevel.WARNING,
                    passed=False,
                    details="Backup directory exists but is empty",
                    suggestion="Create backups of legacy workflows",
                )
            )
        else:
            self.checks.append(
                ReadinessCheck(name="Backup Status", description="Backup files are available for rollback", level=ReadinessLevel.READY, passed=True, details=f"Found {len(backup_files)} backup files")
            )

    def _generate_readiness_report(self) -> bool:
        """Generate deployment readiness report."""
        ready_count = sum(1 for c in self.checks if c.level == ReadinessLevel.READY and c.passed)
        warning_count = sum(1 for c in self.checks if c.level == ReadinessLevel.WARNING or (c.level == ReadinessLevel.READY and not c.passed))
        not_ready_count = sum(1 for c in self.checks if c.level == ReadinessLevel.NOT_READY)
        critical_count = sum(1 for c in self.checks if c.level == ReadinessLevel.CRITICAL)

        total_count = len(self.checks)

        print("\n" + "=" * 80)
        print("DEPLOYMENT READINESS REPORT")
        print("=" * 80)
        print(f"Total Checks: {total_count}")
        print(f"Ready: {ready_count}")
        print(f"Warnings: {warning_count}")
        print(f"Not Ready: {not_ready_count}")
        print(f"Critical Issues: {critical_count}")

        # Determine overall readiness
        if critical_count > 0:
            overall_status = "🔴 NOT READY FOR DEPLOYMENT"
            deployment_ready = False
        elif not_ready_count > 0:
            overall_status = "🟡 NOT READY FOR DEPLOYMENT"
            deployment_ready = False
        elif warning_count > 0:
            overall_status = "🟡 READY WITH WARNINGS"
            deployment_ready = True
        else:
            overall_status = "🟢 READY FOR DEPLOYMENT"
            deployment_ready = True

        print(f"\nOverall Status: {overall_status}")

        # Show detailed results
        if self.checks:
            print(f"\n📋 Detailed Results:")

            for check in self.checks:
                if check.level == ReadinessLevel.CRITICAL:
                    icon = "🔴"
                elif check.level == ReadinessLevel.NOT_READY:
                    icon = "❌"
                elif check.level == ReadinessLevel.WARNING:
                    icon = "⚠️"
                else:
                    icon = "✅" if check.passed else "⚠️"

                print(f"  {icon} {check.name}: {check.description}")
                print(f"      {check.details}")

                if check.suggestion:
                    print(f"      Suggestion: {check.suggestion}")

        # Provide next steps
        if deployment_ready:
            print(f"\n🎉 DEPLOYMENT APPROVED!")
            print(f"\n📋 Next Steps:")
            print("1. Commit and push the modular workflow changes")
            print("2. Update branch protection rules to use new workflow names")
            print("3. Monitor the first few workflow runs carefully")
            print("4. Keep legacy backups until confident in new system")

            if warning_count > 0:
                print(f"\n⚠️  Address {warning_count} warnings when possible")
        else:
            print(f"\n🛑 DEPLOYMENT BLOCKED!")
            print(f"\n📋 Required Actions:")

            for check in self.checks:
                if check.level in [ReadinessLevel.CRITICAL, ReadinessLevel.NOT_READY]:
                    print(f"• Fix: {check.name}")
                    if check.suggestion:
                        print(f"  Action: {check.suggestion}")

        # Save report
        self._save_readiness_report(deployment_ready)

        return deployment_ready

    def _save_readiness_report(self, deployment_ready: bool):
        """Save deployment readiness report."""
        report_dir = self.root_path / ".github" / "deployment-readiness"
        report_dir.mkdir(parents=True, exist_ok=True)

        report_file = report_dir / f"readiness-report-{datetime.now().strftime('%Y%m%d-%H%M%S')}.json"

        report_data = {
            "timestamp": datetime.now().isoformat(),
            "deployment_ready": deployment_ready,
            "summary": {
                "total_checks": len(self.checks),
                "ready_checks": sum(1 for c in self.checks if c.level == ReadinessLevel.READY and c.passed),
                "warning_checks": sum(1 for c in self.checks if c.level == ReadinessLevel.WARNING or (c.level == ReadinessLevel.READY and not c.passed)),
                "not_ready_checks": sum(1 for c in self.checks if c.level == ReadinessLevel.NOT_READY),
                "critical_checks": sum(1 for c in self.checks if c.level == ReadinessLevel.CRITICAL),
            },
            "checks": [{"name": c.name, "description": c.description, "level": c.level.value, "passed": c.passed, "details": c.details, "suggestion": c.suggestion} for c in self.checks],
        }

        with open(report_file, "w") as f:
            json.dump(report_data, f, indent=2)

        print(f"\nDetailed readiness report saved to: {report_file}")


def main():
    """Main entry point for deployment readiness checker."""
    parser = argparse.ArgumentParser(description="GraphBit Deployment Readiness Checker", formatter_class=argparse.RawDescriptionHelpFormatter)

    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Root directory of the project")

    args = parser.parse_args()

    checker = DeploymentReadinessChecker(args.root)
    ready = checker.check_deployment_readiness()

    sys.exit(0 if ready else 1)


if __name__ == "__main__":
    main()
