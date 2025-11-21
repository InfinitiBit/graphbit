from .llm_config import VLLMConfig
from .llm import VLLMProvider
from .embeddings import VLLMEmbeddings

class vLLM:
    def __init__(
        self,
        model: str,
        tensor_parallel_size: int = 1,
        gpu_memory_utilization: float = 0.9,
        **kwargs
    ):
        # Create configuration
        self.config = VLLMConfig(
            model=model,
            tensor_parallel_size=tensor_parallel_size,
            gpu_memory_utilization=gpu_memory_utilization,
            **kwargs
        )

        # Initialize LLM and embeddings with the config
        self.llm = VLLMProvider(config=self.config)
        self.embeddings = VLLMEmbeddings(config=self.config)
