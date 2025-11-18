"""Basic test to verify test discovery and core functionality."""

import sys

from graphbit import LlmConfig, version


def test_graphbit_import():
    """Test that graphbit can be imported."""
    assert version is not None
    assert LlmConfig is not None


def test_python_version_compatibility():
    """Test Python version compatibility."""
    # Should work with Python 3.10+
    assert sys.version_info >= (3, 10)
