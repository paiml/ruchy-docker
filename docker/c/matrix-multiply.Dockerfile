# Multi-stage Dockerfile for Matrix Multiply benchmark (C)
# Target: <10MB image size, Naive O(n³) baseline (no SIMD)
#
# Stage 1: Build stage (includes gcc, make, build tools)
# Stage 2: Runtime stage (FROM scratch, static binary only)

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM gcc:13-bookworm AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/matrix-multiply/main.c .

# Compile and verify in one layer
# -O3: Maximum optimization level
# -static: Static linking for FROM scratch compatibility
# -s: Strip debug symbols
# -Wall -Wextra: Enable all warnings
RUN gcc -O3 -static -s -Wall -Wextra -o matrix-multiply main.c && \
    ldd matrix-multiply 2>&1 | grep -q "not a dynamic executable" || \
    (echo "ERROR: Binary is not statically linked" && exit 1)

# ============================================================================
# Stage 2: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy statically-linked binary from builder
COPY --from=builder /build/matrix-multiply /matrix-multiply

# Set binary as entrypoint
ENTRYPOINT ["/matrix-multiply"]

# Metadata labels
LABEL org.opencontainers.image.title="Matrix Multiply Benchmark (C)"
LABEL org.opencontainers.image.description="BENCH-006: 128×128 matrix multiply, naive O(n³)"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="c"
LABEL benchmark.simd="false"
