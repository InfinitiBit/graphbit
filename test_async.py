import asyncio
from graphbit.providers import Litellm

llm = Litellm()

async def test_async():
    embeddings = await llm.embeddings.aembed(
        model="openai/text-embedding-3-small",
        text="What is Langchain?",
    )

    await llm.aclose()
    return embeddings

response = asyncio.run(test_async())

print(response)

# if __name__ == "__main__":
#     asyncio.run(test_async())
