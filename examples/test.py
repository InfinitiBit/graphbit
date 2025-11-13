import os
from graphbit import Huggingface
from dotenv import load_dotenv

load_dotenv()

# Initialize Hugging Face integration
hf = Huggingface(os.getenv("HUGGINGFACE_API_KEY"))

messages=[
        {"role": "user", "content": "What is langchain?"},
    ]

# Generate text using a model
# response = hf.inference.generate_text(
#     model="openai/gpt-oss-20b",
#     prompt="What is langchain?",
#     max_tokens=50,
#     temperature=0.8
# )

# print("Generated Text:\n", response)
# print("\n\n\n")
# print("Content: ", response.choices[0].message.content)

# CHAT WITH A MODEL
# response = hf.llm.chat(
#     model="moonshotai/Kimi-K2-Thinking",
#     messages=[
#         {"role": "user", "content": "What is langchain?"},
#     ],
#     max_tokens=500,
#     temperature=0.5
# )
# print("\nChat Response:\n", response)
# print("\n\n\n")

# result = hf.llm.get_output_content(response)

# print("Content:\n", result)

# EMBEDDINGS EXAMPLE
# text = "This is an example of embedding generation."
# embeddings = hf.embeddings.embed(model="sentence-transformers/all-MiniLM-L6-v2", text=text)
# print(embeddings)

# SIMILARITY EXAMPLE
sentence = "Machine learning is so easy."
other_sentences=[
    "Deep learning is so straightforward.",
    "This is so difficult, like rocket science.",
    "I can't believe how much I struggled with this.",
]
similarities = hf.embeddings.similarity(
    model="sentence-transformers/all-MiniLM-L6-v2", 
    sentence=sentence, 
    other_sentences=other_sentences
)

print(similarities)
