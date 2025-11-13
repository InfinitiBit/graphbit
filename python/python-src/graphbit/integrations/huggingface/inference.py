from huggingface_hub import InferenceClient

class HuggingfaceInference:
    def __init__(self, token: str):
        self.client = InferenceClient(token=token)

    def generate_image(self, model: str, prompt: str, **kwargs):
        return self.client.text_to_image(model=model, prompt=prompt, **kwargs)
