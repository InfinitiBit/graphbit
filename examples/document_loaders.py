import os
import graphbit

# Initialize GraphBit
print("Initializing GraphBit...")
graphbit.init()

# Create a document loader with default configuration
print("\nCreating default DocumentLoader...")
loader = graphbit.DocumentLoader()

print("\nCreated default DocumentLoader...")

# Custom configuration (optional)
from graphbit import DocumentLoaderConfig
config = DocumentLoaderConfig(
    max_file_size=5 * 1024 * 1024,  # 5MB
    default_encoding="utf-8",
    preserve_formatting=True,
    extraction_settings={}
)
custom_loader = graphbit.DocumentLoader(config)

# List of supported document types and example files
files = [
    ("examples/data/sample.txt", "txt"),
    ("examples/data/ex1.pdf", "pdf"),
    ("examples/data/sample.docx", "docx"),
    ("examples/data/sample.json", "json"),
    ("examples/data/sample.csv", "csv"),
    ("examples/data/sample.xml", "xml"),
    ("examples/data/sample.html", "html"),
]

print("\n--- Document Loader Test ---\n")
for path, doc_type in files:
    if not os.path.exists(path):
        print(f"[WARN] File not found for {doc_type.upper()}: {path}")
        continue
    try:
        doc = loader.load_document(path, document_type=doc_type)
        print(f"[OK] Loaded {doc_type.upper()} from {path}")
        print(f"  Type: {doc.document_type}")
        print(f"  File size: {doc.file_size} bytes")
        print(f"  Content (first 100 chars): {doc.content[:100]}\n")
    except Exception as e:
        print(f"[ERROR] Failed to load {doc_type.upper()} from {path}: {e}")

print("\n--- Custom Config Loader Test (TXT only) ---\n")
txt_path = "examples/data/sample.txt"
if os.path.exists(txt_path):
    try:
        doc = custom_loader.load_document(txt_path, document_type="txt")
        print(f"[OK] Loaded TXT with custom config: {txt_path}")
        print(f"  File size: {doc.file_size} bytes")
        print(f"  Content (first 100 chars): {doc.content[:100]}\n")
    except Exception as e:
        print(f"[ERROR] Custom config loader failed: {e}")
else:
    print(f"[WARN] TXT file for custom config not found: {txt_path}")

print("\nAll document loader tests complete.")