"""
Comprehensive HuggingFace Model Testing Script.

This script tests multiple HuggingFace models for both embeddings and LLM capabilities.
It systematically tries different models and reports which ones work and which ones fail.
"""

import os
import time
from typing import Dict, List, Optional

import graphbit

# Initialize GraphBit
graphbit.init()

# Get API key
api_key = os.getenv("HUGGINGFACE_API_KEY")
if not api_key:
    print("❌ HUGGINGFACE_API_KEY not set")
    print("💡 Set it with: export HUGGINGFACE_API_KEY='your-api-key-here'")
    exit(1)

print("✅ API key found")
print("🚀 Starting comprehensive HuggingFace model testing...\n")

# Test data
TEST_SENTENCE = "GraphBit is a framework for LLM workflows and agent orchestration."
TEST_PROMPT = "Explain what GraphBit is in one sentence."

# Embedding models to test
EMBEDDING_MODELS = [
    # Sentence Transformers (most popular)
    "sentence-transformers/all-MiniLM-L6-v2",
    "sentence-transformers/all-mpnet-base-v2",
    "sentence-transformers/paraphrase-MiniLM-L3-v2",
    "sentence-transformers/msmarco-MiniLM-L-6-v3",
    "sentence-transformers/all-MiniLM-L12-v2",
    "sentence-transformers/multi-qa-MiniLM-L6-cos-v1",
    "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2",
    # Multilingual models
    "intfloat/multilingual-e5-large",
    "intfloat/multilingual-e5-base",
    "intfloat/multilingual-e5-small",
    # BGE models
    "BAAI/bge-large-en-v1.5",
    "BAAI/bge-base-en-v1.5",
    "BAAI/bge-small-en-v1.5",
    "BAAI/bge-large-zh-v1.5",
    # Other popular models
    "sentence-transformers/LaBSE",
    "sentence-transformers/distiluse-base-multilingual-cased-v2",
    "sentence-transformers/paraphrase-multilingual-mpnet-base-v2",
    # Smaller models
    "sentence-transformers/all-MiniLM-L3-v2",
    "sentence-transformers/paraphrase-MiniLM-L6-v2",
    "sentence-transformers/paraphrase-multilingual-MiniLM-L6-v2",
]

# LLM models to test
LLM_MODELS = [
    # GPT models
    "gpt2",
    "gpt2-medium",
    "gpt2-large",
    "distilgpt2",
    # DialoGPT models
    "microsoft/DialoGPT-small",
    "microsoft/DialoGPT-medium",
    "microsoft/DialoGPT-large",
    # GPT-Neo models
    "EleutherAI/gpt-neo-125M",
    "EleutherAI/gpt-neo-1.3B",
    "EleutherAI/gpt-neo-2.7B",
    # OPT models
    "facebook/opt-125m",
    "facebook/opt-350m",
    "facebook/opt-1.3b",
    "facebook/opt-2.7b",
    # BLOOM models
    "bigscience/bloom-560m",
    "bigscience/bloom-1b1",
    "bigscience/bloom-3b",
    # Other models
    "microsoft/DialoGPT-medium",
    "microsoft/DialoGPT-base",
    "microsoft/DialoGPT-small",
    # Smaller models
    "distilgpt2",
    "sshleifer/tiny-gpt2",
    "sshleifer/tiny-gpt2-medium",
]


class ModelTester:
    """Class for testing HuggingFace models."""

    def __init__(self, api_key: str):
        """Initialize the ModelTester with an API key."""
        self.api_key = api_key
        self.embedding_results: List[Dict] = []
        self.llm_results: List[Dict] = []

    def test_embedding_model(self, model_name: str) -> Dict:
        """Test a single embedding model."""
        print(f"🔍 Testing embedding model: {model_name}")

        result: Dict[str, Optional[str]] = {
            "model": model_name,
            "status": "unknown",
            "error": None,
            "embedding_dim": None,
            "time_taken": None,
        }

        start_time = time.time()

        try:
            # Create embedding client
            embedding_client = graphbit.EmbeddingClient(
                graphbit.EmbeddingConfig.huggingface(
                    model=model_name,
                    api_key=self.api_key,
                )
            )

            # Test single embedding
            embedding = embedding_client.embed(TEST_SENTENCE)

            # Test batch embedding
            embeddings = embedding_client.embed_many([TEST_SENTENCE, "Second test sentence"])

            # Test similarity
            similarity = embedding_client.similarity(embedding, embedding)

            result.update(
                {
                    "status": "success",
                    "embedding_dim": str(len(embedding)),
                    "time_taken": str(time.time() - start_time),
                    "batch_works": str(len(embeddings) == 2),
                    "similarity_works": str(isinstance(similarity, (int, float))),
                }
            )

            print(f"  ✅ SUCCESS - {len(embedding)} dimensions, {result['time_taken']:.2f}s")

        except Exception as e:
            result.update({"status": "failed", "error": str(e), "time_taken": str(time.time() - start_time)})
            print(f"  ❌ FAILED - {str(e)}")

        return result

    def test_llm_model(self, model_name: str) -> Dict:
        """Test a single LLM model."""
        print(f"🤖 Testing LLM model: {model_name}")

        result: Dict[str, Optional[str]] = {
            "model": model_name,
            "status": "unknown",
            "error": None,
            "response": None,
            "time_taken": None,
        }

        start_time = time.time()

        try:
            # Create LLM client
            llm_config = graphbit.LlmConfig.huggingface(
                model=model_name,
                api_key=self.api_key,
            )
            llm_client = graphbit.LlmClient(llm_config)

            # Test synchronous completion
            response = llm_client.complete(TEST_PROMPT, max_tokens=50)

            result.update({"status": "success", "response": str(response), "time_taken": str(time.time() - start_time)})

            print(f"  ✅ SUCCESS - Response: {response[:100]}...")

        except Exception as e:
            result.update({"status": "failed", "error": str(e), "time_taken": str(time.time() - start_time)})
            print(f"  ❌ FAILED - {str(e)}")

        return result

    def test_all_embedding_models(self):
        """Test all embedding models."""
        print("=" * 80)
        print("🧪 TESTING EMBEDDING MODELS")
        print("=" * 80)

        for model in EMBEDDING_MODELS:
            result = self.test_embedding_model(model)
            self.embedding_results.append(result)
            time.sleep(1)  # Rate limiting

    def test_all_llm_models(self):
        """Test all LLM models."""
        print("\n" + "=" * 80)
        print("🧪 TESTING LLM MODELS")
        print("=" * 80)

        for model in LLM_MODELS:
            result = self.test_llm_model(model)
            self.llm_results.append(result)
            time.sleep(1)  # Rate limiting

    def generate_report(self):
        """Generate a comprehensive test report."""
        print("\n" + "=" * 80)
        print("📊 COMPREHENSIVE TEST REPORT")
        print("=" * 80)

        # Embedding Results
        print("\n🔍 EMBEDDING MODELS RESULTS:")
        print("-" * 50)

        successful_embeddings = [r for r in self.embedding_results if r["status"] == "success"]
        failed_embeddings = [r for r in self.embedding_results if r["status"] == "failed"]

        print(f"✅ SUCCESSFUL: {len(successful_embeddings)}/{len(self.embedding_results)}")
        print(f"❌ FAILED: {len(failed_embeddings)}/{len(self.embedding_results)}")

        if successful_embeddings:
            print("\n✅ WORKING EMBEDDING MODELS:")
            for result in successful_embeddings:
                print(f"  • {result['model']} ({result['embedding_dim']} dimensions, {result['time_taken']:.2f}s)")

        if failed_embeddings:
            print("\n❌ FAILED EMBEDDING MODELS:")
            for result in failed_embeddings:
                print(f"  • {result['model']}: {result['error']}")

        # LLM Results
        print("\n🤖 LLM MODELS RESULTS:")
        print("-" * 50)

        successful_llms = [r for r in self.llm_results if r["status"] == "success"]
        failed_llms = [r for r in self.llm_results if r["status"] == "failed"]

        print(f"✅ SUCCESSFUL: {len(successful_llms)}/{len(self.llm_results)}")
        print(f"❌ FAILED: {len(failed_llms)}/{len(self.llm_results)}")

        if successful_llms:
            print("\n✅ WORKING LLM MODELS:")
            for result in successful_llms:
                response_preview = result["response"][:50] + "..." if result["response"] else "No response"
                print(f"  • {result['model']} ({result['time_taken']:.2f}s): {response_preview}")

        if failed_llms:
            print("\n❌ FAILED LLM MODELS:")
            for result in failed_llms:
                print(f"  • {result['model']}: {result['error']}")

        # Summary Statistics
        print("\n📈 SUMMARY STATISTICS:")
        print("-" * 50)

        total_models = len(self.embedding_results) + len(self.llm_results)
        total_successful = len(successful_embeddings) + len(successful_llms)
        total_failed = len(failed_embeddings) + len(failed_llms)

        print(f"Total Models Tested: {total_models}")
        print(f"Successful: {total_successful} ({total_successful/total_models*100:.1f}%)")
        print(f"Failed: {total_failed} ({total_failed/total_models*100:.1f}%)")

        # Recommendations
        print("\n💡 RECOMMENDATIONS:")
        print("-" * 50)

        if successful_embeddings:
            best_embedding = min(successful_embeddings, key=lambda x: x["time_taken"])
            print(f"• Best Embedding Model: {best_embedding['model']} ({best_embedding['time_taken']:.2f}s)")

        if successful_llms:
            best_llm = min(successful_llms, key=lambda x: x["time_taken"])
            print(f"• Best LLM Model: {best_llm['model']} ({best_llm['time_taken']:.2f}s)")

        if not successful_llms:
            print("• No LLM models worked - consider using OpenAI or Ollama for LLM tasks")

        print("• Use hybrid approach: HuggingFace for embeddings + OpenAI/Ollama for LLM")


def main():
    """Run the main testing function."""
    tester = ModelTester(api_key)

    # Test all models
    tester.test_all_embedding_models()
    tester.test_all_llm_models()

    # Generate report
    tester.generate_report()

    print("\n🎉 Testing complete! Check the report above for results.")


if __name__ == "__main__":
    main()
