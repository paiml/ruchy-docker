# Multi-stage Dockerfile for Prime Sieve benchmark (Go)
# Target: <10MB image size, <10ms startup time
#
# Stage 1: Build stage (includes Go compiler, build tools)
# Stage 2: Runtime stage (distroless, static binary only)

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM golang:1.23-bookworm AS builder

WORKDIR /build

# Copy benchmark source
COPY benchmarks/primes/main.go .

# Build with static linking
# CGO_ENABLED=0: Disable CGO for pure static binary
# -ldflags '-s -w': Strip debug symbols
RUN \
    CGO_ENABLED=0 go build -ldflags="-s -w" -o primes main.go

# ============================================================================
# Stage 2: Runtime (scratch - absolute minimum)
# ============================================================================
FROM scratch

# Copy statically-linked binary from builder
COPY --from=builder /build/primes /primes

# Set binary as entrypoint
ENTRYPOINT ["/primes"]

# Metadata labels
LABEL org.opencontainers.image.title="Prime Sieve Benchmark (Go)"
LABEL org.opencontainers.image.description="BENCH-008: Recursive Prime Sieve 100K primes"
LABEL org.opencontainers.image.version="1.0.0"
LABEL benchmark.name="primes"
LABEL benchmark.language="go"
LABEL benchmark.expected_result="9592"
