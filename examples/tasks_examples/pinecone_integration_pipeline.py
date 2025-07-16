import logging
import os
import uuid
from typing import List

from dotenv import load_dotenv
from pinecone import Pinecone

import graphbit

load_dotenv()


class PineconeVectorPipeline:
    def __init__(self, index_name: str = "graphbit-vector"):
        graphbit.init()

        # Initialize Pinecone
        self.pinecone_client = Pinecone(api_key=os.getenv("PINECONE_API_KEY"))
        self.index_name = index_name
        index_list = self.pinecone_client.list_indexes()
        if index_name and not any(idx["name"] == index_name for idx in index_list):
            self.pc.create_index(name=index_name, dimension=1536, metric="cosine")

        self.index = self.pc.Index(index_name)
        self.llm_config = graphbit.LlmConfig.openai(model="gpt-4o-mini", api_key=os.getenv("OPENAI_API_KEY"))
        self.executor = graphbit.Executor(self.llm_config)
        self.embedding_config = graphbit.EmbeddingConfig.openai(model="text-embedding-3-small", api_key=os.getenv("OPENAI_API_KEY"))
        self.embedding_client = graphbit.EmbeddingClient(self.embedding_config)

        self.vs = self.index
        self.vs.ensure_collection_exists("graphbit_collection")

    def store(self, texts):
        embeddings = self.embedding_client.embed_many(texts)
        ids = [str(uuid.uuid4()) for _ in texts]
        vectors = [(f"doc_{ids[i]}_{i}", embeddings[i], {"text": texts[i]}) for i in range(len(texts))]
        self.index.upsert(vectors=vectors)
        return f"Stored {len(texts)} docs"

    def retrieve(self, query, top_k=3):
        emb = self.embedding_client.embed(query)
        resp = self.index.query(vector=emb, top_k=2, include_metadata=True)
        docs = []
        for m in resp["matches"]:
            print(m["id"], m["metadata"]["text"], m["score"])
            docs.append(m["metadata"]["text"])
        return docs

    def run_workflow(self, docs: List[str], query: str) -> List[str]:
        """Execute the complete GraphBit workflow with Pinecone operations."""
        print("[LOG] Starting GraphBit-Pinecone integration pipeline...")

        # Create workflow
        workflow = graphbit.Workflow("Pinecone Integration Pipeline")
        # Step 1: Store documents
        store_agent_id = str(uuid.uuid4())
        store_prompt = f"Process and confirm storage of documents: {docs}"
        store_node = graphbit.Node.agent(name="Document Storage", prompt=store_prompt, agent_id=store_agent_id)
        store_node_id = workflow.add_node(store_node)

        # Step 2: Query documents
        query_agent_id = str(uuid.uuid4())
        query_prompt = f"Process query and return relevant results for: {query}"
        query_node = graphbit.Node.agent(name="Document Query", prompt=query_prompt, agent_id=query_agent_id)
        query_node_id = workflow.add_node(query_node)

        # Connect nodes sequentially
        workflow.connect(store_node_id, query_node_id)

        # Validate workflow
        workflow.validate()

        # Execute actual Pinecone operations
        print("[LOG] Storing documents...")
        store_result = self.store_documents(docs)
        print(f"[LOG] {store_result}")

        print("[LOG] Querying documents...")
        query_results = self.query_documents(query)

        # Execute GraphBit workflow for orchestration
        print("[LOG] Executing GraphBit workflow...")
        result = self.executor.execute(workflow)

        if result.is_failed():
            print("[ERROR] Workflow execution failed:", result.state())
        else:
            print("[LOG] Workflow completed successfully")

        return query_results


# ------- Run Pipeline -------
if __name__ == "__main__":
    input_data = {
        "docs": [
            "GraphBit is a framework for LLM workflows and agent orchestration.",
            "Pinecone enables vector search over high-dimensional embeddings.",
            "OpenAI offers tools for LLMs and embeddings.",
        ],
        "query": "What is GraphBit?",
    }

    # Create pipeline instance
    pipeline = PineconeVectorPipeline()

    # Run the complete workflow
    results = pipeline.run_workflow(input_data["docs"], input_data["query"])

    # Display results
    print("\n--- Final Output ---")
    for i, item in enumerate(results, 1):
        print(f"{i}. {item}")
