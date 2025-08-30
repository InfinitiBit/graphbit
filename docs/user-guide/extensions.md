# GraphBit Extension System

GraphBit provides a comprehensive extension system that allows you to integrate with various third-party services and databases. Extensions are organized by category and support optional dependencies, making it easy to install only what you need.

## Installation

Extensions are installed as optional dependencies using the bracket syntax:

```bash
# Install specific extensions
pip install graphbit[pymongo]
pip install graphbit[pinecone]
pip install graphbit[aws_boto3]

# Install multiple extensions
pip install graphbit[pymongo,pinecone,chromadb]

# Install all extensions in a category
pip install graphbit[vector_databases]
pip install graphbit[traditional_databases]
pip install graphbit[cloud_providers]

# Install all available extensions
pip install graphbit[all]
```

## Usage

Once installed, extensions can be imported directly from the `graphbit.extension` module:

```python
# Import specific extensions
from graphbit.extension import pymongo
from graphbit.extension import pinecone
from graphbit.extension import aws_boto3
from graphbit.extension import google_search_api

# Check if dependencies are available
if pymongo.check_dependencies():
    client = pymongo.PyMongoClient(uri="mongodb://localhost:27017")
else:
    print("PyMongo dependencies not installed")
```

## Available Extensions

### Vector Databases

| Extension | Package | Description |
|-----------|---------|-------------|
| `pinecone` | `pinecone-client` | Pinecone vector database |
| `qdrant` | `qdrant-client` | Qdrant vector database |
| `weaviate` | `weaviate-client` | Weaviate vector database |
| `chromadb` | `chromadb` | ChromaDB vector database |
| `milvus` | `pymilvus` | Milvus vector database |
| `faiss` | `faiss-cpu` | Facebook AI Similarity Search |
| `astradb` | `astrapy` | DataStax Astra DB |

### Traditional Databases

| Extension | Package | Description |
|-----------|---------|-------------|
| `pymongo` | `pymongo` | MongoDB database |
| `pgvector` | `psycopg2-binary`, `pgvector` | PostgreSQL with vector support |
| `mariadb` | `mariadb` | MariaDB database |
| `db2` | `ibm-db` | IBM DB2 database |
| `elasticsearch` | `elasticsearch` | Elasticsearch search engine |

### Cloud Providers

| Extension | Package | Description |
|-----------|---------|-------------|
| `aws_boto3` | `boto3` | Amazon Web Services |
| `azure` | `azure-storage-blob`, `azure-cosmos` | Microsoft Azure |
| `google_cloud_platform` | `google-cloud-storage`, `google-cloud-firestore` | Google Cloud Platform |

### Search Engines

| Extension | Package | Description |
|-----------|---------|-------------|
| `google_search_api` | `google-api-python-client` | Google Custom Search API |

## Extension Examples

### MongoDB (PyMongo)

```python
from graphbit.extension import pymongo

# Create client
client = pymongo.PyMongoClient(
    uri="mongodb://localhost:27017",
    database="my_database"
)

# Insert document
client.insert_document("my_collection", {"name": "Alice", "age": 30})

# Find documents
docs = client.find_documents("my_collection", {"age": {"$gte": 25}})
```

### Pinecone Vector Database

```python
from graphbit.extension import pinecone

# Create client
client = pinecone.PineconeClient(api_key="your-api-key")

# Create index
client.create_index("my-index", dimension=1536, metric="cosine")

# Upsert vectors
vectors = [("id1", [0.1, 0.2, ...], {"text": "sample"})]
client.upsert_vectors("my-index", vectors)

# Query vectors
results = client.query_vectors("my-index", [0.1, 0.2, ...], top_k=5)
```

### AWS Boto3

```python
from graphbit.extension import aws_boto3

# Create client
client = aws_boto3.AWSBoto3Client(region_name="us-east-1")

# Upload file to S3
client.upload_file_to_s3("local_file.txt", "my-bucket", "remote_file.txt")

# Put item in DynamoDB
client.put_item_dynamodb("my-table", {"id": "123", "data": "value"})
```

### Google Search API

```python
from graphbit.extension import google_search_api

# Create client
client = google_search_api.GoogleSearchAPIClient(
    api_key="your-api-key",
    search_engine_id="your-search-engine-id"
)

# Perform search
results = client.search("GraphBit AI framework", num_results=10)
for item in results.get('items', []):
    print(f"Title: {item['title']}")
    print(f"URL: {item['link']}")
```

## Best Practices

1. **Check Dependencies**: Always check if dependencies are available before using an extension
2. **Handle Errors**: Use proper error handling for connection and dependency issues
3. **Environment Variables**: Use environment variables for sensitive configuration like API keys
4. **Lazy Loading**: Extensions are loaded lazily, so imports are fast even if dependencies aren't installed
5. **Optional Installation**: Only install extensions you actually need to keep your environment lean