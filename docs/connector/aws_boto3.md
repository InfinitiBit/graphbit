# AWS Boto3 Integration with Graphbit


## Overview

This guide demonstrates how to integrate AWS services with the Graphbit ecosystem using the `boto3` library. We provide a comprehensive example using S3 (object storage) and DynamoDB (NoSQL database), including file upload, download, update, and content extraction for embeddings. You can use similar patterns to connect and interact with other AWS services through Boto3 in your Graphbit-powered workflows.

---

## Prerequisites

- **AWS account** with access to the services you want to use (e.g., S3, DynamoDB, etc.)
- **AWS credentials** configured (via environment variables, AWS CLI, or IAM roles)
- **Python environment** with `boto3` and `graphbit` installed:
  ```bash
  pip install boto3 graphbit
  ```
- **OpenAI API Key** for embeddings
- **Environment variable** for your OpenAI API key:
  ```bash
  OPENAI_API_KEY="your_openai_api_key_here"
  ```

---

## Step 1: Connect to AWS Services with Boto3

Below, we show how to connect to S3 and DynamoDB. You can use the same approach to connect to other AWS services supported by Boto3.

```python
import boto3

REGION = '<your-region>'
BUCKET_NAME = '<your-bucket-name>'
TABLE_NAME = '<your-table-name>'

# Connect to S3
s3 = boto3.client('s3', region_name=REGION)
print(f"Connected to S3 in region '{REGION}'.")

# Connect to DynamoDB
dynamodb = boto3.resource('dynamodb', region_name=REGION)
table = dynamodb.Table(TABLE_NAME)
print(f"Connected to DynamoDB table '{TABLE_NAME}' in region '{REGION}'.")
```

> **Note:** For other AWS services, refer to the [Boto3 documentation](https://boto3.amazonaws.com/v1/documentation/api/latest/index.html) for the appropriate client or resource initialization.

---

## Step 2: S3 File Upload, Download, Update, and Listing

This example demonstrates uploading a file to S3, listing objects, downloading and reading a file, appending new content, and re-uploading the updated file. These patterns can be used for other S3 operations as well.

```python
# Create and upload a test file
with open('test_file.txt', 'w') as f:
    f.write('Hello from Graphbit integration demo!')
    
s3.upload_file('test_file.txt', BUCKET_NAME, 'test_file.txt')
print(f"Uploaded 'test_file.txt' to S3 bucket '{BUCKET_NAME}' as 'test_file.txt'.")

# List objects in S3 bucket
response = s3.list_objects_v2(Bucket=BUCKET_NAME)
for obj in response.get('Contents', []):
    print(" -", obj['Key'])

# Download and read the file from S3
s3.download_file(BUCKET_NAME, 'test_file.txt', 'downloaded_file.txt')
with open('downloaded_file.txt', 'r') as f:
    content = f.read()
    print("Content of file from S3:")
    print(content)

# Append new lines to the file and re-upload
batch_texts = [
    "This is a sample document for vector search.",
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
with open('downloaded_file.txt', 'a') as f:
    for line in batch_texts:
        f.write('\n' + line)
s3.upload_file('downloaded_file.txt', BUCKET_NAME, 'test_file.txt')
print("Updated file uploaded to S3.")

# Download and print updated file
s3.download_file(BUCKET_NAME, 'test_file.txt', 'downloaded_file.txt')
with open('downloaded_file.txt', 'r') as f:
    print("\nContent of file from S3 after update:")
    print(f.read())
```
---

## Step 3: Embedding with Graphbit and Storing in DynamoDB

You can use Graphbit to generate vector embeddings and store them in DynamoDB. In this workflow, we extract the second line from the file downloaded from S3 for a single embedding.

```python

import os
from decimal import Decimal
from graphbit import EmbeddingConfig as gb_ecg, EmbeddingClient as gb_etc

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
if not OPENAI_API_KEY:
    raise ValueError("Please set the OPENAI_API_KEY environment variable.")

embedding_config = gb_ecg.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = gb_etc(embedding_config)

def float_list_to_decimal(lst):
    return [Decimal(str(x)) for x in lst]

# Extract the second line for single embedding
with open('downloaded_file.txt', 'r') as f:
    lines = f.readlines()
if len(lines) >= 2:
    text = lines[1].strip()
    print("Extracted second sentence:", text)
else:
    print("File does not have a second sentence.")
embedding = embedding_client.embed(text)
table.put_item(Item={
    "itemID": "item-embedding-1",
    "embedding": float_list_to_decimal(embedding),
    "metadata": {"category": "test"}
})
print("Stored single embedding in DynamoDB.")
```

---

## Step 4: Batch Embedding Example (Extracting from File)

This example demonstrates how to generate and store multiple embeddings in DynamoDB in a batch. Here, we extract lines 3 to 5 from the file downloaded from S3 for batch embedding.

```python
# Extract lines 3 to 5 for batch embedding
with open('downloaded_file.txt', 'r') as f:
    lines = f.readlines()
if len(lines) >= 5:
    batch_texts = [lines[2].strip(), lines[3].strip(), lines[4].strip()]
    print("batch_texts =", batch_texts)
else:
    print("File does not have enough lines.")
batch_embeddings = embedding_client.embed_many(batch_texts)
batch_items = [
    {
        "itemID": f"batch_{i}",
        "embedding": float_list_to_decimal(emb),
        "metadata": {"text": text}
    }
    for i, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
]
with table.batch_writer() as batch:
    for item in batch_items:
        batch.put_item(Item=item)
print(f"Inserted {len(batch_items)} batch embeddings.")
```
---

## Step 5: Vector Search using Graphbit

This example shows how to perform a simple vector similarity search using embeddings stored in DynamoDB. You can adapt this logic for other AWS data stores or search services.

```python
query_embedding = embedding_client.embed("Find documents related to vector search.")
scan_resp = table.scan()
items = scan_resp.get('Items', [])
best_score = -1
best_item = None
for item in items:
    if 'embedding' in item:
        score = gb_etc.similarity(query_embedding, item['embedding'])
        if score > best_score:
            best_score = score
            best_item = item
if best_item:
    print(f"Most similar itemID: {best_item['itemID']} (score: {best_score:.4f})")
else:
    print("No embeddings found in table.")
```

---

**This connector pattern enables you to use AWS Boto3 to integrate Graphbit with a wide range of AWS services. While this guide uses S3 and DynamoDB as examples, you can extend these patterns to other AWS services to build powerful, cloud-native AI workflows.** 
