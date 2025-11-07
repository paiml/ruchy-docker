# BENCH-008: Prime Sieve (Sieve of Eratosthenes)
#
# Find all prime numbers up to 100,000 using the Sieve of Eratosthenes algorithm.
# Expected result: 9,592 primes
#
# This benchmark tests:
# - Array allocation and manipulation
# - Bit/boolean array operations
# - Nested loop performance
# - Memory access patterns (cache performance)
# - Integer arithmetic

"""
    sieve_of_eratosthenes(n::Int) -> Int

Sieve of Eratosthenes implementation.
Returns count of primes up to n.
"""
function sieve_of_eratosthenes(n::Int)::Int
    if n < 2
        return 0
    end

    # Create boolean array "is_prime[1..n+1]" and initialize all entries as true
    is_prime = fill(true, n + 1)
    is_prime[1] = false
    is_prime[2] = false  # Julia uses 1-based indexing, so is_prime[2] is index 1 (value 0)

    p = 2
    while p * p <= n
        # If is_prime[p] is not changed, then it is a prime
        if is_prime[p + 1]  # +1 for 1-based indexing
            # Mark all multiples of p as not prime
            i = p * p
            while i <= n
                is_prime[i + 1] = false  # +1 for 1-based indexing
                i += p
            end
        end
        p += 1
    end

    # Count primes
    return count(is_prime)
end

function main()
    # Measure startup time (initialization)
    t0 = time_ns()

    n = 100_000

    t1 = time_ns()

    # Compute benchmark
    result = sieve_of_eratosthenes(n)

    t2 = time_ns()

    # Calculate times in microseconds
    startup_time_us = div(t1 - t0, 1_000)
    compute_time_us = div(t2 - t1, 1_000)

    # Output standardized format
    println("STARTUP_TIME_US: ", startup_time_us)
    println("COMPUTE_TIME_US: ", compute_time_us)
    println("RESULT: ", result)

    # Validate result
    @assert result == 9592 "Expected 9592 primes up to 100,000, got $result"
end

# Call main
main()
