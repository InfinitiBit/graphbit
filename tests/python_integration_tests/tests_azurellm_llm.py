"""Azure LLM Integration Tests.

Integration tests for Azure LLM provider that mirror OpenAI test coverage.
These tests require actual Azure LLM credentials.
"""

import asyncio
import os
from typing import Optional, Tuple

import pytest

from graphbit import LlmClient, LlmConfig


def get_azurellm_credentials() -> Optional[Tuple[str, str, str, str]]:
    """Get Azure LLM credentials from environment variables."""
    api_key = os.getenv("AZURELLM_API_KEY")
    endpoint = os.getenv("AZURELLM_ENDPOINT")
    deployment = os.getenv("AZURELLM_DEPLOYMENT")
    api_version = os.getenv("AZURELLM_API_VERSION", "2024-10-21")

    if api_key and endpoint and deployment:
        return api_key, endpoint, deployment, api_version
    return None


def has_azurellm_credentials() -> bool:
    """Check if Azure LLM credentials are available."""
    return get_azurellm_credentials() is not None


@pytest.fixture(scope="session")
def azure_credentials():
    """Session-scoped fixture for Azure LLM credentials."""
    credentials = get_azurellm_credentials()
    if not credentials:
        pytest.skip("Azure LLM credentials not found. Set AZURELLM_API_KEY, AZURELLM_ENDPOINT, and AZURELLM_DEPLOYMENT environment variables.")
    return credentials


@pytest.fixture(scope="session")
def azure_config(azure_credentials):
    """Session-scoped fixture for Azure LLM configuration."""
    api_key, endpoint, deployment, api_version = azure_credentials
    return LlmConfig.azurellm(api_key=api_key, deployment_name=deployment, endpoint=endpoint, api_version=api_version)


@pytest.fixture
def azure_client(azure_config):
    """Create Azure LLM client."""
    return LlmClient(azure_config)


class TestAzureLlmBasicFunctionality:
    """Test basic Azure LLM functionality."""

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_simple_completion(self, azure_client):
        """Test simple text completion with Azure LLM."""
        response = azure_client.complete("Say 'Hello' in one word only.", max_tokens=10, temperature=0.0)
        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Azure LLM simple completion: {response}")

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    @pytest.mark.asyncio
    async def test_azurellm_async_completion(self, azure_config):
        """Test async completion with Azure LLM."""
        client = LlmClient(azure_config)
        response = await client.complete_async("Say 'Hello' in one word only.", max_tokens=10, temperature=0.0)
        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Azure LLM async completion: {response}")

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_conversation(self, azure_client):
        """Test Azure LLM conversation handling."""
        # First message
        response1 = azure_client.complete("My name is Alice.", max_tokens=50, temperature=0.1)
        assert isinstance(response1, str)

        # Follow-up message (this is a simple test, real conversation would need message history)
        response2 = azure_client.complete(f"previous chat: user: My name is Alice. assistant: {response1} query: What's my name?", max_tokens=50, temperature=0.1)
        assert isinstance(response2, str)
        assert "alice" in response2.lower()
        print(f"Azure LLM conversation: {response2}")


class TestAzureLlmAdvancedFeatures:
    """Test advanced Azure LLM features."""

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    @pytest.mark.asyncio
    async def test_azurellm_streaming(self, azure_client):
        """Test Azure LLM streaming completion."""
        response = await azure_client.complete_stream("Tell me a short story about a robot.", max_tokens=100, temperature=0.7)
        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Azure LLM streaming: {response}")

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    @pytest.mark.asyncio
    async def test_azurellm_batch_processing(self, azure_client):
        """Test Azure LLM batch processing."""
        prompts = ["Say 'A' in one word.", "Say 'B' in one word.", "Say 'C' in one word."]

        responses = await azure_client.complete_batch(prompts, max_tokens=5, max_concurrency=2, temperature=0.0)

        assert isinstance(responses, list)
        assert len(responses) == 3
        assert all(isinstance(r, str) and len(r) > 0 for r in responses)
        print(f"Azure LLM batch responses: {responses}")

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    @pytest.mark.asyncio
    async def test_azurellm_chat_optimized(self, azure_client):
        """Test Azure LLM chat optimization."""
        messages = [("user", "Hello, how are you?"), ("assistant", "I'm doing well, thank you!"), ("user", "What's the weather like?")]

        response = await azure_client.chat_optimized(messages, max_tokens=50, temperature=0.1)

        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Azure LLM chat optimized: {response}")


class TestAzureLlmParameterTesting:
    """Test Azure LLM with different parameters."""

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_temperature_variations(self, azure_client):
        """Test Azure LLM with different temperature settings."""
        prompt = "Tell me a creative story in one sentence."

        # Low temperature (more deterministic)
        response_low = azure_client.complete(prompt, max_tokens=50, temperature=0.1)

        # High temperature (more creative)
        response_high = azure_client.complete(prompt, max_tokens=50, temperature=0.9)

        assert isinstance(response_low, str)
        assert isinstance(response_high, str)
        assert len(response_low) > 0
        assert len(response_high) > 0

        print(f"Low temperature: {response_low}")
        print(f"High temperature: {response_high}")

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_max_tokens_variations(self, azure_client):
        """Test Azure LLM with different max_tokens settings."""
        prompt = "Explain quantum computing."

        # Short response
        response_short = azure_client.complete(prompt, max_tokens=20, temperature=0.1)

        # Longer response
        response_long = azure_client.complete(prompt, max_tokens=100, temperature=0.1)

        assert isinstance(response_short, str)
        assert isinstance(response_long, str)
        assert len(response_short) > 0
        assert len(response_long) > 0
        assert len(response_long) > len(response_short)

        print(f"Short response: {response_short}")
        print(f"Long response: {response_long}")


class TestAzureLlmErrorHandling:
    """Test Azure LLM error handling."""

    def test_azurellm_invalid_credentials(self):
        """Test Azure LLM with invalid credentials."""
        config = LlmConfig.azurellm(api_key="invalid-key-that-is-long-enough-for-validation", deployment_name="invalid-deployment", endpoint="https://invalid.openai.azure.com")

        client = LlmClient(config)

        with pytest.raises(Exception, match="(?i)(error|failed|invalid|unauthorized|forbidden)"):
            client.complete("Hello, Azure LLM!", max_tokens=50)

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_empty_prompt(self, azure_client):
        """Test Azure LLM with empty prompt."""
        with pytest.raises(Exception, match="(?i)(empty|invalid|error)"):
            azure_client.complete("", max_tokens=50)

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_invalid_max_tokens(self, azure_client):
        """Test Azure LLM with invalid max_tokens."""
        with pytest.raises(Exception, match="(?i)(invalid|negative|error)"):
            azure_client.complete("Hello, Azure LLM!", max_tokens=-1)


class TestAzureLlmPerformance:
    """Test Azure LLM performance characteristics."""

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    @pytest.mark.asyncio
    async def test_azurellm_concurrent_requests(self, azure_config):
        """Test Azure LLM with concurrent requests."""

        async def make_request(prompt_suffix):
            client = LlmClient(azure_config)
            return await client.complete_async(f"Say 'Hello {prompt_suffix}' in one sentence.", max_tokens=20, temperature=0.1)

        # Make 3 concurrent requests
        tasks = [make_request("World"), make_request("Azure"), make_request("LLM")]

        responses = await asyncio.gather(*tasks)

        assert len(responses) == 3
        assert all(isinstance(r, str) and len(r) > 0 for r in responses)

        for i, response in enumerate(responses):
            print(f"Concurrent response {i+1}: {response}")

    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_response_time(self, azure_client):
        """Test Azure LLM response time (basic timing)."""
        import time

        start_time = time.time()
        response = azure_client.complete("Say 'Hello' in one word.", max_tokens=5, temperature=0.0)
        end_time = time.time()

        response_time = end_time - start_time

        assert isinstance(response, str)
        assert len(response) > 0
        assert response_time < 30  # Should respond within 30 seconds

        print(f"Azure LLM response time: {response_time:.2f} seconds")
        print(f"Response: {response}")


class TestAzureLlmComparison:
    """Test Azure LLM compared to other providers."""

    def test_azurellm_vs_openai_interface(self):
        """Test that Azure LLM has same interface as OpenAI."""
        # Create configs for both providers
        azure_config = LlmConfig.azurellm(api_key="test-key-that-is-long-enough-for-validation", deployment_name="gpt-4o-mini", endpoint="https://test.openai.azure.com")

        openai_config = LlmConfig.openai(api_key="test-key-that-is-long-enough-for-validation", model="gpt-4o")

        # Create clients
        azure_client = LlmClient(azure_config)
        openai_client = LlmClient(openai_config)

        # Both should have the same methods
        assert hasattr(azure_client, "complete")
        assert hasattr(openai_client, "complete")
        assert hasattr(azure_client, "complete_async")
        assert hasattr(openai_client, "complete_async")
        assert hasattr(azure_client, "complete_stream")
        assert hasattr(openai_client, "complete_stream")
        assert hasattr(azure_client, "complete_batch")
        assert hasattr(openai_client, "complete_batch")
        assert hasattr(azure_client, "chat_optimized")
        assert hasattr(openai_client, "chat_optimized")

    def test_azurellm_provider_identification(self, azure_config):
        """Test that Azure LLM is correctly identified as a provider."""
        assert azure_config.provider() == "azurellm"

        # Should be different from OpenAI
        openai_config = LlmConfig.openai(api_key="test-key-that-is-long-enough-for-validation", model="gpt-4o")
        assert openai_config.provider() == "openai"
        assert azure_config.provider() != openai_config.provider()


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
