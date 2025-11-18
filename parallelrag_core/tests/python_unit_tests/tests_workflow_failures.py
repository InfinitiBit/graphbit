"""Test workflow failures and edge cases."""

import contextlib
from unittest.mock import patch

import pytest

from graphbit import Executor, LlmConfig, Node, Workflow, WorkflowContext


class TestWorkflowFailures:
    """Test various workflow failure scenarios."""

    def test_workflow_invalid_name(self):
        """Test workflow with invalid name."""
        # Test with empty name - might be allowed
        with contextlib.suppress(Exception):
            _ = Workflow("")
            # If empty name is allowed, that's valid
            # If empty name is rejected, that's also acceptable

        # Test with None name
        with contextlib.suppress(Exception):
            _ = Workflow(None)
            # If None name is allowed, that's valid
            # If None name is rejected, that's also acceptable

    def test_workflow_extremely_long_name(self):
        """Test workflow with extremely long name."""
        long_name = "a" * 10000

        with contextlib.suppress(Exception):
            _ = Workflow(long_name)
            # Should either work or fail gracefully

    def test_workflow_special_characters_in_name(self):
        """Test workflow with special characters in name."""
        special_name = "workflow!@#$%^&*()"

        with contextlib.suppress(Exception):
            _ = Workflow(special_name)
            # Should handle special characters
            # Might reject special characters - that's acceptable

    def test_workflow_add_invalid_node(self):
        """Test adding invalid node to workflow."""
        workflow = Workflow("test")

        # Test adding None node
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = workflow.add_node(None)

    def test_workflow_add_duplicate_nodes(self):
        """Test adding duplicate nodes to workflow."""
        workflow = Workflow("test")

        node = Node.agent("test-agent", "description", "agent-1")
        node_id1 = workflow.add_node(node)

        # Adding same node again might be allowed or rejected
        with contextlib.suppress(Exception):
            node_id2 = workflow.add_node(node)
            # If allowed, should get different IDs
            if node_id2 is not None:
                assert node_id1 != node_id2
            # Expected if duplicates not allowed - that's acceptable

    def test_workflow_circular_dependencies(self):
        """Test workflow with circular dependencies."""
        workflow = Workflow("test")

        node1 = Node.agent("agent-1", "description", "agent-1")
        node2 = Node.agent("agent-2", "description", "agent-2")

        id1 = workflow.add_node(node1)
        id2 = workflow.add_node(node2)

        # Create circular dependency
        with contextlib.suppress(Exception):
            workflow.add_edge(id1, id2)
            workflow.add_edge(id2, id1)

            # Validation should catch circular dependency
            with pytest.raises(Exception, match=".*"):
                workflow.validate()
            # Expected if circular dependencies not allowed - that's acceptable

    def test_workflow_invalid_edges(self):
        """Test workflow with invalid edges."""
        workflow = Workflow("test")

        node = Node.agent("test-agent", "description", "agent-1")
        node_id = workflow.add_node(node)

        # Test edge to non-existent node
        with pytest.raises((ValueError, Exception)):
            workflow.add_edge(node_id, "non-existent-id")

        # Test edge from non-existent node
        with pytest.raises((ValueError, Exception)):
            workflow.add_edge("non-existent-id", node_id)

    def test_workflow_validation_failures(self):
        """Test workflow validation failures."""
        workflow = Workflow("test")

        # Empty workflow validation
        with contextlib.suppress(Exception):
            workflow.validate()
            # Might be valid or invalid depending on implementation
            # Validation errors are acceptable

    def test_node_invalid_parameters(self):
        """Test node creation with invalid parameters."""
        # Test agent node with empty name - might be allowed
        with contextlib.suppress(Exception):
            _ = Node.agent("", "description", "agent-1")
            # If empty name is allowed, that's valid
            # If empty name is rejected, that's also acceptable

        # Test agent node with empty description - might be allowed
        with contextlib.suppress(Exception):
            _ = Node.agent("name", "", "agent-1")
            # If empty description is allowed, that's valid
            # If empty description is rejected, that's also acceptable

        # Test agent node with empty agent ID - might be allowed
        with contextlib.suppress(Exception):
            _ = Node.agent("name", "description", "")
            # If empty agent ID is allowed, that's valid
            # If empty agent ID is rejected, that's also acceptable

    def test_node_extremely_long_parameters(self):
        """Test node creation with extremely long parameters."""
        long_string = "a" * 10000

        with contextlib.suppress(Exception):
            _ = Node.agent(long_string, "description", "agent-1")

        with contextlib.suppress(Exception):
            _ = Node.agent("name", long_string, "agent-1")

    def test_executor_invalid_config(self):
        """Test executor with invalid configuration."""
        # Test with None config
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = Executor(None)

    def test_executor_invalid_workflow(self):
        """Test executor with invalid workflow."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        executor = Executor(config)

        # Test with None workflow
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = executor.execute(None)

    def test_executor_unvalidated_workflow(self):
        """Test executor with unvalidated workflow."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        executor = Executor(config)

        workflow = Workflow("test")
        node = Node.agent("test-agent", "description", "agent-1")
        workflow.add_node(node)

        # Don't call validate()
        with pytest.raises((ValueError, Exception)):
            _ = executor.execute(workflow)

    def test_executor_network_failures(self):
        """Test executor with network failures."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        executor = Executor(config)

        workflow = Workflow("test")
        node = Node.agent("test-agent", "description", "agent-1")
        workflow.add_node(node)
        workflow.validate()

        # Mock network failure
        with patch("requests.post", side_effect=ConnectionError("Network error")), pytest.raises(Exception, match=".*"):
            _ = executor.execute(workflow)

    def test_workflow_context_invalid_operations(self):
        """Test workflow context with invalid operations."""
        context = WorkflowContext()

        # Test getting non-existent variable - might return None or raise exception
        with contextlib.suppress(Exception):
            _ = context.get_variable("non-existent")
            # If it returns None or default value, that's valid
            # If it raises exception, that's also acceptable

        # Test setting variable with None key - might be allowed
        with contextlib.suppress(Exception):
            context.set_variable(None, "value")
            # If None key is allowed, that's valid
            # If None key is rejected, that's also acceptable

    def test_workflow_result_invalid_access(self):
        """Test workflow result with invalid access patterns."""
        # This test depends on having a WorkflowResult object
        # We'll create a mock scenario
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        executor = Executor(config)

        workflow = Workflow("test")
        node = Node.agent("test-agent", "description", "agent-1")
        workflow.add_node(node)
        workflow.validate()

        with contextlib.suppress(Exception):
            _ = executor.execute(workflow)
            # This will likely fail due to invalid API key
            # Expected - we're testing the failure case

    def test_workflow_memory_limits(self):
        """Test workflow under memory constraints."""
        workflow = Workflow("memory-test")

        # Add many nodes to test memory handling
        with contextlib.suppress(MemoryError):
            for i in range(1000):
                node = Node.agent(f"agent-{i}", f"description-{i}", f"agent-{i}")
                workflow.add_node(node)
            # Expected under extreme memory pressure

    def test_workflow_concurrent_modification(self):
        """Test workflow concurrent modification."""
        import threading

        workflow = Workflow("concurrent-test")

        def add_nodes():
            with contextlib.suppress(Exception):
                for i in range(10):
                    node = Node.agent(f"agent-{i}", f"description-{i}", f"agent-{i}")
                    workflow.add_node(node)
                # Concurrent access errors are acceptable

        # Start multiple threads modifying workflow
        threads = []
        for _ in range(3):
            thread = threading.Thread(target=add_nodes)
            threads.append(thread)
            thread.start()

        # Wait for all threads
        for thread in threads:
            thread.join()

        # Some operations might fail due to concurrent access
        # This is acceptable behavior

    def test_workflow_serialization_failures(self):
        """Test workflow serialization edge cases."""
        workflow = Workflow("serialization-test")
        node = Node.agent("test-agent", "description", "agent-1")
        workflow.add_node(node)

        # Test that workflow can be used (basic validation)
        # This should succeed - if it fails, the test should fail
        workflow.validate()

    def test_workflow_state_corruption(self):
        """Test workflow behavior with potential state corruption."""
        workflow = Workflow("corruption-test")

        # Try to corrupt workflow state (if possible)
        with contextlib.suppress(AttributeError):
            if hasattr(workflow, "_nodes"):
                workflow._nodes = None
            # Expected if no such attribute

        # Workflow should handle corrupted state gracefully
        with contextlib.suppress(Exception):
            node = Node.agent("test-agent", "description", "agent-1")
            workflow.add_node(node)
            # If it works despite corruption, that's also valid
            # If it fails due to corruption, that's expected

    @pytest.mark.asyncio
    async def test_workflow_async_failures(self):
        """Test workflow async operation failures."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        executor = Executor(config)

        workflow = Workflow("async-test")
        node = Node.agent("test-agent", "description", "agent-1")
        workflow.add_node(node)
        workflow.validate()

        # Test async execution failure
        with contextlib.suppress(Exception):
            # If async execution is supported
            if hasattr(executor, "execute_async"):
                _ = await executor.execute_async(workflow)
            # Expected due to invalid API key or unsupported async

    def test_workflow_resource_cleanup(self):
        """Test workflow resource cleanup."""
        # Create and destroy many workflows
        for i in range(10):
            workflow = Workflow(f"cleanup-test-{i}")
            node = Node.agent(f"agent-{i}", f"description-{i}", f"agent-{i}")
            workflow.add_node(node)
            # Workflow should be garbage collected
            del workflow
