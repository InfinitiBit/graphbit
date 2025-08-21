"""Integration tests for GraphBit validation functionality."""

import contextlib
import os

import pytest

import graphbit


class TestAPIKeyValidation:
    """Comprehensive tests for API key validation across all providers."""

    def test_openai_api_key_validation(self) -> None:
        """Test OpenAI API key validation with various invalid keys."""
        # Test empty API key
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.LlmConfig.openai("")
        assert "empty" in str(exc_info.value).lower()

        # Test too short API key
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.LlmConfig.openai("sk-short")
        assert "short" in str(exc_info.value).lower()

        # Test invalid format
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.LlmConfig.openai("invalid-format-key")

        # Test very short key
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.LlmConfig.openai("sk-123")

    def test_anthropic_api_key_validation(self) -> None:
        """Test Anthropic API key validation with various invalid keys."""
        # Test empty API key
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.LlmConfig.anthropic("")
        assert "empty" in str(exc_info.value).lower()

        # Test too short API key
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.LlmConfig.anthropic("sk-ant-short")
        assert "short" in str(exc_info.value).lower()

        # Test invalid format
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.LlmConfig.anthropic("invalid")

    def test_huggingface_api_key_validation(self) -> None:
        """Test HuggingFace API key validation with various invalid keys."""
        # Test empty API key for LLM
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.LlmConfig.huggingface("", "gpt2")
        assert "empty" in str(exc_info.value).lower()

        # Test too short API key for LLM
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.LlmConfig.huggingface("hf_short", "gpt2")
        assert "short" in str(exc_info.value).lower()

        # Test empty API key for embeddings
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.EmbeddingConfig.huggingface("", "facebook/bart-base")
        assert "empty" in str(exc_info.value).lower()

        # Test too short API key for embeddings
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.EmbeddingConfig.huggingface("hf_short", "facebook/bart-base")
        assert "short" in str(exc_info.value).lower()

    def test_openai_embedding_api_key_validation(self) -> None:
        """Test OpenAI embedding API key validation."""
        # Test empty API key
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.EmbeddingConfig.openai("")
        assert "empty" in str(exc_info.value).lower()

        # Test too short API key
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            graphbit.EmbeddingConfig.openai("sk-short")
        assert "short" in str(exc_info.value).lower()

    def test_valid_api_key_formats(self) -> None:
        """Test that properly formatted (but potentially invalid) API keys pass validation."""
        # These should pass validation but may fail authentication
        try:
            # OpenAI format
            config1 = graphbit.LlmConfig.openai("sk-" + "x" * 48, "gpt-3.5-turbo")
            assert config1.provider() == "openai"

            # Anthropic format
            config2 = graphbit.LlmConfig.anthropic("sk-ant-" + "x" * 40, "claude-3-sonnet-20240229")
            assert config2.provider() == "anthropic"

            # HuggingFace format
            config3 = graphbit.LlmConfig.huggingface("hf_" + "x" * 30, "gpt2")
            assert config3.provider() == "huggingface"

        except Exception as e:
            pytest.fail(f"Valid API key format validation failed: {e}")


class TestWorkflowValidation:
    """Comprehensive tests for workflow structure validation."""

    def test_empty_workflow_validation(self) -> None:
        """Test validation of empty workflows."""
        workflow = graphbit.Workflow("empty_test")

        # Empty workflow should be valid or raise specific error
        try:
            workflow.validate()
        except (ValueError, RuntimeError) as e:
            # If it fails, it should be for a specific reason
            error_msg = str(e).lower()
            assert any(word in error_msg for word in ["empty", "no nodes", "invalid"])

    def test_single_node_validation(self) -> None:
        """Test validation of single node workflows."""
        # Test different node types
        node_types = [graphbit.Node.agent("test_agent", "Process data", "agent_001"), graphbit.Node.transform("test_transform", "uppercase"), graphbit.Node.condition("test_condition", "value > 0")]

        for node in node_types:
            test_workflow = graphbit.Workflow(f"single_{node.name()}")
            test_workflow.add_node(node)

            with contextlib.suppress(ValueError, RuntimeError):
                test_workflow.validate()

    def test_circular_dependency_validation(self) -> None:
        """Test detection of circular dependencies in workflows."""
        workflow = graphbit.Workflow("circular_test")

        # Create nodes
        agent1 = graphbit.Node.agent("agent1", "First agent", "agent_001")
        agent2 = graphbit.Node.agent("agent2", "Second agent", "agent_002")

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)

        # Create circular dependency
        workflow.connect(id1, id2)
        workflow.connect(id2, id1)

        # This should raise an error
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            workflow.validate()

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["circular", "cycle", "dependency", "loop"])

    def test_invalid_connection_validation(self) -> None:
        """Test validation of invalid node connections."""
        workflow = graphbit.Workflow("invalid_connection")

        agent = graphbit.Node.agent("agent", "Test agent", "agent_001")
        workflow.add_node(agent)

        # Try to connect to non-existent node
        with pytest.raises((ValueError, RuntimeError)) as exc_info:
            workflow.connect(agent.id(), "non_existent_id")

        error_msg = str(exc_info.value).lower()
        assert any(word in error_msg for word in ["not found", "invalid", "missing", "unknown"])

    def test_self_connection_validation(self) -> None:
        """Test validation of nodes connecting to themselves."""
        workflow = graphbit.Workflow("self_connection")

        agent = graphbit.Node.agent("self_agent", "Self connecting agent", "agent_001")
        node_id = workflow.add_node(agent)

        # Try to connect node to itself
        try:
            workflow.connect(node_id, node_id)
            # If it doesn't raise an error, validate the workflow
            with pytest.raises((ValueError, RuntimeError)):
                workflow.validate()
        except (ValueError, RuntimeError):
            # Either connection creation or validation should fail
            pass

    def test_orphaned_nodes_validation(self) -> None:
        """Test validation of workflows with orphaned nodes."""
        workflow = graphbit.Workflow("orphaned_nodes")

        # Create connected chain
        agent1 = graphbit.Node.agent("agent1", "First agent", "agent_001")
        agent2 = graphbit.Node.agent("agent2", "Second agent", "agent_002")

        # Create orphaned node
        orphan = graphbit.Node.agent("orphan", "Orphaned agent", "orphan_001")

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)
        workflow.add_node(orphan)

        workflow.connect(id1, id2)

        # Validation may or may not allow orphaned nodes
        try:
            workflow.validate()
        except (ValueError, RuntimeError) as e:
            error_msg = str(e).lower()
            assert any(word in error_msg for word in ["orphan", "disconnected", "isolated", "unreachable"])


class TestParameterValidation:
    """Comprehensive tests for parameter validation across all components."""

    def test_llm_parameter_validation(self) -> None:
        """Test validation of LLM parameters."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Test invalid max_tokens
        with pytest.raises((ValueError, RuntimeError)):
            client.complete("Test", max_tokens=0)

        with pytest.raises((ValueError, RuntimeError)):
            client.complete("Test", max_tokens=-1)

        # Test invalid temperature
        with pytest.raises((ValueError, RuntimeError)):
            client.complete("Test", temperature=-1.0)

        with pytest.raises((ValueError, RuntimeError)):
            client.complete("Test", temperature=2.5)

    def test_executor_parameter_validation(self) -> None:
        """Test validation of executor parameters."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

        # Test invalid timeout values
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Executor(config, timeout_seconds=0)

        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Executor(config, timeout_seconds=4000)  # Too high

        # Test invalid specialized executor parameters
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Executor.new_low_latency(config, timeout_seconds=0)

        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Executor.new_low_latency(config, timeout_seconds=400)  # Too high for low latency

    def test_runtime_parameter_validation(self) -> None:
        """Test validation of runtime configuration parameters."""
        # Test invalid worker thread counts
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(worker_threads=0)

        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(max_blocking_threads=0)

        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(thread_stack_size_mb=0)

    def test_batch_parameter_validation(self) -> None:
        """Test validation of batch operation parameters."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Test empty batch
        with pytest.raises((ValueError, RuntimeError)):
            client.complete_batch([])

        # Test invalid concurrency
        with pytest.raises((ValueError, RuntimeError)):
            client.complete_batch(["test"], max_concurrency=0)


class TestNodeValidation:
    """Comprehensive tests for node creation and validation."""

    def test_agent_node_validation(self) -> None:
        """Test validation of agent node parameters."""
        # Test empty name
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Node.agent("", "Valid prompt", "agent_001")

        # Test empty prompt
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Node.agent("valid_name", "", "agent_001")

        # Test invalid agent ID format
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.Node.agent("test", "test prompt", "invalid id with spaces")

    def test_condition_node_validation(self) -> None:
        """Test validation of condition node parameters."""
        # Test empty name
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Node.condition("", "value > 0")

        # Test empty expression
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Node.condition("test_condition", "")

    def test_transform_node_validation(self) -> None:
        """Test validation of transform node parameters."""
        # Test empty name
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Node.transform("", "uppercase")

        # Test empty transformation
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.Node.transform("test_transform", "")


class TestEmbeddingValidation:
    """Comprehensive tests for embedding parameter validation."""

    def test_embedding_input_validation(self) -> None:
        """Test validation of embedding inputs."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.EmbeddingConfig.openai(api_key, "text-embedding-3-small")
        client = graphbit.EmbeddingClient(config)

        # Test empty text
        with pytest.raises((ValueError, RuntimeError)):
            client.embed("")

        # Test empty batch
        with pytest.raises((ValueError, RuntimeError)):
            client.embed_many([])

        # Test batch with empty strings
        with pytest.raises((ValueError, RuntimeError)):
            client.embed_many(["valid", "", "also valid"])

    def test_similarity_validation(self) -> None:
        """Test validation of similarity calculation inputs."""
        # Test empty vectors
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.EmbeddingClient.similarity([], [1.0, 2.0])

        with pytest.raises((ValueError, RuntimeError)):
            graphbit.EmbeddingClient.similarity([1.0, 2.0], [])

        # Test mismatched vector dimensions
        with pytest.raises((ValueError, RuntimeError)):
            graphbit.EmbeddingClient.similarity([1.0, 2.0], [1.0, 2.0, 3.0])


@pytest.mark.integration
class TestSystemValidation:
    """Integration tests for system-wide validation."""

    def test_initialization_validation(self) -> None:
        """Test validation during system initialization."""
        # Test invalid log levels
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.init(log_level="invalid_level")

    def test_cross_component_validation(self) -> None:
        """Test validation when components interact."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        # Create components
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        executor = graphbit.Executor(config)

        # Test executing invalid workflow
        invalid_workflow = graphbit.Workflow("invalid")

        with pytest.raises((ValueError, RuntimeError)):
            executor.execute(invalid_workflow)

    def test_resource_validation(self) -> None:
        """Test validation of resource constraints."""
        # Test system info validation
        system_info = graphbit.get_system_info()

        # Validate that system info contains expected structure
        assert isinstance(system_info, dict)
        assert "cpu_count" in system_info
        assert isinstance(system_info["cpu_count"], int)
        assert system_info["cpu_count"] > 0

        # Test health check validation
        health = graphbit.health_check()
        assert isinstance(health, dict)

        if "overall_healthy" in health:
            assert isinstance(health["overall_healthy"], bool)
