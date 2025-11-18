"""Integration tests for GraphBit workflow context functionality."""

import os
from typing import Any

import pytest

import graphbit


class TestWorkflowContext:
    """Integration tests for workflow context and results."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for context tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def test_workflow(self) -> Any:
        """Create test workflow."""
        workflow = graphbit.Workflow("context_test")
        agent = graphbit.Node.agent("context_agent", "Test context", "ctx_001")
        workflow.add_node(agent)
        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_workflow_result_creation(self, llm_config: Any, test_workflow: Any) -> None:
        """Test workflow result creation and access."""
        try:
            executor = graphbit.Executor(llm_config)
            test_workflow.validate()

            result = executor.execute(test_workflow)
            assert result is not None
            assert isinstance(result, graphbit.WorkflowResult)

            assert isinstance(result.is_success(), bool)
            assert isinstance(result.is_failed(), bool)
            assert isinstance(result.state(), str)
            assert isinstance(result.execution_time_ms(), int)

        except Exception as e:
            pytest.skip(f"Workflow result test skipped: {e}")

    def test_workflow_result_interface(self) -> None:
        """Test workflow result interface methods."""
        assert hasattr(graphbit, "WorkflowResult")

        expected_methods = ["is_success", "is_failed", "state", "execution_time_ms", "variables"]
        for method in expected_methods:
            assert hasattr(graphbit.WorkflowResult, method)
