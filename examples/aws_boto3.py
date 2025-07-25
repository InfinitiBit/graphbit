import os
import boto3

# --- Config ---
REGION = 'ap-southeast-1'
BUCKET_NAME = 'graphbit-integration'
TABLE_NAME = 'graphbit-integration'

# --- Connect to AWS Services ---
s3 = boto3.client('s3', region_name=REGION)
print(f"Connected to S3 in region '{REGION}'.")

dynamodb = boto3.resource('dynamodb', region_name=REGION)
table = dynamodb.Table(TABLE_NAME)
print(f"Connected to DynamoDB table '{TABLE_NAME}' in region '{REGION}'.")



# --- S3 Upload ---
print("\n== S3 File Upload ==")
# Creating a test file
with open('test_file.txt', 'w') as f:
    f.write('Hello from Graphbit integration demo!')

s3.upload_file('test_file.txt', BUCKET_NAME, 'test_file.txt')
print(f"Uploaded 'test_file.txt' to S3 bucket '{BUCKET_NAME}' as 'test_file.txt'.")



# --- S3 List ---
print("\n== S3 List Objects ==")
response = s3.list_objects_v2(Bucket=BUCKET_NAME)
for obj in response.get('Contents', []):
    print(" -", obj['Key'])



# Downloading and Reading File from S3
print("\n== Downloading and Reading File from S3 ==")
s3.download_file(BUCKET_NAME, 'test_file.txt', 'downloaded_file.txt')

with open('downloaded_file.txt', 'r') as f:
    content = f.read()
    print("Content of file from S3:")
    print(content)    



# Appending text in the file uploaded in s3 
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

s3.download_file(BUCKET_NAME, 'test_file.txt', 'downloaded_file.txt')
with open('downloaded_file.txt', 'r') as f:
    print("\nContent of file from S3 after update:")
    print(f.read())






# --- Graphbit Embedding Example ---

from graphbit import EmbeddingConfig as gb_ecg, EmbeddingClient as gb_ect

print("\n== Embedding with Graphbit ==")

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
if not OPENAI_API_KEY:
    raise ValueError("Please set the OPENAI_API_KEY environment variable.")

embedding_config = gb_ecg.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = gb_ect(embedding_config)


from decimal import Decimal

def float_list_to_decimal(lst):
    return [Decimal(str(x)) for x in lst]



# --- Single Embedding ---
# Extracting second line from the file uploaded in s3 for single embedding
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



# --- Batch Embedding ---
# Extracting lines 3 to 5 from the file uploaded in s3 for batch embedding
print("\n== Batch Embedding Example ==")
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
        "itemID": f"item-embedding-{i+3}",
        "embedding": float_list_to_decimal(emb),
        "metadata": {"text": text}
    }
    for i, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
]
with table.batch_writer() as batch:
    for item in batch_items:
        batch.put_item(Item=item)
print(f"Inserted {len(batch_items)} batch embeddings.")




# Vector search example
print("\n== Vector Search in DynamoDB ==")
query_embedding = embedding_client.embed("Find documents related to openai embedding models.")
scan_resp = table.scan()
items = scan_resp.get('Items', [])

best_score = -1
best_item = None
for item in items:
    if 'embedding' in item:
        score = gb_ect.similarity(query_embedding, item['embedding'])
        if score > best_score:
            best_score = score
            best_item = item
if best_item:
    print(f"Most similar itemID: {best_item['itemID']} (score: {best_score:.4f})")
else:
    print("No embeddings found in table.")


print("\n=== All operations complete! ===")
