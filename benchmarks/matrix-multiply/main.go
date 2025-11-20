// Matrix Multiply Benchmark (128×128)
// Naive O(n³) implementation (no SIMD)
// Expected: Baseline for comparison - trueno should be ~7× faster

package main

import (
	"fmt"
	"time"
)

const size = 128

// Naive matrix multiplication O(n³)
func matmul(a, b [][]float64) [][]float64 {
	n := len(a)
	c := make([][]float64, n)
	for i := range c {
		c[i] = make([]float64, n)
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			sum := 0.0
			for k := 0; k < n; k++ {
				sum += a[i][k] * b[k][j]
			}
			c[i][j] = sum
		}
	}
	return c
}

func main() {
	t0 := time.Now()

	// Initialize matrices with sequential values
	a := make([][]float64, size)
	b := make([][]float64, size)
	for i := 0; i < size; i++ {
		a[i] = make([]float64, size)
		b[i] = make([]float64, size)
		for j := 0; j < size; j++ {
			idx := i*size + j
			a[i][j] = float64(idx % 100)
			b[i][j] = float64((idx * 2) % 100)
		}
	}

	t1 := time.Now()
	startupTimeUs := t1.Sub(t0).Microseconds()

	// Perform matrix multiplication
	c := matmul(a, b)

	t2 := time.Now()
	computeTimeUs := t2.Sub(t1).Microseconds()

	// Verify result (checksum)
	sum := 0.0
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			sum += c[i][j]
		}
	}

	// Standardized output format
	fmt.Printf("STARTUP_TIME_US: %d\n", startupTimeUs)
	fmt.Printf("COMPUTE_TIME_US: %d\n", computeTimeUs)
	fmt.Printf("RESULT: %d\n", int64(sum))
}
