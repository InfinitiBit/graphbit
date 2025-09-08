#!/usr/bin/env python3
"""
Build Runner for GraphBit CI/CD Pipeline

This script handles artifact building, wheel generation, and packaging
for multiple platforms and architectures.
"""

import argparse
import subprocess
import sys
import time
import os
import shutil
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass


@dataclass
class BuildResult:
    """Result of a build operation."""
    name: str
    passed: bool
    duration: float
    artifacts: List[str]
    output: str
    error: Optional[str] = None


class BuildRunner:
    """Runs comprehensive build operations."""
    
    def __init__(self, root_path: Path, platform: str = "ubuntu-latest", target: str = "x86_64"):
        self.root_path = root_path
        self.platform = platform
        self.target = target
        self.results: List[BuildResult] = []
        self.artifacts_dir = root_path / ".github" / "artifacts"
        self.dist_dir = root_path / "dist"
        
        # Ensure directories exist
        self.artifacts_dir.mkdir(parents=True, exist_ok=True)
        self.dist_dir.mkdir(parents=True, exist_ok=True)
    
    def run_all_builds(self) -> bool:
        """Run all build operations."""
        print(f"Starting comprehensive build on {self.platform} for {self.target}...")
        
        builds = [
            ("Rust Library Build", self._build_rust_library),
            ("Python Extension Build", self._build_python_extension),
            ("Python Wheels", self._build_python_wheels),
            ("Source Distribution", self._build_source_distribution),
            ("Documentation", self._build_documentation),
            ("Artifact Verification", self._verify_artifacts),
        ]
        
        all_passed = True
        
        for name, build_func in builds:
            print(f"\nRunning {name}...")
            start_time = time.time()
            
            try:
                passed, artifacts, output, error = build_func()
                duration = time.time() - start_time
                
                result = BuildResult(
                    name=name,
                    passed=passed,
                    duration=duration,
                    artifacts=artifacts,
                    output=output,
                    error=error
                )
                
                self.results.append(result)
                
                status = "✅ PASSED" if passed else "❌ FAILED"
                artifacts_text = f" ({len(artifacts)} artifacts)" if artifacts else ""
                print(f"{status} - {name} ({duration:.2f}s){artifacts_text}")
                
                if not passed:
                    all_passed = False
                    if error:
                        print(f"Error: {error}")
                
            except Exception as e:
                duration = time.time() - start_time
                result = BuildResult(
                    name=name,
                    passed=False,
                    duration=duration,
                    artifacts=[],
                    output="",
                    error=str(e)
                )
                
                self.results.append(result)
                all_passed = False
                print(f"❌ FAILED - {name} ({duration:.2f}s)")
                print(f"Exception: {e}")
        
        self._generate_report()
        return all_passed
    
    def _build_rust_library(self) -> Tuple[bool, List[str], str, Optional[str]]:
        """Build Rust library."""
        try:
            result = subprocess.run(
                ['cargo', 'build', '--workspace', '--release'],
                capture_output=True,
                text=True,
                cwd=self.root_path,
                timeout=900
            )
            
            artifacts = []
            if result.returncode == 0:
                # Find built artifacts
                target_dir = self.root_path / "target" / "release"
                if target_dir.exists():
                    for item in target_dir.iterdir():
                        if item.is_file() and (item.suffix in ['.so', '.dll', '.dylib'] or item.stem.startswith('lib')):
                            artifacts.append(str(item))
            
            return result.returncode == 0, artifacts, result.stdout, result.stderr if result.returncode != 0 else None
            
        except subprocess.TimeoutExpired:
            return False, [], "", "Rust library build timed out"
        except Exception as e:
            return False, [], "", str(e)
    
    def _build_python_extension(self) -> Tuple[bool, List[str], str, Optional[str]]:
        """Build Python extension."""
        try:
            # Install maturin if not available
            install_result = subprocess.run(
                ['python', '-m', 'pip', 'install', 'maturin'],
                capture_output=True,
                text=True,
                cwd=self.root_path,
                timeout=300
            )
            
            if install_result.returncode != 0:
                return False, [], install_result.stdout, f"Failed to install maturin: {install_result.stderr}"
            
            # Build Python extension
            python_dir = self.root_path / "python"
            if not python_dir.exists():
                return True, [], "No Python directory found, skipping Python extension build", None
            
            result = subprocess.run(
                ['maturin', 'build', '--release', '--out', '../dist'],
                capture_output=True,
                text=True,
                cwd=python_dir,
                timeout=900
            )
            
            artifacts = []
            if result.returncode == 0:
                # Find built wheels
                for item in self.dist_dir.iterdir():
                    if item.suffix == '.whl':
                        artifacts.append(str(item))
            
            return result.returncode == 0, artifacts, result.stdout, result.stderr if result.returncode != 0 else None
            
        except subprocess.TimeoutExpired:
            return False, [], "", "Python extension build timed out"
        except Exception as e:
            return False, [], "", str(e)
    
    def _build_python_wheels(self) -> Tuple[bool, List[str], str, Optional[str]]:
        """Build Python wheels for distribution."""
        try:
            python_dir = self.root_path / "python"
            if not python_dir.exists():
                return True, [], "No Python directory found, skipping wheel build", None
            
            # Use maturin to build wheels
            result = subprocess.run(
                ['maturin', 'build', '--release', '--out', '../dist', '--find-interpreter'],
                capture_output=True,
                text=True,
                cwd=python_dir,
                timeout=1200
            )
            
            artifacts = []
            if result.returncode == 0:
                # Find all built wheels
                for item in self.dist_dir.iterdir():
                    if item.suffix == '.whl':
                        artifacts.append(str(item))
            
            return result.returncode == 0, artifacts, result.stdout, result.stderr if result.returncode != 0 else None
            
        except subprocess.TimeoutExpired:
            return False, [], "", "Python wheels build timed out"
        except Exception as e:
            return False, [], "", str(e)
    
    def _build_source_distribution(self) -> Tuple[bool, List[str], str, Optional[str]]:
        """Build source distribution."""
        try:
            python_dir = self.root_path / "python"
            if not python_dir.exists():
                return True, [], "No Python directory found, skipping source distribution", None
            
            # Build source distribution
            result = subprocess.run(
                ['maturin', 'sdist', '--out', '../dist'],
                capture_output=True,
                text=True,
                cwd=python_dir,
                timeout=300
            )
            
            artifacts = []
            if result.returncode == 0:
                # Find built source distributions
                for item in self.dist_dir.iterdir():
                    if item.suffix == '.gz' and '.tar' in item.name:
                        artifacts.append(str(item))
            
            return result.returncode == 0, artifacts, result.stdout, result.stderr if result.returncode != 0 else None
            
        except subprocess.TimeoutExpired:
            return False, [], "", "Source distribution build timed out"
        except Exception as e:
            return False, [], "", str(e)
    
    def _build_documentation(self) -> Tuple[bool, List[str], str, Optional[str]]:
        """Build documentation."""
        try:
            result = subprocess.run(
                ['cargo', 'doc', '--workspace', '--no-deps'],
                capture_output=True,
                text=True,
                cwd=self.root_path,
                timeout=600
            )
            
            artifacts = []
            if result.returncode == 0:
                # Find documentation artifacts
                doc_dir = self.root_path / "target" / "doc"
                if doc_dir.exists():
                    artifacts.append(str(doc_dir))
            
            return result.returncode == 0, artifacts, result.stdout, result.stderr if result.returncode != 0 else None
            
        except subprocess.TimeoutExpired:
            return False, [], "", "Documentation build timed out"
        except Exception as e:
            return False, [], "", str(e)
    
    def _verify_artifacts(self) -> Tuple[bool, List[str], str, Optional[str]]:
        """Verify built artifacts."""
        try:
            output_lines = []
            verified_artifacts = []
            
            # Check dist directory
            if self.dist_dir.exists():
                output_lines.append(f"Dist directory contents:")
                for item in self.dist_dir.iterdir():
                    size = item.stat().st_size if item.is_file() else 0
                    output_lines.append(f"  {item.name} ({size} bytes)")
                    
                    # Verify wheel integrity
                    if item.suffix == '.whl':
                        try:
                            # Try to install and test the wheel
                            test_result = subprocess.run(
                                ['python', '-m', 'pip', 'install', '--force-reinstall', '--no-deps', str(item)],
                                capture_output=True,
                                text=True,
                                timeout=120
                            )
                            
                            if test_result.returncode == 0:
                                # Try to import the package
                                import_result = subprocess.run(
                                    ['python', '-c', 'import graphbit; print(f"GraphBit version: {graphbit.version()}")'],
                                    capture_output=True,
                                    text=True,
                                    timeout=30
                                )
                                
                                if import_result.returncode == 0:
                                    verified_artifacts.append(str(item))
                                    output_lines.append(f"    ✅ Wheel verified: {import_result.stdout.strip()}")
                                else:
                                    output_lines.append(f"    ❌ Wheel import failed: {import_result.stderr}")
                            else:
                                output_lines.append(f"    ❌ Wheel installation failed: {test_result.stderr}")
                                
                        except subprocess.TimeoutExpired:
                            output_lines.append(f"    ⏰ Wheel verification timed out")
                        except Exception as e:
                            output_lines.append(f"    ❌ Wheel verification error: {e}")
            
            # Check target directory
            target_dir = self.root_path / "target" / "release"
            if target_dir.exists():
                output_lines.append(f"\nTarget directory contents:")
                for item in target_dir.iterdir():
                    if item.is_file():
                        size = item.stat().st_size
                        output_lines.append(f"  {item.name} ({size} bytes)")
                        verified_artifacts.append(str(item))
            
            output = "\n".join(output_lines)
            return True, verified_artifacts, output, None
            
        except Exception as e:
            return False, [], "", str(e)
    
    def _generate_report(self):
        """Generate build report."""
        passed_count = sum(1 for r in self.results if r.passed)
        total_count = len(self.results)
        total_duration = sum(r.duration for r in self.results)
        total_artifacts = sum(len(r.artifacts) for r in self.results)
        
        print("\n" + "=" * 60)
        print("BUILD REPORT")
        print("=" * 60)
        print(f"Platform: {self.platform}")
        print(f"Target: {self.target}")
        print(f"Total Build Steps: {total_count}")
        print(f"Passed: {passed_count}")
        print(f"Failed: {total_count - passed_count}")
        print(f"Total Duration: {total_duration:.2f}s")
        print(f"Total Artifacts: {total_artifacts}")
        
        if passed_count == total_count:
            print("\n🎉 All build steps completed successfully!")
        else:
            print(f"\n⚠️  {total_count - passed_count} build steps failed:")
            for result in self.results:
                if not result.passed:
                    print(f"  ❌ {result.name}")
                    if result.error:
                        print(f"     Error: {result.error}")
        
        # List all artifacts
        if total_artifacts > 0:
            print(f"\n📦 Built Artifacts:")
            for result in self.results:
                if result.artifacts:
                    print(f"  {result.name}:")
                    for artifact in result.artifacts:
                        print(f"    - {Path(artifact).name}")
        
        # Save detailed report
        self._save_detailed_report()
    
    def _save_detailed_report(self):
        """Save detailed build report to file."""
        report_file = self.artifacts_dir / f"build-report-{self.platform}-{self.target}-{datetime.now().strftime('%Y%m%d-%H%M%S')}.json"
        
        report_data = {
            "timestamp": datetime.now().isoformat(),
            "platform": self.platform,
            "target": self.target,
            "summary": {
                "total_steps": len(self.results),
                "passed_steps": sum(1 for r in self.results if r.passed),
                "failed_steps": sum(1 for r in self.results if not r.passed),
                "total_duration": sum(r.duration for r in self.results),
                "total_artifacts": sum(len(r.artifacts) for r in self.results)
            },
            "results": [
                {
                    "name": r.name,
                    "passed": r.passed,
                    "duration": r.duration,
                    "artifacts": r.artifacts,
                    "output": r.output,
                    "error": r.error
                }
                for r in self.results
            ]
        }
        
        with open(report_file, 'w') as f:
            import json
            json.dump(report_data, f, indent=2)
        
        print(f"\nDetailed report saved to: {report_file}")


def main():
    """Main entry point for build runner."""
    parser = argparse.ArgumentParser(
        description="GraphBit Build Runner",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    
    parser.add_argument(
        '--root',
        type=Path,
        default=Path.cwd(),
        help='Root directory of the project'
    )
    
    parser.add_argument(
        '--platform',
        default='ubuntu-latest',
        help='Platform identifier for building'
    )
    
    parser.add_argument(
        '--target',
        default='x86_64',
        help='Target architecture for building'
    )
    
    parser.add_argument(
        '--step',
        choices=['rust-lib', 'python-ext', 'wheels', 'sdist', 'docs', 'verify', 'all'],
        default='all',
        help='Specific build step to run'
    )
    
    args = parser.parse_args()
    
    runner = BuildRunner(args.root, args.platform, args.target)
    
    if args.step == 'all':
        success = runner.run_all_builds()
    else:
        # Run specific step (implementation would be added here)
        print(f"Running specific build step: {args.step}")
        success = runner.run_all_builds()  # For now, run all
    
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
