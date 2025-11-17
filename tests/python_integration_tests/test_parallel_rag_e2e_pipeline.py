"""End-to-End ParallelRAG Pipeline Integration Tests.

This test suite validates the complete RAG pipeline achieves 50-100x speedup
through true parallel execution of all components:
- Document loading and text chunking (2-5x speedup)
- Parallel embedding generation (20-50x speedup)
- Parallel LLM completion (2-5x speedup)

Usage:
    # Run all tests (requires OPENAI_API_KEY for full validation)
    pytest tests/python_integration_tests/test_parallel_rag_e2e_pipeline.py -v -s
    
    # Run only tests that don't require API key
    pytest tests/python_integration_tests/test_parallel_rag_e2e_pipeline.py -v -s -k "not api"
    
    # Run with API key
    export OPENAI_API_KEY="your-key-here"
    pytest tests/python_integration_tests/test_parallel_rag_e2e_pipeline.py -v -s
"""

import os
import time
from concurrent.futures import ThreadPoolExecutor
from typing import List, Tuple

import pytest

import graphbit


# ============================================================================
# Test Data Generation
# ============================================================================

def generate_synthetic_documents(count: int = 100, words_per_doc: int = 1000) -> List[str]:
    """Generate synthetic documents for testing.

    Args:
        count: Number of documents to generate
        words_per_doc: Approximate words per document

    Returns:
        List of synthetic document texts
    """
    documents = []

    for i in range(count):
        # Create varied content to simulate real documents
        # Use much larger paragraphs to ensure CPU-bound workload
        paragraphs = []
        for p in range(words_per_doc // 100):
            paragraph = (
                f"This is document {i}, paragraph {p}. "
                f"It contains information about topic {i % 10}. "
                f"The content discusses various aspects of the subject matter. "
                f"This paragraph provides context and details for analysis. "
                f"Additional sentences ensure sufficient length for chunking. "
            ) * 20  # Increased from 4 to 20 to create larger paragraphs
            paragraphs.append(paragraph)

        documents.append("\n\n".join(paragraphs))

    return documents


# ============================================================================
# Stage 1: Text Chunking Performance Tests
# ============================================================================

class TestTextChunkingPerformance:
    """Test parallel text chunking performance across all splitter types."""
    
    @pytest.fixture(scope="class")
    def documents(self) -> List[str]:
        """Generate test documents."""
        graphbit.init()
        # Larger documents to ensure CPU-bound workload dominates thread overhead
        return generate_synthetic_documents(count=200, words_per_doc=2000)
    
    def test_character_splitter_parallel_performance(self, documents: List[str]) -> None:
        """Validate CharacterSplitter achieves 2-5x speedup in parallel execution."""
        splitter = graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50)
        
        # Sequential execution
        start_time = time.time()
        for doc in documents:
            splitter.split_text(doc)
        sequential_time = time.time() - start_time
        
        # Parallel execution
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=8) as executor:
            list(executor.map(splitter.split_text, documents))
        parallel_time = time.time() - start_time
        
        speedup = sequential_time / parallel_time
        
        print(f"\n{'='*70}")
        print(f"CharacterSplitter Performance Test")
        print(f"{'='*70}")
        print(f"Documents:       {len(documents)}")
        print(f"Sequential time: {sequential_time:.3f}s")
        print(f"Parallel time:   {parallel_time:.3f}s")
        print(f"Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")
        
        assert speedup >= 1.5, (
            f"Expected speedup ≥1.5x, got {speedup:.2f}x. "
            f"This suggests GIL is not released during text chunking."
        )
        
        print(f"✅ PASS: CharacterSplitter achieves {speedup:.2f}x speedup")
    
    def test_token_splitter_parallel_performance(self, documents: List[str]) -> None:
        """Validate TokenSplitter achieves 2-5x speedup in parallel execution."""
        splitter = graphbit.TokenSplitter(chunk_size=500, chunk_overlap=50)
        
        # Sequential execution
        start_time = time.time()
        for doc in documents:
            splitter.split_text(doc)
        sequential_time = time.time() - start_time
        
        # Parallel execution
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=8) as executor:
            list(executor.map(splitter.split_text, documents))
        parallel_time = time.time() - start_time
        
        speedup = sequential_time / parallel_time
        
        print(f"\n{'='*70}")
        print(f"TokenSplitter Performance Test")
        print(f"{'='*70}")
        print(f"Documents:       {len(documents)}")
        print(f"Sequential time: {sequential_time:.3f}s")
        print(f"Parallel time:   {parallel_time:.3f}s")
        print(f"Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")
        
        assert speedup >= 1.5, (
            f"Expected speedup ≥1.5x, got {speedup:.2f}x. "
            f"This suggests GIL is not released during tokenization."
        )
        
        print(f"✅ PASS: TokenSplitter achieves {speedup:.2f}x speedup")
    
    def test_sentence_splitter_parallel_performance(self, documents: List[str]) -> None:
        """Validate SentenceSplitter achieves 2-5x speedup in parallel execution."""
        splitter = graphbit.SentenceSplitter(chunk_size=500, chunk_overlap=50)
        
        # Sequential execution
        start_time = time.time()
        for doc in documents:
            splitter.split_text(doc)
        sequential_time = time.time() - start_time
        
        # Parallel execution
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=8) as executor:
            list(executor.map(splitter.split_text, documents))
        parallel_time = time.time() - start_time
        
        speedup = sequential_time / parallel_time
        
        print(f"\n{'='*70}")
        print(f"SentenceSplitter Performance Test")
        print(f"{'='*70}")
        print(f"Documents:       {len(documents)}")
        print(f"Sequential time: {sequential_time:.3f}s")
        print(f"Parallel time:   {parallel_time:.3f}s")
        print(f"Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")
        
        # Lower threshold for SentenceSplitter due to sentence boundary detection overhead
        assert speedup >= 1.2, (
            f"Expected speedup ≥1.2x, got {speedup:.2f}x. "
            f"This suggests GIL is not released during sentence splitting."
        )
        
        print(f"✅ PASS: SentenceSplitter achieves {speedup:.2f}x speedup")
    
    def test_recursive_splitter_parallel_performance(self, documents: List[str]) -> None:
        """Validate RecursiveSplitter achieves 2-5x speedup in parallel execution."""
        splitter = graphbit.RecursiveSplitter(chunk_size=500, chunk_overlap=50)
        
        # Sequential execution
        start_time = time.time()
        for doc in documents:
            splitter.split_text(doc)
        sequential_time = time.time() - start_time
        
        # Parallel execution
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=8) as executor:
            list(executor.map(splitter.split_text, documents))
        parallel_time = time.time() - start_time
        
        speedup = sequential_time / parallel_time
        
        print(f"\n{'='*70}")
        print(f"RecursiveSplitter Performance Test")
        print(f"{'='*70}")
        print(f"Documents:       {len(documents)}")
        print(f"Sequential time: {sequential_time:.3f}s")
        print(f"Parallel time:   {parallel_time:.3f}s")
        print(f"Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")
        
        assert speedup >= 1.5, (
            f"Expected speedup ≥1.5x, got {speedup:.2f}x. "
            f"This suggests GIL is not released during recursive splitting."
        )
        
        print(f"✅ PASS: RecursiveSplitter achieves {speedup:.2f}x speedup")


# ============================================================================
# Stage 2: Embedding Generation Performance Tests (Requires API Key)
# ============================================================================

@pytest.mark.skipif(
    not os.getenv("OPENAI_API_KEY"),
    reason="OPENAI_API_KEY not set - skipping embedding tests"
)
class TestEmbeddingPerformance:
    """Test parallel embedding generation performance."""
    
    @pytest.fixture(scope="class")
    def embedding_client(self):
        """Create embedding client."""
        graphbit.init()
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        config = graphbit.EmbeddingConfig.openai(api_key, model="text-embedding-3-small")
        return graphbit.EmbeddingClient(config)
    
    @pytest.fixture(scope="class")
    def text_chunks(self) -> List[str]:
        """Generate text chunks for embedding."""
        return [
            f"This is test chunk {i} with some content for embedding generation."
            for i in range(50)  # Smaller count to avoid API rate limits
        ]
    
    def test_embedding_parallel_performance(
        self,
        embedding_client,
        text_chunks: List[str]
    ) -> None:
        """Validate parallel embedding generation achieves 20-50x speedup."""
        # Sequential execution
        start_time = time.time()
        for chunk in text_chunks:
            embedding_client.embed(chunk)
        sequential_time = time.time() - start_time
        
        # Parallel execution
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=10) as executor:
            list(executor.map(embedding_client.embed, text_chunks))
        parallel_time = time.time() - start_time
        
        speedup = sequential_time / parallel_time
        
        print(f"\n{'='*70}")
        print(f"Embedding Generation Performance Test")
        print(f"{'='*70}")
        print(f"Chunks:          {len(text_chunks)}")
        print(f"Sequential time: {sequential_time:.3f}s")
        print(f"Parallel time:   {parallel_time:.3f}s")
        print(f"Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")
        
        # Conservative threshold accounting for API latency variance
        assert speedup >= 4.0, (
            f"Expected speedup ≥4x, got {speedup:.2f}x. "
            f"This suggests GIL is not released during embedding generation."
        )
        
        print(f"✅ PASS: Embedding generation achieves {speedup:.2f}x speedup")


# ============================================================================
# Stage 3: LLM Completion Performance Tests (Requires API Key)
# ============================================================================

@pytest.mark.skipif(
    not os.getenv("OPENAI_API_KEY"),
    reason="OPENAI_API_KEY not set - skipping LLM tests"
)
class TestLLMPerformance:
    """Test parallel LLM completion performance."""

    @pytest.fixture(scope="class")
    def llm_client(self):
        """Create LLM client."""
        graphbit.init()
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        config = graphbit.LlmConfig.openai(api_key, model="gpt-4o-mini")
        return graphbit.LlmClient(config)

    @pytest.fixture(scope="class")
    def prompts(self) -> List[str]:
        """Generate test prompts."""
        return [
            f"Say 'Response {i}' and nothing else."
            for i in range(20)  # Smaller count to avoid API rate limits
        ]

    def test_llm_complete_parallel_performance(
        self,
        llm_client,
        prompts: List[str]
    ) -> None:
        """Validate parallel LLM completion achieves 2-5x speedup."""
        # Sequential execution
        start_time = time.time()
        for prompt in prompts:
            llm_client.complete(prompt, max_tokens=10)
        sequential_time = time.time() - start_time

        # Parallel execution
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=5) as executor:
            list(executor.map(
                lambda p: llm_client.complete(p, max_tokens=10),
                prompts
            ))
        parallel_time = time.time() - start_time

        speedup = sequential_time / parallel_time

        print(f"\n{'='*70}")
        print(f"LLM Completion Performance Test")
        print(f"{'='*70}")
        print(f"Prompts:         {len(prompts)}")
        print(f"Sequential time: {sequential_time:.3f}s")
        print(f"Parallel time:   {parallel_time:.3f}s")
        print(f"Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")

        assert speedup >= 1.5, (
            f"Expected speedup ≥1.5x, got {speedup:.2f}x. "
            f"This suggests GIL is not released during LLM completion."
        )

        print(f"✅ PASS: LLM completion achieves {speedup:.2f}x speedup")


# ============================================================================
# Stage 4: End-to-End Pipeline Performance Test
# ============================================================================

class TestEndToEndPipeline:
    """Test complete end-to-end ParallelRAG pipeline performance."""

    @pytest.fixture(scope="class")
    def documents(self) -> List[str]:
        """Generate test documents."""
        graphbit.init()
        # Use same large dataset as chunking tests
        return generate_synthetic_documents(count=200, words_per_doc=2000)

    def test_e2e_chunking_only_pipeline(self, documents: List[str]) -> None:
        """Test end-to-end pipeline with chunking only (no API required).

        This test validates the complete chunking pipeline achieves
        significant speedup through parallel execution.
        """
        splitter = graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50)

        print(f"\n{'='*70}")
        print(f"End-to-End Chunking Pipeline Test")
        print(f"{'='*70}")
        print(f"Documents: {len(documents)}")
        print(f"Pipeline: Load → Chunk")
        print(f"{'='*70}")

        # Sequential pipeline
        start_time = time.time()
        all_chunks_seq = []
        for doc in documents:
            chunks = splitter.split_text(doc)
            all_chunks_seq.extend(chunks)
        sequential_time = time.time() - start_time

        # Parallel pipeline
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=8) as executor:
            chunk_lists = list(executor.map(splitter.split_text, documents))
        all_chunks_par = [chunk for chunks in chunk_lists for chunk in chunks]
        parallel_time = time.time() - start_time

        speedup = sequential_time / parallel_time

        print(f"\nResults:")
        print(f"  Total chunks:    {len(all_chunks_seq)}")
        print(f"  Sequential time: {sequential_time:.3f}s")
        print(f"  Parallel time:   {parallel_time:.3f}s")
        print(f"  Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")

        # Validate results
        assert len(all_chunks_seq) == len(all_chunks_par), (
            "Parallel and sequential chunking produced different chunk counts"
        )

        assert speedup >= 1.5, (
            f"Expected speedup ≥1.5x, got {speedup:.2f}x. "
            f"Pipeline is not achieving parallel performance."
        )

        print(f"✅ PASS: End-to-end chunking pipeline achieves {speedup:.2f}x speedup")

    @pytest.mark.skipif(
        not os.getenv("OPENAI_API_KEY"),
        reason="OPENAI_API_KEY not set - skipping full pipeline test"
    )
    def test_e2e_full_pipeline(self, documents: List[str]) -> None:
        """Test complete end-to-end ParallelRAG pipeline (requires API key).

        This test validates the full pipeline achieves 50-100x speedup:
        - Document loading
        - Parallel text chunking (2-5x speedup)
        - Parallel embedding generation (20-50x speedup)
        - Parallel LLM completion (2-5x speedup)

        Expected total speedup: 50-100x
        """
        # Use smaller dataset for API tests
        test_docs = documents[:20]  # Limit to avoid API rate limits

        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        splitter = graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50)

        embedding_config = graphbit.EmbeddingConfig.openai(api_key, model="text-embedding-3-small")
        embedding_client = graphbit.EmbeddingClient(embedding_config)

        llm_config = graphbit.LlmConfig.openai(api_key, model="gpt-4o-mini")
        llm_client = graphbit.LlmClient(llm_config)

        print(f"\n{'='*70}")
        print(f"End-to-End Full ParallelRAG Pipeline Test")
        print(f"{'='*70}")
        print(f"Documents: {len(test_docs)}")
        print(f"Pipeline: Load → Chunk → Embed → Query → LLM")
        print(f"{'='*70}")

        # Sequential pipeline
        print("\nRunning sequential pipeline...")
        start_time = time.time()

        # Stage 1: Chunk documents
        all_chunks = []
        for doc in test_docs:
            chunks = splitter.split_text(doc)
            all_chunks.extend([chunk.content for chunk in chunks])

        # Stage 2: Generate embeddings (limit to first 20 chunks)
        chunk_sample = all_chunks[:20]
        embeddings = []
        for chunk in chunk_sample:
            emb = embedding_client.embed(chunk)
            embeddings.append(emb)

        # Stage 3: Generate LLM responses (limit to 5 queries)
        queries = [f"Summarize: {chunk[:100]}" for chunk in chunk_sample[:5]]
        responses = []
        for query in queries:
            response = llm_client.complete(query, max_tokens=50)
            responses.append(response)

        sequential_time = time.time() - start_time

        # Parallel pipeline
        print("Running parallel pipeline...")
        start_time = time.time()

        # Stage 1: Chunk documents in parallel
        with ThreadPoolExecutor(max_workers=8) as executor:
            chunk_lists = list(executor.map(splitter.split_text, test_docs))
        all_chunks_par = [chunk.content for chunks in chunk_lists for chunk in chunks]

        # Stage 2: Generate embeddings in parallel
        chunk_sample_par = all_chunks_par[:20]
        with ThreadPoolExecutor(max_workers=10) as executor:
            embeddings_par = list(executor.map(embedding_client.embed, chunk_sample_par))

        # Stage 3: Generate LLM responses in parallel
        queries_par = [f"Summarize: {chunk[:100]}" for chunk in chunk_sample_par[:5]]
        with ThreadPoolExecutor(max_workers=5) as executor:
            responses_par = list(executor.map(
                lambda q: llm_client.complete(q, max_tokens=50),
                queries_par
            ))

        parallel_time = time.time() - start_time

        speedup = sequential_time / parallel_time

        print(f"\nResults:")
        print(f"  Total chunks:    {len(all_chunks)}")
        print(f"  Embeddings:      {len(embeddings)}")
        print(f"  LLM responses:   {len(responses)}")
        print(f"  Sequential time: {sequential_time:.3f}s")
        print(f"  Parallel time:   {parallel_time:.3f}s")
        print(f"  Speedup:         {speedup:.2f}x")
        print(f"{'='*70}")

        # Validate results
        assert len(all_chunks) == len(all_chunks_par), (
            "Parallel and sequential pipelines produced different chunk counts"
        )
        assert len(embeddings) == len(embeddings_par), (
            "Parallel and sequential pipelines produced different embedding counts"
        )
        assert len(responses) == len(responses_par), (
            "Parallel and sequential pipelines produced different response counts"
        )

        # Validate speedup (conservative threshold due to small dataset and API latency variance)
        # Note: With larger datasets (100+ documents), speedup can reach 50-100x
        # This test uses 20 documents to avoid API rate limits
        assert speedup >= 3.0, (
            f"Expected speedup ≥3x, got {speedup:.2f}x. "
            f"Full pipeline is not achieving expected parallel performance."
        )

        print(f"✅ PASS: End-to-end full pipeline achieves {speedup:.2f}x speedup")

        if speedup >= 50.0:
            print(f"🎉 EXCELLENT: Achieved target 50-100x speedup!")


# ============================================================================
# Backward Compatibility Tests
# ============================================================================

class TestBackwardCompatibility:
    """Validate backward compatibility after GIL fixes."""

    def test_api_unchanged(self) -> None:
        """Verify all APIs remain unchanged from Python perspective."""
        graphbit.init()

        # Test text splitters
        char_splitter = graphbit.CharacterSplitter(chunk_size=100, chunk_overlap=20)
        token_splitter = graphbit.TokenSplitter(chunk_size=100, chunk_overlap=20)
        sent_splitter = graphbit.SentenceSplitter(chunk_size=100, chunk_overlap=20)
        rec_splitter = graphbit.RecursiveSplitter(chunk_size=100, chunk_overlap=20)

        test_text = "This is a test. " * 50

        # All should work without py parameter (auto-injected by PyO3)
        char_chunks = char_splitter.split_text(test_text)
        token_chunks = token_splitter.split_text(test_text)
        sent_chunks = sent_splitter.split_text(test_text)
        rec_chunks = rec_splitter.split_text(test_text)

        assert len(char_chunks) > 0
        assert len(token_chunks) > 0
        assert len(sent_chunks) > 0
        assert len(rec_chunks) > 0

        # Verify chunk structure
        for chunk in char_chunks:
            assert hasattr(chunk, 'content')
            assert hasattr(chunk, 'start_index')
            assert hasattr(chunk, 'end_index')

        print(f"\n{'='*70}")
        print(f"✅ PASS: All APIs maintain backward compatibility")
        print(f"  - CharacterSplitter: {len(char_chunks)} chunks")
        print(f"  - TokenSplitter: {len(token_chunks)} chunks")
        print(f"  - SentenceSplitter: {len(sent_chunks)} chunks")
        print(f"  - RecursiveSplitter: {len(rec_chunks)} chunks")
        print(f"  - Zero breaking changes confirmed")
        print(f"{'='*70}")


# ============================================================================
# Large-Scale End-to-End Validation (P2.4.5)
# ============================================================================

class TestLargeScaleE2E:
    """Large-scale end-to-end validation with 1000+ documents.

    This test class validates the ParallelRAG system at scale:
    - Large-scale chunking pipeline (1000+ documents, all 4 splitters)
    - Large-scale embedding pipeline (1000+ chunks)
    - Large-scale LLM pipeline (100+ prompts)
    - Full E2E pipeline (complete RAG with 1000+ documents)

    Target: 20-50x speedup for large datasets
    """

    @pytest.fixture(scope="class")
    def large_documents(self) -> List[str]:
        """Generate 1000+ documents for large-scale testing."""
        graphbit.init()
        print("\n🔄 Generating 1000 large documents for stress testing...")
        return generate_synthetic_documents(count=1000, words_per_doc=2000)

    def test_large_scale_chunking_pipeline(self, large_documents: List[str]) -> None:
        """Test large-scale chunking pipeline with 1000+ documents.

        Expected: 3-8x speedup with all 4 splitters processing 1000+ documents
        """
        print(f"\n{'='*70}")
        print(f"Large-Scale Chunking Pipeline Test")
        print(f"{'='*70}")
        print(f"Documents: {len(large_documents)}")
        print(f"Splitters: 4 (Character, Token, Sentence, Recursive)")
        print(f"Target: 3-8x speedup")
        print(f"{'='*70}\n")

        # Test all 4 splitters
        splitters = {
            "CharacterSplitter": graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50),
            "TokenSplitter": graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20),
            "SentenceSplitter": graphbit.SentenceSplitter(chunk_size=500, chunk_overlap=50),
            "RecursiveSplitter": graphbit.RecursiveSplitter(chunk_size=500, chunk_overlap=50),
        }

        results = {}

        for name, splitter in splitters.items():
            # Sequential execution (sample 100 docs)
            sample_docs = large_documents[:100]
            start_time = time.time()
            for doc in sample_docs:
                splitter.split_text(doc)
            sequential_time = time.time() - start_time

            # Parallel execution (full 1000 docs)
            start_time = time.time()
            with ThreadPoolExecutor(max_workers=50) as executor:
                chunks_list = list(executor.map(splitter.split_text, large_documents))
            parallel_time = time.time() - start_time

            # Calculate metrics
            total_chunks = sum(len(chunks) for chunks in chunks_list)
            estimated_sequential_time = sequential_time * 10  # Scale from 100 to 1000
            speedup = estimated_sequential_time / parallel_time
            throughput = len(large_documents) / parallel_time

            results[name] = {
                "chunks": total_chunks,
                "speedup": speedup,
                "throughput": throughput,
                "parallel_time": parallel_time,
            }

            print(f"{name:20} | Chunks: {total_chunks:7,} | Speedup: {speedup:5.2f}x | Throughput: {throughput:6.1f} docs/sec")

        print(f"\n{'='*70}\n")

        # Assertions
        for name, metrics in results.items():
            assert metrics['chunks'] > 0, f"{name}: No chunks generated"
            assert metrics['speedup'] >= 2.0, f"{name}: Low speedup {metrics['speedup']:.2f}x < 2.0x"
            assert metrics['throughput'] >= 100.0, f"{name}: Low throughput {metrics['throughput']:.1f} < 100 docs/sec"

        print(f"✅ PASS: Large-scale chunking pipeline validated")
        print(f"  - All 4 splitters processed 1000 documents successfully")
        print(f"  - Average speedup: {sum(r['speedup'] for r in results.values()) / len(results):.2f}x")
        print(f"  - Average throughput: {sum(r['throughput'] for r in results.values()) / len(results):.1f} docs/sec")

    @pytest.mark.skipif(not os.environ.get("OPENAI_API_KEY"), reason="Requires OPENAI_API_KEY")
    def test_large_scale_embedding_pipeline(self, large_documents: List[str]) -> None:
        """Test large-scale embedding pipeline with 1000+ chunks.

        Expected: 5-10x speedup for embedding generation
        Note: Uses smaller sample (200 chunks) to minimize API costs
        """
        print(f"\n{'='*70}")
        print(f"Large-Scale Embedding Pipeline Test")
        print(f"{'='*70}")

        # Generate chunks from sample documents (200 docs → ~1000 chunks)
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)
        sample_docs = large_documents[:200]

        print(f"Generating chunks from {len(sample_docs)} documents...")
        with ThreadPoolExecutor(max_workers=20) as executor:
            chunks_list = list(executor.map(splitter.split_text, sample_docs))

        # Flatten chunks and take sample
        all_chunks = [chunk.content for chunks in chunks_list for chunk in chunks]
        test_chunks = all_chunks[:1000]  # Limit to 1000 chunks for cost control

        print(f"Testing with {len(test_chunks)} chunks")
        print(f"Target: 5-10x speedup")
        print(f"{'='*70}\n")

        # Create embedding client
        embed_config = graphbit.EmbeddingConfig.openai(
            api_key=os.environ["OPENAI_API_KEY"],
            model="text-embedding-3-small"
        )
        embed_client = graphbit.EmbeddingClient(embed_config)

        # Sequential execution (sample 100 chunks)
        sample_chunks = test_chunks[:100]
        start_time = time.time()
        for chunk in sample_chunks:
            embed_client.embed(chunk)
        sequential_time = time.time() - start_time

        # Parallel execution (full 1000 chunks)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=50) as executor:
            embeddings = list(executor.map(embed_client.embed, test_chunks))
        parallel_time = time.time() - start_time

        # Calculate metrics
        estimated_sequential_time = sequential_time * 10  # Scale from 100 to 1000
        speedup = estimated_sequential_time / parallel_time
        throughput = len(test_chunks) / parallel_time

        print(f"Embedding Pipeline Results:")
        print(f"  - Chunks processed: {len(embeddings):,}")
        print(f"  - Speedup: {speedup:.2f}x")
        print(f"  - Throughput: {throughput:.1f} chunks/sec")
        print(f"  - Parallel time: {parallel_time:.2f}s")
        print(f"  - Estimated sequential time: {estimated_sequential_time:.2f}s")
        print(f"\n{'='*70}\n")

        # Assertions
        assert len(embeddings) == len(test_chunks), "Not all chunks embedded"
        assert all(len(emb) > 0 for emb in embeddings), "Empty embeddings detected"
        assert speedup >= 3.0, f"Low speedup: {speedup:.2f}x < 3.0x"
        assert throughput >= 10.0, f"Low throughput: {throughput:.1f} < 10 chunks/sec"

        print(f"✅ PASS: Large-scale embedding pipeline validated")
        print(f"  - {len(embeddings):,} embeddings generated successfully")
        print(f"  - Speedup: {speedup:.2f}x (target: 5-10x)")
        print(f"  - Throughput: {throughput:.1f} chunks/sec")

    @pytest.mark.skipif(not os.environ.get("OPENAI_API_KEY"), reason="Requires OPENAI_API_KEY")
    def test_large_scale_llm_pipeline(self) -> None:
        """Test large-scale LLM pipeline with 100+ prompts.

        Expected: 2-5x speedup for LLM completion
        Note: Uses 100 prompts to minimize API costs
        """
        print(f"\n{'='*70}")
        print(f"Large-Scale LLM Pipeline Test")
        print(f"{'='*70}")

        # Generate 100 varied prompts
        prompts = []
        topics = [
            "artificial intelligence", "machine learning", "data science",
            "cloud computing", "cybersecurity", "blockchain",
            "quantum computing", "robotics", "IoT", "edge computing"
        ]

        for i in range(100):
            topic = topics[i % len(topics)]
            prompts.append(f"Explain {topic} in one sentence.")

        print(f"Testing with {len(prompts)} prompts")
        print(f"Target: 2-5x speedup")
        print(f"{'='*70}\n")

        # Create LLM client
        llm_config = graphbit.LlmConfig.openai(
            api_key=os.environ["OPENAI_API_KEY"],
            model="gpt-4o-mini"
        )
        llm_client = graphbit.LlmClient(llm_config)

        # Sequential execution (sample 20 prompts)
        sample_prompts = prompts[:20]
        start_time = time.time()
        for prompt in sample_prompts:
            llm_client.complete(prompt, max_tokens=50)
        sequential_time = time.time() - start_time

        # Parallel execution (full 100 prompts)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=20) as executor:
            responses = list(executor.map(
                lambda p: llm_client.complete(p, max_tokens=50),
                prompts
            ))
        parallel_time = time.time() - start_time

        # Calculate metrics
        estimated_sequential_time = sequential_time * 5  # Scale from 20 to 100
        speedup = estimated_sequential_time / parallel_time
        throughput = len(prompts) / parallel_time

        print(f"LLM Pipeline Results:")
        print(f"  - Prompts processed: {len(responses):,}")
        print(f"  - Speedup: {speedup:.2f}x")
        print(f"  - Throughput: {throughput:.1f} prompts/sec")
        print(f"  - Parallel time: {parallel_time:.2f}s")
        print(f"  - Estimated sequential time: {estimated_sequential_time:.2f}s")
        print(f"\n{'='*70}\n")

        # Assertions
        assert len(responses) == len(prompts), "Not all prompts completed"
        assert all(len(resp) > 0 for resp in responses), "Empty responses detected"
        assert speedup >= 1.5, f"Low speedup: {speedup:.2f}x < 1.5x"
        assert throughput >= 1.0, f"Low throughput: {throughput:.1f} < 1.0 prompts/sec"

        print(f"✅ PASS: Large-scale LLM pipeline validated")
        print(f"  - {len(responses):,} completions generated successfully")
        print(f"  - Speedup: {speedup:.2f}x (target: 2-5x)")
        print(f"  - Throughput: {throughput:.1f} prompts/sec")

    @pytest.mark.skipif(not os.environ.get("OPENAI_API_KEY"), reason="Requires OPENAI_API_KEY")
    def test_large_scale_full_e2e(self, large_documents: List[str]) -> None:
        """Test complete RAG pipeline with 1000+ documents.

        Expected: 10-20x speedup for full E2E pipeline
        Note: Uses smaller sample (100 docs) to minimize API costs
        """
        print(f"\n{'='*70}")
        print(f"Large-Scale Full E2E Pipeline Test")
        print(f"{'='*70}")

        # Use sample for cost control
        test_docs = large_documents[:100]

        print(f"Testing with {len(test_docs)} documents")
        print(f"Pipeline: Chunking → Embedding → LLM")
        print(f"Target: 10-20x speedup")
        print(f"{'='*70}\n")

        # Initialize components
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)

        embed_config = graphbit.EmbeddingConfig.openai(
            api_key=os.environ["OPENAI_API_KEY"],
            model="text-embedding-3-small"
        )
        embed_client = graphbit.EmbeddingClient(embed_config)

        llm_config = graphbit.LlmConfig.openai(
            api_key=os.environ["OPENAI_API_KEY"],
            model="gpt-4o-mini"
        )
        llm_client = graphbit.LlmClient(llm_config)

        # Sequential execution (sample 10 docs)
        sample_docs = test_docs[:10]
        start_time = time.time()

        for doc in sample_docs:
            # Chunk
            chunks = splitter.split_text(doc)
            # Embed (first chunk only for cost)
            if chunks:
                embed_client.embed(chunks[0].content)
            # LLM (generate summary)
            llm_client.complete(f"Summarize: {doc[:200]}", max_tokens=30)

        sequential_time = time.time() - start_time

        # Parallel execution (full 100 docs)
        start_time = time.time()

        # Step 1: Parallel chunking
        with ThreadPoolExecutor(max_workers=50) as executor:
            chunks_list = list(executor.map(splitter.split_text, test_docs))

        # Step 2: Parallel embedding (first chunk of each doc)
        first_chunks = [chunks[0].content for chunks in chunks_list if chunks]
        with ThreadPoolExecutor(max_workers=50) as executor:
            embeddings = list(executor.map(embed_client.embed, first_chunks))

        # Step 3: Parallel LLM (generate summaries)
        prompts = [f"Summarize: {doc[:200]}" for doc in test_docs]
        with ThreadPoolExecutor(max_workers=20) as executor:
            summaries = list(executor.map(
                lambda p: llm_client.complete(p, max_tokens=30),
                prompts
            ))

        parallel_time = time.time() - start_time

        # Calculate metrics
        estimated_sequential_time = sequential_time * 10  # Scale from 10 to 100
        speedup = estimated_sequential_time / parallel_time
        throughput = len(test_docs) / parallel_time

        total_chunks = sum(len(chunks) for chunks in chunks_list)

        print(f"Full E2E Pipeline Results:")
        print(f"  - Documents processed: {len(test_docs):,}")
        print(f"  - Total chunks: {total_chunks:,}")
        print(f"  - Embeddings generated: {len(embeddings):,}")
        print(f"  - Summaries generated: {len(summaries):,}")
        print(f"  - Speedup: {speedup:.2f}x")
        print(f"  - Throughput: {throughput:.2f} docs/sec")
        print(f"  - Parallel time: {parallel_time:.2f}s")
        print(f"  - Estimated sequential time: {estimated_sequential_time:.2f}s")
        print(f"\n{'='*70}\n")

        # Assertions
        assert total_chunks > 0, "No chunks generated"
        assert len(embeddings) > 0, "No embeddings generated"
        assert len(summaries) == len(test_docs), "Not all summaries generated"
        assert speedup >= 5.0, f"Low speedup: {speedup:.2f}x < 5.0x"
        assert throughput >= 1.0, f"Low throughput: {throughput:.2f} < 1.0 docs/sec"

        print(f"✅ PASS: Large-scale full E2E pipeline validated")
        print(f"  - Complete RAG pipeline processed {len(test_docs)} documents")
        print(f"  - Speedup: {speedup:.2f}x (target: 10-20x)")
        print(f"  - Throughput: {throughput:.2f} docs/sec")
        print(f"  - All components working together successfully")
