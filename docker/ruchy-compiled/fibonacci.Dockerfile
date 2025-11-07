# Multi-stage Dockerfile for Fibonacci benchmark (Ruchy native compilation)
# Target: <10MB image size, <10ms startup time
#
# Stage 1: Install Ruchy compiler from local development directory
# Stage 2: Compile Ruchy natively
# Stage 3: Runtime stage (distroless, static binary only)

# ============================================================================
# Stage 1: Install Ruchy Compiler
# ============================================================================
FROM rustlang/rust:nightly-slim AS ruchy-installer

WORKDIR /ruchy

# Copy Ruchy compiler source from parent directory
# Note: Docker build context must include parent dir: docker build -f ... -t ... ..
COPY ruchy /ruchy

# Install build dependencies and Ruchy compiler in one layer
RUN \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install --path .

# ============================================================================
# Stage 2: Builder
# ============================================================================
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /build

# Copy Ruchy compiler binary from installer stage
COPY --from=ruchy-installer /usr/local/cargo/bin/ruchy /usr/local/bin/ruchy

# Copy benchmark source (from ruchy-docker subdirectory in parent context)
COPY ruchy-docker/benchmarks/fibonacci/main.ruchy .

# Compile Ruchy natively with NASA optimization (314 KB binary, default for Docker builds)
# --optimize nasa: LTO, single codegen unit, strip symbols, native CPU (12.2× size reduction)
RUN \
    ruchy compile main.ruchy --optimize nasa --output fibonacci

# ============================================================================
# Stage 3: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy statically-linked binary from builder
COPY --from=builder /build/fibonacci /fibonacci

# Set binary as entrypoint
ENTRYPOINT ["/fibonacci"]

# Metadata labels
LABEL org.opencontainers.image.title="Fibonacci Benchmark (Ruchy Compiled)"
LABEL org.opencontainers.image.description="BENCH-007: Recursive Fibonacci fib(35) - Ruchy native"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="fibonacci"
LABEL benchmark.language="ruchy-compiled"
LABEL benchmark.expected_result="9227465"
