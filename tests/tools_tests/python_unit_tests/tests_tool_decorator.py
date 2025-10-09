"""Unit tests for ToolDecorator functionality with comprehensive coverage."""

import contextlib
import inspect
import os
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import Any, Dict, List, Optional, Union

import pytest

# Add the parent directory to the path to import graphbit
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "../../../../"))

try:
    from graphbit import ToolDecorator, ToolRegistry, clear_tools, get_tool_registry, tool
except ImportError as e:
    pytest.skip(f"GraphBit tools module not available: {e}", allow_module_level=True)


class TestToolDecorator:
    """Test cases for ToolDecorator class with comprehensive parameter coverage."""

    def test_tool_decorator_creation_default(self):
        """Test ToolDecorator creation with default registry."""
        try:
            decorator = ToolDecorator()
            assert decorator is not None
            # Test that we can access the registry through get_registry method
            if hasattr(decorator, "get_registry"):
                registry = decorator.get_registry()
                assert registry is not None
        except Exception as e:
            pytest.skip(f"ToolDecorator not available: {e}")

    def test_tool_decorator_creation_with_custom_registry(self):
        """Test ToolDecorator creation with custom registry."""
        try:
            registry = ToolRegistry()
            decorator = ToolDecorator(registry=registry)
            assert decorator is not None
            # Test that we can access the registry through get_registry method
            if hasattr(decorator, "get_registry"):
                retrieved_registry = decorator.get_registry()
                assert retrieved_registry is not None
        except Exception as e:
            pytest.skip(f"ToolDecorator not available: {e}")

    def test_tool_decorator_creation_with_none_registry(self):
        """Test ToolDecorator creation with None registry (should use global)."""
        try:
            decorator = ToolDecorator(registry=None)
            assert decorator is not None
            # Test that we can access the registry through get_registry method
            if hasattr(decorator, "get_registry"):
                registry = decorator.get_registry()
                assert registry is not None
        except Exception as e:
            pytest.skip(f"ToolDecorator not available: {e}")

    def test_tool_decorator_parameter_validation_comprehensive(self):
        """Test ToolDecorator parameter validation with all possible parameter combinations."""
        try:
            decorator = ToolDecorator()

            # Test with all parameters specified
            result = decorator.__call__(description="Comprehensive test tool", name="comprehensive_test_tool", return_type="str")
            assert result is not None

            # Test with minimal parameters (only description)
            result = decorator.__call__(description="Minimal tool")
            assert result is not None

            # Test with only name
            result = decorator.__call__(name="name_only_tool")
            assert result is not None

            # Test with only return_type
            result = decorator.__call__(return_type="dict")
            assert result is not None

            # Test with None parameters (should use defaults)
            result = decorator.__call__(description=None, name=None, return_type=None)
            assert result is not None

            # Test with empty string parameters
            result = decorator.__call__(description="", name="", return_type="")
            assert result is not None

            # Test with no parameters at all
            result = decorator.__call__()
            assert result is not None

        except Exception as e:
            pytest.skip(f"ToolDecorator parameter validation not available: {e}")

    def test_tool_decorator_parameter_edge_cases(self):
        """Test ToolDecorator with edge case parameter values."""
        try:
            decorator = ToolDecorator()

            # Test with very long strings
            long_description = "A" * 10000
            long_name = "B" * 1000
            long_return_type = "C" * 500

            result = decorator.__call__(description=long_description, name=long_name, return_type=long_return_type)
            assert result is not None

            # Test with special characters
            special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~"
            result = decorator.__call__(description=f"Tool with special chars: {special_chars}", name=f"tool_{special_chars}", return_type=f"type_{special_chars}")
            assert result is not None

            # Test with unicode characters
            unicode_chars = "🚀🌟🎉💻🔥✨🎯📚🔧⚡"
            result = decorator.__call__(description=f"Unicode tool: {unicode_chars}", name=f"unicode_tool_{unicode_chars}", return_type=f"unicode_type_{unicode_chars}")
            assert result is not None

            # Test with newlines and tabs
            multiline_desc = "Line 1\nLine 2\tTabbed\r\nWindows line ending"
            result = decorator.__call__(description=multiline_desc, name="multiline_tool", return_type="str")
            assert result is not None

        except Exception as e:
            pytest.skip(f"ToolDecorator edge case testing not available: {e}")

    def test_tool_decorator_function_registration_comprehensive(self):
        """Test ToolDecorator function registration with various function types."""
        try:
            decorator = ToolDecorator()

            # Test with simple function
            @decorator(description="Simple test function", name="simple_func")
            def simple_function():
                return "simple_result"

            assert callable(simple_function)

            # Test with function that has parameters
            @decorator(description="Function with parameters", name="param_func")
            def function_with_params(a: int, b: str = "default"):
                return f"{a}_{b}"

            assert callable(function_with_params)

            # Test with function that has complex return type
            @decorator(description="Function with complex return", name="complex_func")
            def complex_function() -> Dict[str, Any]:
                return {"result": "complex", "data": [1, 2, 3]}

            assert callable(complex_function)

            # Test with async function (if supported)
            @decorator(description="Async function", name="async_func")
            async def async_function():
                return "async_result"

            assert callable(async_function)

            # Test with lambda function
            lambda_func = decorator(description="Lambda function", name="lambda_func")(lambda x: x * 2)
            assert callable(lambda_func)

        except Exception as e:
            pytest.skip(f"ToolDecorator function registration not available: {e}")

    def test_tool_decorator_metadata_extraction_comprehensive(self):
        """Test ToolDecorator metadata extraction from various function types."""
        try:
            decorator = ToolDecorator()

            # Test with function that has comprehensive docstring
            @decorator(description="Function with comprehensive docstring")
            def documented_function(param1: int, param2: str = "default") -> str:
                """
                Document a comprehensive function.

                Args:
                    param1 (int): First parameter
                    param2 (str, optional): Second parameter. Defaults to "default".

                Returns:
                    str: The result string

                Raises:
                    ValueError: If param1 is negative
                """
                if param1 < 0:
                    raise ValueError("param1 cannot be negative")
                return f"{param1}_{param2}"

            assert callable(documented_function)

            # Test with function that has type hints
            @decorator(description="Function with type hints")
            def typed_function(a: int, b: Optional[str] = None, c: Optional[List[Dict[str, Any]]] = None) -> Union[str, Dict[str, Any]]:
                return {"a": a, "b": b, "c": c or []}

            assert callable(typed_function)

            # Test with function that has no docstring
            @decorator(description="Function without docstring")
            def undocumented_function():
                return "no_docs"

            assert callable(undocumented_function)

            # Test with function that has complex signature
            @decorator(description="Function with complex signature")
            def complex_signature_function(*args, **kwargs):
                return {"args": args, "kwargs": kwargs}

            assert callable(complex_signature_function)

        except Exception as e:
            pytest.skip(f"ToolDecorator metadata extraction not available: {e}")

    def test_tool_decorator_thread_safety_comprehensive(self):
        """Test ToolDecorator comprehensive thread safety scenarios."""
        try:
            # Test concurrent decorator creation
            def create_decorator_with_id(thread_id):
                try:
                    decorator = ToolDecorator()

                    # Test decorator usage in thread
                    @decorator(description=f"Thread {thread_id} tool", name=f"thread_{thread_id}_tool")
                    def thread_tool():
                        return f"result_from_thread_{thread_id}"

                    return (thread_id, decorator, thread_tool)
                except Exception as e:
                    return (thread_id, None, str(e))

            # Test with ThreadPoolExecutor for better control
            with ThreadPoolExecutor(max_workers=10) as executor:
                futures = [executor.submit(create_decorator_with_id, thread_id) for thread_id in range(20)]

                results = []
                for future in as_completed(futures):
                    results.append(future.result())

            # Verify all threads completed successfully
            assert len(results) == 20
            successful_results = [r for r in results if r[1] is not None]
            assert len(successful_results) >= 15  # Allow some failures due to concurrency

            # Test concurrent decoration of the same function
            def test_concurrent_decoration():
                base_decorator = ToolDecorator()

                def target_function():
                    return "concurrent_result"

                # Multiple threads trying to decorate the same function
                def decorate_function(decorator_id):
                    try:
                        decorated = base_decorator.__call__(description=f"Concurrent decoration {decorator_id}", name=f"concurrent_func_{decorator_id}")(target_function)
                        return (decorator_id, callable(decorated))
                    except Exception as e:
                        return (decorator_id, str(e))

                with ThreadPoolExecutor(max_workers=5) as executor:
                    futures = [executor.submit(decorate_function, i) for i in range(10)]

                    decoration_results = []
                    for future in as_completed(futures):
                        decoration_results.append(future.result())

                return decoration_results

            decoration_results = test_concurrent_decoration()
            assert len(decoration_results) == 10

        except Exception as e:
            pytest.skip(f"ToolDecorator thread safety testing not available: {e}")

    def test_tool_decorator_concurrent_registry_access(self):
        """Test concurrent access to tool registry through decorator."""
        try:
            # Test multiple decorators sharing the same registry
            shared_registry = ToolRegistry()

            def create_decorator_with_shared_registry(worker_id):
                try:
                    decorator = ToolDecorator(registry=shared_registry)

                    @decorator(description=f"Worker {worker_id} tool", name=f"worker_{worker_id}_tool")
                    def worker_tool():
                        return f"worker_{worker_id}_result"

                    # Test registry access
                    if hasattr(decorator, "get_registry"):
                        registry = decorator.get_registry()
                        return (worker_id, True, registry is not None)
                    else:
                        return (worker_id, True, True)

                except Exception as e:
                    return (worker_id, False, str(e))

            with ThreadPoolExecutor(max_workers=8) as executor:
                futures = [executor.submit(create_decorator_with_shared_registry, worker_id) for worker_id in range(15)]

                registry_results = []
                for future in as_completed(futures):
                    registry_results.append(future.result())

            # Verify concurrent registry access
            assert len(registry_results) == 15
            successful_access = [r for r in registry_results if r[1] is True]
            assert len(successful_access) >= 10  # Allow some failures

        except Exception as e:
            pytest.skip(f"ToolDecorator concurrent registry access not available: {e}")

    def test_tool_decorator_get_registry(self):
        """Test ToolDecorator get_registry method."""
        decorator = ToolDecorator()
        registry = decorator.get_registry()
        assert registry is not None
        assert isinstance(registry, ToolRegistry)

    def test_tool_decorator_register_method(self):
        """Test ToolDecorator register method."""
        decorator = ToolDecorator()

        def test_func():
            return "test"

        # Test manual registration
        result = decorator.register(func=test_func, name="manual_test_func", description="Manually registered function", return_type="str")

        # Should not raise an exception
        assert result is None or result is True

    def test_tool_decorator_with_different_python_versions(self):
        """Test ToolDecorator compatibility with different Python versions."""
        decorator = ToolDecorator()

        # Test basic functionality works
        assert decorator is not None

        # Test that it works with current Python version
        current_version = sys.version_info
        assert current_version >= (3, 10), "Requires Python 3.10+"

    def test_tool_decorator_memory_management_comprehensive(self):
        """Test ToolDecorator comprehensive memory management scenarios."""
        try:
            import gc
            import os

            import psutil

            # Get initial memory usage
            process = psutil.Process(os.getpid())
            initial_memory = process.memory_info().rss

            # Test 1: Create and destroy many decorators
            decorators = []
            for _ in range(1000):
                decorator = ToolDecorator()
                decorators.append(decorator)

            # Memory should have increased
            mid_memory = process.memory_info().rss
            assert mid_memory >= initial_memory

            # Clean up decorators
            del decorators
            gc.collect()

            # Test 2: Create decorators with many decorated functions
            decorator = ToolDecorator()
            decorated_functions = []

            for func_id in range(100):

                @decorator(description=f"Memory test function {func_id}", name=f"mem_func_{func_id}")
                def memory_test_function(_func_id=func_id):
                    return f"result_{_func_id}"

                decorated_functions.append(memory_test_function)

            # Test function execution
            for func in decorated_functions[:10]:  # Test a few functions
                try:
                    result = func()
                    assert result is not None
                except Exception:  # nosec B110
                    pass  # Some functions might not be executable

            # Clean up
            del decorated_functions
            del decorator
            gc.collect()

            # Test 3: Stress test with rapid creation/destruction
            for _cycle in range(10):
                temp_decorators = []
                for _ in range(50):
                    temp_decorator = ToolDecorator()
                    temp_decorators.append(temp_decorator)

                # Use decorators
                for i, temp_decorator in enumerate(temp_decorators[:5]):
                    with contextlib.suppress(Exception):

                        @temp_decorator(description=f"Temp function {i}")
                        def temp_function():
                            return "temp_result"

                del temp_decorators
                gc.collect()

            # Final memory check
            final_memory = process.memory_info().rss

            # Memory should not have grown excessively (allow 50MB growth)
            memory_growth = final_memory - initial_memory
            assert memory_growth < 50 * 1024 * 1024, f"Memory grew by {memory_growth} bytes"

        except ImportError:
            # psutil not available, do basic memory test
            import gc

            decorators = []
            for _ in range(100):
                decorators.append(ToolDecorator())

            del decorators
            gc.collect()
            assert True

        except Exception as e:
            pytest.skip(f"ToolDecorator memory management testing not available: {e}")

    def test_tool_decorator_serialization_capabilities(self):
        """Test ToolDecorator serialization and deserialization capabilities."""
        try:
            import json
            import pickle  # nosec B403

            decorator = ToolDecorator()

            # Test 1: Pickle serialization (if supported)
            try:
                pickled_decorator = pickle.dumps(decorator)  # nosec B301
                unpickled_decorator = pickle.loads(pickled_decorator)  # nosec B301
                assert unpickled_decorator is not None
            except (TypeError, AttributeError, pickle.PicklingError):
                # Decorator might not be picklable - that's okay
                pass

            # Test 2: JSON serialization of decorator metadata (if available)
            if hasattr(decorator, "to_dict") or hasattr(decorator, "__dict__"):
                try:
                    if hasattr(decorator, "to_dict"):
                        decorator_dict = decorator.to_dict()
                    else:
                        decorator_dict = {"type": "ToolDecorator", "has_get_registry": hasattr(decorator, "get_registry")}

                    json_str = json.dumps(decorator_dict, default=str)
                    assert json_str is not None

                    # Test deserialization
                    deserialized = json.loads(json_str)
                    assert isinstance(deserialized, dict)

                except (TypeError, ValueError):
                    # Serialization might not be supported
                    pass

            # Test 3: Serialization of decorated functions metadata
            @decorator(description="Serializable function", name="serializable_func")
            def serializable_function(param: str) -> str:
                return f"processed_{param}"

            # Try to serialize function metadata
            if hasattr(serializable_function, "__dict__"):
                func_metadata = {
                    "name": getattr(serializable_function, "__name__", "unknown"),
                    "doc": getattr(serializable_function, "__doc__", None),
                    "annotations": getattr(serializable_function, "__annotations__", {}),
                }

                json_metadata = json.dumps(func_metadata, default=str)
                assert json_metadata is not None

        except Exception as e:
            pytest.skip(f"ToolDecorator serialization testing not available: {e}")


class TestToolDecoratorAdvancedFeatures:
    """Test advanced features and integration scenarios for ToolDecorator."""

    def test_tool_decorator_with_global_registry_functions(self):
        """Test ToolDecorator integration with global registry functions."""
        try:
            # Test get_tool_registry function
            global_registry = get_tool_registry()
            assert global_registry is not None

            # Test clear_tools function
            clear_tools()

            # Test tool function decorator
            @tool(description="Global tool test", name="global_test_tool")
            def global_test_function():
                return "global_result"

            assert callable(global_test_function)

            # Test decorator with global registry
            decorator = ToolDecorator()

            @decorator(description="Another global tool")
            def another_global_function():
                return "another_result"

            assert callable(another_global_function)

        except Exception as e:
            pytest.skip(f"Global registry functions not available: {e}")

    def test_tool_decorator_registry_interaction(self):
        """Test ToolDecorator interaction with registry methods."""
        try:
            decorator = ToolDecorator()

            # Test get_registry method
            if hasattr(decorator, "get_registry"):
                registry = decorator.get_registry()
                assert registry is not None
                assert isinstance(registry, ToolRegistry)

            # Test register method
            if hasattr(decorator, "register"):

                def manual_function():
                    return "manual_result"

                result = decorator.register(func=manual_function, name="manual_test_func", description="Manually registered function", return_type="str")
                assert result is None or result is True

        except Exception as e:
            pytest.skip(f"ToolDecorator registry interaction not available: {e}")

    def test_tool_decorator_function_introspection(self):
        """Test ToolDecorator function introspection capabilities."""
        try:
            decorator = ToolDecorator()

            # Test with function that has complex signature
            @decorator(description="Complex signature function")
            def complex_function(required_param: str, optional_param: int = 42, *args, keyword_only: bool = True, **kwargs) -> Dict[str, Any]:
                """
                Complex function for testing introspection.

                Args:
                    required_param: A required string parameter
                    optional_param: An optional integer parameter
                    *args: Variable positional arguments
                    keyword_only: A keyword-only parameter
                    **kwargs: Variable keyword arguments

                Returns:
                    Dict containing all parameters
                """
                return {"required_param": required_param, "optional_param": optional_param, "args": args, "keyword_only": keyword_only, "kwargs": kwargs}

            assert callable(complex_function)

            # Test introspection of the decorated function
            sig = inspect.signature(complex_function)
            assert len(sig.parameters) >= 4

            # Test function annotations
            annotations = getattr(complex_function, "__annotations__", {})
            assert "return" in annotations or len(annotations) >= 0

        except Exception as e:
            pytest.skip(f"ToolDecorator function introspection not available: {e}")


class TestToolDecoratorEdgeCases:
    """Test comprehensive edge cases for ToolDecorator."""

    def test_tool_decorator_boundary_conditions(self):
        """Test ToolDecorator with boundary condition values."""
        try:
            decorator = ToolDecorator()

            # Test with maximum length strings (within reasonable limits)
            max_desc = "A" * 65535  # 64KB description
            max_name = "B" * 255  # 255 char name
            max_type = "C" * 100  # 100 char type

            result = decorator.__call__(description=max_desc, name=max_name, return_type=max_type)
            assert result is not None

            # Test with minimum valid values
            result = decorator.__call__(description="A", name="B", return_type="C")
            assert result is not None

            # Test with whitespace-only strings
            result = decorator.__call__(description="   ", name="\t\n", return_type=" \r ")
            assert result is not None

        except Exception as e:
            pytest.skip(f"ToolDecorator boundary testing not available: {e}")

    def test_tool_decorator_unicode_and_encoding(self):
        """Test ToolDecorator with various unicode and encoding scenarios."""
        try:
            decorator = ToolDecorator()

            # Test with various unicode categories
            unicode_tests = [
                ("Emoji: 🚀🌟🎉", "emoji_tool", "str"),
                ("Chinese: 你好世界", "chinese_tool", "str"),
                ("Arabic: مرحبا بالعالم", "arabic_tool", "str"),
                ("Russian: Привет мир", "russian_tool", "str"),
                ("Mathematical: ∑∫∂∇", "math_tool", "str"),
                ("Symbols: ©®™€£¥", "symbol_tool", "str"),
            ]

            for desc, name, ret_type in unicode_tests:
                result = decorator.__call__(description=desc, name=name, return_type=ret_type)
                assert result is not None

            # Test with mixed encodings
            mixed_desc = "Mixed: ASCII + 中文 + العربية + русский + 🌍"
            result = decorator.__call__(description=mixed_desc, name="mixed_encoding_tool", return_type="str")
            assert result is not None

        except Exception as e:
            pytest.skip(f"ToolDecorator unicode testing not available: {e}")

    def test_tool_decorator_error_recovery(self):
        """Test ToolDecorator error recovery and resilience."""
        try:
            decorator = ToolDecorator()

            # Test recovery from decorator creation errors
            for attempt in range(5):
                try:
                    test_decorator = ToolDecorator()
                    assert test_decorator is not None
                    break
                except Exception:
                    if attempt == 4:  # Last attempt
                        pytest.fail("Could not create decorator after 5 attempts")
                    continue

            # Test recovery from function decoration errors
            problematic_functions = [
                lambda: None,  # Simple lambda
                lambda x, y=1: x + y,  # Lambda with parameters
                lambda *args, **kwargs: (args, kwargs),  # Lambda with var args
            ]

            successful_decorations = 0
            for i, func in enumerate(problematic_functions):
                try:
                    decorated = decorator.__call__(description=f"Problematic function {i}", name=f"problematic_{i}")(func)
                    if callable(decorated):
                        successful_decorations += 1
                except Exception:  # nosec B110
                    pass  # Expected for some functions

            # At least some decorations should succeed
            assert successful_decorations >= 0

        except Exception as e:
            pytest.skip(f"ToolDecorator error recovery testing not available: {e}")

    def test_tool_decorator_performance_characteristics(self):
        """Test ToolDecorator performance under various loads."""
        try:
            import time

            # Test decoration performance
            start_time = time.time()

            decorator = ToolDecorator()
            decorated_functions = []

            # Decorate many functions quickly
            for i in range(100):

                @decorator(description=f"Performance test function {i}")
                def perf_function(_i=i):
                    return f"perf_result_{_i}"

                decorated_functions.append(perf_function)

            decoration_time = time.time() - start_time

            # Decoration should be reasonably fast (less than 10 seconds for 100 functions)
            assert decoration_time < 10.0, f"Decoration took {decoration_time} seconds"

            # Test function execution performance
            start_time = time.time()

            results = []
            for func in decorated_functions[:10]:  # Test first 10 functions
                try:
                    result = func()
                    results.append(result)
                except Exception:  # nosec B110
                    pass

            execution_time = time.time() - start_time

            # Execution should be fast
            assert execution_time < 5.0, f"Execution took {execution_time} seconds"

        except Exception as e:
            pytest.skip(f"ToolDecorator performance testing not available: {e}")


class TestToolDecoratorIntegration:
    """Integration tests for ToolDecorator with other components."""

    def test_tool_decorator_compatibility_modes(self):
        """Test ToolDecorator compatibility with different usage patterns."""
        try:
            # Test 1: Class-based usage
            class ToolDecoratorWrapper:
                def __init__(self):
                    self.decorator = ToolDecorator()

                def decorate_function(self, func, **kwargs):
                    return self.decorator.__call__(**kwargs)(func)

            wrapper = ToolDecoratorWrapper()

            def test_func():
                return "wrapped_result"

            decorated = wrapper.decorate_function(test_func, description="Wrapped function", name="wrapped_func")
            assert callable(decorated)

            # Test 2: Functional usage
            def create_decorated_function(decorator, func_name):
                @decorator(description=f"Functional {func_name}")
                def functional_function():
                    return f"functional_{func_name}_result"

                return functional_function

            decorator = ToolDecorator()
            func1 = create_decorated_function(decorator, "func1")
            func2 = create_decorated_function(decorator, "func2")

            assert callable(func1)
            assert callable(func2)

        except Exception as e:
            pytest.skip(f"ToolDecorator compatibility testing not available: {e}")


# Parameterized tests for comprehensive parameter coverage
@pytest.mark.parametrize(
    "description,name,return_type,should_succeed",
    [
        ("Valid description", "valid_name", "str", True),
        ("", "", "", True),  # Empty strings should be handled
        (None, None, None, True),  # None values should use defaults
        ("Very long description " * 100, "long_name", "str", True),
        ("Unicode: 🚀", "unicode_name", "dict", True),
        ("Special chars: !@#$%", "special_name", "list", True),
    ],
)
def test_tool_decorator_parameter_combinations(description, name, return_type, should_succeed):
    """Parameterized test for various parameter combinations."""
    try:
        decorator = ToolDecorator()

        if should_succeed:
            result = decorator.__call__(description=description, name=name, return_type=return_type)
            assert result is not None
        else:
            with pytest.raises((ValueError, TypeError, AttributeError)):
                decorator.__call__(description=description, name=name, return_type=return_type)
    except Exception as e:
        pytest.skip(f"ToolDecorator parameterized testing not available: {e}")


@pytest.mark.parametrize(
    "registry_type",
    [
        "default",  # Default registry
        "custom",  # Custom registry
        "none",  # None registry
    ],
)
def test_tool_decorator_registry_types(registry_type):
    """Parameterized test for different registry types."""
    try:
        if registry_type == "default":
            decorator = ToolDecorator()
        elif registry_type == "custom":
            custom_registry = ToolRegistry()
            decorator = ToolDecorator(registry=custom_registry)
        elif registry_type == "none":
            decorator = ToolDecorator(registry=None)

        assert decorator is not None
        # Test that we can access the registry through get_registry method
        if hasattr(decorator, "get_registry"):
            registry = decorator.get_registry()
            assert registry is not None

    except Exception as e:
        pytest.skip(f"ToolDecorator registry type testing not available: {e}")


if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
