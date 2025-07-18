# GraphBit Chatbot Example

A chatbot using GraphBit with vector database storage and Streamlit interface.

## Quick Start

### 1. Prerequisites
- Python 3.11+
- GraphBit installed
- OpenAI API key

### 2. Installation

```bash
cd examples/chatbot
poetry install
```

### 3. Configuration

```bash
export OPENAI_API_KEY="api_key"
```

### 4. Run the Application

**Terminal 1 - Backend:**
```bash
poetry run uvicorn backend.main:app --reload
```

**Terminal 2 - Frontend:**
```bash
poetry run streamlit run frontend/chatbot.py
```

### 5. Access

- Backend API: http://localhost:8000
- Frontend UI: http://localhost:8501

The chatbot will automatically initialize the vector database on first use.
