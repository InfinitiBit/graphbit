from huggingface_hub import InferenceClient

class HuggingfaceLLM:
    def __init__(self, token: str):
        self.client = InferenceClient(token=token)

    def chat(self, model: str, messages: list, **kwargs):
        # text = "\n".join([f"{m['role']}: {m['content']}" for m in messages])
        return self.client.chat.completions.create(model=model, messages=messages, **kwargs)
    