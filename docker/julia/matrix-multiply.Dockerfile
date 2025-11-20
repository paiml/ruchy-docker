# Dockerfile for Matrix Multiply benchmark (Julia)
# Julia uses LLVM-based JIT with SIMD optimization
# Note: Large image (~711 MB) due to JIT compiler

FROM julia:1.10-bullseye

WORKDIR /app
COPY benchmarks/matrix-multiply/main.jl .

# Pre-compile for faster execution (optional)
RUN julia -e 'using Pkg; Pkg.precompile()'

CMD ["julia", "main.jl"]

LABEL org.opencontainers.image.title="Matrix Multiply Benchmark (Julia)"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="julia"
LABEL benchmark.simd="true"
LABEL benchmark.jit="llvm"
