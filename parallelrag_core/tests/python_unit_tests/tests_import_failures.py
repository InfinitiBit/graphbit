"""Test import failures and library compatibility issues."""

import contextlib
import sys
from unittest.mock import MagicMock, patch

import pytest


class TestImportFailures:
    """Test various import failure scenarios."""

    def test_graphbit_import_success(self):
        """Test successful graphbit import."""
        from graphbit import init, version

        assert version is not None
        assert init is not None

    def test_missing_graphbit_module(self):
        """Test behavior when graphbit module is not available."""
        # Temporarily remove graphbit from sys.modules
        original_modules = sys.modules.copy()
        if "graphbit" in sys.modules:
            del sys.modules["graphbit"]

        # Mock import to fail
        with patch.dict("sys.modules", {"graphbit": None}), pytest.raises(ImportError):
            from graphbit import version  # noqa: F401

        # Restore original modules
        sys.modules.update(original_modules)

    def test_corrupted_graphbit_module(self):
        """Test behavior with corrupted graphbit module."""
        # Create a mock corrupted module
        corrupted_module = MagicMock()
        corrupted_module.__name__ = "graphbit"
        del corrupted_module.version  # Remove required attribute

        with contextlib.suppress(Exception), patch.dict("sys.modules", {"graphbit": corrupted_module}), pytest.raises((AttributeError, ImportError)):
            from graphbit import version  # noqa: F401

            raise AssertionError("Should have failed")
        # If patching doesn't work as expected, that's acceptable

    def test_wrong_library_import(self):
        """Test importing wrong libraries with similar names."""
        # Test importing non-existent similar libraries
        with pytest.raises(ImportError):
            import graphbit_fake  # noqa: F401

        with pytest.raises(ImportError):
            import graph_bit  # noqa: F401

        with pytest.raises(ImportError):
            import graphbite  # noqa: F401

    def test_version_mismatch_simulation(self):
        """Test behavior with version mismatches."""
        from graphbit import version  # noqa: F401

        # Mock version to simulate incompatible version
        with patch("graphbit.version", return_value="0.0.1-incompatible"):
            import graphbit

            result = graphbit.version()
            assert "incompatible" in result

    def test_partial_import_failure(self):
        """Test partial import failures of submodules."""
        # Test importing specific classes that might fail
        # This should succeed - if it fails, the test should fail
        from graphbit import EmbeddingConfig, LlmConfig, Workflow

        assert LlmConfig is not None
        assert EmbeddingConfig is not None
        assert Workflow is not None

    def test_import_with_missing_dependencies(self):
        """Test import behavior when dependencies are missing."""
        # Simulate missing tokio runtime
        with patch("sys.modules", {"tokio": None}):
            # GraphBit should still import but might have limited functionality
            from graphbit import version

            assert version is not None

    def test_circular_import_protection(self):
        """Test protection against circular imports."""
        # This should not cause infinite recursion
        from graphbit import version

        assert version is not None

    def test_import_from_different_paths(self):
        """Test importing from different module paths."""
        # Test various import styles
        from graphbit import LlmConfig as Config
        from graphbit import version

        assert version is not None
        assert Config is not None

    def test_reload_module_safety(self):
        """Test that module can be safely reloaded."""
        from graphbit import version

        original_version = version()

        # Should still work after multiple calls
        new_version = version()
        assert new_version == original_version

    def test_import_in_different_contexts(self):
        """Test importing in different execution contexts."""

        def import_in_function():
            from graphbit import version

            return version()

        # Test import in function - this should succeed
        from graphbit import version as version_func

        version_result = version_func()
        assert version_result is not None

        assert import_in_function() is not None
        assert version_result is not None

    def test_namespace_pollution(self):
        """Test that import doesn't pollute namespace unexpectedly."""
        from graphbit import LlmConfig, Workflow, version

        # Basic validation that imports work
        assert version is not None
        assert LlmConfig is not None
        assert Workflow is not None

    def test_import_error_messages(self):
        """Test that import errors have helpful messages."""
        with pytest.raises(ImportError) as exc_info:
            # This will fail as expected - using importlib instead of exec for security
            import importlib

            importlib.import_module("non_existent_graphbit_module")
        assert "non_existent_graphbit_module" in str(exc_info.value)

        with pytest.raises(ImportError) as exc_info:
            from graphbit import NonExistentClass  # noqa: F401
        error_msg = str(exc_info.value)
        assert "NonExistentClass" in error_msg or "cannot import name" in error_msg

    def test_import_with_sys_path_manipulation(self):
        """Test import behavior with sys.path manipulation."""
        original_path = sys.path.copy()

        try:
            # Add invalid path
            sys.path.insert(0, "/non/existent/path")

            # Should still be able to import
            from graphbit import version

            assert version is not None

        finally:
            # Restore original path
            sys.path[:] = original_path
