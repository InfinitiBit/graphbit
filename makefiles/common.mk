# GraphBit Common Utilities
# =========================
# Shared targets and utilities used across all Makefiles

# Include platform configuration
include makefiles/platform.mk

# Common development targets
.PHONY: install install-dev install-quick install-force
.PHONY: clean clean-all build build-dev build-release
.PHONY: docs docs-serve security pre-commit
.PHONY: examples benchmark watch

# Installation targets
install: validate-tools ## Comprehensive install: dependencies, build, and setup
	$(call print_header,"$(EMOJI_ROCKET) Starting Comprehensive Installation")
	@$(MAKE) check-poetry
	@$(MAKE) check-environment
	@$(MAKE) install-dependencies
	@$(MAKE) validate-installation
	$(call print_success,"$(EMOJI_SUCCESS) Comprehensive installation completed successfully!")
	$(call print_success,"$(EMOJI_ROCKET) Your GraphBit development environment is ready!")

install-dev: ## Install development dependencies and setup environment
	$(call print_header,"$(EMOJI_ROCKET) Setting Up Development Environment")
	@$(PYTHON_ENV) poetry install --with dev,benchmarks
	@cargo build --workspace
	@$(PYTHON_ENV) pre-commit install
	@$(PYTHON_ENV) pre-commit install --hook-type commit-msg
	@$(PYTHON_ENV) pre-commit install --hook-type pre-push
	$(call print_success,"$(EMOJI_SUCCESS) Development environment ready!")

install-quick: ## Quick install assuming environment is set up
	$(call print_header,"$(EMOJI_ROCKET) Quick Installation")
	@$(PYTHON_ENV) poetry install --with dev,benchmarks || { \
		echo "$(MSG_ERROR) Poetry install failed"; \
		exit 1; \
	}
	@cargo fetch || { \
		echo "$(MSG_ERROR) Cargo fetch failed"; \
		exit 1; \
	}
	@cargo build --workspace || { \
		echo "$(MSG_ERROR) Cargo build failed"; \
		exit 1; \
	}
	@$(PYTHON_ENV) maturin develop || { \
		echo "$(MSG_ERROR) Maturin develop failed"; \
		exit 1; \
	}
	$(call print_success,"$(EMOJI_SUCCESS) Quick installation completed!")

install-force: ## Force install without checks (use with caution)
	$(call print_header,"$(EMOJI_WARNING) Force Installation (No Safety Checks)")
	@poetry install --with dev,benchmarks || { \
		echo "$(MSG_ERROR) Poetry install failed"; \
		exit 1; \
	}
	@cargo fetch || { \
		echo "$(MSG_ERROR) Cargo fetch failed"; \
		exit 1; \
	}
	@cargo build --workspace || { \
		echo "$(MSG_ERROR) Cargo build failed"; \
		exit 1; \
	}
	@poetry run maturin develop || { \
		echo "$(MSG_ERROR) Maturin develop failed"; \
		exit 1; \
	}
	$(call print_success,"$(EMOJI_SUCCESS) Force installation completed!")

# Environment checking (from original Makefile)
check-poetry: ## Check if Poetry is installed and install if needed
	$(call print_header,"$(EMOJI_INFO) Checking Poetry Installation")
ifeq ($(SHELL_TYPE),windows)
	@powershell -Command "try { \
		if (!(Get-Command poetry -ErrorAction SilentlyContinue)) { \
			Write-Host '$(EMOJI_PYTHON) Poetry not found. Installing Poetry...' -ForegroundColor Yellow; \
			(Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | python -; \
			Write-Host '$(EMOJI_SUCCESS) Poetry installed successfully!' -ForegroundColor Green; \
			Write-Host '$(EMOJI_WARNING) Please restart your terminal or run the following command:' -ForegroundColor Yellow; \
			Write-Host '   refreshenv' -ForegroundColor Cyan; \
			Write-Host '   Then run: make install' -ForegroundColor Cyan; \
			exit 1; \
		} else { \
			Write-Host '$(EMOJI_SUCCESS) Poetry is already installed' -ForegroundColor Green; \
		} \
	} catch { \
		Write-Host '$(EMOJI_ERROR) Failed to install Poetry automatically' -ForegroundColor Red; \
		Write-Host '$(EMOJI_INFO) Please install Poetry manually:' -ForegroundColor Yellow; \
		Write-Host '   Visit: https://python-poetry.org/docs/#installation' -ForegroundColor Cyan; \
		exit 1; \
	}"
else
	@if ! command -v poetry >/dev/null 2>&1; then \
		echo "$(EMOJI_PYTHON) Poetry not found. Installing Poetry..."; \
		if curl -sSL https://install.python-poetry.org | python3 -; then \
			echo "$(EMOJI_SUCCESS) Poetry installed successfully!"; \
			echo "$(EMOJI_WARNING) Please restart your terminal or run:"; \
			echo "   source ~/.bashrc"; \
			echo "   Then run: make install"; \
			exit 1; \
		else \
			echo "$(EMOJI_ERROR) Failed to install Poetry automatically"; \
			echo "$(EMOJI_INFO) Please install Poetry manually:"; \
			echo "   Visit: https://python-poetry.org/docs/#installation"; \
			exit 1; \
		fi; \
	else \
		echo "$(EMOJI_SUCCESS) Poetry is already installed"; \
	fi
endif

check-environment: ## Check and validate environment setup
	$(call print_header,"$(EMOJI_INFO) Checking Environment Setup")
	@echo "$(EMOJI_INFO) Environment type: $(ENV_TYPE)"
ifeq ($(ENV_TYPE),conda)
	@echo "$(EMOJI_PYTHON) Using Conda environment: $(PROJECT_NAME)"
	@if ! conda info --envs | grep -q "^$(PROJECT_NAME) "; then \
		echo "$(EMOJI_ERROR) Conda environment '$(PROJECT_NAME)' not found!"; \
		echo "$(EMOJI_INFO) Creating conda environment..."; \
		$(MAKE) create-conda-env; \
	else \
		echo "$(EMOJI_SUCCESS) Conda environment '$(PROJECT_NAME)' found"; \
	fi
else ifeq ($(ENV_TYPE),venv)
	@echo "$(EMOJI_PYTHON) Using virtual environment: .venv"
	$(call validate_directory,.venv)
else ifeq ($(ENV_TYPE),poetry)
	@echo "$(EMOJI_PYTHON) Using Poetry environment management"
	@echo "$(EMOJI_SUCCESS) Poetry will manage the virtual environment automatically"
else
	@echo "$(EMOJI_WARNING) Unknown ENV_TYPE: $(ENV_TYPE), defaulting to conda"
	@$(MAKE) ENV_TYPE=conda check-environment
endif

install-dependencies: ## Install all dependencies and build workspace
	$(call print_header,"$(EMOJI_PYTHON) Installing Dependencies and Building Workspace")
	@echo "$(EMOJI_PYTHON) Installing Python dependencies with Poetry..."
	@$(PYTHON_ENV) poetry install --with dev,benchmarks || { \
		echo "$(MSG_ERROR) Poetry install failed"; \
		exit 1; \
	}
	@echo "$(EMOJI_RUST) Fetching Rust dependencies..."
	@cargo fetch || { \
		echo "$(MSG_ERROR) Cargo fetch failed"; \
		exit 1; \
	}
	@echo "$(EMOJI_BUILD) Building Rust workspace..."
	@cargo build --workspace || { \
		echo "$(MSG_ERROR) Cargo build failed"; \
		exit 1; \
	}
	@echo "$(EMOJI_PYTHON) Installing Python extension module in development mode..."
	@$(PYTHON_ENV) maturin develop || { \
		echo "$(MSG_ERROR) Maturin develop failed"; \
		exit 1; \
	}
	$(call print_success,"$(EMOJI_SUCCESS) All dependencies installed and workspace built successfully!")

validate-installation: ## Validate that installation was successful
	$(call print_header,"$(EMOJI_INFO) Validating Installation")
	@echo "$(EMOJI_INFO) Checking Poetry environment..."
	@$(PYTHON_ENV) poetry check || echo "$(EMOJI_WARNING) Poetry check failed"
	@echo "$(EMOJI_INFO) Checking Python packages..."
	@$(PYTHON_ENV) python -c "import sys; print(f'$(EMOJI_SUCCESS) Python {sys.version} is working')"
	@echo "$(EMOJI_INFO) Checking Rust toolchain..."
	@cargo --version || echo "$(EMOJI_WARNING) Cargo not available"
	@echo "$(EMOJI_INFO) Checking Maturin installation..."
	@$(PYTHON_ENV) python -c "import maturin; print('$(EMOJI_SUCCESS) Maturin is available')" || echo "$(EMOJI_WARNING) Maturin not available"
	@echo "$(EMOJI_INFO) Checking key Python dependencies..."
	@$(PYTHON_ENV) python -c "import click, rich, typer; print('$(EMOJI_SUCCESS) Core dependencies available')" || echo "$(EMOJI_WARNING) Some Python dependencies may not be fully installed"
	@echo "$(EMOJI_INFO) Checking if Python extension module can be imported..."
	@$(PYTHON_ENV) python -c "try: import graphbit; print('$(EMOJI_SUCCESS) GraphBit Python extension module is working'); except ImportError as e: print(f'$(EMOJI_WARNING) GraphBit module import failed: {e}')" || echo "$(EMOJI_WARNING) Could not test GraphBit module import"
	$(call print_success,"$(EMOJI_SUCCESS) Installation validation completed!")

# Environment creation
create-conda-env: ## Create conda environment if it doesn't exist
	$(call print_header,"$(EMOJI_PYTHON) Creating Conda Environment")
	@echo "$(EMOJI_INFO) Checking if conda environment '$(PROJECT_NAME)' exists..."
	@conda info --envs | grep -q "^$(PROJECT_NAME) " || { \
		echo "$(EMOJI_INFO) Creating conda environment '$(PROJECT_NAME)' with Python $(PYTHON_VERSION)..."; \
		conda create -n $(PROJECT_NAME) python=$(PYTHON_VERSION) -y; \
		echo "$(EMOJI_SUCCESS) Conda environment '$(PROJECT_NAME)' created successfully!"; \
	}

# Clean targets
clean: ## Clean build artifacts for both Rust and Python
	$(call print_header,"$(EMOJI_CLEAN) Cleaning Build Artifacts")
	@$(MAKE) rust-clean
	@$(MAKE) python-clean
	$(call print_success,"$(MSG_SUCCESS)")

clean-all: clean ## Comprehensive clean including all generated files
	$(call print_header,"$(EMOJI_CLEAN) Comprehensive Clean")
	$(call remove_dir,$(TARGET_DIR))
	$(call remove_dir,node_modules)
	$(call remove_dir,.coverage)
	$(call remove_dir,.pytest_cache)
	$(call remove_dir,.mypy_cache)
	$(call print_success,"$(MSG_SUCCESS)")

# Build targets
build: ## Build both Rust and Python components
	$(call print_header,"$(EMOJI_BUILD) Building All Components")
	@$(MAKE) rust-build
	@$(MAKE) python-build
	$(call print_success,"$(MSG_SUCCESS)")

build-dev: ## Build workspace and install Python extension in development mode
	$(call print_header,"$(EMOJI_BUILD) Building Development Environment")
	@$(MAKE) rust-build
	@$(PYTHON_ENV) maturin develop
	$(call print_success,"$(MSG_SUCCESS)")

build-release: ## Build all components in release mode
	$(call print_header,"$(EMOJI_BUILD) Building Release Version")
	@$(MAKE) rust-build-release
	@$(MAKE) python-build
	$(call print_success,"$(MSG_SUCCESS)")

# Documentation
docs: ## Generate documentation for all components
	$(call print_header,"$(EMOJI_INFO) Generating Documentation")
	@$(MAKE) rust-doc
	@$(PYTHON_ENV) cd $(DOCS_SRC) && make html
	$(call print_success,"$(MSG_SUCCESS)")

docs-serve: ## Serve documentation locally
	$(call print_header,"$(EMOJI_INFO) Serving Documentation")
	@$(PYTHON_ENV) cd $(DOCS_SRC) && python -m http.server 8000

# Security
security: ## Run security checks for all components
	$(call print_header,"$(EMOJI_INFO) Running Security Checks")
	@$(MAKE) rust-audit
	@$(MAKE) python-security
	@$(MAKE) secrets
	$(call print_success,"$(MSG_SUCCESS)")

# Secrets scanning
secrets: ## Scan for secrets in codebase
	$(call print_header,"$(EMOJI_INFO) Scanning for Secrets")
	@$(PYTHON_ENV) detect-secrets scan --baseline .secrets.baseline
	$(call print_success,"$(MSG_SUCCESS)")

# Pre-commit hooks
pre-commit: ## Run pre-commit hooks on all files
	$(call print_header,"$(EMOJI_INFO) Running Pre-commit Hooks")
	@$(PYTHON_ENV) pre-commit run --all-files
	$(call print_success,"$(MSG_SUCCESS)")

# Examples
examples: ## Run example scripts
	$(call print_header,"$(EMOJI_INFO) Running Examples")
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "$(EMOJI_WARNING) OPENAI_API_KEY required for examples"; \
		exit 1; \
	fi
	@$(PYTHON_ENV) python $(EXAMPLES_SRC)/basic_workflow.py
	$(call print_success,"$(MSG_SUCCESS)")

# Benchmarking
benchmark: ## Run comprehensive benchmarks
	$(call print_header,"$(EMOJI_INFO) Running Benchmarks")
	@$(MAKE) rust-bench
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "$(EMOJI_WARNING) OPENAI_API_KEY required for Python benchmarks"; \
		exit 1; \
	fi
	@$(PYTHON_ENV) python -m $(BENCHMARKS_SRC).run_benchmarks
	$(call print_success,"$(MSG_SUCCESS)")

# Development utilities
watch: ## Watch for changes and run quick checks
	$(call print_header,"$(EMOJI_INFO) Starting Development Watch Mode")
	@echo "$(EMOJI_INFO) Choose watch mode:"
	@echo "  make rust-watch-test    - Watch Rust tests"
	@echo "  make rust-watch-check   - Watch Rust check + clippy"
	@echo "  make rust-watch-build   - Watch Rust build"
