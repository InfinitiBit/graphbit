"""Test suite for building workflows using PyWorkflowBuilder."""

import graphbit


class TestWorkflowBuilder:
    """Tests the builder pattern for GraphBit workflows."""

    def test_workflow_builder_roundtrip(self):
        """Should create and validate a workflow using the builder interface."""
        builder = graphbit.PyWorkflowBuilder("builder_test")
        builder.description("Workflow created using builder")

        # Create two distinct agent nodes
        node1 = graphbit.PyWorkflowNode.agent_node("n1", "desc", "agent_001", "Prompt 1")
        node2 = graphbit.PyWorkflowNode.agent_node("n2", "desc", "agent_002", "Prompt 2")

        id1 = builder.add_node(node1)
        id2 = builder.add_node(node2)

        # Connect node1 -> node2 to avoid circular dependency
        builder.connect(id1, id2, graphbit.PyWorkflowEdge.control_flow())

        # Build the workflow
        workflow = builder.build()

        # Validate the result
        assert workflow.name() == "builder_test"
        assert workflow.node_count() == 2
        assert workflow.edge_count() == 1
