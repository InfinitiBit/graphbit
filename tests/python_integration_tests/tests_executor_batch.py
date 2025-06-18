"""Tests for batch and concurrent agent task execution via PyWorkflowExecutor."""

import os

import pytest

import graphbit


class TestExecutorBatch:
    """Tests the execution of multiple workflows and parallel agent tasks."""

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_execute_batch(self):
        """Should execute multiple workflows in a batch and return their contexts."""
        llm = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo")

        workflow1 = graphbit.PyWorkflow("batch1", "Batch test 1")
        node1 = graphbit.PyWorkflowNode.agent_node("node1", "desc", "agent_001", "prompt")
        workflow1.add_node(node1)

        workflow2 = graphbit.PyWorkflow("batch2", "Batch test 2")
        node2 = graphbit.PyWorkflowNode.agent_node("node2", "desc", "agent_001", "prompt")
        workflow2.add_node(node2)

        executor = graphbit.PyWorkflowExecutor(llm)
        results = executor.execute_batch([workflow1, workflow2])

        assert len(results) == 2
        for ctx in results:
            assert ctx.is_completed() or ctx.is_failed()

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_execute_concurrent_agent_tasks(self):
        """Should execute prompts concurrently using agent task interface."""
        llm = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo")
        executor = graphbit.PyWorkflowExecutor.new_high_throughput(llm)

        prompts = ["Say hi", "Say bye"]
        results = executor.execute_concurrent_agent_tasks(prompts, "agent_001")

        assert len(results) == 2
        assert all(isinstance(r, str) for r in results)
