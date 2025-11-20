# Multi-stage Dockerfile for Matrix Multiply benchmark (Rust + trueno)
# Target: <10MB image size, SIMD-accelerated via trueno
#
# Stage 1: Build stage (includes rustc, cargo, build tools)
# Stage 2: Runtime stage (FROM scratch, static binary only)

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /build

# Copy trueno source from parent directory (local development version)
COPY trueno /trueno

# Copy benchmark source
COPY ruchy-docker/benchmarks/matrix-multiply/main.rs .

# Configure with local trueno dependency and build
RUN echo '[package]\n\
name = "matrix-multiply"\n\
version = "0.1.0"\n\
edition = "2021"\n\
\n\
[dependencies]\n\
trueno = { path = "/trueno" }\n\
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
    RUSTFLAGS="-C target-cpu=native" cargo build --release && \
    strip target/release/matrix-multiply || true

# ============================================================================
# Stage 2: Runtime (gcr.io/distroless for glibc)
# ============================================================================
FROM gcr.io/distroless/cc-debian12:latest

# Copy dynamically-linked binary from builder
COPY --from=builder /build/target/release/matrix-multiply /matrix-multiply

# Set binary as entrypoint
ENTRYPOINT ["/matrix-multiply"]

# Metadata labels
LABEL org.opencontainers.image.title="Matrix Multiply Benchmark (Rust + trueno)"
LABEL org.opencontainers.image.description="BENCH-006: 128×128 matrix multiplication with SIMD"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="rust"
LABEL benchmark.simd="true"
LABEL benchmark.library="trueno-0.4.0"
