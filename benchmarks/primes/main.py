"""
BENCH-008: Prime Sieve (Sieve of Eratosthenes)

Find all prime numbers up to 100,000 using the Sieve of Eratosthenes algorithm.
Expected result: 9,592 primes

This benchmark tests:
- Array allocation and manipulation
- Bit/boolean array operations
- Nested loop performance
- Memory access patterns (cache performance)
- Integer arithmetic
"""

import time


def sieve_of_eratosthenes(n: int) -> int:
    """
    Sieve of Eratosthenes implementation.
    Returns count of primes up to n.
    """
    if n < 2:
        return 0

    # Create boolean array "is_prime[0..n]" and initialize all entries as True
    is_prime = [True] * (n + 1)
    is_prime[0] = False
    is_prime[1] = False

    p = 2
    while p * p <= n:
        # If is_prime[p] is not changed, then it is a prime
        if is_prime[p]:
            # Mark all multiples of p as not prime
            for i in range(p * p, n + 1, p):
                is_prime[i] = False
        p += 1

    # Count primes
    return sum(is_prime)


def main():
    # Measure startup time (initialization)
    t0 = time.perf_counter()

    n = 100_000

    t1 = time.perf_counter()

    # Compute benchmark
    result = sieve_of_eratosthenes(n)

    t2 = time.perf_counter()

    # Calculate times in microseconds
    startup_time_us = int((t1 - t0) * 1_000_000)
    compute_time_us = int((t2 - t1) * 1_000_000)

    # Output standardized format
    print(f"STARTUP_TIME_US: {startup_time_us}")
    print(f"COMPUTE_TIME_US: {compute_time_us}")
    print(f"RESULT: {result}")

    # Validate result
    assert result == 9592, f"Expected 9592 primes up to 100,000, got {result}"


if __name__ == "__main__":
    main()
