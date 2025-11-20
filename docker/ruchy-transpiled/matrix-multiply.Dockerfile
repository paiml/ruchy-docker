# Multi-stage Dockerfile for Matrix Multiply benchmark (Ruchy transpiled to Rust + trueno)
# Target: <10MB image size, SIMD-accelerated via trueno
#
# Stage 1: Install Ruchy compiler
# Stage 2: Transpile Ruchy → Rust and compile with trueno
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

# Transpile and build (no dependencies - naive O(n³))
RUN ruchy transpile main.ruchy -o main.rs && \
    echo '[package]\n\
name = "matrix-multiply"\n\
version = "1.0.0"\n\
edition = "2021"\n\
\n\
[[bin]]\n\
name = "matrix-multiply"\n\
path = "main.rs"\n\
\n\
[profile.release]\n\
opt-level = 3\n\
lto = true\n\
codegen-units = 1\n\
strip = true\n\
panic = "abort"\n' > Cargo.toml && \
    rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y musl-tools && \
    rm -rf /var/lib/apt/lists/* && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/matrix-multiply || true

# ============================================================================
# Stage 3: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/matrix-multiply /matrix-multiply

ENTRYPOINT ["/matrix-multiply"]

LABEL org.opencontainers.image.title="Matrix Multiply (Ruchy Transpiled)"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="ruchy-transpiled"
LABEL benchmark.simd="false"
LABEL benchmark.algorithm="naive-O(n³)"
