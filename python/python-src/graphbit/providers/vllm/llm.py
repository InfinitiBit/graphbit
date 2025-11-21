from vllm import LLM, SamplingParams
from typing import Optional, List, Union, Any, Dict
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
        
    # def chat(
    #     self,
    #     messages: Union[List[Dict[str, str]], List[List[Dict[str, str]]]],
    #     temperature: Optional[float] = None,
    #     max_tokens: Optional[int] = None,
    #     top_p: Optional[float] = None,
    #     top_k: Optional[int] = None,
    #     **kwargs,
    # ) -> Union[str, List[str]]:
    #     # Build sampling parameters
    #     sp = SamplingParams(
    #         temperature = temperature if temperature is not None else self.temperature,
    #         max_tokens = max_tokens if max_tokens is not None else self.max_tokens,
    #         top_p = top_p if top_p is not None else self.top_p,
    #         top_k = top_k if top_k is not None else self.top_k,
    #         **kwargs,
    #     )

    #     # Decide if single conversation or batch
    #     is_single = (len(messages) > 0 and isinstance(messages[0], dict))
    #     convs = [messages] if is_single else messages  # wrap single into list

    #     # Use vLLM chat API
    #     outputs = self.llm.chat(
    #         convs,
    #         sampling_params = sp,
    #         # you can pass other chat-specific args here:
    #         #   use_tqdm=False, chat_template=..., etc.
    #     )

    #     # outputs is a list of RequestOutput, extract text
    #     results = []
    #     for out in outputs:
    #         # out.outputs is a list of “generations”; take first
    #         results.append(out.outputs[0].text)

    #     # Return either single string or list
    #     if is_single:
    #         return results[0]
    #     else:
    #         return results
        
    def chat(
        self,
        messages,
        temperature: Optional[float] = None,
        max_tokens: Optional[int] = None,
        top_p: Optional[float] = None,
        top_k: Optional[int] = None,
        chat_template: Optional[str] = None,
        add_generation_prompt: bool = True,
        continue_final_message: bool = False,
        tools: Optional[List[Dict[str, Any]]] = None,
        chat_template_kwargs: Optional[Dict[str, Any]] = None,
        mm_processor_kwargs: Optional[Dict[str, Any]] = None,
        **kwargs: Any,
    ) -> Union[str, List[str]]:
        """
        Generate chat responses using vLLM, exposing full chat API options.
        """

        # Build sampling params (either single or list)
        sampling = SamplingParams(
            temperature=temperature if temperature is not None else self.temperature,
            max_tokens=max_tokens if max_tokens is not None else self.max_tokens,
            top_p=top_p if top_p is not None else self.top_p,
            top_k=top_k if top_k is not None else self.top_k,
            **kwargs,
        )

        # Decide -> pass single SamplingParams or list. For simplicity, we use a single one.
        sampling_params = sampling

        # Call vLLM chat
        outputs = self.llm.chat(
            messages=messages,
            sampling_params=sampling_params,
            chat_template=chat_template,
            add_generation_prompt=add_generation_prompt,
            continue_final_message=continue_final_message,
            tools=tools,
            chat_template_kwargs=chat_template_kwargs,
            mm_processor_kwargs=mm_processor_kwargs,
        )

        # Extract text from each RequestOutput; take the first generation
        results = [out.outputs[0].text for out in outputs]

        # Determine if single conversation
        # If input was a single conversation (not batch), return single string
        is_single_conv = (
            len(messages) > 0 and not isinstance(messages[0], list)
        )
        if is_single_conv:
            return results[0]
        return results
        