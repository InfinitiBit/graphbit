import pytest
from graphbit_py.llm import LlmConfig, LlmMessage, LlmRequest, LlmRole, LlmClient
from graphbit_py.errors import GraphBitError

def test_cloudflare_config_validation():
    # Test valid config
    config = LlmConfig.cloudflare(
        api_key="test_key",
        model="@cf/meta/llama-2-7b-chat-int8",
        account_id="test_account"
    )
    assert config is not None

    # Test invalid config - empty API key
    with pytest.raises(GraphBitError):
        LlmConfig.cloudflare(
            api_key="",
            model="@cf/meta/llama-2-7b-chat-int8",
            account_id="test_account"
        )

@pytest.mark.asyncio
async def test_cloudflare_client():
    config = LlmConfig.cloudflare(
        api_key="test_key",
        model="@cf/meta/llama-2-7b-chat-int8",
        account_id="test_account"
    )
    client = LlmClient(config)
    assert client is not None

    # Test chat completion with invalid API key (should raise error)
    request = LlmRequest(
        messages=[
            LlmMessage(
                role=LlmRole.USER,
                content="Test message"
            )
        ],
        temperature=0.7,
        max_tokens=100,
        top_p=1.0
    )
    with pytest.raises(GraphBitError):
        await client.complete(request)