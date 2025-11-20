# Multi-stage Dockerfile for Matrix Multiply benchmark (Ruchy native compilation + trueno)
# Target: <10MB image size, SIMD-accelerated via trueno
#
# Stage 1: Install Ruchy compiler
# Stage 2: Compile Ruchy natively with NASA optimization
# Stage 3: Runtime stage (FROM scratch, static binary only)

# ============================================================================
# Stage 1: Install Ruchy Compiler
# ============================================================================
FROM rustlang/rust:nightly-slim AS ruchy-installer

WORKDIR /ruchy
COPY ruchy /ruchy

# Install build dependencies and Ruchy compiler
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install --path .

# ============================================================================
# Stage 2: Builder
# ============================================================================
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /build

# Copy Ruchy compiler binary
COPY --from=ruchy-installer /usr/local/cargo/bin/ruchy /usr/local/bin/ruchy

# Copy benchmark source
COPY ruchy-docker/benchmarks/matrix-multiply/main.ruchy .

# Compile Ruchy natively with NASA optimization (naive O(n³), no SIMD)
RUN ruchy compile main.ruchy --optimize nasa --output matrix-multiply

# ============================================================================
# Stage 3: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

COPY --from=builder /build/matrix-multiply /matrix-multiply

ENTRYPOINT ["/matrix-multiply"]

LABEL org.opencontainers.image.title="Matrix Multiply (Ruchy Compiled)"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="ruchy-compiled"
LABEL benchmark.simd="false"
LABEL benchmark.algorithm="naive-O(n³)"
