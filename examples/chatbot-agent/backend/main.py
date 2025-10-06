"""
FastAPI backend server for GraphBit chatbot application.

This module provides REST API endpoints and WebSocket connections for
chatbot interactions, including vector store management and real-time chat.
"""

import json

from fastapi import FastAPI, HTTPException, WebSocket, WebSocketDisconnect
from pydantic import BaseModel

from .chatbot_manager import ChatbotManager

app = FastAPI()
chatbot = ChatbotManager()


class ChatRequest(BaseModel):
    """Request model for chat API endpoints."""

    message: str
    session_id: str


@app.get("/")
def root():
    """Root endpoint that returns a welcome message."""
    return {"message": "Welcome to the Chatbot API!"}


@app.post("/index/")
def create_index():
    """Endpoint to trigger the creation of the vector store index."""
    try:
        chatbot._create_index()
        return {"message": "Vector store index created successfully."}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/chat/")
async def chat(req: ChatRequest):
    """
    Process a chat message and return a response.

    Args:
        req (ChatRequest): The chat request containing message and session ID.

    Returns:
        dict: Response containing the chatbot's reply.

    Raises:
        HTTPException: If there's an error processing the request.
    """
    try:
        response = await chatbot.chat(req.session_id, req.message)
        return {"response": response}
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.websocket("/ws/chat/")
async def websocket_chat(websocket: WebSocket):
    """
    Handle WebSocket endpoint for real-time chat communication.

    Args:
        websocket (WebSocket): The WebSocket connection for bidirectional communication.

    Handles:
        - Accepting WebSocket connections
        - Processing incoming chat messages
        - Sending responses back to the client
        - Error handling and connection management
    """
    await websocket.accept()
    try:
        while True:

            data = await websocket.receive_text()
            message_data = json.loads(data)

            message = message_data.get("message")
            session_id = message_data.get("session_id")

            if not message or not session_id:
                await websocket.send_text(json.dumps({"error": "Missing message or session_id"}))
                continue

            await chatbot.chat(websocket, session_id, message)

            await websocket.send_text(json.dumps({"response": "", "session_id": session_id, "type": "end"}))

    except WebSocketDisconnect:
        print("Client disconnected")
    except json.JSONDecodeError:
        await websocket.send_text(json.dumps({"error": "Invalid JSON format"}))

    except Exception as e:
        await websocket.send_text(json.dumps({"error": str(e)}))
