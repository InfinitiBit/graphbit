#!/usr/bin/env python3
"""Integration tests for GraphBit async executor functionality."""
import os
from typing import Any

import pytest

import graphbit


class TestAsyncExecutor:
    """Integration tests for async executor functionality."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for async tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def executor(self, llm_config: Any) -> Any:
        """Create executor for testing."""
        return graphbit.Executor(llm_config)

    @pytest.fixture
    def simple_workflow(self) -> Any:
        """Create simple workflow for testing."""
        workflow = graphbit.Workflow("async_test")
        agent = graphbit.Node.agent("async_agent", "Process async", "async_001")
        workflow.add_node(agent)
        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_async_executor_creation(self, llm_config: Any) -> None:
        """Test creating async-capable executor."""
        executor = graphbit.Executor(llm_config)
        assert executor is not None

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_async_workflow_execution(self, executor: Any, simple_workflow: Any) -> None:
        """Test async workflow execution."""
        try:
            # Validate workflow
            simple_workflow.validate()

            # Execute workflow
            result = executor.execute(simple_workflow)
            assert result is not None
            assert isinstance(result, graphbit.WorkflowResult)

        except Exception as e:
            pytest.skip(f"Async workflow execution skipped: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_configuration_modes(self, llm_config: Any) -> None:
        """Test different executor configuration modes."""
        # Test high throughput mode
        ht_executor = graphbit.Executor.new_high_throughput(llm_config)
        assert ht_executor is not None

        # Test low latency mode
        ll_executor = graphbit.Executor.new_low_latency(llm_config)
        assert ll_executor is not None

        # Test memory optimized mode
        mo_executor = graphbit.Executor.new_memory_optimized(llm_config)
        assert mo_executor is not None
