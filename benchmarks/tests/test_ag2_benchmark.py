"""Unit tests for AG2Benchmark — no LLM calls, no API keys required."""
import asyncio
from unittest.mock import MagicMock, patch

import pytest

from frameworks.ag2_benchmark import AG2Benchmark, _make_llm_config
from frameworks.common import LLMConfig as BenchLLMConfig, LLMProvider


def _make_bench_config(provider: LLMProvider = LLMProvider.OPENAI) -> dict:
    llm_config = BenchLLMConfig(
        provider=provider,
        model="gpt-4o-mini",
        api_key="test-key",
        temperature=0.0,
        max_tokens=256,
    )
    return {"llm_config": llm_config, "concurrency": 2}


class TestMakeLLMConfig:
    def test_openai_config(self):
        bench = BenchLLMConfig(
            provider=LLMProvider.OPENAI,
            model="gpt-4o",
            api_key="sk-test",
            temperature=0.5,
            max_tokens=512,
        )
        config = _make_llm_config(bench)
        # LLMConfig is a valid AG2 LLMConfig object
        assert config is not None
        assert config.temperature == 0.5
        assert config.max_tokens == 512

    def test_anthropic_config(self):
        bench = BenchLLMConfig(
            provider=LLMProvider.ANTHROPIC,
            model="claude-sonnet-4-6",
            api_key="ant-key",
            temperature=0.0,
            max_tokens=1024,
        )
        config = _make_llm_config(bench)
        assert config is not None

    def test_ollama_config(self):
        bench = BenchLLMConfig(
            provider=LLMProvider.OLLAMA,
            model="llama3",
            api_key=None,
            temperature=0.7,
            max_tokens=256,
            base_url="http://localhost:11434",
        )
        config = _make_llm_config(bench)
        assert config is not None


class TestAG2BenchmarkSetup:
    def test_setup_creates_executor(self):
        bench = AG2Benchmark(_make_bench_config())
        asyncio.run(bench.setup())
        assert bench._llm_config is not None
        assert bench._executor is not None
        asyncio.run(bench.teardown())

    def test_teardown_clears_state(self):
        bench = AG2Benchmark(_make_bench_config())
        asyncio.run(bench.setup())
        asyncio.run(bench.teardown())
        assert bench._llm_config is None
        assert bench._executor is None


class TestAG2BenchmarkScenarios:
    """Test that scenarios handle errors correctly without making LLM calls."""

    def _bench(self):
        b = AG2Benchmark(_make_bench_config())
        asyncio.run(b.setup())
        return b

    def test_simple_task_error_handling(self):
        bench = self._bench()
        with patch.object(bench, "_ask", side_effect=RuntimeError("API error")):
            metrics = asyncio.run(bench.run_simple_task())
        assert metrics.error_rate == 1.0
        asyncio.run(bench.teardown())

    def test_sequential_pipeline_error_handling(self):
        bench = self._bench()
        with patch.object(bench, "_ask", side_effect=RuntimeError("API error")):
            metrics = asyncio.run(bench.run_sequential_pipeline())
        assert metrics.error_rate == 1.0
        asyncio.run(bench.teardown())

    def test_parallel_pipeline_error_handling(self):
        bench = self._bench()
        with patch.object(bench, "_ask", side_effect=RuntimeError("API error")):
            metrics = asyncio.run(bench.run_parallel_pipeline())
        assert metrics.error_rate == 1.0
        asyncio.run(bench.teardown())

    def test_complex_workflow_uses_groupchat(self):
        """Verify GroupChat is constructed and used (not single-agent loop)."""
        bench = self._bench()

        groupchat_instances = []

        def mock_initiate_chat(recipient, **kwargs):
            groupchat_instances.append(type(recipient).__name__)
            mock_result = MagicMock()
            mock_result.chat_history = [
                {"role": "assistant", "content": "Step 1 done"},
                {"role": "assistant", "content": "Step 2 done"},
            ]
            return mock_result

        with patch.object(bench._executor, "initiate_chat", side_effect=mock_initiate_chat):
            metrics = asyncio.run(bench.run_complex_workflow())

        # Should have been called once with GroupChatManager
        assert len(groupchat_instances) == 1
        assert groupchat_instances[0] == "GroupChatManager"
        assert metrics.error_rate == 0.0
        asyncio.run(bench.teardown())
