from .inference import HuggingfaceInference
from .llm import HuggingfaceLLM
from .embeddings import HuggingfaceEmbeddings

class Huggingface:
    """
    Unified Hugging Face integration for inference, LLM, and embeddings.
    """

    def __init__(self, api_token: str):
        self.api_token = api_token
        self.inference = HuggingfaceInference(api_token)
        self.llm = HuggingfaceLLM(api_token)
        self.embeddings = HuggingfaceEmbeddings(api_token)
