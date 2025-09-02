# GraphBit Development Makefile (Flat Unified Version)
# -----------------------------------------

# Load environment variables from .env if present
ifneq (,$(wildcard .env))
	export $(shell sed 's/=.*//' .env)
endif

# Default environment type (can be overridden by .env)
ENV_TYPE ?= poetry

# Set shell type for OS-specific logic
ifeq ($(OS),Windows_NT)
	SHELL_TYPE := windows
else
	SHELL_TYPE := unix
endif

# Detect Python environment activation command
ifeq ($(ENV_TYPE),conda)
	PYTHON_ENV := conda activate graphbit
else ifeq ($(ENV_TYPE),venv)
	ifeq ($(SHELL_TYPE),windows)
		PYTHON_ENV := call .venv\Scripts\activate.bat
	else
		PYTHON_ENV := . .venv/bin/activate
	endif
else ifeq ($(ENV_TYPE),poetry)
	PYTHON_ENV := poetry run
else
	PYTHON_ENV := conda activate graphbit
endif

.PHONY: help install clean test test-rust test-python lint lint-rust lint-python \
        format format-rust format-python build docs dev-setup all-checks ci \
        secrets secrets-audit secrets-baseline secrets-update \
        build-perf install-perf test-perf benchmark-perf \
        quick quick-python pre-commit-install pre-commit-run pre-commit-update pre-commit-clean \
        examples watch-test watch-check release-check typos lint-fix format-check test-integration test-coverage \
        create-env create-conda-env check-env init check-poetry check-environment install-dependencies \
        validate-installation install-quick install-force

# -------------------------------------------
#  Help
# -------------------------------------------
help:
	@echo "GraphBit Development Commands:"
	@echo ""
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Environment variables:"
	@echo "  ENV_TYPE: Environment type to use (poetry, conda, venv)"
	@echo "  PYTHON_ENV: Python environment activation logic based on ENV_TYPE"
	@echo "  OPENAI_API_KEY: Required for LLM-based tasks"

# -------------------------------------------
# 🛠 Interactive Setup
# -------------------------------------------
check-env: ## Ask interactively if .env is missing (with defaults)
	@if [ ! -f .env ]; then \
		echo ".env not found. Let's set it up interactively."; \
		read -p "Choose ENV_TYPE (poetry/conda/venv) [poetry]: " ENV_TYPE_INPUT; \
		ENV_TYPE_INPUT=$${ENV_TYPE_INPUT:-poetry}; \
		read -p "Enter your OPENAI_API_KEY [sk-xxxxx]: " API_KEY_INPUT; \
		API_KEY_INPUT=$${API_KEY_INPUT:-sk-xxxxx}; \
		echo "ENV_TYPE=$$ENV_TYPE_INPUT" > .env; \
		echo "OPENAI_API_KEY=$$API_KEY_INPUT" >> .env; \
		echo " .env created with defaults."; \
	else \
		echo ".env already exists."; \
	fi

init: check-env create-env dev-setup ## One-click first-time project setup
	@echo " Project environment initialized completely!"

# -------------------------------------------
#  Environment Creation
# -------------------------------------------
create-env: create-conda-env ## Create environment based on ENV_TYPE
	@echo " Environment creation logic (conda by default)"

create-conda-env: ## Create conda environment if it doesn't exist
	@echo "Checking if conda environment 'graphbit' exists..."
	@conda info --envs | grep -q "^graphbit " || { \
		echo "Creating conda environment 'graphbit' with Python 3.11.0..."; \
		conda create -n graphbit python=3.11.0 -y; \
		echo "Conda environment 'graphbit' created successfully!"; \
	}

dev-setup: ## Set up development environment
	@echo "Setting up development environment..."
	$(PYTHON_ENV) poetry install --with dev,benchmarks
	cargo build --workspace
	$(PYTHON_ENV) pre-commit install
	$(PYTHON_ENV) pre-commit install --hook-type commit-msg
	$(PYTHON_ENV) pre-commit install --hook-type pre-push
	@echo " Development environment ready!"

install: ## Comprehensive install: dependencies, Cargo build, and Maturin development setup
	@echo "🚀 Starting comprehensive installation process..."
	@$(MAKE) check-poetry
	@$(MAKE) check-environment
	@$(MAKE) install-dependencies
	@$(MAKE) validate-installation
	@echo "✅ Comprehensive installation completed successfully!"
	@echo "🎉 Your GraphBit development environment is ready!"

check-poetry: ## Check if Poetry is installed and install if needed
	@echo "🔍 Checking Poetry installation..."
ifeq ($(SHELL_TYPE),windows)
	@powershell -Command "try { \
		if (!(Get-Command poetry -ErrorAction SilentlyContinue)) { \
			Write-Host '📦 Poetry not found. Installing Poetry...' -ForegroundColor Yellow; \
			(Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | python -; \
			Write-Host '✅ Poetry installed successfully!' -ForegroundColor Green; \
			Write-Host '⚠️  Please restart your terminal or run the following command:' -ForegroundColor Yellow; \
			Write-Host '   refreshenv' -ForegroundColor Cyan; \
			Write-Host '   Then run: make install' -ForegroundColor Cyan; \
			exit 1; \
		} else { \
			Write-Host '✅ Poetry is already installed' -ForegroundColor Green; \
		} \
	} catch { \
		Write-Host '❌ Failed to install Poetry automatically' -ForegroundColor Red; \
		Write-Host '💡 Please install Poetry manually:' -ForegroundColor Yellow; \
		Write-Host '   Visit: https://python-poetry.org/docs/#installation' -ForegroundColor Cyan; \
		exit 1; \
	}"
else
	@if ! command -v poetry >/dev/null 2>&1; then \
		echo "📦 Poetry not found. Installing Poetry..."; \
		if curl -sSL https://install.python-poetry.org | python3 -; then \
			echo "✅ Poetry installed successfully!"; \
			echo "⚠️  Please restart your terminal or run:"; \
			echo "   source ~/.bashrc"; \
			echo "   Then run: make install"; \
			exit 1; \
		else \
			echo "❌ Failed to install Poetry automatically"; \
			echo "💡 Please install Poetry manually:"; \
			echo "   Visit: https://python-poetry.org/docs/#installation"; \
			exit 1; \
		fi; \
	else \
		echo "✅ Poetry is already installed"; \
	fi
endif

check-environment: ## Check and validate Python environment
	@echo "🔍 Checking Python environment..."
ifeq ($(ENV_TYPE),conda)
	@echo "🐍 Using Conda environment: graphbit"
	@if ! conda info --envs | grep -q "^graphbit "; then \
		echo "❌ Conda environment 'graphbit' not found!"; \
		echo "💡 Creating conda environment..."; \
		$(MAKE) create-conda-env; \
	else \
		echo "✅ Conda environment 'graphbit' found"; \
	fi
else ifeq ($(ENV_TYPE),venv)
	@echo "🐍 Using virtual environment: .venv"
ifeq ($(SHELL_TYPE),windows)
	@if not exist ".venv\Scripts\activate.bat" ( \
		echo "❌ Virtual environment not found at .venv!"; \
		echo "💡 Please create it with: python -m venv .venv"; \
		echo "💡 Then activate it with: .venv\Scripts\activate.bat"; \
		exit /b 1; \
	) else ( \
		echo "✅ Virtual environment found at .venv"; \
	)
else
	@if [ ! -f ".venv/bin/activate" ]; then \
		echo "❌ Virtual environment not found at .venv!"; \
		echo "💡 Please create it with: python -m venv .venv"; \
		echo "💡 Then activate it with: source .venv/bin/activate"; \
		exit 1; \
	else \
		echo "✅ Virtual environment found at .venv"; \
	fi
endif
else ifeq ($(ENV_TYPE),poetry)
	@echo "🐍 Using Poetry environment management"
	@echo "✅ Poetry will manage the virtual environment automatically"
else
	@echo "⚠️  Unknown ENV_TYPE: $(ENV_TYPE), defaulting to conda"
	@$(MAKE) ENV_TYPE=conda check-environment
endif

install-dependencies: ## Install Python and Rust dependencies, build workspace, and setup development environment
	@echo "📦 Installing Python dependencies with Poetry..."
	@$(PYTHON_ENV) poetry install --with dev,benchmarks || { echo "❌ Poetry install failed"; exit 1; }
	@echo "🦀 Fetching Rust dependencies..."
	@cargo fetch || { echo "❌ Cargo fetch failed"; exit 1; }
	@echo "🔨 Building Rust workspace..."
	@cargo build --workspace || { echo "❌ Cargo build failed"; exit 1; }
	@echo "🐍 Installing Python extension module in development mode..."
	@$(PYTHON_ENV) maturin develop || { echo "❌ Maturin develop failed"; exit 1; }
	@echo "✅ All dependencies installed and workspace built successfully!"

validate-installation: ## Validate that installation was successful
	@echo "🔍 Validating installation..."
	@echo "📋 Checking Poetry environment..."
	@$(PYTHON_ENV) poetry check || echo "⚠️  Poetry check failed"
	@echo "📋 Checking Python packages..."
	@$(PYTHON_ENV) python -c "import sys; print(f'✅ Python {sys.version} is working')"
	@echo "📋 Checking Rust toolchain..."
	@cargo --version || echo "⚠️  Cargo not available"
	@echo "📋 Checking Maturin installation..."
	@$(PYTHON_ENV) python -c "import maturin; print('✅ Maturin is available')" || echo "⚠️  Maturin not available"
	@echo "📋 Checking key Python dependencies..."
	@$(PYTHON_ENV) python -c "import click, rich, typer; print('✅ Core dependencies available')" || echo "⚠️  Some Python dependencies may not be fully installed"
	@echo "📋 Checking if Python extension module can be imported..."
	@$(PYTHON_ENV) python -c "try: import graphbit; print('✅ GraphBit Python extension module is working'); except ImportError as e: print(f'⚠️  GraphBit module import failed: {e}')" || echo "⚠️  Could not test GraphBit module import"
	@echo "✅ Installation validation completed!"

install-quick: ## Quick install with build assuming Poetry and environment are already set up
	@echo "⚡ Quick installation (skipping environment checks)..."
	@$(PYTHON_ENV) poetry install --with dev,benchmarks || { echo "❌ Poetry install failed"; exit 1; }
	@cargo fetch || { echo "❌ Cargo fetch failed"; exit 1; }
	@cargo build --workspace || { echo "❌ Cargo build failed"; exit 1; }
	@$(PYTHON_ENV) maturin develop || { echo "❌ Maturin develop failed"; exit 1; }
	@echo "✅ Quick installation completed!"

install-force: ## Force install with build without any checks (use with caution)
	@echo "⚠️  Force installation (no safety checks)..."
	@poetry install --with dev,benchmarks || { echo "❌ Poetry install failed"; exit 1; }
	@cargo fetch || { echo "❌ Cargo fetch failed"; exit 1; }
	@cargo build --workspace || { echo "❌ Cargo build failed"; exit 1; }
	@poetry run maturin develop || { echo "❌ Maturin develop failed"; exit 1; }
	@echo "✅ Force installation completed!"

clean: ## Clean build artifacts
	cargo clean
	$(PYTHON_ENV) find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	$(PYTHON_ENV) find . -type f -name "*.pyc" -delete 2>/dev/null || true
	$(PYTHON_ENV) find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true

test: test-rust test-python ## Run all tests

test-rust:
	cargo test --workspace --all-features

test-python:
	@if [ -z "$$OPENAI_API_KEY" ]; then echo " OPENAI_API_KEY is required"; exit 1; fi
	$(PYTHON_ENV) pytest -v

test-coverage:
	cargo tarpaulin --workspace --out Html --output-dir target/coverage
	$(PYTHON_ENV) pytest --cov=graphbit --cov-report=html:target/coverage/python

lint: lint-rust lint-python

lint-rust:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

lint-python:
	$(PYTHON_ENV) flake8 graphbit/ tests/ benchmarks/
	$(PYTHON_ENV) mypy graphbit/ --ignore-missing-imports

lint-fix:
	cargo clippy --workspace --all-targets --all-features --fix --allow-staged --allow-dirty
	$(PYTHON_ENV) isort graphbit/ tests/ benchmarks/

format: format-rust format-python

format-rust:
	cargo fmt --all

format-python:
	$(PYTHON_ENV) black graphbit/ tests/ benchmarks/
	$(PYTHON_ENV) isort graphbit/ tests/ benchmarks/

format-check:
	cargo fmt --all -- --check
	$(PYTHON_ENV) black --check graphbit/ tests/ benchmarks/
	$(PYTHON_ENV) isort --check-only graphbit/ tests/ benchmarks/

build:
	cargo build --workspace --release
	$(PYTHON_ENV) poetry build

build-dev: ## Build workspace and install Python extension in development mode
	@echo "🔨 Building Rust workspace in development mode..."
	@cargo build --workspace || { echo "❌ Cargo build failed"; exit 1; }
	@echo "🐍 Installing Python extension module in development mode..."
	@$(PYTHON_ENV) maturin develop || { echo "❌ Maturin develop failed"; exit 1; }
	@echo "✅ Development build completed!"

docs:
	cargo doc --workspace --no-deps --open
	$(PYTHON_ENV) cd docs && make html

docs-serve:
	$(PYTHON_ENV) cd docs && python -m http.server 8000

security:
	cargo audit
	$(PYTHON_ENV) safety check
	$(PYTHON_ENV) bandit -r graphbit/
	$(MAKE) secrets

# -------------------------------------------
# 🔐 Secrets and Typos
# -------------------------------------------
secrets:
	$(PYTHON_ENV) detect-secrets scan --baseline .secrets.baseline

secrets-audit:
	$(PYTHON_ENV) detect-secrets scan --baseline .secrets.baseline
	$(PYTHON_ENV) detect-secrets audit .secrets.baseline

secrets-baseline:
	$(PYTHON_ENV) detect-secrets scan > .secrets.baseline

secrets-update:
	@echo "Edit .secrets.baseline to update detect-secrets configuration"
	@echo "See: https://github.com/Yelp/detect-secrets"

typos:
	typos

# -------------------------------------------
#  Benchmarking
# -------------------------------------------
bench:
	cargo bench
	@if [ -z "$$OPENAI_API_KEY" ]; then echo " OPENAI_API_KEY required"; exit 1; fi
	$(PYTHON_ENV) python -m benchmarks.run_benchmarks

build-perf:
	@echo " Building GraphBit with performance optimizations..."
	@conda run -n graphbit cargo build --release --features performance
	@conda run -n graphbit maturin develop --release

install-perf: build-perf
	@echo " Installing GraphBit with performance optimizations..."
	@conda run -n graphbit pip install -e python/

test-perf: build-perf
	@echo " Running performance tests..."
	@conda run -n graphbit python performance_test.py

benchmark-perf: build-perf
	@echo " Running comprehensive benchmarks..."
	@conda run -n graphbit python benchmarks/run_comprehensive_benchmark.py

# -------------------------------------------
#  Pre-commit Hooks
# -------------------------------------------
pre-commit-install:
	$(PYTHON_ENV) pre-commit install
	$(PYTHON_ENV) pre-commit install --hook-type commit-msg
	$(PYTHON_ENV) pre-commit install --hook-type pre-push

pre-commit-run:
	$(PYTHON_ENV) pre-commit run --all-files

pre-commit-update:
	$(PYTHON_ENV) pre-commit autoupdate

pre-commit-clean:
	$(PYTHON_ENV) pre-commit clean

# -------------------------------------------
#  CI & Release
# -------------------------------------------
all-checks: format-check lint test secrets
	@echo " All checks passed!"

ci: clean all-checks
	@echo " CI pipeline completed successfully!"

release-check: all-checks docs
	cargo publish --dry-run
	$(PYTHON_ENV) poetry check

# -------------------------------------------
#  Quick Commands
# -------------------------------------------
quick: format-rust lint-rust test-rust

quick-python: format-python lint-python test-python

# -------------------------------------------
#  Examples & Dev Watch
# -------------------------------------------
examples:
	@if [ -z "$$OPENAI_API_KEY" ]; then echo " OPENAI_API_KEY required"; exit 1; fi
	$(PYTHON_ENV) python examples/basic_workflow.py

watch-test:
	cargo watch -x "test --workspace"

watch-check:
	cargo watch -x "check --workspace" -x "clippy --workspace"
