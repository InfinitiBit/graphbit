.PHONY: help install dev build build-release test lint format check clean bump-version

# Default target
.DEFAULT_GOAL := help

# Detect Python command (cross-platform)
PYTHON := $(shell command -v python3 2>/dev/null || command -v python 2>/dev/null || echo python)

help: ## Show this help message
	@echo "GraphBit Python Package - Makefile Commands"
	@echo "============================================"
	@echo ""
	@echo "Available commands:"
	@echo ""
	@echo "  make install         - Install production dependencies"
	@echo "  make dev             - Install in development mode (maturin develop)"
	@echo "  make build           - Build wheel package (debug mode)"
	@echo "  make build-release   - Build optimized release wheel"
	@echo "  make test            - Run Python tests"
	@echo "  make lint            - Run code quality checks (black, flake8)"
	@echo "  make format          - Auto-format code (black, isort)"
	@echo "  make check           - Run all checks (lint + test)"
	@echo "  make clean           - Remove build artifacts and cache"
	@echo "  make bump-version    - Bump version (usage: make bump-version VERSION=0.6.3)"
	@echo "  make help            - Show this help message"
	@echo ""

install: ## Install production dependencies
	@echo "Installing production dependencies..."
	$(PYTHON) -m pip install --upgrade pip
	$(PYTHON) -m pip install maturin
	$(PYTHON) -m pip install -e .

dev: ## Install in development mode with maturin
	@echo "Installing in development mode..."
	$(PYTHON) -m pip install --upgrade pip
	$(PYTHON) -m pip install maturin
	$(PYTHON) -m maturin develop --release

build: ## Build wheel package (debug mode)
	@echo "Building wheel package (debug mode)..."
	$(PYTHON) -m pip install --upgrade maturin
	$(PYTHON) -m maturin build

build-release: ## Build optimized release wheel
	@echo "Building optimized release wheel..."
	$(PYTHON) -m pip install --upgrade maturin
	$(PYTHON) -m maturin build --release --strip

test: ## Run Python tests
	@echo "Running tests..."
	$(PYTHON) -m pip install pytest pytest-asyncio
	$(PYTHON) -m pytest tests/ -v

lint: ## Run code quality checks
	@echo "Running code quality checks..."
	$(PYTHON) -m pip install black flake8 isort
	@echo "Checking code formatting with black..."
	$(PYTHON) -m black --check python-src/
	@echo "Checking import order with isort..."
	$(PYTHON) -m isort --check-only python-src/
	@echo "Running flake8..."
	$(PYTHON) -m flake8 python-src/ --max-line-length=120 --extend-ignore=E203,W503

format: ## Auto-format code with black and isort
	@echo "Formatting code..."
	$(PYTHON) -m pip install black isort
	$(PYTHON) -m black python-src/
	$(PYTHON) -m isort python-src/

check: lint test ## Run all checks (lint + test)

clean: ## Remove build artifacts and cache
	@echo "Cleaning build artifacts..."
	@$(PYTHON) -c "import shutil, pathlib; \
		dirs = ['target', 'dist', '__pycache__', '.pytest_cache']; \
		[shutil.rmtree(d, ignore_errors=True) for d in dirs]; \
		[shutil.rmtree(p, ignore_errors=True) for p in pathlib.Path('.').glob('*.egg-info')]; \
		[shutil.rmtree(p, ignore_errors=True) for p in pathlib.Path('python-src').rglob('__pycache__')]; \
		pathlib.Path('build_error.log').unlink(missing_ok=True) if pathlib.Path('build_error.log').exists() else None"
	@echo "Clean complete!"

bump-version: ## Bump version (usage: make bump-version VERSION=0.6.3)
ifndef VERSION
	@echo "Error: VERSION is required. Usage: make bump-version VERSION=0.6.3"
	@exit 1
endif
	@echo "Bumping version to $(VERSION)..."
	$(PYTHON) ../scripts/bump-version.py --set $(VERSION)
	@echo "Version bumped to $(VERSION)"
	@echo "Running verification..."
	$(PYTHON) ../scripts/verify-version-sync.py
