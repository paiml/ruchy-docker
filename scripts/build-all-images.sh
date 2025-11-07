#!/usr/bin/env bash
#
# build-all-images.sh - Build all Docker benchmark images
#
# Builds all 16 Docker containers (2 benchmarks × 8 languages)
# Usage: ./scripts/build-all-images.sh

set -euo pipefail

echo "=================================="
echo "Building All Docker Images"
echo "=================================="
echo ""
echo "Total: 2 benchmarks × 8 languages = 16 containers"
echo ""

# Build Fibonacci benchmark (8 languages)
echo "Building Fibonacci benchmark (8 languages)..."
echo ""

echo "[1/16] Building C fibonacci..."
docker build -f docker/c/fibonacci.Dockerfile -t c:fibonacci .

echo "[2/16] Building Rust fibonacci..."
docker build -f docker/rust/fibonacci.Dockerfile -t rust:fibonacci .

echo "[3/16] Building Go fibonacci..."
docker build -f docker/go/fibonacci.Dockerfile -t go:fibonacci .

echo "[4/16] Building Python fibonacci..."
docker build -f docker/python/fibonacci.Dockerfile -t python:fibonacci .

echo "[5/16] Building Julia fibonacci..."
docker build -f docker/julia/fibonacci.Dockerfile -t julia:fibonacci .

echo "[6/16] Building Deno fibonacci..."
docker build -f docker/deno/fibonacci.Dockerfile -t deno:fibonacci .

echo "[7/16] Building Ruchy transpiled fibonacci..."
cd .. && docker build -f ruchy-docker/docker/ruchy-transpiled/fibonacci.Dockerfile -t ruchy-transpiled:fibonacci . && cd ruchy-docker

echo "[8/16] Building Ruchy compiled fibonacci..."
cd .. && docker build -f ruchy-docker/docker/ruchy-compiled/fibonacci.Dockerfile -t ruchy-compiled:fibonacci . && cd ruchy-docker

echo ""
echo "Fibonacci benchmark complete!"
echo ""

# Build Primes benchmark (8 languages)
echo "Building Primes benchmark (8 languages)..."
echo ""

echo "[9/16] Building C primes..."
docker build -f docker/c/primes.Dockerfile -t c:primes .

echo "[10/16] Building Rust primes..."
docker build -f docker/rust/primes.Dockerfile -t rust:primes .

echo "[11/16] Building Go primes..."
docker build -f docker/go/primes.Dockerfile -t go:primes .

echo "[12/16] Building Python primes..."
docker build -f docker/python/primes.Dockerfile -t python:primes .

echo "[13/16] Building Julia primes..."
docker build -f docker/julia/primes.Dockerfile -t julia:primes .

echo "[14/16] Building Deno primes..."
docker build -f docker/deno/primes.Dockerfile -t deno:primes .

echo "[15/16] Building Ruchy transpiled primes..."
cd .. && docker build -f ruchy-docker/docker/ruchy-transpiled/primes.Dockerfile -t ruchy-transpiled:primes . && cd ruchy-docker

echo "[16/16] Building Ruchy compiled primes..."
cd .. && docker build -f ruchy-docker/docker/ruchy-compiled/primes.Dockerfile -t ruchy-compiled:primes . && cd ruchy-docker

echo ""
echo "=================================="
echo "✅ All 16 images built successfully!"
echo "=================================="
echo ""
echo "Note: Julia is EXPERIMENTAL (JIT compilation, large image size)"
echo ""
echo "Verify with: docker images | grep -E '(fibonacci|primes)'"
