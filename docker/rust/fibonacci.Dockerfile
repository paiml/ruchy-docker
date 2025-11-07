# Multi-stage Dockerfile for Fibonacci benchmark (Rust)
# Target: <10MB image size, <10ms startup time
#
# Stage 1: Build stage (includes rustc, cargo, build tools)
# Stage 2: Runtime stage (distroless, static binary only)

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM rust:1.83-slim AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/fibonacci/main.rs .

# Configure, install dependencies, and build in one layer
RUN echo '[package]\n\
name = "fibonacci"\n\
version = "0.1.0"\n\
edition = "2021"\n\
\n\
[[bin]]\n\
name = "fibonacci"\n\
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
    strip target/x86_64-unknown-linux-musl/release/fibonacci || true

# ============================================================================
# Stage 2: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy statically-linked binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/fibonacci /fibonacci

# Set binary as entrypoint
ENTRYPOINT ["/fibonacci"]

# Metadata labels
LABEL org.opencontainers.image.title="Fibonacci Benchmark (Rust)"
LABEL org.opencontainers.image.description="BENCH-007: Recursive Fibonacci fib(35)"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="fibonacci"
LABEL benchmark.language="rust"
LABEL benchmark.expected_result="9227465"
