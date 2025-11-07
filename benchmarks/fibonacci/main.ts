#!/usr/bin/env -S deno run --allow-hrtime
/**
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

/**
 * Naive recursive Fibonacci implementation.
 */
function fibonacci(n: number): number {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

function main(): void {
  // Measure startup time
  const t0 = performance.now();

  // Startup phase: allocate data structures, initialize state
  let warmup = 0;
  for (let i = 0; i < 100000; i++) {
    warmup += i;
  }

  const n = 35;
  // Use warmup to prevent optimizer elimination
  if (warmup === 0) {
    throw new Error("warmup failed");
  }

  const t1 = performance.now();

  // Compute benchmark
  const result = fibonacci(n);

  const t2 = performance.now();

  // Calculate times in microseconds
  const startupTimeUs = Math.floor((t1 - t0) * 1000);
  const computeTimeUs = Math.floor((t2 - t1) * 1000);

  // Output standardized format
  console.log(`STARTUP_TIME_US: ${startupTimeUs}`);
  console.log(`COMPUTE_TIME_US: ${computeTimeUs}`);
  console.log(`RESULT: ${result}`);

  // Validate result
  if (result !== 9227465) {
    throw new Error(`Expected fib(35) = 9227465, got ${result}`);
  }
}

if (import.meta.main) {
  main();
}
