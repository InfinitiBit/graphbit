# GraphBit Development Makefile (Modular Version)
# ===============================================
# Main Makefile that orchestrates all development tasks
# Uses modular sub-Makefiles for better organization and maintainability

# Include all modular Makefiles
include makefiles/config.mk
include makefiles/platform.mk
include makefiles/validation.mk
include makefiles/rust.mk
include makefiles/python.mk
include makefiles/common.mk

# Main targets that provide unified interface
.PHONY: help test tests test-all test-coverage test-quick
.PHONY: test-rust test-rust-unit test-rust-integration
.PHONY: test-python test-python-unit test-python-integration
.PHONY: lint lint-rust lint-python format format-rust format-python
.PHONY: build build-dev build-release clean clean-all
.PHONY: install install-dev install-quick install-force
.PHONY: docs docs-serve security pre-commit examples benchmark
.PHONY: all-checks ci release-check quick quick-python
.PHONY: init check-env create-env validate-all validate-quick

# ===============================================
# MAIN INTERFACE TARGETS
# ===============================================
# These targets provide a unified interface and maintain backward compatibility

# Default target
.DEFAULT_GOAL := help

help: ## Show this help message with all available targets
	$(call print_header,"$(EMOJI_ROCKET) GraphBit Development Commands")
	@echo ""
	@echo "$(COLOR_BOLD)$(COLOR_GREEN)🚀 Quick Start:$(COLOR_RESET)"
	@echo "  $(COLOR_CYAN)make install$(COLOR_RESET)    - Complete setup (Poetry, dependencies, Cargo build, Maturin)"
	@echo "  $(COLOR_CYAN)make test$(COLOR_RESET)       - Run all tests (Rust + Python)"
	@echo "  $(COLOR_CYAN)make validate-all$(COLOR_RESET) - Validate all paths and tools"
	@echo ""
	@echo "$(COLOR_BOLD)$(COLOR_BLUE)📋 Main Categories:$(COLOR_RESET)"
	@echo "  $(COLOR_YELLOW)Testing:$(COLOR_RESET)     test, test-rust, test-python, test-coverage"
	@echo "  $(COLOR_YELLOW)Building:$(COLOR_RESET)    build, build-dev, build-release"
	@echo "  $(COLOR_YELLOW)Quality:$(COLOR_RESET)     lint, format, security, all-checks"
	@echo "  $(COLOR_YELLOW)Setup:$(COLOR_RESET)       install, install-dev, validate-all"
	@echo ""
	@echo "$(COLOR_BOLD)$(COLOR_MAGENTA)🔧 Environment Variables:$(COLOR_RESET)"
	@echo "  $(COLOR_CYAN)ENV_TYPE$(COLOR_RESET):      Environment type (poetry, conda, venv) [current: $(ENV_TYPE)]"
	@echo "  $(COLOR_CYAN)OPENAI_API_KEY$(COLOR_RESET): Required for LLM-based tasks and Python tests"
	@echo ""
	@echo "$(COLOR_BOLD)$(COLOR_GREEN)💡 Detailed Help:$(COLOR_RESET)"
	@echo "  $(COLOR_CYAN)make help-rust$(COLOR_RESET)   - Show Rust-specific commands"
	@echo "  $(COLOR_CYAN)make help-python$(COLOR_RESET) - Show Python-specific commands"
	@echo "  $(COLOR_CYAN)make help-all$(COLOR_RESET)    - Show all available commands"
	@echo ""
	@echo "$(COLOR_BOLD)$(COLOR_GREEN)🎯 Pro Tip:$(COLOR_RESET) Run '$(COLOR_CYAN)make install$(COLOR_RESET)' first, then '$(COLOR_CYAN)make test$(COLOR_RESET)' to verify everything works!"

help-all: ## Show all available commands from all Makefiles
	$(call print_header,"$(EMOJI_INFO) All Available Commands")
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) makefiles/*.mk | \
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(COLOR_CYAN)%-30s$(COLOR_RESET) %s\n", $$1, $$2}'

help-rust: ## Show Rust-specific commands
	@echo "Rust Commands"
	@echo "================================================"
	@echo "  rust-test-coverage        - Run all Rust tests with coverage reporting"
	@echo "  rust-test-quick           - Run Rust tests without coverage (faster)"
	@echo "  rust-test-unit            - Run only Rust unit tests with coverage"
	@echo "  rust-test-integration     - Run only Rust integration tests with coverage"
	@echo "  rust-build                - Build Rust workspace in debug mode"
	@echo "  rust-build-release        - Build Rust workspace in release mode"
	@echo "  rust-clean                - Clean Rust build artifacts"
	@echo "  rust-clippy               - Run Clippy linter on Rust code"
	@echo "  rust-format               - Format Rust code"
	@echo "  rust-format-check         - Check Rust code formatting"

help-python: ## Show Python-specific commands
	@echo "Python Commands"
	@echo "================================================"
	@echo "  python-test-coverage      - Run all Python tests with coverage reporting"
	@echo "  python-test-quick         - Run Python tests without coverage (faster)"
	@echo "  python-test-unit          - Run only Python unit tests with coverage"
	@echo "  python-test-integration   - Run only Python integration tests with coverage"
	@echo "  python-install            - Install Python dependencies"
	@echo "  python-install-dev        - Install Python development dependencies"
	@echo "  python-build              - Build Python package"
	@echo "  python-clean              - Clean Python build artifacts and cache"
	@echo "  python-lint               - Run Python linting (flake8 + mypy)"
	@echo "  python-format             - Format Python code with black and isort"

# ===============================================
# UNIFIED TEST INTERFACE
# ===============================================
# These targets provide backward compatibility and unified access

# Main test targets
test: ## Run comprehensive test suite for both Rust and Python components
	$(call print_header,"$(EMOJI_TEST) Starting Comprehensive Test Suite")
	@$(MAKE) rust-test-coverage
	@$(MAKE) python-test-coverage
	$(call format_test_results)
	$(call print_success,"$(EMOJI_SUCCESS) All tests completed successfully!")

tests: test ## Alias for 'test' command

test-all: validate-test-env ## Run comprehensive test suite with full validation
	$(call print_header,"$(EMOJI_TEST) Starting Comprehensive Test Suite with Validation")
	@$(MAKE) validate-installation
	@$(MAKE) rust-test-coverage
	@$(MAKE) python-test-coverage
	$(call format_test_results)
	$(call print_success,"$(EMOJI_SUCCESS) All comprehensive tests completed successfully!")

test-coverage: ## Run tests with coverage reporting for both languages
	$(call print_header,"$(EMOJI_TEST) Running Tests with Coverage Reporting")
	@$(MAKE) rust-test-coverage
	@$(MAKE) python-test-coverage
	$(call format_test_results)
	$(call print_success,"$(EMOJI_SUCCESS) Coverage reports generated!")

test-quick: ## Quick test run without coverage (for development iteration)
	$(call print_header,"$(EMOJI_TEST) Running Quick Test Suite")
	@$(MAKE) rust-test-quick
	@$(MAKE) python-test-quick
	$(call format_test_results)
	$(call print_success,"$(EMOJI_SUCCESS) Quick tests completed successfully!")

# Language-specific test aliases (backward compatibility)
test-rust: rust-test-coverage ## Run all Rust tests with coverage
test-rust-unit: rust-test-unit ## Run only Rust unit tests
test-rust-integration: rust-test-integration ## Run only Rust integration tests

test-python: python-test-coverage ## Run all Python tests with coverage
test-python-unit: python-test-unit ## Run only Python unit tests
test-python-integration: python-test-integration ## Run only Python integration tests

# ===============================================
# UNIFIED BUILD AND DEVELOPMENT INTERFACE
# ===============================================

# Linting and formatting (unified interface)
lint: ## Run linting for both Rust and Python
	$(call print_header,"$(EMOJI_INFO) Running Linting for All Components")
	@$(MAKE) rust-lint
	@$(MAKE) python-lint
	$(call print_success,"$(MSG_SUCCESS)")

lint-rust: rust-clippy ## Run Rust linting
lint-python: python-lint ## Run Python linting

format: ## Format code for both Rust and Python
	$(call print_header,"$(EMOJI_INFO) Formatting All Code")
	@$(MAKE) rust-format
	@$(MAKE) python-format
	$(call print_success,"$(MSG_SUCCESS)")

format-rust: rust-format ## Format Rust code
format-python: python-format ## Format Python code

format-check: ## Check code formatting for both languages
	$(call print_header,"$(EMOJI_INFO) Checking Code Formatting")
	@$(MAKE) rust-format-check
	@$(MAKE) python-format-check
	$(call print_success,"$(MSG_SUCCESS)")

# ===============================================
# INTERACTIVE SETUP AND INITIALIZATION
# ===============================================

check-env: ## Ask interactively if .env is missing (with defaults)
	$(call print_header,"$(EMOJI_INFO) Environment Configuration Setup")
	@if [ ! -f .env ]; then \
		echo "$(EMOJI_INFO) .env not found. Let's set it up interactively."; \
		read -p "Choose ENV_TYPE (poetry/conda/venv) [poetry]: " ENV_TYPE_INPUT; \
		ENV_TYPE_INPUT=$${ENV_TYPE_INPUT:-poetry}; \
		read -p "Enter your OPENAI_API_KEY [sk-xxxxx]: " API_KEY_INPUT; \
		API_KEY_INPUT=$${API_KEY_INPUT:-sk-xxxxx}; \
		echo "ENV_TYPE=$$ENV_TYPE_INPUT" > .env; \
		echo "OPENAI_API_KEY=$$API_KEY_INPUT" >> .env; \
		echo "$(EMOJI_SUCCESS) .env created with defaults."; \
	else \
		echo "$(EMOJI_SUCCESS) .env already exists."; \
	fi

init: check-env create-conda-env install-dev ## One-click first-time project setup
	$(call print_success,"$(EMOJI_SUCCESS) Project environment initialized completely!")

create-env: create-conda-env ## Create environment based on ENV_TYPE (alias)

# ===============================================
# COMPREHENSIVE QUALITY CHECKS AND CI
# ===============================================

all-checks: format-check lint test security ## Run all quality checks (CI-ready)
	$(call print_header,"$(EMOJI_SUCCESS) All Quality Checks")
	$(call print_success,"$(EMOJI_SUCCESS) All checks passed! Ready for CI/CD.")

ci: clean all-checks ## Complete CI pipeline
	$(call print_header,"$(EMOJI_ROCKET) CI Pipeline")
	$(call print_success,"$(EMOJI_SUCCESS) CI pipeline completed successfully!")

release-check: all-checks docs ## Pre-release validation
	$(call print_header,"$(EMOJI_INFO) Release Validation")
	@cargo publish --dry-run
	@$(PYTHON_ENV) poetry check
	$(call print_success,"$(EMOJI_SUCCESS) Release validation completed!")

# Quick development checks
quick: rust-quick-checks ## Quick Rust checks for development
quick-python: python-quick-checks ## Quick Python checks for development

# Pre-commit integration
pre-commit-install: ## Install pre-commit hooks
	$(call print_header,"$(EMOJI_INFO) Installing Pre-commit Hooks")
	@$(PYTHON_ENV) pre-commit install
	@$(PYTHON_ENV) pre-commit install --hook-type commit-msg
	@$(PYTHON_ENV) pre-commit install --hook-type pre-push
	$(call print_success,"$(MSG_SUCCESS)")

pre-commit-run: pre-commit ## Run pre-commit hooks on all files (alias)

pre-commit-update: ## Update pre-commit hooks
	$(call print_header,"$(EMOJI_INFO) Updating Pre-commit Hooks")
	@$(PYTHON_ENV) pre-commit autoupdate
	$(call print_success,"$(MSG_SUCCESS)")

pre-commit-clean: ## Clean pre-commit cache
	$(call print_header,"$(EMOJI_CLEAN) Cleaning Pre-commit Cache")
	@$(PYTHON_ENV) pre-commit clean
	$(call print_success,"$(MSG_SUCCESS)")

# ===============================================
# DEBUG AND UTILITIES
# ===============================================

debug-env: ## Show environment configuration
	@echo "ENV_TYPE: $(ENV_TYPE)"
	@echo "SHELL_TYPE: $(SHELL_TYPE)"
	@echo "PYTHON_ENV: $(PYTHON_ENV)"
	@echo "PYTEST_CMD: $(PYTEST_CMD)"

# ===============================================
# ADDITIONAL UTILITIES AND LEGACY SUPPORT
# ===============================================

# Secrets and security scanning
secrets-audit: ## Audit secrets baseline
	$(call print_header,"$(EMOJI_INFO) Auditing Secrets")
	@$(PYTHON_ENV) detect-secrets scan --baseline .secrets.baseline
	@$(PYTHON_ENV) detect-secrets audit .secrets.baseline
	$(call print_success,"$(MSG_SUCCESS)")

secrets-baseline: ## Create new secrets baseline
	$(call print_header,"$(EMOJI_INFO) Creating Secrets Baseline")
	@$(PYTHON_ENV) detect-secrets scan > .secrets.baseline
	$(call print_success,"$(MSG_SUCCESS)")

secrets-update: ## Update secrets configuration
	@echo "$(EMOJI_INFO) Edit .secrets.baseline to update detect-secrets configuration"
	@echo "$(EMOJI_INFO) See: https://github.com/Yelp/detect-secrets"

typos: ## Check for typos in codebase
	$(call print_header,"$(EMOJI_INFO) Checking for Typos")
	@typos
	$(call print_success,"$(MSG_SUCCESS)")

# Development watch modes (legacy aliases)
watch-test: rust-watch-test ## Watch for changes and run Rust tests
watch-check: rust-watch-check ## Watch for changes and run Rust check + clippy

# Performance and benchmarking
build-perf: ## Build with performance optimizations
	$(call print_header,"$(EMOJI_BUILD) Building with Performance Optimizations")
	@conda run -n $(PROJECT_NAME) cargo build --release --features performance
	@conda run -n $(PROJECT_NAME) maturin develop --release
	$(call print_success,"$(MSG_SUCCESS)")

install-perf: build-perf ## Install with performance optimizations
	$(call print_header,"$(EMOJI_ROCKET) Installing with Performance Optimizations")
	@conda run -n $(PROJECT_NAME) pip install -e python/
	$(call print_success,"$(MSG_SUCCESS)")

test-perf: build-perf ## Run performance tests
	$(call print_header,"$(EMOJI_TEST) Running Performance Tests")
	@conda run -n $(PROJECT_NAME) python performance_test.py
	$(call print_success,"$(MSG_SUCCESS)")

benchmark-perf: build-perf ## Run comprehensive performance benchmarks
	$(call print_header,"$(EMOJI_TEST) Running Comprehensive Benchmarks")
	@conda run -n $(PROJECT_NAME) python $(BENCHMARKS_SRC)/run_comprehensive_benchmark.py
	$(call print_success,"$(MSG_SUCCESS)")

# Legacy aliases for backward compatibility
dev-setup: install-dev ## Legacy alias for install-dev
lint-fix: rust-clippy-fix python-format ## Legacy alias for auto-fixing
test-integration: test-all ## Legacy alias for comprehensive tests

# Note: All other targets are now defined in the included makefiles:
# - makefiles/config.mk     - Configuration and variables
# - makefiles/platform.mk   - Cross-platform compatibility
# - makefiles/validation.mk - Path and environment validation
# - makefiles/rust.mk       - Rust-specific targets
# - makefiles/python.mk     - Python-specific targets
# - makefiles/common.mk     - Common utilities and installation


# ===============================================
# DOCUMENTATION AND FINAL NOTES
# ===============================================

# All targets are now defined in the included makefiles:
# - makefiles/config.mk     - Configuration and variables
# - makefiles/platform.mk   - Cross-platform compatibility
# - makefiles/validation.mk - Path and environment validation
# - makefiles/rust.mk       - Rust-specific targets
# - makefiles/python.mk     - Python-specific targets
# - makefiles/common.mk     - Common utilities and installation

# The modular structure provides:
# ✅ Better organization and maintainability
# ✅ Cross-platform compatibility (Windows/Unix)
# ✅ Centralized configuration management
# ✅ Proper path validation and error handling
# ✅ Backward compatibility with existing commands
# ✅ Enhanced developer experience with colored output and emojis

# For detailed help on specific components:
# make help-rust    - Show Rust-specific commands
# make help-python  - Show Python-specific commands
# make help-all     - Show all available commands
