#!/usr/bin/env bash

# test_python_versions.sh
# Run GraphBit build & core checks against specific Python versions using Poetry.

set -euo pipefail

# ----------------- Configuration -----------------

PYTHON_VERSIONS=("3.9.2" "3.11")

REPO_ROOT="$(pwd)"
LOGS_DIR="$REPO_ROOT/logs"
CHECKER_SCRIPT="$REPO_ROOT/scripts/core-functionalities-checker.py"

# Global for current log file
LOG_FILE=""

# ----------------- Helpers -----------------

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    local color="$1"
    local message="$2"
    echo -e "${color}${message}${NC}"
}

log() {
    local message="$1"
    if [ -n "${LOG_FILE:-}" ]; then
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] $message" >> "$LOG_FILE"
    fi
}

get_poetry_venv_root() {
    # Use last line in case poetry prints extra info
    local venv_root
    venv_root="$(poetry config virtualenvs.path 2>/dev/null | tail -n 1 || true)"
    if [ -z "$venv_root" ]; then
        print_status "$RED" "Could not determine Poetry virtualenvs.path. Aborting to avoid rm -rf on an empty path."
        exit 1
    fi
    echo "$venv_root"
}

resolve_python_binary() {
    # Resolve a python binary that exactly matches the given version (e.g. 3.9.2)
    local desired_version="$1"
    local major_minor="${desired_version%.*}"
    local candidate=""

    # Try pythonX.Y.Z first
    if command -v "python${desired_version}" >/dev/null 2>&1; then
        candidate="python${desired_version}"
    elif command -v "python${major_minor}" >/dev/null 2>&1; then
        candidate="python${major_minor}"
    else
        return 1
    fi

    # Validate exact version via sys.version_info
    local full_ver
    full_ver="$("$candidate" -c 'import sys; print(".".join(map(str, sys.version_info[:3])))' 2>/dev/null || echo "")"

    if [ "$full_ver" != "$desired_version" ]; then
        return 1
    fi

    PY_BIN="$candidate"
    return 0
}

check_prerequisites() {
    print_status "$BLUE" "Checking prerequisites..."

    if ! command -v poetry >/dev/null 2>&1; then
        print_status "$RED" "Poetry is not installed. Please install Poetry first."
        exit 1
    fi

    if ! command -v cargo >/dev/null 2>&1; then
        print_status "$RED" "Cargo (Rust) is not installed. Please install Rust & Cargo."
        exit 1
    fi

    if [ ! -f "$REPO_ROOT/pyproject.toml" ]; then
        print_status "$RED" "pyproject.toml not found. Please run this script from the repository root."
        exit 1
    fi

    if [ ! -d "$REPO_ROOT/core" ]; then
        print_status "$RED" "core/ directory not found in repo root."
        exit 1
    fi

    if [ ! -d "$REPO_ROOT/python" ]; then
        print_status "$RED" "python/ directory not found in repo root."
        exit 1
    fi

    if [ ! -f "$CHECKER_SCRIPT" ]; then
        print_status "$RED" "Checker script not found: $CHECKER_SCRIPT"
        exit 1
    fi

    print_status "$GREEN" "Prerequisites OK."
}

# ----------------- Core per-version routine -----------------

test_python_version() {
    local python_version="$1"
    LOG_FILE="$LOGS_DIR/python_${python_version}.log"
    : > "$LOG_FILE"  # truncate / create

    print_status "$BLUE" "=== Testing Python $python_version ==="
    log "Starting test for Python $python_version"

    # 1) Deactivate any currently active venv (if running in same shell)
    if command -v deactivate >/dev/null 2>&1; then
        log "Deactivating existing virtualenv"
        deactivate || true
    fi

    # 2) Remove all Poetry virtualenvs
    local venv_root
    venv_root="$(get_poetry_venv_root)"
    log "Removing Poetry virtualenvs at: $venv_root"
    rm -rf "$venv_root"

    # 3) Refresh lockfile
    print_status "$YELLOW" "  Running: poetry lock"
    log "Running: poetry lock"
    if ! poetry lock >>"$LOG_FILE" 2>&1; then
        print_status "$RED" "  poetry lock failed"
        log "ERROR: poetry lock failed"
        return 1
    fi

    # 4) Resolve Python binary & configure Poetry to use it
    print_status "$YELLOW" "  Resolving Python $python_version"
    if ! resolve_python_binary "$python_version"; then
        print_status "$RED" "  Could not find a python binary for exact version $python_version"
        log "ERROR: No suitable Python binary found for version $python_version"
        return 1
    fi
    log "Using Python binary: $PY_BIN"

    print_status "$YELLOW" "  Running: poetry env use $PY_BIN"
    log "Running: poetry env use $PY_BIN"
    if ! poetry env use "$PY_BIN" >>"$LOG_FILE" 2>&1; then
        print_status "$RED" "  poetry env use failed for $PY_BIN"
        log "ERROR: poetry env use failed for $PY_BIN"
        return 1
    fi

    # 5) Verify Poetry env is *exactly* the requested Python version
    local actual_version
    actual_version="$(poetry run python -c 'import sys; print(".".join(map(str, sys.version_info[:3])))' 2>>"$LOG_FILE" || true)"
    log "Poetry env Python version: $actual_version"

    if [ "$actual_version" != "$python_version" ]; then
        print_status "$RED" "  Poetry env is using Python $actual_version, expected $python_version"
        log "ERROR: Poetry env using Python $actual_version, expected $python_version"
        return 1
    fi
    print_status "$GREEN" "  Verified Poetry env uses Python $python_version"

    # 6) Activate the Poetry env in this shell
    local venv_path
    venv_path="$(poetry env info --path 2>>"$LOG_FILE" || true)"
    if [ -z "$venv_path" ] || [ ! -d "$venv_path" ]; then
        print_status "$RED" "  Failed to get Poetry env path"
        log "ERROR: poetry env info --path returned '$venv_path'"
        return 1
    fi
    log "Activating env at: $venv_path"

    # shellcheck source=/dev/null
    . "$venv_path/bin/activate"

    # 7) Install dependencies
    print_status "$YELLOW" "  Running: poetry install"
    log "Running: poetry install"
    if ! poetry install --no-root >>"$LOG_FILE" 2>&1; then
        print_status "$RED" "  poetry install failed"
        log "ERROR: poetry install failed"
        return 1
    fi

    # 8) cargo clean in core/
    print_status "$YELLOW" "  Running: cargo clean (core/)"
    log "Running: (cd core && cargo clean)"
    if ! (cd core && cargo clean >>"$LOG_FILE" 2>&1); then
        print_status "$RED" "  cargo clean failed in core/"
        log "ERROR: cargo clean failed in core/"
        return 1
    fi

    # 9) cargo build in core/
    print_status "$YELLOW" "  Running: cargo build (core/)"
    log "Running: (cd core && cargo build)"
    if ! (cd core && cargo build >>"$LOG_FILE" 2>&1); then
        print_status "$RED" "  cargo build failed in core/"
        log "ERROR: cargo build failed in core/"
        return 1
    fi

    # 10) maturin develop in python/
    print_status "$YELLOW" "  Running: maturin develop (python/)"
    log "Running: (cd python && maturin develop)"
    if ! (cd python && maturin develop >>"$LOG_FILE" 2>&1); then
        print_status "$RED" "  maturin develop failed in python/"
        log "ERROR: maturin develop failed in python/"
        return 1
    fi

    # 11) Run core functionalities checker with this Poetry env
    print_status "$YELLOW" "  Running: core-functionalities-checker.py"
    log "Running: poetry run python $CHECKER_SCRIPT"
    if ! poetry run python "$CHECKER_SCRIPT" >>"$LOG_FILE" 2>&1; then
        print_status "$RED" "  core-functionalities-checker.py failed"
        log "ERROR: core-functionalities-checker.py failed"
        return 1
    fi

    print_status "$GREEN" "  Python $python_version: SUCCESS"
    log "All steps completed successfully for Python $python_version"
    return 0
}

# ----------------- Main -----------------

trap 'print_status "'"$YELLOW"'" "Script interrupted. Exiting..."; exit 1' INT TERM

main() {
    print_status "$BLUE" "GraphBit Python Version Support Test"
    print_status "$BLUE" "===================================="

    check_prerequisites

    mkdir -p "$LOGS_DIR"

    local total="${#PYTHON_VERSIONS[@]}"
    local passed=()
    local failed=()

    print_status "$BLUE" "Testing Python versions: ${PYTHON_VERSIONS[*]}"
    echo

    for version in "${PYTHON_VERSIONS[@]}"; do
        if test_python_version "$version"; then
            passed+=("$version")
        else
            failed+=("$version")
            print_status "$YELLOW" "  See log: $LOGS_DIR/python_${version}.log"
        fi
        echo
    done

    print_status "$BLUE" "========== Summary =========="
    print_status "$GREEN" "Passed: ${#passed[@]}/$total"
    if [ "${#passed[@]}" -gt 0 ]; then
        print_status "$GREEN" "  ${passed[*]}"
    fi

    if [ "${#failed[@]}" -gt 0 ]; then
        print_status "$RED" "Failed: ${#failed[@]}/$total"
        print_status "$RED" "  ${failed[*]}"
        print_status "$YELLOW" "Check logs in: $LOGS_DIR"
        exit 1
    fi

    print_status "$GREEN" "All configured Python versions passed."
}

main "$@"
