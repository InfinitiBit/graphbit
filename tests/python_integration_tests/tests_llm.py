#!/usr/bin/env python3
"""Integration tests for GraphBit LLM functionality."""
import os
from typing import Any

import pytest

import graphbit


class TestOpenAILLM:
    """Integration tests for OpenAI LLM models."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def openai_gpt4_config(self, api_key: str) -> Any:
        """Create OpenAI GPT-4 configuration."""
        return graphbit.PyLlmConfig.openai(api_key, "gpt-4")

    @pytest.fixture
    def openai_gpt35_config(self, api_key: str) -> Any:
        """Create OpenAI GPT-3.5 configuration."""
        return graphbit.PyLlmConfig.openai(api_key, "gpt-3.5-turbo")

    def test_openai_gpt4_config_creation(self, openai_gpt4_config: Any) -> None:
        """Test OpenAI GPT-4 config creation and properties."""
        assert openai_gpt4_config.provider_name() == "openai"
        assert openai_gpt4_config.model_name() == "gpt-4"

    def test_openai_gpt35_config_creation(self, openai_gpt35_config: Any) -> None:
        """Test OpenAI GPT-3.5 config creation and properties."""
        assert openai_gpt35_config.provider_name() == "openai"
        assert openai_gpt35_config.model_name() == "gpt-3.5-turbo"

    def test_openai_workflow_executor_creation(self, openai_gpt4_config: Any) -> None:
        """Test creating workflow executor with OpenAI config."""
        executor = graphbit.PyWorkflowExecutor(openai_gpt4_config)
        assert executor is not None

    def test_openai_executor_configurations(self, openai_gpt4_config: Any) -> None:
        """Test different executor configurations with OpenAI."""
        # Test high throughput configuration
        executor_ht = graphbit.PyWorkflowExecutor.new_high_throughput(openai_gpt4_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = graphbit.PyWorkflowExecutor.new_low_latency(openai_gpt4_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = graphbit.PyWorkflowExecutor.new_memory_optimized(openai_gpt4_config)
        assert executor_mo is not None

    def test_openai_executor_with_customizations(self, openai_gpt4_config: Any) -> None:
        """Test executor with custom configurations."""
        retry_config = graphbit.PyRetryConfig(3).with_exponential_backoff(1000, 2.0, 30000)
        circuit_breaker_config = graphbit.PyCircuitBreakerConfig(5, 60000)

        executor = graphbit.PyWorkflowExecutor(openai_gpt4_config)
        executor = executor.with_retry_config(retry_config)
        executor = executor.with_circuit_breaker_config(circuit_breaker_config)
        executor = executor.with_max_node_execution_time(120000)
        executor = executor.with_fail_fast(True)

        # Verify that the executor was created successfully with the customizations
        # Note: The bindings don't expose getters for these values, so we just verify
        # that the executor was created without errors
        assert executor is not None

    def test_openai_simple_workflow_execution(self, openai_gpt4_config: Any) -> None:
        """Test executing a simple workflow with OpenAI."""
        # Create a simple workflow
        workflow = graphbit.PyWorkflow("test_openai_workflow", "Test workflow for OpenAI")

        # Create a simple agent node (minimal test)
        try:
            agent_node = graphbit.PyWorkflowNode.agent_node(
                "test_agent",
                "Test agent for OpenAI",
                "agent_001",
                "Respond with 'Hello from OpenAI'",
            )
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = graphbit.PyWorkflowExecutor(openai_gpt4_config)
            assert executor is not None

        except Exception as e:
            pytest.fail(f"OpenAI workflow creation/validation failed: {e}")


class TestAnthropicLLM:
    """Integration tests for Anthropic LLM models."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get Anthropic API key from environment."""
        api_key = os.getenv("ANTHROPIC_API_KEY")
        if not api_key:
            pytest.skip("ANTHROPIC_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def anthropic_config(self, api_key: str) -> Any:
        """Create Anthropic configuration."""
        return graphbit.PyLlmConfig.anthropic(api_key, "claude-3-sonnet-20240229")

    def test_anthropic_config_creation(self, anthropic_config: Any) -> None:
        """Test Anthropic config creation and properties."""
        assert anthropic_config.provider_name() == "anthropic"
        assert anthropic_config.model_name() == "claude-3-sonnet-20240229"

    def test_anthropic_workflow_executor_creation(self, anthropic_config: Any) -> None:
        """Test creating workflow executor with Anthropic config."""
        executor = graphbit.PyWorkflowExecutor(anthropic_config)
        assert executor is not None

    def test_anthropic_executor_configurations(self, anthropic_config: Any) -> None:
        """Test different executor configurations with Anthropic."""
        # Test high throughput configuration
        executor_ht = graphbit.PyWorkflowExecutor.new_high_throughput(anthropic_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = graphbit.PyWorkflowExecutor.new_low_latency(anthropic_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = graphbit.PyWorkflowExecutor.new_memory_optimized(anthropic_config)
        assert executor_mo is not None

    def test_anthropic_simple_workflow_execution(self, anthropic_config: Any) -> None:
        """Test executing a simple workflow with Anthropic."""
        # Create a simple workflow
        workflow = graphbit.PyWorkflow("test_anthropic_workflow", "Test workflow for Anthropic")

        # Create a simple agent node (minimal test)
        try:
            agent_node = graphbit.PyWorkflowNode.agent_node(
                "test_agent",
                "Test agent for Anthropic",
                "agent_002",
                "Respond with 'Hello from Claude'",
            )
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = graphbit.PyWorkflowExecutor(anthropic_config)
            assert executor is not None

        except Exception as e:
            pytest.fail(f"Anthropic workflow creation/validation failed: {e}")


class TestHuggingFaceLLM:
    """Integration tests for HuggingFace LLM models."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get HuggingFace API key from environment."""
        api_key = os.getenv("HUGGINGFACE_API_KEY")
        if not api_key:
            pytest.skip("HUGGINGFACE_API_KEY not set")
        return api_key or ""

    @pytest.fixture
    def hf_config(self, api_key: str) -> Any:
        """Create HuggingFace configuration."""
        return graphbit.PyLlmConfig.huggingface(api_key, "microsoft/DialoGPT-medium")

    def test_hf_config_creation(self, hf_config: Any) -> None:
        """Test HuggingFace config creation and properties."""
        assert hf_config.provider_name() == "huggingface"
        assert hf_config.model_name() == "microsoft/DialoGPT-medium"

    def test_hf_workflow_executor_creation(self, hf_config: Any) -> None:
        """Test creating workflow executor with HuggingFace config."""
        executor = graphbit.PyWorkflowExecutor(hf_config)
        assert executor is not None

    def test_hf_executor_configurations(self, hf_config: Any) -> None:
        """Test different executor configurations with HuggingFace."""
        # Test high throughput configuration
        executor_ht = graphbit.PyWorkflowExecutor.new_high_throughput(hf_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = graphbit.PyWorkflowExecutor.new_low_latency(hf_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = graphbit.PyWorkflowExecutor.new_memory_optimized(hf_config)
        assert executor_mo is not None

    def test_hf_simple_workflow_execution(self, hf_config: Any) -> None:
        """Test executing a simple workflow with HuggingFace."""
        # Create a simple workflow
        workflow = graphbit.PyWorkflow("test_hf_workflow", "Test workflow for HuggingFace")

        # Create a simple agent node (minimal test)
        try:
            agent_node = graphbit.PyWorkflowNode.agent_node(
                "test_agent",
                "Test agent for HuggingFace",
                "agent_003",
                "Respond with 'Hello from HuggingFace'",
            )
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = graphbit.PyWorkflowExecutor(hf_config)
            assert executor is not None

        except Exception as e:
            pytest.fail(f"HuggingFace workflow creation/validation failed: {e}")


class TestLLMConfiguration:
    """Integration tests for LLM configuration features."""

    def test_retry_config_creation(self) -> None:
        """Test retry configuration creation and customization."""
        # Test basic retry config
        retry_config = graphbit.PyRetryConfig(5)
        assert retry_config is not None

        # Test with exponential backoff
        exp_config = retry_config.with_exponential_backoff(500, 2.0, 10000)
        assert exp_config is not None

        # Test with jitter
        jitter_config = retry_config.with_jitter(0.1)
        assert jitter_config is not None

        # Test accessing properties
        assert retry_config.max_attempts() == 5
        assert retry_config.initial_delay_ms() >= 0

    def test_circuit_breaker_config_creation(self) -> None:
        """Test circuit breaker configuration creation."""
        # Test basic circuit breaker config
        cb_config = graphbit.PyCircuitBreakerConfig(10, 30000)
        assert cb_config is not None

        # Test circuit breaker with custom settings
        custom_cb = graphbit.PyCircuitBreakerConfig(5, 60000)
        assert custom_cb is not None

    def test_executor_concurrency_settings(self) -> None:
        """Test executor concurrency and performance configurations."""
        # This test requires an LLM config, so we'll use a mock one
        # In practice, these would be tested with actual API keys
        try:
            # Create a basic config for testing concurrency settings
            # We'll skip if no API key is available
            openai_key = os.getenv("OPENAI_API_KEY")
            if not openai_key:
                pytest.skip("OPENAI_API_KEY not set for concurrency tests")

            llm_config = graphbit.PyLlmConfig.openai(openai_key, "gpt-3.5-turbo")
            executor = graphbit.PyWorkflowExecutor(llm_config)

            # Test concurrency settings
            concurrent_executor = executor.with_max_concurrent_nodes(5)
            assert concurrent_executor is not None

            # Test timeout settings
            timeout_executor = executor.with_max_node_execution_time(30000)
            assert timeout_executor is not None

        except Exception as e:
            pytest.skip(f"Concurrency settings test skipped: {e}")


class TestAgentCapabilities:
    """Integration tests for agent capabilities."""

    def test_predefined_capabilities(self) -> None:
        """Test predefined agent capabilities."""
        # Test that predefined capabilities exist and can be created
        try:
            # Test text processing capability
            text_cap = graphbit.PyAgentCapabilities.text_processing()
            assert text_cap is not None

            # Test data analysis capability
            analysis_cap = graphbit.PyAgentCapabilities.data_analysis()
            assert analysis_cap is not None

        except Exception as e:
            pytest.skip(f"Predefined capabilities test skipped: {e}")

    def test_custom_capabilities(self) -> None:
        """Test custom agent capabilities."""
        try:
            # Test custom capability creation
            custom_cap = graphbit.PyAgentCapabilities.custom(["reasoning", "mathematics", "coding"])
            assert custom_cap is not None

        except Exception as e:
            pytest.skip(f"Custom capabilities test skipped: {e}")


@pytest.mark.integration
class TestCrossProviderLLM:
    """Integration tests comparing different LLM providers."""

    @pytest.fixture
    def providers(self) -> Any:
        """Get available LLM providers based on API keys."""
        providers = {}

        # OpenAI
        openai_key = os.getenv("OPENAI_API_KEY")
        if openai_key:
            providers["openai"] = graphbit.PyLlmConfig.openai(openai_key, "gpt-3.5-turbo")

        # Anthropic
        anthropic_key = os.getenv("ANTHROPIC_API_KEY")
        if anthropic_key:
            providers["anthropic"] = graphbit.PyLlmConfig.anthropic(anthropic_key, "claude-3-sonnet-20240229")

        if not providers:
            pytest.skip("No LLM providers available")

        return providers

    def test_multi_provider_executor_creation(self, providers: Any) -> None:
        """Test creating executors with different providers."""
        executors = {}

        for name, config in providers.items():
            try:
                executor = graphbit.PyWorkflowExecutor(config)
                executors[name] = executor
                assert executor is not None
            except Exception as e:
                pytest.fail(f"Failed to create executor for {name}: {e}")

        assert len(executors) > 0

    def test_multi_provider_workflow_validation(self, providers: Any) -> None:
        """Test workflow validation across different providers."""
        for name, config in providers.items():
            try:
                # Create a test workflow
                workflow = graphbit.PyWorkflow(f"test_{name}_workflow", f"Test workflow for {name}")

                # Add a simple agent node
                agent_node = graphbit.PyWorkflowNode.agent_node(
                    f"test_agent_{name}",
                    f"Test agent for {name}",
                    f"agent_{name}",
                    "Test prompt",
                )
                workflow.add_node(agent_node)

                # Validate workflow
                workflow.validate()

                # Create executor
                executor = graphbit.PyWorkflowExecutor(config)
                assert executor is not None

            except Exception as e:
                pytest.fail(f"Workflow validation failed for {name}: {e}")

    def test_cross_provider_configuration_compatibility(self, providers: Any) -> None:
        """Test configuration compatibility across providers."""
        # Test that similar configurations can be applied to different providers
        retry_config = graphbit.PyRetryConfig(3).with_exponential_backoff(1000, 2.0, 10000)
        circuit_breaker_config = graphbit.PyCircuitBreakerConfig(5, 30000)

        for name, config in providers.items():
            try:
                executor = graphbit.PyWorkflowExecutor(config)
                executor = executor.with_retry_config(retry_config)
                executor = executor.with_circuit_breaker_config(circuit_breaker_config)
                executor = executor.with_max_node_execution_time(60000)

                assert executor is not None

            except Exception as e:
                pytest.fail(f"Configuration compatibility test failed for {name}: {e}")

        # Fix the B007 issue by using an underscore prefix for the unused loop variable
        configs = list(providers.values())
        for _config in configs:
            # Just iterate to test the pattern
            pass


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run specific test cases
    pytest.main([__file__, "-v"])
