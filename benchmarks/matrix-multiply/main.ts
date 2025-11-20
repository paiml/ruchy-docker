/// Matrix Multiply Benchmark (128x128)
/// Naive O(n³) implementation (no SIMD)
/// Expected: Baseline comparable to C/Go

function matmul(a: number[][], b: number[][]): number[][] {
    const n = a.length;
    const c: number[][] = Array(n).fill(0).map(() => Array(n).fill(0));

    for (let i = 0; i < n; i++) {
        for (let j = 0; j < n; j++) {
            let sum = 0.0;
            for (let k = 0; k < n; k++) {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }
    return c;
}

function main() {
    const t0 = performance.now();

    // Initialize matrices with sequential values
    const size = 128;
    const a: number[][] = Array(size).fill(0).map(() => Array(size).fill(0));
    const b: number[][] = Array(size).fill(0).map(() => Array(size).fill(0));

    for (let i = 0; i < size; i++) {
        for (let j = 0; j < size; j++) {
            const idx = i * size + j;
            a[i][j] = idx % 100;
            b[i][j] = (idx * 2) % 100;
        }
    }

    const t1 = performance.now();
    const startup_time_us = Math.floor((t1 - t0) * 1000);

    // Perform matrix multiplication
    const c = matmul(a, b);

    const t2 = performance.now();
    const compute_time_us = Math.floor((t2 - t1) * 1000);

    // Verify result (checksum)
    let sum = 0.0;
    for (let i = 0; i < size; i++) {
        for (let j = 0; j < size; j++) {
            sum += c[i][j];
        }
    }

    // Standardized output format
    console.log('STARTUP_TIME_US: ' + startup_time_us);
    console.log('COMPUTE_TIME_US: ' + compute_time_us);
    console.log('RESULT: ' + Math.floor(sum));
}

main();
