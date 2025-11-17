#!/usr/bin/env python3
"""
Test GraphBit performance with varying worker counts to find optimal configuration.
"""

import json
import time
import graphbit
from pathlib import Path
import tempfile
from typing import List, Dict, Any

def generate_test_documents(num_docs: int, words_per_doc: int) -> List[str]:
    """Generate test documents and return their paths."""
    temp_dir = Path(tempfile.mkdtemp())
    doc_paths = []
    
    # Generate sample text
    sample_text = " ".join([f"word{i}" for i in range(words_per_doc)])
    
    for i in range(num_docs):
        doc_path = temp_dir / f"doc_{i}.txt"
        with open(doc_path, 'w') as f:
            f.write(sample_text)
        doc_paths.append(str(doc_path))
    
    return doc_paths

def test_worker_count(num_docs: int, num_workers: int, words_per_doc: int = 200) -> Dict[str, Any]:
    """Test GraphBit with specific worker count."""
    print(f"\n{'='*80}")
    print(f"Testing with {num_workers} workers, {num_docs} documents ({words_per_doc} words each)")
    print(f"{'='*80}")
    
    # Generate documents
    print(f"📝 Generating {num_docs} documents...")
    doc_paths = generate_test_documents(num_docs, words_per_doc)
    
    # Initialize GraphBit RAG
    from examples.parallel_rag_optimized import ParallelRAG
    rag = ParallelRAG(
        chunk_size=500,
        chunk_overlap=50,
        max_workers=num_workers,
        openai_api_key="dummy-key-for-testing"  # Not needed for load/chunk operations
    )
    
    # Test loading
    start_time = time.time()
    documents = rag.load_documents_parallel(doc_paths)
    load_time = time.time() - start_time
    print(f"✅ Loaded {len(documents)} documents in {load_time:.2f}s")
    
    # Test chunking
    start_time = time.time()
    chunks = rag.chunk_documents_parallel(documents)
    chunk_time = time.time() - start_time
    print(f"✅ Created {len(chunks)} chunks in {chunk_time:.2f}s")
    
    total_time = load_time + chunk_time
    throughput = num_docs / total_time
    
    print(f"✅ Total time: {total_time:.2f}s")
    print(f"✅ Throughput: {throughput:.1f} docs/sec")
    
    # Cleanup
    import shutil
    shutil.rmtree(Path(doc_paths[0]).parent)
    
    return {
        'num_workers': num_workers,
        'num_documents': num_docs,
        'words_per_doc': words_per_doc,
        'load_time': load_time,
        'chunk_time': chunk_time,
        'total_time': total_time,
        'throughput_docs_per_sec': throughput,
        'chunks_created': len(chunks)
    }

def main():
    """Main function to test different worker counts."""
    print("🔄 Initializing GraphBit...")
    graphbit.init()
    print("✅ GraphBit initialized\n")
    
    # Test configuration
    num_docs = 5000
    words_per_doc = 200
    worker_counts = [1, 5, 10, 20, 30, 50]
    
    results = []
    
    for num_workers in worker_counts:
        try:
            result = test_worker_count(num_docs, num_workers, words_per_doc)
            results.append(result)
        except Exception as e:
            print(f"❌ Error with {num_workers} workers: {e}")
            continue
    
    # Print summary
    print(f"\n{'='*80}")
    print("WORKER COUNT OPTIMIZATION SUMMARY")
    print(f"{'='*80}\n")
    
    print(f"{'Workers':<10} {'Total Time':<15} {'Throughput':<20} {'Speedup':<10}")
    print(f"{'-'*10} {'-'*15} {'-'*20} {'-'*10}")
    
    baseline_time = results[0]['total_time'] if results else 1
    
    for result in results:
        speedup = baseline_time / result['total_time']
        print(f"{result['num_workers']:<10} {result['total_time']:<15.2f} "
              f"{result['throughput_docs_per_sec']:<20.1f} {speedup:<10.2f}x")
    
    # Find optimal worker count
    best_result = max(results, key=lambda x: x['throughput_docs_per_sec'])
    print(f"\n🏆 Optimal worker count: {best_result['num_workers']} workers")
    print(f"   Throughput: {best_result['throughput_docs_per_sec']:.1f} docs/sec")
    print(f"   Total time: {best_result['total_time']:.2f}s")
    
    # Save results
    output_file = 'worker_optimization_results.json'
    with open(output_file, 'w') as f:
        json.dump({
            'test_config': {
                'num_documents': num_docs,
                'words_per_doc': words_per_doc,
                'worker_counts_tested': worker_counts
            },
            'results': results,
            'optimal_config': {
                'num_workers': best_result['num_workers'],
                'throughput_docs_per_sec': best_result['throughput_docs_per_sec'],
                'total_time': best_result['total_time']
            }
        }, f, indent=2)
    
    print(f"\n💾 Results saved to: {output_file}")
    print(f"\n{'='*80}")
    print("✅ Worker count optimization complete!")
    print(f"{'='*80}\n")

if __name__ == "__main__":
    main()

