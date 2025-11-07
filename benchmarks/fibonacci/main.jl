#!/usr/bin/env julia
#=
BENCH-007: Recursive Fibonacci

Compute fib(35) using naive recursive algorithm.
Expected result: 9,227,465

This benchmark tests:
- Function call overhead
- Stack frame allocation
- Integer arithmetic
- Recursion depth handling
- JIT compilation performance
=#

function fibonacci(n::Int64)::Int64
    """Naive recursive Fibonacci implementation."""
    if n <= 1
        return n
    end
    return fibonacci(n - 1) + fibonacci(n - 2)
end

function main()
    # Measure startup time
    t0 = time_ns()

    # Startup phase: allocate data structures, initialize state
    # Use a warmup loop to prevent optimizer elimination
    warmup = 0
    for i in 1:100000
        warmup += i
    end

    n = 35
    # Use warmup to prevent optimizer elimination
    if warmup == 0
        error("warmup failed")
    end

    t1 = time_ns()

    # Compute benchmark
    result = fibonacci(n)

    t2 = time_ns()

    # Calculate times in microseconds
    startup_time_us = div(t1 - t0, 1000)
    compute_time_us = div(t2 - t1, 1000)

    # Output standardized format
    println("STARTUP_TIME_US: ", startup_time_us)
    println("COMPUTE_TIME_US: ", compute_time_us)
    println("RESULT: ", result)

    # Validate result
    @assert result == 9227465 "Expected fib(35) = 9227465, got $result"
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end
