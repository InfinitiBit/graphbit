# vLLM Provider for GraphBit

This is a pure Python implementation of a vLLM provider for GraphBit, independent from the Rust core.

## Overview

The vLLM provider enables high-throughput local LLM inference using vLLM's state-of-the-art serving engine with PagedAttention and continuous batching.

## Installation

```bash
# Install graphbit with vLLM support
pip install graphbit[vllm]

# Or install vLLM separately
pip install vllm
```

**Requirements:**
- Python 3.10-3.13
- CUDA-capable GPU (recommended)
- CUDA 11.8 or higher

## Quick Start

```python
from graphbit.vllm import VLLMConfig, VLLMProvider

# Configure vLLM
config = VLLMConfig(
    model="facebook/opt-125m",
    gpu_memory_utilization=0.9,
)

# Create provider
provider = VLLMProvider(config)

# Generate text
result = provider.generate(
    "The capital of France is",
    max_tokens=50,
    temperature=0.7,
)
print(result)
```

## API Reference

### VLLMConfig

Configuration class for vLLM provider.

**Parameters:**
- `model` (str): Model name or path (from Hugging Face)
- `tensor_parallel_size` (int): Number of GPUs for tensor parallelism (default: 1)
- `dtype` (str): Data type for model weights (default: "auto")
- `gpu_memory_utilization` (float): GPU memory utilization (0.0-1.0, default: 0.9)
- `max_model_len` (Optional[int]): Maximum sequence length (default: None)
- `trust_remote_code` (bool): Trust remote code from model (default: False)
- `download_dir` (Optional[str]): Model cache directory (default: None)
- `seed` (int): Random seed (default: 0)
- `revision` (Optional[str]): Model revision (default: None)
- `tokenizer_revision` (Optional[str]): Tokenizer revision (default: None)
- `quantization` (Optional[str]): Quantization method ("awq", "gptq", etc.)
- `enforce_eager` (bool): Disable CUDA graphs (default: False)
- `max_num_seqs` (int): Maximum sequences per iteration (default: 256)
- `max_num_batched_tokens` (Optional[int]): Maximum tokens per batch (default: None)
- `enable_prefix_caching` (bool): Enable prefix caching (default: False)
- `disable_custom_all_reduce` (bool): Disable custom all-reduce (default: False)

**Methods:**
- `to_dict()`: Convert configuration to dictionary

### VLLMProvider

Provider class for vLLM inference.

**Methods:**

#### `generate(prompts, **sampling_params) -> Union[str, List[str]]`

Generate text from prompt(s).

**Parameters:**
- `prompts` (Union[str, List[str]]): Single prompt or list of prompts
- `max_tokens` (int): Maximum tokens to generate (default: 256)
- `temperature` (float): Sampling temperature (default: 0.7)
- `top_p` (float): Nucleus sampling probability (default: 1.0)
- `top_k` (int): Top-k sampling (default: -1, disabled)
- `frequency_penalty` (float): Frequency penalty (default: 0.0)
- `presence_penalty` (float): Presence penalty (default: 0.0)
- `stop` (Optional[Union[str, List[str]]]): Stop sequences (default: None)
- `n` (int): Number of completions per prompt (default: 1)
- `best_of` (Optional[int]): Generate best_of and return n best (default: None)
- `skip_special_tokens` (bool): Whether to skip special tokens in output (default: True)

**Note:** Parameters `use_beam_search`, `length_penalty`, and `early_stopping` are not supported in vLLM 0.11.0.
- `use_beam_search` (bool): Use beam search (default: False)

**Returns:**
- Single string if input is string
- List of strings if input is list

#### `chat(conversations, **sampling_params) -> Union[str, List[str]]`

Generate chat completions using model's chat template.

**Parameters:**
- `conversations` (Union[List[Dict], List[List[Dict]]]): Single conversation or list of conversations
  - Each message: `{"role": "user"|"assistant"|"system", "content": "..."}`
- `**sampling_params`: Same as `generate()`

**Returns:**
- Single string if input is single conversation
- List of strings if input is list of conversations

#### `get_model_info() -> Dict[str, Any]`

Get model configuration information.

**Returns:**
- Dictionary with model configuration

## Examples

### Batch Generation

```python
from graphbit.vllm import VLLMConfig, VLLMProvider

config = VLLMConfig(model="meta-llama/Llama-2-7b-hf")
provider = VLLMProvider(config)

prompts = [
    "What is machine learning?",
    "What is deep learning?",
    "What is neural network?",
]

results = provider.generate(prompts, max_tokens=100, temperature=0.7)

for prompt, result in zip(prompts, results):
    print(f"Q: {prompt}")
    print(f"A: {result}\n")
```

### Chat Interface

```python
from graphbit.vllm import VLLMConfig, VLLMProvider

config = VLLMConfig(model="meta-llama/Llama-2-7b-chat-hf")
provider = VLLMProvider(config)

messages = [
    {"role": "user", "content": "What is the capital of France?"}
]

response = provider.chat(messages, max_tokens=100, temperature=0.7)
print(response)
```

### Advanced Configuration

```python
from graphbit.vllm import VLLMConfig, VLLMProvider

config = VLLMConfig(
    model="meta-llama/Llama-2-7b-hf",
    tensor_parallel_size=2,  # Use 2 GPUs
    dtype="float16",
    gpu_memory_utilization=0.8,
    max_model_len=4096,
    enable_prefix_caching=True,
    quantization="awq",  # Use AWQ quantization
)

provider = VLLMProvider(config)
```

## Architecture

The vLLM provider is implemented as a pure Python module:

```
python/python-src/graphbit/
├── __init__.py          # Package initialization
└── vllm.py             # vLLM provider implementation
```

Key design decisions:

1. **Pure Python**: Independent from Rust core, no PyO3 bindings
2. **Lazy Initialization**: vLLM engine initialized on first use
3. **Flexible API**: Supports both single and batch inference
4. **Chat Support**: Built-in chat template support
5. **Error Handling**: Graceful handling of missing vLLM installation

## Testing

### Unit Tests

```bash
python3 -m pytest tests/python_unit_tests/test_vllm_provider.py -v
```

Unit tests cover:
- VLLMConfig initialization and configuration
- VLLMProvider initialization and lazy loading
- Generate method with single and batch prompts
- Error handling for missing vLLM

### Integration Tests

```bash
python3 -m pytest tests/python_integration_tests/test_vllm_integration.py -v -m integration
```

Integration tests require vLLM installation and GPU:
- Text generation with various sampling parameters
- Batch generation
- Chat interface
- Prefix caching
- Model information retrieval

## Performance Tips

1. **Batch Processing**: Always batch requests for maximum throughput
2. **GPU Memory**: Adjust `gpu_memory_utilization` (0.8-0.9 typical)
3. **Quantization**: Use AWQ/GPTQ for larger models
4. **Tensor Parallelism**: Use multiple GPUs for models >13B
5. **Prefix Caching**: Enable for repeated prompt prefixes

## Troubleshooting

### ImportError: vLLM not installed

```bash
pip install vllm
```

### CUDA out of memory

Reduce `gpu_memory_utilization` or use quantization:

```python
config = VLLMConfig(
    model="your-model",
    gpu_memory_utilization=0.7,  # Reduce from 0.9
    quantization="awq",  # Use quantization
)
```

### Model not found

Ensure model exists on Hugging Face or provide local path:

```python
config = VLLMConfig(model="/path/to/local/model")
```

## Contributing

The vLLM provider is part of the GraphBit project. See the main GraphBit documentation for contribution guidelines.

## License

Same as GraphBit project license.

