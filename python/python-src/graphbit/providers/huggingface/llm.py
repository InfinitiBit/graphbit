from huggingface_hub import InferenceClient

class HuggingfaceLLM:
    def __init__(self, token: str, **kwargs):
        self.client = InferenceClient(token=token, **kwargs)

    def chat(self, model: str, messages: list, **kwargs):
        '''Chat with a pre-trained model on Hugging Face Hub.'''
        # text = "\n".join([f"{m['role']}: {m['content']}" for m in messages])
        return self.client.chat_completion(model=model, messages=messages, **kwargs)

    def get_output_content(self, response):
        '''Extract the output content from the response.'''
        return response.choices[0].message.content
