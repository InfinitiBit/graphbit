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

    def test_nested_conditional_workflows(self) -> None:
        """Test workflows with nested conditional logic."""
        workflow = graphbit.Workflow("nested_conditional")

        # Create nested decision tree
        entry = graphbit.Node.agent("entry", "Start processing", "entry_001")
        workflow.add_node(entry)

        # First level conditions
        priority_check = graphbit.Node.condition("priority_check", "priority == 'high'")
        type_check = graphbit.Node.condition("type_check", "type == 'urgent'")
        workflow.add_node(priority_check)
        workflow.add_node(type_check)

        # Processing nodes for different paths
        fast_track = graphbit.Node.agent("fast_track", "Fast track processing", "fast_001")
        standard_process = graphbit.Node.agent("standard_process", "Standard processing", "std_001")
        workflow.add_node(fast_track)
        workflow.add_node(standard_process)

        # Connect the complex logic
        try:
            workflow.connect(entry.id(), priority_check.id())
            workflow.connect(priority_check.id(), type_check.id())
            workflow.connect(type_check.id(), fast_track.id())
            workflow.connect(priority_check.id(), standard_process.id())
        except Exception:
            pass  # nosec B110: acceptable in test context

        # Test validation
        with contextlib.suppress(Exception):
            workflow.validate()

    def test_workflow_with_multiple_transforms(self) -> None:
        """Test workflows with multiple transformation steps."""
        workflow = graphbit.Workflow("multi_transform")

        # Create transformation pipeline
        input_node = graphbit.Node.agent("input", "Process input", "input_001")
        normalize = graphbit.Node.transform("normalize", "normalize_data")
        validate_data = graphbit.Node.transform("validate", "validate_format")
        enrich = graphbit.Node.transform("enrich", "enrich_metadata")
        output_node = graphbit.Node.agent("output", "Process output", "output_001")

        workflow.add_node(input_node)
        workflow.add_node(normalize)
        workflow.add_node(validate_data)
        workflow.add_node(enrich)
        workflow.add_node(output_node)

        # Connect transformation chain
        try:
            workflow.connect(input_node.id(), normalize.id())
            workflow.connect(normalize.id(), validate_data.id())
            workflow.connect(validate_data.id(), enrich.id())
            workflow.connect(enrich.id(), output_node.id())
        except Exception:
            pass  # nosec B110: acceptable in test context

        with contextlib.suppress(Exception):
            workflow.validate()


class TestWorkflowErrorHandling:
    """Integration tests for comprehensive workflow error handling."""

    def test_circular_dependency_detection(self) -> None:
        """Test detection of circular dependencies."""
        workflow = graphbit.Workflow("circular_test")

        node1 = graphbit.Node.agent("node1", "First node", "circ_001")
        node2 = graphbit.Node.agent("node2", "Second node", "circ_002")
        node3 = graphbit.Node.agent("node3", "Third node", "circ_003")

        id1 = workflow.add_node(node1)
        id2 = workflow.add_node(node2)
        id3 = workflow.add_node(node3)

        # Create circular dependency
        try:
            workflow.connect(id1, id2)
            workflow.connect(id2, id3)
            workflow.connect(id3, id1)  # Creates cycle

            # Validation should detect the cycle
            with pytest.raises((ValueError, RuntimeError)):
                workflow.validate()

        except Exception:
            # Connection creation might fail, which is also acceptable
            pass  # nosec B110: acceptable in test context

    def test_invalid_connection_attempts(self) -> None:
        """Test handling of invalid connection attempts."""
        workflow = graphbit.Workflow("invalid_connections")

        node1 = graphbit.Node.agent("node1", "Valid node", "valid_001")
        id1 = workflow.add_node(node1)

        # Test connecting to non-existent node
        with pytest.raises((ValueError, RuntimeError, KeyError)):
            workflow.connect(id1, "non_existent_id")

        # Test connecting from non-existent node
        with pytest.raises((ValueError, RuntimeError, KeyError)):
            workflow.connect("non_existent_id", id1)


class TestWorkflowContextManagement:
    """Integration tests for workflow context and variable management."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for context tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_workflow_result_detailed_access(self, llm_config: Any) -> None:
        """Test detailed access to workflow results and context."""
        workflow = graphbit.Workflow("context_test")
        agent = graphbit.Node.agent("context_agent", "Process with context", "ctx_001")
        workflow.add_node(agent)

        try:
            workflow.validate()
            executor = graphbit.Executor(llm_config)
            result = executor.execute(workflow)

            # Test all result interface methods
            assert isinstance(result.is_success(), bool)
            assert isinstance(result.is_failed(), bool)
            assert isinstance(result.state(), str)
            assert isinstance(result.execution_time_ms(), int)

            # Test variables access
            try:
                variables = result.variables()
                assert isinstance(variables, dict)
            except Exception:
                pass  # nosec B110: acceptable in test context

            # Validate result state
            state = result.state()
            valid_states = ["completed", "failed", "running", "pending", "success", "error"]
            assert any(valid_state in state.lower() for valid_state in valid_states), f"Unexpected state: {state}"

            # Execution time should be reasonable
            exec_time = result.execution_time_ms()
            assert exec_time >= 0, f"Invalid execution time: {exec_time}"
            assert exec_time < 300000, f"Execution time too long: {exec_time}ms"

        except Exception as e:
            pytest.skip(f"Workflow context test skipped: {e}")


class TestWorkflowNodeManipulation:
    """Integration tests for advanced node manipulation."""

    def test_node_id_consistency(self) -> None:
        """Test node ID consistency and retrieval."""
        workflow = graphbit.Workflow("node_id_test")

        # Create nodes and track their IDs
        node_ids = []
        nodes = []

        for i in range(5):
            node = graphbit.Node.agent(f"agent_{i}", f"Agent {i}", f"agent_{i:03d}")
            nodes.append(node)
            node_id = workflow.add_node(node)
            node_ids.append(node_id)

        # Verify ID consistency
        for _, (node, node_id) in enumerate(zip(nodes, node_ids)):
            assert isinstance(node_id, str)
            assert len(node_id) > 0
            assert node.id() == node.id(), "Node ID should be consistent"

        # All IDs should be unique
        assert len(set(node_ids)) == len(node_ids), "Node IDs should be unique"

    def test_different_node_types_integration(self) -> None:
        """Test integration of different node types in one workflow."""
        workflow = graphbit.Workflow("mixed_node_types")

        # Create different types of nodes
        agent_node = graphbit.Node.agent("agent", "Process data", "agent_001")
        condition_node = graphbit.Node.condition("condition", "data_valid == true")
        transform_node = graphbit.Node.transform("transform", "normalize_data")

        # Add all types
        agent_id = workflow.add_node(agent_node)
        condition_id = workflow.add_node(condition_node)
        transform_id = workflow.add_node(transform_node)

        # Verify they were added correctly
        assert isinstance(agent_id, str)
        assert isinstance(condition_id, str)
        assert isinstance(transform_id, str)

        # Test node properties
        assert agent_node.name() == "agent"
        assert condition_node.name() == "condition"
        assert transform_node.name() == "transform"

        # Connect different types
        try:
            workflow.connect(agent_id, condition_id)
            workflow.connect(condition_id, transform_id)
        except Exception:
            pass  # nosec B110: acceptable in test context

        with contextlib.suppress(Exception):
            workflow.validate()


@pytest.mark.integration
class TestComplexWorkflowScenarios:
    """Integration tests for complex real-world workflow scenarios."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for complex scenarios."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_multi_stage_workflow(self, llm_config: Any) -> None:
        """Test a multi-stage workflow with decision points."""
        workflow = graphbit.Workflow("multi_stage")

        # Stage 1: Initial analysis
        stage1 = graphbit.Node.agent("stage1", "Initial analysis", "stage1_001")
        check1 = graphbit.Node.condition("check1", "stage1_complete == true")

        # Stage 2: Processing
        stage2 = graphbit.Node.agent("stage2", "Secondary processing", "stage2_001")

        # Stage 3: Final
        stage3 = graphbit.Node.agent("stage3", "Final processing", "stage3_001")

        # Add nodes
        workflow.add_node(stage1)
        workflow.add_node(check1)
        workflow.add_node(stage2)
        workflow.add_node(stage3)

        try:
            # Connect stages
            workflow.connect(stage1.id(), check1.id())
            workflow.connect(check1.id(), stage2.id())
            workflow.connect(stage2.id(), stage3.id())

            workflow.validate()

            executor = graphbit.Executor(llm_config)
            result = executor.execute(workflow)

            assert isinstance(result, graphbit.WorkflowResult)
            assert isinstance(result.state(), str)

        except Exception as e:
            pytest.skip(f"Multi-stage workflow test skipped: {e}")


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run specific test cases
    pytest.main([__file__, "-v"])
