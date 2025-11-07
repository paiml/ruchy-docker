#!/usr/bin/env python3
"""
BENCH-007: Recursive Fibonacci

Compute fib(35) using naive recursive algorithm.
Expected result: 9,227,465

This benchmark tests:
- Function call overhead
- Stack frame allocation
- Integer arithmetic
- Recursion depth handling
"""

import time


def fibonacci(n: int) -> int:
    """Naive recursive Fibonacci implementation."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


def main() -> None:
    # Measure startup time
    t0 = time.perf_counter_ns()

    # Startup phase: allocate data structures, initialize state
    warmup = 0
    for i in range(100000):
        warmup += i

    n = 35
    # Use warmup to prevent optimizer elimination
    if warmup == 0:
        raise ValueError("warmup failed")

    t1 = time.perf_counter_ns()

    # Compute benchmark
    result = fibonacci(n)

    t2 = time.perf_counter_ns()

    # Calculate times in microseconds
    startup_time_us = (t1 - t0) // 1000
    compute_time_us = (t2 - t1) // 1000

    # Output standardized format
    print(f"STARTUP_TIME_US: {startup_time_us}")
    print(f"COMPUTE_TIME_US: {compute_time_us}")
    print(f"RESULT: {result}")

    # Validate result
    assert result == 9227465, f"Expected fib(35) = 9227465, got {result}"


if __name__ == "__main__":
    main()
