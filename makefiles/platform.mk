# GraphBit Platform-Specific Configuration
# =========================================
# Cross-platform compatibility layer

# Include base configuration
include makefiles/config.mk

# Platform-specific utilities (simplified for compatibility)
ifeq ($(SHELL_TYPE),windows)
	PLATFORM_ECHO := @echo
	PLATFORM_MKDIR := @if not exist
	PLATFORM_RMDIR := @if exist
else
	PLATFORM_ECHO := @echo
	PLATFORM_MKDIR := @mkdir -p
	PLATFORM_RMDIR := @rm -rf
endif
	
# Common utility functions (simplified)
print_header = @echo "" && echo "$(COLOR_BOLD)$(COLOR_CYAN)$(1)$(COLOR_RESET)" && echo "$(COLOR_CYAN)================================================$(COLOR_RESET)"
print_success = @echo "$(COLOR_GREEN)$(1)$(COLOR_RESET)"
print_warning = @echo "$(COLOR_YELLOW)$(1)$(COLOR_RESET)"
print_error = @echo "$(COLOR_RED)$(1)$(COLOR_RESET)"
print_info = @echo "$(COLOR_BLUE)$(1)$(COLOR_RESET)"

# Validation utilities
validate_directory = @if [ ! -d "$(1)" ]; then echo "$(EMOJI_ERROR) Directory $(1) does not exist"; exit 1; else echo "$(EMOJI_SUCCESS) Directory $(1) exists"; fi
validate_tool = @command -v $(1) >/dev/null 2>&1 && echo "$(EMOJI_SUCCESS) Tool $(1) is available" || { echo "$(EMOJI_ERROR) Tool $(1) not found"; exit 1; }

# File operations
clean_python_cache = find . -type d -name "__pycache__" -exec rm -rf {} + 2>$(NULL_DEVICE) || true; find . -type f -name "*.pyc" -delete 2>$(NULL_DEVICE) || true; find . -type d -name "*.egg-info" -exec rm -rf {} + 2>$(NULL_DEVICE) || true

# Test result formatting
format_test_results = @echo "" && echo "$(COLOR_BOLD)$(COLOR_GREEN)Test Results Summary:$(COLOR_RESET)" && echo "$(COLOR_GREEN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(COLOR_RESET)"
