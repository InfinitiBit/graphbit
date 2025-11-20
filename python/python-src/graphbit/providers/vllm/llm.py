from vllm import LLM, SamplingParams
from typing import Optional, List, Union
from .llm_config import VLLMConfig


class VLLMProvider:

    def __init__(
        self,
        config: Optional[VLLMConfig] = None,
        model: Optional[str] = None,
        temperature: float = 1.0,
        max_tokens: Optional[int] = 16,
        top_p: float = 1.0,
        top_k: int = -1,
        **llm_kwargs
    ):
        if config is None and model is None:
            raise ValueError("Either config or model must be provided")

        # Initialize LLM
        if config is not None:
            self.config = config
            self.llm = LLM(**config.to_dict())
        else:
            self.config = VLLMConfig(model=model, **llm_kwargs)
            self.llm = LLM(model=model, **llm_kwargs)

        # Store sampling parameters
        self.temperature = temperature
        self.max_tokens = max_tokens
        self.top_p = top_p
        self.top_k = top_k

    def generate(
        self,
        prompt: Union[str, List[str]],
        temperature: Optional[float] = None,
        max_tokens: Optional[int] = None,
        top_p: Optional[float] = None,
        top_k: Optional[int] = None,
        **kwargs
    ) -> Union[str, List[str]]:
        # Create sampling parameters
        sampling_params = SamplingParams(
            temperature=temperature if temperature is not None else self.temperature,
            max_tokens=max_tokens if max_tokens is not None else self.max_tokens,
            top_p=top_p if top_p is not None else self.top_p,
            top_k=top_k if top_k is not None else self.top_k,
            **kwargs
        )

        # Handle single prompt vs list of prompts
        is_single = isinstance(prompt, str)
        prompts = [prompt] if is_single else prompt

        # Generate outputs
        outputs = self.llm.generate(prompts, sampling_params)

        # Extract text from outputs
        results = [output.outputs[0].text for output in outputs]

        return results[0] if is_single else results
        