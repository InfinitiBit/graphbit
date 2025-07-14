from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from chatbot_manager import ChatbotManager

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
        chatbot.create_index()
        return {"message": "Vector store index created successfully."}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/chat/")
def chat(req: ChatRequest):
    try:
        response = chatbot.chat(req.session_id, req.message)
        return {"response": response}
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))