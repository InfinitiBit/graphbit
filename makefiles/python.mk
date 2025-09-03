# GraphBit Python-Specific Makefile
# ==================================
# All Python-related test, lint, format, and development targets

# Include platform configuration
include makefiles/platform.mk

# Python-specific configuration
PYTHON_VERSION := 3.11
PYTHON_COVERAGE_DIR := $(PYTHON_COVERAGE_OUTPUT)
PYTHON_PACKAGE_DIR := $(GRAPHBIT_SRC)

# Python test configuration
PYTHON_TEST_PATHS := $(PYTHON_UNIT_TESTS) $(PYTHON_INTEGRATION_TESTS)
PYTHON_COVERAGE_PATHS := --cov=$(PYTHON_UNIT_TESTS) --cov=$(PYTHON_INTEGRATION_TESTS)
PYTHON_LINT_PATHS := $(PYTHON_PACKAGE_DIR) $(TESTS_ROOT) $(BENCHMARKS_SRC)

# Cross-platform pytest command construction
ifeq ($(SHELL_TYPE),windows)
	PYTEST_CMD := $(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS)
	PYTEST_COV_CMD := $(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS) $(PYTHON_COVERAGE_PATHS) $(PYTEST_COVERAGE_FLAGS)
else
	PYTEST_CMD := $(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS)
	PYTEST_COV_CMD := $(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS) $(PYTHON_COVERAGE_PATHS) $(PYTEST_COVERAGE_FLAGS) $(PYTEST_COV_CONFIG)
endif

# Python build and test targets
.PHONY: python-install python-install-dev python-build python-clean
.PHONY: python-test python-test-unit python-test-integration python-test-coverage
.PHONY: python-lint python-format python-format-check python-type-check
.PHONY: python-security python-deps-check

# Installation and setup
python-install: ## Install Python dependencies
	$(call print_header,"$(EMOJI_PYTHON) Installing Python Dependencies")
	@$(PYTHON_ENV) poetry install
	$(call print_success,"$(MSG_SUCCESS)")

python-install-dev: ## Install Python dependencies including development tools
	$(call print_header,"$(EMOJI_PYTHON) Installing Python Development Dependencies")
	@$(PYTHON_ENV) poetry install --with dev,benchmarks
	$(call print_success,"$(MSG_SUCCESS)")

python-build: ## Build Python package
	$(call print_header,"$(EMOJI_BUILD) Building Python Package")
	@$(PYTHON_ENV) poetry build
	$(call print_success,"$(MSG_SUCCESS)")

python-clean: ## Clean Python build artifacts and cache
	$(call print_header,"$(EMOJI_CLEAN) Cleaning Python Artifacts")
	$(call clean_python_cache)
	$(call remove_dir,$(PYTHON_COVERAGE_DIR))
	$(call remove_dir,$(BUILD_DIR))
	$(call remove_dir,$(DIST_DIR))
	$(call remove_dir,.pytest_cache)
	$(call remove_dir,.mypy_cache)
	$(call print_success,"$(MSG_SUCCESS)")

# Test targets
python-test: python-test-coverage ## Run all Python tests with coverage (default)

python-test-unit: ## Run only Python unit tests with coverage
	@echo "Running Python Unit Tests"
	@echo "================================================"
	@echo "Checking OPENAI_API_KEY..."
ifeq ($(SHELL_TYPE),windows)
	@if not defined OPENAI_API_KEY (echo "WARNING: OPENAI_API_KEY is required for Python tests" && exit /b 1)
else
	@if [ -z "$$OPENAI_API_KEY" ]; then echo "WARNING: OPENAI_API_KEY is required for Python tests"; exit 1; fi
endif
	@echo "Running unit tests with coverage..."
ifeq ($(SHELL_TYPE),windows)
	@$(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS) --cov=$(PYTHON_UNIT_TESTS) $(PYTEST_COVERAGE_FLAGS) $(PYTHON_UNIT_TESTS) -v --tb=short || (echo "ERROR: Python unit tests failed" && exit /b 1)
else
	@$(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS) --cov=$(PYTHON_UNIT_TESTS) $(PYTEST_COVERAGE_FLAGS) $(PYTEST_COV_CONFIG) $(PYTHON_UNIT_TESTS) -v --tb=short || (echo "ERROR: Python unit tests failed" && exit 1)
endif
	@echo "SUCCESS: Python unit tests completed successfully!"

python-test-integration: ## Run only Python integration tests with coverage
	@echo "Running Python Integration Tests"
	@echo "================================================"
	@echo "Checking OPENAI_API_KEY..."
ifeq ($(SHELL_TYPE),windows)
	@if not defined OPENAI_API_KEY (echo "WARNING: OPENAI_API_KEY is required for Python tests" && exit /b 1)
else
	@if [ -z "$$OPENAI_API_KEY" ]; then echo "WARNING: OPENAI_API_KEY is required for Python tests"; exit 1; fi
endif
	@echo "Running integration tests with coverage..."
ifeq ($(SHELL_TYPE),windows)
	@$(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS) --cov=$(PYTHON_INTEGRATION_TESTS) $(PYTEST_COVERAGE_FLAGS) $(PYTHON_INTEGRATION_TESTS) -v --tb=short || (echo "ERROR: Python integration tests failed" && exit /b 1)
else
	@$(ENV_VAR_SET) $(PYTHON_ENV) pytest $(PYTEST_FLAGS) --cov=$(PYTHON_INTEGRATION_TESTS) $(PYTEST_COVERAGE_FLAGS) $(PYTEST_COV_CONFIG) $(PYTHON_INTEGRATION_TESTS) -v --tb=short || (echo "ERROR: Python integration tests failed" && exit 1)
endif
	@echo "SUCCESS: Python integration tests completed successfully!"

python-test-coverage: ## Run all Python tests with coverage reporting
	@echo "Running Python Tests with Coverage"
	@echo "================================================"
	@echo "Checking OPENAI_API_KEY..."
ifeq ($(SHELL_TYPE),windows)
	@if not defined OPENAI_API_KEY (echo "WARNING: OPENAI_API_KEY is required for Python tests" && exit /b 1)
else
	@if [ -z "$$OPENAI_API_KEY" ]; then echo "WARNING: OPENAI_API_KEY is required for Python tests"; exit 1; fi
endif
	@echo "Running comprehensive Python tests with coverage..."
	@$(PYTEST_COV_CMD) $(PYTHON_TEST_PATHS) || (echo "ERROR: Python tests with coverage failed" && exit 1)
	@echo "Coverage report generated at: $(PYTHON_COVERAGE_DIR)/index.html"
	@echo "SUCCESS: Python tests with coverage completed successfully!"

python-test-quick: ## Run Python tests without coverage (faster for development)
	@echo "Running Quick Python Tests"
	@echo "================================================"
	@echo "Checking OPENAI_API_KEY..."
ifeq ($(SHELL_TYPE),windows)
	@if not defined OPENAI_API_KEY (echo "WARNING: OPENAI_API_KEY is required for Python tests" && exit /b 1)
else
	@if [ -z "$$OPENAI_API_KEY" ]; then echo "WARNING: OPENAI_API_KEY is required for Python tests"; exit 1; fi
endif
	@echo "Running tests without coverage for faster feedback..."
	@$(PYTEST_CMD) $(PYTHON_TEST_PATHS) -v --tb=line || (echo "ERROR: Python tests failed" && exit 1)
	@echo "SUCCESS: Quick Python tests completed!"

python-test-specific: ## Run specific Python test (usage: make python-test-specific TEST=test_name)
ifndef TEST
	@echo "ERROR: Please specify TEST variable: make python-test-specific TEST=test_name"
ifeq ($(SHELL_TYPE),windows)
	@exit /b 1
else
	@exit 1
endif
endif
	@echo "Running Specific Test: $(TEST)"
	@echo "================================================"
	@$(PYTEST_CMD) -k "$(TEST)" $(PYTHON_TEST_PATHS) -v --tb=short
	@echo "SUCCESS: Test $(TEST) completed!"

# Linting and formatting
python-lint: ## Run Python linting (flake8 + mypy)
	@echo "Running Python Linting"
	@echo "================================================"
	@echo "Running flake8..."
	@$(PYTHON_ENV) flake8 $(PYTHON_LINT_PATHS)
	@echo "Running mypy..."
	@$(PYTHON_ENV) mypy $(PYTHON_PACKAGE_DIR) --ignore-missing-imports
	@echo "SUCCESS: Python linting completed!"

python-format: ## Format Python code with black and isort
	@echo "Formatting Python Code"
	@echo "================================================"
	@echo "Running black..."
	@$(PYTHON_ENV) black $(PYTHON_LINT_PATHS)
	@echo "Running isort..."
	@$(PYTHON_ENV) isort $(PYTHON_LINT_PATHS)
	@echo "SUCCESS: Python formatting completed!"

python-format-check: ## Check Python code formatting
	@echo "Checking Python Code Formatting"
	@echo "================================================"
	@echo "Checking with black..."
	@$(PYTHON_ENV) black --check $(PYTHON_LINT_PATHS)
	@echo "Checking with isort..."
	@$(PYTHON_ENV) isort --check-only $(PYTHON_LINT_PATHS)
	@echo "SUCCESS: Python formatting check completed!"

python-type-check: ## Run Python type checking with mypy
	$(call print_header,"$(EMOJI_PYTHON) Running Python Type Checking")
	@$(PYTHON_ENV) mypy $(PYTHON_PACKAGE_DIR) --ignore-missing-imports
	$(call print_success,"$(MSG_SUCCESS)")

# Security and dependency checking
python-security: ## Run Python security checks
	$(call print_header,"$(EMOJI_PYTHON) Running Python Security Checks")
	@echo "$(EMOJI_INFO) Running safety check..."
	@$(PYTHON_ENV) safety check
	@echo "$(EMOJI_INFO) Running bandit..."
	@$(PYTHON_ENV) bandit -r $(PYTHON_PACKAGE_DIR)/
	$(call print_success,"$(MSG_SUCCESS)")

python-deps-check: ## Check Python dependencies for updates
	$(call print_header,"$(EMOJI_PYTHON) Checking Python Dependencies")
	@$(PYTHON_ENV) poetry show --outdated
	$(call print_success,"$(MSG_SUCCESS)")

# Development utilities
python-shell: ## Start Python shell with project environment
	$(call print_header,"$(EMOJI_PYTHON) Starting Python Shell")
	@$(PYTHON_ENV) python

python-notebook: ## Start Jupyter notebook server
	$(call print_header,"$(EMOJI_PYTHON) Starting Jupyter Notebook")
	@$(PYTHON_ENV) jupyter notebook

# Comprehensive targets
python-all-checks: python-format-check python-lint python-type-check python-security python-test-coverage ## Run all Python quality checks
	$(call print_success,"$(EMOJI_SUCCESS) All Python checks passed!")

python-quick-checks: python-format-check python-lint python-test-quick ## Quick Python quality checks for development
	$(call print_success,"$(EMOJI_SUCCESS) Quick Python checks completed!")
