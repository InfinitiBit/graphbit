"""
Unit tests for vLLM provider.

These tests verify the configuration and basic functionality of the vLLM provider
without requiring actual vLLM installation or GPU resources.
"""

import pytest
import sys
from unittest.mock import Mock, MagicMock, patch


class TestVLLMConfig:
    """Test VLLMConfig class."""
    
    def test_config_initialization_minimal(self):
        """Test VLLMConfig initialization with minimal parameters."""
        # Import here to avoid import errors if graphbit not installed
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig
        
        config = VLLMConfig(model="facebook/opt-125m")
        
        assert config.model == "facebook/opt-125m"
        assert config.tensor_parallel_size == 1
        assert config.dtype == "auto"
        assert config.gpu_memory_utilization == 0.9
        assert config.max_model_len is None
        assert config.trust_remote_code is False
        assert config.seed == 0
    
    def test_config_initialization_full(self):
        """Test VLLMConfig initialization with all parameters."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig
        
        config = VLLMConfig(
            model="meta-llama/Llama-2-7b-hf",
            tensor_parallel_size=2,
            dtype="float16",
            gpu_memory_utilization=0.8,
            max_model_len=4096,
            trust_remote_code=True,
            download_dir="/tmp/models",
            seed=42,
            revision="main",
            tokenizer_revision="main",
            quantization="awq",
            enforce_eager=True,
            max_num_seqs=128,
            max_num_batched_tokens=2048,
            enable_prefix_caching=True,
            disable_custom_all_reduce=True,
        )
        
        assert config.model == "meta-llama/Llama-2-7b-hf"
        assert config.tensor_parallel_size == 2
        assert config.dtype == "float16"
        assert config.gpu_memory_utilization == 0.8
        assert config.max_model_len == 4096
        assert config.trust_remote_code is True
        assert config.download_dir == "/tmp/models"
        assert config.seed == 42
        assert config.revision == "main"
        assert config.tokenizer_revision == "main"
        assert config.quantization == "awq"
        assert config.enforce_eager is True
        assert config.max_num_seqs == 128
        assert config.max_num_batched_tokens == 2048
        assert config.enable_prefix_caching is True
        assert config.disable_custom_all_reduce is True
    
    def test_config_to_dict_minimal(self):
        """Test VLLMConfig.to_dict() with minimal parameters."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig
        
        config = VLLMConfig(model="facebook/opt-125m")
        config_dict = config.to_dict()
        
        assert config_dict["model"] == "facebook/opt-125m"
        assert config_dict["tensor_parallel_size"] == 1
        assert config_dict["dtype"] == "auto"
        assert config_dict["gpu_memory_utilization"] == 0.9
        assert "max_model_len" not in config_dict  # Should not include None values
        assert "download_dir" not in config_dict
    
    def test_config_to_dict_full(self):
        """Test VLLMConfig.to_dict() with all parameters."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig
        
        config = VLLMConfig(
            model="meta-llama/Llama-2-7b-hf",
            max_model_len=4096,
            quantization="awq",
        )
        config_dict = config.to_dict()
        
        assert config_dict["model"] == "meta-llama/Llama-2-7b-hf"
        assert config_dict["max_model_len"] == 4096
        assert config_dict["quantization"] == "awq"


class TestVLLMProvider:
    """Test VLLMProvider class."""
    
    def test_provider_initialization(self):
        """Test VLLMProvider initialization."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(model="facebook/opt-125m")
        
        # Mock vllm import to avoid requiring actual installation
        with patch.dict('sys.modules', {'vllm': MagicMock()}):
            provider = VLLMProvider(config)
            
            assert provider.config == config
            assert provider._llm is None  # Lazy initialization
    
    def test_provider_missing_vllm(self):
        """Test VLLMProvider raises error when vLLM not installed."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(model="facebook/opt-125m")
        
        # Simulate vllm not being installed
        with patch.dict('sys.modules', {'vllm': None}):
            with pytest.raises(ImportError, match="vLLM is not installed"):
                VLLMProvider(config)
    
    def test_provider_lazy_initialization(self):
        """Test that LLM engine is initialized lazily."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(model="facebook/opt-125m")
        
        # Mock vllm module
        mock_vllm = MagicMock()
        mock_llm_class = MagicMock()
        mock_vllm.LLM = mock_llm_class
        
        with patch.dict('sys.modules', {'vllm': mock_vllm}):
            provider = VLLMProvider(config)
            
            # LLM should not be initialized yet
            assert provider._llm is None
            mock_llm_class.assert_not_called()
            
            # Access llm property to trigger initialization
            _ = provider.llm
            
            # Now LLM should be initialized
            mock_llm_class.assert_called_once()
    
    def test_provider_generate_single_prompt(self):
        """Test generate() with a single prompt."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(model="facebook/opt-125m")
        
        # Mock vllm module and LLM
        mock_vllm = MagicMock()
        mock_llm_instance = MagicMock()
        mock_output = MagicMock()
        mock_output.outputs = [MagicMock(text="Generated text")]
        mock_llm_instance.generate.return_value = [mock_output]
        mock_vllm.LLM.return_value = mock_llm_instance
        mock_vllm.SamplingParams = MagicMock()
        
        with patch.dict('sys.modules', {'vllm': mock_vllm}):
            provider = VLLMProvider(config)
            result = provider.generate("Hello, my name is", max_tokens=50)
            
            assert result == "Generated text"
            mock_llm_instance.generate.assert_called_once()
    
    def test_provider_generate_multiple_prompts(self):
        """Test generate() with multiple prompts."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(model="facebook/opt-125m")
        
        # Mock vllm module and LLM
        mock_vllm = MagicMock()
        mock_llm_instance = MagicMock()
        mock_output1 = MagicMock()
        mock_output1.outputs = [MagicMock(text="Generated text 1")]
        mock_output2 = MagicMock()
        mock_output2.outputs = [MagicMock(text="Generated text 2")]
        mock_llm_instance.generate.return_value = [mock_output1, mock_output2]
        mock_vllm.LLM.return_value = mock_llm_instance
        mock_vllm.SamplingParams = MagicMock()
        
        with patch.dict('sys.modules', {'vllm': mock_vllm}):
            provider = VLLMProvider(config)
            results = provider.generate(["Prompt 1", "Prompt 2"], max_tokens=50)
            
            assert results == ["Generated text 1", "Generated text 2"]
            mock_llm_instance.generate.assert_called_once()
    
    def test_provider_get_model_info(self):
        """Test get_model_info() method."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(
            model="facebook/opt-125m",
            tensor_parallel_size=2,
            max_model_len=2048
        )
        
        with patch.dict('sys.modules', {'vllm': MagicMock()}):
            provider = VLLMProvider(config)
            info = provider.get_model_info()
            
            assert info["model"] == "facebook/opt-125m"
            assert info["tensor_parallel_size"] == 2
            assert info["max_model_len"] == 2048
    
    def test_provider_repr(self):
        """Test __repr__() method."""
        sys.path.insert(0, 'python/python-src')
        from graphbit.vllm import VLLMConfig, VLLMProvider
        
        config = VLLMConfig(model="facebook/opt-125m")
        
        with patch.dict('sys.modules', {'vllm': MagicMock()}):
            provider = VLLMProvider(config)
            repr_str = repr(provider)
            
            assert "VLLMProvider" in repr_str
            assert "facebook/opt-125m" in repr_str


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

