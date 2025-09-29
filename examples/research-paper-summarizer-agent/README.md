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
   poetry add fastapi==0.116.0 streamlit pydantic uvicorn python-dotenv requests python-multipart faiss
   ```

3. **Set up environment variables**:
   ```bash
   # Create a .env file using the example file
   cp examples/research-paper-summarizer-agent/.env.example examples/research-paper-summarizer-agent/.env
   ```

### Running the Application

#### Using Poetry

1. Start the backend server:
   ```bash
   # In terminal 1 - from the main graphbit directory
   poetry run uvicorn examples.research-paper-summarizer-agent.backend.main-server:app --reload --host 0.0.0.0 --port 8000
   ```

2. Start the frontend application:
   ```bash
   # In terminal 2 - from the main graphbit directory
   poetry run streamlit run examples/research-paper-summarizer-agent/frontend/app.py --server.port 8501 --server.address 0.0.0.0
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
