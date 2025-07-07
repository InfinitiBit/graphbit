FROM python:3.11-slim

# Set environment variables
ENV PYTHONUNBUFFERED=1
ENV PYTHONDONTWRITEBYTECODE=1
ENV DEBIAN_FRONTEND=noninteractive
ENV RUST_LOG=info

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Poetry
RUN pip install --no-cache-dir poetry
RUN pip install --no-cache-dir maturin

# Configure Poetry
ENV POETRY_NO_INTERACTION=1 \
    POETRY_VENV_IN_PROJECT=1 \
    POETRY_CACHE_DIR=/tmp/poetry_cache

# Set working directory
WORKDIR /app

# Copy dependency files
COPY pyproject.toml poetry.lock* ./
COPY Cargo.toml ./
COPY src/ ./src/
COPY core/ ./core/
COPY python/ ./python/

# Install Python dependencies
ENV POETRY_HTTP_TIMEOUT=500 \
    POETRY_MAX_RETRIES=3
RUN poetry lock
RUN poetry install --with benchmarks

# Build Rust components
RUN cargo build --release
WORKDIR /app/python
RUN poetry run maturin develop --release
WORKDIR /app

# Copy benchmark files
COPY benchmarks/ ./benchmarks/
COPY examples/ ./examples/

# Create results and logs directories
RUN mkdir -p /app/benchmarks/results /app/benchmarks/logs

# Keep working directory at app root for Poetry
# WORKDIR /app/benchmarks

# Create entrypoint script
RUN echo '#!/bin/bash' > /app/entrypoint.sh && \
    echo 'set -e' >> /app/entrypoint.sh && \
    echo 'if [ -n "$OPENAI_API_KEY" ]; then' >> /app/entrypoint.sh && \
    echo '    export OPENAI_API_KEY="$OPENAI_API_KEY"' >> /app/entrypoint.sh && \
    echo 'fi' >> /app/entrypoint.sh && \
    echo 'if [ -n "$ANTHROPIC_API_KEY" ]; then' >> /app/entrypoint.sh && \
    echo '    export ANTHROPIC_API_KEY="$ANTHROPIC_API_KEY"' >> /app/entrypoint.sh && \
    echo 'fi' >> /app/entrypoint.sh && \
    echo 'if [ -n "$HUGGINGFACE_API_KEY" ]; then' >> /app/entrypoint.sh && \
    echo '    export HUGGINGFACE_API_KEY="$HUGGINGFACE_API_KEY"' >> /app/entrypoint.sh && \
    echo 'fi' >> /app/entrypoint.sh && \
    echo 'cd /app' >> /app/entrypoint.sh && \
    echo 'exec poetry run python benchmarks/"$@"' >> /app/entrypoint.sh && \
    chmod +x /app/entrypoint.sh

# Default command
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["run_benchmark.py", "--help"]
