"""Tests validation behavior for workflows with structural errors."""

import pytest

from graphbit import Node, Workflow


class TestValidationResult:
    """Validates error handling of invalid workflows via ValidationResult."""

    def test_validation_result_with_errors(self):
        """Should raise an exception on cyclic workflow graph structure."""
        workflow = Workflow("invalid")

        # Create two agent nodes
        agent1 = Node.agent("a1", "First agent description", "agent_001")
        agent2 = Node.agent("a2", "Second agent description", "agent_002")

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)

        # Create a circular dependency
        workflow.connect(id1, id2)
        workflow.connect(id2, id1)

        # This should raise an exception during validation
        with pytest.raises((ValueError, RuntimeError)):
            workflow.validate()
