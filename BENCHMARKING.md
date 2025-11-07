# How to Run Benchmarks

## Container Benchmarks (Instrumented)

### Build Images First
```bash
# Build all 16 containers (2 benchmarks × 8 languages)
make build-images

# Or build just fibonacci
make build-fibonacci

# Or build just primes
make build-primes
```

### Run Single Benchmark
```bash
# Run any language's fibonacci benchmark
docker run --rm c:fibonacci
docker run --rm rust:fibonacci
docker run --rm ruchy-compiled:fibonacci
docker run --rm ruchy-transpiled:fibonacci
docker run --rm python:fibonacci
docker run --rm go:fibonacci
docker run --rm julia:fibonacci
docker run --rm deno:fibonacci

# Run primes benchmark
docker run --rm c:primes
docker run --rm rust:primes
docker run --rm ruchy-compiled:primes
# ... etc
```

### Run with Timing
```bash
# Time the full container lifecycle
time docker run --rm ruchy-compiled:fibonacci
time docker run --rm python:fibonacci
```

### Output Format
```
STARTUP_TIME_US: 8234      # App initialization time (microseconds)
COMPUTE_TIME_US: 21000     # Actual computation time (microseconds)
RESULT: 9227465            # Validation result (fibonacci(35))
```

## Local Benchmarks (CLI with bashrs)

### Extract Binaries from Containers
```bash
# Extract all binaries
make extract-binaries

# Check what was extracted
ls -lh bin/
```

### Run CLI Benchmarks
```bash
# Run full benchmark suite with bashrs
make bench-cli
```

This runs:
- 3 warmup iterations
- 10 measured iterations
- Statistical analysis (mean, std dev, outliers)
- JSON output to `results/cli/fibonacci-cli-bench.json`

### Manual Local Execution
```bash
# Run extracted binaries directly
./bin/fibonacci_c
./bin/fibonacci_rust
./bin/fibonacci_ruchy_compiled

# Time them manually
time ./bin/fibonacci_c
time ./bin/fibonacci_python.sh
```

## Quick Comparison

```bash
# Compare all languages (container)
for lang in c rust ruchy-compiled python; do
  echo "=== $lang ==="
  docker run --rm $lang:fibonacci | grep COMPUTE_TIME_US
done

# Compare all languages (local)
for bin in bin/fibonacci_*.sh; do
  echo "=== $(basename $bin) ==="
  time $bin 2>&1 | grep RESULT
done
```

## Results

### Container Benchmarks
**Pros:**
- Isolated environment
- Reproduces production deployment
- Includes Docker overhead
- Measures full lifecycle

**Cons:**
- Includes Docker startup overhead
- Slower than local

**Use when:**
- Testing production images
- Comparing deployment sizes
- Full system benchmarking

### Local Benchmarks
**Pros:**
- Faster iteration
- Lower overhead
- Direct binary execution
- Statistical rigor (bashrs)

**Cons:**
- Doesn't test Docker image
- Requires extraction step
- Not available for interpreted langs (Python, Julia)

**Use when:**
- Development/iteration
- Precise timing needed
- Comparing compiled binaries

## All Available Commands

```bash
# Container benchmarks
make bench BENCHMARK=fibonacci LANGUAGE=ruchy-compiled
make bench-all                    # Run all benchmarks (not implemented yet)

# Local benchmarks
make extract-binaries             # Extract from containers
make bench-cli                    # Run bashrs benchmarks

# Build
make build-images                 # Build all
make build-fibonacci              # Build fibonacci only
make build-primes                 # Build primes only
```

## Example Session

```bash
# 1. Build images
make build-fibonacci

# 2. Run container benchmark
docker run --rm ruchy-compiled:fibonacci
# Output:
# STARTUP_TIME_US: 8234
# COMPUTE_TIME_US: 21000
# RESULT: 9227465

# 3. Extract for local benchmarking
make extract-binaries

# 4. Run local benchmark with timing
time ./bin/fibonacci_ruchy_compiled

# 5. Run statistical benchmark
make bench-cli
# Output: JSON with mean, std dev, confidence intervals
```

## Performance Expectations

### Fibonacci(35) Compute Time

**Container (docker run):**
- C: ~11ms total (8ms compute)
- Rust: ~22ms total (19ms compute)
- Ruchy-compiled: ~22ms total (21ms compute)
- Python: ~697ms total (610ms compute)

**Local (direct execution):**
- C: ~8ms
- Rust: ~19ms
- Ruchy-compiled: ~21ms
- Python: ~610ms

### Primes (100,000 limit)

**Container:**
- C: ~15ms total
- Rust: ~28ms total
- Ruchy-compiled: ~30ms total
- Python: ~850ms total

## Troubleshooting

**Images not found:**
```bash
make build-fibonacci
```

**Extract-binaries fails:**
```bash
# Build images first
make build-images
# Then extract
make extract-binaries
```

**bashrs not found:**
```bash
# Install bashrs
cd ../bashrs && cargo install --path rash
# Or
make install-tools
```

**Permission denied on binaries:**
```bash
chmod +x bin/*
```
