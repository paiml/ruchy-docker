# Matrix Multiply Benchmark (128×128)
# Naive O(n³) implementation (no SIMD)
# Expected: Baseline comparable to C/Go

function matmul(a::Matrix{Float64}, b::Matrix{Float64})::Matrix{Float64}
    n = size(a, 1)
    c = zeros(Float64, n, n)
    
    for i in 1:n
        for j in 1:n
            sum = 0.0
            for k in 1:n
                sum += a[i, k] * b[k, j]
            end
            c[i, j] = sum
        end
    end
    return c
end

function main()
    t0 = time_ns()
    
    # Initialize matrices with sequential values
    size = 128
    a = zeros(Float64, size, size)
    b = zeros(Float64, size, size)
    
    for i in 1:size
        for j in 1:size
            idx = (i - 1) * size + (j - 1)
            a[i, j] = Float64(idx % 100)
            b[i, j] = Float64((idx * 2) % 100)
        end
    end
    
    t1 = time_ns()
    startup_time = div(t1 - t0, 1000)
    
    # Perform matrix multiplication
    c = matmul(a, b)
    
    t2 = time_ns()
    compute_time = div(t2 - t1, 1000)
    
    # Verify result (checksum)
    result = sum(c)
    
    # Standardized output format
    println("STARTUP_TIME_US: ", startup_time)
    println("COMPUTE_TIME_US: ", compute_time)
    println("RESULT: ", Int64(floor(result)))
end

main()
