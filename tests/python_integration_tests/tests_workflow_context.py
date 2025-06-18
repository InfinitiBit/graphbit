"""Tests properties of PyWorkflowContext after workflow execution."""

import os

import pytest

import graphbit


class TestWorkflowContext:
    """Verifies accessors in the PyWorkflowContext object."""

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_workflow_context_accessors(self):
        """Should expose workflow ID, execution state, variables, and timing."""
        llm = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo")
        workflow = graphbit.PyWorkflow("context_test", "Test context access")

        node = graphbit.PyWorkflowNode.agent_node("n1", "desc", "agent_001", "Prompt")
        workflow.add_node(node)

        executor = graphbit.PyWorkflowExecutor(llm)
        result = executor.execute(workflow)

        # Access various workflow context fields
        assert isinstance(result.workflow_id(), str)
        assert isinstance(result.state(), str)
        assert isinstance(result.execution_time_ms(), int)
        assert isinstance(result.variables(), list)
