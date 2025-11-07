# Multi-stage Dockerfile for Primes benchmark (Deno)
# Target: Minimal image size, Deno 2.x with V8 JIT
#
# Stage 1: Cache dependencies (Deno pre-compiles TypeScript)
# Stage 2: Runtime with compiled binary

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM denoland/deno:2.1.4 AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/primes/main.ts .

# Compile TypeScript to single executable binary
# --allow-hrtime: Required for high-resolution performance.now()
# --no-check: Skip type checking for faster builds (already type-checked in development)
RUN deno compile \
    --allow-hrtime \
    --output /build/primes \
    main.ts

# ============================================================================
# Stage 2: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy compiled binary (includes embedded V8 runtime)
COPY --from=builder /build/primes /primes

# Set entrypoint
ENTRYPOINT ["/primes"]

# Metadata labels
LABEL org.opencontainers.image.title="Primes Benchmark (Deno)"
LABEL org.opencontainers.image.description="BENCH-008: Prime Sieve (Sieve of Eratosthenes)"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="primes"
LABEL benchmark.language="deno"
LABEL benchmark.expected_result="9592"
