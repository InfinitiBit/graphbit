# GraphBit Chatbot

An intelligent conversational AI chatbot built with the **GraphBit framework**. This application provides real-time chat capabilities with context-aware responses powered by vector database memory and advanced AI.

## 🌟 Features

- **💬 Real-time Chat**: WebSocket-based streaming responses for instant interaction
- **🧠 Context-Aware Memory**: ChromaDB vector database for intelligent conversation context retrieval
- **📚 Knowledge Base**: Automatic indexing and retrieval of relevant information
- **🔄 Session Management**: Persistent conversation history across sessions
- **⚡ Streaming Responses**: Token-by-token response streaming for better user experience
- **🎨 Modern UI**: Clean, responsive Streamlit interface with emoji avatars
- **🔧 GraphBit Integration**: Built on GraphBit's powerful LLM and embedding clients

## 🏗️ Architecture

The application follows a modern microservices architecture powered by GraphBit's native AI capabilities:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │    Backend      │    │   GraphBit      │
│   (Streamlit)   │◄──►│   (FastAPI)     │◄──►│   Framework     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌─────────────────┐
                       │  Vector Store   │
                       │   (ChromaDB)    │
                       └─────────────────┘
```

### Components

- **Frontend**: Streamlit web application with WebSocket connectivity for real-time chat
- **Backend**: FastAPI server handling chat logic, context retrieval, and conversation storage
- **GraphBit Framework**: Core AI orchestration with native LLM and embedding clients
- **Vector Store**: ChromaDB-based semantic search and conversation memory
- **Session Management**: In-memory session storage with persistent vector database backup

### 🚀 GraphBit AI Integration

This application leverages GraphBit's advanced AI capabilities:

- **Native LLM Client**: Uses GraphBit's `LlmClient` with OpenAI GPT-3.5-turbo for response generation
- **Streaming Support**: Employs GraphBit's `complete_stream` for real-time token-by-token responses
- **Embedding Generation**: Utilizes GraphBit's `EmbeddingClient` with OpenAI's text-embedding-3-small model
- **Batch Embeddings**: Efficient `embed_many` for processing multiple text chunks simultaneously
- **Context-Aware Responses**: Combines vector similarity search with conversation history for intelligent replies

## 🚀 Setup Instructions

### Prerequisites
- Python 3.11+
- OpenAI API key
- [Poetry installed](https://python-poetry.org/docs/#installation)

### Installation

1. **Clone the repository** (to access the example):
   ```bash
   git clone https://github.com/InfinitiBit/graphbit.git
   cd graphbit/examples/chatbot
   ```

2. **Install dependencies**:
   ```bash
   poetry install
   ```

   This will automatically install GraphBit and all other required dependencies listed in `pyproject.toml`.

3. **Install GraphBit Tracer** (for tracing and observability):
   ```bash
   pip install graphbit-tracer
   ```

4. **Set up environment variables**:
   ```bash
   # Set your OpenAI API key
   export OPENAI_API_KEY="your_api_key_here"

   # Set GraphBit Tracer environment variables (required for tracing)
   export GRAPHBIT_TRACING_API_KEY="your_graphbit_api_key_here"
   export GRAPHBIT_TRACEABLE_PROJECT="Chatbot"

   # Optional: Custom API endpoint (defaults to production endpoint)
   export GRAPHBIT_TRACING_API_URL="https://platform-api.graphbit.ai/api/v1/tracing/single-run"
   ```

   > **Note**: The `GRAPHBIT_TRACING_API_KEY` and `GRAPHBIT_TRACEABLE_PROJECT` environment variables are required for the tracer to function properly. If you don't have a GraphBit API key yet, you can obtain one from the GraphBit platform.

### Running the Application

#### Using Poetry

1. Start the backend server:
   ```bash
   poetry run uvicorn backend.main:app --reload --host 0.0.0.0 --port 8000
   ```

2. Start the frontend application:
   ```bash
   poetry run streamlit run frontend/chatbot.py --server.port 8501 --server.address 0.0.0.0
   ```

**Note**: The application will automatically initialize the vector database on first use.

### Access the Application

- Frontend: http://localhost:8501
- Backend API: http://localhost:8000
- API Documentation: http://localhost:8000/docs

## 📖 Usage Guide

### 1. Initial Setup

1. Open the application in your browser at http://localhost:8501
2. Wait for the automatic knowledge base initialization (happens once on first launch)
3. See the welcome message from the AI assistant

### 2. Start Chatting

1. Type your message in the chat input at the bottom of the page
2. Press Enter or click the send button
3. Watch as the AI assistant streams its response in real-time
4. Continue the conversation naturally

### 3. How It Works

- **Context Retrieval**: Each message triggers a semantic search in the vector database to find relevant context
- **Response Generation**: The AI combines retrieved context with your question to generate informed responses
- **Memory Storage**: Every conversation exchange is automatically saved to the vector database for future reference
- **Session Tracking**: Your conversation history is maintained throughout the session

## 📁 Project Structure

```
chatbot/
├── README.md                    # This file
├── pyproject.toml               # Project dependencies
├── backend/                     # FastAPI backend
│   ├── main.py                 # API endpoints and WebSocket handler
│   ├── chatbot_manager.py      # Core chatbot orchestration logic
│   ├── vectordb_manager.py     # ChromaDB vector store management
│   ├── llm_manager.py          # GraphBit LLM and embedding client wrapper
│   ├── const.py                # Configuration constants
│   └── data/
│       └── vectordb.txt        # Initial knowledge base content
├── frontend/                    # Streamlit frontend
│   └── chatbot.py              # Main chat interface application
├── logs/                        # Application logs
│   └── chatbot.log             # Runtime logs
└── vector_index_chatbot/        # ChromaDB persistent storage
    └── chroma.sqlite3          # Vector database file
```

## 🔌 API Reference

### Endpoints

- `GET /` - Root endpoint returning welcome message
- `POST /index/` - Create or recreate the vector store index from the knowledge base file
- `POST /chat/` - Send a chat message and receive a response (non-streaming)
- `WebSocket /ws/chat/` - Real-time chat with streaming responses

### WebSocket Protocol

**Client Message Format**:
```json
{
  "message": "Your question here",
  "session_id": "unique-session-id"
}
```

**Server Response Format**:
```json
{
  "response": "token or full response",
  "session_id": "unique-session-id",
  "type": "chunk|end"
}
```

## ⚙️ Configuration

Configuration constants are defined in `backend/const.py`:

- `OPENAI_LLM_MODEL`: GPT model for chat responses (default: gpt-3.5-turbo)
- `OPENAI_EMBEDDING_MODEL`: Embedding model for vector search (default: text-embedding-3-small)
- `CHUNK_SIZE`: Text chunk size for vector indexing (default: 1000)
- `OVERLAP_SIZE`: Overlap between chunks (default: 100)
- `RETRIEVE_CONTEXT_N_RESULTS`: Number of similar documents to retrieve (default: 5)
- `MAX_TOKENS`: Maximum tokens in LLM response (default: 200)
- `COLLECTION_NAME`: ChromaDB collection name (default: chatbot_memory)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Commit your changes: `git commit -am 'Add feature'`
5. Push to the branch: `git push origin feature-name`
6. Submit a pull request

## 📄 License

This project is part of the GraphBit framework and follows the same licensing terms.

---

For more information about GraphBit framework, visit the [main repository](https://github.com/InfinitiBit/graphbit).
