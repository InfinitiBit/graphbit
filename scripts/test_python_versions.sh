#!/bin/bash

# test_python_versions.sh
# Script to test GraphBit repository against Python versions using Poetry and aarch64-apple-darwin

set -e  # Exit on any error

# Configuration
PYTHON_VERSIONS=("3.9.2" "3.13.8")
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
LOGS_DIR="$REPO_ROOT/logs"
EXAMPLE_SCRIPT="$REPO_ROOT/examples/manual_check_core_version_support.py"

# Force Rust builds to use Apple Silicon target
RUST_TARGET="aarch64-apple-darwin"
export CARGO_BUILD_TARGET="$RUST_TARGET"

# Resolved python binary for current version under test
PY_BIN=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

log_with_timestamp() {
    local message=$1
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $message"
}

create_logs_dir() {
    if [ ! -d "$LOGS_DIR" ]; then
        mkdir -p "$LOGS_DIR"
        print_status $GREEN "Created logs directory: $LOGS_DIR"
    fi
}

check_python_version() {
    local version=$1

    # Try exact match: version
    if command -v "python$version" >/dev/null 2>&1; then
        PY_BIN="python$version"
        return 0
    fi

    local major_minor="${version%.*}"
    if command -v "python$major_minor" >/dev/null 2>&1; then
        # Get the full version from this binary
        local full_ver
        full_ver=$("python$major_minor" -c 'import sys; print(".".join(map(str, sys.version_info[:3])))' 2>/dev/null || echo "")

        if [ "$full_ver" = "$version" ]; then
            PY_BIN="python$major_minor"
            return 0
        fi
    fi

    # If we reach here, we couldn't find a matching binary
    PY_BIN=""
    return 1
}

# Ensure Rust toolchain + target are ready
check_rust_setup() {
    if ! command -v rustup >/dev/null 2>&1; then
        print_status $RED "❌ rustup is not installed. Please install Rust via rustup."
        exit 1
    fi

    # Install the desired target if missing
    if ! rustup target list --installed | grep -q "^$RUST_TARGET$"; then
        print_status $YELLOW "⚠️ Rust target $RUST_TARGET not found. Adding it..."
        rustup target add "$RUST_TARGET"
        print_status $GREEN "✅ Rust target $RUST_TARGET installed."
    else
        print_status $GREEN "✅ Rust target $RUST_TARGET already installed."
    fi
}

test_python_version() {
    local python_version=$1
    local log_file="$LOGS_DIR/${python_version}_support_test.log"

    print_status $BLUE "Testing Python $python_version..."
    log_with_timestamp "Starting test for Python $python_version (target: $RUST_TARGET)" > "$log_file"

    # Check if Python version is available
    if ! check_python_version "$python_version"; then
        local error_msg="Python $python_version is not available on this system (no suitable python binary found)"
        print_status $RED "❌ $error_msg"
        log_with_timestamp "ERROR: $error_msg" >> "$log_file"
        return 1
    fi

    cd "$REPO_ROOT"

    # Step 1: Create Poetry virtual environment
    print_status $YELLOW "  Creating Poetry environment with Python $python_version (binary: $PY_BIN)..."
    if ! poetry env use "$PY_BIN" >> "$log_file" 2>&1; then
        local error_msg="Failed to create Poetry environment with Python $python_version"
        print_status $RED "  ❌ $error_msg"
        log_with_timestamp "ERROR: $error_msg" >> "$log_file"
        return 1
    fi
    log_with_timestamp "Poetry environment created successfully" >> "$log_file"

    # Step 2: Install dependencies (without building the root package)
    print_status $YELLOW "  Installing dependencies..."
    if ! poetry install --no-root >> "$log_file" 2>&1; then
        local error_msg="Failed to install dependencies"
        print_status $RED "  ❌ $error_msg"
        log_with_timestamp "ERROR: $error_msg" >> "$log_file"
        print_status $YELLOW "  Last 20 log lines for Python $python_version:"
        tail -n 20 "$log_file" || true
        cleanup_environment "$python_version"
        return 1
    fi
    log_with_timestamp "Dependencies installed successfully" >> "$log_file"

    # Step 3: Build Python bindings with maturin (into THIS Poetry env)
    print_status $YELLOW "  Building Python bindings with maturin (target: $RUST_TARGET)..."
    if ! poetry run maturin develop \
        --target "$RUST_TARGET" \
        --manifest-path "$REPO_ROOT/python/Cargo.toml" >> "$log_file" 2>&1; then
        local error_msg="Failed to build Python bindings with maturin"
        print_status $RED "  ❌ $error_msg"
        log_with_timestamp "ERROR: $error_msg" >> "$log_file"
        print_status $YELLOW "  Last 20 log lines for Python $python_version:"
        tail -n 20 "$log_file" || true
        cleanup_environment "$python_version"
        return 1
    fi
    log_with_timestamp "Python bindings built successfully" >> "$log_file"

    # Step 4: Run the example script
    print_status $YELLOW "  Running example script..."
    if ! poetry run python "$EXAMPLE_SCRIPT" >> "$log_file" 2>&1; then
        local error_msg="Example script execution failed"
        print_status $RED "  ❌ $error_msg"
        log_with_timestamp "ERROR: $error_msg" >> "$log_file"
        print_status $YELLOW "  Last 20 log lines for Python $python_version:"
        tail -n 20 "$log_file" || true
        cleanup_environment "$python_version"
        return 1
    fi
    log_with_timestamp "Example script executed successfully" >> "$log_file"

    # Step 5: Run the test suite
    print_status $YELLOW "  Running test suite..."
    if ! poetry run pytest -v tests >> "$log_file" 2>&1; then
        local error_msg="Test suite execution failed"
        print_status $RED "  ❌ $error_msg"
        log_with_timestamp "ERROR: $error_msg" >> "$log_file"
        print_status $YELLOW "  Last 20 log lines for Python $python_version:"
        tail -n 20 "$log_file" || true
        cleanup_environment "$python_version"
        return 1
    fi
    log_with_timestamp "Test suite completed successfully" >> "$log_file"

    # Step 6: Cleanup
    cleanup_environment "$python_version"

    print_status $GREEN "  ✅ Python $python_version test completed successfully"
    log_with_timestamp "All tests completed successfully for Python $python_version" >> "$log_file"
    return 0
}

cleanup_environment() {
    local python_version=$1
    print_status $YELLOW "  Cleaning up environment..."

    if [ -n "$PY_BIN" ] && poetry env remove "$PY_BIN" >/dev/null 2>&1; then
        print_status $GREEN "  ✅ Environment cleaned up"
    else
        print_status $YELLOW "  ⚠️ Environment cleanup completed (may have been already removed)"
    fi
}

check_prerequisites() {
    print_status $BLUE "Checking prerequisites..."

    if ! command -v poetry >/dev/null 2>&1; then
        print_status $RED "❌ Poetry is not installed. Please install Poetry first."
        print_status $YELLOW "   Visit: https://python-poetry.org/docs/#installation"
        exit 1
    fi
    print_status $GREEN "✅ Poetry is available"

    if [ ! -f "$REPO_ROOT/pyproject.toml" ]; then
        print_status $RED "❌ pyproject.toml not found. Please run this script from the repository root or scripts directory."
        exit 1
    fi
    print_status $GREEN "✅ Repository structure verified"

    if [ ! -f "$EXAMPLE_SCRIPT" ]; then
        print_status $RED "❌ Example script not found: $EXAMPLE_SCRIPT"
        exit 1
    fi
    print_status $GREEN "✅ Example script found"

    if ! poetry run maturin --version >/dev/null 2>&1; then
        print_status $RED "❌ Maturin is not available in the Poetry environment"
        print_status $YELLOW "   Please ensure maturin is listed in dev-dependencies"
        exit 1
    fi
    print_status $GREEN "✅ Maturin is available"

    check_rust_setup
}

main() {
    print_status $BLUE "🚀 GraphBit Python Version Support Test (Python $PYTHON_VERSIONS / $RUST_TARGET)"
    print_status $BLUE "==================================================================="

    check_prerequisites
    create_logs_dir

    local total_versions=${#PYTHON_VERSIONS[@]}
    local passed_versions=()
    local failed_versions=()

    print_status $BLUE "\nTesting against Python versions: ${PYTHON_VERSIONS[*]}"
    print_status $BLUE "Logs will be saved to: $LOGS_DIR"
    print_status $BLUE "===================================================================\n"

    for version in "${PYTHON_VERSIONS[@]}"; do
        if test_python_version "$version"; then
            passed_versions+=("$version")
        else
            failed_versions+=("$version")
        fi
        echo
    done

    print_status $BLUE "=========================================="
    print_status $BLUE "📊 Test Summary"
    print_status $BLUE "=========================================="
    print_status $GREEN "✅ Passed: ${#passed_versions[@]}/$total_versions versions"
    if [ ${#passed_versions[@]} -gt 0 ]; then
        print_status $GREEN "   Successful versions: ${passed_versions[*]}"
    fi

    if [ ${#failed_versions[@]} -gt 0 ]; then
        print_status $RED "❌ Failed: ${#failed_versions[@]}/$total_versions versions"
        print_status $RED "   Failed versions: ${failed_versions[*]}"
        print_status $YELLOW "   Check individual log files in $LOGS_DIR for details"
    fi

    print_status $BLUE "=========================================="

    if [ ${#failed_versions[@]} -eq 0 ]; then
        print_status $GREEN "🎉 All Python versions passed!"
        exit 0
    else
        print_status $RED "❌ Some Python versions failed. Check logs for details."
        exit 1
    fi
}

trap 'print_status $YELLOW "\n⚠️ Script interrupted. Cleaning up..."; exit 1' INT TERM

main "$@"
