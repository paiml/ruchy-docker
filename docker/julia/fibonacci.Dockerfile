# Multi-stage Dockerfile for Fibonacci benchmark (Julia)
# Target: Reasonable image size, Julia 1.10 JIT
# NOTE: EXPERIMENTAL - Julia included for comparison, JIT warmup affects startup time
#
# Stage 1: Base Julia slim image
# Stage 2: Runtime with only required files

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM julia:1.10-bullseye AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/fibonacci/main.jl .

# Make executable
RUN chmod +x main.jl

# ============================================================================
# Stage 2: Runtime
# ============================================================================
FROM julia:1.10-bullseye

WORKDIR /app

# Copy benchmark script
COPY --from=builder /build/main.jl /app/fibonacci.jl

# Run as non-root user for security
RUN useradd -m -u 1000 benchmark && \
    chown -R benchmark:benchmark /app

USER benchmark

# Set entrypoint
ENTRYPOINT ["julia", "/app/fibonacci.jl"]

# Metadata labels
LABEL org.opencontainers.image.title="Fibonacci Benchmark (Julia - EXPERIMENTAL)"
LABEL org.opencontainers.image.description="BENCH-007: Recursive Fibonacci fib(35) - JIT compilation"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="fibonacci"
LABEL benchmark.language="julia"
LABEL benchmark.expected_result="9227465"
LABEL benchmark.notes="EXPERIMENTAL: JIT warmup affects startup time measurement"
