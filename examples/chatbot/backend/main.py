from fastapi import FastAPI, HTTPException, WebSocket, WebSocketDisconnect
from pydantic import BaseModel
from .chatbot_manager import ChatbotManager

import json

app = FastAPI()
chatbot = ChatbotManager()

class ChatRequest(BaseModel):
    message: str
    session_id: str

@app.get("/")
def root():
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
    try:
        response = await chatbot.chat(req.session_id, req.message)
        return {"response": response}
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
    


@app.websocket("/ws/chat/")

async def websocket_chat(websocket: WebSocket):

    await websocket.accept()
    try:
        while True:

            data = await websocket.receive_text()
            message_data = json.loads(data)

            message = message_data.get("message")
            session_id = message_data.get("session_id")

            if not message or not session_id:
                await websocket.send_text(json.dumps({
                    "error": "Missing message or session_id"
                }))
                continue

            response = await chatbot.chat(session_id, message)

            await websocket.send_text(json.dumps({
                "response": response,
                "session_id": session_id
            }))

    except WebSocketDisconnect:
        print("Client disconnected")
    except json.JSONDecodeError:
        await websocket.send_text(json.dumps({
            "error": "Invalid JSON format"
        }))

    except Exception as e:
        await websocket.send_text(json.dumps({
            "error": str(e)
        }))