use std::time::Instant;

/// BENCH-007: Recursive Fibonacci
///
/// Compute fib(35) using naive recursive algorithm.
/// Expected result: 9,227,465
///
/// This benchmark tests:
/// - Function call overhead
/// - Stack frame allocation
/// - Integer arithmetic
/// - Recursion depth handling
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    // Measure startup time
    let t0 = Instant::now();

    // Startup phase: allocate data structures, initialize state
    let mut warmup: u64 = 0;
    for i in 0..100000 {
        warmup += i;
    }

    let n: u64 = 35;
    // Use warmup to prevent optimizer elimination
    if warmup == 0 { panic!("warmup failed"); }

    let t1 = Instant::now();
    let startup_time = t1.duration_since(t0);

    // Compute benchmark
    let result = fibonacci(n);

    let t2 = Instant::now();
    let compute_time = t2.duration_since(t1);

    // Output standardized format
    println!("STARTUP_TIME_US: {}", startup_time.as_micros());
    println!("COMPUTE_TIME_US: {}", compute_time.as_micros());
    println!("RESULT: {}", result);

    // Validate result
    assert_eq!(result, 9227465, "fib(35) should equal 9227465");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
    }

    #[test]
    fn test_fibonacci_35() {
        assert_eq!(fibonacci(35), 9227465);
    }

    #[test]
    fn test_fibonacci_sequence() {
        // Verify Fibonacci property: F(n) = F(n-1) + F(n-2)
        for n in 2..20 {
            assert_eq!(fibonacci(n), fibonacci(n - 1) + fibonacci(n - 2));
        }
    }
}
