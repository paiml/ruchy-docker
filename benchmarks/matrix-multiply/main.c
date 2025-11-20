/**
 * Matrix Multiply Benchmark (128×128)
 * Naive O(n³) implementation (no SIMD)
 * Expected: Baseline for comparison - trueno should be ~7× faster
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>

#define SIZE 128

// Get time in microseconds
uint64_t get_time_us() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000 + (uint64_t)ts.tv_nsec / 1000;
}

// Naive matrix multiplication O(n³)
void matmul(double *a, double *b, double *c, int n) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            double sum = 0.0;
            for (int k = 0; k < n; k++) {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

int main() {
    uint64_t t0 = get_time_us();

    // Allocate matrices
    double *a = (double*)malloc(SIZE * SIZE * sizeof(double));
    double *b = (double*)malloc(SIZE * SIZE * sizeof(double));
    double *c = (double*)malloc(SIZE * SIZE * sizeof(double));

    // Initialize with sequential values
    for (int i = 0; i < SIZE * SIZE; i++) {
        a[i] = (double)(i % 100);
        b[i] = (double)((i * 2) % 100);
    }

    uint64_t t1 = get_time_us();
    uint64_t startup_time_us = t1 - t0;

    // Perform matrix multiplication
    matmul(a, b, c, SIZE);

    uint64_t t2 = get_time_us();
    uint64_t compute_time_us = t2 - t1;

    // Verify result (checksum)
    double sum = 0.0;
    for (int i = 0; i < SIZE * SIZE; i++) {
        sum += c[i];
    }

    // Standardized output format
    printf("STARTUP_TIME_US: %lu\n", startup_time_us);
    printf("COMPUTE_TIME_US: %lu\n", compute_time_us);
    printf("RESULT: %ld\n", (int64_t)sum);

    free(a);
    free(b);
    free(c);

    return 0;
}
