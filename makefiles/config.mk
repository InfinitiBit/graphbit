# GraphBit Makefile Configuration
# ================================
# Centralized configuration for all Makefiles
# This file contains all paths, tool configurations, and settings

# Project Information
PROJECT_NAME := graphbit
PROJECT_VERSION := 0.1.0

# Load environment variables from .env if present
ifneq (,$(wildcard .env))
	export $(shell sed 's/=.*//' .env)
endif

# Default environment type (can be overridden by .env)
ENV_TYPE ?= poetry

# Cross-platform detection
ifeq ($(OS),Windows_NT)
	SHELL_TYPE := windows
	PATH_SEP := \\
	NULL_DEVICE := NUL
	PYTHON_VENV_ACTIVATE := .venv\Scripts\activate.bat
	PYTHON_VENV_PYTHON := .venv\Scripts\python.exe
else
	SHELL_TYPE := unix
	PATH_SEP := /
	NULL_DEVICE := /dev/null
	PYTHON_VENV_ACTIVATE := .venv/bin/activate
	PYTHON_VENV_PYTHON := .venv/bin/python
endif

# Python Environment Detection
ifeq ($(ENV_TYPE),conda)
	PYTHON_ENV := conda activate $(PROJECT_NAME)
else ifeq ($(ENV_TYPE),venv)
	ifeq ($(SHELL_TYPE),windows)
		PYTHON_ENV := call $(PYTHON_VENV_ACTIVATE)
	else
		PYTHON_ENV := . $(PYTHON_VENV_ACTIVATE)
	endif
else ifeq ($(ENV_TYPE),poetry)
	PYTHON_ENV := poetry run
else
	PYTHON_ENV := conda activate $(PROJECT_NAME)
endif

# Test Directories
TESTS_ROOT := tests
PYTHON_UNIT_TESTS := $(TESTS_ROOT)/python_unit_tests
PYTHON_INTEGRATION_TESTS := $(TESTS_ROOT)/python_integration_tests
RUST_UNIT_TESTS := $(TESTS_ROOT)/rust_unit_tests
RUST_INTEGRATION_TESTS := $(TESTS_ROOT)/rust_integration_tests

# Source Directories
CORE_SRC := core/src
PYTHON_SRC := python/src
GRAPHBIT_SRC := graphbit
BENCHMARKS_SRC := benchmarks
EXAMPLES_SRC := examples
DOCS_SRC := docs

# Build Directories
TARGET_DIR := target
BUILD_DIR := build
DIST_DIR := dist

# Coverage Configuration
RUST_COVERAGE_EXCLUDE := 'core/src/llm/|core/src/embeddings.rs|python/src/'
RUST_COVERAGE_OUTPUT := $(TARGET_DIR)/llvm-cov/html
PYTHON_COVERAGE_OUTPUT := htmlcov

# Tool Configuration
CARGO_FEATURES := --workspace --all-targets --all-features
CARGO_TEST_FLAGS := --workspace
PYTEST_FLAGS := --import-mode=importlib
PYTEST_COVERAGE_FLAGS := --cov-branch --cov-report=term-missing:skip-covered --cov-report=html

# Cross-platform specific configurations
ifeq ($(SHELL_TYPE),windows)
	PYTEST_COV_CONFIG := 
	ENV_VAR_SET := powershell -Command "[Environment]::SetEnvironmentVariable('TEST_REMOTE_URLS', 'true', 'Process');"
else
	PYTEST_COV_CONFIG := --cov-config=$(NULL_DEVICE)
	ENV_VAR_SET := export TEST_REMOTE_URLS=true &&
endif

# Validation Paths (for checking if directories exist)
REQUIRED_DIRS := $(TESTS_ROOT) $(PYTHON_UNIT_TESTS) $(PYTHON_INTEGRATION_TESTS) \
                 $(RUST_UNIT_TESTS) $(RUST_INTEGRATION_TESTS) $(CORE_SRC)

# Tool Availability Checks
REQUIRED_TOOLS := cargo poetry python
OPTIONAL_TOOLS := cargo-llvm-cov pytest typos detect-secrets

# Color codes for output
COLOR_RESET := \033[0m
COLOR_BOLD := \033[1m
COLOR_RED := \033[31m
COLOR_GREEN := \033[32m
COLOR_YELLOW := \033[33m
COLOR_BLUE := \033[34m
COLOR_MAGENTA := \033[35m
COLOR_CYAN := \033[36m

# Emoji and formatting
EMOJI_ROCKET := 🚀
EMOJI_TEST := 🧪
EMOJI_RUST := 🦀
EMOJI_PYTHON := 🐍
EMOJI_SUCCESS := ✅
EMOJI_WARNING := ⚠️
EMOJI_ERROR := ❌
EMOJI_INFO := 📋
EMOJI_BUILD := 🔨
EMOJI_CLEAN := 🧹

# Common Messages
MSG_STARTING := "$(EMOJI_ROCKET) Starting"
MSG_SUCCESS := "$(EMOJI_SUCCESS) Completed successfully!"
MSG_ERROR := "$(EMOJI_ERROR) Failed"
MSG_WARNING := "$(EMOJI_WARNING) Warning"
