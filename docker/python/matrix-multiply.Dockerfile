# Dockerfile for Matrix Multiply benchmark (Python + NumPy)
# Note: Python requires full runtime (~177MB with NumPy)
# NumPy provides SIMD via OpenBLAS/MKL

FROM python:3.12-slim

WORKDIR /app

# Install NumPy (SIMD-accelerated matrix operations)
RUN pip install --no-cache-dir numpy==1.26.4

# Copy benchmark source
COPY benchmarks/matrix-multiply/main.py .

# Run Python script
CMD ["python3", "main.py"]

LABEL org.opencontainers.image.title="Matrix Multiply Benchmark (Python + NumPy)"
LABEL benchmark.name="matrix-multiply"
LABEL benchmark.language="python"
LABEL benchmark.simd="true"
LABEL benchmark.library="numpy-1.26.4"
