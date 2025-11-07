/*
 * BENCH-008: Prime Sieve (Sieve of Eratosthenes)
 *
 * Find all prime numbers up to 100,000 using the Sieve of Eratosthenes algorithm.
 * Expected result: 9,592 primes
 *
 * This benchmark tests:
 * - Array allocation and manipulation
 * - Bit/boolean array operations
 * - Nested loop performance
 * - Memory access patterns (cache performance)
 * - Integer arithmetic
 */

use std::time::Instant;

/// Sieve of Eratosthenes implementation
/// Returns count of primes up to n
fn sieve_of_eratosthenes(n: usize) -> usize {
    if n < 2 {
        return 0;
    }

    // Create boolean array "is_prime[0..n]" and initialize all entries as true
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= n {
        // If is_prime[p] is not changed, then it is a prime
        if is_prime[p] {
            // Mark all multiples of p as not prime
            let mut i = p * p;
            while i <= n {
                is_prime[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    // Count primes
    is_prime.iter().filter(|&&prime| prime).count()
}

fn main() {
    // Measure startup time (initialization)
    let t0 = Instant::now();

    let n = 100_000;

    let t1 = Instant::now();

    // Compute benchmark
    let result = sieve_of_eratosthenes(n);

    let t2 = Instant::now();

    // Calculate times in microseconds
    let startup_time_us = t1.duration_since(t0).as_micros();
    let compute_time_us = t2.duration_since(t1).as_micros();

    // Output standardized format
    println!("STARTUP_TIME_US: {}", startup_time_us);
    println!("COMPUTE_TIME_US: {}", compute_time_us);
    println!("RESULT: {}", result);

    // Validate result
    assert_eq!(
        result, 9592,
        "Expected 9592 primes up to 100,000, got {}",
        result
    );
}
