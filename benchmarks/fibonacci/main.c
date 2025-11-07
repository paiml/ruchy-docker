/*
 * BENCH-007: Recursive Fibonacci
 *
 * Compute fib(35) using naive recursive algorithm.
 * Expected result: 9,227,465
 *
 * This benchmark tests:
 * - Function call overhead
 * - Stack frame allocation
 * - Integer arithmetic
 * - Recursion depth handling
 */

#include <stdint.h>
#include <stdio.h>
#include <time.h>

uint64_t fibonacci(uint64_t n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main(void) {
    struct timespec t0, t1, t2;

    // Measure startup time
    clock_gettime(CLOCK_MONOTONIC, &t0);

    // Startup phase: allocate data structures, initialize state
    volatile uint64_t warmup = 0;
    for (int i = 0; i < 100000; i++) {
        warmup += i;
    }

    const uint64_t n = 35;
    // Use warmup to prevent optimizer elimination
    if (warmup == 0) return 1;

    clock_gettime(CLOCK_MONOTONIC, &t1);

    // Compute benchmark
    uint64_t result = fibonacci(n);

    clock_gettime(CLOCK_MONOTONIC, &t2);

    // Calculate times in microseconds
    uint64_t startup_time_us = (t1.tv_sec - t0.tv_sec) * 1000000 +
                                (t1.tv_nsec - t0.tv_nsec) / 1000;
    uint64_t compute_time_us = (t2.tv_sec - t1.tv_sec) * 1000000 +
                                (t2.tv_nsec - t1.tv_nsec) / 1000;

    // Output standardized format
    printf("STARTUP_TIME_US: %lu\n", startup_time_us);
    printf("COMPUTE_TIME_US: %lu\n", compute_time_us);
    printf("RESULT: %lu\n", result);

    // Validate result
    if (result != 9227465) {
        fprintf(stderr, "ERROR: Expected fib(35) = 9227465, got %lu\n", result);
        return 1;
    }

    return 0;
}
