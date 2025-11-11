from graphbit import Huggingface

# Initialize Hugging Face integration
hf = Huggingface(api_token="")

# # Generate text using a model
# response = hf.inference.generate_text(
#     model="openai/gpt-oss-20b",
#     prompt="Once upon a time in a world where AI ruled the cities,",
#     max_new_tokens=50,
#     temperature=0.8
# )

# print("Generated Text:\n", response)

# chat with a model
response = hf.llm.chat(
    model="google/gemma-2-2b-it",
    messages=[
        {"role": "user", "content": "Hello, how are you?"},
        {"role": "assistant", "content": "I'm doing well, thank you!"},
        {"role": "user", "content": "Can you help me with Python programming?"}
    ]
)
print("\nChat Response:\n", response)
