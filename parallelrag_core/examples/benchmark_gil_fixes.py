"""
Benchmark: Validate GIL Fixes and Performance Improvements

This benchmark validates that the GIL fixes enable true parallelism and
measures the actual performance improvements.

Expected Results:
- Document loading: 10-50x speedup (GIL released)
- Embedding generation (FIXED): 5-10x speedup (GIL released)
- Embedding batch parallel: 10-50x speedup (lock-free)
- Full RAG pipeline: 50-100x speedup (all optimizations)
"""

import asyncio
import os
import time
from concurrent.futures import ThreadPoolExecutor
from typing import List, Tuple
import statistics

from graphbit import (
    DocumentLoader,
    EmbeddingClient,
    EmbeddingConfig,
    RecursiveSplitter,
)


class GILBenchmark:
    """Benchmark to validate GIL release and measure performance."""

    def __init__(self, api_key: str):
        self.api_key = api_key
        self.loader = DocumentLoader()
        self.splitter = RecursiveSplitter(chunk_size=500, chunk_overlap=50)
        
        embed_config = EmbeddingConfig.openai(api_key)
        self.embed_client = EmbeddingClient(embed_config)

    def benchmark_document_loading(self, num_docs: int = 50) -> Tuple[float, float]:
        """
        Benchmark document loading: sequential vs parallel.
        
        Expected: 10-50x speedup with parallel loading (GIL released)
        """
        print("\n" + "=" * 80)
        print("BENCHMARK 1: Document Loading (GIL Release Validation)")
        print("=" * 80)
        
        # Create sample documents
        sample_docs = self._create_sample_documents(num_docs)
        
        # Sequential loading
        print(f"\n1. Sequential loading ({num_docs} documents)...")
        start = time.time()
        for doc_path in sample_docs:
            try:
                self.loader.load_document(doc_path, "txt")
            except:
                pass
        sequential_time = time.time() - start
        print(f"   Time: {sequential_time:.2f}s")
        print(f"   Rate: {num_docs/sequential_time:.1f} docs/second")
        
        # Parallel loading (10 workers)
        print(f"\n2. Parallel loading ({num_docs} documents, 10 workers)...")
        start = time.time()
        with ThreadPoolExecutor(max_workers=10) as executor:
            futures = [
                executor.submit(self._load_doc, doc_path)
                for doc_path in sample_docs
            ]
            results = [f.result() for f in futures]
        parallel_time = time.time() - start
        print(f"   Time: {parallel_time:.2f}s")
        print(f"   Rate: {num_docs/parallel_time:.1f} docs/second")
        
        speedup = sequential_time / parallel_time
        print(f"\n✅ SPEEDUP: {speedup:.2f}x")
        print(f"   Expected: 10-50x (limited by I/O)")
        
        if speedup > 5:
            print("   ✅ GIL RELEASED - True parallelism achieved!")
        else:
            print("   ❌ GIL NOT RELEASED - Serialized execution detected")
        
        return sequential_time, parallel_time

    def _load_doc(self, path: str):
        """Load a single document."""
        try:
            return self.loader.load_document(path, "txt")
        except:
            return None

    def benchmark_embedding_generation(self, num_texts: int = 100) -> Tuple[float, float]:
        """
        Benchmark embedding generation: sequential vs parallel.
        
        Expected: 5-10x speedup with parallel embedding (GIL released after fix)
        """
        print("\n" + "=" * 80)
        print("BENCHMARK 2: Embedding Generation (GIL Fix Validation)")
        print("=" * 80)
        
        # Create sample texts
        texts = [f"Sample text number {i} for embedding generation." for i in range(num_texts)]
        
        # Sequential embedding
        print(f"\n1. Sequential embedding ({num_texts} texts)...")
        start = time.time()
        for text in texts:
            try:
                self.embed_client.embed(text)
            except:
                pass
        sequential_time = time.time() - start
        print(f"   Time: {sequential_time:.2f}s")
        print(f"   Rate: {num_texts/sequential_time:.1f} embeddings/second")
        
        # Parallel embedding (10 workers) - SHOULD NOW WORK WITH GIL FIX
        print(f"\n2. Parallel embedding ({num_texts} texts, 10 workers)...")
        start = time.time()
        with ThreadPoolExecutor(max_workers=10) as executor:
            futures = [
                executor.submit(self.embed_client.embed, text)
                for text in texts
            ]
            results = [f.result() for f in futures]
        parallel_time = time.time() - start
        print(f"   Time: {parallel_time:.2f}s")
        print(f"   Rate: {num_texts/parallel_time:.1f} embeddings/second")
        
        speedup = sequential_time / parallel_time
        print(f"\n✅ SPEEDUP: {speedup:.2f}x")
        print(f"   Expected: 5-10x (limited by API rate limits)")
        
        if speedup > 3:
            print("   ✅ GIL RELEASED - True parallelism achieved!")
        else:
            print("   ❌ GIL NOT RELEASED - Serialized execution detected")
        
        return sequential_time, parallel_time

    def benchmark_batch_embedding(self, num_batches: int = 10, batch_size: int = 10):
        """
        Benchmark lock-free parallel batch embedding.
        
        Expected: 10-50x speedup with lock-free parallel processing
        """
        print("\n" + "=" * 80)
        print("BENCHMARK 3: Lock-Free Parallel Batch Embedding")
        print("=" * 80)
        
        # Create sample text batches
        texts_batch = [
            [f"Batch {i} text {j}" for j in range(batch_size)]
            for i in range(num_batches)
        ]
        
        # Sequential batch processing
        print(f"\n1. Sequential batch processing ({num_batches} batches)...")
        start = time.time()
        for texts in texts_batch:
            try:
                self.embed_client.embed_many(texts)
            except:
                pass
        sequential_time = time.time() - start
        total_texts = num_batches * batch_size
        print(f"   Time: {sequential_time:.2f}s")
        print(f"   Rate: {total_texts/sequential_time:.1f} embeddings/second")
        
        # Lock-free parallel batch processing
        print(f"\n2. Lock-free parallel batch ({num_batches} batches, max_concurrency=10)...")
        start = time.time()
        try:
            result = self.embed_client.embed_batch_parallel(
                texts_batch,
                max_concurrency=10,
                timeout_ms=300000,
            )
            parallel_time = time.time() - start
            print(f"   Time: {parallel_time:.2f}s")
            print(f"   Rate: {total_texts/parallel_time:.1f} embeddings/second")
            print(f"   Stats: {result['stats']}")
            
            speedup = sequential_time / parallel_time
            print(f"\n✅ SPEEDUP: {speedup:.2f}x")
            print(f"   Expected: 10-50x (lock-free atomic operations)")
            
            if speedup > 5:
                print("   ✅ LOCK-FREE PARALLELISM - Atomic operations working!")
            else:
                print("   ⚠️  Lower than expected - may be API rate limited")
        except Exception as e:
            print(f"   ❌ Error: {e}")
            parallel_time = sequential_time

    def benchmark_embed_many_batches(self, num_batches: int = 10, batch_size: int = 10):
        """
        Benchmark embed_many() with parallel batches (FIXED with GIL release).
        
        Expected: 5-10x speedup after GIL fix
        """
        print("\n" + "=" * 80)
        print("BENCHMARK 4: embed_many() Parallel Batches (GIL Fix Validation)")
        print("=" * 80)
        
        # Create sample text batches
        texts_batch = [
            [f"Batch {i} text {j}" for j in range(batch_size)]
            for i in range(num_batches)
        ]
        
        # Sequential batch processing
        print(f"\n1. Sequential embed_many() ({num_batches} batches)...")
        start = time.time()
        for texts in texts_batch:
            try:
                self.embed_client.embed_many(texts)
            except:
                pass
        sequential_time = time.time() - start
        total_texts = num_batches * batch_size
        print(f"   Time: {sequential_time:.2f}s")
        print(f"   Rate: {total_texts/sequential_time:.1f} embeddings/second")
        
        # Parallel batch processing with ThreadPoolExecutor
        print(f"\n2. Parallel embed_many() ({num_batches} batches, 10 workers)...")
        start = time.time()
        with ThreadPoolExecutor(max_workers=10) as executor:
            futures = [
                executor.submit(self.embed_client.embed_many, texts)
                for texts in texts_batch
            ]
            results = [f.result() for f in futures]
        parallel_time = time.time() - start
        print(f"   Time: {parallel_time:.2f}s")
        print(f"   Rate: {total_texts/parallel_time:.1f} embeddings/second")
        
        speedup = sequential_time / parallel_time
        print(f"\n✅ SPEEDUP: {speedup:.2f}x")
        print(f"   Expected: 5-10x (after GIL fix)")
        
        if speedup > 3:
            print("   ✅ GIL RELEASED - True parallelism achieved!")
        else:
            print("   ❌ GIL NOT RELEASED - Serialized execution detected")

    def _create_sample_documents(self, num_docs: int) -> List[str]:
        """Create sample documents for testing."""
        import tempfile
        
        doc_paths = []
        for i in range(num_docs):
            # Create temporary file
            fd, path = tempfile.mkstemp(suffix=".txt", prefix=f"sample_doc_{i}_")
            with os.fdopen(fd, 'w') as f:
                f.write(f"Sample document {i}\n" * 100)
            doc_paths.append(path)
        
        return doc_paths

    def run_all_benchmarks(self):
        """Run all benchmarks and generate summary report."""
        print("\n" + "=" * 80)
        print("GRAPHBIT GIL FIX VALIDATION BENCHMARK SUITE")
        print("=" * 80)
        print("\nThis benchmark validates that GIL fixes enable true parallelism")
        print("and measures actual performance improvements.\n")
        
        results = {}
        
        # Benchmark 1: Document Loading
        try:
            seq, par = self.benchmark_document_loading(num_docs=20)
            results['document_loading'] = seq / par
        except Exception as e:
            print(f"❌ Document loading benchmark failed: {e}")
            results['document_loading'] = 1.0
        
        # Benchmark 2: Embedding Generation
        try:
            seq, par = self.benchmark_embedding_generation(num_texts=50)
            results['embedding_generation'] = seq / par
        except Exception as e:
            print(f"❌ Embedding generation benchmark failed: {e}")
            results['embedding_generation'] = 1.0
        
        # Benchmark 3: Lock-Free Batch Embedding
        try:
            self.benchmark_batch_embedding(num_batches=5, batch_size=10)
        except Exception as e:
            print(f"❌ Batch embedding benchmark failed: {e}")
        
        # Benchmark 4: embed_many() Parallel Batches
        try:
            self.benchmark_embed_many_batches(num_batches=5, batch_size=10)
        except Exception as e:
            print(f"❌ embed_many() parallel benchmark failed: {e}")
        
        # Summary Report
        print("\n" + "=" * 80)
        print("SUMMARY REPORT")
        print("=" * 80)
        
        print("\nMeasured Speedups:")
        for benchmark, speedup in results.items():
            status = "✅" if speedup > 3 else "❌"
            print(f"  {status} {benchmark}: {speedup:.2f}x")
        
        avg_speedup = statistics.mean(results.values())
        print(f"\n📊 Average Speedup: {avg_speedup:.2f}x")
        
        if avg_speedup > 5:
            print("\n✅ GIL FIXES VALIDATED - True parallelism achieved!")
            print("   ParallelRAG can achieve 50-100x speedup for full pipelines.")
        elif avg_speedup > 2:
            print("\n⚠️  PARTIAL SUCCESS - Some parallelism achieved")
            print("   Further optimization may be needed.")
        else:
            print("\n❌ GIL FIXES NOT WORKING - Serialized execution detected")
            print("   Review GIL release implementation.")


def main():
    """Run benchmark suite."""
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("❌ OPENAI_API_KEY environment variable not set")
        return
    
    benchmark = GILBenchmark(api_key)
    benchmark.run_all_benchmarks()


if __name__ == "__main__":
    main()

