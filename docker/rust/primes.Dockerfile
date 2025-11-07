# Multi-stage Dockerfile for Prime Sieve benchmark (Rust)
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
COPY benchmarks/primes/main.rs .

# Create minimal Cargo.toml for standalone binary
RUN echo '[package]\n\
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
panic = "abort"' > Cargo.toml

# Install musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Build with maximum optimizations
# Note: musl target produces fully static binaries by default
RUN cargo build --release --target x86_64-unknown-linux-musl

# ============================================================================
# Stage 2: Runtime (distroless)
# ============================================================================
FROM gcr.io/distroless/static-debian12:latest

# Copy statically-linked binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/primes /primes

# Set binary as entrypoint
ENTRYPOINT ["/primes"]

# Metadata labels
LABEL org.opencontainers.image.title="Prime Sieve Benchmark (Rust)"
LABEL org.opencontainers.image.description="BENCH-008: Sieve of Eratosthenes (100K primes)"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="primes"
LABEL benchmark.language="rust"
LABEL benchmark.expected_result="9592"
