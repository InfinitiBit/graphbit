"""This is where the fast api works in backend."""

import os
import shutil
from collections import defaultdict

from fastapi import FastAPI, File, Form, UploadFile
from fastapi.middleware.cors import CORSMiddleware

from .caching import hash_pdf, load_from_cache, save_to_cache
from .faiss_store import create_faiss_index, embed_chunks, search_faiss_index
from .summarizer import answer_question, chunk_text, summarize_pdf_sections

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

SESSION_DATA = {}

CACHE_DIR = "cache"


@app.post("/upload/")
async def upload_pdf(file: UploadFile = File(...)):
    """Upload a pdf file and summarize it."""
    if not os.path.exists(CACHE_DIR):
        os.makedirs(CACHE_DIR)

    pdf_path = f"temp_{file.filename}"
    with open(pdf_path, "wb") as buffer:
        shutil.copyfileobj(file.file, buffer)

    # Hash the PDF and check cache
    hash_id = hash_pdf(pdf_path)
    cache_folder = os.path.join(CACHE_DIR, hash_id)

    if os.path.exists(cache_folder):
        summaries, chunk_dict, chunk_titles, index = load_from_cache(hash_id)
    else:
        summaries, sections = summarize_pdf_sections(pdf_path)
        chunk_dict = defaultdict(list)
        for section, content in sections.items():
            for chunk in chunk_text(content):
                chunk_dict[section].append(chunk)
        chunk_titles, chunk_vectors = embed_chunks(chunk_dict)
        index = create_faiss_index(chunk_vectors)
        save_to_cache(hash_id, summaries, chunk_dict, chunk_titles, index)

    os.remove(pdf_path)
    SESSION_DATA[hash_id] = {
        "summaries": summaries,
        "chunk_dict": chunk_dict,
        "chunk_titles": chunk_titles,
        "index": index,
    }
    return {"session_id": hash_id, "summaries": summaries}


@app.post("/ask/")
async def ask_question(session_id: str = Form(...), query: str = Form(...)):
    """Ask a question about the pdf file."""
    data = SESSION_DATA[session_id]
    results = search_faiss_index(data["index"], query, data["chunk_titles"], data["chunk_dict"])
    # Combine text from top 4 chunks
    context = "\n\n".join([chunk for _, chunk in results])
    answer = answer_question(context, query)
    return {"answer": answer}
