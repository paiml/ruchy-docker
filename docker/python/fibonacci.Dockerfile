# Multi-stage Dockerfile for Fibonacci benchmark (Python)
# Target: Reasonable image size, CPython 3.12
#
# Stage 1: Base Python slim image
# Stage 2: Runtime with only required files

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM python:3.12-slim AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/fibonacci/main.py .

# Make executable
RUN \
    chmod +x main.py

# ============================================================================
# Stage 2: Runtime
# ============================================================================
FROM python:3.12-slim

WORKDIR /app

# Copy benchmark script
COPY --from=builder /build/main.py /app/fibonacci.py

# Run as non-root user for security
RUN \
    useradd -m -u 1000 benchmark && \
    chown -R benchmark:benchmark /app

USER benchmark

# Set entrypoint
ENTRYPOINT ["python3", "/app/fibonacci.py"]

# Metadata labels
LABEL org.opencontainers.image.title="Fibonacci Benchmark (Python)"
LABEL org.opencontainers.image.description="BENCH-007: Recursive Fibonacci fib(35)"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="fibonacci"
LABEL benchmark.language="python"
LABEL benchmark.expected_result="9227465"
