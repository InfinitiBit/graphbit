#!/usr/bin/env python3
"""
Workflow System Integrity Validator for GraphBit

This script validates the integrity of the modular workflow system,
ensuring all components are properly configured and functional.
"""

import argparse
import json
import os
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

import yaml


class ValidationLevel(Enum):
    """Validation severity levels."""

    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    CRITICAL = "critical"


@dataclass
class ValidationIssue:
    """A validation issue found during checks."""

    level: ValidationLevel
    category: str
    message: str
    file_path: Optional[str] = None
    line_number: Optional[int] = None
    suggestion: Optional[str] = None


class WorkflowValidator:
    """Validates workflow system integrity."""

    def __init__(self, root_path: Path):
        self.root_path = root_path
        self.workflows_dir = root_path / ".github" / "workflows"
        self.scripts_dir = root_path / "scripts"
        self.issues: List[ValidationIssue] = []

        # Expected workflow structure
        self.expected_workflows = {
            "01-validation.yml": {"name": "01 - Validation", "required_jobs": ["validation"], "required_triggers": ["push", "pull_request", "workflow_dispatch"]},
            "02-testing.yml": {"name": "02 - Testing", "required_jobs": ["prerequisites-check", "test-matrix"], "required_triggers": ["workflow_dispatch", "workflow_call"]},
            "03-build.yml": {"name": "03 - Build", "required_jobs": ["prerequisites-check", "build-matrix"], "required_triggers": ["workflow_dispatch", "workflow_call"]},
            "04-release.yml": {"name": "04 - Release", "required_jobs": ["prerequisites-check", "release-process"], "required_triggers": ["workflow_dispatch", "workflow_call"]},
            "05-deployment.yml": {"name": "05 - Deployment", "required_jobs": ["prerequisites-check", "deployment-verification"], "required_triggers": ["workflow_dispatch", "workflow_call"]},
            "pipeline-orchestrator.yml": {"name": "Pipeline Orchestrator", "required_jobs": ["initialize-pipeline", "validation-phase"], "required_triggers": ["push", "workflow_dispatch"]},
        }

        self.expected_scripts = {
            "workflow-orchestrator.py": {"description": "Phase coordination script", "required_functions": ["start", "complete", "status", "reset", "check"]},
            "validation-runner.py": {"description": "Validation execution script", "required_functions": ["run_all_validations"]},
            "testing-runner.py": {"description": "Testing execution script", "required_functions": ["run_all_tests"]},
            "build-runner.py": {"description": "Build execution script", "required_functions": ["run_all_builds"]},
            "verify-version-sync.py": {"description": "Version management script (legacy)", "required_functions": []},  # Legacy script, minimal validation
        }

    def validate_system(self) -> bool:
        """Run comprehensive system validation."""
        print("Starting Workflow System Validation")
        print("=" * 60)

        validation_checks = [
            ("File Structure", self._validate_file_structure),
            ("Workflow Syntax", self._validate_workflow_syntax),
            ("Workflow Configuration", self._validate_workflow_configuration),
            ("Script Functionality", self._validate_script_functionality),
            ("Dependencies", self._validate_dependencies),
            ("Permissions", self._validate_permissions),
            ("Integration Points", self._validate_integration_points),
            ("Security", self._validate_security),
        ]

        for check_name, check_func in validation_checks:
            print(f"\nRunning {check_name} validation...")

            try:
                check_func()
                print(f"[PASS] {check_name} validation completed")
            except Exception as e:
                self.issues.append(ValidationIssue(level=ValidationLevel.CRITICAL, category=check_name, message=f"Validation check failed: {str(e)}"))
                print(f"[FAIL] {check_name} validation failed: {e}")

        self._generate_validation_report()

        # Determine overall result
        critical_issues = [i for i in self.issues if i.level == ValidationLevel.CRITICAL]
        error_issues = [i for i in self.issues if i.level == ValidationLevel.ERROR]

        return len(critical_issues) == 0 and len(error_issues) == 0

    def _validate_file_structure(self):
        """Validate that all required files exist."""
        # Check workflow files
        for workflow_file in self.expected_workflows.keys():
            workflow_path = self.workflows_dir / workflow_file

            if not workflow_path.exists():
                self.issues.append(
                    ValidationIssue(
                        level=ValidationLevel.CRITICAL,
                        category="File Structure",
                        message=f"Required workflow file missing: {workflow_file}",
                        file_path=str(workflow_path),
                        suggestion=f"Create {workflow_file} with proper workflow configuration",
                    )
                )

        # Check script files
        for script_file in self.expected_scripts.keys():
            script_path = self.scripts_dir / script_file

            if not script_path.exists():
                level = ValidationLevel.CRITICAL if script_file != "verify-version-sync.py" else ValidationLevel.WARNING
                self.issues.append(
                    ValidationIssue(
                        level=level,
                        category="File Structure",
                        message=f"Required script file missing: {script_file}",
                        file_path=str(script_path),
                        suggestion=f"Create {script_file} with proper functionality",
                    )
                )

        # Check directory structure
        required_dirs = [self.workflows_dir, self.scripts_dir, self.root_path / ".github" / "artifacts"]

        for required_dir in required_dirs:
            if not required_dir.exists():
                self.issues.append(
                    ValidationIssue(
                        level=ValidationLevel.WARNING,
                        category="File Structure",
                        message=f"Required directory missing: {required_dir.name}",
                        file_path=str(required_dir),
                        suggestion=f"Create directory: {required_dir}",
                    )
                )

    def _validate_workflow_syntax(self):
        """Validate YAML syntax of all workflow files."""
        for workflow_file in self.expected_workflows.keys():
            workflow_path = self.workflows_dir / workflow_file

            if not workflow_path.exists():
                continue  # Already reported in file structure check

            try:
                with open(workflow_path, "r", encoding="utf-8") as f:
                    yaml.safe_load(f)

            except yaml.YAMLError as e:
                self.issues.append(
                    ValidationIssue(
                        level=ValidationLevel.CRITICAL,
                        category="Workflow Syntax",
                        message=f"Invalid YAML syntax in {workflow_file}: {str(e)}",
                        file_path=str(workflow_path),
                        line_number=getattr(e, "problem_mark", {}).get("line", None),
                        suggestion="Fix YAML syntax errors",
                    )
                )
            except Exception as e:
                self.issues.append(ValidationIssue(level=ValidationLevel.ERROR, category="Workflow Syntax", message=f"Could not read {workflow_file}: {str(e)}", file_path=str(workflow_path)))

    def _validate_workflow_configuration(self):
        """Validate workflow configuration and structure."""
        for workflow_file, expected_config in self.expected_workflows.items():
            workflow_path = self.workflows_dir / workflow_file

            if not workflow_path.exists():
                continue

            try:
                with open(workflow_path, "r", encoding="utf-8") as f:
                    workflow_data = yaml.safe_load(f)

                # Check workflow name
                if workflow_data.get("name") != expected_config["name"]:
                    self.issues.append(
                        ValidationIssue(
                            level=ValidationLevel.WARNING,
                            category="Workflow Configuration",
                            message=f"Workflow name mismatch in {workflow_file}",
                            file_path=str(workflow_path),
                            suggestion=f"Expected name: '{expected_config['name']}'",
                        )
                    )

                # Check required triggers
                # YAML parser converts 'on:' to boolean True, so check both
                on_config = workflow_data.get("on", {}) or workflow_data.get(True, {})

                # Handle both string and dict trigger formats
                trigger_names = []
                if isinstance(on_config, dict):
                    trigger_names = list(on_config.keys())
                elif isinstance(on_config, list):
                    trigger_names = on_config

                for required_trigger in expected_config["required_triggers"]:
                    if required_trigger not in trigger_names:
                        self.issues.append(
                            ValidationIssue(
                                level=ValidationLevel.ERROR,
                                category="Workflow Configuration",
                                message=f"Missing required trigger '{required_trigger}' in {workflow_file}",
                                file_path=str(workflow_path),
                                suggestion=f"Add '{required_trigger}' trigger to workflow",
                            )
                        )

                # Check required jobs
                jobs = workflow_data.get("jobs", {})
                for required_job in expected_config["required_jobs"]:
                    if required_job not in jobs:
                        self.issues.append(
                            ValidationIssue(
                                level=ValidationLevel.ERROR,
                                category="Workflow Configuration",
                                message=f"Missing required job '{required_job}' in {workflow_file}",
                                file_path=str(workflow_path),
                                suggestion=f"Add '{required_job}' job to workflow",
                            )
                        )

                # Check permissions
                if "permissions" not in workflow_data:
                    self.issues.append(
                        ValidationIssue(
                            level=ValidationLevel.WARNING,
                            category="Workflow Configuration",
                            message=f"No permissions specified in {workflow_file}",
                            file_path=str(workflow_path),
                            suggestion="Add appropriate permissions block",
                        )
                    )

            except Exception as e:
                self.issues.append(
                    ValidationIssue(
                        level=ValidationLevel.ERROR, category="Workflow Configuration", message=f"Could not validate configuration for {workflow_file}: {str(e)}", file_path=str(workflow_path)
                    )
                )

    def _validate_script_functionality(self):
        """Validate that scripts are functional."""
        for script_file, expected_config in self.expected_scripts.items():
            script_path = self.scripts_dir / script_file

            if not script_path.exists():
                continue

            # Test basic script execution
            try:
                result = subprocess.run([sys.executable, str(script_path), "--help"], capture_output=True, text=True, cwd=self.root_path, timeout=30)

                if result.returncode != 0:
                    self.issues.append(
                        ValidationIssue(
                            level=ValidationLevel.ERROR,
                            category="Script Functionality",
                            message=f"Script {script_file} not functional: {result.stderr}",
                            file_path=str(script_path),
                            suggestion="Fix script execution issues",
                        )
                    )

            except subprocess.TimeoutExpired:
                self.issues.append(
                    ValidationIssue(
                        level=ValidationLevel.ERROR,
                        category="Script Functionality",
                        message=f"Script {script_file} timed out during execution",
                        file_path=str(script_path),
                        suggestion="Fix script performance issues",
                    )
                )
            except Exception as e:
                self.issues.append(ValidationIssue(level=ValidationLevel.ERROR, category="Script Functionality", message=f"Could not test script {script_file}: {str(e)}", file_path=str(script_path)))

    def _validate_dependencies(self):
        """Validate script and workflow dependencies."""
        # Check Python dependencies
        try:
            import requests
            import yaml
        except ImportError as e:
            self.issues.append(
                ValidationIssue(
                    level=ValidationLevel.CRITICAL, category="Dependencies", message=f"Missing Python dependency: {str(e)}", suggestion="Install required Python packages: pip install pyyaml requests"
                )
            )

        # Check if git is available
        try:
            result = subprocess.run(["git", "--version"], capture_output=True, text=True, timeout=10)

            if result.returncode != 0:
                self.issues.append(
                    ValidationIssue(level=ValidationLevel.ERROR, category="Dependencies", message="Git is not available or not functional", suggestion="Install Git and ensure it's in PATH")
                )

        except Exception:
            self.issues.append(ValidationIssue(level=ValidationLevel.ERROR, category="Dependencies", message="Git is not available", suggestion="Install Git and ensure it's in PATH"))

    def _validate_permissions(self):
        """Validate file permissions and access."""
        # Check script executability (on Unix systems)
        if os.name != "nt":  # Not Windows
            for script_file in self.expected_scripts.keys():
                script_path = self.scripts_dir / script_file

                if script_path.exists() and not os.access(script_path, os.X_OK):
                    self.issues.append(
                        ValidationIssue(
                            level=ValidationLevel.WARNING,
                            category="Permissions",
                            message=f"Script {script_file} is not executable",
                            file_path=str(script_path),
                            suggestion=f"Make script executable: chmod +x {script_path}",
                        )
                    )

        # Check directory write permissions
        test_dirs = [self.root_path / ".github" / "artifacts", self.root_path / ".github" / "test-artifacts"]

        for test_dir in test_dirs:
            if test_dir.exists() and not os.access(test_dir, os.W_OK):
                self.issues.append(
                    ValidationIssue(
                        level=ValidationLevel.ERROR, category="Permissions", message=f"Directory {test_dir.name} is not writable", file_path=str(test_dir), suggestion="Fix directory permissions"
                    )
                )

    def _validate_integration_points(self):
        """Validate integration points between workflows."""
        # Check workflow_call configurations
        orchestrator_path = self.workflows_dir / "pipeline-orchestrator.yml"

        if orchestrator_path.exists():
            try:
                with open(orchestrator_path, "r", encoding="utf-8") as f:
                    orchestrator_data = yaml.safe_load(f)

                jobs = orchestrator_data.get("jobs", {})

                # Check that orchestrator calls other workflows
                expected_phases = ["validation-phase", "testing-phase", "build-phase", "release-phase", "deployment-phase"]

                for phase in expected_phases:
                    if phase not in jobs:
                        self.issues.append(
                            ValidationIssue(
                                level=ValidationLevel.ERROR,
                                category="Integration Points",
                                message=f"Orchestrator missing phase: {phase}",
                                file_path=str(orchestrator_path),
                                suggestion=f"Add {phase} job to orchestrator",
                            )
                        )
                    else:
                        job_config = jobs[phase]
                        if "uses" not in job_config:
                            self.issues.append(
                                ValidationIssue(
                                    level=ValidationLevel.ERROR,
                                    category="Integration Points",
                                    message=f"Phase {phase} does not use workflow_call",
                                    file_path=str(orchestrator_path),
                                    suggestion="Add 'uses' directive to call modular workflow",
                                )
                            )

            except Exception as e:
                self.issues.append(
                    ValidationIssue(level=ValidationLevel.ERROR, category="Integration Points", message=f"Could not validate orchestrator integration: {str(e)}", file_path=str(orchestrator_path))
                )

    def _validate_security(self):
        """Validate security configurations."""
        # Check for hardcoded secrets or tokens
        sensitive_patterns = ["token", "password", "secret", "key"]

        for workflow_file in self.expected_workflows.keys():
            workflow_path = self.workflows_dir / workflow_file

            if not workflow_path.exists():
                continue

            try:
                with open(workflow_path, "r", encoding="utf-8") as f:
                    content = f.read().lower()

                for pattern in sensitive_patterns:
                    # More specific check to avoid false positives
                    if f"{pattern}:" in content and "secrets." not in content and "cache" not in content and "github_token" not in content and "${{" not in content:
                        # Additional check for actual hardcoded values
                        lines = content.split("\n")
                        for line in lines:
                            if f"{pattern}:" in line and "=" in line and "secrets." not in line and "${{" not in line:
                                self.issues.append(
                                    ValidationIssue(
                                        level=ValidationLevel.WARNING,
                                        category="Security",
                                        message=f"Potential hardcoded {pattern} in {workflow_file}",
                                        file_path=str(workflow_path),
                                        suggestion="Use GitHub secrets instead of hardcoded values",
                                    )
                                )
                                break

            except Exception:
                pass  # Skip if file can't be read

        # Check permissions are not overly broad
        for workflow_file in self.expected_workflows.keys():
            workflow_path = self.workflows_dir / workflow_file

            if not workflow_path.exists():
                continue

            try:
                with open(workflow_path, "r", encoding="utf-8") as f:
                    workflow_data = yaml.safe_load(f)

                permissions = workflow_data.get("permissions", {})

                if permissions == "write-all" or permissions.get("contents") == "write":
                    # Only release workflow should have write permissions
                    if workflow_file not in ["04-release.yml", "pipeline-orchestrator.yml"]:
                        self.issues.append(
                            ValidationIssue(
                                level=ValidationLevel.WARNING,
                                category="Security",
                                message=f"Overly broad permissions in {workflow_file}",
                                file_path=str(workflow_path),
                                suggestion="Use minimal required permissions",
                            )
                        )

            except Exception:
                pass  # Skip if file can't be parsed

    def _generate_validation_report(self):
        """Generate comprehensive validation report."""
        critical_count = sum(1 for i in self.issues if i.level == ValidationLevel.CRITICAL)
        error_count = sum(1 for i in self.issues if i.level == ValidationLevel.ERROR)
        warning_count = sum(1 for i in self.issues if i.level == ValidationLevel.WARNING)
        info_count = sum(1 for i in self.issues if i.level == ValidationLevel.INFO)
        total_count = len(self.issues)

        print("\n" + "=" * 80)
        print("WORKFLOW SYSTEM VALIDATION REPORT")
        print("=" * 80)
        print(f"Total Issues Found: {total_count}")
        print(f"Critical: {critical_count}")
        print(f"Errors: {error_count}")
        print(f"Warnings: {warning_count}")
        print(f"Info: {info_count}")

        if total_count == 0:
            print("\n[SUCCESS] No issues found! The workflow system is properly configured.")
        else:
            # Group issues by category
            issues_by_category = {}
            for issue in self.issues:
                if issue.category not in issues_by_category:
                    issues_by_category[issue.category] = []
                issues_by_category[issue.category].append(issue)

            for category, category_issues in issues_by_category.items():
                print(f"\n[{category.upper()}] ({len(category_issues)} issues):")

                for issue in category_issues:
                    level_icon = {ValidationLevel.CRITICAL: "[CRITICAL]", ValidationLevel.ERROR: "[ERROR]", ValidationLevel.WARNING: "[WARNING]", ValidationLevel.INFO: "[INFO]"}[issue.level]

                    print(f"  {level_icon} {issue.message}")

                    if issue.file_path:
                        file_info = issue.file_path
                        if issue.line_number:
                            file_info += f":{issue.line_number}"
                        print(f"      File: {file_info}")

                    if issue.suggestion:
                        print(f"      Suggestion: {issue.suggestion}")

        # Save detailed report
        self._save_validation_report()

        # Provide summary
        if critical_count > 0:
            print(f"\n[CRITICAL] {critical_count} critical issues must be fixed before deployment")
        elif error_count > 0:
            print(f"\n[ERRORS] {error_count} errors should be fixed before deployment")
        elif warning_count > 0:
            print(f"\n[WARNINGS] {warning_count} warnings should be reviewed")
        else:
            print("\n[VALIDATION PASSED] System is ready for deployment")

    def _save_validation_report(self):
        """Save detailed validation report to file."""
        report_dir = self.root_path / ".github" / "validation-reports"
        report_dir.mkdir(parents=True, exist_ok=True)

        report_file = report_dir / f"validation-report-{datetime.now().strftime('%Y%m%d-%H%M%S')}.json"

        report_data = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total_issues": len(self.issues),
                "critical_issues": sum(1 for i in self.issues if i.level == ValidationLevel.CRITICAL),
                "error_issues": sum(1 for i in self.issues if i.level == ValidationLevel.ERROR),
                "warning_issues": sum(1 for i in self.issues if i.level == ValidationLevel.WARNING),
                "info_issues": sum(1 for i in self.issues if i.level == ValidationLevel.INFO),
            },
            "issues": [
                {"level": issue.level.value, "category": issue.category, "message": issue.message, "file_path": issue.file_path, "line_number": issue.line_number, "suggestion": issue.suggestion}
                for issue in self.issues
            ],
        }

        with open(report_file, "w") as f:
            json.dump(report_data, f, indent=2)

        print(f"\nDetailed validation report saved to: {report_file}")


def main():
    """Main entry point for workflow validator."""
    parser = argparse.ArgumentParser(description="GraphBit Workflow System Validator", formatter_class=argparse.RawDescriptionHelpFormatter)

    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Root directory of the project")

    parser.add_argument("--level", choices=["info", "warning", "error", "critical"], default="warning", help="Minimum validation level to report")

    args = parser.parse_args()

    validator = WorkflowValidator(args.root)
    success = validator.validate_system()

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
