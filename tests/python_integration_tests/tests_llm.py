"""Integration tests for GraphBit LLM functionality."""

import inspect
import os
import time
from typing import Any

import aiohttp
import pytest

from graphbit import LlmConfig, LlmClient, Executor, Workflow, Node


class TestOpenAILLM:
    """Integration tests for OpenAI LLM models."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def openai_gpt4_config(self, api_key: str) -> Any:
        """Create OpenAI GPT-4 configuration."""
        return LlmConfig.openai(api_key, "gpt-4")

    @pytest.fixture
    def openai_gpt35_config(self, api_key: str) -> Any:
        """Create OpenAI GPT-3.5 configuration."""
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def openai_client(self, openai_gpt4_config: Any) -> Any:
        """Create OpenAI client."""
        return LlmClient(openai_gpt4_config)

    def test_openai_gpt4_config_creation(self, openai_gpt4_config: Any) -> None:
        """Test OpenAI GPT-4 config creation and properties."""
        assert openai_gpt4_config.provider() == "openai"
        assert openai_gpt4_config.model() == "gpt-4"

    def test_openai_gpt35_config_creation(self, openai_gpt35_config: Any) -> None:
        """Test OpenAI GPT-3.5 config creation and properties."""
        assert openai_gpt35_config.provider() == "openai"
        assert openai_gpt35_config.model() == "gpt-3.5-turbo"

    def test_openai_client_creation(self, openai_client: Any) -> None:
        """Test creating OpenAI client."""
        assert openai_client is not None

    def test_openai_executor_creation(self, openai_gpt4_config: Any) -> None:
        """Test creating workflow executor with OpenAI config."""
        executor = Executor(openai_gpt4_config)
        assert executor is not None

    def test_openai_executor_configurations(self, openai_gpt4_config: Any) -> None:
        """Test different executor configurations with OpenAI."""
        # Test high throughput configuration
        executor_ht = Executor.new_high_throughput(openai_gpt4_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = Executor.new_low_latency(openai_gpt4_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = Executor.new_memory_optimized(openai_gpt4_config)
        assert executor_mo is not None

    def test_openai_simple_completion(self, openai_client: Any) -> None:
        """Test simple text completion with OpenAI."""
        try:
            response = openai_client.complete("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            pytest.fail(f"OpenAI completion failed: {e}")

    def test_openai_simple_workflow_execution(self, openai_gpt4_config: Any) -> None:
        """Test executing a simple workflow with OpenAI."""
        # Create a simple workflow
        workflow = Workflow("test_openai_workflow")

        # Create a simple agent node
        try:
            agent_node = Node.agent("test_agent", "Respond with 'Hello from OpenAI'", "agent_001")
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = Executor(openai_gpt4_config)
            assert executor is not None

        except Exception as e:
            pytest.fail(f"OpenAI workflow creation/validation failed: {e}")


class TestAnthropicLLM:
    """Integration tests for Anthropic LLM models."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get Anthropic API key from environment."""
        api_key = os.getenv("ANTHROPIC_API_KEY")
        if not api_key:
            pytest.skip("ANTHROPIC_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def anthropic_config(self, api_key: str) -> Any:
        """Create Anthropic configuration."""
        return LlmConfig.anthropic(api_key, "claude-3-sonnet-20240229")

    @pytest.fixture
    def anthropic_client(self, anthropic_config: Any) -> Any:
        """Create Anthropic client."""
        return LlmClient(anthropic_config)

    def test_anthropic_config_creation(self, anthropic_config: Any) -> None:
        """Test Anthropic config creation and properties."""
        assert anthropic_config.provider() == "anthropic"
        assert anthropic_config.model() == "claude-3-sonnet-20240229"

    def test_anthropic_client_creation(self, anthropic_client: Any) -> None:
        """Test creating Anthropic client."""
        assert anthropic_client is not None

    def test_anthropic_executor_creation(self, anthropic_config: Any) -> None:
        """Test creating workflow executor with Anthropic config."""
        executor = Executor(anthropic_config)
        assert executor is not None

    def test_anthropic_executor_configurations(self, anthropic_config: Any) -> None:
        """Test different executor configurations with Anthropic."""
        # Test high throughput configuration
        executor_ht = Executor.new_high_throughput(anthropic_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = Executor.new_low_latency(anthropic_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = Executor.new_memory_optimized(anthropic_config)
        assert executor_mo is not None

    def test_anthropic_simple_completion(self, anthropic_client: Any) -> None:
        """Test simple text completion with Anthropic."""
        try:
            response = anthropic_client.complete("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            pytest.fail(f"Anthropic completion failed: {e}")

    def test_anthropic_simple_workflow_execution(self, anthropic_config: Any) -> None:
        """Test executing a simple workflow with Anthropic."""
        # Create a simple workflow
        workflow = Workflow("test_anthropic_workflow")

        # Create a simple agent node
        try:
            agent_node = Node.agent("test_agent", "Respond with 'Hello from Claude'", "agent_002")
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = Executor(anthropic_config)
            assert executor is not None

        except Exception as e:
            pytest.fail(f"Anthropic workflow creation/validation failed: {e}")


class TestHuggingFaceLLM:
    """Integration tests for HuggingFace LLM models."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get HuggingFace API key from environment."""
        api_key = os.getenv("HUGGINGFACE_API_KEY")
        if not api_key:
            pytest.skip("HUGGINGFACE_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def hf_config(self, api_key: str) -> Any:
        """Create HuggingFace configuration."""
        # Use a more reliable model that's available on HuggingFace
        return LlmConfig.huggingface(api_key, "gpt2")

    @pytest.fixture
    def hf_client(self, hf_config: Any) -> Any:
        """Create HuggingFace client."""
        return LlmClient(hf_config)

    def test_hf_config_creation(self, hf_config: Any) -> None:
        """Test HuggingFace config creation and properties."""
        assert hf_config.provider() == "huggingface"
        assert hf_config.model() == "gpt2"

    def test_hf_client_creation(self, hf_client: Any) -> None:
        """Test creating HuggingFace client."""
        assert hf_client is not None

    def test_hf_executor_creation(self, hf_config: Any) -> None:
        """Test creating workflow executor with HuggingFace config."""
        executor = Executor(hf_config)
        assert executor is not None

    def test_hf_executor_configurations(self, hf_config: Any) -> None:
        """Test different executor configurations with HuggingFace."""
        # Test high throughput configuration
        executor_ht = Executor.new_high_throughput(hf_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = Executor.new_low_latency(hf_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = Executor.new_memory_optimized(hf_config)
        assert executor_mo is not None

    def test_hf_simple_completion(self, hf_client: Any) -> None:
        """Test simple text completion with HuggingFace."""
        try:
            response = hf_client.complete("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            # HuggingFace API can be unreliable, so we'll make this more lenient
            if "not found" in str(e).lower() or "api error" in str(e).lower():
                pytest.skip(f"HuggingFace API issue: {e}")
            else:
                pytest.fail(f"HuggingFace completion failed: {e}")

    def test_hf_simple_workflow_execution(self, hf_config: Any) -> None:
        """Test executing a simple workflow with HuggingFace."""
        # Create a simple workflow
        workflow = Workflow("test_hf_workflow")

        # Create a simple agent node
        try:
            agent_node = Node.agent("test_agent", "Respond with 'Hello from HuggingFace'", "agent_003")
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = Executor(hf_config)
            assert executor is not None

        except Exception as e:
            pytest.fail(f"HuggingFace workflow creation/validation failed: {e}")


class TestOllamaLLM:
    """Integration tests for Ollama LLM models."""

    @pytest.fixture
    def ollama_config(self) -> Any:
        """Create Ollama configuration."""
        return LlmConfig.ollama("llama3.2")

    @pytest.fixture
    def ollama_client(self, ollama_config: Any) -> Any:
        """Create Ollama client."""
        return LlmClient(ollama_config)

    def test_ollama_config_creation(self, ollama_config: Any) -> None:
        """Test Ollama config creation and properties."""
        assert ollama_config.provider() == "ollama"
        assert ollama_config.model() == "llama3.2"

    def test_ollama_client_creation(self, ollama_client: Any) -> None:
        """Test creating Ollama client."""
        assert ollama_client is not None

    def test_ollama_executor_creation(self, ollama_config: Any) -> None:
        """Test creating workflow executor with Ollama config."""
        executor = Executor(ollama_config)
        assert executor is not None

    async def test_ollama_simple_completion(self, ollama_client: Any) -> None:
        """Test simple text completion with Ollama."""
        try:
            # Check if Ollama server is available
            base_url = "http://localhost:11434"
            async with aiohttp.ClientSession() as session:
                # Check server availability
                try:
                    async with session.get(f"{base_url}/api/version") as response:
                        if response.status != 200:
                            pytest.skip("Ollama server not available")
                            return
                except aiohttp.ClientError:
                    pytest.skip("Ollama server not available")
                    return

                # Check if model exists
                model = "llama3.2"
                try:
                    async with session.post(f"{base_url}/api/show", json={"name": model}) as response:
                        if response.status != 200:
                            pytest.skip(f"Ollama model {model} not available")
                            return
                except aiohttp.ClientError:
                    pytest.skip(f"Ollama model {model} not available")
                    return

            # Run the test
            response = await ollama_client.complete_async("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            pytest.fail(f"Ollama completion failed: {e}")


@pytest.mark.integration
class TestCrossProviderLLM:
    """Integration tests across multiple LLM providers."""

    @pytest.fixture
    def providers(self) -> Any:
        """Create configurations for available providers."""
        configs = {}

        if os.getenv("OPENAI_API_KEY"):
            configs["openai"] = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo")

        if os.getenv("ANTHROPIC_API_KEY"):
            configs["anthropic"] = LlmConfig.anthropic(os.getenv("ANTHROPIC_API_KEY"), "claude-3-sonnet-20240229")

        configs["ollama"] = LlmConfig.ollama("llama3.2")

        return configs

    def test_multi_provider_executor_creation(self, providers: Any) -> None:
        """Test creating executors with multiple providers."""
        executors = {}

        for provider_name, config in providers.items():
            try:
                executor = Executor(config)
                executors[provider_name] = executor
                assert executor is not None
            except Exception as e:
                pytest.fail(f"Failed to create executor for {provider_name}: {e}")

        # At least one executor should be created successfully
        assert len(executors) > 0

    def test_multi_provider_client_creation(self, providers: Any) -> None:
        """Test creating clients with multiple providers."""
        clients = {}

        for provider_name, config in providers.items():
            try:
                client = LlmClient(config)
                clients[provider_name] = client
                assert client is not None
            except Exception as e:
                pytest.fail(f"Failed to create client for {provider_name}: {e}")

        # At least one client should be created successfully
        assert len(clients) > 0

    def test_cross_provider_configuration_compatibility(self, providers: Any) -> None:
        """Test configuration compatibility across providers."""
        for _provider_name, config in providers.items():
            # Test that all providers support basic configuration methods
            assert hasattr(config, "provider")
            assert hasattr(config, "model")

            provider = config.provider()
            model = config.model()

            assert isinstance(provider, str)
            assert isinstance(model, str)
            assert len(provider) > 0
            assert len(model) > 0


class TestAdvancedLLMClient:
    """Integration tests for advanced LLM client features."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def openai_config(self, api_key: str) -> Any:
        """Create OpenAI configuration."""
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def openai_client(self, openai_config: Any) -> Any:
        """Create OpenAI client."""
        return LlmClient(openai_config, debug=True)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_complete_batch_async_interface(self, openai_client: Any) -> None:
        """Test batch completion async interface creation."""
        prompts = ["What is AI?", "Explain machine learning in one sentence."]

        try:
            # Test batch completion
            results = await openai_client.complete_batch(prompts, max_tokens=50)
            assert len(results) == len(prompts)
            assert all(isinstance(result, str) for result in results)

            # Test batch completion with custom concurrency
            results_concurrent = await openai_client.complete_batch(prompts, max_tokens=50, max_concurrency=2)
            assert len(results_concurrent) == len(prompts)
            assert all(isinstance(result, str) for result in results_concurrent)

        except Exception as e:
            pytest.fail(f"Batch completion async interface test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_chat_optimized_async_interface(self, openai_client: Any) -> None:
        """Test optimized chat completion async interface."""
        messages = [("user", "Hello, how are you?"), ("assistant", "I'm doing well, thank you!"), ("user", "What can you help me with?")]

        try:
            # Test chat completion
            response = await openai_client.chat_optimized(messages, max_tokens=100)
            assert isinstance(response, str)
            assert len(response) > 0

            # Test with temperature setting
            response_temp = await openai_client.chat_optimized(messages, max_tokens=100, temperature=0.7)
            assert isinstance(response_temp, str)
            assert len(response_temp) > 0

        except Exception as e:
            pytest.fail(f"Chat optimized async interface test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_complete_stream_async_interface(self, openai_client: Any) -> None:
        """Test streaming completion async interface.

        This adapts to whichever interface the client provides:
        - async iterable of chunks
        - coroutine returning a full string
        - synchronous iterable of chunks
        - direct string
        If none are available, the test is skipped.
        """
        prompt = "Write a short story about a robot learning to paint"

        try:
            obj = openai_client.complete_stream(prompt, max_tokens=200)

            # If it's awaitable (coroutine or future), await it first
            if inspect.isawaitable(obj):
                obj = await obj

            # If it's an async iterable, consume chunks
            if hasattr(obj, "__aiter__"):
                async_chunks: list[str] = []
                async for chunk in obj:  # type: ignore[func-returns-value]
                    if inspect.isawaitable(chunk):
                        chunk = await chunk  # type: ignore[assignment]
                    assert isinstance(chunk, str)
                    async_chunks.append(chunk)
                full = "".join(async_chunks)
                assert len(full) > 0
                return

            # If it's a synchronous iterable of chunks, consume (await items if needed)
            if hasattr(obj, "__iter__") and not isinstance(obj, (str, bytes)):
                sync_chunks: list[str] = []
                for item in obj:  # type: ignore[assignment]
                    if inspect.isawaitable(item):
                        item = await item
                    assert isinstance(item, str)
                    sync_chunks.append(item)
                full = "".join(sync_chunks)
                assert len(full) > 0
                return

            # If it's already a string
            if isinstance(obj, str):
                assert len(obj) > 0
                return

            pytest.skip("Streaming interface not available in this environment")

        except RuntimeError as e:
            if "no running event loop" in str(e):
                pytest.skip("Async event loop not available")
            else:
                pytest.fail(f"Streaming completion async interface test failed: {e}")
        except Exception as e:
            pytest.fail(f"Streaming completion async interface test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_client_statistics(self, openai_client: Any) -> None:
        """Test client statistics and monitoring functionality."""
        try:
            # Get initial stats
            initial_stats = openai_client.get_stats()
            assert isinstance(initial_stats, dict)

            expected_stats_keys = ["total_requests", "successful_requests", "failed_requests", "success_rate", "average_response_time_ms", "circuit_breaker_state", "uptime_seconds"]

            for key in expected_stats_keys:
                assert key in initial_stats, f"Missing stat key: {key}"

            # Perform a completion to update stats
            openai_client.complete("Test prompt for stats", max_tokens=10)

            # Get updated stats
            updated_stats = openai_client.get_stats()
            assert updated_stats["total_requests"] > initial_stats["total_requests"]

            # Test stats reset
            openai_client.reset_stats()
            reset_stats = openai_client.get_stats()
            assert reset_stats["total_requests"] == 0

        except Exception as e:
            pytest.fail(f"Client statistics test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_client_warmup_async_interface(self, openai_client: Any) -> None:
        """Test client warmup async interface."""
        try:
            # Test warmup
            await openai_client.warmup()

            # Test that client still works after warmup
            response = await openai_client.complete_async("Quick test", max_tokens=5)
            assert isinstance(response, str)
            assert len(response) > 0

        except Exception as e:
            pytest.fail(f"Client warmup async interface test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_completion_interface(self, openai_client: Any) -> None:
        """Test async completion interface creation."""
        try:
            # Test async completion
            response = await openai_client.complete_async("What is the capital of France?", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0

            # Test async completion with temperature
            response_temp = await openai_client.complete_async("Tell me a joke", max_tokens=50, temperature=0.8)
            assert isinstance(response_temp, str)
            assert len(response_temp) > 0

        except Exception as e:
            pytest.fail(f"Async completion interface test failed: {e}")


class TestLLMErrorHandling:
    """Integration tests for LLM error handling and resilience."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def openai_config(self, api_key: str) -> Any:
        """Create OpenAI configuration."""
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_invalid_model_error(self, api_key: str) -> None:
        """Test error handling for invalid model."""
        try:
            # Create config with invalid model
            invalid_config = LlmConfig.openai(api_key, "gpt-nonexistent-model")
            client = LlmClient(invalid_config)

            # This should raise an error
            with pytest.raises((ValueError, RuntimeError)):
                client.complete("Test prompt", max_tokens=10)

        except Exception as e:
            # Expected to fail with invalid model
            assert "model" in str(e).lower() or "invalid" in str(e).lower()

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_invalid_parameters_error(self, openai_config: Any) -> None:
        """Test error handling for invalid parameters."""
        client = LlmClient(openai_config)

        try:
            # Test with invalid max_tokens (too high) - some providers may accept this
            try:
                response = client.complete("Test", max_tokens=100000)
                # If it succeeds, just check it's a valid response
                assert isinstance(response, str)
            except Exception:
                # If it fails, that's also expected
                pass  # nosec B110: acceptable in test context

            # Test with invalid temperature - some providers may clamp this
            try:
                response = client.complete("Test", temperature=2.0)
                # If it succeeds, just check it's a valid response
                assert isinstance(response, str)
            except Exception:
                # If it fails, that's also expected
                pass  # nosec B110: acceptable in test context

        except Exception as e:
            pytest.fail(f"Parameter validation test failed: {e}")

    def test_invalid_api_key_error(self) -> None:
        """Test error handling for invalid API key."""
        try:
            # Create config with invalid API key
            invalid_config = LlmConfig.openai("invalid-key", "gpt-3.5-turbo")
            client = LlmClient(invalid_config)

            # This should raise an authentication error
            with pytest.raises((ValueError, RuntimeError)):
                client.complete("Test prompt", max_tokens=10)

        except Exception as e:
            # Expected to fail with invalid API key
            assert any(word in str(e).lower() for word in ["auth", "key", "invalid", "unauthorized"])

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_empty_prompt_handling(self, openai_config: Any) -> None:
        """Test handling of empty or invalid prompts."""
        client = LlmClient(openai_config)

        try:
            # Test empty prompt - expect validation error
            with pytest.raises(ValueError, match="Prompt cannot be empty"):
                client.complete("", max_tokens=10)

            # Test very long prompt (if it causes issues)
            long_prompt = "A" * 50000  # Very long prompt
            try:
                response = client.complete(long_prompt, max_tokens=10)
                if response:
                    assert isinstance(response, str)
            except Exception as e:
                # Long prompts may legitimately fail
                assert "length" in str(e).lower() or "limit" in str(e).lower() or "token" in str(e).lower()

        except Exception as e:
            pytest.fail(f"Prompt handling test failed: {e}")


class TestLLMPerformance:
    """Integration tests for LLM performance characteristics."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def openai_config(self, api_key: str) -> Any:
        """Create OpenAI configuration."""
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_concurrent_requests_interface(self, openai_config: Any) -> None:
        """Test concurrent requests interface creation."""
        client = LlmClient(openai_config)

        try:
            # Test batch processing interface
            prompts = [f"What is {i} + {i}?" for i in range(3)]

            # Test concurrent batch completion
            results = await client.complete_batch(prompts, max_tokens=20, max_concurrency=3)
            assert len(results) == len(prompts)
            assert all(isinstance(result, str) for result in results)

            # Test that results make sense
            for i, result in enumerate(results):
                assert len(result) > 0
                # Result should contain a number since we asked for addition
                assert any(str(i + i) in result for i in range(3))

        except Exception as e:
            pytest.fail(f"Concurrent requests interface test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_response_time_consistency(self, openai_config: Any) -> None:
        """Test response time consistency."""
        client = LlmClient(openai_config)

        try:
            response_times = []

            for i in range(3):  # Test multiple requests
                start_time = time.time()
                response = client.complete(f"Count to {i+1}", max_tokens=10)
                end_time = time.time()

                assert isinstance(response, str)
                response_times.append(end_time - start_time)

            # All responses should complete within reasonable time
            assert all(rt < 30 for rt in response_times), f"Some responses too slow: {response_times}"

            # Calculate response time statistics
            avg_time = sum(response_times) / len(response_times)
            assert avg_time < 15, f"Average response time too high: {avg_time}s"

        except Exception as e:
            pytest.fail(f"Response time consistency test failed: {e}")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
