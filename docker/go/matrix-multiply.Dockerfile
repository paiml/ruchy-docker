# Multi-stage Dockerfile for Matrix Multiply benchmark (Go)
# Target: <10MB image size, Naive O(n³) baseline

FROM golang:1.23-bookworm AS builder

WORKDIR /build
COPY benchmarks/matrix-multiply/main.go .

# Build static binary
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -a -ldflags="-s -w" -o matrix-multiply main.go

FROM scratch
COPY --from=builder /build/matrix-multiply /matrix-multiply
ENTRYPOINT ["/matrix-multiply"]

LABEL org.opencontainers.image.title="Matrix Multiply Benchmark (Go)"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="go"
LABEL benchmark.simd="false"
