#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Advanced Changelog Generation System for GraphBit

This script provides comprehensive changelog generation with:
- Multi-source analysis (commits, PRs, manual entries)
- Semantic categorization and intelligent parsing
- Integration with version management system
- Support for conventional commits and descriptive messages
- Quality validation and consistency checks

Usage:
    python scripts/generate-changelog.py --version 0.4.0
    python scripts/generate-changelog.py --version 0.4.0 --since-tag v0.3.0
    python scripts/generate-changelog.py --validate-only
    python scripts/generate-changelog.py --preview --version 0.4.0
"""

import argparse
import json
import os
import re
import subprocess
import sys
import urllib.request
import urllib.error
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple, Optional, Set
from dataclasses import dataclass
from enum import Enum

# Fix Windows console encoding issues
if sys.platform == "win32":
    import codecs
    sys.stdout = codecs.getwriter("utf-8")(sys.stdout.detach())
    sys.stderr = codecs.getwriter("utf-8")(sys.stderr.detach())


class ChangeType(Enum):
    """Types of changes for changelog categorization."""
    BREAKING = "breaking"
    SECURITY = "security"
    FEATURE = "feature"
    FIX = "fix"
    PERFORMANCE = "performance"
    DEPRECATED = "deprecated"
    REMOVED = "removed"
    DOCUMENTATION = "documentation"
    TESTING = "testing"
    CHORE = "chore"
    OTHERS = "others"  # For unclassified changes


@dataclass
class ChangelogEntry:
    """Represents a single changelog entry."""
    change_type: ChangeType
    description: str
    commit_hash: str
    pr_number: Optional[int] = None
    breaking_change: bool = False
    scope: Optional[str] = None
    author: Optional[str] = None
    date: Optional[str] = None


@dataclass
class ChangelogSection:
    """Represents a section in the changelog."""
    title: str
    emoji: str
    entries: List[ChangelogEntry]
    priority: int


class AdvancedChangelogGenerator:
    """Advanced changelog generation with multi-source analysis."""
    
    def __init__(self, root_path: Path):
        self.root_path = root_path
        self.github_repo = "InfinitiBit/graphbit"
        
        # Change type mapping with priorities
        self.change_type_config = {
            ChangeType.BREAKING: ChangelogSection("🚨 Breaking Changes", "🚨", [], 1),
            ChangeType.SECURITY: ChangelogSection("🔒 Security", "🔒", [], 2),
            ChangeType.FEATURE: ChangelogSection("✨ New Features", "✨", [], 3),
            ChangeType.FIX: ChangelogSection("🐛 Bug Fixes", "🐛", [], 4),
            ChangeType.PERFORMANCE: ChangelogSection("⚡ Performance", "⚡", [], 5),
            ChangeType.DEPRECATED: ChangelogSection("⚠️ Deprecated", "⚠️", [], 6),
            ChangeType.REMOVED: ChangelogSection("🗑️ Removed", "🗑️", [], 7),
            ChangeType.DOCUMENTATION: ChangelogSection("📚 Documentation", "📚", [], 8),
            ChangeType.TESTING: ChangelogSection("🧪 Testing", "🧪", [], 9),
            ChangeType.CHORE: ChangelogSection("🔧 Maintenance", "🔧", [], 10),
            ChangeType.OTHERS: ChangelogSection("🔄 Others", "🔄", [], 11),
        }
        
        # Commit message patterns for categorization
        self.commit_patterns = {
            # Conventional commits
            ChangeType.BREAKING: [
                r'BREAKING CHANGE',
                r'breaking:',
                r'major:',
                r'!:'
            ],
            ChangeType.FEATURE: [
                r'^feat(\([^)]*\))?:',
                r'^feature(\([^)]*\))?:',
                r'^\[Feature\]',
                r'minor:'
            ],
            ChangeType.FIX: [
                r'^fix(\([^)]*\))?:',
                r'^patch:',
                r'^\[Fix\]',
                r'^\[Hotfix\]'
            ],
            ChangeType.SECURITY: [
                r'^security(\([^)]*\))?:',
                r'^\[Security\]',
                r'security fix',
                r'vulnerability'
            ],
            ChangeType.PERFORMANCE: [
                r'^perf(\([^)]*\))?:',
                r'^\[Performance\]',
                r'performance',
                r'optimization'
            ],
            ChangeType.DOCUMENTATION: [
                r'^docs(\([^)]*\))?:',
                r'^\[Documentation\]',
                r'^\[Doc\]',
                r'documentation'
            ],
            ChangeType.TESTING: [
                r'^test(\([^)]*\))?:',
                r'^\[Test\]',
                r'^\[Tests\]',
                r'unit test',
                r'integration test'
            ],
            ChangeType.CHORE: [
                r'^chore(\([^)]*\))?:',
                r'^style(\([^)]*\))?:',
                r'^refactor(\([^)]*\))?:',
                r'^build(\([^)]*\))?:',
                r'^ci(\([^)]*\))?:'
            ]
        }
    
    def get_commits_since_tag(self, since_tag: Optional[str] = None) -> List[Dict]:
        """Get commits since the specified tag or all commits if no tag."""
        try:
            if since_tag:
                cmd = ["git", "log", f"{since_tag}..HEAD", "--oneline", "--format=%H|%s|%an|%ad", "--date=short"]
            else:
                cmd = ["git", "log", "--oneline", "--format=%H|%s|%an|%ad", "--date=short", "-20"]
            
            result = subprocess.run(cmd, capture_output=True, text=True, encoding='utf-8', errors='replace', cwd=self.root_path)
            if result.returncode != 0:
                print(f"Warning: Could not get git commits: {result.stderr}")
                return []
            
            commits = []
            for line in result.stdout.strip().split('\n'):
                if line:
                    parts = line.split('|', 3)
                    if len(parts) >= 4:
                        commits.append({
                            'hash': parts[0],
                            'message': parts[1],
                            'author': parts[2],
                            'date': parts[3]
                        })
            
            return commits
            
        except Exception as e:
            print(f"Error getting commits: {e}")
            return []
    
    def categorize_commit(self, commit_message: str) -> Tuple[ChangeType, bool, Optional[str]]:
        """Categorize a commit message and extract scope."""
        message_lower = commit_message.lower()
        
        # Check for breaking changes first
        breaking_change = any(
            re.search(pattern, commit_message, re.IGNORECASE)
            for pattern in self.commit_patterns[ChangeType.BREAKING]
        )
        
        # Extract scope from conventional commit format
        scope_match = re.search(r'\(([^)]+)\):', commit_message)
        scope = scope_match.group(1) if scope_match else None
        
        # Categorize by patterns
        for change_type, patterns in self.commit_patterns.items():
            if change_type == ChangeType.BREAKING:
                continue  # Already handled above
                
            for pattern in patterns:
                if re.search(pattern, commit_message, re.IGNORECASE):
                    return change_type, breaking_change, scope
        
        # Default categorization based on keywords
        if any(word in message_lower for word in ['add', 'implement', 'create', 'new']):
            return ChangeType.FEATURE, breaking_change, scope
        elif any(word in message_lower for word in ['fix', 'resolve', 'correct', 'patch']):
            return ChangeType.FIX, breaking_change, scope
        elif any(word in message_lower for word in ['update', 'improve', 'enhance', 'optimize']):
            return ChangeType.FEATURE, breaking_change, scope
        elif any(word in message_lower for word in ['remove', 'delete', 'drop']):
            return ChangeType.REMOVED, breaking_change, scope
        elif any(word in message_lower for word in ['chore', 'maintenance', 'refactor', 'cleanup']):
            return ChangeType.CHORE, breaking_change, scope
        else:
            # Default to "Others" for unrecognizable patterns
            return ChangeType.OTHERS, breaking_change, scope
    
    def clean_commit_message(self, message: str) -> str:
        """Clean and format commit message for changelog."""
        # Remove conventional commit prefixes
        cleaned = re.sub(r'^(feat|fix|docs|style|refactor|perf|test|chore|build|ci|security)(\([^)]*\))?:\s*', '', message)
        
        # Remove square bracket prefixes like [Feature], [Fix], etc.
        cleaned = re.sub(r'^\[[^\]]+\]\s*', '', cleaned)
        
        # Remove PR numbers at the end
        cleaned = re.sub(r'\s*\(#\d+\)$', '', cleaned)
        
        # Capitalize first letter
        if cleaned:
            cleaned = cleaned[0].upper() + cleaned[1:]
        
        return cleaned.strip()
    
    def extract_pr_number(self, commit_message: str) -> Optional[int]:
        """Extract PR number from commit message."""
        pr_match = re.search(r'#(\d+)', commit_message)
        return int(pr_match.group(1)) if pr_match else None
    
    def generate_changelog_entries(self, commits: List[Dict]) -> Dict[ChangeType, List[ChangelogEntry]]:
        """Generate changelog entries from commits."""
        entries_by_type = {change_type: [] for change_type in ChangeType}
        
        for commit in commits:
            message = commit['message']
            
            # Skip merge commits and version bump commits
            if (message.startswith('Merge ') or 
                'bump version' in message.lower() or
                'chore: bump version' in message.lower()):
                continue
            
            change_type, breaking_change, scope = self.categorize_commit(message)
            cleaned_message = self.clean_commit_message(message)
            pr_number = self.extract_pr_number(message)
            
            entry = ChangelogEntry(
                change_type=change_type,
                description=cleaned_message,
                commit_hash=commit['hash'][:8],
                pr_number=pr_number,
                breaking_change=breaking_change,
                scope=scope,
                author=commit['author'],
                date=commit['date']
            )
            
            entries_by_type[change_type].append(entry)
        
        return entries_by_type
    
    def format_changelog_section(self, section: ChangelogSection) -> str:
        """Format a changelog section with detailed attribution."""
        if not section.entries:
            return ""

        lines = [f"### {section.title}\n"]

        for entry in section.entries:
            # Format entry with comprehensive attribution
            description = entry.description

            if entry.scope:
                description = f"**{entry.scope}**: {description}"

            # Add author attribution
            attribution_parts = []
            if entry.author:
                attribution_parts.append(f"by @{entry.author.replace(' ', '-').lower()}")

            # Add PR number with link
            if entry.pr_number:
                attribution_parts.append(f"in [#{entry.pr_number}](https://github.com/{self.github_repo}/pull/{entry.pr_number})")

            # Add commit hash reference
            if entry.commit_hash:
                short_hash = entry.commit_hash[:7]
                attribution_parts.append(f"([{short_hash}](https://github.com/{self.github_repo}/commit/{entry.commit_hash}))")

            # Add date if available
            if entry.date:
                attribution_parts.append(f"on {entry.date}")

            # Combine description with attribution
            if attribution_parts:
                description += f" {' '.join(attribution_parts)}"

            # Add breaking change indicator
            if entry.breaking_change and entry.change_type != ChangeType.BREAKING:
                description += " ⚠️ **BREAKING CHANGE**"

            lines.append(f"- {description}")

        lines.append("")  # Empty line after section
        return "\n".join(lines)

    def generate_full_changelog_entry(self, version: str, entries_by_type: Dict[ChangeType, List[ChangelogEntry]]) -> str:
        """Generate a complete changelog entry for a version."""
        today = datetime.now().strftime('%Y-%m-%d')
        lines = [f"## [{version}] - {today}\n"]

        # Sort sections by priority
        sorted_sections = []
        for change_type in ChangeType:
            if entries_by_type[change_type]:
                section = self.change_type_config[change_type]
                section.entries = entries_by_type[change_type]
                sorted_sections.append(section)

        sorted_sections.sort(key=lambda x: x.priority)

        # Generate sections
        for section in sorted_sections:
            section_content = self.format_changelog_section(section)
            if section_content:
                lines.append(section_content)

        # Add summary statistics
        total_changes = sum(len(entries) for entries in entries_by_type.values())
        breaking_changes = sum(
            len([e for e in entries if e.breaking_change])
            for entries in entries_by_type.values()
        )

        if total_changes > 0:
            lines.append("---")
            lines.append(f"**Total Changes**: {total_changes}")

            # Add changes per category
            category_stats = []
            for section in sorted_sections:
                if section.entries:
                    count = len(section.entries)
                    category_name = section.title.split(' ', 1)[1]  # Remove emoji
                    category_stats.append(f"{section.emoji} {category_name}: {count}")

            if category_stats:
                lines.append(f"**Changes by Category**: {' | '.join(category_stats)}")

            if breaking_changes > 0:
                lines.append(f"**Breaking Changes**: {breaking_changes} ⚠️")
            lines.append("")

        return "\n".join(lines)

    def update_changelog_file(self, version: str, changelog_entry: str, preview_only: bool = False) -> bool:
        """Update the CHANGELOG.md file with new entry."""
        changelog_path = self.root_path / "CHANGELOG.md"

        if not changelog_path.exists():
            print("❌ CHANGELOG.md not found")
            return False

        try:
            with open(changelog_path, 'r', encoding='utf-8') as f:
                content = f.read()

            # Find insertion point (after title and before first version)
            lines = content.split('\n')

            # Find where to insert (after "# Changelog" and any intro text)
            insert_index = 1  # Default after title
            for i, line in enumerate(lines):
                if line.strip().startswith('## ['):  # First version entry
                    insert_index = i
                    break
                elif i > 0 and line.strip() == '':  # First empty line after title
                    insert_index = i + 1

            # Insert new entry
            lines.insert(insert_index, changelog_entry.rstrip())
            lines.insert(insert_index + 1, "")  # Add spacing

            new_content = '\n'.join(lines)

            if preview_only:
                print("📋 CHANGELOG PREVIEW:")
                print("=" * 60)
                print(changelog_entry)
                print("=" * 60)
                return True
            else:
                with open(changelog_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"✅ Updated CHANGELOG.md with version {version}")
                return True

        except Exception as e:
            print(f"❌ Error updating CHANGELOG.md: {e}")
            return False

    def validate_changelog(self) -> bool:
        """Validate the current CHANGELOG.md format and consistency."""
        changelog_path = self.root_path / "CHANGELOG.md"

        if not changelog_path.exists():
            print("❌ CHANGELOG.md not found")
            return False

        try:
            with open(changelog_path, 'r', encoding='utf-8') as f:
                content = f.read()

            issues = []

            # Check basic structure
            if not content.startswith('# Changelog'):
                issues.append("Missing '# Changelog' title")

            # Check version format
            version_pattern = r'## \[[^\]]+\] - \d{4}-\d{2}-\d{2}'
            versions = re.findall(version_pattern, content)

            if not versions:
                issues.append("No properly formatted version entries found")

            # Check for duplicate versions
            version_numbers = re.findall(r'## \[([^\]]+)\]', content)
            if len(version_numbers) != len(set(version_numbers)):
                issues.append("Duplicate version entries detected")

            if issues:
                print("❌ CHANGELOG validation issues:")
                for issue in issues:
                    print(f"   • {issue}")
                return False
            else:
                print("✅ CHANGELOG.md validation passed")
                return True

        except Exception as e:
            print(f"❌ Error validating CHANGELOG.md: {e}")
            return False

    def get_latest_tag(self) -> Optional[str]:
        """Get the latest git tag."""
        try:
            result = subprocess.run(
                ["git", "describe", "--tags", "--abbrev=0"],
                capture_output=True,
                text=True,
                encoding='utf-8',
                errors='replace',
                cwd=self.root_path
            )
            if result.returncode == 0:
                return result.stdout.strip()
        except Exception:
            pass
        return None


def main():
    parser = argparse.ArgumentParser(
        description="Advanced Changelog Generation for GraphBit",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/generate-changelog.py --version 0.4.0
  python scripts/generate-changelog.py --version 0.4.0 --since-tag v0.3.0
  python scripts/generate-changelog.py --validate-only
  python scripts/generate-changelog.py --preview --version 0.4.0
        """
    )

    parser.add_argument("--version", type=str, help="Version for the changelog entry")
    parser.add_argument("--since-tag", type=str, help="Generate changelog since this tag")
    parser.add_argument("--preview", action="store_true", help="Preview changelog without updating file")
    parser.add_argument("--validate-only", action="store_true", help="Only validate existing changelog")
    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Root directory of the project")

    args = parser.parse_args()

    generator = AdvancedChangelogGenerator(args.root)

    if args.validate_only:
        success = generator.validate_changelog()
        exit(0 if success else 1)

    if not args.version:
        print("❌ Version is required (use --version)")
        exit(1)

    print(f"Generating changelog for version {args.version}")

    # Determine since tag
    since_tag = args.since_tag
    if not since_tag:
        since_tag = generator.get_latest_tag()
        if since_tag:
            print(f"📍 Using latest tag as baseline: {since_tag}")
        else:
            print("📍 No previous tag found, using recent commits")

    # Get commits
    commits = generator.get_commits_since_tag(since_tag)
    if not commits:
        print("⚠️  No commits found for changelog generation")
        exit(1)

    print(f"Analyzing {len(commits)} commits...")

    # Generate entries
    entries_by_type = generator.generate_changelog_entries(commits)

    # Generate changelog entry
    changelog_entry = generator.generate_full_changelog_entry(args.version, entries_by_type)

    # Update or preview
    success = generator.update_changelog_file(args.version, changelog_entry, args.preview)

    if success and not args.preview:
        print(f"Successfully generated changelog for version {args.version}")

        # Show summary
        total_changes = sum(len(entries) for entries in entries_by_type.values())
        print(f"Summary: {total_changes} changes categorized")

        for change_type, entries in entries_by_type.items():
            if entries:
                section = generator.change_type_config[change_type]
                print(f"   {section.title}: {len(entries)}")

    exit(0 if success else 1)


if __name__ == "__main__":
    main()
