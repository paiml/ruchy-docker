"""
Matrix Multiply Benchmark (128×128)
Julia implementation with LLVM-based SIMD optimization
Expected: Competitive with trueno due to LLVM optimization
"""

function matmul_benchmark()
    t0 = time_ns()

    # Initialize two 128×128 matrices with sequential values
    size = 128
    a = reshape([Float64(i % 100) for i in 0:(size*size-1)], size, size)
    b = reshape([Float64((i * 2) % 100) for i in 0:(size*size-1)], size, size)

    t1 = time_ns()
    startup_time_us = div(t1 - t0, 1000)

    # Perform matrix multiplication (LLVM-optimized)
    c = a * b

    t2 = time_ns()
    compute_time_us = div(t2 - t1, 1000)

    # Verify result (checksum)
    result = Int64(sum(c))

    # Standardized output format
    println("STARTUP_TIME_US: $startup_time_us")
    println("COMPUTE_TIME_US: $compute_time_us")
    println("RESULT: $result")
end

matmul_benchmark()
