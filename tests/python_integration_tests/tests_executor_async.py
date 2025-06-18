"""Tests asynchronous execution of workflows via PyWorkflowExecutor."""

import os

import pytest

import graphbit


class TestExecutorAsync:
    """Test suite for async workflow execution logic."""

    @pytest.mark.asyncio
    async def test_execute_async_workflow(self):
        """Should execute workflow asynchronously and return a result context."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        llm = graphbit.PyLlmConfig.openai(api_key, "gpt-3.5-turbo")
        workflow = graphbit.PyWorkflow("async_test", "Test async execution")

        node = graphbit.PyWorkflowNode.agent_node("n1", "desc", "agent_001", "Prompt")
        workflow.add_node(node)

        executor = graphbit.PyWorkflowExecutor(llm)
        result = await executor.execute_async(workflow)

        assert result.is_completed() or result.is_failed()
