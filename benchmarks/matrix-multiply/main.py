"""
Matrix Multiply Benchmark (128×128)
Uses NumPy for SIMD-accelerated matrix multiplication via OpenBLAS/MKL
Expected: Competitive with trueno on CPU, but 177 MB runtime vs 312 KB
"""

import time
import numpy as np

def main():
    t0 = time.perf_counter()

    # Initialize two 128×128 matrices with sequential values
    size = 128
    a = np.array([i % 100 for i in range(size * size)], dtype=np.float64).reshape(size, size)
    b = np.array([(i * 2) % 100 for i in range(size * size)], dtype=np.float64).reshape(size, size)

    t1 = time.perf_counter()
    startup_time_us = int((t1 - t0) * 1_000_000)

    # Perform matrix multiplication (SIMD-accelerated via NumPy/OpenBLAS)
    c = np.matmul(a, b)

    t2 = time.perf_counter()
    compute_time_us = int((t2 - t1) * 1_000_000)

    # Verify result (checksum to prevent optimization elimination)
    result = int(np.sum(c))

    # Standardized output format
    print(f"STARTUP_TIME_US: {startup_time_us}")
    print(f"COMPUTE_TIME_US: {compute_time_us}")
    print(f"RESULT: {result}")

if __name__ == "__main__":
    main()
