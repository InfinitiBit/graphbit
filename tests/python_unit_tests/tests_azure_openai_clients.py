"""Azure OpenAI LLM Client Unit Tests.

Comprehensive unit tests for Azure OpenAI provider that mirror OpenAI test coverage.
"""

import os

import pytest

from graphbit import LlmClient, LlmConfig


def get_azure_openai_test_config():
    """Get test Azure OpenAI configuration with long enough API key."""
    return {"api_key": "test-azure-openai-api-key-that-is-long-enough-for-validation", "deployment_name": "gpt-4o-mini", "endpoint": "https://test.openai.azure.com", "api_version": "2024-10-21"}


def get_azure_openai_credentials():
    """Get Azure OpenAI credentials from environment variables."""
    api_key = os.getenv("AZURE_OPENAI_API_KEY")
    endpoint = os.getenv("AZURE_OPENAI_ENDPOINT")
    deployment = os.getenv("AZURE_OPENAI_DEPLOYMENT")
    api_version = os.getenv("AZURE_OPENAI_API_VERSION", "2024-10-21")

    if api_key and endpoint and deployment:
        return api_key, endpoint, deployment, api_version
    return None


def has_azure_openai_credentials():
    """Check if Azure OpenAI credentials are available."""
    return get_azure_openai_credentials() is not None


class TestAzureOpenAIConfig:
    """Test Azure OpenAI configuration classes."""

    def test_azure_openai_config_creation(self):
        """Test creating Azure OpenAI LLM configuration."""
        config_data = get_azure_openai_test_config()
        config = LlmConfig.azure_openai(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])
        assert config is not None
        assert config.provider() == "azure_openai"
        assert config.model() == config_data["deployment_name"]

    def test_azure_openai_config_with_api_version(self):
        """Test creating Azure OpenAI configuration with custom API version."""
        config_data = get_azure_openai_test_config()
        config = LlmConfig.azure_openai(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"], api_version="2024-06-01")
        assert config is not None
        assert config.provider() == "azure_openai"
        assert config.model() == config_data["deployment_name"]

    def test_azure_openai_config_different_deployments(self):
        """Test Azure OpenAI configuration with different deployment names."""
        deployments = ["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"]
        config_data = get_azure_openai_test_config()

        for deployment in deployments:
            config = LlmConfig.azure_openai(api_key=config_data["api_key"], deployment_name=deployment, endpoint=config_data["endpoint"])
            assert config is not None
            assert config.provider() == "azure_openai"
            assert config.model() == deployment

    def test_azure_openai_config_empty_api_key(self):
        """Test that empty API key raises validation error."""
        config_data = get_azure_openai_test_config()
        with pytest.raises(ValueError, match="API key cannot be empty"):
            LlmConfig.azure_openai(api_key="", deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

    def test_azure_openai_config_whitespace_api_key(self):
        """Test that whitespace-only API key raises validation error."""
        config_data = get_azure_openai_test_config()
        with pytest.raises(ValueError, match="Azure OpenAI API key too short"):
            LlmConfig.azure_openai(api_key="   ", deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

    def test_azure_openai_config_short_api_key(self):
        """Test that short API key raises validation error."""
        config_data = get_azure_openai_test_config()
        with pytest.raises(ValueError, match="Azure OpenAI API key too short"):
            LlmConfig.azure_openai(api_key="short", deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])


class TestAzureOpenAIClient:
    """Test Azure OpenAI client functionality."""

    def test_azure_openai_client_creation(self):
        """Test creating Azure OpenAI client."""
        config_data = get_azure_openai_test_config()
        config = LlmConfig.azure_openai(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

        client = LlmClient(config)
        assert client is not None

    def test_azure_openai_client_with_debug(self):
        """Test creating Azure OpenAI client with debug mode."""
        config_data = get_azure_openai_test_config()
        config = LlmConfig.azure_openai(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

        client = LlmClient(config, debug=True)
        assert client is not None

    def test_azure_openai_client_creation_invalid_config(self):
        """Test creating Azure OpenAI client with invalid configuration."""
        with pytest.raises((ValueError, TypeError)):
            LlmClient("invalid_config")

    @pytest.mark.asyncio
    async def test_azure_openai_client_complete_no_credentials(self):
        """Test Azure OpenAI completion without real credentials (should fail)."""
        config_data = get_azure_openai_test_config()
        config = LlmConfig.azure_openai(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])
        client = LlmClient(config)

        # This should fail because we're using test credentials
        with pytest.raises(Exception, match="(?i)(error|failed|invalid|unauthorized)"):
            await client.complete_async("Test prompt")


class TestAzureOpenAIIntegration:
    """Integration tests for Azure OpenAI provider.

    These tests require actual Azure OpenAI credentials and should be run with:
    AZURE_OPENAI_API_KEY=your_key AZURE_OPENAI_ENDPOINT=your_endpoint AZURE_OPENAI_DEPLOYMENT=your_deployment pytest -m integration
    """

    @pytest.fixture
    def azure_credentials(self):
        """Get Azure OpenAI credentials from environment variables."""
        credentials = get_azure_openai_credentials()
        if not credentials:
            pytest.skip("Azure OpenAI credentials not found. Set AZURE_OPENAI_API_KEY, AZURE_OPENAI_ENDPOINT, and AZURE_OPENAI_DEPLOYMENT environment variables.")
        return credentials

    @pytest.fixture
    def azure_config(self, azure_credentials):
        """Create Azure OpenAI configuration."""
        api_key, endpoint, deployment, api_version = azure_credentials
        return LlmConfig.azure_openai(api_key=api_key, deployment_name=deployment, endpoint=endpoint, api_version=api_version)

    @pytest.fixture
    def azure_client(self, azure_config):
        """Create Azure OpenAI client."""
        return LlmClient(azure_config)

    @pytest.mark.integration
    def test_azure_openai_simple_completion(self, azure_client):
        """Test simple text completion with Azure OpenAI."""
        try:
            response = azure_client.complete("Hello, Azure OpenAI! Please respond with a simple greeting.", max_tokens=50)
            assert isinstance(response, str)
            assert len(response) > 0
            print(f"Azure OpenAI Response: {response}")
        except Exception as e:
            pytest.fail(f"Azure OpenAI completion failed: {e}")

    @pytest.mark.integration
    @pytest.mark.asyncio
    async def test_azure_openai_async_completion(self, azure_config):
        """Test async completion with Azure OpenAI."""
        client = LlmClient(azure_config)
        try:
            response = await client.complete_async("Hello, Azure OpenAI! Please respond with a simple greeting.", max_tokens=50, temperature=0.1)
            assert isinstance(response, str)
            assert len(response) > 0
            print(f"Azure OpenAI Async Response: {response}")
        except Exception as e:
            pytest.fail(f"Azure OpenAI async completion failed: {e}")

    @pytest.mark.integration
    @pytest.mark.asyncio
    async def test_azure_openai_stream_batch_chat(self, azure_client):
        """Ensure stream, batch, and chat work for Azure OpenAI."""
        try:
            # Test streaming
            stream_response = await azure_client.complete_stream("stream hello")
            assert isinstance(stream_response, str) and len(stream_response) > 0

            # Test batch processing
            batch_results = await azure_client.complete_batch(["A", "B"], max_tokens=5, max_concurrency=2)
            assert isinstance(batch_results, list) and len(batch_results) == 2
            assert all(isinstance(r, str) and r for r in batch_results)

            # Test chat optimization
            chat_response = await azure_client.chat_optimized([("user", "say hi")], max_tokens=8)
            assert isinstance(chat_response, str) and len(chat_response) > 0

        except Exception as e:
            pytest.fail(f"Azure OpenAI advanced features failed: {e}")

    @pytest.mark.integration
    def test_azure_openai_with_different_temperatures(self, azure_client):
        """Test Azure OpenAI with different temperature settings."""
        try:
            # Low temperature (more deterministic)
            response_low = azure_client.complete("Tell me a creative story in one sentence.", max_tokens=50, temperature=0.1)

            # High temperature (more creative)
            response_high = azure_client.complete("Tell me a creative story in one sentence.", max_tokens=50, temperature=0.9)

            assert response_low is not None
            assert response_high is not None
            print(f"Low temperature: {response_low}")
            print(f"High temperature: {response_high}")
        except Exception as e:
            pytest.fail(f"Azure OpenAI temperature test failed: {e}")

    @pytest.mark.integration
    def test_azure_openai_with_max_tokens(self, azure_client):
        """Test Azure OpenAI with different max_tokens settings."""
        try:
            # Short response
            response_short = azure_client.complete("Explain quantum computing.", max_tokens=20, temperature=0.1)

            # Longer response
            response_long = azure_client.complete("Explain quantum computing.", max_tokens=100, temperature=0.1)

            assert response_short is not None
            assert response_long is not None
            assert len(response_long) > len(response_short)
            print(f"Short response: {response_short}")
            print(f"Long response: {response_long}")
        except Exception as e:
            pytest.fail(f"Azure OpenAI max_tokens test failed: {e}")


class TestAzureOpenAIErrorHandling:
    """Test Azure OpenAI client error handling."""

    def test_azure_openai_error_handling(self):
        """Test error handling with invalid credentials."""
        config = LlmConfig.azure_openai(api_key="invalid-key-that-is-long-enough-for-validation", deployment_name="invalid-deployment", endpoint="https://invalid.openai.azure.com")

        client = LlmClient(config)

        with pytest.raises(Exception, match="(?i)(error|failed|invalid|unauthorized|forbidden)"):
            client.complete("Hello, Azure OpenAI!", max_tokens=50)


class TestAzureOpenAIComparison:
    """Test Azure OpenAI provider compared to other providers."""

    def test_azure_openai_vs_openai_config(self):
        """Test that Azure OpenAI and OpenAI configs are different."""
        azure_config = LlmConfig.azure_openai(api_key="test-key-that-is-long-enough-for-validation", deployment_name="gpt-4o-mini", endpoint="https://test.openai.azure.com")

        openai_config = LlmConfig.openai(api_key="test-key-that-is-long-enough-for-validation", model="gpt-4o")

        assert azure_config.provider() == "azure_openai"
        assert openai_config.provider() == "openai"
        assert azure_config.model() == "gpt-4o-mini"
        assert openai_config.model() == "gpt-4o"

    def test_azure_openai_client_consistency(self):
        """Test that Azure OpenAI client has same interface as other providers."""
        azure_config = LlmConfig.azure_openai(api_key="test-key-that-is-long-enough-for-validation", deployment_name="gpt-4o-mini", endpoint="https://test.openai.azure.com")

        openai_config = LlmConfig.openai(api_key="test-key-that-is-long-enough-for-validation", model="gpt-4o")

        azure_client = LlmClient(azure_config)
        openai_client = LlmClient(openai_config)

        # Both clients should have the same interface
        assert hasattr(azure_client, "complete")
        assert hasattr(openai_client, "complete")
        assert hasattr(azure_client, "complete_async")
        assert hasattr(openai_client, "complete_async")


if __name__ == "__main__":
    pytest.main([__file__])
