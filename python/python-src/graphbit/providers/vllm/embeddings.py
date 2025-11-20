from vllm import LLM
from typing import Optional, List, Union
from .llm_config import VLLMConfig

class VLLMEmbeddings:
    def __init__(
        self,
        config: Optional[VLLMConfig] = None,
        model: Optional[str] = None,
        **llm_kwargs
    ):
        if config is None and model is None:
            raise ValueError("Either config or model must be provided")

        # Initialize LLM for embeddings
        if config is not None:
            self.config = config
            self.llm = LLM(**config.to_dict())
        else:
            self.config = VLLMConfig(model=model, **llm_kwargs)
            self.llm = LLM(model=model, **llm_kwargs)

    def embed(
        self,
        text: Union[str, List[str]],
        **kwargs
    ) -> Union[List[float], List[List[float]]]:
        # Handle single text vs list of texts
        is_single = isinstance(text, str)
        texts = [text] if is_single else text

        # Generate embeddings using vLLM's encode method
        outputs = self.llm.encode(texts, **kwargs)

        # Extract embeddings from outputs
        embeddings = [output.outputs.embedding for output in outputs]

        return embeddings
    