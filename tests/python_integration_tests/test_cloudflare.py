"""Integration tests for Cloudflare Worker AI provider"""

import pytest
from graphbit import LlmConfig, LlmClient, LlmMessage, LlmRequest

def test_cloudflare_config_creation():
    """Test creating Cloudflare Worker AI configuration"""
    config = LlmConfig.cloudflare(
        api_key="test-key",
        model="openai/gpt-5-mini",
        account_id="test-account",
        gateway_id="test-gateway"
    )
    assert config is not None

def test_cloudflare_config_validation():
    """Test Cloudflare Worker AI configuration validation"""
    with pytest.raises(ValueError):
        LlmConfig.cloudflare(
            api_key="",  # Empty API key should fail
            model="openai/gpt-5-mini",
            account_id="test-account",
            gateway_id="test-gateway"
        )

def test_cloudflare_client_creation():
    """Test creating LLM client with Cloudflare Worker AI configuration"""
    config = LlmConfig.cloudflare(
        api_key="test-key",
        model="google-ai-studio/gemini-2.5-flash",
        account_id="test-account",
        gateway_id="test-gateway"
    )
    client = LlmClient(config)
    assert client is not None

@pytest.mark.asyncio
async def test_cloudflare_request():
    """Test Cloudflare Worker AI request structure"""
    config = LlmConfig.cloudflare(
        api_key="test-key",
        model="anthropic/claude-sonnet-4-5",
        account_id="test-account",
        gateway_id="test-gateway"
    )
    client = LlmClient(config)
    
    # Test with a simple request
    request = LlmRequest("Test message")
    with pytest.raises(Exception):  # Should fail with test credentials
        await client.complete(request)

@pytest.mark.asyncio
async def test_cloudflare_complex_request():
    """Test Cloudflare Worker AI with complex message structure"""
    config = LlmConfig.cloudflare(
        api_key="test-key",
        model="anthropic/claude-sonnet-4-5",
        account_id="test-account",
        gateway_id="test-gateway"
    )
    client = LlmClient(config)
    
    messages = [
        LlmMessage.system("You are a helpful assistant."),
        LlmMessage.user("Hello!"),
        LlmMessage.assistant("Hi! How can I help you today?")
    ]
    request = LlmRequest.with_messages(messages)
    
    with pytest.raises(Exception):  # Should fail with test credentials
        await client.complete(request)