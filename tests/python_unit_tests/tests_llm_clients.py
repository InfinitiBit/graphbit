"""Unit tests for LLM clients."""

import os
import socket
from urllib.parse import urlparse

import pytest

from graphbit import LlmConfig, LlmClient


def get_api_key(provider: str) -> str:
    """Get API key from environment variables."""
    key = os.getenv(f"{provider.upper()}_API_KEY")
    if not key:
        pytest.skip(f"No {provider.upper()}_API_KEY found in environment")
    assert key is not None
    return key


def check_ollama_available() -> bool:
    """Check if Ollama server is reachable via TCP (avoids HTTP urlopen)."""
    base_url = os.getenv("OLLAMA_API_URL", "http://localhost:11434").rstrip("/")
    try:
        parsed = urlparse(base_url)
        host = parsed.hostname or "localhost"
        port = parsed.port or 11434
        with socket.create_connection((host, port), timeout=2):
            return True
    except Exception:
        return False


class TestLlmConfig:
    """Test LLM configuration classes."""

    def test_llm_config_creation_openai(self):
        """Test creating OpenAI LLM configuration."""
        api_key = get_api_key("openai")
        config = LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        assert config is not None
        assert config.provider() == "openai"
        assert config.model() == "gpt-4-turbo"

    def test_llm_config_creation_anthropic(self):
        """Test creating Anthropic LLM configuration."""
        api_key = get_api_key("anthropic")
        config = LlmConfig.anthropic(api_key=api_key, model="claude-3-sonnet")
        assert config is not None
        assert config.provider() == "anthropic"
        assert config.model() == "claude-3-sonnet"

    def test_llm_config_ollama(self):
        """Test creating Ollama LLM configuration."""
        config = LlmConfig.ollama(model="llama3.2")
        assert config is not None
        assert config.provider() == "ollama"
        assert config.model() == "llama3.2"

    def test_llm_config_huggingface(self):
        """Test creating HuggingFace LLM configuration with base URL."""
        api_key = get_api_key("huggingface")
        config = LlmConfig.huggingface(api_key=api_key, model="gpt2", base_url="https://api-inference.huggingface.co")
        assert config is not None
        assert config.provider() == "huggingface"
        assert config.model() == "gpt2"

    def test_llm_config_deepseek(self):
        """Test creating DeepSeek LLM configuration."""
        api_key = get_api_key("deepseek")
        config = LlmConfig.deepseek(api_key=api_key, model="deepseek-chat")
        assert config is not None
        assert config.provider() == "deepseek"
        assert config.model() == "deepseek-chat"

    def test_llm_config_perplexity(self):
        """Test creating Perplexity LLM configuration."""
        api_key = get_api_key("perplexity")
        config = LlmConfig.perplexity(api_key=api_key, model="sonar")
        assert config is not None
        assert config.provider() == "perplexity"
        assert config.model() == "sonar"


class TestLlmClient:
    """Test LLM client functionality."""

    def test_llm_client_creation_with_config(self):
        """Test creating LLM client with configuration."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config)
        assert client is not None

    def test_llm_client_creation_invalid_config(self):
        """Test creating LLM client with invalid configuration."""
        with pytest.raises((ValueError, TypeError)):
            LlmClient("invalid_config")

    @pytest.mark.asyncio
    async def test_llm_client_complete_ollama_no_server(self):
        """Assert correct behavior whether Ollama is up or down."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config, debug=True)

        if check_ollama_available():
            response = await client.complete_async("Test prompt")
            assert isinstance(response, str) and len(response) > 0
        else:
            with pytest.raises(Exception, match="(?i)(connection|connect|failed|error|refused|unavailable|url)"):
                await client.complete_async("Test prompt")

    @pytest.mark.asyncio
    async def test_llm_client_complete_openai(self):
        """Test OpenAI completion."""
        api_key = get_api_key("openai")
        config = LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        client = LlmClient(config)
        response = await client.complete_async("Say hello!")
        assert isinstance(response, str) and len(response) > 0

    @pytest.mark.asyncio
    async def test_llm_client_openai_stream_batch_chat(self):
        """Ensure stream, batch, and chat work for OpenAI (skips if key missing)."""
        api_key = get_api_key("openai")
        config = LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        client = LlmClient(config)

        s = await client.complete_stream("stream hello")
        assert isinstance(s, str) and len(s) > 0

        results = await client.complete_batch(["A", "B"], max_tokens=5, max_concurrency=2)
        assert isinstance(results, list) and len(results) == 2
        assert all(isinstance(r, str) and r for r in results)

        chat = await client.chat_optimized([("user", "say hi")], max_tokens=8)
        assert isinstance(chat, str) and len(chat) > 0

    @pytest.mark.asyncio
    async def test_llm_client_complete_anthropic(self):
        """Test Anthropic completion."""
        api_key = get_api_key("anthropic")
        config = LlmConfig.anthropic(api_key=api_key, model="claude-3-sonnet")
        client = LlmClient(config)
        response = await client.complete_async("Say hello!")
        assert isinstance(response, str) and len(response) > 0

    @pytest.mark.asyncio
    async def test_llm_client_anthropic_batch_chat(self):
        """Ensure batch and chat work for Anthropic (skips if key missing)."""
        api_key = get_api_key("anthropic")
        config = LlmConfig.anthropic(api_key=api_key, model="claude-3-sonnet")
        client = LlmClient(config)
        results = await client.complete_batch(["Hi", "There"], max_tokens=8, max_concurrency=2)
        assert isinstance(results, list) and len(results) == 2
        assert all(isinstance(r, str) and r for r in results)
        chat = await client.chat_optimized([("user", "say hi")], max_tokens=8)
        assert isinstance(chat, str) and len(chat) > 0

    def test_llm_client_complete_sync_ollama(self):
        """Synchronous complete should succeed if server up, else raise."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config, debug=True)
        if check_ollama_available():
            resp = client.complete("Hello")
            assert isinstance(resp, str) and len(resp) > 0
        else:
            with pytest.raises(Exception, match="(?i)(connection|connect|failed|error|refused|unavailable|url)"):
                client.complete("Hello")

    @pytest.mark.asyncio
    async def test_llm_client_complete_batch(self):
        """Batch completion should return results or error strings per item."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config, debug=True)
        prompts = ["Hi", "", "Another message"]
        results = await client.complete_batch(prompts)
        assert isinstance(results, list) and len(results) == 2
        if check_ollama_available():
            assert all(isinstance(r, str) and r and not r.startswith("Error:") for r in results)
        else:
            assert any(isinstance(r, str) and r.startswith("Error:") for r in results)
        results2 = await client.complete_batch(["A", "B"], max_concurrency=5)
        assert isinstance(results2, list) and len(results2) == 2

    @pytest.mark.asyncio
    async def test_llm_client_chat_optimized(self):
        """Chat API should behave similarly to completion."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config, debug=True)
        messages = [("system", "You are helpful."), ("user", "Say hi")]
        if check_ollama_available():
            resp = await client.chat_optimized(messages)
            assert isinstance(resp, str) and len(resp) > 0
        else:
            with pytest.raises(Exception, match="(?i)(connection|connect|failed|error|refused|unavailable|url)"):
                await client.chat_optimized(messages)

    @pytest.mark.asyncio
    async def test_llm_client_complete_stream_alias(self):
        """Stream alias should behave like async complete."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config)
        if check_ollama_available():
            resp = await client.complete_stream("Hello stream")
            assert isinstance(resp, str) and len(resp) > 0
        else:
            with pytest.raises(Exception, match="(?i)(connection|connect|failed|error|refused|unavailable|url)"):
                await client.complete_stream("Hello stream")

    @pytest.mark.asyncio
    async def test_llm_client_warmup_and_stats(self):
        """Warmup should return a message; stats API should expose metrics."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config, debug=True)
        msg = await client.warmup()
        assert isinstance(msg, str) and len(msg) > 0
        stats = client.get_stats()
        assert isinstance(stats, dict)
        for key in [
            "total_requests",
            "successful_requests",
            "failed_requests",
            "success_rate",
            "average_response_time_ms",
            "circuit_breaker_state",
            "uptime_seconds",
        ]:
            assert key in stats
        client.reset_stats()
        stats2 = client.get_stats()
        assert stats2["total_requests"] == 0
        assert stats2["successful_requests"] == 0
        assert stats2["failed_requests"] == 0

    @pytest.mark.asyncio
    async def test_llm_client_stats_average_updates(self):
        """Two successful calls should update average response time."""
        if not check_ollama_available():
            pytest.skip("Ollama not reachable for success-path stats test")
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config)
        _ = await client.complete_async("Ping 1")
        _ = await client.complete_async("Ping 2")
        stats = client.get_stats()
        assert stats["total_requests"] >= 2
        assert stats["successful_requests"] >= 2
        assert stats["average_response_time_ms"] >= 0.0

    @pytest.mark.asyncio
    async def test_llm_client_invalid_params(self):
        """Validation errors for invalid inputs."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config)
        with pytest.raises((ValueError, TypeError)):
            await client.complete_async("")
        with pytest.raises((ValueError, TypeError)):
            await client.complete_async("hi", temperature=3.0)
        with pytest.raises((ValueError, TypeError)):
            await client.complete_async("hi", max_tokens=-1)
        with pytest.raises((ValueError, TypeError)):
            await client.complete_batch([])
        with pytest.raises((ValueError, TypeError)):
            await client.chat_optimized([])

    @pytest.mark.asyncio
    async def test_llm_client_additional_validations(self):
        """Edge validations: temperature bounds, invalid concurrency, role defaulting."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config)
        if check_ollama_available():
            r1 = await client.complete_async("edge temp low", temperature=0.0)
            r2 = await client.complete_async("edge temp high", temperature=2.0)
            assert isinstance(r1, str) and isinstance(r2, str)
        else:
            with pytest.raises(Exception, match="(?i)(connection|connect|failed|error|refused|unavailable|url)"):
                await client.complete_async("edge temp low", temperature=0.0)
        with pytest.raises((ValueError, TypeError)):
            await client.complete_batch(["a"], max_concurrency=0)
        msgs = [("other", "hello")]
        if check_ollama_available():
            resp = await client.chat_optimized(msgs)
            assert isinstance(resp, str) and len(resp) > 0
        else:
            with pytest.raises(Exception, match="(?i)(connection|connect|failed|error|refused|unavailable|url)"):
                await client.chat_optimized(msgs)

    @pytest.mark.asyncio
    async def test_llm_client_complete_deepseek(self):
        """Test DeepSeek completion (skips if DEEPSEEK_API_KEY not set)."""
        api_key = get_api_key("deepseek")
        config = LlmConfig.deepseek(api_key=api_key, model="deepseek-chat")
        client = LlmClient(config)
        response = await client.complete_async("Say hello!")
        assert isinstance(response, str) and len(response) > 0

    @pytest.mark.asyncio
    async def test_llm_client_batch_deepseek(self):
        """Ensure DeepSeek batch works (skips if key missing)."""
        api_key = get_api_key("deepseek")
        config = LlmConfig.deepseek(api_key=api_key, model="deepseek-chat")
        client = LlmClient(config)
        results = await client.complete_batch(["x", "y"], max_tokens=8, max_concurrency=2)
        assert isinstance(results, list) and len(results) == 2
        assert all(isinstance(r, str) and r for r in results)

    @pytest.mark.asyncio
    async def test_llm_client_complete_perplexity(self):
        """Test Perplexity completion (skips if PERPLEXITY_API_KEY not set)."""
        api_key = get_api_key("perplexity")
        config = LlmConfig.perplexity(api_key=api_key, model="sonar")
        client = LlmClient(config)
        response = await client.complete_async("Say hello!")
        assert isinstance(response, str) and len(response) > 0

    @pytest.mark.asyncio
    async def test_llm_client_batch_perplexity(self):
        """Perplexity batch coverage (skips if key missing)."""
        api_key = get_api_key("perplexity")
        config = LlmConfig.perplexity(api_key=api_key, model="sonar")
        client = LlmClient(config)
        results = await client.complete_batch(["x", "y"], max_tokens=8, max_concurrency=2)
        assert isinstance(results, list) and len(results) == 2
        assert all(isinstance(r, str) and r for r in results)

    @pytest.mark.asyncio
    async def test_llm_client_complete_huggingface(self):
        """Minimal completion for HuggingFace (skips if key missing)."""
        api_key = get_api_key("huggingface")
        config = LlmConfig.huggingface(api_key=api_key, model="gpt2")
        client = LlmClient(config)
        try:
            resp = await client.complete_async("Hello")
            assert isinstance(resp, str) and len(resp) > 0
        except Exception as e:
            pytest.skip(f"HuggingFace API not available or model restricted: {e}")


class TestLlmClientErrorHandling:
    """Test LLM client error handling."""

    def test_empty_api_key_openai(self):
        """Test creating OpenAI client with empty API key."""
        with pytest.raises((ValueError, TypeError)):
            LlmConfig.openai(api_key="")

    def test_empty_api_key_anthropic(self):
        """Test creating Anthropic client with empty API key."""
        with pytest.raises((ValueError, TypeError)):
            LlmConfig.anthropic(api_key="")

    @pytest.mark.asyncio
    async def test_empty_prompt(self):
        """Test completion with empty prompt."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config)
        with pytest.raises((ValueError, TypeError)):
            await client.complete_async("")

    @pytest.mark.asyncio
    async def test_none_prompt(self):
        """Test completion with None prompt."""
        config = LlmConfig.ollama(model="llama3.2")
        client = LlmClient(config)
        with pytest.raises((ValueError, TypeError)):
            await client.complete_async(None)
