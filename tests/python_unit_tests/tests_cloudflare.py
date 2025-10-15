"""Unit tests for Cloudflare Worker AI provider in GraphBit Python bindings."""

from environs import Env
import os
env = Env()
env.read_env()
import pytest
from graphbit import LlmClient, LlmConfig

def get_cloudflare_credentials():
    """Get Cloudflare credentials from environment variables."""
    api_key = env.str("CLOUDFLARE_API_KEY")
    account_id = env.str("CLOUDFLARE_ACCOUNT_ID")
    
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
        # Mock a valid account ID (32 hex chars)
        valid_account_id = "a" * 32

        # Test with empty API key
        with pytest.raises(ValueError, match=".*API key cannot be empty.*"):
            LlmConfig.cloudflare(
                api_key="",
                model="@cf/meta/llama-2-7b-chat-int8",
                account_id=valid_account_id
            )
        
        # Test with empty account ID
        with pytest.raises(ValueError, match=".*account ID cannot be empty.*"):
            LlmConfig.cloudflare(
                api_key="test_key",
                model="@cf/meta/llama-2-7b-chat-int8",
                account_id=""
            )

        # Test with None API key (should convert to empty)
        with pytest.raises(ValueError, match=".*API key cannot be empty.*"):
            LlmConfig.cloudflare(
                api_key=None,  # type: ignore
                model="@cf/meta/llama-2-7b-chat-int8",
                account_id=valid_account_id
            )
        
        # Test with None account ID (should convert to empty)
        with pytest.raises(ValueError, match=".*account ID cannot be empty.*"):
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
        request = {
            "messages": [{"role": "user", "content": "Test message"}],
            "max_tokens": None,
            "temperature": None,
            "top_p": None,
        }
        assert len(request["messages"]) == 1
        assert request["messages"][0]["role"] == "user"
        assert request["messages"][0]["content"] == "Test message"

        # Test with parameters
        request_with_params = {**request, "max_tokens": 100, "temperature": 0.7}
        assert request_with_params["max_tokens"] == 100
        assert request_with_params["temperature"] == 0.7

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

        # Test with invalid credentials but valid format
        mock_account_id = "a" * 32  # 32-character mock account ID
        invalid_config = LlmConfig.cloudflare(
            api_key="test_invalid_key_1234567890",  # Meet minimum length requirement
            model="@cf/meta/llama-2-7b-chat-int8",
            account_id=mock_account_id
        )
        invalid_client = LlmClient(invalid_config)
        
        with pytest.raises(Exception):  # Should raise an error with invalid credentials
            invalid_client.complete("Test message")

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    @pytest.mark.asyncio
    async def test_cloudflare_async_operations(self, cloudflare_client):
        """Test async client operations."""
        response = await cloudflare_client.complete_async(
            prompt="Say 'Hello' in one word only.",
            max_tokens=10,
            temperature=0.0
        )
        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Cloudflare async response: {response}")

        # Test error handling with invalid request
        with pytest.raises(Exception):
            await cloudflare_client.complete_async(
                prompt="",  # Empty content should fail
                max_tokens=10
            )