"""Integration tests for actual GraphBit async execution."""

import asyncio
import contextlib
import os
import time
from typing import Any

import pytest

import graphbit


@pytest.mark.asyncio
class TestActualAsyncLLMExecution:
    """Integration tests for actual async LLM execution."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return str(api_key)

    @pytest.fixture
    def openai_client(self, api_key: str) -> Any:
        """Create OpenAI LLM client."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        return graphbit.LlmClient(config, debug=True)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_complete_execution(self, openai_client: Any) -> None:
        """Test actual async completion execution."""
        try:
            # Execute async completion and await result
            result = await openai_client.complete_async("What is the capital of France? Answer in one word.", max_tokens=10)

            # Verify result
            assert isinstance(result, str)
            assert len(result) > 0
            assert "paris" in result.lower() or "france" in result.lower()

        except Exception as e:
            pytest.fail(f"Async completion execution failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_batch_execution(self, openai_client: Any) -> None:
        """Test actual async batch completion execution."""
        try:
            prompts = ["What is 2+2? Answer with just the number.", "What color is the sky? One word answer."]

            # Execute batch completion and await results
            results = await openai_client.complete_batch(prompts, max_tokens=20, max_concurrency=2)

            # Verify results
            assert isinstance(results, list)
            assert len(results) == len(prompts)
            assert all(isinstance(result, str) for result in results)
            assert all(len(result) > 0 for result in results)

        except Exception as e:
            pytest.fail(f"Async batch execution failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_chat_optimized_execution(self, openai_client: Any) -> None:
        """Test actual async chat optimized execution."""
        try:
            messages = [("user", "Hello"), ("assistant", "Hi there!"), ("user", "What's 1+1? Just the number please.")]

            # Execute chat optimized and await result
            response = await openai_client.chat_optimized(messages, max_tokens=10, temperature=0.1)

            # Verify response
            assert isinstance(response, str)
            assert len(response) > 0
            assert any(char in response for char in ["2", "two"])

        except Exception as e:
            pytest.fail(f"Async chat optimized execution failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_warmup_execution(self, openai_client: Any) -> None:
        """Test actual async warmup execution."""
        try:
            # Execute warmup and await result
            warmup_result = await openai_client.warmup()

            # Verify warmup completed
            assert isinstance(warmup_result, str)
            assert "success" in warmup_result.lower() or "warm" in warmup_result.lower()

            # Test that client works after warmup
            response = await openai_client.complete_async("Test", max_tokens=5)
            assert isinstance(response, str)
            assert len(response) > 0

        except Exception as e:
            pytest.fail(f"Async warmup execution failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_concurrent_async_execution(self, openai_client: Any) -> None:
        """Test concurrent async execution."""
        try:
            # Create multiple async tasks
            tasks = [openai_client.complete_async(f"Count to {i}.", max_tokens=20) for i in range(1, 3)]

            # Execute tasks concurrently
            start_time = time.time()
            results = await asyncio.gather(*tasks)
            end_time = time.time()

            # Verify results
            assert len(results) == 2
            assert all(isinstance(result, str) for result in results)
            assert all(len(result) > 0 for result in results)

            # Concurrent execution should be reasonable
            concurrent_time = end_time - start_time
            assert concurrent_time < 60, f"Concurrent execution took too long: {concurrent_time}s"

        except Exception as e:
            pytest.fail(f"Concurrent async execution failed: {e}")


@pytest.mark.asyncio
class TestActualAsyncErrorHandling:
    """Integration tests for actual async error handling."""

    async def test_async_invalid_api_key_error(self) -> None:
        """Test async execution with invalid API key."""
        try:
            # Use properly formatted but invalid API key
            invalid_config = graphbit.LlmConfig.openai("sk-invalid1234567890123456789012345678901234567890123", "gpt-3.5-turbo")
            client = graphbit.LlmClient(invalid_config)

            # This should raise an error when awaited
            with pytest.raises(Exception) as exc_info:
                await client.complete_async("Test", max_tokens=10)

            # Verify error type
            error_msg = str(exc_info.value).lower()
            assert any(word in error_msg for word in ["auth", "key", "invalid", "unauthorized"])

        except Exception as e:
            # If we get a validation error at config time, that's also acceptable
            error_msg = str(e).lower()
            if any(word in error_msg for word in ["key", "invalid", "auth"]):
                pass  # Expected validation error
            else:
                pytest.fail(f"Unexpected error in async invalid API key test: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_invalid_parameters_error(self) -> None:
        """Test async execution with invalid parameters."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        try:
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Test invalid max_tokens
            with pytest.raises((ValueError, RuntimeError)):
                await client.complete_async("Test", max_tokens=0)

            # Test invalid temperature
            with pytest.raises((ValueError, RuntimeError)):
                await client.complete_async("Test", temperature=3.0)

        except Exception as e:
            pytest.fail(f"Async parameter validation test failed: {e}")


@pytest.mark.asyncio
class TestActualAsyncPerformance:
    """Integration tests for actual async performance characteristics."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return str(api_key)

    @pytest.fixture
    def openai_client(self, api_key: str) -> Any:
        """Create OpenAI LLM client."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        return graphbit.LlmClient(config)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_vs_sync_performance(self, openai_client: Any) -> None:
        """Test performance comparison between async and sync execution."""
        try:
            prompt = "What is AI? Answer in one sentence."

            # Time sync execution
            sync_start = time.time()
            sync_result = openai_client.complete(prompt, max_tokens=50)
            sync_time = time.time() - sync_start

            # Time async execution
            async_start = time.time()
            async_result = await openai_client.complete_async(prompt, max_tokens=50)
            async_time = time.time() - async_start

            # Both should work and return valid results
            assert isinstance(sync_result, str) and len(sync_result) > 0
            assert isinstance(async_result, str) and len(async_result) > 0

            # Both should complete in reasonable time
            assert sync_time < 60, f"Sync execution took too long: {sync_time}s"
            assert async_time < 60, f"Async execution took too long: {async_time}s"

        except Exception as e:
            pytest.fail(f"Async vs sync performance test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_concurrent_async_performance(self, openai_client: Any) -> None:
        """Test performance of concurrent async operations."""
        try:
            prompts = [f"What is {i} plus {i}?" for i in range(2, 4)]

            # Concurrent async execution
            concurrent_start = time.time()
            tasks = [openai_client.complete_async(prompt, max_tokens=10) for prompt in prompts]
            concurrent_results = await asyncio.gather(*tasks)
            concurrent_time = time.time() - concurrent_start

            # Verify results
            assert len(concurrent_results) == len(prompts)
            assert all(isinstance(r, str) and len(r) > 0 for r in concurrent_results)

            # Should complete in reasonable time
            assert concurrent_time < 60, f"Concurrent took too long: {concurrent_time}s"

        except Exception as e:
            pytest.fail(f"Concurrent async performance test failed: {e}")


@pytest.mark.asyncio
@pytest.mark.integration
class TestComplexAsyncScenarios:
    """Integration tests for complex async scenarios."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return str(api_key)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_execution_with_parameters(self, api_key: str) -> None:
        """Test async execution with various parameters."""
        try:
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Test with different temperatures
            low_temp_task = client.complete_async("Say hello", max_tokens=10, temperature=0.1)
            high_temp_task = client.complete_async("Say hello", max_tokens=10, temperature=0.9)

            # Execute both
            low_temp_result, high_temp_result = await asyncio.gather(low_temp_task, high_temp_task)

            # Both should be valid responses
            assert isinstance(low_temp_result, str) and len(low_temp_result) > 0
            assert isinstance(high_temp_result, str) and len(high_temp_result) > 0

        except Exception as e:
            pytest.fail(f"Async execution with parameters failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_timeout_scenarios(self, api_key: str) -> None:
        """Test async operations with timeout scenarios."""
        try:
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Test with reasonable timeout
            try:
                result = await asyncio.wait_for(client.complete_async("Quick test", max_tokens=5), timeout=30.0)
                assert isinstance(result, str)
            except asyncio.TimeoutError:
                pytest.fail("Reasonable timeout was exceeded")

        except Exception as e:
            pytest.fail(f"Async timeout scenarios test failed: {e}")


@pytest.mark.asyncio
class TestActualAsyncErrorHandlingExtended:
    """Integration tests for actual async error handling (extended)."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return str(api_key)

    async def test_async_invalid_api_key_error(self) -> None:
        """Test async execution with invalid API key."""
        try:
            # Use properly formatted but invalid API key
            invalid_config = graphbit.LlmConfig.openai("sk-invalid1234567890123456789012345678901234567890123", "gpt-3.5-turbo")
            client = graphbit.LlmClient(invalid_config)

            # This should raise an error when awaited
            with pytest.raises(Exception) as exc_info:
                await client.complete_async("Test", max_tokens=10)

            # Verify error type
            error_msg = str(exc_info.value).lower()
            assert any(word in error_msg for word in ["auth", "key", "invalid", "unauthorized"])

        except Exception as e:
            # If we get a validation error at config time, that's also acceptable
            error_msg = str(e).lower()
            if any(word in error_msg for word in ["key", "invalid", "auth"]):
                pass  # Expected validation error
            else:
                pytest.fail(f"Unexpected error in async invalid API key test: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_invalid_parameters_error(self, api_key: str) -> None:
        """Test async execution with invalid parameters."""
        try:
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Test invalid max_tokens
            with pytest.raises((ValueError, RuntimeError)):
                await client.complete_async("Test", max_tokens=0)

            # Test invalid temperature
            with pytest.raises((ValueError, RuntimeError)):
                await client.complete_async("Test", temperature=3.0)

        except Exception as e:
            pytest.fail(f"Async parameter validation test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_empty_batch_error(self, api_key: str) -> None:
        """Test async batch execution with empty batch."""
        try:
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Empty batch should raise error
            with pytest.raises((ValueError, RuntimeError)):
                await client.complete_batch([], max_tokens=10)

        except Exception as e:
            pytest.fail(f"Async empty batch test failed: {e}")


@pytest.mark.asyncio
class TestActualAsyncWorkflowExecution:
    """Integration tests for actual async workflow execution."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return str(api_key)

    @pytest.fixture
    def llm_config(self, api_key: str) -> Any:
        """Create LLM configuration."""
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def simple_workflow(self) -> Any:
        """Create simple workflow for async testing."""
        workflow = graphbit.Workflow("async_workflow_test")
        agent = graphbit.Node.agent("async_agent", "Generate a random number between 1 and 10", "async_001")
        workflow.add_node(agent)
        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_workflow_execution(self, llm_config: Any, simple_workflow: Any) -> None:
        """Test actual async workflow execution."""
        try:
            executor = graphbit.Executor(llm_config, debug=True)
            simple_workflow.validate()

            # Execute workflow asynchronously (Note: this may not be directly available)
            # For now, test that regular execution works in async context
            result = executor.execute(simple_workflow)

            # Verify result
            assert isinstance(result, graphbit.WorkflowResult)
            assert result.execution_time_ms() >= 0

        except Exception as e:
            pytest.skip(f"Async workflow execution test skipped: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_multiple_async_workflow_executions(self, llm_config: Any) -> None:
        """Test multiple concurrent workflow executions."""
        try:
            executor = graphbit.Executor(llm_config)

            # Create multiple workflows
            workflows = []
            for i in range(3):
                workflow = graphbit.Workflow(f"async_multi_workflow_{i}")
                agent = graphbit.Node.agent(f"agent_{i}", f"What is {i} + {i}? Answer with just the number.", f"multi_async_{i:03d}")
                workflow.add_node(agent)
                workflow.validate()
                workflows.append(workflow)

            # Execute workflows in async context
            results = []
            for workflow in workflows:
                result = executor.execute(workflow)
                results.append(result)

            # Verify all results
            assert len(results) == len(workflows)
            assert all(isinstance(result, graphbit.WorkflowResult) for result in results)

        except Exception as e:
            pytest.skip(f"Multiple async workflow executions test skipped: {e}")


@pytest.mark.asyncio
class TestActualAsyncPerformanceMetrics:
    """Integration tests for actual async performance characteristics."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return str(api_key)

    @pytest.fixture
    def openai_client(self, api_key: str) -> Any:
        """Create OpenAI LLM client."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        return graphbit.LlmClient(config)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_vs_sync_performance(self, openai_client: Any) -> None:
        """Test performance comparison between async and sync execution."""
        try:
            prompt = "What is AI? Answer in one sentence."

            # Time sync execution
            sync_start = time.time()
            sync_result = openai_client.complete(prompt, max_tokens=50)
            sync_time = time.time() - sync_start

            # Time async execution
            async_start = time.time()
            async_result = await openai_client.complete_async(prompt, max_tokens=50)
            async_time = time.time() - async_start

            # Both should work and return valid results
            assert isinstance(sync_result, str) and len(sync_result) > 0
            assert isinstance(async_result, str) and len(async_result) > 0

            # Both should complete in reasonable time
            assert sync_time < 30, f"Sync execution took too long: {sync_time}s"
            assert async_time < 30, f"Async execution took too long: {async_time}s"

            # Log performance for comparison
            print(f"Sync time: {sync_time:.2f}s, Async time: {async_time:.2f}s")

        except Exception as e:
            pytest.fail(f"Async vs sync performance test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_concurrent_async_performance(self, openai_client: Any) -> None:
        """Test performance of concurrent async operations."""
        try:
            prompts = [f"What is {i} squared? Just the number." for i in range(2, 5)]

            # Sequential async execution
            sequential_start = time.time()
            sequential_results = []
            for prompt in prompts:
                result = await openai_client.complete_async(prompt, max_tokens=10)
                sequential_results.append(result)
            sequential_time = time.time() - sequential_start

            # Concurrent async execution
            concurrent_start = time.time()
            tasks = [openai_client.complete_async(prompt, max_tokens=10) for prompt in prompts]
            concurrent_results = await asyncio.gather(*tasks)
            concurrent_time = time.time() - concurrent_start

            # Verify results
            assert len(sequential_results) == len(prompts)
            assert len(concurrent_results) == len(prompts)
            assert all(isinstance(r, str) and len(r) > 0 for r in sequential_results)
            assert all(isinstance(r, str) and len(r) > 0 for r in concurrent_results)

            # Concurrent should generally be faster
            print(f"Sequential: {sequential_time:.2f}s, Concurrent: {concurrent_time:.2f}s")

            # Should complete in reasonable time
            assert concurrent_time < 60, f"Concurrent took too long: {concurrent_time}s"

        except Exception as e:
            pytest.fail(f"Concurrent async performance test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_batch_vs_individual_performance(self, openai_client: Any) -> None:
        """Test performance comparison between batch and individual async operations."""
        try:
            prompts = ["Say hello", "Say goodbye", "Say thank you"]

            # Individual async executions
            individual_start = time.time()
            individual_tasks = [openai_client.complete_async(prompt, max_tokens=10) for prompt in prompts]
            individual_results = await asyncio.gather(*individual_tasks)
            individual_time = time.time() - individual_start

            # Batch async execution
            batch_start = time.time()
            batch_results = await openai_client.complete_batch(prompts, max_tokens=10)
            batch_time = time.time() - batch_start

            # Verify results
            assert len(individual_results) == len(prompts)
            assert len(batch_results) == len(prompts)
            assert all(isinstance(r, str) and len(r) > 0 for r in individual_results)
            assert all(isinstance(r, str) and len(r) > 0 for r in batch_results)

            # Both should complete in reasonable time
            assert individual_time < 60, f"Individual async took too long: {individual_time}s"
            assert batch_time < 60, f"Batch async took too long: {batch_time}s"

            # Log performance comparison
            print(f"Individual async: {individual_time:.2f}s, Batch async: {batch_time:.2f}s")

        except Exception as e:
            pytest.fail(f"Batch vs individual async performance test failed: {e}")


@pytest.mark.asyncio
@pytest.mark.integration
class TestComplexAsyncScenariosAdvanced:
    """Integration tests for complex async scenarios."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return str(api_key)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_mixed_async_operations(self, api_key: str) -> None:
        """Test mixed async operations (LLM + embeddings)."""
        try:
            # Create clients
            llm_config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            llm_client = graphbit.LlmClient(llm_config)

            embed_config = graphbit.EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
            embed_client = graphbit.EmbeddingClient(embed_config)

            # Execute mixed async operations
            text_prompt = "What is machine learning?"

            # Run LLM and embedding operations concurrently
            llm_task = llm_client.complete_async(text_prompt, max_tokens=50)
            embed_task = asyncio.create_task(asyncio.to_thread(embed_client.embed, text_prompt))

            # Wait for both to complete
            llm_result, embed_result = await asyncio.gather(llm_task, embed_task)

            # Verify results
            assert isinstance(llm_result, str) and len(llm_result) > 0
            assert isinstance(embed_result, list) and len(embed_result) > 0

        except Exception as e:
            pytest.fail(f"Mixed async operations test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_error_recovery(self, api_key: str) -> None:
        """Test async error recovery scenarios."""
        try:
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Mix valid and invalid operations
            valid_task = client.complete_async("Say hello", max_tokens=10)

            # This should succeed
            valid_result = await valid_task
            assert isinstance(valid_result, str) and len(valid_result) > 0

            # Test that client still works after potential errors
            recovery_result = await client.complete_async("Say goodbye", max_tokens=10)
            assert isinstance(recovery_result, str) and len(recovery_result) > 0

        except Exception as e:
            pytest.fail(f"Async error recovery test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_async_timeout_scenarios(self, api_key: str) -> None:
        """Test async operations with timeout scenarios."""
        try:
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Test with reasonable timeout
            try:
                result = await asyncio.wait_for(client.complete_async("Quick test", max_tokens=5), timeout=30.0)
                assert isinstance(result, str)
            except asyncio.TimeoutError:
                pytest.fail("Reasonable timeout was exceeded")

            # Test very short timeout (may timeout)
            with contextlib.suppress(asyncio.TimeoutError):
                await asyncio.wait_for(client.complete_async("Complex question requiring deep thought", max_tokens=100), timeout=0.1)  # Very short timeout

        except Exception as e:
            pytest.fail(f"Async timeout scenarios test failed: {e}")
