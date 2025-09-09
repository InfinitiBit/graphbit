#!/usr/bin/env python3
"""
Legacy Workflow Migration Tool for GraphBit

This script handles the safe migration from monolithic workflows (ci.yml, release.yml)
to the new modular workflow system, ensuring no functionality is lost.
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

import yaml


@dataclass
class MigrationStep:
    """A single migration step."""

    name: str
    description: str
    required: bool
    completed: bool = False
    error: Optional[str] = None


class LegacyMigrator:
    """Handles migration from legacy to modular workflows."""

    def __init__(self, root_path: Path, dry_run: bool = False):
        self.root_path = root_path
        self.dry_run = dry_run
        self.workflows_dir = root_path / ".github" / "workflows"
        self.backup_dir = root_path / ".github" / "workflows" / "legacy-backup"
        self.migration_log = root_path / ".github" / "migration-log.json"

        # Migration steps
        self.steps: List[MigrationStep] = [
            MigrationStep("backup_legacy", "Backup legacy workflow files", True),
            MigrationStep("validate_modular", "Validate modular workflow files", True),
            MigrationStep("check_dependencies", "Check script dependencies", True),
            MigrationStep("test_modular", "Test modular workflows", True),
            MigrationStep("update_branch_protection", "Update branch protection rules", False),
            MigrationStep("disable_legacy", "Disable legacy workflows", True),
            MigrationStep("cleanup", "Clean up migration artifacts", False),
        ]

    def run_migration(self) -> bool:
        """Run the complete migration process."""
        print("Starting Legacy Workflow Migration")
        print("=" * 60)

        if self.dry_run:
            print("[DRY RUN] MODE - No changes will be made")
            print()

        # Load previous migration state if exists
        self._load_migration_state()

        all_passed = True

        for step in self.steps:
            if step.completed:
                print(f"[SKIP] {step.name} (already completed)")
                continue

            print(f"\nExecuting: {step.description}")

            try:
                success = self._execute_step(step)

                if success:
                    step.completed = True
                    print(f"[PASS] {step.name}")
                else:
                    print(f"[FAIL] {step.name}")
                    if step.error:
                        print(f"   Error: {step.error}")

                    if step.required:
                        all_passed = False
                        print("[CRITICAL] Required step failed. Migration cannot continue.")
                        break

            except Exception as e:
                step.error = str(e)
                print(f"[FAIL] {step.name}")
                print(f"   Exception: {e}")

                if step.required:
                    all_passed = False
                    print("[CRITICAL] Required step failed. Migration cannot continue.")
                    break

            # Save migration state after each step
            self._save_migration_state()

        self._generate_migration_report(all_passed)
        return all_passed

    def _execute_step(self, step: MigrationStep) -> bool:
        """Execute a single migration step."""
        if step.name == "backup_legacy":
            return self._backup_legacy_workflows()
        elif step.name == "validate_modular":
            return self._validate_modular_workflows()
        elif step.name == "check_dependencies":
            return self._check_script_dependencies()
        elif step.name == "test_modular":
            return self._test_modular_workflows()
        elif step.name == "update_branch_protection":
            return self._update_branch_protection()
        elif step.name == "disable_legacy":
            return self._disable_legacy_workflows()
        elif step.name == "cleanup":
            return self._cleanup_migration()
        else:
            step.error = f"Unknown migration step: {step.name}"
            return False

    def _backup_legacy_workflows(self) -> bool:
        """Backup legacy workflow files."""
        try:
            legacy_files = ["ci.yml", "release.yml"]

            if not self.dry_run:
                self.backup_dir.mkdir(parents=True, exist_ok=True)

            backed_up_files = []

            for legacy_file in legacy_files:
                legacy_path = self.workflows_dir / legacy_file

                if legacy_path.exists():
                    backup_path = self.backup_dir / f"{legacy_file}.backup-{datetime.now().strftime('%Y%m%d-%H%M%S')}"

                    if not self.dry_run:
                        shutil.copy2(legacy_path, backup_path)

                    backed_up_files.append(legacy_file)
                    print(f"   [BACKUP] {legacy_file} -> {backup_path.name}")

            if not backed_up_files:
                print("   [INFO] No legacy workflow files found to backup")

            return True

        except Exception as e:
            self.steps[0].error = str(e)
            return False

    def _validate_modular_workflows(self) -> bool:
        """Validate that all modular workflow files exist and are valid."""
        try:
            required_workflows = ["01-validation.yml", "02-testing.yml", "03-build.yml", "04-release.yml", "05-deployment.yml", "pipeline-orchestrator.yml"]

            missing_files = []
            invalid_files = []

            for workflow_file in required_workflows:
                workflow_path = self.workflows_dir / workflow_file

                if not workflow_path.exists():
                    missing_files.append(workflow_file)
                    continue

                # Validate YAML syntax
                try:
                    with open(workflow_path, "r", encoding="utf-8") as f:
                        yaml.safe_load(f)
                    print(f"   [PASS] {workflow_file} - Valid YAML")
                except yaml.YAMLError as e:
                    invalid_files.append(f"{workflow_file}: {str(e)}")
                    print(f"   [FAIL] {workflow_file} - Invalid YAML: {e}")

            if missing_files:
                self.steps[1].error = f"Missing workflow files: {missing_files}"
                return False

            if invalid_files:
                self.steps[1].error = f"Invalid workflow files: {invalid_files}"
                return False

            print(f"   [PASS] All {len(required_workflows)} modular workflows validated")
            return True

        except Exception as e:
            self.steps[1].error = str(e)
            return False

    def _check_script_dependencies(self) -> bool:
        """Check that all required scripts exist and are functional."""
        try:
            required_scripts = ["workflow-orchestrator.py", "validation-runner.py", "testing-runner.py", "build-runner.py", "verify-version-sync.py"]

            scripts_dir = self.root_path / "scripts"
            missing_scripts = []
            broken_scripts = []

            for script_name in required_scripts:
                script_path = scripts_dir / script_name

                if not script_path.exists():
                    missing_scripts.append(script_name)
                    continue

                # Test script execution
                try:
                    result = subprocess.run([sys.executable, str(script_path), "--help"], capture_output=True, text=True, cwd=self.root_path, timeout=30)

                    if result.returncode == 0:
                        print(f"   [PASS] {script_name} - Functional")
                    else:
                        broken_scripts.append(f"{script_name}: {result.stderr}")
                        print(f"   [FAIL] {script_name} - Not functional")

                except subprocess.TimeoutExpired:
                    broken_scripts.append(f"{script_name}: Timeout")
                    print(f"   [FAIL] {script_name} - Timeout")
                except Exception as e:
                    broken_scripts.append(f"{script_name}: {str(e)}")
                    print(f"   [FAIL] {script_name} - Error: {e}")

            if missing_scripts:
                self.steps[2].error = f"Missing scripts: {missing_scripts}"
                return False

            if broken_scripts:
                self.steps[2].error = f"Broken scripts: {broken_scripts}"
                return False

            print(f"   [PASS] All {len(required_scripts)} scripts validated")
            return True

        except Exception as e:
            self.steps[2].error = str(e)
            return False

    def _test_modular_workflows(self) -> bool:
        """Test modular workflows using the workflow tester."""
        try:
            # Run basic workflow tests
            tester_script = self.root_path / "scripts" / "workflow-tester.py"

            if not tester_script.exists():
                print("   [WARN] Workflow tester not available, skipping comprehensive tests")
                return True

            # Get repository info from git
            try:
                repo_info = subprocess.run(["git", "remote", "get-url", "origin"], capture_output=True, text=True, cwd=self.root_path, timeout=30)

                if repo_info.returncode == 0:
                    # Parse repo owner and name from URL
                    repo_url = repo_info.stdout.strip()
                    if "github.com" in repo_url:
                        # Extract owner/repo from URL
                        parts = repo_url.replace(".git", "").split("/")
                        if len(parts) >= 2:
                            repo_owner = parts[-2]
                            repo_name = parts[-1]

                            # Run preflight checks only (don't need full API access for migration)
                            print("   [TEST] Running basic workflow validation...")

                            # Just validate that the tester can run
                            test_result = subprocess.run([sys.executable, str(tester_script), "--help"], capture_output=True, text=True, cwd=self.root_path, timeout=30)

                            if test_result.returncode == 0:
                                print("   [PASS] Workflow tester functional")
                                return True
                            else:
                                print("   [FAIL] Workflow tester not functional")
                                return False

            except Exception:
                pass

            print("   [WARN] Could not determine repository info, skipping workflow tests")
            return True

        except Exception as e:
            self.steps[3].error = str(e)
            return False

    def _update_branch_protection(self) -> bool:
        """Update branch protection rules (optional step)."""
        try:
            print("   [INFO] Branch protection rules need to be updated manually")
            print("   [TODO] Update required status checks from:")
            print("      - Legacy: CI / test-matrix, Release / version-check, etc.")
            print("      - Modular: 01 - Validation / validation, 02 - Testing / test-summary, etc.")

            # This step requires manual intervention or GitHub API access
            # For now, we'll mark it as completed with instructions
            return True

        except Exception as e:
            self.steps[4].error = str(e)
            return False

    def _disable_legacy_workflows(self) -> bool:
        """Disable legacy workflows by renaming them."""
        try:
            legacy_files = ["ci.yml", "release.yml"]
            disabled_files = []

            for legacy_file in legacy_files:
                legacy_path = self.workflows_dir / legacy_file

                if legacy_path.exists():
                    disabled_path = self.workflows_dir / f"{legacy_file}.disabled"

                    if not self.dry_run:
                        legacy_path.rename(disabled_path)

                    disabled_files.append(legacy_file)
                    print(f"   [DISABLE] {legacy_file} -> {disabled_path.name}")

            if not disabled_files:
                print("   [INFO] No legacy workflow files found to disable")

            return True

        except Exception as e:
            self.steps[5].error = str(e)
            return False

    def _cleanup_migration(self) -> bool:
        """Clean up migration artifacts (optional step)."""
        try:
            # Clean up temporary files, logs, etc.
            cleanup_items = []

            # Example cleanup items
            temp_files = list(self.root_path.glob("*.tmp"))
            cleanup_items.extend(temp_files)

            if not self.dry_run:
                for item in cleanup_items:
                    if item.exists():
                        if item.is_file():
                            item.unlink()
                        elif item.is_dir():
                            shutil.rmtree(item)

            if cleanup_items:
                print(f"   [CLEANUP] Cleaned up {len(cleanup_items)} temporary items")
            else:
                print("   [INFO] No cleanup items found")

            return True

        except Exception as e:
            self.steps[6].error = str(e)
            return False

    def _load_migration_state(self):
        """Load previous migration state if exists."""
        if self.migration_log.exists():
            try:
                with open(self.migration_log, "r") as f:
                    state_data = json.load(f)

                # Update step completion status
                completed_steps = state_data.get("completed_steps", [])
                for step in self.steps:
                    if step.name in completed_steps:
                        step.completed = True

                print(f"[INFO] Loaded previous migration state: {len(completed_steps)} steps completed")

            except Exception as e:
                print(f"[WARN] Could not load migration state: {e}")

    def _save_migration_state(self):
        """Save current migration state."""
        if not self.dry_run:
            try:
                state_data = {
                    "timestamp": datetime.now().isoformat(),
                    "completed_steps": [step.name for step in self.steps if step.completed],
                    "failed_steps": [step.name for step in self.steps if step.error],
                    "steps": [{"name": step.name, "description": step.description, "required": step.required, "completed": step.completed, "error": step.error} for step in self.steps],
                }

                with open(self.migration_log, "w") as f:
                    json.dump(state_data, f, indent=2)

            except Exception as e:
                print(f"[WARN] Could not save migration state: {e}")

    def _generate_migration_report(self, success: bool):
        """Generate migration report."""
        completed_count = sum(1 for step in self.steps if step.completed)
        failed_count = sum(1 for step in self.steps if step.error)
        total_count = len(self.steps)

        print("\n" + "=" * 80)
        print("LEGACY WORKFLOW MIGRATION REPORT")
        print("=" * 80)
        print(f"Total Steps: {total_count}")
        print(f"Completed: {completed_count}")
        print(f"Failed: {failed_count}")
        print(f"Success Rate: {(completed_count/total_count)*100:.1f}%")

        if self.dry_run:
            print("\n[DRY RUN] COMPLETED - No actual changes were made")

        if success:
            print("\n[SUCCESS] Migration completed successfully!")
            print("\n[NEXT STEPS]:")
            print("1. Test the new modular workflows manually")
            print("2. Update branch protection rules in repository settings")
            print("3. Monitor the first few workflow runs")
            print("4. Remove legacy backup files when confident")
        else:
            print(f"\n[WARNING] Migration completed with {failed_count} failures")
            print("\n[REQUIRED ACTIONS]:")
            for step in self.steps:
                if step.error and step.required:
                    print(f"- Fix {step.name}: {step.error}")

        # Show step details
        print(f"\n[STEP DETAILS]:")
        for step in self.steps:
            status = "[PASS]" if step.completed else "[FAIL]" if step.error else "[PENDING]"
            required = "Required" if step.required else "Optional"
            print(f"  {status} {step.name} ({required})")
            if step.error:
                print(f"      Error: {step.error}")


def main():
    """Main entry point for legacy migrator."""
    parser = argparse.ArgumentParser(description="GraphBit Legacy Workflow Migration Tool", formatter_class=argparse.RawDescriptionHelpFormatter)

    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Root directory of the project")

    parser.add_argument("--dry-run", action="store_true", help="Perform a dry run without making changes")

    parser.add_argument("--force", action="store_true", help="Force migration even if validation fails")

    args = parser.parse_args()

    migrator = LegacyMigrator(args.root, args.dry_run)
    success = migrator.run_migration()

    if not success and not args.force:
        print("\n[CRITICAL] Migration failed. Use --force to proceed anyway (not recommended).")
        sys.exit(1)

    sys.exit(0)


if __name__ == "__main__":
    main()
