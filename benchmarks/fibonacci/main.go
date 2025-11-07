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

package main

import (
	"fmt"
	"time"
)

func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

func main() {
	// Measure startup time
	t0 := time.Now()

	// Startup phase: allocate data structures, initialize state
	warmup := 0
	for i := 0; i < 100000; i++ {
		warmup += i
	}

	n := 35
	// Use warmup to prevent optimizer elimination
	if warmup == 0 {
		panic("warmup failed")
	}

	t1 := time.Now()

	// Compute benchmark
	result := fibonacci(n)

	t2 := time.Now()

	// Calculate times in microseconds
	startupTimeUs := t1.Sub(t0).Microseconds()
	computeTimeUs := t2.Sub(t1).Microseconds()

	// Output standardized format
	fmt.Printf("STARTUP_TIME_US: %d\n", startupTimeUs)
	fmt.Printf("COMPUTE_TIME_US: %d\n", computeTimeUs)
	fmt.Printf("RESULT: %d\n", result)

	// Validate result
	if result != 9227465 {
		panic(fmt.Sprintf("Expected fib(35) = 9227465, got %d", result))
	}
}
