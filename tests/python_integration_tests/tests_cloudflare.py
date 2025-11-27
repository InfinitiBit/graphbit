"""Integrationdef get_cloudflare_credentials() -> Optional[Tuple[str, str]]:
    api_key = os.getenv("CLOUDFLARE_API_KEY")
    account_id = os.getenv("CLOUDFLARE_ACCOUNT_ID")

    if api_key and account_id:
        return api_key, account_idor Cloudflare Worker AI provider.

Integration tests for Cloudflare Worker AI that mirror the integration test pattern
of other providers. These tests require actual Cloudflare credentials.
"""

from environs import Env
import os
env = Env()
env.read_env()
from typing import Optional, Tuple

import pytest

from graphbit import LlmClient, LlmConfig


def get_cloudflare_credentials() -> Optional[Tuple[str, str, str]]:
    """Get Cloudflare credentials from environment variables."""
    api_key = env.str("CLOUDFLARE_API_KEY")
    account_id = env.str("CLOUDFLARE_ACCOUNT_ID")

    if api_key and account_id:
        return api_key, account_id
    return None


def has_cloudflare_credentials() -> bool:
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


class TestCloudflareIntegration:
    """Test Cloudflare Worker AI integration."""

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    def test_cloudflare_simple_completion(self, cloudflare_client):
        """Test simple text completion with Cloudflare."""
        response = cloudflare_client.complete(
            "Say 'Hello' in one word only.", 
            max_tokens=10, 
            temperature=0.0
        )
        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Cloudflare simple completion: {response}")

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    @pytest.mark.asyncio
    async def test_cloudflare_async_completion(self, cloudflare_config):
        """Test async completion with Cloudflare."""
        client = LlmClient(cloudflare_config)
        response = await client.complete_async(
            "Say 'Hello' in one word only.", 
            max_tokens=10, 
            temperature=0.0
        )
        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Cloudflare async completion: {response}")

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    def test_cloudflare_conversation(self, cloudflare_client):
        """Test Cloudflare conversation handling."""
        # First message
        response1 = cloudflare_client.complete("My name is Alice.", max_tokens=50, temperature=0.1)
        assert isinstance(response1, str)

        # Follow-up message
        response2 = cloudflare_client.complete(
            f"previous chat: user: My name is Alice. assistant: {response1} query: What's my name?",
            max_tokens=50, 
            temperature=0.1
        )
        assert isinstance(response2, str)
        assert "alice" in response2.lower()
        print(f"Cloudflare conversation: {response2}")


class TestCloudflareAdvancedFeatures:
    """Test advanced Cloudflare Worker AI features."""

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    def test_cloudflare_different_models(self, cloudflare_credentials):
        """Test Cloudflare with different models."""
        api_key, account_id = cloudflare_credentials

        # Test with Llama model
        llama_config = LlmConfig.cloudflare(
            api_key=api_key,
            model="@cf/meta/llama-2-7b-chat-int8",
            account_id=account_id
        )
        llama_client = LlmClient(llama_config)
        llama_response = llama_client.complete("Test", max_tokens=10)
        assert isinstance(llama_response, str)

        # Test with Mistral model
        mistral_config = LlmConfig.cloudflare(
            api_key=api_key,
            model="@cf/mistral/mistral-7b-instruct-v0.1",
            account_id=account_id
        )
        mistral_client = LlmClient(mistral_config)
        mistral_response = mistral_client.complete("Test", max_tokens=10)
        assert isinstance(mistral_response, str)

    @pytest.mark.skipif(not has_cloudflare_credentials(), reason="Cloudflare credentials not available")
    @pytest.mark.asyncio
    async def test_cloudflare_streaming(self, cloudflare_client):
        """Test Cloudflare streaming completion."""
        response = await cloudflare_client.complete_stream(
            "Tell me a short story about a robot.",
            max_tokens=100,
            temperature=0.7
        )
        assert isinstance(response, str)
        assert len(response) > 0
        print(f"Cloudflare streaming: {response}")