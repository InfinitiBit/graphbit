"""Basic test to verify test discovery."""

import pytest


def test_graphbit_import():
    """Test that graphbit can be imported."""
    import graphbit

    assert graphbit is not None


def test_graphbit_version():
    """Test that graphbit version is available."""
    import graphbit

    version = graphbit.version()
    assert isinstance(version, str)
    assert len(version) > 0


@pytest.mark.asyncio
async def test_async_basic():
    """Basic async test."""
    assert True
