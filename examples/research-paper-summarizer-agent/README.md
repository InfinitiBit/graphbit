# Research Paper Summarizer Agent

A powerful research paper analysis and Q&A system built with the **GraphBit framework**. This application automatically extracts, summarizes, and enables intelligent questioning of research papers using advanced AI capabilities.

## 🌟 Features

- **📄 PDF Processing**: Upload and automatically process research papers in PDF format
- **📝 Section-wise Summarization**: Intelligent extraction and summarization of paper sections (Abstract, Introduction, Methods, Results, etc.)
- **🔍 Semantic Search**: Advanced vector-based search through paper content using embeddings
- **💬 Interactive Q&A**: Ask natural language questions about the paper content
- **⚡ Caching System**: Smart caching for faster re-processing of previously analyzed papers
- **🎨 Modern UI**: Clean, responsive Streamlit interface with enhanced user experience
- **🔧 GraphBit Integration**: Built on GraphBit's powerful workflow automation framework

## 🏗️ Architecture

The application follows a modern microservices architecture powered by GraphBit's native document processing:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │    Backend      │    │   GraphBit      │
│   (Streamlit)   │◄──►│   (FastAPI)     │◄──►│   Framework     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌─────────────────┐
                       │  Vector Store   │
                       │    (FAISS)      │
                       └─────────────────┘
```

### Components

- **Frontend**: Streamlit web application for user interaction
- **Backend**: FastAPI server handling PDF processing and Q&A
- **GraphBit Framework**: Core AI workflow orchestration with native document processing
- **Vector Store**: FAISS-based semantic search and retrieval
- **Caching Layer**: Persistent storage for processed papers

### 🚀 GraphBit Document Processing Enhancements

This application leverages GraphBit's advanced document processing capabilities:

- **Native PDF Processing**: Uses GraphBit's `DocumentLoader` for robust PDF text extraction with formatting preservation
- **Intelligent Text Splitting**: Employs GraphBit's `TextSplitter` with recursive splitting strategy that:
  - Preserves semantic boundaries (paragraphs, sentences)
  - Maintains context across chunks with intelligent overlap
  - Adapts chunk sizes based on section types (methods, results, etc.)
  - Handles academic text structure optimally
- **Context-Aware Chunking**: Section-specific chunking strategies that preserve research paper structure
- **Enhanced Retrieval**: Better context preservation leads to more accurate question answering

## 🚀 Quick Start

### Prerequisites

- Python 3.10 or higher (3.11+ recommended)
- Rust (for building GraphBit Python bindings)
- Poetry (for dependency management)
- OpenAI API key
- Git

**Install Rust** (if not already installed):
```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows
# Download and run rustup-init.exe from https://rustup.rs/
```

**Install Poetry** (if not already installed):
```bash
pip install poetry
```

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/InfinitiBit/graphbit.git
   cd graphbit
   ```

2. **Install GraphBit and dependencies**:
   ```bash
   # Install Poetry if not already installed
   pip install poetry

   # Install GraphBit dependencies
   poetry install --no-root --extras faiss

   # Build GraphBit Python bindings
   cd python
   maturin develop
   cd ..

   # Install additional dependencies for the research paper summarizer
   poetry add fastapi==0.116.0 streamlit pydantic uvicorn python-dotenv requests python-multipart
   ```

3. **Set up environment variables**:
   ```bash
   # Navigate to the research paper summarizer directory
   cd examples/research-paper-summarizer-agent

   # Create a .env file in the project root
   echo "OPENAI_API_KEY=your_openai_api_key_here" > .env
   ```

### Running the Application

#### Using Poetry

1. Start the backend server:
   ```bash
   # In terminal 1 
   cd examples/research-paper-summarizer-agent/backend
   poetry run python -m uvicorn main:app --reload --host 0.0.0.0 --port 8000
   ```

2. Start the frontend application:
   ```bash
   # In terminal 2 - from the main graphbit directory
   cd examples/research-paper-summarizer-agent/frontend
   poetry run streamlit run app.py --server.port 8501 --server.address 0.0.0.0
   ```

**Note**: The startup scripts require GraphBit to be properly installed and available in your Python environment.

### Access the Application

- Frontend: http://localhost:8501
- Backend API: http://localhost:8000
- API Documentation: http://localhost:8000/docs

## 📖 Usage Guide

### 1. Upload a Research Paper

1. Open the application in your browser
2. Click "Choose a PDF file" and select your research paper
3. Wait for the processing to complete (this may take 1-2 minutes for the first time)
4. View the generated section-wise summaries

### 2. Explore Summaries

- **Tabbed View**: For papers with many sections, summaries are organized in tabs
- **Expandable Sections**: For shorter papers, use expandable sections
- **Section Order**: Summaries follow standard academic paper structure

### 3. Ask Questions

1. Scroll to the "Q&A about the Paper" section
2. Type your question in the text area
3. Click "Ask Question" to get an AI-generated answer
4. View conversation history and ask follow-up questions

### Example Questions

- "What is the main contribution of this paper?"
- "What methodology was used in this research?"
- "What are the key findings and results?"
- "What are the limitations of this study?"
- "How does this work compare to previous research?"

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
# Required
OPENAI_API_KEY=your_openai_api_key_here

# Optional (with defaults)
LLM_MODEL=gpt-3.5-turbo
EMBEDDING_MODEL=text-embedding-ada-002
CACHE_DIR=cache
LOG_DIR=logs
SERVER_HOST=localhost
SERVER_PORT=8000
```

### Advanced Configuration

Edit `backend/const.py` to customize:

- **LLM Settings**: Model, temperature, max tokens
- **Processing Parameters**: Chunk size, section length limits
- **Search Configuration**: Number of results, similarity thresholds
- **Caching Options**: Directory paths, retention policies

## 📁 Project Structure

```
research-paper-summarizer-agent/
├── README.md                 # This file
├── pyproject.toml            # Project dependencies
├── backend/                  # FastAPI backend
│   ├── main.py              # API endpoints
│   ├── paper_manager.py     # Core paper processing logic
│   ├── summarizer.py        # PDF processing and summarization
│   ├── faiss_store.py       # Vector storage and search
│   ├── const.py             # Configuration constants
│   └── utils/
│       └── caching.py       # Caching utilities
└── frontend/                # Streamlit frontend
    ├── app.py               # Main application
    └── test.py              # Testing utilities
```

## 🔌 API Reference

### Endpoints

- `POST /upload/` - Upload and process a PDF file
- `POST /ask/` - Ask a question about a processed paper
- `GET /sessions/` - List active sessions
- `GET /sessions/{session_id}/summaries/` - Get summaries for a session
- `DELETE /sessions/{session_id}/` - Clear a specific session
- `GET /stats/` - Get application statistics

### Example API Usage

```python
import requests

# Upload a PDF
with open("paper.pdf", "rb") as f:
    response = requests.post(
        "http://localhost:8000/upload/",
        files={"file": f}
    )
session_data = response.json()

# Ask a question
response = requests.post(
    "http://localhost:8000/ask/",
    data={
        "session_id": session_data["session_id"],
        "query": "What is the main contribution?"
    }
)
answer = response.json()["answer"]
```

## 🧪 Testing

Run the test suite:

```bash
# Backend tests
cd backend
python -m pytest

# Frontend tests
cd frontend
python test.py
```

## 🐛 Troubleshooting

### Common Issues

1. **"OPENAI_API_KEY not set"**
   - Ensure your `.env` file contains a valid OpenAI API key
   - Check that the `.env` file is in the `examples/research-paper-summarizer-agent` directory

2. **"Connection refused" errors**
   - Make sure the backend server is running on port 8000
   - Check firewall settings and port availability

3. **"ModuleNotFoundError: No module named 'graphbit'"**
   - Ensure GraphBit is properly installed and built using `maturin develop`
   - Make sure you're using the correct Python environment (Poetry virtual environment)
   - Try running commands with `poetry run` prefix

4. **"ModuleNotFoundError: No module named 'faiss'"**
   - Install the faiss extra: `poetry install --extras faiss`
   - Or install faiss-cpu directly: `poetry add faiss-cpu`

5. **"pip install -e ." fails**
   - Use the Poetry-based installation method instead
   - The package structure requires Poetry for proper dependency management

6. **Import errors with relative imports**
   - Run the backend from the main graphbit directory using the full module path
   - Use: `poetry run python -m uvicorn examples.research-paper-summarizer-agent.backend.main:app`

7. **PDF processing fails**
   - Ensure the PDF is not password-protected
   - Try with a smaller PDF file first
   - Check the logs in the `logs/` directory

8. **Slow processing**
   - Large PDFs take longer to process initially
   - Subsequent access to the same PDF will be faster due to caching
   - Consider using a more powerful OpenAI model for faster processing

9. **Unicode Character Boundary Errors**
   - If you see errors like "byte index X is not a char boundary", this is automatically handled
   - The application includes Unicode normalization for em dashes, smart quotes, and other special characters
   - Fallback mechanisms ensure processing continues even with problematic Unicode
   - Test Unicode handling: `poetry run python examples/research-paper-summarizer-agent/test_unicode_handling.py`

10. **GraphBit Text Splitter Issues**
    - The application automatically falls back to simple word-based chunking if GraphBit's splitter fails
    - Unicode characters are normalized before processing to prevent boundary issues
    - Check logs for "Warning: GraphBit splitter failed" messages indicating fallback usage

### Performance Optimization

- **Caching**: Processed papers are automatically cached for faster re-access
- **Chunking**: Large papers are split into optimal chunks for better search
- **Model Selection**: Use `gpt-4o-mini` for faster processing or `gpt-4` for higher quality

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Commit your changes: `git commit -am 'Add feature'`
5. Push to the branch: `git push origin feature-name`
6. Submit a pull request

## 📄 License

This project is part of the GraphBit framework and follows the same licensing terms.

## 🙏 Acknowledgments

- Built with [GraphBit](https://github.com/InfinitiBit/graphbit) framework
- Powered by [OpenAI](https://openai.com/) language models
- UI built with [Streamlit](https://streamlit.io/)
- Vector search using [FAISS](https://github.com/facebookresearch/faiss)

---

For more information about GraphBit framework, visit the [main repository](https://github.com/InfinitiBit/graphbit).
