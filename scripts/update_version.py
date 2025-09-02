#!/usr/bin/env python3
"""
GraphBit Version Management Script

This script automatically updates version numbers across the entire GraphBit repository.
It handles various file formats and version patterns while maintaining data integrity.

Usage:
    python scripts/update_version.py <new_version> [--dry-run] [--backup]

Examples:
    python scripts/update_version.py 0.2.0
    python scripts/update_version.py 1.0.0-beta.1 --dry-run
    python scripts/update_version.py 2.1.3 --backup
"""

import argparse
import json
import os
import re
import shutil
import sys
import tempfile
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple, Optional

try:
    import toml
except ImportError:
    print("Warning: toml package not found. Install with: pip install toml")
    toml = None


class VersionPattern:
    """Defines a version pattern for different file types."""
    
    def __init__(self, pattern: str, replacement: str, description: str):
        self.pattern = pattern
        self.replacement = replacement
        self.description = description


class VersionManager:
    """Manages version updates across the GraphBit repository."""
    
    def __init__(self, repo_root: Path):
        self.repo_root = repo_root
        self.backup_dir: Optional[Path] = None
        self.modified_files: List[Path] = []
        
        # Define version patterns for different file types
        self.patterns = {
            'cargo_toml': VersionPattern(
                r'^(\s*version\s*=\s*["\'])([^"\']+)(["\'].*?)$',
                r'\g<1>{new_version}\g<3>',
                'Cargo.toml version field'
            ),
            'pyproject_toml': VersionPattern(
                r'^(\s*version\s*=\s*["\'])([^"\']+)(["\'].*?)$',
                r'\g<1>{new_version}\g<3>',
                'pyproject.toml version field'
            ),
            'package_json': VersionPattern(
                r'^(\s*["\']version["\']\s*:\s*["\'])([^"\']+)(["\'].*?)$',
                r'\g<1>{new_version}\g<3>',
                'package.json version field'
            ),
            'python_version': VersionPattern(
                r'^(\s*__version__\s*=\s*["\'])([^"\']+)(["\'].*?)$',
                r'\g<1>{new_version}\g<3>',
                'Python __version__ variable'
            )
        }
        
        # File type mappings
        self.file_patterns = {
            'Cargo.toml': ['cargo_toml'],
            'pyproject.toml': ['pyproject_toml'],
            'package.json': ['package_json'],
            '__init__.py': ['python_version']
        }

    def validate_version(self, version: str) -> bool:
        """Validate semantic version format."""
        # Semantic versioning pattern: MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
        pattern = r'^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$'
        return bool(re.match(pattern, version))

    def create_backup(self) -> Path:
        """Create a backup directory with timestamp."""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        backup_name = f"version_backup_{timestamp}"
        self.backup_dir = self.repo_root / "target" / backup_name
        self.backup_dir.mkdir(parents=True, exist_ok=True)
        return self.backup_dir

    def backup_file(self, file_path: Path) -> None:
        """Backup a single file before modification."""
        if self.backup_dir is None:
            return
            
        relative_path = file_path.relative_to(self.repo_root)
        backup_path = self.backup_dir / relative_path
        backup_path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(file_path, backup_path)

    def find_version_files(self) -> List[Path]:
        """Find all files that potentially contain version information."""
        version_files = []

        # Known configuration files (explicit paths)
        known_files = [
            'Cargo.toml',
            'core/Cargo.toml',
            'python/Cargo.toml',
            'nodejs/Cargo.toml',
            'pyproject.toml',
            'nodejs/package.json'
        ]

        for file_path in known_files:
            full_path = self.repo_root / file_path
            if full_path.exists():
                version_files.append(full_path)

        # Search for Python __init__.py files with __version__ (excluding virtual environments)
        for init_file in self.repo_root.rglob('__init__.py'):
            if self._should_include_file(init_file) and self._file_contains_version(init_file):
                version_files.append(init_file)

        return sorted(set(version_files))

    def _should_include_file(self, file_path: Path) -> bool:
        """Check if a file should be included in version updates."""
        path_str = str(file_path)

        # Exclude virtual environments and build directories
        exclude_patterns = [
            '.venv',
            'venv',
            'env',
            'target',
            'node_modules',
            '.git',
            '__pycache__',
            '.pytest_cache',
            'site-packages',
            'dist',
            'build'
        ]

        for pattern in exclude_patterns:
            if pattern in path_str:
                return False

        return True

    def _file_contains_version(self, file_path: Path) -> bool:
        """Check if a file contains version information."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                # Look for common version patterns
                patterns = [
                    r'__version__\s*=',
                    r'version\s*=\s*["\']',
                    r'["\']version["\']\s*:',
                ]
                return any(re.search(pattern, content, re.IGNORECASE) for pattern in patterns)
        except (UnicodeDecodeError, PermissionError):
            return False

    def update_file(self, file_path: Path, new_version: str, dry_run: bool = False) -> List[str]:
        """Update version in a single file."""
        changes = []

        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            updated_lines = []
            file_modified = False

            # Context tracking for Cargo.toml files
            current_section = None

            for line_num, line in enumerate(lines, 1):
                original_line = line
                updated_line = line

                # Track sections in TOML files
                if file_path.name.endswith('.toml'):
                    section_match = re.match(r'^\s*\[([^\]]+)\]', line)
                    if section_match:
                        current_section = section_match.group(1)

                # Try each pattern that applies to this file type
                for pattern_name in self._get_applicable_patterns(file_path):
                    pattern = self.patterns[pattern_name]
                    match = re.match(pattern.pattern, line)
                    if match:
                        old_version = match.group(2)

                        # Additional validation for specific file types
                        should_update = True

                        if file_path.name == 'Cargo.toml' and current_section != 'package':
                            # Only update version in [package] section for Cargo.toml
                            should_update = False
                        elif file_path.name == 'package.json' and 'napi' in old_version:
                            # Skip script commands in package.json
                            should_update = False

                        if should_update:
                            updated_line = re.sub(
                                pattern.pattern,
                                pattern.replacement.format(new_version=new_version),
                                line
                            )
                            if updated_line != original_line:
                                changes.append(f"Line {line_num}: {old_version} → {new_version}")
                                file_modified = True
                        break

                updated_lines.append(updated_line)

            if file_modified and not dry_run:
                self.backup_file(file_path)
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.writelines(updated_lines)
                self.modified_files.append(file_path)

            return changes

        except Exception as e:
            raise RuntimeError(f"Failed to update {file_path}: {e}")

    def _get_applicable_patterns(self, file_path: Path) -> List[str]:
        """Get applicable patterns for a file based on its name and extension."""
        file_name = file_path.name

        # Direct file name matches
        if file_name in self.file_patterns:
            return self.file_patterns[file_name]

        # Extension-based matches with more specific logic
        if file_name.endswith('.toml'):
            if 'pyproject' in file_name.lower():
                return ['pyproject_toml']
            elif file_name == 'Cargo.toml':
                return ['cargo_toml']
        elif file_name == 'package.json':
            return ['package_json']
        elif file_name == '__init__.py':
            return ['python_version']

        # No patterns for other files
        return []

    def update_versions(self, new_version: str, dry_run: bool = False, create_backup: bool = False) -> Dict[str, List[str]]:
        """Update versions across all relevant files."""
        if not self.validate_version(new_version):
            raise ValueError(f"Invalid version format: {new_version}")
        
        if create_backup and not dry_run:
            backup_path = self.create_backup()
            print(f"Created backup at: {backup_path}")
        
        version_files = self.find_version_files()
        results = {}
        
        print(f"Found {len(version_files)} files with version information:")
        for file_path in version_files:
            print(f"  - {file_path.relative_to(self.repo_root)}")
        
        print(f"\n{'DRY RUN: ' if dry_run else ''}Updating versions to {new_version}...")
        
        for file_path in version_files:
            try:
                changes = self.update_file(file_path, new_version, dry_run)
                if changes:
                    relative_path = str(file_path.relative_to(self.repo_root))
                    results[relative_path] = changes
                    print(f"\n{relative_path}:")
                    for change in changes:
                        print(f"  {change}")
                else:
                    print(f"\n{file_path.relative_to(self.repo_root)}: No changes needed")
            except Exception as e:
                print(f"ERROR updating {file_path.relative_to(self.repo_root)}: {e}")
                if not dry_run:
                    self.rollback()
                    raise
        
        return results

    def rollback(self) -> None:
        """Rollback changes using backup files."""
        if self.backup_dir is None or not self.backup_dir.exists():
            print("No backup available for rollback")
            return

        print("Rolling back changes...")
        for file_path in self.modified_files:
            try:
                relative_path = file_path.relative_to(self.repo_root)
                backup_path = self.backup_dir / relative_path
                if backup_path.exists():
                    shutil.copy2(backup_path, file_path)
                    print(f"  Restored: {relative_path}")
            except Exception as e:
                print(f"  Failed to restore {file_path}: {e}")

    def get_current_version(self) -> Optional[str]:
        """Get the current version from the main Cargo.toml file."""
        main_cargo = self.repo_root / 'Cargo.toml'
        if not main_cargo.exists():
            return None

        try:
            with open(main_cargo, 'r', encoding='utf-8') as f:
                lines = f.readlines()
                # Look for version in [package] section
                in_package_section = False
                for line in lines:
                    line = line.strip()
                    if line == '[package]':
                        in_package_section = True
                        continue
                    elif line.startswith('[') and line != '[package]':
                        in_package_section = False
                        continue

                    if in_package_section and line.startswith('version'):
                        match = re.search(r'version\s*=\s*["\']([^"\']+)["\']', line)
                        if match:
                            return match.group(1)

                return None
        except Exception:
            return None

    def validate_repository_state(self) -> bool:
        """Validate that the repository is in a good state for version updates."""
        # Check if we're in a git repository
        if not (self.repo_root / '.git').exists():
            print("Warning: Not in a git repository")
            return False

        # Check for uncommitted changes
        try:
            import subprocess
            result = subprocess.run(
                ['git', 'status', '--porcelain'],
                cwd=self.repo_root,
                capture_output=True,
                text=True
            )
            if result.returncode == 0 and result.stdout.strip():
                print("Warning: Repository has uncommitted changes")
                print("Consider committing or stashing changes before updating version")
                return False
        except (subprocess.SubprocessError, FileNotFoundError):
            print("Warning: Could not check git status")

        return True

    def generate_report(self, results: Dict[str, List[str]], new_version: str, dry_run: bool) -> str:
        """Generate a detailed report of the version update."""
        report_lines = [
            "# GraphBit Version Update Report",
            f"Generated: {datetime.now().isoformat()}",
            f"Target Version: {new_version}",
            f"Mode: {'Dry Run' if dry_run else 'Applied Changes'}",
            ""
        ]

        if results:
            report_lines.extend([
                "## Modified Files",
                ""
            ])

            for file_path, changes in results.items():
                report_lines.append(f"### {file_path}")
                for change in changes:
                    report_lines.append(f"- {change}")
                report_lines.append("")
        else:
            report_lines.append("## No Changes Required")
            report_lines.append("All files already have the target version.")

        return "\n".join(report_lines)


def main():
    """Main entry point for the version management script."""
    parser = argparse.ArgumentParser(
        description="Update version numbers across the GraphBit repository",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/update_version.py 0.2.0
  python scripts/update_version.py 1.0.0-beta.1 --dry-run
  python scripts/update_version.py 2.1.3 --backup --report
        """
    )

    parser.add_argument(
        'new_version',
        help='New version number (semantic versioning format)'
    )
    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Show what would be changed without making actual changes'
    )
    parser.add_argument(
        '--backup',
        action='store_true',
        help='Create backup before making changes'
    )
    parser.add_argument(
        '--report',
        action='store_true',
        help='Generate a detailed report file'
    )
    parser.add_argument(
        '--force',
        action='store_true',
        help='Skip repository state validation'
    )

    args = parser.parse_args()

    # Find repository root
    script_dir = Path(__file__).parent
    repo_root = script_dir.parent

    if not (repo_root / 'Cargo.toml').exists():
        print("Error: Could not find repository root (no Cargo.toml found)")
        sys.exit(1)

    try:
        manager = VersionManager(repo_root)

        # Show current version
        current_version = manager.get_current_version()
        if current_version:
            print(f"Current version: {current_version}")
            if current_version == args.new_version:
                print(f"Version is already {args.new_version}")
                sys.exit(0)

        # Validate repository state unless forced
        if not args.force and not manager.validate_repository_state():
            print("Use --force to skip repository state validation")
            sys.exit(1)

        # Update versions
        results = manager.update_versions(
            args.new_version,
            dry_run=args.dry_run,
            create_backup=args.backup
        )

        # Generate report if requested
        if args.report:
            report = manager.generate_report(results, args.new_version, args.dry_run)
            report_file = repo_root / f"version_update_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.md"
            with open(report_file, 'w', encoding='utf-8') as f:
                f.write(report)
            print(f"\nReport saved to: {report_file}")

        print(f"\n{'DRY RUN ' if args.dry_run else ''}Summary:")
        print(f"  Version: {current_version} → {args.new_version}")
        print(f"  Files processed: {len(results)}")
        print(f"  Total changes: {sum(len(changes) for changes in results.values())}")

        if not args.dry_run and results:
            print(f"\n✅ Version successfully updated to {args.new_version}")
            print("Next steps:")
            print("  1. Review the changes: git diff")
            print("  2. Test the build: make build")
            print("  3. Run tests: make test")
            print("  4. Commit changes: git add . && git commit -m 'chore: bump version to {}'".format(args.new_version))
        elif args.dry_run:
            print(f"\n🔍 Dry run completed. Use without --dry-run to apply changes.")

    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


if __name__ == '__main__':
    main()
