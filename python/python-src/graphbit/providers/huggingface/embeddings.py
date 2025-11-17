from huggingface_hub import InferenceClient

class HuggingfaceEmbeddings:
    def __init__(self, token: str, **kwargs):
        self.client = InferenceClient(token=token, **kwargs)

    def embed(self, model: str, text: str, **kwargs):
        '''Embed the given text using a pre-trained model on Hugging Face Hub.'''''
        embeddings = self.client.feature_extraction(model=model, text=text, **kwargs)
        return embeddings
    
    def similarity(self, model: str, sentence: str, other_sentences: list, **kwargs):
        similarities = self.client.sentence_similarity(model=model, sentence=sentence, other_sentences=other_sentences, **kwargs)
        return similarities

