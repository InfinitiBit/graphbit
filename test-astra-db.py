# import os
# from astrapy import DataAPIClient
# import graphbit

# graphbit.init()
# OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
# ASTRA_DB_APPLICATION_TOKEN = os.getenv("ASTRA_DB_APPLICATION_TOKEN")
# ASTRA_DB_API_ENDPOINT = os.getenv("ASTRA_DB_API_ENDPOINT")

# client = DataAPIClient(ASTRA_DB_APPLICATION_TOKEN)
# astra_db = client.get_database_by_api_endpoint(ASTRA_DB_API_ENDPOINT)

# # General CRUD
# col = astra_db.create_collection("general_data")
# col.insert_one({"name": "Alice", "role": "engineer", "age": 30})
# print(col.find_one({"name": "Alice"}))
# col.update_one({"name": "Alice"}, {"$set": {"age": 31}})
# print(col.find_one({"name": "Alice"}))
# col.delete_one({"name": "Alice"})

# # Vector storage and search
# embedding_config = graphbit.EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
# embedding_client = graphbit.EmbeddingClient(embedding_config)
# text = "This is a sample document for vector search."
# embedding = embedding_client.embed(text)

# vec_col = astra_db.create_collection(
#     name="vector_data",
#     definition={"vector": {"dimension": 1536, "metric": "cosine"}}
# )
# vec_col.insert_one({"_id": "item123", "$vector": embedding})

# query_embedding = embedding_client.embed("Find documents related to vector search.")
# results = list(vec_col.find({}, sort={"$vector": query_embedding}, limit=5))
# best_doc = max(results, key=lambda doc: doc.get("$similarity", 0), default=None)
# if best_doc:
#     print(f"Most similar document: {best_doc['_id']}")

# # Batch insert
# batch_texts = [
#     "Graph databases are great for relationships.",
#     "Vector search enables semantic retrieval.",
#     "OpenAI provides powerful embedding models.",
# ]
# batch_embeddings = embedding_client.embed_many(batch_texts)
# vec_col.insert_many([
#     {"_id": f"batch_{i}", "$vector": emb, "metadata": {"text": text}}
#     for i, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
# ])
import os
from astrapy import DataAPIClient
import graphbit

graphbit.init()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
ASTRA_DB_APPLICATION_TOKEN = os.getenv("ASTRA_DB_APPLICATION_TOKEN")
ASTRA_DB_API_ENDPOINT = os.getenv("ASTRA_DB_API_ENDPOINT")

client = DataAPIClient(ASTRA_DB_APPLICATION_TOKEN)
astra_db = client.get_database_by_api_endpoint(ASTRA_DB_API_ENDPOINT)

# General CRUD
col = astra_db.create_collection("general_data")
col.insert_one({"name": "Alice", "role": "engineer", "age": 30})
print(col.find_one({"name": "Alice"}))
col.update_one({"name": "Alice"}, {"$set": {"age": 31}})
print(col.find_one({"name": "Alice"}))
col.delete_one({"name": "Alice"})

# Vector storage and search
embedding_config = graphbit.EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = graphbit.EmbeddingClient(embedding_config)
text = "This is a sample document for vector search."
embedding = embedding_client.embed(text)
vec_col = astra_db.create_collection(
    name="vector_data",
    definition={"vector": {"dimension": 1536, "metric": "cosine"}}
)
vec_col.insert_one({"_id": "item123", "$vector": embedding})

query_embedding = embedding_client.embed("Find documents related to vector search.")
results = list(vec_col.find({}, sort={"$vector": query_embedding}, limit=5))
best_doc = max(results, key=lambda doc: doc.get("$similarity", 0), default=None)
if best_doc:
    print(f"Most similar document: {best_doc['_id']}")

# Batch insert
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
batch_embeddings = embedding_client.embed_many(batch_texts)
vec_col.insert_many([
    {"_id": f"batch_{i}", "$vector": emb, "metadata": {"text": text}}
    for i, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
])
