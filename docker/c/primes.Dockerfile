# Multi-stage Dockerfile for Prime Sieve benchmark (C)
# Target: <10MB image size, <10ms startup time
#
# Stage 1: Build stage (includes gcc, make, build tools)
# Stage 2: Runtime stage (scratch, static binary only)

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM gcc:13-bookworm AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/primes/main.c .

# Compile and verify in one layer
# -O3: Maximum optimization level
# -static: Static linking for scratch compatibility
# -s: Strip debug symbols
# -Wall -Wextra: Enable all warnings
RUN gcc -O3 -static -s -Wall -Wextra -o primes main.c && \
    ldd primes 2>&1 | grep -q "not a dynamic executable" || \
    (echo "ERROR: Binary is not statically linked" && exit 1)

# ============================================================================
# Stage 2: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy statically-linked binary from builder
COPY --from=builder /build/primes /primes

# Set binary as entrypoint
ENTRYPOINT ["/primes"]

# Metadata labels
LABEL org.opencontainers.image.title="Prime Sieve Benchmark (C)"
LABEL org.opencontainers.image.description="BENCH-008: Sieve of Eratosthenes (100K primes)"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="primes"
LABEL benchmark.language="c"
LABEL benchmark.expected_result="9592"
