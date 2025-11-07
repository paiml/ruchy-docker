# Multi-stage Dockerfile for Prime Sieve benchmark (Ruchy transpiled to Rust)
# Target: <10MB image size, <10ms startup time
#
# Stage 1: Install Ruchy compiler from local development directory
# Stage 2: Transpile Ruchy → Rust and compile
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
COPY ruchy-docker/benchmarks/primes/main.ruchy .

# Transpile, configure, and build in one layer
RUN \
    ruchy transpile main.ruchy -o main.rs && \
    echo '[package]\n\
name = "primes"\n\
version = "1.0.0"\n\
edition = "2021"\n\
\n\
[[bin]]\n\
name = "primes"\n\
path = "main.rs"\n\
\n\
[profile.release]\n\
opt-level = 3\n\
lto = true\n\
codegen-units = 1\n\
strip = true\n\
panic = "abort"' > Cargo.toml && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# ============================================================================
# Stage 3: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy statically-linked binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/primes /primes

# Set binary as entrypoint
ENTRYPOINT ["/primes"]

# Metadata labels
LABEL org.opencontainers.image.title="Prime Sieve Benchmark (Ruchy Transpiled)"
LABEL org.opencontainers.image.description="BENCH-008: Recursive Prime Sieve 100K primes - Ruchy→Rust"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="primes"
LABEL benchmark.language="ruchy-transpiled"
LABEL benchmark.expected_result="9592"
