"""Integration tests for GIL release validation in GraphBit LLM and Text Splitters.

This test suite validates that the GIL fixes in:
- python/src/llm/client.rs (complete, complete_full)
- python/src/text_splitter/splitter.rs (all 4 splitter types)

Enable true parallelism for LLM calls and text chunking operations.
"""

import os
import time
from concurrent.futures import ThreadPoolExecutor
from typing import Any

import pytest

import graphbit


class TestLLMGILRelease:
    """Tests to validate that GIL is released during LLM operations."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key

    @pytest.fixture
    def llm_client(self, api_key: str) -> Any:
        """Create OpenAI LLM client."""
        config = graphbit.LlmConfig.openai(
            api_key=api_key, model="gpt-3.5-turbo"
        )
        return graphbit.LlmClient(config)

    def test_complete_releases_gil(self, llm_client: Any) -> None:
        """Verify that complete() releases GIL and enables parallel execution.
        
        This test validates the GIL fix in python/src/llm/client.rs:310-383
        by measuring speedup from parallel execution.
        """
        prompts = [f"Say 'Test {i}' and nothing else." for i in range(5)]

        # Sequential execution (baseline)
        start_time = time.time()
        for prompt in prompts:
            llm_client.complete(prompt, max_tokens=10)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=3) as executor:
            list(executor.map(
                lambda p: llm_client.complete(p, max_tokens=10),
                prompts
            ))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - LLM complete():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        # Speedup > 2x indicates true parallelism (GIL released)
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during complete() execution."
        )

    def test_complete_full_releases_gil(self, llm_client: Any) -> None:
        """Verify that complete_full() releases GIL and enables parallel execution.
        
        This test validates the GIL fix in python/src/llm/client.rs:731-798
        by measuring speedup from parallel execution.
        """
        prompts = [f"Say 'Full test {i}' and nothing else." for i in range(5)]

        # Sequential execution (baseline)
        start_time = time.time()
        for prompt in prompts:
            llm_client.complete_full(prompt, max_tokens=10)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=3) as executor:
            list(executor.map(
                lambda p: llm_client.complete_full(p, max_tokens=10),
                prompts
            ))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - LLM complete_full():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during complete_full() execution."
        )

    def test_backward_compatibility_complete(self, llm_client: Any) -> None:
        """Verify existing code using complete() still works."""
        response = llm_client.complete("Say 'Hello'", max_tokens=10)
        
        assert isinstance(response, str)
        assert len(response) > 0

    def test_backward_compatibility_complete_full(self, llm_client: Any) -> None:
        """Verify existing code using complete_full() still works."""
        response = llm_client.complete_full("Say 'Hello'", max_tokens=10)
        
        assert hasattr(response, 'content')
        assert isinstance(response.content, str)
        assert len(response.content) > 0


class TestTextSplitterGILRelease:
    """Tests to validate that GIL is released during text splitting operations."""

    def test_character_splitter_releases_gil(self) -> None:
        """Verify that CharacterSplitter.split_text() releases GIL.
        
        This test validates the GIL fix in python/src/text_splitter/splitter.rs:109-120
        by measuring speedup from parallel execution.
        """
        splitter = graphbit.CharacterSplitter(chunk_size=100, chunk_overlap=20)
        
        # Create test texts
        texts = [f"This is test text number {i}. " * 50 for i in range(20)]

        # Sequential execution (baseline)
        start_time = time.time()
        for text in texts:
            splitter.split_text(text)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=5) as executor:
            list(executor.map(splitter.split_text, texts))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - CharacterSplitter.split_text():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during CharacterSplitter.split_text() execution."
        )

    def test_token_splitter_releases_gil(self) -> None:
        """Verify that TokenSplitter.split_text() releases GIL.
        
        This test validates the GIL fix in python/src/text_splitter/splitter.rs:185-208
        by measuring speedup from parallel execution.
        """
        splitter = graphbit.TokenSplitter(
            chunk_size=100,
            chunk_overlap=20,
            model_name="gpt-3.5-turbo"
        )
        
        # Create test texts
        texts = [f"This is test text number {i}. " * 50 for i in range(20)]

        # Sequential execution (baseline)
        start_time = time.time()
        for text in texts:
            splitter.split_text(text)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=5) as executor:
            list(executor.map(splitter.split_text, texts))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - TokenSplitter.split_text():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during TokenSplitter.split_text() execution."
        )

    def test_sentence_splitter_releases_gil(self) -> None:
        """Verify that SentenceSplitter.split_text() releases GIL.
        
        This test validates the GIL fix in python/src/text_splitter/splitter.rs:260-283
        by measuring speedup from parallel execution.
        """
        splitter = graphbit.SentenceSplitter(chunk_size=100, chunk_overlap=20)
        
        # Create test texts with sentences
        texts = [
            f"This is sentence {i}. This is another sentence. " * 25
            for i in range(20)
        ]

        # Sequential execution (baseline)
        start_time = time.time()
        for text in texts:
            splitter.split_text(text)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=5) as executor:
            list(executor.map(splitter.split_text, texts))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - SentenceSplitter.split_text():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during SentenceSplitter.split_text() execution."
        )

    def test_recursive_splitter_releases_gil(self) -> None:
        """Verify that RecursiveSplitter.split_text() releases GIL.
        
        This test validates the GIL fix in python/src/text_splitter/splitter.rs:336-359
        by measuring speedup from parallel execution.
        """
        splitter = graphbit.RecursiveSplitter(chunk_size=100, chunk_overlap=20)
        
        # Create test texts
        texts = [f"This is test text number {i}.\n\n" * 25 for i in range(20)]

        # Sequential execution (baseline)
        start_time = time.time()
        for text in texts:
            splitter.split_text(text)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=5) as executor:
            list(executor.map(splitter.split_text, texts))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - RecursiveSplitter.split_text():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during RecursiveSplitter.split_text() execution."
        )


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run tests
    pytest.main([__file__, "-v", "-s"])

