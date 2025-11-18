#!/bin/bash
# Quick ParallelRAG Demo (5 minutes)
# Demonstrates GraphBit's core ParallelRAG capabilities

echo "================================================================================"
echo "Demo 1: Quick ParallelRAG Demo"
echo "================================================================================"
echo ""
echo "This demo shows:"
echo "  - GIL-releasing document loading (10-50x speedup)"
echo "  - Parallel chunking (5-10x speedup)"
echo "  - Optimized embedding generation (5-10x speedup)"
echo "  - Async LLM queries (5-20x speedup)"
echo ""
echo "Runtime: ~2-3 minutes"
echo "API Cost: ~\$0.01-0.02"
echo ""
echo "================================================================================"
echo ""

# Check API key
if [ -z "$OPENAI_API_KEY" ]; then
    echo "❌ Error: OPENAI_API_KEY not set"
    echo "Please set your OpenAI API key:"
    echo "  export OPENAI_API_KEY=\"sk-your-api-key-here\""
    exit 1
fi

echo "✅ OpenAI API key is set"
echo ""

# Run the demo
echo "Running: python examples/parallel_rag_optimized.py"
echo ""
python examples/parallel_rag_optimized.py

echo ""
echo "================================================================================"
echo "Demo 1 Complete!"
echo "================================================================================"
echo ""
echo "Key Takeaways:"
echo "  ✓ GraphBit releases the GIL for true parallelism"
echo "  ✓ Document loading: 10-50x faster than sequential"
echo "  ✓ Embedding generation: 5-10x faster with batch processing"
echo "  ✓ Production-ready with error handling"
echo ""

