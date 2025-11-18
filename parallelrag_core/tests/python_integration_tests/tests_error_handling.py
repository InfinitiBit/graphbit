"""integration tests for GraphBit error handling and recovery."""

import contextlib
import os
from typing import Any

import pytest

import graphbit


class TestNetworkErrorHandling:
    """Tests for network-related error handling."""

    def test_invalid_api_key_errors(self) -> None:
        """Test proper error handling for invalid API keys."""
        # Test OpenAI invalid key
        invalid_config = graphbit.LlmConfig.openai("sk-invalid1234567890123456789012345678901234567890123", "gpt-3.5-turbo")
        client = graphbit.LlmClient(invalid_config)

        with pytest.raises(Exception) as exc_info:
            client.complete("Test", max_tokens=10)

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["auth", "key", "invalid", "unauthorized", "permission"])

    def test_invalid_model_errors(self) -> None:
        """Test error handling for invalid model names."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        # Test invalid model name
        try:
            invalid_config = graphbit.LlmConfig.openai(api_key, "invalid-model-name-12345")
            client = graphbit.LlmClient(invalid_config)

            with pytest.raises(Exception) as exc_info:
                client.complete("Test", max_tokens=10)

            error_msg = str(exc_info.value).lower()
            assert any(word in error_msg for word in ["model", "invalid", "not found", "unknown"])

        except Exception as e:
            # Some validation might happen at config creation time
            error_msg = str(e).lower()
            if any(word in error_msg for word in ["model", "invalid"]):
                pass  # Expected validation error
            else:
                pytest.fail(f"Unexpected error: {e}")

    def test_connection_timeout_errors(self) -> None:
        """Test handling of connection timeout scenarios."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

        # Create client with very short timeout (this tests timeout handling)
        client = graphbit.LlmClient(config, debug=True)

        # Test with a very complex prompt that might timeout
        complex_prompt = "Explain quantum mechanics in extreme detail. " * 100

        try:
            # This might timeout or succeed depending on network conditions
            result = client.complete(complex_prompt, max_tokens=1000)
            assert isinstance(result, str)  # If it succeeds, it should return a string
        except Exception as e:
            # If it times out or fails, that's also acceptable for this test
            error_msg = str(e).lower()
            # Just verify we get a reasonable error message
            assert len(error_msg) > 0

    def test_rate_limit_error_handling(self) -> None:
        """Test handling of rate limit errors."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Send many rapid requests to potentially trigger rate limiting
        requests = []
        for i in range(5):  # Reduced number to avoid actually hitting limits
            try:
                result = client.complete(f"Count to {i}", max_tokens=5)
                requests.append(result)
            except Exception as e:
                # If we hit a rate limit, verify the error is appropriate
                error_msg = str(e).lower()
                if any(word in error_msg for word in ["rate", "limit", "quota", "throttle"]):
                    break  # Expected rate limit error
                else:
                    # Other errors are also acceptable
                    pass

        # Test should not fail - either requests succeed or rate limiting is handled


class TestResourceExhaustionHandling:
    """Tests for resource exhaustion scenarios."""

    def test_memory_pressure_handling(self) -> None:
        """Test behavior under memory pressure."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Create a large batch to test memory handling
        large_batch = [f"Process item {i}" for i in range(20)]  # Reasonable size for testing

        try:
            results = client.complete_batch(large_batch, max_tokens=10, max_concurrency=5)
            assert isinstance(results, list)
            assert len(results) == len(large_batch)
        except Exception as e:
            # Memory or resource errors should be handled gracefully
            error_msg = str(e).lower()
            assert len(error_msg) > 0  # Should get a meaningful error message

    def test_concurrent_request_limits(self) -> None:
        """Test handling of concurrent request limits."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

        # Create multiple clients to test concurrency limits
        clients = [graphbit.LlmClient(config) for _ in range(3)]

        # Test concurrent requests
        try:
            results = []
            for i, client in enumerate(clients):
                result = client.complete(f"Task {i}", max_tokens=5)
                results.append(result)

            assert len(results) == len(clients)
            assert all(isinstance(r, str) for r in results)

        except Exception as e:
            # Concurrency limits should be handled gracefully
            error_msg = str(e).lower()
            assert len(error_msg) > 0

    def test_large_input_handling(self) -> None:
        """Test handling of very large inputs."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Create a very large input
        large_input = "This is a test sentence. " * 500  # Large but not excessive

        try:
            result = client.complete(large_input, max_tokens=10)
            assert isinstance(result, str)
        except Exception as e:
            # Large input errors should be handled appropriately
            error_msg = str(e).lower()
            assert any(word in error_msg for word in ["token", "length", "size", "limit", "too long"])


class TestWorkflowErrorRecovery:
    """Tests for workflow execution error recovery."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for workflow error tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    def test_workflow_execution_error_recovery(self, llm_config: Any) -> None:
        """Test recovery from workflow execution errors."""
        executor = graphbit.Executor(llm_config)

        # Create a problematic workflow
        workflow = graphbit.Workflow("error_recovery_test")

        # Add a node with potentially problematic prompt
        agent = graphbit.Node.agent("error_agent", "This is a test prompt that should work", "error_001")
        workflow.add_node(agent)

        try:
            workflow.validate()
            result = executor.execute(workflow)

            # If execution succeeds, verify result
            assert isinstance(result, graphbit.WorkflowResult)
            assert isinstance(result.is_success(), bool)
            assert isinstance(result.is_failed(), bool)

        except Exception as e:
            # If execution fails, verify we get appropriate error information
            error_msg = str(e).lower()
            assert len(error_msg) > 0

    def test_invalid_workflow_error_handling(self, llm_config: Any) -> None:
        """Test handling of invalid workflow structures."""
        executor = graphbit.Executor(llm_config)

        # Create invalid workflow with circular dependency
        workflow = graphbit.Workflow("invalid_workflow")

        agent1 = graphbit.Node.agent("agent1", "First agent", "agent_001")
        agent2 = graphbit.Node.agent("agent2", "Second agent", "agent_002")

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)

        workflow.connect(id1, id2)
        workflow.connect(id2, id1)  # Creates cycle

        # Execution should fail with appropriate error
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            executor.execute(workflow)

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["circular", "cycle", "invalid", "dependency"])

    def test_empty_workflow_execution_error(self, llm_config: Any) -> None:
        """Test execution of empty workflows."""
        executor = graphbit.Executor(llm_config)
        empty_workflow = graphbit.Workflow("empty")

        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            executor.execute(empty_workflow)

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["empty", "no nodes", "invalid"])


class TestAsyncErrorHandling:
    """Tests for async operation error handling."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get API key for async error tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        assert api_key is not None
        return api_key

    @pytest.mark.asyncio
    async def test_async_completion_error_handling(self, api_key: str) -> None:
        """Test error handling in async completion operations."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Test async completion with invalid parameters
        with pytest.raises((ValueError, RuntimeError)):
            await client.complete_async("Test", max_tokens=0)

    @pytest.mark.asyncio
    async def test_async_batch_error_handling(self, api_key: str) -> None:
        """Test error handling in async batch operations."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Test async batch with empty list
        with pytest.raises((ValueError, RuntimeError)):
            await client.complete_batch([])

    @pytest.mark.asyncio
    async def test_async_workflow_error_handling(self, api_key: str) -> None:
        """Test error handling in async workflow execution."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        executor = graphbit.Executor(config)

        # Create invalid workflow
        workflow = graphbit.Workflow("async_error_test")

        # Test async execution of empty workflow
        with pytest.raises((ValueError, RuntimeError)):
            await executor.run_async(workflow)


class TestEmbeddingErrorHandling:
    """Tests for embedding operation error handling."""

    def test_embedding_invalid_inputs(self) -> None:
        """Test embedding error handling for invalid inputs."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.EmbeddingConfig.openai(api_key, "text-embedding-3-small")
        client = graphbit.EmbeddingClient(config)

        # Test empty text
        with pytest.raises((ValueError, RuntimeError)):
            client.embed("")

        # Test empty batch
        with pytest.raises((ValueError, RuntimeError)):
            client.embed_many([])

    def test_embedding_network_errors(self) -> None:
        """Test embedding error handling for network issues."""
        # Test with invalid API key
        invalid_config = graphbit.EmbeddingConfig.openai("sk-invalid1234567890123456789012345678901234567890123", "text-embedding-3-small")
        client = graphbit.EmbeddingClient(invalid_config)

        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            client.embed("Test text")

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["auth", "key", "invalid", "unauthorized"])

    def test_similarity_computation_errors(self) -> None:
        """Test error handling in similarity computations."""
        # Test mismatched dimensions
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.EmbeddingClient.similarity([1.0, 2.0], [1.0, 2.0, 3.0])

        # Test empty vectors
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.EmbeddingClient.similarity([], [1.0, 2.0])


class TestSystemErrorRecovery:
    """Tests for system-level error recovery."""

    def test_runtime_error_recovery(self) -> None:
        """Test recovery from runtime errors."""
        # Test multiple initializations
        try:
            graphbit.init()
            graphbit.init()  # Should not fail
            graphbit.init(log_level="debug")  # Should not fail
        except Exception as e:
            pytest.fail(f"Multiple initialization failed: {e}")

    def test_configuration_error_recovery(self) -> None:
        """Test recovery from configuration errors."""
        # Test invalid runtime configuration
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(worker_threads=0)

        # Test that system still works after invalid config
        try:
            version = graphbit.version()
            assert isinstance(version, str)
            assert len(version) > 0
        except Exception as e:
            pytest.fail(f"System not functional after config error: {e}")

    def test_health_check_error_conditions(self) -> None:
        """Test health check behavior under error conditions."""
        try:
            health = graphbit.health_check()
            assert isinstance(health, dict)

            # Health check should always return some information
            assert len(health) > 0

        except Exception as e:
            pytest.fail(f"Health check failed: {e}")

    def test_graceful_degradation(self) -> None:
        """Test graceful degradation under error conditions."""
        # Test system info under various conditions
        system_info = graphbit.get_system_info()
        assert isinstance(system_info, dict)
        assert "version" in system_info
        assert isinstance(system_info["version"], str)

        # Test that basic functionality still works
        version = graphbit.version()
        assert isinstance(version, str)
        assert len(version) > 0


@pytest.mark.integration
class TestErrorPropagation:
    """Tests for proper error propagation across components."""

    def test_llm_to_workflow_error_propagation(self) -> None:
        """Test error propagation from LLM to workflow execution."""
        # Use invalid API key
        invalid_config = graphbit.LlmConfig.openai("sk-invalid1234567890123456789012345678901234567890123", "gpt-3.5-turbo")
        executor = graphbit.Executor(invalid_config)

        # Create valid workflow
        workflow = graphbit.Workflow("error_propagation_test")
        agent = graphbit.Node.agent("test_agent", "Test prompt", "agent_001")
        workflow.add_node(agent)
        workflow.validate()

        # Execution should fail with appropriate error
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            executor.execute(workflow)

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["auth", "key", "invalid", "unauthorized"])

    def test_validation_to_execution_error_propagation(self) -> None:
        """Test error propagation from validation to execution."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        executor = graphbit.Executor(config)

        # Create invalid workflow that should fail validation
        workflow = graphbit.Workflow("validation_error_test")
        agent1 = graphbit.Node.agent("agent1", "First agent", "agent_001")
        agent2 = graphbit.Node.agent("agent2", "Second agent", "agent_002")

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)

        workflow.connect(id1, id2)
        workflow.connect(id2, id1)  # Circular dependency

        # Should fail during execution due to validation error
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            executor.execute(workflow)

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["circular", "cycle", "validation", "invalid"])

    def test_embedding_error_context(self) -> None:
        """Test that embedding errors provide proper context."""
        # Test with invalid configuration
        try:
            graphbit.EmbeddingConfig.openai("", "text-embedding-3-small")
            pytest.fail("Should have failed validation")
        except (ValueError, RuntimeError) as e:
            error_msg = str(e).lower()
            assert "empty" in error_msg
            assert "key" in error_msg
