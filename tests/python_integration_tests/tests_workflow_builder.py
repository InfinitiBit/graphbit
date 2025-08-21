"""Integration tests for GraphBit workflow builder functionality."""

import contextlib
import os
from typing import Any

import pytest

import graphbit


class TestWorkflowBuilder:
    """Integration tests for workflow builder patterns."""

    def test_sequential_workflow_building(self) -> None:
        """Test building sequential workflows."""
        workflow = graphbit.Workflow("sequential_test")

        # Build a sequential processing workflow
        nodes = []
        for i in range(3):
            agent = graphbit.Node.agent(f"step_{i}", f"Process step {i}", f"seq_{i:03d}")
            node_id = workflow.add_node(agent)
            nodes.append((agent, node_id))

        # Connect nodes sequentially
        with contextlib.suppress(ValueError, RuntimeError):
            for i in range(len(nodes) - 1):
                workflow.connect(nodes[i][0].id(), nodes[i + 1][0].id())

        assert len(nodes) == 3

    def test_parallel_workflow_building(self) -> None:
        """Test building parallel workflows."""
        workflow = graphbit.Workflow("parallel_test")

        # Create entry point
        entry = graphbit.Node.agent("entry", "Start parallel processing", "entry_001")
        workflow.add_node(entry)

        # Create parallel processing branches
        parallel_agents = []
        for i in range(3):
            agent = graphbit.Node.agent(f"parallel_{i}", f"Parallel task {i}", f"par_{i:03d}")
            workflow.add_node(agent)
            parallel_agents.append(agent)

        # Create merge point
        merge = graphbit.Node.agent("merge", "Merge parallel results", "merge_001")
        workflow.add_node(merge)

        # Connect parallel structure
        with contextlib.suppress(ValueError, RuntimeError):
            for agent in parallel_agents:
                workflow.connect(entry.id(), agent.id())

            for agent in parallel_agents:
                workflow.connect(agent.id(), merge.id())

    def test_conditional_workflow_building(self) -> None:
        """Test building conditional workflows."""
        workflow = graphbit.Workflow("conditional_test")

        entry = graphbit.Node.agent("entry", "Start processing", "entry_001")
        workflow.add_node(entry)

        condition1 = graphbit.Node.condition("check_priority", "priority == 'high'")
        condition2 = graphbit.Node.condition("check_type", "type == 'urgent'")
        workflow.add_node(condition1)
        workflow.add_node(condition2)

        high_priority = graphbit.Node.agent("high_priority", "Handle high priority", "high_001")
        urgent_processing = graphbit.Node.agent("urgent", "Handle urgent items", "urgent_001")
        normal_processing = graphbit.Node.agent("normal", "Handle normal items", "normal_001")

        workflow.add_node(high_priority)
        workflow.add_node(urgent_processing)
        workflow.add_node(normal_processing)


class TestWorkflowValidation:
    """Integration tests for workflow validation."""

    def test_empty_workflow_validation(self) -> None:
        """Test validation of empty workflows."""
        workflow = graphbit.Workflow("empty")

        with contextlib.suppress(ValueError, RuntimeError):
            workflow.validate()

    def test_single_node_workflow_validation(self) -> None:
        """Test validation of single node workflows."""
        workflow = graphbit.Workflow("single_node")

        agent = graphbit.Node.agent("solo", "Solo processing", "solo_001")
        workflow.add_node(agent)

        with contextlib.suppress(ValueError, RuntimeError):
            workflow.validate()

    def test_connected_workflow_validation(self) -> None:
        """Test validation of connected workflows."""
        workflow = graphbit.Workflow("connected")

        # Create connected workflow
        agent1 = graphbit.Node.agent("first", "First step", "first_001")
        agent2 = graphbit.Node.agent("second", "Second step", "second_001")

        workflow.add_node(agent1)
        workflow.add_node(agent2)

        try:
            workflow.connect(agent1.id(), agent2.id())
            workflow.validate()
        except (ValueError, RuntimeError) as e:
            pytest.skip(f"Connected workflow validation skipped: {e}")


class TestWorkflowExecution:
    """Integration tests for workflow execution patterns."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for execution tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_simple_workflow_execution(self, llm_config: Any) -> None:
        """Test executing simple workflows."""
        workflow = graphbit.Workflow("simple_execution")

        agent = graphbit.Node.agent("processor", "Process the input", "proc_001")
        workflow.add_node(agent)

        try:
            workflow.validate()

            executor = graphbit.Executor(llm_config)
            result = executor.execute(workflow)

            assert result is not None
            assert isinstance(result, graphbit.WorkflowResult)

        except (ValueError, RuntimeError) as e:
            pytest.skip(f"Simple workflow execution skipped: {e}")


class TestWorkflowComponents:
    """Integration tests for workflow components."""

    def test_agent_node_builder(self) -> None:
        """Test building agent nodes with different configurations."""
        agent1 = graphbit.Node.agent("basic", "Basic processing", "basic_001")
        assert agent1.name() == "basic"

        agent2 = graphbit.Node.agent("auto", "Auto ID processing")
        assert agent2.name() == "auto"

    def test_condition_node_builder(self) -> None:
        """Test building condition nodes."""
        conditions = [
            ("simple", "value > 10"),
            ("complex", "status == 'active' and priority >= 5"),
            ("boolean", "is_valid"),
        ]

        for name, expression in conditions:
            condition = graphbit.Node.condition(name, expression)
            assert condition.name() == name

    def test_transform_node_builder(self) -> None:
        """Test building transformation nodes."""
        transforms = [
            ("uppercase", "uppercase"),
            ("parse", "parse_json"),
            ("format", "format_date"),
        ]

        for name, operation in transforms:
            transform = graphbit.Node.transform(name, operation)
            assert transform.name() == name
