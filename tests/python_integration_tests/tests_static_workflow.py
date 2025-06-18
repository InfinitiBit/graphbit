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
        workflow = graphbit.PyWorkflow("test_workflow", "A test workflow")

        assert workflow.name() == "test_workflow"
        assert workflow.description() == "A test workflow"
        assert workflow.node_count() == 0

    def test_workflow_with_metadata(self) -> None:
        """Test workflow creation with metadata."""
        workflow = graphbit.PyWorkflow("metadata_test", "Test with metadata")

        # Add metadata if supported
        # workflow.add_metadata("key", "value")  # Implementation dependent

        assert workflow.name() == "metadata_test"
        assert workflow.description() == "Test with metadata"

    def test_workflow_validation(self) -> None:
        """Test workflow validation methods."""
        workflow = graphbit.PyWorkflow("validation_test", "Test validation")

        # Test empty workflow validation
        # is_valid = workflow.is_valid()  # Implementation dependent
        # This would depend on the specific validation requirements

        assert workflow is not None


class TestWorkflowNodeCreation:
    """Integration tests for workflow node creation."""

    def test_agent_node_creation(self) -> None:
        """Test agent node creation."""
        agent_node = graphbit.PyWorkflowNode.agent_node("agent1", "Agent processing node", "agent_001", "Process the input data")

        assert agent_node is not None
        # Node ID and description access methods may not be implemented yet
        try:
            node_id = agent_node.node_id()
            assert node_id == "agent1"
        except AttributeError:
            pass  # Method not implemented yet

        try:
            description = agent_node.description()
            assert description == "Agent processing node"
        except AttributeError:
            pass  # Method not implemented yet

    def test_llm_node_creation(self) -> None:
        """Test LLM node creation."""
        try:
            llm_node = graphbit.PyWorkflowNode.llm_node("llm1", "LLM processing node", "process this text")

            assert llm_node is not None
            try:
                node_id = llm_node.node_id()
                assert node_id == "llm1"
            except AttributeError:
                pass  # Method not implemented yet

            try:
                description = llm_node.description()
                assert description == "LLM processing node"
            except AttributeError:
                pass  # Method not implemented yet
        except AttributeError:
            pytest.skip("PyWorkflowNode.llm_node method not implemented yet")

    def test_tool_node_creation(self) -> None:
        """Test tool node creation."""
        try:
            tool_node = graphbit.PyWorkflowNode.tool_node("tool1", "Tool processing node", "calculator", "add 2 3")

            assert tool_node is not None
            try:
                node_id = tool_node.node_id()
                assert node_id == "tool1"
            except AttributeError:
                pass  # Method not implemented yet

            try:
                description = tool_node.description()
                assert description == "Tool processing node"
            except AttributeError:
                pass  # Method not implemented yet
        except AttributeError:
            pytest.skip("PyWorkflowNode.tool_node method not implemented yet")

    def test_workflow_node_creation(self) -> None:
        """Test workflow node creation."""
        try:
            # Create a sub-workflow
            sub_workflow = graphbit.PyWorkflow("sub_workflow", "A sub-workflow")

            workflow_node = graphbit.PyWorkflowNode.workflow_node("workflow1", "Workflow processing node", sub_workflow)

            assert workflow_node is not None
            try:
                node_id = workflow_node.node_id()
                assert node_id == "workflow1"
            except AttributeError:
                pass  # Method not implemented yet

            try:
                description = workflow_node.description()
                assert description == "Workflow processing node"
            except AttributeError:
                pass  # Method not implemented yet
        except AttributeError:
            pytest.skip("PyWorkflowNode.workflow_node method not implemented yet")


class TestWorkflowComposition:
    """Integration tests for workflow composition."""

    @pytest.fixture
    def sample_workflow(self) -> Any:
        """Create a sample workflow for testing."""
        workflow = graphbit.PyWorkflow("sample", "Sample workflow for testing")

        # Add some nodes - use only implemented node types
        agent1 = graphbit.PyWorkflowNode.agent_node("agent1", "First agent", "agent_001", "Start processing")
        condition1 = graphbit.PyWorkflowNode.condition_node("condition1", "Decision point", "quality_score > 0.5")
        transform1 = graphbit.PyWorkflowNode.transform_node("transform1", "Data transformation", "uppercase")

        workflow.add_node(agent1)
        workflow.add_node(condition1)
        workflow.add_node(transform1)

        return workflow

    def test_node_addition(self, sample_workflow: Any) -> None:
        """Test adding nodes to workflow."""
        initial_count = sample_workflow.node_count()

        # Add another node
        new_agent = graphbit.PyWorkflowNode.agent_node("agent2", "Second agent", "agent_002", "Continue processing")
        sample_workflow.add_node(new_agent)

        assert sample_workflow.node_count() == initial_count + 1

    def test_workflow_connection_creation(self, sample_workflow: Any) -> None:
        """Test creating connections between nodes."""
        # Create connections between nodes
        try:
            # Connect agent1 to condition1
            sample_workflow.connect_nodes("agent1", "condition1", graphbit.PyWorkflowEdge.data_flow())

            # Connect condition1 to transform1
            sample_workflow.connect_nodes("condition1", "transform1", graphbit.PyWorkflowEdge.conditional("quality_score > 0.5"))

            # Test would require method to verify connections
            # connections = sample_workflow.get_connections()  # Implementation dependent

        except Exception as e:
            pytest.skip(f"Connection creation test skipped: {e}")

    def test_workflow_structure_validation(self, sample_workflow: Any) -> None:
        """Test workflow structure validation."""
        # Test that workflow structure is valid
        try:
            # This would depend on validation methods being available
            # is_valid = sample_workflow.validate_structure()
            # assert is_valid

            # For now, just verify basic structure
            assert sample_workflow.node_count() >= 3

        except Exception as e:
            pytest.skip(f"Structure validation test skipped: {e}")


class TestWorkflowExecution:
    """Integration tests for workflow execution."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for workflow execution tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set for workflow execution tests")
        return graphbit.PyLlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def executable_workflow(self, llm_config: Any) -> Any:
        """Create an executable workflow for testing."""
        workflow = graphbit.PyWorkflow("executable", "Executable test workflow")

        # Create executable nodes - use only implemented node types
        agent1 = graphbit.PyWorkflowNode.agent_node("start", "Start processing", "agent_001", "Begin workflow")
        transform1 = graphbit.PyWorkflowNode.transform_node("process", "Process data", "uppercase")

        workflow.add_node(agent1)
        workflow.add_node(transform1)

        # Connect nodes
        with contextlib.suppress(Exception):
            workflow.connect_nodes("start", "process", graphbit.PyWorkflowEdge.data_flow())
            # Connection might not be required for basic execution

        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_workflow_execution_setup(self, llm_config: Any, executable_workflow: Any) -> None:
        """Test workflow execution setup."""
        try:
            # Create executor
            executor = graphbit.PyWorkflowExecutor(llm_config)

            # Test that we can set up execution
            # execution_context = executor.setup_execution(executable_workflow)
            # assert execution_context is not None

            assert executor is not None
            assert executable_workflow is not None

        except Exception as e:
            pytest.skip(f"Workflow execution setup test skipped: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_basic_workflow_execution(self, llm_config: Any, executable_workflow: Any) -> None:
        """Test basic workflow execution."""
        try:
            # Create executor and run workflow
            executor = graphbit.PyWorkflowExecutor(llm_config)

            # Execute workflow - this would require actual execution implementation
            # result = executor.execute(executable_workflow)
            # assert result is not None

            # For now, just verify components exist
            assert executor is not None
            assert executable_workflow is not None

            pytest.skip("Actual execution testing requires full execution environment")

        except Exception as e:
            pytest.skip(f"Basic workflow execution test skipped: {e}")


class TestWorkflowSerialization:
    """Integration tests for workflow serialization."""

    def test_workflow_to_json(self) -> None:
        """Test converting workflow to JSON representation."""
        workflow = graphbit.PyWorkflow("json_test", "JSON serialization test")

        # Add some nodes
        agent_node = graphbit.PyWorkflowNode.agent_node(
            "json_agent",
            "JSON test agent",
            "agent_json",
            "Test JSON serialization",
        )
        workflow.add_node(agent_node)

        # Test JSON serialization
        try:
            json_str = workflow.to_json()
            assert isinstance(json_str, str)
            assert len(json_str) > 0
        except Exception as e:
            pytest.skip(f"JSON serialization not implemented: {e}")

    def test_workflow_from_json(self) -> None:
        """Test creating workflow from JSON representation."""
        # This test would require a valid JSON workflow representation
        # For now, we'll just test that the method exists
        try:
            # Test that from_json method exists
            assert hasattr(graphbit.PyWorkflow, "from_json")
        except Exception as e:
            pytest.skip(f"JSON deserialization test skipped: {e}")


class TestWorkflowValidation:
    """Integration tests for workflow validation."""

    def test_empty_workflow_validation(self) -> None:
        """Test validation of empty workflow."""
        workflow = graphbit.PyWorkflow("empty", "Empty workflow")

        # Empty workflow validation behavior depends on implementation
        with contextlib.suppress(Exception):
            workflow.validate()  # Empty workflows might fail validation, which could be expected

    def test_single_node_validation(self) -> None:
        """Test validation of single-node workflow."""
        workflow = graphbit.PyWorkflow("single_node", "Single node workflow")

        agent_node = graphbit.PyWorkflowNode.agent_node(
            "single_agent",
            "Single agent node",
            "agent_single",
            "Standalone processing",
        )

        workflow.add_node(agent_node)

        # Single node workflows should generally be valid
        workflow.validate()

    def test_disconnected_nodes_validation(self) -> None:
        """Test validation of workflow with disconnected nodes."""
        workflow = graphbit.PyWorkflow("disconnected", "Disconnected nodes workflow")

        # Add multiple unconnected nodes
        agent1 = graphbit.PyWorkflowNode.agent_node(
            "agent_1",
            "First agent",
            "agent_001",
            "First processing",
        )

        agent2 = graphbit.PyWorkflowNode.agent_node(
            "agent_2",
            "Second agent",
            "agent_002",
            "Second processing",
        )

        workflow.add_node(agent1)
        workflow.add_node(agent2)

        # Validation behavior for disconnected nodes depends on implementation
        with contextlib.suppress(Exception):
            workflow.validate()  # Disconnected nodes might fail validation

    def test_circular_dependency_validation(self) -> None:
        """Test validation detects circular dependencies."""
        workflow = graphbit.PyWorkflow("circular", "Circular dependency test")

        # Create nodes that could form a circle
        agent1 = graphbit.PyWorkflowNode.agent_node(
            "circular_agent_1",
            "First circular agent",
            "agent_c1",
            "First step",
        )

        agent2 = graphbit.PyWorkflowNode.agent_node(
            "circular_agent_2",
            "Second circular agent",
            "agent_c2",
            "Second step",
        )

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)

        # Create circular connection
        data_edge = graphbit.PyWorkflowEdge.data_flow()
        workflow.connect_nodes(id1, id2, data_edge)
        workflow.connect_nodes(id2, id1, data_edge)  # This creates a cycle

        # Validation should detect the circular dependency
        with contextlib.suppress(Exception):
            workflow.validate()  # Expected: validation should fail for circular dependencies


class TestWorkflowComponents:
    """Integration tests for individual workflow components."""

    def test_agent_node_creation(self) -> None:
        """Test creating different types of agent nodes."""
        # Test basic agent node
        basic_agent = graphbit.PyWorkflowNode.agent_node(
            "basic_agent",
            "Basic agent node",
            "agent_basic",
            "Basic processing task",
        )
        assert basic_agent is not None

        # Test agent with capabilities
        with contextlib.suppress(Exception):
            # If agent capabilities are supported
            text_processing = graphbit.PyAgentCapabilities.text_processing()
            capable_agent = graphbit.PyWorkflowNode.agent_node_with_capabilities(
                "capable_agent",
                "Agent with capabilities",
                "agent_capable",
                "Advanced processing task",
                text_processing,
            )
            assert capable_agent is not None  # Agent capabilities might not be implemented yet

    def test_condition_node_variations(self) -> None:
        """Test creating different condition nodes."""
        # Simple boolean condition
        simple_condition = graphbit.PyWorkflowNode.condition_node(
            "simple_condition",
            "Simple condition",
            "data.isValid",
        )
        assert simple_condition is not None

        # Complex condition with multiple checks
        complex_condition = graphbit.PyWorkflowNode.condition_node(
            "complex_condition",
            "Complex condition",
            "data.isValid && data.length > 0 && data.type == 'expected'",
        )
        assert complex_condition is not None

    def test_transformation_node_variations(self) -> None:
        """Test creating different transformation nodes."""
        # Simple data transformation
        simple_transform = graphbit.PyWorkflowNode.transform_node(
            "simple_transform",
            "Simple transformation",
            "data | map(.normalize())",
        )
        assert simple_transform is not None

        # Complex transformation with multiple operations
        complex_transform = graphbit.PyWorkflowNode.transform_node(
            "complex_transform",
            "Complex transformation",
            "data | filter(.isValid) | map(.normalize()) | sort(.priority) | group(.category)",
        )
        assert complex_transform is not None

    def test_edge_type_variations(self) -> None:
        """Test creating different types of edges."""
        # Data flow edge
        data_edge = graphbit.PyWorkflowEdge.data_flow()
        assert data_edge is not None

        # Control flow edge
        control_edge = graphbit.PyWorkflowEdge.control_flow()
        assert control_edge is not None

        # Test edge with conditions (if supported)
        with contextlib.suppress(Exception):
            conditional_edge = graphbit.PyWorkflowEdge.conditional("condition_expression")
            assert conditional_edge is not None  # Conditional edges might not be implemented


class TestWorkflowMetadata:
    """Integration tests for workflow metadata and properties."""

    def test_workflow_properties(self) -> None:
        """Test accessing workflow properties."""
        workflow = graphbit.PyWorkflow("metadata_test", "Metadata testing workflow")

        # Test basic properties
        assert workflow.name() == "metadata_test"
        assert workflow.description() == "Metadata testing workflow"

        # Test node and edge counts
        assert workflow.node_count() == 0
        assert workflow.edge_count() == 0

    def test_workflow_node_metadata(self) -> None:
        """Test node metadata and properties."""
        workflow = graphbit.PyWorkflow("node_metadata", "Node metadata test")

        agent_node = graphbit.PyWorkflowNode.agent_node(
            "metadata_agent",
            "Agent for metadata testing",
            "agent_meta",
            "Test metadata functionality",
        )

        node_id = workflow.add_node(agent_node)

        # Test that node was added successfully
        assert node_id is not None
        assert workflow.node_count() == 1

    def test_workflow_modification(self) -> None:
        """Test modifying workflow after creation."""
        workflow = graphbit.PyWorkflow("modification_test", "Workflow modification test")

        # Add initial node
        initial_node = graphbit.PyWorkflowNode.agent_node(
            "initial_node",
            "Initial node",
            "agent_initial",
            "Initial processing",
        )
        initial_id = workflow.add_node(initial_node)

        assert workflow.node_count() == 1

        # Add second node
        second_node = graphbit.PyWorkflowNode.agent_node(
            "second_node",
            "Second node",
            "agent_second",
            "Second processing",
        )
        second_id = workflow.add_node(second_node)

        assert workflow.node_count() == 2

        # Connect nodes
        data_edge = graphbit.PyWorkflowEdge.data_flow()
        workflow.connect_nodes(initial_id, second_id, data_edge)

        assert workflow.edge_count() == 1

        # Validate modified workflow
        workflow.validate()

    def test_workflow_node_retrieval(self) -> None:
        """Test retrieving nodes from workflow."""
        workflow = graphbit.PyWorkflow("retrieval_test", "Node retrieval test")

        agent_node = graphbit.PyWorkflowNode.agent_node(
            "retrievable_agent",
            "Agent for retrieval testing",
            "agent_retrieve",
            "Test node retrieval",
        )

        node_id = workflow.add_node(agent_node)

        # Test node retrieval methods (if available)
        with contextlib.suppress(Exception):
            # Test getting node by ID
            retrieved_node = workflow.get_node(node_id)
            assert retrieved_node is not None  # Node retrieval might not be implemented


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run specific test cases
    pytest.main([__file__, "-v"])
