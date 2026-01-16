FROM python:3.12-slim

# Set environment variables
ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUTF8=1 \
    DEBIAN_FRONTEND=noninteractive \
    RUST_LOG=info

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install UV (modern Python package installer)
RUN pip install --no-cache-dir uv

# Set working directory
WORKDIR /app/benchmarks

# Copy pyproject.toml (build context is benchmarks/, so no prefix needed)
COPY pyproject.toml ./

# Install dependencies from PyPI
# UV will fetch graphbit>=0.6.0 from PyPI as a binary wheel
RUN uv venv && \
    . .venv/bin/activate && \
    uv pip install graphbit>=0.6.0 && \
    uv sync --no-dev

# Copy all benchmark source files
COPY . ./

# Create results and logs directories
RUN mkdir -p results logs

# Create entrypoint script
RUN echo '#!/bin/bash' > /entrypoint.sh && \
    echo 'set -e' >> /entrypoint.sh && \
    echo 'cd /app/benchmarks' >> /entrypoint.sh && \
    echo '. .venv/bin/activate' >> /entrypoint.sh && \
    echo '# If first arg is a .py file or benchmark script, prepend python' >> /entrypoint.sh && \
    echo 'if [[ "$1" == *.py ]] || [[ "$1" == run_benchmark.py ]]; then' >> /entrypoint.sh && \
    echo '  exec python "$@"' >> /entrypoint.sh && \
    echo 'else' >> /entrypoint.sh && \
    echo '  exec "$@"' >> /entrypoint.sh && \
    echo 'fi' >> /entrypoint.sh && \
    chmod +x /entrypoint.sh

# Health check - verify all frameworks import successfully
RUN . .venv/bin/activate && python -c "\
import graphbit; print(f'✓ GraphBit {graphbit.__version__}'); \
import langchain; print('✓ LangChain'); \
import langgraph; print('✓ LangGraph'); \
import autogen_agentchat; print('✓ AutoGen'); \
import crewai; print('✓ CrewAI'); \
import pydantic_ai; print('✓ PydanticAI'); \
import llama_index; print('✓ LlamaIndex'); \
print('All 7 frameworks OK!')"

WORKDIR /app/benchmarks

# Default command
ENTRYPOINT ["/entrypoint.sh"]
CMD ["run_benchmark.py", "--help"]
