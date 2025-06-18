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
        create-env create-conda-env check-env init

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

install: ## Install all dependencies
	$(PYTHON_ENV) poetry install --with dev,benchmarks
	cargo fetch

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

build-dev:
	cargo build --workspace
	$(PYTHON_ENV) pip install -e .

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
