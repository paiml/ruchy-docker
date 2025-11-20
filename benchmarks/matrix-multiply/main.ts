/**
 * Matrix Multiply Benchmark (128×128)
 * Naive O(n³) implementation (no SIMD)
 * Deno TypeScript implementation
 * Expected: Baseline for comparison - trueno should be ~7× faster
 */

const SIZE = 128;

// Naive matrix multiplication O(n³)
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
    const a: number[][] = [];
    const b: number[][] = [];
    for (let i = 0; i < SIZE; i++) {
        a[i] = [];
        b[i] = [];
        for (let j = 0; j < SIZE; j++) {
            const idx = i * SIZE + j;
            a[i][j] = idx % 100;
            b[i][j] = (idx * 2) % 100;
        }
    }

    const t1 = performance.now();
    const startupTimeUs = Math.floor((t1 - t0) * 1000);

    // Perform matrix multiplication
    const c = matmul(a, b);

    const t2 = performance.now();
    const computeTimeUs = Math.floor((t2 - t1) * 1000);

    // Verify result (checksum)
    let sum = 0.0;
    for (let i = 0; i < SIZE; i++) {
        for (let j = 0; j < SIZE; j++) {
            sum += c[i][j];
        }
    }

    // Standardized output format
    console.log(`STARTUP_TIME_US: ${startupTimeUs}`);
    console.log(`COMPUTE_TIME_US: ${computeTimeUs}`);
    console.log(`RESULT: ${Math.floor(sum)}`);
}

main();
