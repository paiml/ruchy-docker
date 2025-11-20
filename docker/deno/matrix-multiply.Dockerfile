# Multi-stage Dockerfile for Matrix Multiply benchmark (Deno)
# Deno compiles to standalone binary (~90MB with V8 runtime)

FROM denoland/deno:debian-2.1.4 AS builder

WORKDIR /build
COPY benchmarks/matrix-multiply/main.ts .

# Compile TypeScript to standalone binary
RUN deno compile --allow-all --output matrix-multiply main.ts

FROM debian:12-slim
COPY --from=builder /build/matrix-multiply /matrix-multiply
ENTRYPOINT ["/matrix-multiply"]

LABEL org.opencontainers.image.title="Matrix Multiply Benchmark (Deno)"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="deno"
LABEL benchmark.simd="false"
