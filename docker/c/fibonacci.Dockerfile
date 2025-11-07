# Multi-stage Dockerfile for Fibonacci benchmark (C)
# Target: <10MB image size, <10ms startup time
#
# Stage 1: Build stage (includes gcc, make, build tools)
# Stage 2: Runtime stage (distroless, static binary only)

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM gcc:13-bookworm AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/fibonacci/main.c .

# Compile and verify in one layer
# -O3: Maximum optimization level
# -static: Static linking for distroless compatibility
# -s: Strip debug symbols
# -Wall -Wextra: Enable all warnings
RUN gcc -O3 -static -s -Wall -Wextra -o fibonacci main.c && \
    ldd fibonacci 2>&1 | grep -q "not a dynamic executable" || \
    (echo "ERROR: Binary is not statically linked" && exit 1)

# ============================================================================
# Stage 2: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy statically-linked binary from builder
COPY --from=builder /build/fibonacci /fibonacci

# Set binary as entrypoint
ENTRYPOINT ["/fibonacci"]

# Metadata labels
LABEL org.opencontainers.image.title="Fibonacci Benchmark (C)"
LABEL org.opencontainers.image.description="BENCH-007: Recursive Fibonacci fib(35)"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="fibonacci"
LABEL benchmark.language="c"
LABEL benchmark.expected_result="9227465"
