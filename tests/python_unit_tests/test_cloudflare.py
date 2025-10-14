"""Unit tests for Cloudflare Worker AI provider in GraphBit Python bindings."""

import os
import pytest
from graphbit import (
    LlmConfig,
    LlmMessage,
    LlmRequest,
    LlmRole,
    LlmClient,
    GraphBitError,
)


def get_cloudflare_credentials():
    """Get Cloudflare credentials from environment variables."""
    api_key = os.getenv("CLOUDFLARE_API_KEY")
    account_id = os.getenv("CLOUDFLARE_ACCOUNT_ID")
    
    if api_key and account_id:
        return api_key, account_id
    return None


def has_cloudflare_credentials():
    """Check if Cloudflare credentials are available."""
    return get_cloudflare_credentials() is not None


@pytest.fixture(scope="session")
def cloudflare_credentials():
    """Session-scoped fixture for Cloudflare credentials."""
    credentials = get_cloudflare_credentials()
    if not credentials:
        pytest.skip("Cloudflare credentials not found. Set CLOUDFLARE_API_KEY and CLOUDFLARE_ACCOUNT_ID environment variables.")
    return credentials


@pytest.fixture(scope="session")
def cloudflare_config(cloudflare_credentials):
    """Session-scoped fixture for Cloudflare configuration."""
    api_key, account_id = cloudflare_credentials
    return LlmConfig.cloudflare(
        api_key=api_key,
        model="@cf/meta/llama-2-7b-chat-int8",
        account_id=account_id
    )


@pytest.fixture
def cloudflare_client(cloudflare_config):
    """Create Cloudflare client."""
    return LlmClient(cloudflare_config)


class TestCloudflareBasicFunctionality:
    """Test basic Cloudflare Worker AI functionality."""

    def test_cloudflare_config_creation_with_env_vars(self, cloudflare_credentials):
        """Test creating Cloudflare configuration with environment variables."""
        api_key, account_id = cloudflare_credentials
        config = LlmConfig.cloudflare(
            api_key=api_key,
            model="@cf/meta/llama-2-7b-chat-int8",
            account_id=account_id
        )
        assert config.provider() == "cloudflare"
        assert config.model() == "@cf/meta/llama-2-7b-chat-int8"

    def test_cloudflare_config_validation(self):
        """Test that Cloudflare configuration validates required fields."""
        # Test with empty API key
        with pytest.raises(ValueError, match="API key"):
            LlmConfig.cloudflare(
                api_key="",
                model="@cf/meta/llama-2-7b-chat-int8",
                account_id="test_account"
            )
        
        # Test with empty account ID
        with pytest.raises(ValueError, match="Account ID"):
            LlmConfig.cloudflare(
                api_key="test_key",
                model="@cf/meta/llama-2-7b-chat-int8",
                account_id=""
            )
        
        # Test with missing API key
        with pytest.raises(ValueError, match="API key"):
            LlmConfig.cloudflare(
                api_key=None,  # type: ignore
                model="@cf/meta/llama-2-7b-chat-int8",
                account_id="test_account"
            )
        
        # Test with missing account ID
        with pytest.raises(ValueError, match="Account ID"):
            LlmConfig.cloudflare(
                api_key="test_key",
                model="@cf/meta/llama-2-7b-chat-int8",
                account_id=None  # type: ignore
            )


    def test_cloudflare_client_creation(self, cloudflare_client):
        """Test creating Cloudflare client."""
        assert cloudflare_client is not None
        assert isinstance(cloudflare_client, LlmClient)
        
    def test_cloudflare_request_creation(self):
        """Test creating Cloudflare requests."""
        # Test basic message
        request = LlmRequest(
            messages=[LlmMessage(role=LlmRole.USER, content="Test message")]
        )
        assert len(request.messages) == 1
        assert request.messages[0].role == LlmRole.USER
        assert request.messages[0].content == "Test message"

        # Test with parameters
        request_with_params = request.with_max_tokens(100).with_temperature(0.7)
        assert request_with_params.max_tokens == 100
        assert request_with_params.temperature == 0.7

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    def test_cloudflare_client_validation(self, cloudflare_credentials):
        """Test client validation with real and invalid configurations."""
        api_key, account_id = cloudflare_credentials
        
        # Test with real credentials
        valid_config = LlmConfig.cloudflare(
            api_key=api_key,
            model="@cf/meta/llama-2-7b-chat-int8",
            account_id=account_id
        )
        valid_client = LlmClient(valid_config)
        assert valid_client is not None

        # Test with invalid credentials
        invalid_config = LlmConfig.cloudflare(
            api_key="invalid_key",
            model="@cf/meta/llama-2-7b-chat-int8",
            account_id="invalid_account"
        )
        invalid_client = LlmClient(invalid_config)
        
        request = LlmRequest(
            messages=[LlmMessage(role=LlmRole.USER, content="Test message")]
        )
        with pytest.raises(GraphBitError):  # Should raise GraphBitError with invalid credentials
            invalid_client.complete(request)

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    @pytest.mark.asyncio
    async def test_cloudflare_async_operations(self, cloudflare_client):
        """Test async client operations."""
        request = LlmRequest(
            messages=[LlmMessage(role=LlmRole.USER, content="Say 'Hello' in one word only.")]
        ).with_max_tokens(10).with_temperature(0.0)

        response = await cloudflare_client.complete_async(request)
        assert response is not None
        assert response.content is not None
        assert len(response.content) > 0
        print(f"Cloudflare async response: {response.content}")

        # Test error handling with invalid request
        invalid_request = LlmRequest(
            messages=[LlmMessage(role=LlmRole.USER, content="")]  # Empty content should fail
        )
        with pytest.raises(GraphBitError):
            await cloudflare_client.complete_async(invalid_request)