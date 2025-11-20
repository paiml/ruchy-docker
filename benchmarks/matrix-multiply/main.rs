/// Matrix Multiply Benchmark (128×128)
/// SIMD-accelerated via trueno (AVX-512/AVX2/SSE2)
/// Expected: 7× faster than naive O(n³) baseline (C/Go/Ruchy/Deno)
/// This demonstrates trueno's performance advantage on compute-intensive workloads

use std::time::Instant;
use trueno::Matrix;

fn main() {
    let t0 = Instant::now();

    // Initialize two 128×128 matrices with sequential values
    let size = 128;
    let mut a_data = Vec::with_capacity(size * size);
    let mut b_data = Vec::with_capacity(size * size);

    for i in 0..(size * size) {
        a_data.push((i % 100) as f32);
        b_data.push(((i * 2) % 100) as f32);
    }

    let a = Matrix::from_vec(size, size, a_data).unwrap();
    let b = Matrix::from_vec(size, size, b_data).unwrap();

    let t1 = Instant::now();
    let startup_time = t1.duration_since(t0);

    // Perform matrix multiplication (SIMD-accelerated via trueno)
    let c = a.matmul(&b).unwrap();

    let t2 = Instant::now();
    let compute_time = t2.duration_since(t1);

    // Verify result (checksum to prevent optimization elimination)
    let mut sum = 0.0;
    for i in 0..(size * size) {
        sum += c.as_slice()[i];
    }

    // Standardized output format
    println!("STARTUP_TIME_US: {}", startup_time.as_micros());
    println!("COMPUTE_TIME_US: {}", compute_time.as_micros());
    println!("RESULT: {}", sum as i64);
}
