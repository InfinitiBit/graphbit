"""Tests validation behavior for workflows with structural errors."""

import pytest

import graphbit


class TestValidationResult:
    """Validates error handling of invalid workflows via PyValidationResult."""

    def test_validation_result_with_errors(self):
        """Should raise an exception on cyclic workflow graph structure."""
        workflow = graphbit.PyWorkflow("invalid", "Will create a cycle")

        # Create two agent nodes
        agent1 = graphbit.PyWorkflowNode.agent_node("a1", "desc", "agent_001", "prompt")
        agent2 = graphbit.PyWorkflowNode.agent_node("a2", "desc", "agent_002", "prompt")

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)

        # Create a circular dependency
        edge = graphbit.PyWorkflowEdge.data_flow()
        workflow.connect_nodes(id1, id2, edge)
        workflow.connect_nodes(id2, id1, edge)

        # This should raise an exception during validation
        with pytest.raises(RuntimeError, match="contains cycles"):
            workflow.validate()
