import requests

class HuggingfaceEmbeddings:
    def __init__(self, token: str):
        self.token = token

    def embed(self, model: str, text: str):
        headers = {"Authorization": f"Bearer {self.token}"}
        response = requests.post(
            f"https://api-inference.huggingface.co/pipeline/feature-extraction/{model}",
            headers=headers,
            json={"inputs": text}
        )
        response.raise_for_status()
        return response.json()
