#!/usr/bin/env python3
"""
Simple version bump script for GraphBit.

Usage:
  python scripts/bump-version.py --set 0.6.0 [--dry-run]
  python scripts/bump-version.py --part (major|minor|patch) [--dry-run]

Effects:
- Updates version in:
  - python/pyproject.toml  -> [project].version
  - pyproject.toml         -> [project].version and [tool.poetry].version (if present)
  - Cargo.toml             -> [workspace.package].version

Notes:
- Preserves file formatting by targeted regex replacement within the relevant sections.
- Prints a summary of changes. Use --dry-run to preview without writing.
"""
from __future__ import annotations
import argparse
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]

PY_PYPROJECT = ROOT / "python" / "pyproject.toml"
ROOT_PYPROJECT = ROOT / "pyproject.toml"
ROOT_CARGO = ROOT / "Cargo.toml"

SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


def read_text(p: pathlib.Path) -> str:
    return p.read_text(encoding="utf-8")


def write_text(p: pathlib.Path, content: str, dry: bool) -> None:
    if dry:
        return
    p.write_text(content, encoding="utf-8")


def bump_part(v: str, part: str) -> str:
    m = SEMVER_RE.match(v)
    if not m:
        raise ValueError(f"Not a semver: {v}")
    major, minor, patch = map(int, m.groups())
    if part == "major":
        return f"{major+1}.0.0"
    if part == "minor":
        return f"{major}.{minor+1}.0"
    if part == "patch":
        return f"{major}.{minor}.{patch+1}"
    raise ValueError(part)


def replace_version_in_project_section(toml: str, new_version: str) -> tuple[str, bool]:
    # Find [project] section and replace first version = "..." inside it
    pattern = re.compile(
        r'(\n\[project\][\s\S]*?\n)(version\s*=\s*\")([^\"]+)(\"\s*)(\n)',
        re.MULTILINE,
    )
    def _repl(m: re.Match) -> str:
        return f"{m.group(1)}{m.group(2)}{new_version}{m.group(4)}{m.group(5)}"
    new_toml, n = pattern.subn(_repl, toml, count=1)
    return new_toml, n > 0


def replace_poetry_version(toml: str, new_version: str) -> tuple[str, bool]:
    pattern = re.compile(
        r'(\n\[tool\.poetry\][\s\S]*?\n)(version\s*=\s*\")([^\"]+)(\"\s*)(\n)',
        re.MULTILINE,
    )
    def _repl(m: re.Match) -> str:
        return f"{m.group(1)}{m.group(2)}{new_version}{m.group(4)}{m.group(5)}"
    new_toml, n = pattern.subn(_repl, toml, count=1)
    return new_toml, n > 0


def replace_workspace_package_version(toml: str, new_version: str) -> tuple[str, bool]:
    pattern = re.compile(
        r'(\n\[workspace\.package\][\s\S]*?\n)(version\s*=\s*\")([^\"]+)(\"\s*)(\n)',
        re.MULTILINE,
    )
    def _repl(m: re.Match) -> str:
        return f"{m.group(1)}{m.group(2)}{new_version}{m.group(4)}{m.group(5)}"
    new_toml, n = pattern.subn(_repl, toml, count=1)
    return new_toml, n > 0


def get_current_versions() -> dict:
    versions: dict[str, str | None] = {
        "python/pyproject.toml": None,
        "pyproject.toml:[project]": None,
        "pyproject.toml:[tool.poetry]": None,
        "Cargo.toml:[workspace.package]": None,
    }
    if PY_PYPROJECT.exists():
        m = re.search(r"\n\[project\][\s\S]*?\nversion\s*=\s*\"([^\"]+)\"", read_text(PY_PYPROJECT), re.MULTILINE)
        if m:
            versions["python/pyproject.toml"] = m.group(1)
    if ROOT_PYPROJECT.exists():
        txt = read_text(ROOT_PYPROJECT)
        m1 = re.search(r"\n\[project\][\s\S]*?\nversion\s*=\s*\"([^\"]+)\"", txt, re.MULTILINE)
        if m1:
            versions["pyproject.toml:[project]"] = m1.group(1)
        m2 = re.search(r"\n\[tool\.poetry\][\s\S]*?\nversion\s*=\s*\"([^\"]+)\"", txt, re.MULTILINE)
        if m2:
            versions["pyproject.toml:[tool.poetry]"] = m2.group(1)
    if ROOT_CARGO.exists():
        m = re.search(r"\n\[workspace\.package\][\s\S]*?\nversion\s*=\s*\"([^\"]+)\"", read_text(ROOT_CARGO), re.MULTILINE)
        if m:
            versions["Cargo.toml:[workspace.package]"] = m.group(1)
    return versions


def main() -> int:
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--set", dest="set_version", help="Set exact version (e.g., 0.6.0)")
    g.add_argument("--part", choices=["major", "minor", "patch"], help="Bump semver part")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    current = get_current_versions()
    # Choose baseline current version from python/pyproject if available
    baseline = current.get("python/pyproject.toml") or current.get("pyproject.toml:[project]") or current.get("Cargo.toml:[workspace.package]")
    if not baseline:
        print("Could not determine current version.", file=sys.stderr)
        return 2

    if args.set_version:
        new_version = args.set_version
        if not SEMVER_RE.match(new_version):
            print("--set must be a semantic version like 1.2.3", file=sys.stderr)
            return 2
    else:
        new_version = bump_part(str(baseline), args.part)

    print("Current versions:")
    for k, v in current.items():
        print(f"  {k}: {v}")
    print(f"New version: {new_version}")

    # Apply replacements
    # 1) python/pyproject.toml [project].version
    if PY_PYPROJECT.exists():
        txt = read_text(PY_PYPROJECT)
        new_txt, changed = replace_version_in_project_section(txt, new_version)
        if changed:
            write_text(PY_PYPROJECT, new_txt, args.dry_run)
            print(f"Updated: {PY_PYPROJECT}")
        else:
            print(f"No change in: {PY_PYPROJECT}")

    # 2) root pyproject.toml [project].version and [tool.poetry].version
    if ROOT_PYPROJECT.exists():
        txt = read_text(ROOT_PYPROJECT)
        txt2, ch1 = replace_version_in_project_section(txt, new_version)
        txt3, ch2 = replace_poetry_version(txt2, new_version)
        if ch1 or ch2:
            write_text(ROOT_PYPROJECT, txt3, args.dry_run)
            print(f"Updated: {ROOT_PYPROJECT}")
        else:
            print(f"No change in: {ROOT_PYPROJECT}")

    # 3) root Cargo.toml [workspace.package].version
    if ROOT_CARGO.exists():
        txt = read_text(ROOT_CARGO)
        new_txt, changed = replace_workspace_package_version(txt, new_version)
        if changed:
            write_text(ROOT_CARGO, new_txt, args.dry_run)
            print(f"Updated: {ROOT_CARGO}")
        else:
            print(f"No change in: {ROOT_CARGO}")

    print("Done." + (" (dry-run)" if args.dry_run else ""))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())