#!/usr/bin/env python3
"""
Advanced Version Synchronization and Detection Script

This script provides comprehensive version management for GraphBit:
- Detects authoritative version from multiple sources (local files, git tags, GitHub releases)
- Verifies synchronization across all version-containing files
- Provides automatic conflict resolution and version promotion
- Integrates with CI/CD for automated version management

Usage:
    python scripts/verify-version-sync.py                    # Verify synchronization
    python scripts/verify-version-sync.py --fix              # Auto-fix discrepancies
    python scripts/verify-version-sync.py --detect-latest    # Detect latest authoritative version
    python scripts/verify-version-sync.py --promote-version 0.3.0  # Promote specific version
"""

import json
import re
import sys
import argparse
import subprocess
import urllib.request
import urllib.error
from pathlib import Path
from typing import Dict, List, Tuple, Optional, Set
from dataclasses import dataclass
from packaging import version


@dataclass
class VersionReference:
    """Represents a version reference in a file."""
    file_path: str
    version: str
    pattern: str
    line_number: int
    is_master: bool = False
    source_type: str = "file"  # "file", "git_tag", "github_release"


@dataclass
class VersionReport:
    """Comprehensive report of version synchronization status."""
    master_versions: Dict[str, str]
    derived_versions: Dict[str, str]
    remote_versions: Dict[str, str]  # Git tags and GitHub releases
    inconsistencies: List[Tuple[str, str, str]]  # (file, found_version, expected_version)
    authoritative_version: str
    is_synchronized: bool
    needs_promotion: bool = False
    recommendations: List[str] = None


@dataclass
class VersionSource:
    """Represents a version source with priority and detection metadata."""
    name: str
    version: str
    priority: int  # Higher number = higher priority
    source_type: str  # "master", "derived", "remote"
    last_updated: Optional[str] = None
    confidence: float = 1.0  # Confidence in this version (0.0-1.0)


class AdvancedVersionManager:
    """Advanced version management with remote detection and conflict resolution."""

    def __init__(self, root_path: Path):
        self.root_path = root_path
        self.version_refs: List[VersionReference] = []
        self.remote_versions: Dict[str, str] = {}
        self.github_repo = "InfinitiBit/graphbit"  # Detected from context

    def detect_remote_versions(self) -> Dict[str, str]:
        """Detect versions from remote sources (git tags, GitHub releases)."""
        remote_versions = {}

        # Get git tags
        try:
            result = subprocess.run(
                ["git", "tag", "--sort=-version:refname"],
                capture_output=True,
                text=True,
                cwd=self.root_path
            )
            if result.returncode == 0:
                tags = [tag.strip() for tag in result.stdout.split('\n') if tag.strip()]
                if tags:
                    # Get the latest tag
                    latest_tag = tags[0]
                    # Remove 'v' prefix if present
                    clean_version = latest_tag.lstrip('v')
                    remote_versions['git_latest_tag'] = clean_version
                    remote_versions['git_all_tags'] = ', '.join(tags[:5])  # Top 5 tags
        except Exception as e:
            print(f"Warning: Could not fetch git tags: {e}")

        # Get GitHub releases
        try:
            url = f"https://api.github.com/repos/{self.github_repo}/releases/latest"
            with urllib.request.urlopen(url) as response:
                data = json.loads(response.read().decode())
                tag_name = data.get('tag_name', '')
                clean_version = tag_name.lstrip('v')
                remote_versions['github_latest_release'] = clean_version
        except Exception as e:
            print(f"Warning: Could not fetch GitHub releases: {e}")

        return remote_versions

    def determine_authoritative_version(self, local_versions: Dict[str, str],
                                      remote_versions: Dict[str, str]) -> Tuple[str, List[str]]:
        """Determine the authoritative version using priority-based resolution."""
        version_sources = []
        recommendations = []

        # Priority system:
        # 1. Master sources (if consistent): Priority 100
        # 2. GitHub latest release: Priority 90
        # 3. Git latest tag: Priority 80
        # 4. Derived sources (if consistent): Priority 70

        # Check master source consistency
        master_versions = {k: v for k, v in local_versions.items()
                          if k in ['Cargo.toml (workspace)', 'python/pyproject.toml']}

        if len(set(master_versions.values())) == 1:
            # Master sources are consistent
            master_version = list(master_versions.values())[0]
            version_sources.append(VersionSource(
                "master_sources", master_version, 100, "master"
            ))
        else:
            # Master sources inconsistent - use highest version
            if master_versions:
                highest_master = max(master_versions.values(), key=lambda v: version.parse(v))
                version_sources.append(VersionSource(
                    "master_sources_highest", highest_master, 95, "master"
                ))
                recommendations.append(f"⚠️  Master sources inconsistent. Promoting highest: {highest_master}")

        # Add remote sources
        if 'github_latest_release' in remote_versions:
            version_sources.append(VersionSource(
                "github_release", remote_versions['github_latest_release'], 90, "remote"
            ))

        if 'git_latest_tag' in remote_versions:
            version_sources.append(VersionSource(
                "git_tag", remote_versions['git_latest_tag'], 80, "remote"
            ))

        # Determine authoritative version (highest priority, then highest version)
        if not version_sources:
            return "0.1.0", ["⚠️  No version sources found. Using default 0.1.0"]

        # Sort by priority, then by version
        version_sources.sort(key=lambda x: (x.priority, version.parse(x.version)), reverse=True)
        authoritative = version_sources[0]

        # Check for version conflicts
        remote_higher = False
        for source in version_sources[1:]:
            if (source.source_type == "remote" and
                version.parse(source.version) > version.parse(authoritative.version)):
                remote_higher = True
                recommendations.append(
                    f"🚀 Remote version {source.version} is higher than local {authoritative.version}. "
                    f"Consider updating local versions."
                )
                break

        return authoritative.version, recommendations

    def find_all_versions(self) -> List[VersionReference]:
        """Find all version references in the codebase."""
        refs = []
        
        # 1. Cargo.toml workspace version (MASTER)
        refs.extend(self._find_in_file(
            "Cargo.toml",
            r'\[workspace\.package\][\s\S]*?^version = "([^"]+)"',
            is_master=True
        ))

        # 2. Python pyproject.toml (MASTER)
        refs.extend(self._find_in_file(
            "python/pyproject.toml",
            r'^version = "([^"]+)"',
            is_master=True
        ))

        # 3. Root pyproject.toml (look for [tool.poetry] section)
        refs.extend(self._find_in_file(
            "pyproject.toml",
            r'\[tool\.poetry\][\s\S]*?^version = "([^"]+)"'
        ))
        
        # 4. Node.js package.json
        refs.extend(self._find_in_file(
            "nodejs/package.json",
            r'"version":\s*"([^"]+)"'
        ))
        
        # 5. Benchmarks __init__.py
        refs.extend(self._find_in_file(
            "benchmarks/frameworks/__init__.py",
            r'__version__ = "([^"]+)"'
        ))
        
        # 6. CHANGELOG.md (latest version - first occurrence only)
        refs.extend(self._find_in_file(
            "CHANGELOG.md",
            r'## \[([^\]]+)\]',
            first_only=True
        ))
        
        # 7. Core README.md
        refs.extend(self._find_in_file(
            "core/README.md",
            r'graphbit-core = "([^"]+)"'
        ))
        
        self.version_refs = refs
        return refs
    
    def _find_in_file(self, file_path: str, pattern: str, is_master: bool = False, first_only: bool = False) -> List[VersionReference]:
        """Find version references in a specific file."""
        full_path = self.root_path / file_path
        if not full_path.exists():
            return []

        refs = []
        try:
            with open(full_path, 'r', encoding='utf-8') as f:
                content = f.read()

            # Use multiline matching for complex patterns
            matches = re.finditer(pattern, content, re.MULTILINE | re.DOTALL)
            for match in matches:
                # Extract version from the first capturing group
                if match.groups():
                    version = match.group(1)
                else:
                    version = match.group(0)

                # Calculate line number
                line_num = content[:match.start()].count('\n') + 1

                refs.append(VersionReference(
                    file_path=file_path,
                    version=version,
                    pattern=pattern,
                    line_number=line_num,
                    is_master=is_master
                ))

                # If first_only is True, break after first match
                if first_only:
                    break

                # For most files, only take the first match
                if file_path != "CHANGELOG.md":
                    break

        except Exception as e:
            print(f"⚠️  Error reading {file_path}: {e}")

        return refs
    
    def generate_comprehensive_report(self) -> VersionReport:
        """Generate a comprehensive version synchronization report."""
        # Find all local versions
        self.version_refs = self.find_all_versions()

        # Detect remote versions
        remote_versions = self.detect_remote_versions()

        # Organize versions by type
        master_versions = {}
        derived_versions = {}

        for ref in self.version_refs:
            if ref.is_master:
                master_versions[ref.file_path] = ref.version
            else:
                derived_versions[ref.file_path] = ref.version

        # Combine all local versions for authoritative determination
        all_local_versions = {**master_versions, **derived_versions}

        # Determine authoritative version
        authoritative_version, recommendations = self.determine_authoritative_version(
            all_local_versions, remote_versions
        )

        # Find inconsistencies
        inconsistencies = []
        for ref in self.version_refs:
            if ref.version != authoritative_version:
                # Skip CHANGELOG.md as it can have multiple versions
                if "CHANGELOG.md" not in ref.file_path:
                    inconsistencies.append((
                        ref.file_path, ref.version, authoritative_version
                    ))

        # Determine if promotion is needed
        needs_promotion = False
        if remote_versions:
            # Filter out composite values like "v0.3.0, v0.2.0, v0.1.0"
            valid_remote_versions = []
            for v in remote_versions.values():
                if v and ',' not in v:  # Skip composite values
                    try:
                        version.parse(v)  # Validate version format
                        valid_remote_versions.append(v)
                    except:
                        continue

            if valid_remote_versions:
                latest_remote = max(valid_remote_versions, key=lambda x: version.parse(x))
                if version.parse(latest_remote) > version.parse(authoritative_version):
                    needs_promotion = True

        return VersionReport(
            master_versions=master_versions,
            derived_versions=derived_versions,
            remote_versions=remote_versions,
            inconsistencies=inconsistencies,
            authoritative_version=authoritative_version,
            is_synchronized=len(inconsistencies) == 0,
            needs_promotion=needs_promotion,
            recommendations=recommendations or []
        )

    def print_detailed_report(self, report: VersionReport):
        """Print a detailed version synchronization report."""
        print("=" * 60)
        print("🔍 COMPREHENSIVE VERSION ANALYSIS REPORT")
        print("=" * 60)

        print(f"\n🎯 AUTHORITATIVE VERSION: {report.authoritative_version}")

        if report.recommendations:
            print(f"\n💡 RECOMMENDATIONS:")
            for rec in report.recommendations:
                print(f"   {rec}")

        print(f"\n📊 MASTER SOURCES:")
        for file_path, ver in report.master_versions.items():
            status = "✅" if ver == report.authoritative_version else "❌"
            print(f"   {status} {file_path}: {ver}")

        print(f"\n🔗 DERIVED SOURCES:")
        for file_path, ver in report.derived_versions.items():
            status = "✅" if ver == report.authoritative_version else "❌"
            print(f"   {status} {file_path}: {ver}")

        if report.remote_versions:
            print(f"\n🌐 REMOTE SOURCES:")
            for source, ver in report.remote_versions.items():
                print(f"   📡 {source}: {ver}")

        if report.inconsistencies:
            print(f"\n🚨 INCONSISTENCIES FOUND:")
            for file_path, found_ver, expected_ver in report.inconsistencies:
                print(f"   ❌ {file_path}: {found_ver} (expected {expected_ver})")

        print(f"\n📈 SYNCHRONIZATION STATUS:")
        if report.is_synchronized:
            print("   ✅ All versions are synchronized!")
        else:
            print(f"   ❌ {len(report.inconsistencies)} inconsistencies found")

        if report.needs_promotion:
            print("   🚀 Version promotion recommended (remote version is higher)")

        print("=" * 60)
    
    def promote_version(self, target_version: str) -> bool:
        """Promote a specific version across all files."""
        print(f"🚀 PROMOTING VERSION TO: {target_version}")
        print("=" * 50)

        # Validate version format
        try:
            version.parse(target_version)
        except Exception as e:
            print(f"❌ Invalid version format '{target_version}': {e}")
            return False

        # Update all version files
        files_updated = []

        # 1. Master sources first
        print("📝 Updating master sources...")

        # Cargo.toml workspace version
        if self._update_file_version("Cargo.toml",
                                   r'^version = "[^"]+"',
                                   f'version = "{target_version}"'):
            files_updated.append("Cargo.toml")
            print(f"   ✅ Updated Cargo.toml")

        # Python pyproject.toml
        if self._update_file_version("python/pyproject.toml",
                                   r'version = "[^"]+"',
                                   f'version = "{target_version}"'):
            files_updated.append("python/pyproject.toml")
            print(f"   ✅ Updated python/pyproject.toml")

        # 2. Derived sources
        print("📝 Updating derived sources...")

        # Root pyproject.toml
        if self._update_file_version("pyproject.toml",
                                   r'^version = "[^"]+"',
                                   f'version = "{target_version}"'):
            files_updated.append("pyproject.toml")
            print(f"   ✅ Updated pyproject.toml")

        # Node.js package.json
        if self._update_file_version("nodejs/package.json",
                                   r'"version":\s*"[^"]+"',
                                   f'"version": "{target_version}"'):
            files_updated.append("nodejs/package.json")
            print(f"   ✅ Updated nodejs/package.json")

        # Benchmarks __init__.py
        if self._update_file_version("benchmarks/frameworks/__init__.py",
                                   r'__version__ = "[^"]+"',
                                   f'__version__ = "{target_version}"'):
            files_updated.append("benchmarks/frameworks/__init__.py")
            print(f"   ✅ Updated benchmarks/frameworks/__init__.py")

        # README files
        readme_files = ["README.md", "core/README.md", "python/README.md"]
        for readme in readme_files:
            if self._update_readme_version(readme, target_version):
                files_updated.append(readme)
                print(f"   ✅ Updated {readme}")

        print(f"\n🎉 Successfully updated {len(files_updated)} files to version {target_version}")

        # Generate changelog entry
        changelog_success = self.generate_changelog_entry(target_version)
        if changelog_success:
            files_updated.append("CHANGELOG.md")

        if files_updated:
            print("\n📋 Updated files:")
            for file_path in files_updated:
                print(f"   • {file_path}")

            print(f"\n💡 Next steps:")
            print(f"   1. Review the changes: git diff")
            print(f"   2. Test the build: make test")
            print(f"   3. Commit changes: git add . && git commit -m 'chore: bump version to {target_version}'")
            print(f"   4. Create release: git tag v{target_version} && git push --tags")

        return len(files_updated) > 0

    def generate_changelog_entry(self, target_version: str) -> bool:
        """Generate changelog entry for the target version."""
        print(f"📝 Generating changelog entry for version {target_version}...")

        try:
            # Import and use the changelog generator
            import subprocess
            result = subprocess.run([
                "python", "scripts/generate-changelog.py",
                "--version", target_version
            ], capture_output=True, text=True, cwd=self.root_path)

            if result.returncode == 0:
                print(f"✅ Successfully generated changelog entry for {target_version}")
                return True
            else:
                print(f"❌ Failed to generate changelog: {result.stderr}")
                return False

        except Exception as e:
            print(f"❌ Error generating changelog: {e}")
            return False

    def fix_versions(self) -> bool:
        """Fix version discrepancies by updating to master version."""
        if not self.master_version:
            print("❌ Cannot fix versions without a master version")
            return False
        
        print(f"🔧 Fixing all versions to {self.master_version}")
        
        # Update each file
        files_updated = []
        
        # 1. Root pyproject.toml
        if self._update_file_version("pyproject.toml", 
                                   r'(\[tool\.poetry\][\s\S]*?)version = "[^"]+"',
                                   rf'\1version = "{self.master_version}"'):
            files_updated.append("pyproject.toml")
        
        # 2. Node.js package.json
        if self._update_json_version("nodejs/package.json"):
            files_updated.append("nodejs/package.json")
        
        # 3. Benchmarks __init__.py
        if self._update_file_version("benchmarks/frameworks/__init__.py",
                                   r'__version__ = "[^"]+"',
                                   f'__version__ = "{self.master_version}"'):
            files_updated.append("benchmarks/frameworks/__init__.py")
        
        # 4. Core README.md
        if self._update_file_version("core/README.md",
                                   r'graphbit-core = "[^"]+"',
                                   f'graphbit-core = "{self.master_version}"'):
            files_updated.append("core/README.md")
        
        if files_updated:
            print(f"✅ Updated {len(files_updated)} files:")
            for file in files_updated:
                print(f"   - {file}")
            return True
        else:
            print("ℹ️  No files needed updating")
            return False
    
    def _update_file_version(self, file_path: str, pattern: str, replacement: str) -> bool:
        """Update version in a text file."""
        full_path = self.root_path / file_path
        if not full_path.exists():
            return False
        
        try:
            with open(full_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            new_content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
            
            if new_content != content:
                with open(full_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                return True
                
        except Exception as e:
            print(f"⚠️  Error updating {file_path}: {e}")
            
        return False
    
    def _update_json_version(self, file_path: str) -> bool:
        """Update version in a JSON file."""
        full_path = self.root_path / file_path
        if not full_path.exists():
            return False
        
        try:
            with open(full_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            if data.get('version') != self.master_version:
                data['version'] = self.master_version
                
                with open(full_path, 'w', encoding='utf-8') as f:
                    json.dump(data, f, indent=2)
                    f.write('\n')  # Add trailing newline
                return True
                
        except Exception as e:
            print(f"⚠️  Error updating {file_path}: {e}")
            
        return False

    def _update_readme_version(self, file_path: str, target_version: str) -> bool:
        """Update version references in README files."""
        full_path = self.root_path / file_path
        if not full_path.exists():
            return False

        try:
            with open(full_path, 'r', encoding='utf-8') as f:
                content = f.read()

            original_content = content

            # Update various version patterns in README files
            patterns = [
                # graphbit-core = "0.1.0"
                (r'graphbit-core = "[^"]+"', f'graphbit-core = "{target_version}"'),
                # graphbit = "0.1.0"
                (r'graphbit = "[^"]+"', f'graphbit = "{target_version}"'),
                # version = "0.1.0"
                (r'version = "[^"]+"', f'version = "{target_version}"'),
                # [0.1.0]
                (r'\[[0-9]+\.[0-9]+\.[0-9]+[^\]]*\]', f'[{target_version}]'),
                # v0.1.0
                (r'v[0-9]+\.[0-9]+\.[0-9]+[^\s]*', f'v{target_version}'),
            ]

            for pattern, replacement in patterns:
                content = re.sub(pattern, replacement, content)

            if content != original_content:
                with open(full_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True

        except Exception as e:
            print(f"⚠️  Error updating {file_path}: {e}")

        return False


def main():
    parser = argparse.ArgumentParser(
        description="Advanced GraphBit Version Management System",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/verify-version-sync.py                    # Verify synchronization
  python scripts/verify-version-sync.py --fix              # Auto-fix discrepancies
  python scripts/verify-version-sync.py --detect-latest    # Detect latest authoritative version
  python scripts/verify-version-sync.py --promote-version 0.3.0  # Promote specific version
        """
    )
    parser.add_argument("--fix", action="store_true",
                       help="Automatically fix version discrepancies")
    parser.add_argument("--detect-latest", action="store_true",
                       help="Detect and report the latest authoritative version")
    parser.add_argument("--promote-version", type=str,
                       help="Promote a specific version across all files")
    parser.add_argument("--generate-changelog", type=str,
                       help="Generate changelog entry for specified version")
    parser.add_argument("--root", type=Path, default=Path.cwd(),
                       help="Root directory of the project")

    args = parser.parse_args()

    print("🚀 GraphBit Advanced Version Management System")
    print("=" * 60)

    manager = AdvancedVersionManager(args.root)

    # Handle version promotion
    if args.promote_version:
        success = manager.promote_version(args.promote_version)
        sys.exit(0 if success else 1)

    # Handle changelog generation
    if args.generate_changelog:
        success = manager.generate_changelog_entry(args.generate_changelog)
        sys.exit(0 if success else 1)

    # Generate comprehensive report
    print("📊 Generating comprehensive version analysis...")
    report = manager.generate_comprehensive_report()

    # Print detailed report
    manager.print_detailed_report(report)

    # Handle specific actions
    if args.detect_latest:
        print(f"\n🎯 DETECTED AUTHORITATIVE VERSION: {report.authoritative_version}")
        if report.needs_promotion:
            # Find the highest valid remote version
            valid_remote_versions = []
            for v in report.remote_versions.values():
                if v and ',' not in v:
                    try:
                        version.parse(v)
                        valid_remote_versions.append(v)
                    except:
                        continue

            if valid_remote_versions:
                latest_remote = max(valid_remote_versions, key=lambda x: version.parse(x))
                print(f"💡 Consider promoting to remote version: {latest_remote}")
                print(f"   Command: python scripts/verify-version-sync.py --promote-version {latest_remote}")
        sys.exit(0)

    if args.fix and not report.is_synchronized:
        print(f"\n🔧 FIXING INCONSISTENCIES...")
        success = manager.promote_version(report.authoritative_version)
        sys.exit(0 if success else 1)

    # Final status
    if report.is_synchronized:
        print(f"\n🎉 SUCCESS: All versions are synchronized at {report.authoritative_version}!")
        if report.needs_promotion:
            print(f"💡 Note: Remote versions are higher. Consider updating with --promote-version")
        sys.exit(0)
    else:
        print(f"\n⚠️  ISSUES FOUND: {len(report.inconsistencies)} version inconsistencies detected")
        if args.fix:
            print("💡 Use --fix flag to automatically resolve inconsistencies")
        sys.exit(1)


if __name__ == "__main__":
    main()
