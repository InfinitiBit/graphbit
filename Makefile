# GraphBit Development Makefile - Comprehensive Build & Test System
# ==================================================================
# This Makefile provides comprehensive build, test, and environment management
# for the GraphBit project, supporting both Rust and Python components with
# cross-platform compatibility (Windows PowerShell and Unix-like systems).

# Load environment variables from .env if present
ifneq (,$(wildcard .env))
	export $(shell sed 's/=.*//' .env)
endif

# Default environment type (can be overridden by .env)
ENV_TYPE ?= poetry

# Detect shell/platform type for cross-platform support
ifeq ($(OS),Windows_NT)
	SHELL_TYPE := windows
	DETECTED_OS := Windows
	POETRY_CHECK := where poetry >nul 2>&1
	VENV_ACTIVATE := .venv\Scripts\activate.bat
	ENV_VAR_SET := set
	PATH_SEP := ;
	NULL_DEVICE := NUL
else
	SHELL_TYPE := unix
	DETECTED_OS := $(shell uname -s)
	POETRY_CHECK := command -v poetry >/dev/null 2>&1
	VENV_ACTIVATE := .venv/bin/activate
	ENV_VAR_SET := export
	PATH_SEP := :
	NULL_DEVICE := /dev/null
endif

# Python environment activation command based on ENV_TYPE and platform
ifeq ($(ENV_TYPE),conda)
	ifeq ($(SHELL_TYPE),windows)
		PYTHON_ENV := conda activate graphbit &&
		PYTHON_CMD := conda run -n graphbit python
	else
		PYTHON_ENV := conda activate graphbit &&
		PYTHON_CMD := conda run -n graphbit python
	endif
else ifeq ($(ENV_TYPE),venv)
	ifeq ($(SHELL_TYPE),windows)
		PYTHON_ENV := call $(VENV_ACTIVATE) &&
		PYTHON_CMD := python
	else
		PYTHON_ENV := . $(VENV_ACTIVATE) &&
		PYTHON_CMD := python
	endif
else ifeq ($(ENV_TYPE),poetry)
	PYTHON_ENV := poetry run
	PYTHON_CMD := poetry run python
else
	# Default fallback to conda
	ifeq ($(SHELL_TYPE),windows)
		PYTHON_ENV := conda activate graphbit &&
		PYTHON_CMD := conda run -n graphbit python
	else
		PYTHON_ENV := conda activate graphbit &&
		PYTHON_CMD := conda run -n graphbit python
	endif
endif

# Test environment variables for cross-platform support
ifeq ($(SHELL_TYPE),windows)
	TEST_ENV_VARS := powershell -Command "$$env:TEST_REMOTE_URLS='true';"
	COVERAGE_CONFIG :=
else
	TEST_ENV_VARS := TEST_REMOTE_URLS=true
	COVERAGE_CONFIG := --cov-config=/dev/null
endif

# Define all phony targets
.PHONY: help install clean test test-rust test-python lint lint-rust lint-python \
        format format-rust format-python build docs dev-setup all-checks ci \
        secrets secrets-audit secrets-baseline secrets-update \
        build-perf install-perf test-perf benchmark-perf \
        quick quick-python pre-commit-install pre-commit-run pre-commit-update pre-commit-clean \
        examples watch-test watch-check release-check typos lint-fix format-check test-integration test-coverage \
        create-env create-conda-env create-venv-env check-env check-poetry check-venv init \
        install-poetry install-maturin verify-environment

# ==================================================================
# HELP & INFORMATION
# ==================================================================

help: ## Show this help message with all available targets
	@echo "GraphBit Development Makefile - Comprehensive Build & Test System"
	@echo "=================================================================="
	@echo ""
	@echo "Detected Environment:"
	@echo "  Platform: $(DETECTED_OS)"
	@echo "  Shell Type: $(SHELL_TYPE)"
	@echo "  Environment Type: $(ENV_TYPE)"
	@echo "  Python Command: $(PYTHON_CMD)"
	@echo ""
	@echo "Available Commands:"
	@echo ""
ifeq ($(SHELL_TYPE),windows)
	@powershell -Command "Get-Content Makefile | Select-String '^[a-zA-Z0-9_-]+:.*## .*' | ForEach-Object { $$parts = $$_.Line -split ':.*?## '; Write-Host ('  {0,-25} {1}' -f $$parts[0], $$parts[1]) }"
else
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2}'
endif
	@echo ""
	@echo "Environment Variables:"
	@echo "  ENV_TYPE: Environment type (poetry, conda, venv) [current: $(ENV_TYPE)]"
	@echo "  OPENAI_API_KEY: Required for LLM-based tests and examples"
	@echo "  TEST_REMOTE_URLS: Enable remote URL testing [default: true]"
	@echo ""
	@echo "Quick Start:"
	@echo "  make install    # Install all dependencies and build components"
	@echo "  make test       # Run comprehensive test suites"
	@echo "  make clean      # Clean all build artifacts"

# ==================================================================
# ENVIRONMENT SETUP & VERIFICATION
# ==================================================================

check-poetry: ## Check if Poetry is installed and install if missing
	@echo "Checking Poetry installation..."
ifeq ($(SHELL_TYPE),windows)
	@powershell -Command "if (!(Get-Command poetry -ErrorAction SilentlyContinue)) { \
		Write-Host 'Poetry not found. Installing Poetry...'; \
		(Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | python -; \
		Write-Host 'Poetry installed successfully!'; \
	} else { \
		Write-Host 'Poetry is already installed.'; \
	}"
else
	@if ! $(POETRY_CHECK); then \
		echo "Poetry not found. Installing Poetry..."; \
		curl -sSL https://install.python-poetry.org | python3 -; \
		echo "Poetry installed successfully!"; \
		echo "Please restart your shell or run: source ~/.bashrc"; \
	else \
		echo "Poetry is already installed."; \
	fi
endif

check-venv: ## Check if virtual environment is activated
	@echo "Checking virtual environment status..."
ifeq ($(ENV_TYPE),conda)
	@echo "Using conda environment: graphbit"
	@conda info --envs | grep -q "^graphbit " || { \
		echo "Conda environment 'graphbit' not found. Run 'make create-conda-env' first."; \
		exit 1; \
	}
else ifeq ($(ENV_TYPE),venv)
	@if [ -z "$$VIRTUAL_ENV" ]; then \
		echo "No virtual environment detected."; \
		echo "Please activate your virtual environment or run 'make create-venv-env'."; \
		exit 1; \
	else \
		echo "Virtual environment active: $$VIRTUAL_ENV"; \
	fi
else ifeq ($(ENV_TYPE),poetry)
	@echo "Using Poetry for dependency management."
else
	@echo "Unknown ENV_TYPE: $(ENV_TYPE). Please set ENV_TYPE to poetry, conda, or venv."
	@exit 1
endif

verify-environment: check-poetry check-venv ## Verify all environment prerequisites
	@echo "Environment verification completed successfully!"

check-env: ## Interactive setup for .env file if missing
	@if [ ! -f .env ]; then \
		echo "$(EMOJI_INFO) .env not found. Let's set it up interactively."; \
		read -p "Choose ENV_TYPE (poetry/conda/venv) [poetry]: " ENV_TYPE_INPUT; \
		ENV_TYPE_INPUT=$${ENV_TYPE_INPUT:-poetry}; \
		read -p "Enter your OPENAI_API_KEY [sk-xxxxx]: " API_KEY_INPUT; \
		API_KEY_INPUT=$${API_KEY_INPUT:-sk-xxxxx}; \
		echo "ENV_TYPE=$$ENV_TYPE_INPUT" > .env; \
		echo "OPENAI_API_KEY=$$API_KEY_INPUT" >> .env; \
		echo ".env created with defaults."; \
	else \
		echo "$(EMOJI_SUCCESS) .env already exists."; \
	fi

# ==================================================================
# ENVIRONMENT CREATION
# ==================================================================

create-env: ## Create environment based on ENV_TYPE
ifeq ($(ENV_TYPE),conda)
	$(MAKE) create-conda-env
else ifeq ($(ENV_TYPE),venv)
	$(MAKE) create-venv-env
else ifeq ($(ENV_TYPE),poetry)
	@echo "Using Poetry for dependency management - no separate environment needed."
else
	@echo "Unknown ENV_TYPE: $(ENV_TYPE). Defaulting to conda."
	$(MAKE) create-conda-env
endif

create-conda-env: ## Create conda environment if it doesn't exist
	@echo "Checking if conda environment 'graphbit' exists..."
	@conda info --envs | grep -q "^graphbit " || { \
		echo "Creating conda environment 'graphbit' with Python 3.11..."; \
		conda create -n graphbit python=3.11 -y; \
		echo "Conda environment 'graphbit' created successfully!"; \
	}

create-venv-env: ## Create Python virtual environment if it doesn't exist
	@echo "Checking if virtual environment exists..."
	@if [ ! -d ".venv" ]; then \
		echo "Creating Python virtual environment..."; \
		python -m venv .venv; \
		echo "Virtual environment created successfully!"; \
		echo "Please activate it with: source .venv/bin/activate (Unix) or .venv\\Scripts\\activate.bat (Windows)"; \
	else \
		echo "Virtual environment already exists."; \
	fi

init: check-env create-env install ## Complete first-time project setup
	@echo "Project environment initialized completely!"

# ==================================================================
# INSTALLATION & DEPENDENCY MANAGEMENT
# ==================================================================

install: verify-environment install-rust-deps install-python-deps build-python-bindings ## Install all dependencies and build components
	@echo "All dependencies installed and components built successfully!"

install-rust-deps: ## Install Rust dependencies
	@echo "Installing Rust dependencies..."
	cargo fetch
	@echo "Rust dependencies installed."

install-python-deps: ## Install Python dependencies using Poetry
	@echo "Installing Python dependencies..."
ifeq ($(ENV_TYPE),poetry)
	poetry install --with dev,benchmarks
else ifeq ($(ENV_TYPE),conda)
	conda run -n graphbit pip install poetry
	conda run -n graphbit poetry install --with dev,benchmarks
else ifeq ($(ENV_TYPE),venv)
	$(PYTHON_ENV) pip install poetry
	$(PYTHON_ENV) poetry install --with dev,benchmarks
endif
	@echo "Python dependencies installed."

build-python-bindings: ## Build Python bindings using maturin
	@echo "Building Python bindings..."
	cargo build --package graphbit-core --package graphbit-python
ifeq ($(ENV_TYPE),poetry)
	poetry run maturin develop --release --manifest-path python/Cargo.toml
else ifeq ($(ENV_TYPE),conda)
	conda run -n graphbit maturin develop --release --manifest-path python/Cargo.toml
else ifeq ($(ENV_TYPE),venv)
	$(PYTHON_ENV) maturin develop --release --manifest-path python/Cargo.toml
endif
	@echo "Python bindings built and installed successfully."

clean: ## Clean all build artifacts and temporary files
	@echo "Cleaning build artifacts..."
	cargo clean
ifeq ($(SHELL_TYPE),windows)
	@powershell -Command "Get-ChildItem -Path . -Recurse -Name '__pycache__' | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue"
	@powershell -Command "Get-ChildItem -Path . -Recurse -Name '*.pyc' | Remove-Item -Force -ErrorAction SilentlyContinue"
	@powershell -Command "Get-ChildItem -Path . -Recurse -Name '*.egg-info' | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue"
else
	@find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	@find . -type f -name "*.pyc" -delete 2>/dev/null || true
	@find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
endif
	@echo "Build artifacts cleaned."

# ==================================================================
# COMPREHENSIVE TESTING SYSTEM
# ==================================================================

test: test-rust test-python ## Run comprehensive test suites for both Rust and Python
	@echo "All tests completed successfully!"

test-rust: ## Run Rust tests using cargo llvm-cov with HTML output
	@echo "Running Rust tests with coverage..."
	cargo llvm-cov --package graphbit-core --package graphbit --html --output-dir target/llvm-cov \
		--ignore-filename-regex '.*/(tests?|benches?|core/src/llm|python/src)/.*'
	@echo "Rust tests completed. Coverage report: target/llvm-cov/index.html"

test-python: build-python-bindings ## Run Python tests using pytest with comprehensive coverage
	@echo "Running Python tests with coverage..."
ifeq ($(SHELL_TYPE),windows)
	@powershell -Command "if (-not $$env:OPENAI_API_KEY) { Write-Host 'Warning: OPENAI_API_KEY not set. Some tests may be skipped.' }"
	@powershell -Command "$$env:TEST_REMOTE_URLS='true'; poetry run pytest --import-mode=importlib -v --tb=short --strict-markers --durations=10 --cov=graphbit --cov-report=term-missing --cov-report=html:target/coverage/python --cov-report=xml:target/coverage/python.xml"
else
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "Warning: OPENAI_API_KEY not set. Some tests may be skipped."; \
	fi
	$(TEST_ENV_VARS) $(PYTHON_ENV) pytest \
		--import-mode=importlib \
		-v \
		--tb=short \
		--strict-markers \
		--durations=10 \
		--cov=graphbit \
		--cov-report=term-missing \
		--cov-report=html:target/coverage/python \
		--cov-report=xml:target/coverage/python.xml \
		$(COVERAGE_CONFIG)
endif
	@echo "Python tests completed. Coverage report: target/coverage/python/index.html"

test-coverage: test-rust test-python ## Run comprehensive coverage analysis for both Rust and Python
	@echo "Coverage analysis completed!"
	@echo "Rust coverage: target/llvm-cov/index.html"
	@echo "Python coverage: target/coverage/python/index.html"

# ==================================================================
# CODE QUALITY & FORMATTING
# ==================================================================

lint: lint-rust lint-python ## Run all linting checks

lint-rust: ## Run Rust linting with clippy
	@echo "Running Rust linting..."
	cargo clippy --workspace --all-targets --all-features -- -D warnings

lint-python: ## Run Python linting with flake8 and mypy
	@echo "Running Python linting..."
	$(PYTHON_ENV) flake8 graphbit/ tests/ benchmarks/
	$(PYTHON_ENV) mypy graphbit/ --ignore-missing-imports

lint-fix: ## Auto-fix linting issues where possible
	@echo "Auto-fixing linting issues..."
	cargo clippy --workspace --all-targets --all-features --fix --allow-staged --allow-dirty
	$(PYTHON_ENV) isort graphbit/ tests/ benchmarks/

format: format-rust format-python ## Format all code

format-rust: ## Format Rust code
	@echo "Formatting Rust code..."
	cargo fmt --all

format-python: ## Format Python code
	@echo "Formatting Python code..."
	$(PYTHON_ENV) black graphbit/ tests/ benchmarks/
	$(PYTHON_ENV) isort graphbit/ tests/ benchmarks/

format-check: ## Check code formatting without making changes
	@echo "Checking code formatting..."
	cargo fmt --all -- --check
	$(PYTHON_ENV) black --check graphbit/ tests/ benchmarks/
	$(PYTHON_ENV) isort --check-only graphbit/ tests/ benchmarks/

# ==================================================================
# BUILD SYSTEM
# ==================================================================

build: ## Build all components in release mode
	@echo "Building all components..."
	cargo build --workspace --release
	$(PYTHON_ENV) poetry build
	@echo "Build completed successfully!"

build-dev: ## Build components in development mode
	@echo "Building components in development mode..."
	cargo build --package graphbit-core --package graphbit-python
ifeq ($(ENV_TYPE),poetry)
	poetry run maturin develop --manifest-path python/Cargo.toml
else ifeq ($(ENV_TYPE),conda)
	conda run -n graphbit maturin develop --manifest-path python/Cargo.toml
else ifeq ($(ENV_TYPE),venv)
	$(PYTHON_ENV) maturin develop --manifest-path python/Cargo.toml
endif
	@echo "Development build completed!"

# ==================================================================
# DOCUMENTATION & SECURITY
# ==================================================================

docs: ## Generate documentation
	@echo "Generating documentation..."
	cargo doc --workspace --no-deps --open
	$(PYTHON_ENV) cd docs && make html

docs-serve: ## Serve documentation locally
	@echo "Serving documentation on http://localhost:8000"
	$(PYTHON_ENV) cd docs && python -m http.server 8000

security: ## Run security audits
	@echo "Running security audits..."
	cargo audit
	$(PYTHON_ENV) safety check
	$(PYTHON_ENV) bandit -r graphbit/
	$(MAKE) secrets

secrets: ## Scan for secrets in codebase
	$(PYTHON_ENV) detect-secrets scan --baseline .secrets.baseline

secrets-audit: ## Audit detected secrets
	$(PYTHON_ENV) detect-secrets scan --baseline .secrets.baseline
	$(PYTHON_ENV) detect-secrets audit .secrets.baseline

secrets-baseline: ## Create new secrets baseline
	$(PYTHON_ENV) detect-secrets scan > .secrets.baseline

secrets-update: ## Update secrets configuration
	@echo "Edit .secrets.baseline to update detect-secrets configuration"
	@echo "See: https://github.com/Yelp/detect-secrets"

typos: ## Check for typos in codebase
	typos

# ==================================================================
# BENCHMARKING & PERFORMANCE
# ==================================================================

bench: ## Run benchmarks
	@echo "Running benchmarks..."
	cargo bench
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "Warning: OPENAI_API_KEY not set. Some benchmarks may be skipped."; \
	fi
	$(PYTHON_ENV) python -m benchmarks.run_benchmarks

build-perf: ## Build with performance optimizations
	@echo "Building GraphBit with performance optimizations..."
	cargo build --release --features performance
	$(PYTHON_ENV) maturin develop --release

install-perf: build-perf ## Install with performance optimizations
	@echo "Installing GraphBit with performance optimizations..."
	$(PYTHON_ENV) pip install -e python/

test-perf: build-perf ## Run performance tests
	@echo "Running performance tests..."
	$(PYTHON_CMD) performance_test.py

benchmark-perf: build-perf ## Run comprehensive benchmarks
	@echo "Running comprehensive benchmarks..."
	$(PYTHON_CMD) benchmarks/run_comprehensive_benchmark.py

# ==================================================================
# PRE-COMMIT HOOKS & DEVELOPMENT TOOLS
# ==================================================================

pre-commit-install: ## Install pre-commit hooks
	@echo "Installing pre-commit hooks..."
	$(PYTHON_ENV) pre-commit install
	$(PYTHON_ENV) pre-commit install --hook-type commit-msg
	$(PYTHON_ENV) pre-commit install --hook-type pre-push

pre-commit-run: ## Run pre-commit hooks on all files
	$(PYTHON_ENV) pre-commit run --all-files

pre-commit-update: ## Update pre-commit hooks
	$(PYTHON_ENV) pre-commit autoupdate

pre-commit-clean: ## Clean pre-commit cache
	$(PYTHON_ENV) pre-commit clean

# ==================================================================
# CI/CD & RELEASE MANAGEMENT
# ==================================================================

all-checks: format-check lint test secrets ## Run all quality checks
	@echo "All checks passed successfully!"

ci: clean all-checks ## Run complete CI pipeline
	@echo "CI pipeline completed successfully!"

release-check: all-checks docs ## Check readiness for release
	@echo "Checking release readiness..."
	cargo publish --dry-run
	$(PYTHON_ENV) poetry check
	@echo "Release check completed!"

# ==================================================================
# QUICK DEVELOPMENT COMMANDS
# ==================================================================

quick: format-rust lint-rust test-rust ## Quick Rust development cycle

quick-python: format-python lint-python test-python ## Quick Python development cycle

dev-setup: verify-environment install pre-commit-install ## Complete development environment setup
	@echo "Development environment setup completed!"

# ==================================================================
# EXAMPLES & DEVELOPMENT TOOLS
# ==================================================================

examples: ## Run example workflows
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "Warning: OPENAI_API_KEY not set. Examples may not work properly."; \
	fi
	$(PYTHON_ENV) python examples/basic_workflow.py

watch-test: ## Watch for changes and run tests automatically
	cargo watch -x "test --workspace"

watch-check: ## Watch for changes and run checks automatically
	cargo watch -x "check --workspace" -x "clippy --workspace"

# ==================================================================
# UTILITY TARGETS
# ==================================================================

.DEFAULT_GOAL := help
