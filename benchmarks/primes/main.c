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

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include <assert.h>

// Get time in microseconds
long long get_time_us(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (long long)ts.tv_sec * 1000000LL + ts.tv_nsec / 1000LL;
}

// Sieve of Eratosthenes implementation
// Returns count of primes up to n
int sieve_of_eratosthenes(int n) {
    if (n < 2) {
        return 0;
    }

    // Create boolean array "is_prime[0..n]" and initialize all entries as true
    bool *is_prime = (bool *)malloc((n + 1) * sizeof(bool));
    for (int i = 0; i <= n; i++) {
        is_prime[i] = true;
    }
    is_prime[0] = false;
    is_prime[1] = false;

    int p = 2;
    while (p * p <= n) {
        // If is_prime[p] is not changed, then it is a prime
        if (is_prime[p]) {
            // Mark all multiples of p as not prime
            for (int i = p * p; i <= n; i += p) {
                is_prime[i] = false;
            }
        }
        p++;
    }

    // Count primes
    int count = 0;
    for (int i = 0; i <= n; i++) {
        if (is_prime[i]) {
            count++;
        }
    }

    free(is_prime);
    return count;
}

int main(void) {
    // Measure startup time (initialization)
    long long t0 = get_time_us();

    int n = 100000;

    long long t1 = get_time_us();

    // Compute benchmark
    int result = sieve_of_eratosthenes(n);

    long long t2 = get_time_us();

    // Calculate times in microseconds
    long long startup_time_us = t1 - t0;
    long long compute_time_us = t2 - t1;

    // Output standardized format
    printf("STARTUP_TIME_US: %lld\n", startup_time_us);
    printf("COMPUTE_TIME_US: %lld\n", compute_time_us);
    printf("RESULT: %d\n", result);

    // Validate result
    assert(result == 9592 && "Expected 9592 primes up to 100,000");

    return 0;
}
