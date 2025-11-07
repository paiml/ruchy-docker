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

package main

import (
	"fmt"
	"time"
)

// sieveOfEratosthenes implements the Sieve of Eratosthenes algorithm
// Returns count of primes up to n
func sieveOfEratosthenes(n int) int {
	if n < 2 {
		return 0
	}

	// Create boolean array "isPrime[0..n]" and initialize all entries as true
	isPrime := make([]bool, n+1)
	for i := range isPrime {
		isPrime[i] = true
	}
	isPrime[0] = false
	isPrime[1] = false

	p := 2
	for p*p <= n {
		// If isPrime[p] is not changed, then it is a prime
		if isPrime[p] {
			// Mark all multiples of p as not prime
			for i := p * p; i <= n; i += p {
				isPrime[i] = false
			}
		}
		p++
	}

	// Count primes
	count := 0
	for _, prime := range isPrime {
		if prime {
			count++
		}
	}

	return count
}

func main() {
	// Measure startup time (initialization)
	t0 := time.Now()

	n := 100000

	t1 := time.Now()

	// Compute benchmark
	result := sieveOfEratosthenes(n)

	t2 := time.Now()

	// Calculate times in microseconds
	startupTimeUs := t1.Sub(t0).Microseconds()
	computeTimeUs := t2.Sub(t1).Microseconds()

	// Output standardized format
	fmt.Printf("STARTUP_TIME_US: %d\n", startupTimeUs)
	fmt.Printf("COMPUTE_TIME_US: %d\n", computeTimeUs)
	fmt.Printf("RESULT: %d\n", result)

	// Validate result
	if result != 9592 {
		panic(fmt.Sprintf("Expected 9592 primes up to 100,000, got %d", result))
	}
}
