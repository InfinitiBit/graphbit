"""
FastAPI backend server for GraphBit research paper summarizer application.

This module provides REST API endpoints for PDF upload, processing, and Q&A
interactions using GraphBit's workflow system.
"""

from fastapi import FastAPI, UploadFile, File, Form, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import os
import shutil

from .paper_manager import PaperManager


app = FastAPI(title="Research Paper Summarizer API", version="1.0.0")

# Configure CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize paper manager
paper_manager = PaperManager()


class QuestionRequest(BaseModel):
    """Request model for question API endpoints."""
    session_id: str
    query: str

@app.get("/")
def root():
    """Root endpoint that returns a welcome message."""
    return {"message": "Welcome to the Research Paper Summarizer API powered by GraphBit!"}


@app.post("/upload/")
async def upload_pdf(file: UploadFile = File(...)):
    """
    Upload and process a PDF file for summarization.

    Args:
        file (UploadFile): The PDF file to process.

    Returns:
        dict: Response containing session ID and summaries.

    Raises:
        HTTPException: If there's an error processing the file.
    """
    try:
        # Validate file type
        if not file.filename.lower().endswith('.pdf'):
            raise HTTPException(status_code=400, detail="Only PDF files are supported")

        # Save uploaded file temporarily
        pdf_path = f"temp_{file.filename}"
        with open(pdf_path, "wb") as buffer:
            shutil.copyfileobj(file.file, buffer)

        try:
            # Process the PDF using PaperManager
            session_id, summaries = paper_manager.process_pdf(pdf_path)

            return {"session_id": session_id, "summaries": summaries}

        finally:
            # Clean up temporary file
            if os.path.exists(pdf_path):
                os.remove(pdf_path)

    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error processing PDF: {str(e)}")


@app.post("/ask/")
async def ask_question(session_id: str = Form(...), query: str = Form(...)):
    """
    Ask a question about a processed research paper.

    Args:
        session_id (str): Session ID of the processed paper.
        query (str): User's question.

    Returns:
        dict: Response containing the answer.

    Raises:
        HTTPException: If there's an error processing the question.
    """
    try:
        answer = paper_manager.ask_question(session_id, query)
        return {"answer": answer}

    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error processing question: {str(e)}")


@app.get("/sessions/")
async def list_sessions():
    """
    List all active sessions.

    Returns:
        dict: Response containing list of session IDs.
    """
    sessions = paper_manager.list_sessions()
    return {"sessions": sessions}


@app.get("/sessions/{session_id}/summaries/")
async def get_session_summaries(session_id: str):
    """
    Get summaries for a specific session.

    Args:
        session_id (str): Session ID.

    Returns:
        dict: Response containing summaries.

    Raises:
        HTTPException: If session not found.
    """
    summaries = paper_manager.get_session_summaries(session_id)
    if summaries is None:
        raise HTTPException(status_code=404, detail="Session not found")
    return {"summaries": summaries}


@app.delete("/sessions/{session_id}/")
async def clear_session(session_id: str):
    """
    Clear a specific session.

    Args:
        session_id (str): Session ID to clear.

    Returns:
        dict: Response indicating success.

    Raises:
        HTTPException: If session not found.
    """
    success = paper_manager.clear_session(session_id)
    if not success:
        raise HTTPException(status_code=404, detail="Session not found")
    return {"message": f"Session {session_id} cleared successfully"}


@app.get("/stats/")
async def get_stats():
    """
    Get statistics about the paper manager.

    Returns:
        dict: Response containing statistics.
    """
    stats = paper_manager.get_stats()
    return {"stats": stats}
