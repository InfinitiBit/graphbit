"""
Pytest configuration for GraphBit Python tests.

Provides fixtures and setup for tokio runtime required by Rust bindings.
"""

import pytest
import asyncio
import threading


# Global tokio runtime thread
_runtime_thread = None
_runtime_loop = None


def _run_event_loop(loop):
    """Run event loop in a separate thread."""
    asyncio.set_event_loop(loop)
    loop.run_forever()


@pytest.fixture(scope="session", autouse=True)
def tokio_runtime():
    """
    Create and maintain a tokio runtime for the entire test session.
    
    The GraphBit Python bindings require a tokio runtime to be running
    when creating MemoryManager and other async components.
    """
    global _runtime_thread, _runtime_loop
    
    # Create a new event loop
    _runtime_loop = asyncio.new_event_loop()
    
    # Start the event loop in a separate thread
    _runtime_thread = threading.Thread(target=_run_event_loop, args=(_runtime_loop,), daemon=True)
    _runtime_thread.start()
    
    # Give the thread time to start
    import time
    time.sleep(0.1)
    
    yield _runtime_loop
    
    # Cleanup
    if _runtime_loop and _runtime_loop.is_running():
        _runtime_loop.call_soon_threadsafe(_runtime_loop.stop)
    if _runtime_thread:
        _runtime_thread.join(timeout=1.0)

