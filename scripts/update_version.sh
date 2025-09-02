#!/bin/bash
# GraphBit Version Management Script (Shell wrapper)
# 
# This script provides a shell interface to the Python version management script
# with additional validation and convenience features.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory and repository root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
PYTHON_SCRIPT="$SCRIPT_DIR/update_version.py"

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check prerequisites
check_prerequisites() {
    print_info "Checking prerequisites..."
    
    # Check if Python is available
    if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
        print_error "Python is not installed or not in PATH"
        exit 1
    fi
    
    # Determine Python command
    if command -v python3 &> /dev/null; then
        PYTHON_CMD="python3"
    else
        PYTHON_CMD="python"
    fi
    
    # Check Python version
    PYTHON_VERSION=$($PYTHON_CMD --version 2>&1 | cut -d' ' -f2)
    print_info "Using Python $PYTHON_VERSION"
    
    # Check if we're in the right directory
    if [[ ! -f "$REPO_ROOT/Cargo.toml" ]]; then
        print_error "Not in GraphBit repository root (Cargo.toml not found)"
        exit 1
    fi
    
    # Check if the Python script exists
    if [[ ! -f "$PYTHON_SCRIPT" ]]; then
        print_error "Python version script not found: $PYTHON_SCRIPT"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Function to validate version format
validate_version() {
    local version="$1"
    
    # Semantic versioning regex
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$ ]]; then
        print_error "Invalid version format: $version"
        print_error "Expected semantic versioning format: MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]"
        print_error "Examples: 1.0.0, 1.2.3-beta.1, 2.0.0-rc.1+build.123"
        exit 1
    fi
}

# Function to get current version
get_current_version() {
    if [[ -f "$REPO_ROOT/Cargo.toml" ]]; then
        grep -E '^\s*version\s*=' "$REPO_ROOT/Cargo.toml" | head -1 | sed -E 's/.*version\s*=\s*["\']([^"\']+)["\'].*/\1/'
    fi
}

# Function to show help
show_help() {
    cat << EOF
GraphBit Version Management Script

USAGE:
    $0 <new_version> [OPTIONS]

ARGUMENTS:
    <new_version>    New version number (semantic versioning format)

OPTIONS:
    --dry-run        Show what would be changed without making actual changes
    --backup         Create backup before making changes
    --report         Generate a detailed report file
    --force          Skip repository state validation
    --help, -h       Show this help message

EXAMPLES:
    $0 0.2.0                    # Update to version 0.2.0
    $0 1.0.0-beta.1 --dry-run   # Preview changes for beta version
    $0 2.1.3 --backup --report  # Update with backup and report

SEMANTIC VERSIONING:
    MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
    
    Examples:
    - 1.0.0          (stable release)
    - 1.2.3-beta.1   (pre-release)
    - 2.0.0-rc.1     (release candidate)
    - 1.0.0+build.1  (with build metadata)

EOF
}

# Main function
main() {
    local new_version=""
    local python_args=()
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_help
                exit 0
                ;;
            --dry-run)
                python_args+=("--dry-run")
                shift
                ;;
            --backup)
                python_args+=("--backup")
                shift
                ;;
            --report)
                python_args+=("--report")
                shift
                ;;
            --force)
                python_args+=("--force")
                shift
                ;;
            -*)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
            *)
                if [[ -z "$new_version" ]]; then
                    new_version="$1"
                else
                    print_error "Multiple version arguments provided"
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # Check if version was provided
    if [[ -z "$new_version" ]]; then
        print_error "Version number is required"
        show_help
        exit 1
    fi
    
    # Run checks
    check_prerequisites
    validate_version "$new_version"
    
    # Show current version
    current_version=$(get_current_version)
    if [[ -n "$current_version" ]]; then
        print_info "Current version: $current_version"
        if [[ "$current_version" == "$new_version" ]]; then
            print_warning "Version is already $new_version"
            exit 0
        fi
    fi
    
    # Execute Python script
    print_info "Executing version update..."
    cd "$REPO_ROOT"
    
    if $PYTHON_CMD "$PYTHON_SCRIPT" "$new_version" "${python_args[@]}"; then
        print_success "Version update completed successfully"
    else
        print_error "Version update failed"
        exit 1
    fi
}

# Run main function with all arguments
main "$@"
