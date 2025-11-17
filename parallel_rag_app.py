"""
Production-Ready Parallel RAG Application

This application demonstrates best practices for building a high-performance
RAG system using GraphBit with optimal configurations based on comprehensive benchmarks.

Performance Characteristics (based on benchmarks):
- Chunking: 6.20x speedup with TokenSplitter (20 workers)
- Embedding: 34.81x speedup (20 workers)
- LLM: 19.04x speedup (20 workers)
- End-to-End: 19.22x speedup for complete pipeline

Configuration:
- Text Splitter: TokenSplitter (chunk_size=200, chunk_overlap=20)
- Embedding Model: text-embedding-3-small (OpenAI)
- LLM Model: gpt-4o-mini (OpenAI)
- Worker Counts: 20 workers for all operations (optimal based on benchmarks)
"""

import os
import time
from concurrent.futures import ThreadPoolExecutor
from typing import List, Dict, Any, Optional
from dataclasses import dataclass

import graphbit


@dataclass
class RAGConfig:
    """Configuration for ParallelRAG system."""
    # Text splitting configuration (optimal from benchmarks)
    chunk_size: int = 200
    chunk_overlap: int = 20
    
    # Parallel processing configuration (optimal from benchmarks)
    chunking_workers: int = 20  # 6.20x speedup
    embedding_workers: int = 20  # 34.81x speedup
    llm_workers: int = 20  # 19.04x speedup
    
    # API configuration
    openai_api_key: Optional[str] = None
    embedding_model: str = "text-embedding-3-small"
    llm_model: str = "gpt-4o-mini"
    
    # LLM parameters
    max_tokens: int = 500
    temperature: float = 0.7


class ParallelRAG:
    """Production-ready parallel RAG system using GraphBit."""
    
    def __init__(self, config: Optional[RAGConfig] = None):
        """Initialize ParallelRAG system.
        
        Args:
            config: RAG configuration. If None, uses default configuration.
        """
        self.config = config or RAGConfig()
        
        # Get API key from config or environment
        api_key = self.config.openai_api_key or os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise ValueError(
                "OpenAI API key required. Set OPENAI_API_KEY environment variable "
                "or pass openai_api_key in RAGConfig."
            )
        
        # Initialize GraphBit
        graphbit.init()
        
        # Create text splitter (TokenSplitter - best performance from benchmarks)
        self.splitter = graphbit.TokenSplitter(
            chunk_size=self.config.chunk_size,
            chunk_overlap=self.config.chunk_overlap
        )
        
        # Create embedding client
        embed_config = graphbit.EmbeddingConfig.openai(
            api_key=api_key,
            model=self.config.embedding_model
        )
        self.embed_client = graphbit.EmbeddingClient(embed_config)
        
        # Create LLM client
        llm_config = graphbit.LlmConfig.openai(
            api_key=api_key,
            model=self.config.llm_model
        )
        self.llm_client = graphbit.LlmClient(llm_config)
        
        # Statistics
        self.stats = {
            "documents_processed": 0,
            "chunks_created": 0,
            "embeddings_generated": 0,
            "llm_calls": 0,
            "total_time": 0.0
        }
    
    def chunk_documents(self, documents: List[str]) -> List[List[str]]:
        """Split documents into chunks using parallel processing.
        
        Args:
            documents: List of document texts
            
        Returns:
            List of chunk lists (one per document)
        """
        print(f"Chunking {len(documents)} documents with {self.config.chunking_workers} workers...")
        start_time = time.time()
        
        # Parallel chunking with ThreadPoolExecutor
        # GIL is released during split_text() for true parallelism
        with ThreadPoolExecutor(max_workers=self.config.chunking_workers) as executor:
            chunk_lists = list(executor.map(self.splitter.split_text, documents))
        
        # Extract text content from TextChunk objects
        chunk_texts = [[chunk.content for chunk in chunks] for chunks in chunk_lists]
        
        duration = time.time() - start_time
        total_chunks = sum(len(chunks) for chunks in chunk_texts)
        
        print(f"✅ Created {total_chunks} chunks in {duration:.2f}s ({total_chunks/duration:.1f} chunks/sec)")
        
        self.stats["chunks_created"] += total_chunks
        
        return chunk_texts
    
    def generate_embeddings(self, texts: List[str]) -> List[List[float]]:
        """Generate embeddings for texts using parallel processing.
        
        Args:
            texts: List of text strings
            
        Returns:
            List of embedding vectors
        """
        print(f"Generating embeddings for {len(texts)} texts with {self.config.embedding_workers} workers...")
        start_time = time.time()
        
        # Parallel embedding generation with ThreadPoolExecutor
        # GIL is released during embed() for true parallelism
        with ThreadPoolExecutor(max_workers=self.config.embedding_workers) as executor:
            embeddings = list(executor.map(self.embed_client.embed, texts))
        
        duration = time.time() - start_time
        
        print(f"✅ Generated {len(embeddings)} embeddings in {duration:.2f}s ({len(embeddings)/duration:.1f} embeddings/sec)")
        
        self.stats["embeddings_generated"] += len(embeddings)

        return embeddings

    def generate_completions(self, prompts: List[str]) -> List[str]:
        """Generate LLM completions for prompts using parallel processing.

        Args:
            prompts: List of prompt strings

        Returns:
            List of completion strings
        """
        print(f"Generating completions for {len(prompts)} prompts with {self.config.llm_workers} workers...")
        start_time = time.time()

        # Parallel LLM completion with ThreadPoolExecutor
        # GIL is released during complete() for true parallelism
        with ThreadPoolExecutor(max_workers=self.config.llm_workers) as executor:
            completions = list(executor.map(
                lambda p: self.llm_client.complete(
                    p,
                    max_tokens=self.config.max_tokens,
                    temperature=self.config.temperature
                ),
                prompts
            ))

        duration = time.time() - start_time

        print(f"✅ Generated {len(completions)} completions in {duration:.2f}s ({len(completions)/duration:.1f} completions/sec)")

        self.stats["llm_calls"] += len(completions)

        return completions

    def process_documents(self, documents: List[str]) -> Dict[str, Any]:
        """Process documents through complete RAG pipeline.

        This method demonstrates the full end-to-end RAG workflow:
        1. Chunk documents in parallel
        2. Generate embeddings for chunks in parallel
        3. (Optional) Generate summaries/completions in parallel

        Args:
            documents: List of document texts

        Returns:
            Dictionary containing processed results and statistics
        """
        print(f"\n{'='*80}")
        print(f"Processing {len(documents)} documents through ParallelRAG pipeline")
        print(f"{'='*80}\n")

        overall_start = time.time()

        # Step 1: Chunk documents
        chunk_lists = self.chunk_documents(documents)

        # Flatten chunks for embedding
        all_chunks = [chunk for chunks in chunk_lists for chunk in chunks]

        # Step 2: Generate embeddings
        embeddings = self.generate_embeddings(all_chunks)

        # Step 3: Create document summaries (optional)
        summary_prompts = [
            f"Summarize the following text in 2-3 sentences:\n\n{doc[:500]}"
            for doc in documents
        ]
        summaries = self.generate_completions(summary_prompts)

        overall_duration = time.time() - overall_start

        # Update statistics
        self.stats["documents_processed"] += len(documents)
        self.stats["total_time"] += overall_duration

        # Prepare results
        results = {
            "documents": len(documents),
            "chunks": len(all_chunks),
            "embeddings": len(embeddings),
            "summaries": len(summaries),
            "duration": overall_duration,
            "throughput": len(documents) / overall_duration,
            "chunk_data": [
                {
                    "document_index": i,
                    "chunks": chunks,
                    "chunk_count": len(chunks)
                }
                for i, chunks in enumerate(chunk_lists)
            ],
            "embeddings": embeddings,
            "summaries": summaries
        }

        print(f"\n{'='*80}")
        print(f"Pipeline Complete!")
        print(f"  Documents:   {len(documents)}")
        print(f"  Chunks:      {len(all_chunks)}")
        print(f"  Embeddings:  {len(embeddings)}")
        print(f"  Summaries:   {len(summaries)}")
        print(f"  Duration:    {overall_duration:.2f}s")
        print(f"  Throughput:  {len(documents)/overall_duration:.2f} docs/sec")
        print(f"{'='*80}\n")

        return results

    def get_statistics(self) -> Dict[str, Any]:
        """Get processing statistics.

        Returns:
            Dictionary containing cumulative statistics
        """
        return self.stats.copy()

    def reset_statistics(self) -> None:
        """Reset processing statistics."""
        self.stats = {
            "documents_processed": 0,
            "chunks_created": 0,
            "embeddings_generated": 0,
            "llm_calls": 0,
            "total_time": 0.0
        }


def main():
    """Example usage of ParallelRAG system."""
    # Create sample documents
    documents = [
        """
        Artificial Intelligence (AI) is revolutionizing the way we interact with technology.
        Machine learning algorithms can now process vast amounts of data to identify patterns
        and make predictions with remarkable accuracy. Deep learning, a subset of machine learning,
        uses neural networks with multiple layers to learn hierarchical representations of data.
        This has led to breakthroughs in computer vision, natural language processing, and speech recognition.
        """,
        """
        Cloud computing has transformed the IT landscape by providing on-demand access to computing
        resources over the internet. Organizations can now scale their infrastructure dynamically,
        paying only for what they use. Major cloud providers like AWS, Azure, and Google Cloud offer
        a wide range of services including compute, storage, databases, and machine learning tools.
        This has enabled startups and enterprises alike to innovate faster and reduce capital expenditure.
        """,
        """
        Cybersecurity is becoming increasingly critical as our world becomes more connected.
        Threats are evolving rapidly, with sophisticated attacks targeting everything from
        individual devices to critical infrastructure. Organizations must implement multi-layered
        security strategies including firewalls, encryption, intrusion detection systems, and
        regular security audits. Employee training is also essential as human error remains
        one of the biggest security vulnerabilities.
        """
    ]

    # Create RAG system with default configuration
    rag = ParallelRAG()

    # Process documents
    results = rag.process_documents(documents)

    # Display results
    print("\nDocument Summaries:")
    print("="*80)
    for i, summary in enumerate(results["summaries"]):
        print(f"\nDocument {i+1}:")
        print(f"  {summary}")

    # Display statistics
    print("\n\nCumulative Statistics:")
    print("="*80)
    stats = rag.get_statistics()
    for key, value in stats.items():
        print(f"  {key}: {value}")


if __name__ == "__main__":
    main()


