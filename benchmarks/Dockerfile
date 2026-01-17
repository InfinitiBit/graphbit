# syntax=docker/dockerfile:1
FROM python:3.12-slim

# Install system dependencies
# - curl, ca-certificates: for downloading tools/security
# - build-essential: for compiling C extensions (needed for psutil, etc.)
# - git: for git-based dependencies
# - procps: provides 'ps' command needed by some benchmarks
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    build-essential \
    git \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Install uv for fast Python package management
RUN pip install --no-cache-dir uv

WORKDIR /app/benchmarks

# Copy pyproject.toml and lockfile
COPY pyproject.toml uv.lock ./

# Install dependencies from PyPI
# UV will fetch graphbit>=0.6.0 from PyPI as a binary wheel
# --no-cache-dir reduces image size
# -v enables verbose output to debug resolution issues
RUN uv venv && \
    . .venv/bin/activate && \
    uv pip install --no-cache-dir -r pyproject.toml

# Copy all benchmark source files
# This includes .env if present (and not in .dockerignore)
COPY . ./

# Create results and logs directories
RUN mkdir -p results logs

# Set entrypoint script
RUN echo '#!/bin/bash' > /entrypoint.sh && \
    echo 'set -e' >> /entrypoint.sh && \
    echo '. /app/benchmarks/.venv/bin/activate' >> /entrypoint.sh && \
    echo 'exec "$@"' >> /entrypoint.sh && \
    chmod +x /entrypoint.sh

# Verify installation of key packages
RUN . .venv/bin/activate && python -c "import graphbit; print(f'GraphBit version: {graphbit.__version__}')"

WORKDIR /app/benchmarks
ENTRYPOINT ["/entrypoint.sh"]
CMD ["python", "run_benchmark.py"]
