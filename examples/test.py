from graphbit.providers import Litellm
from graphbit import LlmClient, LlmConfig, Workflow, Node, Executor, EmbeddingClient, EmbeddingConfig
from dotenv import load_dotenv
import os

load_dotenv()

# openai_api_key=os.getenv("OPENAI_API_KEY")
# mistral_api_key=os.getenv("MISTRALAI_API_KEY")
# hf_api_key=os.getenv("HUGGINGFACE_API_KEY")


llm = Litellm()

# response = llm.llm.chat(
#     model="gpt-4o-mini",
#     messages=[{"role": "user", "content": "What is Langchain?"}],
#     max_tokens=10
# )

# print(llm.llm.get_output_content(response))

embeddings = llm.embeddings.embed(
    model="openai/text-embedding-3-small",
    text="What is langchain?",
)

print(embeddings)

# config = LlmConfig.litellm(api_key=openai_api_key, model="")

# # openai_config = LlmConfig.openai(openai_api_key)

# # hf_config = LlmConfig.huggingface(openai_api_key)

# # client = LlmClient(config)

# # response = client.complete(prompt="What is Langchain?", max_tokens=10)

# # print(response)

# # Create executor
# executor = Executor(config)

# # Build workflow
# workflow = Workflow("Analysis Pipeline")

# # Create agent nodes
# smart_agent = Node.agent(
#     name="Smart Agent",
#     prompt="What is Crewai?",
#     system_prompt="You are a skilled agent who only knows about Langchain and nothing else.",
#     max_tokens=50
# )

# workflow.add_node(smart_agent)

# result = executor.execute(workflow)

# print("Result:\n", result.get_node_output("Smart Agent"))

# config = EmbeddingConfig.litellm(api_key=openai_api_key, model="text-embedding-ada-002")

# client = EmbeddingClient(config)

# # embeddings = client.embed("What is langchain?")

# batch_embeddings = client.embed_many(["What is langchain?", "What is GraphBit?", "What is my name?"])

# print(batch_embeddings)
