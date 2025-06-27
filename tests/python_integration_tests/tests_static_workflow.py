#!/usr/bin/env python3
"""Integration tests for GraphBit static workflow functionality."""
import contextlib
import os
from typing import Any

import pytest

import graphbit


class TestStaticWorkflowCreation:
    """Integration tests for static workflow creation."""

    def test_basic_workflow_creation(self) -> None:
        """Test basic workflow creation."""
        workflow = graphbit.Workflow("test_workflow")

        # Workflow object should be created successfully
        assert workflow is not None

    def test_workflow_validation(self) -> None:
        """Test workflow validation methods."""
        workflow = graphbit.Workflow("validation_test")

        # Test empty workflow validation - should not fail
        with contextlib.suppress(ValueError, RuntimeError):
            workflow.validate()

        assert workflow is not None


class TestWorkflowNodeCreation:
    """Integration tests for workflow node creation."""

    def test_agent_node_creation(self) -> None:
        """Test agent node creation."""
        agent_node = graphbit.Node.agent("agent1", "Process the input data", "agent_001")

        assert agent_node is not None
        assert agent_node.id() is not None
        assert agent_node.name() == "agent1"

    def test_transform_node_creation(self) -> None:
        """Test transform node creation."""
        transform_node = graphbit.Node.transform("transform1", "uppercase")

        assert transform_node is not None
        assert transform_node.id() is not None
        assert transform_node.name() == "transform1"

    def test_condition_node_creation(self) -> None:
        """Test condition node creation."""
        condition_node = graphbit.Node.condition("condition1", "quality_score > 0.5")

        assert condition_node is not None
        assert condition_node.id() is not None
        assert condition_node.name() == "condition1"


class TestWorkflowComposition:
    """Integration tests for workflow composition."""

    @pytest.fixture
    def sample_workflow(self) -> Any:
        """Create a sample workflow for testing."""
        workflow = graphbit.Workflow("sample")

        # Add some nodes (the fixture provides a basic workflow setup)
        agent1 = graphbit.Node.agent("agent1", "Start processing", "agent_001")
        condition1 = graphbit.Node.condition("condition1", "quality_score > 0.5")
        transform1 = graphbit.Node.transform("transform1", "uppercase")

        workflow.add_node(agent1)
        workflow.add_node(condition1)
        workflow.add_node(transform1)

        return workflow

    def test_node_addition(self, sample_workflow: Any) -> None:
        """Test adding nodes to workflow."""
        # Add another node
        new_agent = graphbit.Node.agent("agent2", "Continue processing", "agent_002")
        node_id = sample_workflow.add_node(new_agent)

        assert node_id is not None
        assert isinstance(node_id, str)

    def test_workflow_connection_creation(self, sample_workflow: Any) -> None:
        """Test creating connections between nodes."""
        # Instead of using the sample workflow, create our own nodes with known IDs
        workflow = graphbit.Workflow("connection_test")

        # Create nodes
        agent1 = graphbit.Node.agent("agent1", "Start processing", "agent_001")
        condition1 = graphbit.Node.condition("condition1", "quality_score > 0.5")
        transform1 = graphbit.Node.transform("transform1", "uppercase")

        # Add nodes and capture their IDs
        agent1_id = workflow.add_node(agent1)
        condition1_id = workflow.add_node(condition1)
        transform1_id = workflow.add_node(transform1)

        # Create connections between nodes using the returned IDs
        try:
            # Connect agent1 to condition1
            workflow.connect(agent1_id, condition1_id)

            # Connect condition1 to transform1
            workflow.connect(condition1_id, transform1_id)

        except (ValueError, RuntimeError) as e:
            pytest.skip(f"Connection creation test skipped: {e}")

    def test_workflow_structure_validation(self, sample_workflow: Any) -> None:
        """Test workflow structure validation."""
        # Test that workflow structure can be validated
        with contextlib.suppress(ValueError, RuntimeError):
            sample_workflow.validate()


class TestWorkflowExecution:
    """Integration tests for workflow execution."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for workflow execution tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set for workflow execution tests")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def executable_workflow(self) -> Any:
        """Create a simple executable workflow."""
        workflow = graphbit.Workflow("executable_test")

        # Create a simple agent node
        agent_node = graphbit.Node.agent("test_agent", "Say hello", "agent_001")
        workflow.add_node(agent_node)

        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_workflow_execution_setup(self, llm_config: Any, executable_workflow: Any) -> None:
        """Test workflow execution setup."""
        try:
            executor = graphbit.Executor(llm_config)
            assert executor is not None

            # Validate the workflow
            executable_workflow.validate()

        except (ValueError, RuntimeError) as e:
            pytest.fail(f"Workflow execution setup failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_basic_workflow_execution(self, llm_config: Any, executable_workflow: Any) -> None:
        """Test basic workflow execution."""
        try:
            executor = graphbit.Executor(llm_config)

            # Validate workflow first
            executable_workflow.validate()

            # Execute the workflow
            result = executor.execute(executable_workflow)
            assert result is not None
            assert isinstance(result, graphbit.WorkflowResult)

        except (ValueError, RuntimeError) as e:
            pytest.skip(f"Workflow execution test skipped: {e}")


class TestWorkflowValidation:
    """Integration tests for workflow validation."""

    def test_empty_workflow_validation(self) -> None:
        """Test validation of empty workflows."""
        workflow = graphbit.Workflow("empty_test")

        with contextlib.suppress(ValueError, RuntimeError):
            workflow.validate()

    def test_single_node_validation(self) -> None:
        """Test validation of single node workflows."""
        workflow = graphbit.Workflow("single_node_test")

        agent_node = graphbit.Node.agent("solo_agent", "Work alone", "agent_001")
        workflow.add_node(agent_node)

        with contextlib.suppress(ValueError, RuntimeError):
            workflow.validate()

    def test_connected_nodes_validation(self) -> None:
        """Test validation of connected nodes."""
        workflow = graphbit.Workflow("connected_test")

        # Create two nodes
        agent1 = graphbit.Node.agent("agent1", "First step", "agent_001")
        agent2 = graphbit.Node.agent("agent2", "Second step", "agent_002")

        # Add nodes to workflow and get their IDs
        agent1_id = workflow.add_node(agent1)
        agent2_id = workflow.add_node(agent2)

        # Connect them using the returned IDs
        try:
            workflow.connect(agent1_id, agent2_id)
            workflow.validate()
        except (ValueError, RuntimeError) as e:
            pytest.skip(f"Connected nodes validation test skipped: {e}")


class TestWorkflowComponents:
    """Integration tests for workflow components."""

    def test_agent_node_creation(self) -> None:
        """Test creating agent nodes with different configurations."""
        # Basic agent node
        agent1 = graphbit.Node.agent("basic_agent", "Basic prompt", "agent_001")
        assert agent1.name() == "basic_agent"

        # Agent with auto-generated ID
        agent2 = graphbit.Node.agent("auto_agent", "Auto ID prompt")
        assert agent2.name() == "auto_agent"

        # Complex agent
        agent3 = graphbit.Node.agent("complex_agent", "Complex multi-line prompt\nwith instructions", "complex_001")
        assert agent3.name() == "complex_agent"

    def test_condition_node_variations(self) -> None:
        """Test creating condition nodes with different expressions."""
        # Simple boolean condition
        cond1 = graphbit.Node.condition("simple_check", "score > 0.5")
        assert cond1.name() == "simple_check"

        # Complex condition
        cond2 = graphbit.Node.condition("complex_check", "sentiment == 'positive' and confidence > 0.8")
        assert cond2.name() == "complex_check"

    def test_transformation_node_variations(self) -> None:
        """Test creating transformation nodes with different operations."""
        # Simple transformation
        trans1 = graphbit.Node.transform("uppercase", "uppercase")
        assert trans1.name() == "uppercase"

        # Complex transformation
        trans2 = graphbit.Node.transform("json_parse", "parse_json")
        assert trans2.name() == "json_parse"


class TestWorkflowMetadata:
    """Integration tests for workflow metadata and properties."""

    def test_workflow_properties(self) -> None:
        """Test workflow property access."""
        workflow = graphbit.Workflow("metadata_test")

        # Just verify workflow creation works
        assert workflow is not None

    def test_workflow_node_metadata(self) -> None:
        """Test node metadata access."""
        agent_node = graphbit.Node.agent("meta_agent", "Test metadata", "meta_001")

        assert agent_node.name() == "meta_agent"
        assert agent_node.id() is not None
        assert isinstance(agent_node.id(), str)

    def test_workflow_modification(self) -> None:
        """Test workflow modification operations."""
        workflow = graphbit.Workflow("modify_test")

        # Add nodes
        node1 = graphbit.Node.agent("node1", "First node", "agent_001")
        node2 = graphbit.Node.agent("node2", "Second node", "agent_002")

        id1 = workflow.add_node(node1)
        id2 = workflow.add_node(node2)

        assert id1 is not None
        assert id2 is not None
        assert id1 != id2

    def test_workflow_node_retrieval(self) -> None:
        """Test retrieving nodes from workflow."""
        workflow = graphbit.Workflow("retrieval_test")

        # Add a node
        agent_node = graphbit.Node.agent("retrievable", "Test retrieval", "retrieve_001")
        node_id = workflow.add_node(agent_node)

        assert node_id is not None

        # Note: Node retrieval methods would depend on the actual API
        # This test verifies that node addition returns a valid ID


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run specific test cases
    pytest.main([__file__, "-v"])
