"""
vLLM Provider for GraphBit - Pure Python Implementation.

This module provides a pure Python wrapper around vLLM for use with GraphBit.
It is completely independent from the Rust core and can be used as a standalone
provider for high-throughput LLM inference.

vLLM is a fast and easy-to-use library for LLM inference and serving with:
- State-of-the-art serving throughput
- Efficient memory management with PagedAttention
- Continuous batching of incoming requests
- Optimized CUDA kernels

Example:
    >>> from graphbit_providers import VLLMConfig, VLLMProvider
    >>> config = VLLMConfig(model="facebook/opt-125m")
    >>> provider = VLLMProvider(config)
    >>> response = provider.generate("Hello, my name is")
    >>> print(response)
"""

from typing import Any, Dict, List, Optional, Union
import warnings


class VLLMConfig:
    """
    Configuration for vLLM provider.
    
    This class encapsulates all configuration options for the vLLM provider,
    including model selection, sampling parameters, and engine configuration.
    
    Args:
        model: The model name or path (e.g., "facebook/opt-125m", "meta-llama/Llama-2-7b-hf")
        tensor_parallel_size: Number of GPUs to use for tensor parallelism (default: 1)
        dtype: Data type for model weights (default: "auto")
        gpu_memory_utilization: Fraction of GPU memory to use (default: 0.9)
        max_model_len: Maximum sequence length (default: None, uses model's max)
        trust_remote_code: Whether to trust remote code from HuggingFace (default: False)
        download_dir: Directory to download and cache models (default: None)
        seed: Random seed for sampling (default: 0)
        revision: Model revision to use (default: None)
        tokenizer_revision: Tokenizer revision to use (default: None)
        quantization: Quantization method (e.g., "awq", "gptq", None) (default: None)
        enforce_eager: Whether to enforce eager execution (default: False)
        max_num_seqs: Maximum number of sequences per iteration (default: 256)
        max_num_batched_tokens: Maximum number of batched tokens (default: None)
        enable_prefix_caching: Enable automatic prefix caching (default: False)
        disable_custom_all_reduce: Disable custom all-reduce kernel (default: False)
        
    Example:
        >>> config = VLLMConfig(
        ...     model="facebook/opt-125m",
        ...     tensor_parallel_size=1,
        ...     gpu_memory_utilization=0.9
        ... )
    """
    
    def __init__(
        self,
        model: str,
        tensor_parallel_size: int = 1,
        dtype: str = "auto",
        gpu_memory_utilization: float = 0.9,
        max_model_len: Optional[int] = None,
        trust_remote_code: bool = False,
        download_dir: Optional[str] = None,
        seed: int = 0,
        revision: Optional[str] = None,
        tokenizer_revision: Optional[str] = None,
        quantization: Optional[str] = None,
        enforce_eager: bool = False,
        max_num_seqs: int = 256,
        max_num_batched_tokens: Optional[int] = None,
        enable_prefix_caching: bool = False,
        disable_custom_all_reduce: bool = False,
    ):
        """Initialize vLLM configuration."""
        self.model = model
        self.tensor_parallel_size = tensor_parallel_size
        self.dtype = dtype
        self.gpu_memory_utilization = gpu_memory_utilization
        self.max_model_len = max_model_len
        self.trust_remote_code = trust_remote_code
        self.download_dir = download_dir
        self.seed = seed
        self.revision = revision
        self.tokenizer_revision = tokenizer_revision
        self.quantization = quantization
        self.enforce_eager = enforce_eager
        self.max_num_seqs = max_num_seqs
        self.max_num_batched_tokens = max_num_batched_tokens
        self.enable_prefix_caching = enable_prefix_caching
        self.disable_custom_all_reduce = disable_custom_all_reduce
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert configuration to dictionary for vLLM LLM initialization."""
        config_dict = {
            "model": self.model,
            "tensor_parallel_size": self.tensor_parallel_size,
            "dtype": self.dtype,
            "gpu_memory_utilization": self.gpu_memory_utilization,
            "trust_remote_code": self.trust_remote_code,
            "seed": self.seed,
            "enforce_eager": self.enforce_eager,
            "max_num_seqs": self.max_num_seqs,
            "enable_prefix_caching": self.enable_prefix_caching,
            "disable_custom_all_reduce": self.disable_custom_all_reduce,
        }
        
        # Add optional parameters only if they are set
        if self.max_model_len is not None:
            config_dict["max_model_len"] = self.max_model_len
        if self.download_dir is not None:
            config_dict["download_dir"] = self.download_dir
        if self.revision is not None:
            config_dict["revision"] = self.revision
        if self.tokenizer_revision is not None:
            config_dict["tokenizer_revision"] = self.tokenizer_revision
        if self.quantization is not None:
            config_dict["quantization"] = self.quantization
        if self.max_num_batched_tokens is not None:
            config_dict["max_num_batched_tokens"] = self.max_num_batched_tokens
            
        return config_dict


class VLLMProvider:
    """
    vLLM Provider for GraphBit.
    
    This class provides a high-level interface to vLLM for text generation.
    It handles model initialization, text generation, and chat completions.
    
    Args:
        config: VLLMConfig instance with model and engine configuration
        
    Attributes:
        config: The VLLMConfig instance
        llm: The vLLM LLM engine instance (initialized lazily)
        
    Example:
        >>> from graphbit_providers import VLLMConfig, VLLMProvider
        >>> config = VLLMConfig(model="facebook/opt-125m")
        >>> provider = VLLMProvider(config)
        >>> response = provider.generate("Hello, my name is", max_tokens=50)
        >>> print(response)
    """
    
    def __init__(self, config: VLLMConfig):
        """Initialize vLLM provider with configuration."""
        self.config = config
        self._llm = None
        self._check_vllm_installation()
    
    def _check_vllm_installation(self) -> None:
        """Check if vLLM is installed and provide helpful error message if not."""
        try:
            import vllm
        except ImportError:
            raise ImportError(
                "vLLM is not installed. Please install it using:\n"
                "  pip install vllm\n"
                "or for specific CUDA version:\n"
                "  pip install vllm --extra-index-url https://download.pytorch.org/whl/cu118"
            )
    
    @property
    def llm(self):
        """Lazy initialization of vLLM engine."""
        if self._llm is None:
            from vllm import LLM
            self._llm = LLM(**self.config.to_dict())
        return self._llm
    
    def generate(
        self,
        prompts: Union[str, List[str]],
        max_tokens: int = 256,
        temperature: float = 0.7,
        top_p: float = 1.0,
        top_k: int = -1,
        frequency_penalty: float = 0.0,
        presence_penalty: float = 0.0,
        stop: Optional[Union[str, List[str]]] = None,
        n: int = 1,
        best_of: Optional[int] = None,
        skip_special_tokens: bool = True,
    ) -> Union[str, List[str]]:
        """
        Generate text completions for the given prompt(s).

        Args:
            prompts: Single prompt string or list of prompts
            max_tokens: Maximum number of tokens to generate (default: 256)
            temperature: Sampling temperature (default: 0.7)
            top_p: Nucleus sampling probability (default: 1.0)
            top_k: Top-k sampling parameter (default: -1, disabled)
            frequency_penalty: Frequency penalty (default: 0.0)
            presence_penalty: Presence penalty (default: 0.0)
            stop: Stop sequences (default: None)
            n: Number of completions per prompt (default: 1)
            best_of: Number of candidates to generate (default: None)
            skip_special_tokens: Whether to skip special tokens in output (default: True)

        Returns:
            Generated text(s). If input is a single string, returns a single string.
            If input is a list, returns a list of strings.

        Example:
            >>> provider = VLLMProvider(VLLMConfig(model="facebook/opt-125m"))
            >>> text = provider.generate("Hello, my name is", max_tokens=50)
            >>> print(text)
        """
        from vllm import SamplingParams

        # Convert single prompt to list for uniform processing
        is_single_prompt = isinstance(prompts, str)
        if is_single_prompt:
            prompts = [prompts]

        # Create sampling parameters
        # Note: use_beam_search, length_penalty, and early_stopping are not supported in vLLM 0.11.0
        sampling_params = SamplingParams(
            max_tokens=max_tokens,
            temperature=temperature,
            top_p=top_p,
            top_k=top_k,
            frequency_penalty=frequency_penalty,
            presence_penalty=presence_penalty,
            stop=stop,
            n=n,
            best_of=best_of,
            skip_special_tokens=skip_special_tokens,
        )

        # Generate outputs
        outputs = self.llm.generate(prompts, sampling_params)

        # Extract generated texts
        results = []
        for output in outputs:
            if n == 1:
                results.append(output.outputs[0].text)
            else:
                results.append([o.text for o in output.outputs])

        # Return single string if input was single prompt
        if is_single_prompt:
            return results[0]
        return results

    def chat(
        self,
        messages: Union[List[Dict[str, str]], List[List[Dict[str, str]]]],
        max_tokens: int = 256,
        temperature: float = 0.7,
        top_p: float = 1.0,
        top_k: int = -1,
        frequency_penalty: float = 0.0,
        presence_penalty: float = 0.0,
        stop: Optional[Union[str, List[str]]] = None,
        skip_special_tokens: bool = True,
    ) -> Union[str, List[str]]:
        """
        Generate chat completions using the model's chat template.

        Args:
            messages: Single conversation (list of message dicts) or list of conversations.
                     Each message dict should have 'role' and 'content' keys.
                     Example: [{"role": "user", "content": "Hello"}]
            max_tokens: Maximum number of tokens to generate (default: 256)
            temperature: Sampling temperature (default: 0.7)
            top_p: Nucleus sampling probability (default: 1.0)
            top_k: Top-k sampling parameter (default: -1, disabled)
            frequency_penalty: Frequency penalty (default: 0.0)
            presence_penalty: Presence penalty (default: 0.0)
            stop: Stop sequences (default: None)
            skip_special_tokens: Whether to skip special tokens in output (default: True)

        Returns:
            Generated text(s). If input is a single conversation, returns a single string.
            If input is a list of conversations, returns a list of strings.

        Example:
            >>> provider = VLLMProvider(VLLMConfig(model="meta-llama/Llama-2-7b-chat-hf"))
            >>> messages = [
            ...     {"role": "system", "content": "You are a helpful assistant."},
            ...     {"role": "user", "content": "What is the capital of France?"}
            ... ]
            >>> response = provider.chat(messages)
            >>> print(response)
        """
        from vllm import SamplingParams

        # Check if single conversation or list of conversations
        is_single_conversation = isinstance(messages[0], dict)
        if is_single_conversation:
            messages = [messages]

        # Create sampling parameters
        sampling_params = SamplingParams(
            max_tokens=max_tokens,
            temperature=temperature,
            top_p=top_p,
            top_k=top_k,
            frequency_penalty=frequency_penalty,
            presence_penalty=presence_penalty,
            stop=stop,
            skip_special_tokens=skip_special_tokens,
        )

        # Generate outputs using chat interface
        outputs = self.llm.chat(messages, sampling_params)

        # Extract generated texts
        results = [output.outputs[0].text for output in outputs]

        # Return single string if input was single conversation
        if is_single_conversation:
            return results[0]
        return results

    def get_model_info(self) -> Dict[str, Any]:
        """
        Get information about the loaded model.

        Returns:
            Dictionary containing model information including name, max_model_len, etc.

        Example:
            >>> provider = VLLMProvider(VLLMConfig(model="facebook/opt-125m"))
            >>> info = provider.get_model_info()
            >>> print(f"Model: {info['model']}")
        """
        return {
            "model": self.config.model,
            "tensor_parallel_size": self.config.tensor_parallel_size,
            "dtype": self.config.dtype,
            "gpu_memory_utilization": self.config.gpu_memory_utilization,
            "max_model_len": self.config.max_model_len,
        }

    def __repr__(self) -> str:
        """String representation of the provider."""
        return f"VLLMProvider(model='{self.config.model}')"

