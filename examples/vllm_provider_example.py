"""
Example: Using vLLM Provider with GraphBit

This example demonstrates how to use the vLLM provider for high-throughput
LLM inference with GraphBit. vLLM is a pure Python provider that is independent
from the Rust core.

Prerequisites:
    pip install graphbit[vllm]
    # or
    pip install vllm

Note: vLLM requires a CUDA-capable GPU for optimal performance.
"""

import sys
sys.path.insert(0, 'python/python-src')

from graphbit.vllm import VLLMConfig, VLLMProvider


def example_basic_generation():
    """Example 1: Basic text generation with vLLM."""
    print("=" * 80)
    print("Example 1: Basic Text Generation")
    print("=" * 80)
    
    # Configure vLLM with a small model for testing
    config = VLLMConfig(
        model="facebook/opt-125m",
        gpu_memory_utilization=0.5,  # Use 50% of GPU memory
        max_model_len=512,  # Maximum sequence length
    )
    
    # Create provider
    provider = VLLMProvider(config)
    
    # Generate text from a single prompt
    prompt = "The capital of France is"
    result = provider.generate(
        prompt,
        max_tokens=20,
        temperature=0.7,
    )
    
    print(f"Prompt: {prompt}")
    print(f"Generated: {result}")
    print()


def example_batch_generation():
    """Example 2: Batch generation with multiple prompts."""
    print("=" * 80)
    print("Example 2: Batch Generation")
    print("=" * 80)
    
    config = VLLMConfig(
        model="facebook/opt-125m",
        gpu_memory_utilization=0.5,
    )
    
    provider = VLLMProvider(config)
    
    # Generate text from multiple prompts in a batch
    prompts = [
        "The capital of France is",
        "The largest planet in our solar system is",
        "Python is a programming language that",
        "Machine learning is",
    ]
    
    results = provider.generate(
        prompts,
        max_tokens=20,
        temperature=0.7,
    )
    
    print("Batch generation results:")
    for prompt, result in zip(prompts, results):
        print(f"  Prompt: {prompt}")
        print(f"  Result: {result}")
        print()


def example_sampling_parameters():
    """Example 3: Using different sampling parameters."""
    print("=" * 80)
    print("Example 3: Sampling Parameters")
    print("=" * 80)
    
    config = VLLMConfig(model="facebook/opt-125m")
    provider = VLLMProvider(config)
    
    prompt = "Once upon a time"
    
    # Low temperature (more deterministic)
    result_low_temp = provider.generate(
        prompt,
        max_tokens=30,
        temperature=0.1,
    )
    print(f"Low temperature (0.1): {result_low_temp}")
    
    # High temperature (more creative)
    result_high_temp = provider.generate(
        prompt,
        max_tokens=30,
        temperature=1.5,
    )
    print(f"High temperature (1.5): {result_high_temp}")
    
    # With top-p (nucleus sampling)
    result_top_p = provider.generate(
        prompt,
        max_tokens=30,
        temperature=0.8,
        top_p=0.9,
    )
    print(f"With top_p (0.9): {result_top_p}")
    
    # With top-k sampling
    result_top_k = provider.generate(
        prompt,
        max_tokens=30,
        temperature=0.8,
        top_k=50,
    )
    print(f"With top_k (50): {result_top_k}")
    print()


def example_stop_sequences():
    """Example 4: Using stop sequences."""
    print("=" * 80)
    print("Example 4: Stop Sequences")
    print("=" * 80)
    
    config = VLLMConfig(model="facebook/opt-125m")
    provider = VLLMProvider(config)
    
    prompt = "List three colors: 1."
    
    # Generate with stop sequences
    result = provider.generate(
        prompt,
        max_tokens=50,
        temperature=0.7,
        stop=["3.", "\n\n"],  # Stop at "3." or double newline
    )
    
    print(f"Prompt: {prompt}")
    print(f"Generated (with stop): {result}")
    print()


def example_chat_interface():
    """Example 5: Using chat interface (requires chat model)."""
    print("=" * 80)
    print("Example 5: Chat Interface")
    print("=" * 80)
    
    # Note: This example requires a model with chat template
    # For demonstration, we'll use opt-125m, but ideally use a chat model
    # like "meta-llama/Llama-2-7b-chat-hf"
    
    config = VLLMConfig(
        model="facebook/opt-125m",
        gpu_memory_utilization=0.5,
    )
    
    provider = VLLMProvider(config)
    
    # Single conversation
    messages = [
        {"role": "user", "content": "What is 2+2?"}
    ]
    
    try:
        result = provider.chat(
            messages,
            max_tokens=50,
            temperature=0.7,
        )
        print(f"User: {messages[0]['content']}")
        print(f"Assistant: {result}")
    except Exception as e:
        print(f"Note: Chat interface requires a model with chat template")
        print(f"Error: {e}")
    print()


def example_advanced_config():
    """Example 6: Advanced configuration options."""
    print("=" * 80)
    print("Example 6: Advanced Configuration")
    print("=" * 80)
    
    # Configure with advanced options
    config = VLLMConfig(
        model="facebook/opt-125m",
        tensor_parallel_size=1,  # Number of GPUs for tensor parallelism
        dtype="float16",  # Use FP16 for faster inference
        gpu_memory_utilization=0.7,
        max_model_len=1024,
        enable_prefix_caching=True,  # Enable automatic prefix caching
        max_num_seqs=64,  # Maximum sequences per iteration
        seed=42,  # Random seed for reproducibility
    )
    
    provider = VLLMProvider(config)
    
    # Get model information
    info = provider.get_model_info()
    print("Model Information:")
    for key, value in info.items():
        print(f"  {key}: {value}")
    
    # Generate with the configured provider
    result = provider.generate(
        "The future of AI is",
        max_tokens=30,
        temperature=0.7,
    )
    print(f"\nGenerated: {result}")
    print()


def example_quantization():
    """Example 7: Using quantized models (if available)."""
    print("=" * 80)
    print("Example 7: Quantization")
    print("=" * 80)
    
    # Note: This requires a quantized model (e.g., AWQ, GPTQ)
    # For demonstration purposes only - adjust model path as needed
    
    print("To use quantization, configure with a quantized model:")
    print("""
    config = VLLMConfig(
        model="TheBloke/Llama-2-7B-AWQ",
        quantization="awq",
        gpu_memory_utilization=0.5,
    )
    """)
    print("This allows running larger models with less GPU memory.")
    print()


def main():
    """Run all examples."""
    print("\n")
    print("=" * 80)
    print("vLLM Provider Examples for GraphBit")
    print("=" * 80)
    print("\n")
    
    try:
        # Run examples
        example_basic_generation()
        example_batch_generation()
        example_sampling_parameters()
        example_stop_sequences()
        example_chat_interface()
        example_advanced_config()
        example_quantization()
        
        print("=" * 80)
        print("All examples completed!")
        print("=" * 80)
        
    except ImportError as e:
        print(f"Error: {e}")
        print("\nPlease install vLLM:")
        print("  pip install vllm")
        print("\nOr install graphbit with vLLM support:")
        print("  pip install graphbit[vllm]")
    except Exception as e:
        print(f"Error running examples: {e}")
        print("\nNote: These examples require a CUDA-capable GPU.")
        print("If you don't have a GPU, vLLM may not work properly.")


if __name__ == "__main__":
    main()

