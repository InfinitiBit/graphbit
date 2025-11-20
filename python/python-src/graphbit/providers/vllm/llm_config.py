from typing import Optional, Dict, Any

class VLLMConfig:
    def __init__(
        self,
        model: str,
        tensor_parallel_size: int = 1,
        gpu_memory_utilization: float = 0.9,
        max_model_len: Optional[int] = None,
        **kwargs: Any
    ):
        self.model = model
        self.tensor_parallel_size = tensor_parallel_size
        self.gpu_memory_utilization = gpu_memory_utilization
        self.max_model_len = max_model_len
        self.kwargs = kwargs

    def to_dict(self) -> Dict[str, Any]:
        config = {
            "model": self.model,
            "tensor_parallel_size": self.tensor_parallel_size,
            "gpu_memory_utilization": self.gpu_memory_utilization,
        }
        if self.max_model_len is not None:
            config["max_model_len"] = self.max_model_len
        config.update(self.kwargs)
        return config
