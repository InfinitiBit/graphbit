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
        return graphbit.LlmConfig.openai(api_key, "gpt-4")

    @pytest.fixture
    def openai_gpt35_config(self, api_key: str) -> Any:
        """Create OpenAI GPT-3.5 configuration."""
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def openai_client(self, openai_gpt4_config: Any) -> Any:
        """Create OpenAI client."""
        return graphbit.LlmClient(openai_gpt4_config)

    def test_openai_gpt4_config_creation(self, openai_gpt4_config: Any) -> None:
        """Test OpenAI GPT-4 config creation and properties."""
        assert openai_gpt4_config.provider() == "openai"
        assert openai_gpt4_config.model() == "gpt-4"

    def test_openai_gpt35_config_creation(self, openai_gpt35_config: Any) -> None:
        """Test OpenAI GPT-3.5 config creation and properties."""
        assert openai_gpt35_config.provider() == "openai"
        assert openai_gpt35_config.model() == "gpt-3.5-turbo"

    def test_openai_client_creation(self, openai_client: Any) -> None:
        """Test creating OpenAI client."""
        assert openai_client is not None

    def test_openai_executor_creation(self, openai_gpt4_config: Any) -> None:
        """Test creating workflow executor with OpenAI config."""
        executor = graphbit.Executor(openai_gpt4_config)
        assert executor is not None

    def test_openai_executor_configurations(self, openai_gpt4_config: Any) -> None:
        """Test different executor configurations with OpenAI."""
        # Test high throughput configuration
        executor_ht = graphbit.Executor.new_high_throughput(openai_gpt4_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = graphbit.Executor.new_low_latency(openai_gpt4_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = graphbit.Executor.new_memory_optimized(openai_gpt4_config)
        assert executor_mo is not None

    def test_openai_simple_completion(self, openai_client: Any) -> None:
        """Test simple text completion with OpenAI."""
        try:
            response = openai_client.complete("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            pytest.fail(f"OpenAI completion failed: {e}")

    def test_openai_simple_workflow_execution(self, openai_gpt4_config: Any) -> None:
        """Test executing a simple workflow with OpenAI."""
        # Create a simple workflow
        workflow = graphbit.Workflow("test_openai_workflow")

        # Create a simple agent node
        try:
            agent_node = graphbit.Node.agent("test_agent", "Respond with 'Hello from OpenAI'", "agent_001")
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = graphbit.Executor(openai_gpt4_config)
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
        return graphbit.LlmConfig.anthropic(api_key, "claude-3-sonnet-20240229")

    @pytest.fixture
    def anthropic_client(self, anthropic_config: Any) -> Any:
        """Create Anthropic client."""
        return graphbit.LlmClient(anthropic_config)

    def test_anthropic_config_creation(self, anthropic_config: Any) -> None:
        """Test Anthropic config creation and properties."""
        assert anthropic_config.provider() == "anthropic"
        assert anthropic_config.model() == "claude-3-sonnet-20240229"

    def test_anthropic_client_creation(self, anthropic_client: Any) -> None:
        """Test creating Anthropic client."""
        assert anthropic_client is not None

    def test_anthropic_executor_creation(self, anthropic_config: Any) -> None:
        """Test creating workflow executor with Anthropic config."""
        executor = graphbit.Executor(anthropic_config)
        assert executor is not None

    def test_anthropic_executor_configurations(self, anthropic_config: Any) -> None:
        """Test different executor configurations with Anthropic."""
        # Test high throughput configuration
        executor_ht = graphbit.Executor.new_high_throughput(anthropic_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = graphbit.Executor.new_low_latency(anthropic_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = graphbit.Executor.new_memory_optimized(anthropic_config)
        assert executor_mo is not None

    def test_anthropic_simple_completion(self, anthropic_client: Any) -> None:
        """Test simple text completion with Anthropic."""
        try:
            response = anthropic_client.complete("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            pytest.fail(f"Anthropic completion failed: {e}")

    def test_anthropic_simple_workflow_execution(self, anthropic_config: Any) -> None:
        """Test executing a simple workflow with Anthropic."""
        # Create a simple workflow
        workflow = graphbit.Workflow("test_anthropic_workflow")

        # Create a simple agent node
        try:
            agent_node = graphbit.Node.agent("test_agent", "Respond with 'Hello from Claude'", "agent_002")
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = graphbit.Executor(anthropic_config)
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
        return graphbit.LlmConfig.huggingface(api_key, "microsoft/DialoGPT-medium")

    @pytest.fixture
    def hf_client(self, hf_config: Any) -> Any:
        """Create HuggingFace client."""
        return graphbit.LlmClient(hf_config)

    def test_hf_config_creation(self, hf_config: Any) -> None:
        """Test HuggingFace config creation and properties."""
        assert hf_config.provider() == "huggingface"
        assert hf_config.model() == "microsoft/DialoGPT-medium"

    def test_hf_client_creation(self, hf_client: Any) -> None:
        """Test creating HuggingFace client."""
        assert hf_client is not None

    def test_hf_executor_creation(self, hf_config: Any) -> None:
        """Test creating workflow executor with HuggingFace config."""
        executor = graphbit.Executor(hf_config)
        assert executor is not None

    def test_hf_executor_configurations(self, hf_config: Any) -> None:
        """Test different executor configurations with HuggingFace."""
        # Test high throughput configuration
        executor_ht = graphbit.Executor.new_high_throughput(hf_config)
        assert executor_ht is not None

        # Test low latency configuration
        executor_ll = graphbit.Executor.new_low_latency(hf_config)
        assert executor_ll is not None

        # Test memory optimized configuration
        executor_mo = graphbit.Executor.new_memory_optimized(hf_config)
        assert executor_mo is not None

    def test_hf_simple_completion(self, hf_client: Any) -> None:
        """Test simple text completion with HuggingFace."""
        try:
            response = hf_client.complete("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            pytest.fail(f"HuggingFace completion failed: {e}")

    def test_hf_simple_workflow_execution(self, hf_config: Any) -> None:
        """Test executing a simple workflow with HuggingFace."""
        # Create a simple workflow
        workflow = graphbit.Workflow("test_hf_workflow")

        # Create a simple agent node
        try:
            agent_node = graphbit.Node.agent("test_agent", "Respond with 'Hello from HuggingFace'", "agent_003")
            node_id = workflow.add_node(agent_node)
            assert node_id is not None

            # Validate workflow
            workflow.validate()

            # Create executor and test basic execution capability
            executor = graphbit.Executor(hf_config)
            assert executor is not None

        except Exception as e:
            pytest.fail(f"HuggingFace workflow creation/validation failed: {e}")


class TestOllamaLLM:
    """Integration tests for Ollama LLM models."""

    @pytest.fixture
    def ollama_config(self) -> Any:
        """Create Ollama configuration."""
        return graphbit.LlmConfig.ollama("llama3.2")

    @pytest.fixture
    def ollama_client(self, ollama_config: Any) -> Any:
        """Create Ollama client."""
        return graphbit.LlmClient(ollama_config)

    def test_ollama_config_creation(self, ollama_config: Any) -> None:
        """Test Ollama config creation and properties."""
        assert ollama_config.provider() == "ollama"
        assert ollama_config.model() == "llama3.2"

    def test_ollama_client_creation(self, ollama_client: Any) -> None:
        """Test creating Ollama client."""
        assert ollama_client is not None

    def test_ollama_executor_creation(self, ollama_config: Any) -> None:
        """Test creating workflow executor with Ollama config."""
        executor = graphbit.Executor(ollama_config)
        assert executor is not None

    @pytest.mark.skip(reason="Requires local Ollama installation")
    def test_ollama_simple_completion(self, ollama_client: Any) -> None:
        """Test simple text completion with Ollama."""
        try:
            response = ollama_client.complete("Hello, world!", max_tokens=10)
            assert isinstance(response, str)
            assert len(response) > 0
        except Exception as e:
            pytest.fail(f"Ollama completion failed: {e}")


@pytest.mark.integration
class TestCrossProviderLLM:
    """Integration tests across multiple LLM providers."""

    @pytest.fixture
    def providers(self) -> Any:
        """Create configurations for available providers."""
        configs = {}

        if os.getenv("OPENAI_API_KEY"):
            configs["openai"] = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-3.5-turbo")

        if os.getenv("ANTHROPIC_API_KEY"):
            configs["anthropic"] = graphbit.LlmConfig.anthropic(os.getenv("ANTHROPIC_API_KEY"), "claude-3-sonnet-20240229")

        configs["ollama"] = graphbit.LlmConfig.ollama("llama3.2")

        return configs

    def test_multi_provider_executor_creation(self, providers: Any) -> None:
        """Test creating executors with multiple providers."""
        executors = {}

        for provider_name, config in providers.items():
            try:
                executor = graphbit.Executor(config)
                executors[provider_name] = executor
                assert executor is not None
            except Exception as e:
                pytest.fail(f"Failed to create executor for {provider_name}: {e}")

        # At least one executor should be created successfully
        assert len(executors) > 0

    def test_multi_provider_client_creation(self, providers: Any) -> None:
        """Test creating clients with multiple providers."""
        clients = {}

        for provider_name, config in providers.items():
            try:
                client = graphbit.LlmClient(config)
                clients[provider_name] = client
                assert client is not None
            except Exception as e:
                pytest.fail(f"Failed to create client for {provider_name}: {e}")

        # At least one client should be created successfully
        assert len(clients) > 0

    def test_cross_provider_configuration_compatibility(self, providers: Any) -> None:
        """Test configuration compatibility across providers."""
        for _provider_name, config in providers.items():
            # Test that all providers support basic configuration methods
            assert hasattr(config, "provider")
            assert hasattr(config, "model")

            provider = config.provider()
            model = config.model()

            assert isinstance(provider, str)
            assert isinstance(model, str)
            assert len(provider) > 0
            assert len(model) > 0


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run specific test cases
    pytest.main([__file__, "-v"])
