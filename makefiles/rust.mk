# GraphBit Rust-Specific Makefile
# ================================
# All Rust-related build, test, and development targets

# Include platform configuration
include makefiles/platform.mk

# Rust-specific configuration
RUST_TOOLCHAIN := stable
RUST_TARGET_DIR := $(TARGET_DIR)
RUST_COVERAGE_DIR := $(RUST_TARGET_DIR)/llvm-cov
RUST_DOC_DIR := $(RUST_TARGET_DIR)/doc

# Rust test configuration
RUST_TEST_THREADS := $(shell nproc 2>/dev/null || echo 4)
RUST_TEST_FLAGS := $(CARGO_TEST_FLAGS) --color=always
RUST_COVERAGE_FLAGS := --html --workspace --ignore-filename-regex $(RUST_COVERAGE_EXCLUDE)

# Rust build targets
.PHONY: rust-check rust-build rust-build-release rust-clean
.PHONY: rust-test rust-test-unit rust-test-integration rust-test-coverage
.PHONY: rust-lint rust-format rust-format-check rust-clippy
.PHONY: rust-doc rust-bench rust-audit

# Build targets
rust-check: ## Check Rust code without building
	$(call print_header,"$(EMOJI_RUST) Checking Rust Code")
	@cargo check $(CARGO_FEATURES) --color=always
	$(call print_success,"$(MSG_SUCCESS)")

rust-build: ## Build Rust workspace in debug mode
	$(call print_header,"$(EMOJI_BUILD) Building Rust Workspace (Debug)")
	@cargo build $(CARGO_FEATURES) --color=always
	$(call print_success,"$(MSG_SUCCESS)")

rust-build-release: ## Build Rust workspace in release mode
	$(call print_header,"$(EMOJI_BUILD) Building Rust Workspace (Release)")
	@cargo build $(CARGO_FEATURES) --release --color=always
	$(call print_success,"$(MSG_SUCCESS)")

rust-clean: ## Clean Rust build artifacts
	$(call print_header,"$(EMOJI_CLEAN) Cleaning Rust Build Artifacts")
	@cargo clean
	$(call remove_dir,$(RUST_COVERAGE_DIR))
	$(call print_success,"$(MSG_SUCCESS)")

# Test targets
rust-test: rust-test-coverage ## Run all Rust tests with coverage (default)

rust-test-unit: ## Run only Rust unit tests with coverage
	$(call print_header,"$(EMOJI_RUST) Running Rust Unit Tests")
	@echo "$(EMOJI_INFO) Running unit tests with coverage..."
	@cargo llvm-cov test --lib $(RUST_COVERAGE_FLAGS) --color=always || { \
		echo "$(MSG_ERROR) Rust unit tests failed"; \
		exit 1; \
	}
	$(call print_success,"$(EMOJI_SUCCESS) Rust unit tests completed successfully!")

rust-test-integration: ## Run only Rust integration tests with coverage
	$(call print_header,"$(EMOJI_RUST) Running Rust Integration Tests")
	@echo "$(EMOJI_INFO) Running integration tests with coverage..."
	@cargo llvm-cov test --test '*' $(RUST_COVERAGE_FLAGS) --color=always || { \
		echo "$(MSG_ERROR) Rust integration tests failed"; \
		exit 1; \
	}
	$(call print_success,"$(EMOJI_SUCCESS) Rust integration tests completed successfully!")

rust-test-coverage: ## Run all Rust tests with coverage reporting
	@echo "Running Rust Tests with Coverage"
	@echo "================================================"
	@echo "Running comprehensive Rust tests with llvm-cov..."
	@cargo llvm-cov test --html --workspace --ignore-filename-regex "core/src/llm/|core/src/embeddings.rs|python/src/" --color=always || (echo "ERROR: Rust tests with coverage failed" && exit 1)
	@echo "Coverage report generated at: $(RUST_COVERAGE_DIR)/index.html"
	@echo "SUCCESS: Rust tests with coverage completed successfully!"

rust-test-quick: ## Run Rust tests without coverage (faster for development)
	$(call print_header,"$(EMOJI_RUST) Running Quick Rust Tests")
	@echo "$(EMOJI_INFO) Running tests without coverage for faster feedback..."
	@cargo test $(RUST_TEST_FLAGS) || { \
		echo "$(MSG_ERROR) Rust tests failed"; \
		exit 1; \
	}
	$(call print_success,"$(EMOJI_SUCCESS) Quick Rust tests completed!")

rust-test-specific: ## Run specific Rust test (usage: make rust-test-specific TEST=test_name)
ifndef TEST
	@echo "$(EMOJI_ERROR) Please specify TEST variable: make rust-test-specific TEST=test_name"
	@exit 1
endif
	$(call print_header,"$(EMOJI_RUST) Running Specific Test: $(TEST)")
	@cargo test $(TEST) $(RUST_TEST_FLAGS) -- --nocapture
	$(call print_success,"$(EMOJI_SUCCESS) Test $(TEST) completed!")

# Linting and formatting
rust-lint: rust-clippy ## Run Rust linting (alias for clippy)

rust-clippy: ## Run Clippy linter on Rust code
	$(call print_header,"$(EMOJI_RUST) Running Clippy Linter")
	@cargo clippy $(CARGO_FEATURES) --color=always -- -D warnings
	$(call print_success,"$(MSG_SUCCESS)")

rust-clippy-fix: ## Run Clippy with automatic fixes
	$(call print_header,"$(EMOJI_RUST) Running Clippy with Auto-fixes")
	@cargo clippy $(CARGO_FEATURES) --fix --allow-staged --allow-dirty --color=always
	$(call print_success,"$(MSG_SUCCESS)")

rust-format: ## Format Rust code
	$(call print_header,"$(EMOJI_RUST) Formatting Rust Code")
	@cargo fmt --all
	$(call print_success,"$(MSG_SUCCESS)")

rust-format-check: ## Check Rust code formatting
	$(call print_header,"$(EMOJI_RUST) Checking Rust Code Formatting")
	@cargo fmt --all -- --check
	$(call print_success,"$(MSG_SUCCESS)")

# Documentation
rust-doc: ## Generate Rust documentation
	$(call print_header,"$(EMOJI_RUST) Generating Rust Documentation")
	@cargo doc --workspace --no-deps --color=always
	$(call print_success,"$(MSG_SUCCESS)")

rust-doc-open: ## Generate and open Rust documentation
	$(call print_header,"$(EMOJI_RUST) Generating and Opening Rust Documentation")
	@cargo doc --workspace --no-deps --open --color=always
	$(call print_success,"$(MSG_SUCCESS)")

# Benchmarking
rust-bench: ## Run Rust benchmarks
	$(call print_header,"$(EMOJI_RUST) Running Rust Benchmarks")
	@cargo bench --color=always
	$(call print_success,"$(MSG_SUCCESS)")

# Security
rust-audit: ## Run security audit on Rust dependencies
	$(call print_header,"$(EMOJI_RUST) Running Security Audit")
	@cargo audit
	$(call print_success,"$(MSG_SUCCESS)")

# Development utilities
rust-watch-test: ## Watch for changes and run tests
	$(call print_header,"$(EMOJI_RUST) Watching for Changes (Tests)")
	@cargo watch -x "test $(RUST_TEST_FLAGS)"

rust-watch-check: ## Watch for changes and run check + clippy
	$(call print_header,"$(EMOJI_RUST) Watching for Changes (Check + Clippy)")
	@cargo watch -x "check $(CARGO_FEATURES)" -x "clippy $(CARGO_FEATURES)"

rust-watch-build: ## Watch for changes and build
	$(call print_header,"$(EMOJI_RUST) Watching for Changes (Build)")
	@cargo watch -x "build $(CARGO_FEATURES)"

# Comprehensive targets
rust-all-checks: rust-format-check rust-clippy rust-test-coverage rust-audit ## Run all Rust quality checks
	$(call print_success,"$(EMOJI_SUCCESS) All Rust checks passed!")

rust-quick-checks: rust-format-check rust-clippy rust-test-quick ## Quick Rust quality checks for development
	$(call print_success,"$(EMOJI_SUCCESS) Quick Rust checks completed!")
