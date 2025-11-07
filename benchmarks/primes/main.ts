#!/usr/bin/env -S deno run --allow-hrtime
/**
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

/**
 * Sieve of Eratosthenes implementation
 * Returns count of primes up to n
 */
function sieveOfEratosthenes(n: number): number {
  if (n < 2) {
    return 0;
  }

  // Create boolean array "isPrime[0..n]" and initialize all entries as true
  const isPrime = new Array(n + 1).fill(true);
  isPrime[0] = false;
  isPrime[1] = false;

  let p = 2;
  while (p * p <= n) {
    // If isPrime[p] is not changed, then it is a prime
    if (isPrime[p]) {
      // Mark all multiples of p as not prime
      for (let i = p * p; i <= n; i += p) {
        isPrime[i] = false;
      }
    }
    p += 1;
  }

  // Count primes
  return isPrime.filter((prime) => prime).length;
}

function main(): void {
  // Measure startup time (initialization)
  const t0 = performance.now();

  const n = 100_000;

  const t1 = performance.now();

  // Compute benchmark
  const result = sieveOfEratosthenes(n);

  const t2 = performance.now();

  // Calculate times in microseconds
  const startupTimeUs = Math.floor((t1 - t0) * 1000);
  const computeTimeUs = Math.floor((t2 - t1) * 1000);

  // Output standardized format
  console.log(`STARTUP_TIME_US: ${startupTimeUs}`);
  console.log(`COMPUTE_TIME_US: ${computeTimeUs}`);
  console.log(`RESULT: ${result}`);

  // Validate result
  if (result !== 9592) {
    throw new Error(`Expected 9592 primes up to 100,000, got ${result}`);
  }
}

if (import.meta.main) {
  main();
}
