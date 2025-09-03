# GraphBit Validation Utilities
# =============================
# Path validation and environment checking

# Include platform configuration
include makefiles/platform.mk

# Validate all required directories exist
.PHONY: validate-paths
validate-paths: ## Validate that all required test directories exist
	@echo "Validating Project Paths"
	@echo "================================================"
ifeq ($(SHELL_TYPE),windows)
	@if not exist "$(TESTS_ROOT)" (echo "ERROR: Directory $(TESTS_ROOT) does not exist" && exit /b 1) else (echo "SUCCESS: Directory $(TESTS_ROOT) exists")
	@if not exist "$(PYTHON_UNIT_TESTS)" (echo "ERROR: Directory $(PYTHON_UNIT_TESTS) does not exist" && exit /b 1) else (echo "SUCCESS: Directory $(PYTHON_UNIT_TESTS) exists")
	@if not exist "$(PYTHON_INTEGRATION_TESTS)" (echo "ERROR: Directory $(PYTHON_INTEGRATION_TESTS) does not exist" && exit /b 1) else (echo "SUCCESS: Directory $(PYTHON_INTEGRATION_TESTS) exists")
	@if not exist "$(RUST_UNIT_TESTS)" (echo "ERROR: Directory $(RUST_UNIT_TESTS) does not exist" && exit /b 1) else (echo "SUCCESS: Directory $(RUST_UNIT_TESTS) exists")
	@if not exist "$(RUST_INTEGRATION_TESTS)" (echo "ERROR: Directory $(RUST_INTEGRATION_TESTS) does not exist" && exit /b 1) else (echo "SUCCESS: Directory $(RUST_INTEGRATION_TESTS) exists")
	@if not exist "$(CORE_SRC)" (echo "ERROR: Directory $(CORE_SRC) does not exist" && exit /b 1) else (echo "SUCCESS: Directory $(CORE_SRC) exists")
else
	@if [ ! -d "$(TESTS_ROOT)" ]; then echo "ERROR: Directory $(TESTS_ROOT) does not exist"; exit 1; else echo "SUCCESS: Directory $(TESTS_ROOT) exists"; fi
	@if [ ! -d "$(PYTHON_UNIT_TESTS)" ]; then echo "ERROR: Directory $(PYTHON_UNIT_TESTS) does not exist"; exit 1; else echo "SUCCESS: Directory $(PYTHON_UNIT_TESTS) exists"; fi
	@if [ ! -d "$(PYTHON_INTEGRATION_TESTS)" ]; then echo "ERROR: Directory $(PYTHON_INTEGRATION_TESTS) does not exist"; exit 1; else echo "SUCCESS: Directory $(PYTHON_INTEGRATION_TESTS) exists"; fi
	@if [ ! -d "$(RUST_UNIT_TESTS)" ]; then echo "ERROR: Directory $(RUST_UNIT_TESTS) does not exist"; exit 1; else echo "SUCCESS: Directory $(RUST_UNIT_TESTS) exists"; fi
	@if [ ! -d "$(RUST_INTEGRATION_TESTS)" ]; then echo "ERROR: Directory $(RUST_INTEGRATION_TESTS) does not exist"; exit 1; else echo "SUCCESS: Directory $(RUST_INTEGRATION_TESTS) exists"; fi
	@if [ ! -d "$(CORE_SRC)" ]; then echo "ERROR: Directory $(CORE_SRC) does not exist"; exit 1; else echo "SUCCESS: Directory $(CORE_SRC) exists"; fi
endif
	@echo "All required directories validated"

# Validate required tools are available
.PHONY: validate-tools
validate-tools: ## Validate that all required development tools are available
	@echo "Validating Development Tools"
	@echo "================================================"
	@echo "Checking cargo..."
ifeq ($(SHELL_TYPE),windows)
	@where cargo >nul 2>&1 && echo "SUCCESS: cargo is available" || (echo "ERROR: cargo not found" && exit /b 1)
else
	@command -v cargo >/dev/null 2>&1 && echo "SUCCESS: cargo is available" || (echo "ERROR: cargo not found" && exit 1)
endif
	@echo "Checking python..."
ifeq ($(SHELL_TYPE),windows)
	@where python >nul 2>&1 && echo "SUCCESS: python is available" || (echo "ERROR: python not found" && exit /b 1)
else
	@command -v python >/dev/null 2>&1 && echo "SUCCESS: python is available" || (echo "ERROR: python not found" && exit 1)
endif
	@echo "Checking Poetry..."
ifeq ($(SHELL_TYPE),windows)
	@where poetry >nul 2>&1 && echo "SUCCESS: Poetry is available" || (echo "ERROR: Poetry not found" && exit /b 1)
else
	@command -v poetry >/dev/null 2>&1 && echo "SUCCESS: Poetry is available" || (echo "ERROR: Poetry not found" && exit 1)
endif
	@echo "All required tools validated"

# Validate optional tools
.PHONY: validate-optional-tools
validate-optional-tools: ## Check availability of optional development tools
	$(call print_header,"$(EMOJI_INFO) Checking Optional Tools")
	@echo "$(EMOJI_INFO) Checking cargo-llvm-cov..."
	@cargo llvm-cov --version >/dev/null 2>&1 && \
		echo "$(EMOJI_SUCCESS) cargo-llvm-cov is available" || \
		echo "$(EMOJI_WARNING) cargo-llvm-cov not found (install with: cargo install cargo-llvm-cov)"
	@echo "$(EMOJI_INFO) Checking pytest..."
	@$(PYTHON_ENV) pytest --version >/dev/null 2>&1 && \
		echo "$(EMOJI_SUCCESS) pytest is available" || \
		echo "$(EMOJI_WARNING) pytest not found"
	@echo "$(EMOJI_INFO) Checking typos..."
	@typos --version >/dev/null 2>&1 && \
		echo "$(EMOJI_SUCCESS) typos is available" || \
		echo "$(EMOJI_WARNING) typos not found (install with: cargo install typos-cli)"

# Validate Python environment
.PHONY: validate-python-env
validate-python-env: ## Validate Python environment and dependencies
	$(call print_header,"$(EMOJI_PYTHON) Validating Python Environment")
	@echo "$(EMOJI_INFO) Environment type: $(ENV_TYPE)"
	@echo "$(EMOJI_INFO) Python command: $(PYTHON_ENV)"
ifeq ($(ENV_TYPE),conda)
	@echo "$(EMOJI_INFO) Checking conda environment..."
	@conda info --envs | grep -q "^$(PROJECT_NAME) " || { \
		echo "$(EMOJI_ERROR) Conda environment '$(PROJECT_NAME)' not found!"; \
		echo "$(EMOJI_INFO) Create it with: make create-conda-env"; \
		exit 1; \
	}
	@echo "$(EMOJI_SUCCESS) Conda environment '$(PROJECT_NAME)' found"
else ifeq ($(ENV_TYPE),venv)
	@echo "$(EMOJI_INFO) Checking virtual environment..."
ifeq ($(SHELL_TYPE),windows)
	@if not exist "$(PYTHON_VENV_ACTIVATE)" ( \
		echo "$(EMOJI_ERROR) Virtual environment not found at .venv!"; \
		echo "$(EMOJI_INFO) Create it with: python -m venv .venv"; \
		exit /b 1; \
	)
else
	@if [ ! -f "$(PYTHON_VENV_ACTIVATE)" ]; then \
		echo "$(EMOJI_ERROR) Virtual environment not found at .venv!"; \
		echo "$(EMOJI_INFO) Create it with: python -m venv .venv"; \
		exit 1; \
	fi
endif
	@echo "$(EMOJI_SUCCESS) Virtual environment found at .venv"
else ifeq ($(ENV_TYPE),poetry)
	@echo "$(EMOJI_INFO) Using Poetry environment management"
	@$(PYTHON_ENV) poetry check || { \
		echo "$(EMOJI_ERROR) Poetry environment check failed"; \
		exit 1; \
	}
	@echo "$(EMOJI_SUCCESS) Poetry environment is valid"
endif
	@echo "$(EMOJI_INFO) Testing Python execution..."
	@$(PYTHON_ENV) python -c "import sys; print(f'$(EMOJI_SUCCESS) Python {sys.version} is working')"

# Validate Rust environment
.PHONY: validate-rust-env
validate-rust-env: ## Validate Rust environment and toolchain
	$(call print_header,"$(EMOJI_RUST) Validating Rust Environment")
	@echo "$(EMOJI_INFO) Checking Rust toolchain..."
	@cargo --version || { \
		echo "$(EMOJI_ERROR) Cargo not available"; \
		exit 1; \
	}
	@echo "$(EMOJI_INFO) Checking workspace configuration..."
	@cargo check --workspace >/dev/null 2>&1 || { \
		echo "$(EMOJI_ERROR) Rust workspace check failed"; \
		exit 1; \
	}
	@echo "$(EMOJI_SUCCESS) Rust environment is valid"

# Comprehensive validation
.PHONY: validate-all
validate-all: validate-paths validate-tools validate-python-env validate-rust-env ## Run all validation checks
	$(call print_success,"$(EMOJI_SUCCESS) All validation checks passed!")

# Quick validation for development
.PHONY: validate-quick
validate-quick: validate-paths validate-tools ## Quick validation of paths and tools only
	$(call print_success,"$(EMOJI_SUCCESS) Quick validation completed!")

# Environment-specific validation
.PHONY: validate-test-env
validate-test-env: ## Validate test environment setup
	$(call print_header,"$(EMOJI_TEST) Validating Test Environment")
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "$(EMOJI_WARNING) OPENAI_API_KEY is not set"; \
		echo "$(EMOJI_INFO) Some tests may be skipped"; \
	else \
		echo "$(EMOJI_SUCCESS) OPENAI_API_KEY is configured"; \
	fi
	$(call validate_directory,$(PYTHON_UNIT_TESTS))
	$(call validate_directory,$(PYTHON_INTEGRATION_TESTS))
	$(call validate_directory,$(RUST_UNIT_TESTS))
	$(call validate_directory,$(RUST_INTEGRATION_TESTS))
	$(call print_success,"$(EMOJI_SUCCESS) Test environment validated")
