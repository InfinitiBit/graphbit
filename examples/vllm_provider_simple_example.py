"""
Example: Simple vLLM Provider Test for GraphBit

This example demonstrates basic usage of the vLLM provider without requiring
a CUDA-capable GPU. It's designed to work on Mac and other systems.

Prerequisites:
    poetry install  # vLLM is now a core dependency

Note: This example uses a very small model (facebook/opt-125m) that can run on CPU.
For production use with larger models, a CUDA-capable GPU is recommended.
"""

import sys
sys.path.insert(0, 'python/python-src')

from graphbit.vllm import VLLMConfig, VLLMProvider


def example_basic_generation():
    """Example 1: Basic text generation with vLLM on CPU."""
    print("=" * 80)
    print("Example 1: Basic Text Generation (CPU)")
    print("=" * 80)
    
    # Configure vLLM with a very small model for CPU testing
    # Note: This will be slow on CPU but functional
    config = VLLMConfig(
        model="facebook/opt-125m",
        max_model_len=256,  # Keep it small for CPU
    )
    
    # Create provider
    print("Initializing vLLM provider (this may take a moment)...")
    provider = VLLMProvider(config)
    
    # Generate text from a single prompt
    prompt = "The capital of France is"
    print(f"\nPrompt: {prompt}")
    print("Generating...")
    
    result = provider.generate(
        prompt,
        max_tokens=10,  # Keep it short for CPU
        temperature=0.7,
    )
    
    print(f"Generated: {result}")
    print()


def example_batch_generation():
    """Example 2: Batch generation with multiple prompts."""
    print("=" * 80)
    print("Example 2: Batch Generation (CPU)")
    print("=" * 80)
    
    config = VLLMConfig(
        model="facebook/opt-125m",
        max_model_len=256,
    )
    
    print("Initializing vLLM provider...")
    provider = VLLMProvider(config)
    
    # Generate text from multiple prompts in a batch
    prompts = [
        "Hello, my name is",
        "The weather today is",
    ]
    
    print("\nGenerating batch...")
    results = provider.generate(
        prompts,
        max_tokens=10,
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
    print("Example 3: Sampling Parameters (CPU)")
    print("=" * 80)
    
    config = VLLMConfig(model="facebook/opt-125m", max_model_len=256)
    print("Initializing vLLM provider...")
    provider = VLLMProvider(config)
    
    prompt = "Once upon a time"
    
    # Low temperature (more deterministic)
    print(f"\nPrompt: {prompt}")
    print("Generating with low temperature (0.1)...")
    result_low_temp = provider.generate(
        prompt,
        max_tokens=15,
        temperature=0.1,
    )
    print(f"Low temperature (0.1): {result_low_temp}")
    
    # High temperature (more creative)
    print("\nGenerating with high temperature (1.5)...")
    result_high_temp = provider.generate(
        prompt,
        max_tokens=15,
        temperature=1.5,
    )
    print(f"High temperature (1.5): {result_high_temp}")
    
    # With top-p (nucleus sampling)
    print("\nGenerating with top_p (0.9)...")
    result_top_p = provider.generate(
        prompt,
        max_tokens=15,
        temperature=0.8,
        top_p=0.9,
    )
    print(f"With top_p (0.9): {result_top_p}")
    print()


def example_stop_sequences():
    """Example 4: Using stop sequences."""
    print("=" * 80)
    print("Example 4: Stop Sequences (CPU)")
    print("=" * 80)
    
    config = VLLMConfig(model="facebook/opt-125m", max_model_len=256)
    print("Initializing vLLM provider...")
    provider = VLLMProvider(config)
    
    prompt = "Count: 1, 2, 3,"
    
    # Generate with stop sequences
    print(f"\nPrompt: {prompt}")
    print("Generating with stop sequence...")
    result = provider.generate(
        prompt,
        max_tokens=20,
        temperature=0.7,
        stop=[",", "\n"],  # Stop at comma or newline
    )
    
    print(f"Generated (with stop): {result}")
    print()


def example_model_info():
    """Example 5: Getting model information."""
    print("=" * 80)
    print("Example 5: Model Information")
    print("=" * 80)
    
    config = VLLMConfig(
        model="facebook/opt-125m",
        max_model_len=256,
    )
    
    print("Initializing vLLM provider...")
    provider = VLLMProvider(config)
    
    # Get model information
    info = provider.get_model_info()
    print("\nModel Information:")
    for key, value in info.items():
        print(f"  {key}: {value}")
    print()


def main():
    """Run all examples."""
    print("\n")
    print("=" * 80)
    print("vLLM Provider Simple Examples for GraphBit")
    print("=" * 80)
    print("\nNote: These examples use CPU and will be slower than GPU.")
    print("For production use, a CUDA-capable GPU is recommended.")
    print("\n")
    
    try:
        # Run examples one at a time
        example_basic_generation()
        
        # Uncomment to run more examples:
        # example_batch_generation()
        # example_sampling_parameters()
        # example_stop_sequences()
        # example_model_info()
        
        print("=" * 80)
        print("Examples completed!")
        print("=" * 80)
        
    except ImportError as e:
        print(f"Error: {e}")
        print("\nPlease install vLLM:")
        print("  poetry install")
    except Exception as e:
        print(f"Error running examples: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    main()

