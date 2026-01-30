"""Azure LLM Client Unit Tests.

Comprehensive unit tests for Azure LLM provider that mirror OpenAI test coverage.
"""

import os

import pytest

from graphbit import LlmClient, LlmConfig


def get_azurellm_test_config():
    """Get test Azure LLM configuration with long enough API key."""
    return {"api_key": "test-azure-llm-api-key-that-is-long-enough-for-validation", "deployment_name": "gpt-4o-mini", "endpoint": "https://test.openai.azure.com", "api_version": "2024-10-21"}


def get_azurellm_credentials():
    """Get Azure LLM credentials from environment variables."""
    api_key = os.getenv("AZURELLM_API_KEY")
    endpoint = os.getenv("AZURELLM_ENDPOINT")
    deployment = os.getenv("AZURELLM_DEPLOYMENT")
    api_version = os.getenv("AZURELLM_API_VERSION", "2024-10-21")

    if api_key and endpoint and deployment:
        return api_key, endpoint, deployment, api_version
    return None


def has_azurellm_credentials():
    """Check if Azure LLM credentials are available."""
    return get_azurellm_credentials() is not None


class TestAzureLlmConfig:
    """Test Azure LLM configuration classes."""

    def test_azurellm_config_creation(self):
        """Test creating Azure LLM configuration."""
        config_data = get_azurellm_test_config()
        config = LlmConfig.azurellm(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])
        assert config is not None
        assert config.provider() == "azurellm"
        assert config.model() == config_data["deployment_name"]

    def test_azurellm_config_with_api_version(self):
        """Test creating Azure LLM configuration with custom API version."""
        config_data = get_azurellm_test_config()
        config = LlmConfig.azurellm(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"], api_version="2024-06-01")
        assert config is not None
        assert config.provider() == "azurellm"
        assert config.model() == config_data["deployment_name"]

    def test_azurellm_config_different_deployments(self):
        """Test Azure LLM configuration with different deployment names."""
        deployments = ["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"]
        config_data = get_azurellm_test_config()

        for deployment in deployments:
            config = LlmConfig.azurellm(api_key=config_data["api_key"], deployment_name=deployment, endpoint=config_data["endpoint"])
            assert config is not None
            assert config.provider() == "azurellm"
            assert config.model() == deployment

    def test_azurellm_config_empty_api_key(self):
        """Test that empty API key raises validation error."""
        config_data = get_azurellm_test_config()
        with pytest.raises(ValueError, match="API key cannot be empty"):
            LlmConfig.azurellm(api_key="", deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

    def test_azurellm_config_whitespace_api_key(self):
        """Test that whitespace-only API key raises validation error."""
        config_data = get_azurellm_test_config()
        with pytest.raises(ValueError, match="Azure LLM API key too short"):
            LlmConfig.azurellm(api_key="   ", deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

    def test_azurellm_config_short_api_key(self):
        """Test that short API key raises validation error."""
        config_data = get_azurellm_test_config()
        with pytest.raises(ValueError, match="Azure LLM API key too short"):
            LlmConfig.azurellm(api_key="short", deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])


class TestAzureLlmClient:
    """Test Azure LLM client functionality."""

    def test_azurellm_client_creation(self):
        """Test creating Azure LLM client."""
        config_data = get_azurellm_test_config()
        config = LlmConfig.azurellm(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

        client = LlmClient(config)
        assert client is not None

    def test_azurellm_client_with_debug(self):
        """Test creating Azure LLM client with debug mode."""
        config_data = get_azurellm_test_config()
        config = LlmConfig.azurellm(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])

        client = LlmClient(config, debug=True)
        assert client is not None

    def test_azurellm_client_creation_invalid_config(self):
        """Test creating Azure LLM client with invalid configuration."""
        with pytest.raises((ValueError, TypeError)):
            LlmClient("invalid_config")

    @pytest.mark.asyncio
    async def test_azurellm_client_complete_no_credentials(self):
        """Test Azure LLM completion without real credentials (should fail)."""
        config_data = get_azurellm_test_config()
        config = LlmConfig.azurellm(api_key=config_data["api_key"], deployment_name=config_data["deployment_name"], endpoint=config_data["endpoint"])
        client = LlmClient(config)

        # This should fail because we're using test credentials
        with pytest.raises(Exception, match="(?i)(error|failed|invalid|unauthorized)"):
            await client.complete_async("Test prompt")


class TestAzureLlmIntegration:
    """Integration tests for Azure LLM provider.

    These tests require actual Azure LLM credentials and should be run with:
    AZURELLM_API_KEY=your_key AZURELLM_ENDPOINT=your_endpoint AZURELLM_DEPLOYMENT=your_deployment pytest -m integration
    """

    @pytest.fixture
    def azure_credentials(self):
        """Get Azure LLM credentials from environment variables."""
        credentials = get_azurellm_credentials()
        if not credentials:
            pytest.skip("Azure LLM credentials not found. Set AZURELLM_API_KEY, AZURELLM_ENDPOINT, and AZURELLM_DEPLOYMENT environment variables.")
        return credentials

    @pytest.fixture
    def azure_config(self, azure_credentials):
        """Create Azure LLM configuration."""
        api_key, endpoint, deployment, api_version = azure_credentials
        return LlmConfig.azurellm(api_key=api_key, deployment_name=deployment, endpoint=endpoint, api_version=api_version)

    @pytest.fixture
    def azure_client(self, azure_config):
        """Create Azure LLM client."""
        return LlmClient(azure_config)

    @pytest.mark.integration
    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_simple_completion(self, azure_client):
        """Test simple text completion with Azure LLM."""
        try:
            response = azure_client.complete("Hello, Azure LLM! Please respond with a simple greeting.", max_tokens=50)
            assert isinstance(response, str)
            assert len(response) > 0
            print(f"Azure LLM Response: {response}")
        except Exception as e:
            pytest.fail(f"Azure LLM completion failed: {e}")

    @pytest.mark.integration
    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    @pytest.mark.asyncio
    async def test_azurellm_async_completion(self, azure_config):
        """Test async completion with Azure LLM."""
        client = LlmClient(azure_config)
        try:
            response = await client.complete_async("Hello, Azure LLM! Please respond with a simple greeting.", max_tokens=50, temperature=0.1)
            assert isinstance(response, str)
            assert len(response) > 0
            print(f"Azure LLM Async Response: {response}")
        except Exception as e:
            pytest.fail(f"Azure LLM async completion failed: {e}")

    @pytest.mark.integration
    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    @pytest.mark.asyncio
    async def test_azurellm_stream_batch_chat(self, azure_client):
        """Ensure stream, batch, and chat work for Azure LLM."""
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
            pytest.fail(f"Azure LLM advanced features failed: {e}")

    @pytest.mark.integration
    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_with_different_temperatures(self, azure_client):
        """Test Azure LLM with different temperature settings."""
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
            pytest.fail(f"Azure LLM temperature test failed: {e}")

    @pytest.mark.integration
    @pytest.mark.skipif(not has_azurellm_credentials(), reason="Azure LLM credentials not available")
    def test_azurellm_with_max_tokens(self, azure_client):
        """Test Azure LLM with different max_tokens settings."""
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
            pytest.fail(f"Azure LLM max_tokens test failed: {e}")


class TestAzureLlmErrorHandling:
    """Test Azure LLM client error handling."""

    def test_azurellm_error_handling(self):
        """Test error handling with invalid credentials."""
        config = LlmConfig.azurellm(api_key="invalid-key-that-is-long-enough-for-validation", deployment_name="invalid-deployment", endpoint="https://invalid.openai.azure.com")

        client = LlmClient(config)

        with pytest.raises(Exception, match="(?i)(error|failed|invalid|unauthorized|forbidden)"):
            client.complete("Hello, Azure LLM!", max_tokens=50)


class TestAzureLlmComparison:
    """Test Azure LLM provider compared to other providers."""

    def test_azurellm_vs_openai_config(self):
        """Test that Azure LLM and OpenAI configs are different."""
        azure_config = LlmConfig.azurellm(api_key="test-key-that-is-long-enough-for-validation", deployment_name="gpt-4o-mini", endpoint="https://test.openai.azure.com")

        openai_config = LlmConfig.openai(api_key="test-key-that-is-long-enough-for-validation", model="gpt-4o")

        assert azure_config.provider() == "azurellm"
        assert openai_config.provider() == "openai"
        assert azure_config.model() == "gpt-4o-mini"
        assert openai_config.model() == "gpt-4o"

    def test_azurellm_client_consistency(self):
        """Test that Azure LLM client has same interface as other providers."""
        azure_config = LlmConfig.azurellm(api_key="test-key-that-is-long-enough-for-validation", deployment_name="gpt-4o-mini", endpoint="https://test.openai.azure.com")

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
