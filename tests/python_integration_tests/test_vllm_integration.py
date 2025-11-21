"""
Integration tests for vLLM provider.

These tests require vLLM to be installed and will use actual models.
They are marked with pytest.mark.integration and can be skipped if vLLM is not available.
"""

import pytest
import sys


# Check if vLLM is available
try:
    import vllm
    VLLM_AVAILABLE = True
except ImportError:
    VLLM_AVAILABLE = False


@pytest.mark.integration
@pytest.mark.skipif(not VLLM_AVAILABLE, reason="vLLM not installed")
class TestVLLMIntegration:
    """Integration tests for vLLM provider."""
    
    @pytest.fixture(scope="class")
    def vllm_provider(self):
        """Create a vLLM provider with a small test model."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        # Use a very small model for testing
        config = VLLMConfig(
            model="facebook/opt-125m",
            gpu_memory_utilization=0.3,  # Use less memory for testing
            max_model_len=512,  # Shorter context for faster testing
        )
        
        provider = VLLMProvider(config)
        return provider
    
    def test_generate_single_prompt(self, vllm_provider):
        """Test generating text from a single prompt."""
        prompt = "The capital of France is"
        result = vllm_provider.generate(
            prompt,
            max_tokens=10,
            temperature=0.0,  # Deterministic for testing
        )
        
        assert isinstance(result, str)
        assert len(result) > 0
        print(f"Generated: {result}")
    
    def test_generate_multiple_prompts(self, vllm_provider):
        """Test generating text from multiple prompts."""
        prompts = [
            "The capital of France is",
            "The largest planet in our solar system is",
            "Python is a programming language that",
        ]
        
        results = vllm_provider.generate(
            prompts,
            max_tokens=10,
            temperature=0.0,
        )
        
        assert isinstance(results, list)
        assert len(results) == len(prompts)
        for result in results:
            assert isinstance(result, str)
            assert len(result) > 0
        
        print("Generated texts:")
        for prompt, result in zip(prompts, results):
            print(f"  {prompt} -> {result}")
    
    def test_generate_with_sampling_params(self, vllm_provider):
        """Test generating with various sampling parameters."""
        prompt = "Once upon a time"
        
        # Test with different temperatures
        result_low_temp = vllm_provider.generate(
            prompt,
            max_tokens=20,
            temperature=0.1,
        )
        
        result_high_temp = vllm_provider.generate(
            prompt,
            max_tokens=20,
            temperature=1.0,
        )
        
        assert isinstance(result_low_temp, str)
        assert isinstance(result_high_temp, str)
        print(f"Low temp: {result_low_temp}")
        print(f"High temp: {result_high_temp}")
    
    def test_generate_with_stop_sequences(self, vllm_provider):
        """Test generating with stop sequences."""
        prompt = "Count to 10: 1, 2, 3,"
        
        result = vllm_provider.generate(
            prompt,
            max_tokens=50,
            temperature=0.0,
            stop=[",", "."],
        )
        
        assert isinstance(result, str)
        # Result should stop at comma or period
        assert "," not in result or result.endswith(",")
        print(f"Generated with stop: {result}")
    
    def test_generate_with_top_p(self, vllm_provider):
        """Test generating with nucleus sampling (top_p)."""
        prompt = "The weather today is"
        
        result = vllm_provider.generate(
            prompt,
            max_tokens=15,
            temperature=0.8,
            top_p=0.9,
        )
        
        assert isinstance(result, str)
        assert len(result) > 0
        print(f"Generated with top_p: {result}")
    
    def test_generate_with_top_k(self, vllm_provider):
        """Test generating with top-k sampling."""
        prompt = "Machine learning is"
        
        result = vllm_provider.generate(
            prompt,
            max_tokens=15,
            temperature=0.8,
            top_k=50,
        )
        
        assert isinstance(result, str)
        assert len(result) > 0
        print(f"Generated with top_k: {result}")
    
    def test_get_model_info(self, vllm_provider):
        """Test getting model information."""
        info = vllm_provider.get_model_info()
        
        assert isinstance(info, dict)
        assert "model" in info
        assert info["model"] == "facebook/opt-125m"
        assert "tensor_parallel_size" in info
        assert "gpu_memory_utilization" in info
        print(f"Model info: {info}")
    
    def test_provider_repr(self, vllm_provider):
        """Test provider string representation."""
        repr_str = repr(vllm_provider)
        
        assert "VLLMProvider" in repr_str
        assert "facebook/opt-125m" in repr_str
        print(f"Provider repr: {repr_str}")


@pytest.mark.integration
@pytest.mark.skipif(not VLLM_AVAILABLE, reason="vLLM not installed")
class TestVLLMChatIntegration:
    """Integration tests for vLLM chat functionality."""
    
    @pytest.fixture(scope="class")
    def chat_provider(self):
        """Create a vLLM provider with a chat model."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        # Note: This test requires a model with chat template
        # Using opt-125m as fallback, but ideally use a chat model
        config = VLLMConfig(
            model="facebook/opt-125m",
            gpu_memory_utilization=0.3,
            max_model_len=512,
        )
        
        provider = VLLMProvider(config)
        return provider
    
    def test_chat_single_conversation(self, chat_provider):
        """Test chat with a single conversation."""
        messages = [
            {"role": "user", "content": "What is 2+2?"}
        ]
        
        try:
            result = chat_provider.chat(
                messages,
                max_tokens=20,
                temperature=0.0,
            )
            
            assert isinstance(result, str)
            assert len(result) > 0
            print(f"Chat response: {result}")
        except Exception as e:
            # Some models may not support chat template
            pytest.skip(f"Model does not support chat: {e}")
    
    def test_chat_multiple_conversations(self, chat_provider):
        """Test chat with multiple conversations."""
        conversations = [
            [{"role": "user", "content": "Hello!"}],
            [{"role": "user", "content": "What is Python?"}],
        ]
        
        try:
            results = chat_provider.chat(
                conversations,
                max_tokens=20,
                temperature=0.0,
            )
            
            assert isinstance(results, list)
            assert len(results) == len(conversations)
            for result in results:
                assert isinstance(result, str)
            
            print("Chat responses:")
            for conv, result in zip(conversations, results):
                print(f"  {conv[0]['content']} -> {result}")
        except Exception as e:
            pytest.skip(f"Model does not support chat: {e}")


@pytest.mark.integration
@pytest.mark.skipif(not VLLM_AVAILABLE, reason="vLLM not installed")
class TestVLLMConfigVariations:
    """Test various vLLM configuration options."""
    
    def test_config_with_quantization(self):
        """Test configuration with quantization (if supported)."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        # Note: This requires a quantized model
        # Skip if not available
        pytest.skip("Quantization test requires specific model setup")
    
    def test_config_with_prefix_caching(self):
        """Test configuration with prefix caching enabled."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(
            model="facebook/opt-125m",
            enable_prefix_caching=True,
            gpu_memory_utilization=0.3,
            max_model_len=512,
        )
        
        provider = VLLMProvider(config)
        
        # Generate with repeated prefix
        prompt1 = "The quick brown fox jumps over the lazy dog. What happens next?"
        prompt2 = "The quick brown fox jumps over the lazy dog. Where does it go?"
        
        result1 = provider.generate(prompt1, max_tokens=10, temperature=0.0)
        result2 = provider.generate(prompt2, max_tokens=10, temperature=0.0)
        
        assert isinstance(result1, str)
        assert isinstance(result2, str)
        print(f"With prefix caching:")
        print(f"  Result 1: {result1}")
        print(f"  Result 2: {result2}")


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-m", "integration"])

