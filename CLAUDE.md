# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**ruchy-docker** is a scientifically rigorous Docker runtime benchmarking framework that measures and compares programming language performance in containerized environments. The goal is to demonstrate Ruchy's world-class performance for compute-intensive workloads.

**Status**: Specification Complete (v2.0.0) - Ready for Implementation
**Quality Standard**: Peer-reviewed, production-grade, EXTREME TDD

## Development Workflow

**CRITICAL**: Always work on `main` branch. We don't do branching.

### Build and Development

```bash
# Local development
make dev                # Build and start docker-compose

# Build all Docker images (16 containers: 2 benchmarks × 8 languages)
make build-images

# Build single benchmark image
docker build -f docker/ruchy-transpiled/fibonacci.Dockerfile -t ruchy-transpiled:fibonacci .
docker build -f docker/deno/fibonacci.Dockerfile -t deno:fibonacci .
```

### Running Benchmarks

```bash
# Run single benchmark
make bench BENCHMARK=fibonacci LANGUAGE=ruchy-transpiled

# Run full benchmark suite (all 16 containers)
make bench-all

# Run with instrumented timing
docker run --rm ruchy-transpiled:fibonacci
# Output: STARTUP_TIME_US: 8234
#         COMPUTE_TIME_US: 23891
#         RESULT: 9227465
```

### Documentation & Charts

```bash
# Preview proportional ASCII charts (no file modification)
make charts

# Update README.md charts (creates timestamped backup)
make update-readme
# Creates: README.md.backup-2025-11-07T05-04-56-810Z
# Replaces content between <!-- AUTO-GENERATED-CHART: ... --> markers
```

**Chart Generation**:
- Uses Deno TypeScript scripts in `scripts/`
- Mathematically proportional scaling (no manual errors)
- Marker-based safe replacement
- Automatic backups before modification

### Quality Gates (Andon Cord - Fail-Fast)

**ALL quality gates MUST pass before any commit**:

```bash
# Run all quality gates
make quality            # format → lint → test → coverage → mutation → complexity

# Individual gates
make format             # cargo fmt --check, black --check src/
make lint               # clippy -D warnings, pylint src/
make lint-makefile      # bashrs make lint Makefile
make test               # cargo test --all-features, pytest tests/
make coverage           # ≥85% or FAIL (cargo-llvm-cov, pytest-cov)
make mutation           # ≥85% mutation score or FAIL (cargo-mutants, mutmut)
```

**Quality Standards (Zero Tolerance for Defects)**:
- Test Coverage: ≥85% (line coverage)
- Mutation Score: ≥85% (mutants caught by tests)
- Cyclomatic Complexity: ≤15
- Cognitive Complexity: ≤20
- SATD Violations: 0

### Testing

```bash
# Run all tests
make test

# Run specific test suites
cargo test --test unit_tests
pytest tests/unit/test_metrics.py

# Run single test
cargo test test_parse_benchmark_result
pytest tests/integration/test_fibonacci.py::test_ruchy_transpiled

# Coverage with HTML report
cargo llvm-cov --html --open
pytest --cov=src --cov-report=html

# Mutation testing
cargo mutants --output mutants.txt
mutmut run
```

### Deployment (Results Publication)

```bash
# Deploy results (after quality gates pass)
make deploy             # Runs: quality → bench-all → generate-report → publish-results
```

## Architecture

### High-Level Structure

The project follows a **test framework + benchmark matrix** architecture:

```
ruchy-docker/
├── benchmarks/        # Benchmark implementations (8 benchmarks)
│   ├── fibonacci/     # BENCH-007: Recursive Fibonacci (fib(35) = 9,227,465)
│   ├── primes/        # BENCH-008: Prime sieve (100K primes)
│   ├── array-sum/     # BENCH-005: Array sum (10M elements)
│   ├── matrix-mult/   # BENCH-006: Matrix multiply (128×128)
│   ├── hash-map/      # BENCH-009: HashMap ops (100K inserts/lookups)
│   ├── file-io/       # BENCH-010: I/O (100MB, fio-based)
│   ├── startup/       # BENCH-012: Startup time ("hello world")
│   └── http-server/   # BENCH-013: HTTP server (10K reqs, 100 concurrent)
├── docker/            # Dockerfiles per language/benchmark
│   ├── ruchy-transpiled/  # Ruchy → Rust transpilation
│   ├── ruchy-compiled/    # Ruchy native compilation
│   ├── go/, rust/, cpp/, python/, julia/, deno/
├── src/               # Test framework and orchestration
│   ├── runner/        # Docker benchmark runner (orchestrates container execution)
│   ├── metrics/       # Metrics collection (startup time, compute time, wall-clock)
│   ├── analyzer/      # Statistical analysis (geomean, arithmetic mean, harmonic mean, MAD outliers)
│   └── reporter/      # Results reporting (JSON, Markdown, HTML with violin plots)
├── tests/             # EXTREME TDD test suite
│   ├── unit/          # Unit tests (100+ tests, ≥85% coverage)
│   ├── integration/   # Integration tests (16 tests = 2 benchmarks × 8 languages)
│   ├── property/      # Property-based tests (proptest, Hypothesis)
│   ├── mutation/      # Mutation test config (≥85% mutation score)
│   └── fuzz/          # Fuzz testing (cargo-fuzz, 1M executions)
└── scripts/           # Automation scripts
    ├── build-all-images.sh
    ├── run-benchmark.sh
    ├── run-all-benchmarks.sh
    └── capture-environment.sh
```

### Benchmark Matrix

**Total**: 2 benchmarks × 8 languages = **16 Docker containers**

**Languages**:
1. Ruchy (transpiled) - Transpiled to Rust, compiled with rustc
2. Ruchy (compiled) - Native Ruchy compilation
3. C (gcc 13, -O3)
4. Rust (rustc 1.83, opt-level=3)
5. Go (go 1.23)
6. Python (CPython 3.12)
7. Julia (1.10, JIT)
8. Deno (2.1.4, TypeScript with V8 JIT)

**Benchmarks Implemented**:
1. Fibonacci - BENCH-007: Recursive Fibonacci (fib(35) = 9,227,465)
2. Primes - BENCH-008: Prime sieve (100K primes using Sieve of Eratosthenes)

### Key Design Patterns

#### 1. Instrumented Measurement (Genchi Genbutsu)
All benchmarks use **embedded instrumentation** to isolate application startup from Docker overhead:

```rust
fn main() {
    let t0 = Instant::now();
    // Initialization (imports, allocations)
    let t1 = Instant::now();
    let startup_time = t1.duration_since(t0);

    // Benchmark computation
    let result = benchmark_function();
    let t2 = Instant::now();
    let compute_time = t2.duration_since(t1);

    // Standardized output format
    println!("STARTUP_TIME_US: {}", startup_time.as_micros());
    println!("COMPUTE_TIME_US: {}", compute_time.as_micros());
    println!("RESULT: {}", result);
}
```

**Why**: The `time docker run` approach conflates Docker daemon communication, container init, and application startup. Instrumented measurement provides precise, low-noise data (Felter et al. USENIX ATC 2015).

#### 2. Multi-Stage Docker Builds
All Dockerfiles use **multi-stage builds** for minimal runtime images:

```dockerfile
# Stage 1: Build (includes compilers, build tools)
FROM rust:1.83 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/benchmark

# Stage 2: Runtime (distroless, <10MB)
FROM gcr.io/distroless/static-debian12:latest
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/benchmark /benchmark
ENTRYPOINT ["/benchmark"]
```

**Targets**: <10MB images, <10ms startup time

**Deno Approach**: Deno uses `deno compile` to create standalone executables that bundle the V8 runtime and TypeScript code into a single binary. This approach:
- Eliminates Node.js-style `node_modules` complexity
- Pre-compiles TypeScript to optimized V8 bytecode
- Creates portable binaries (~100MB including V8 runtime)
- Uses `performance.now()` for high-resolution timing (similar to Rust's `Instant::now()`)

#### 3. Statistical Rigor (Kaizen)
The framework reports **three aggregation metrics** (not just geometric mean):

1. **Geometric Mean**: Traditional benchmark metric `(∏ speedups)^(1/n)`
2. **Arithmetic Mean**: Total CPU time `(∑ times) / n`
3. **Harmonic Mean**: Average speedup `n / (∑ 1/speedups)`

**Outlier Detection**: Uses **Median Absolute Deviation (MAD)** instead of arbitrary thresholds:
```python
median = np.median(measurements)
mad = np.median(np.abs(measurements - median))
threshold = median + 3 * 1.4826 * mad
```

**Visualization**: Box plots + violin plots showing full distributions (Jidoka - making problems visible)

#### 4. I/O Benchmark Cache Control
File I/O benchmarks (BENCH-010) require **mandatory cache clearing** to avoid measuring RAM speed:

```bash
# Clear page cache before EACH run
sudo sync
sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches'
sleep 1
docker run --rm benchmark:file-io
```

Uses **fio** (Flexible I/O Tester) with `direct=1` (O_DIRECT) to bypass page cache.

#### 5. JIT Warmup Strategy
Different warmup strategies per language type:
- **Compiled (C, Rust, Go, Ruchy)**: 3 warmup iterations (disk cache)
- **Interpreted (Python)**: 3 warmup iterations (bytecode compilation)
- **JIT (Julia)**: **Steady-state detection** - run until variance stabilizes (<5% coefficient of variation over 5 runs), min 5 iterations, max 50

### Output Format Standards

All benchmarks must produce **standardized output** for automated parsing:

**Microbenchmarks**:
```
STARTUP_TIME_US: 8234
COMPUTE_TIME_US: 23891
RESULT: 9227465
```

**I/O Benchmarks**:
```
WRITE_THROUGHPUT_MBS: 245.67
READ_THROUGHPUT_MBS: 312.89
```

**HTTP Server** (via wrk load testing):
```
REQUESTS_PER_SEC: 12543.21
LATENCY_P50_MS: 7.89
LATENCY_P95_MS: 12.34
LATENCY_P99_MS: 18.92
```

## Toyota Way Principles

This project applies **Toyota Way** principles throughout:

1. **Genchi Genbutsu** (Go and See): Instrumented measurement, perf stat, root cause analysis
2. **Kaizen** (Continuous Improvement): Multiple aggregation metrics, enhanced I/O methodology, steady-state JIT warmup
3. **Jidoka** (Automation with Human Touch): Andon Cord quality gates (fail-fast), distribution visualizations

### Andon Cord

Any developer can stop the pipeline if quality gates fail. **NEVER** bypass quality gates.

## Benchmark Implementation Requirements

When implementing new benchmarks, follow these requirements:

1. **Identical Algorithm**: Same logic across all 7 languages
2. **Validated Output**: All implementations must produce identical, verifiable results
3. **Deterministic**: No random number generators (unless seeded identically)
4. **Single-Threaded**: No parallelism (unless part of benchmark goal)
5. **Instrumented**: Embed timing code in `main()` to measure startup vs compute
6. **Standardized Output**: Use the output format conventions above

### Example: Adding a New Benchmark

1. Create benchmark directory: `benchmarks/my-benchmark/`
2. Implement in all 7 languages with identical algorithm
3. Create Dockerfiles in `docker/{language}/my-benchmark.Dockerfile`
4. Add integration tests in `tests/integration/test_my_benchmark.rs`
5. Validate output matches expected result
6. Run quality gates: `make quality`
7. Add to benchmark matrix in specification

## Environment Requirements

### System Configuration (Required for Reproducibility)

Before running benchmarks, configure the system:

```bash
# Set CPU governor to performance (not powersave)
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Disable swap
sudo swapoff -a

# For I/O benchmarks, record filesystem type
df -T /var/lib/docker | tail -1 | awk '{print $2}'  # ext4, xfs, btrfs

# Capture environment
./scripts/capture-environment.sh > results/environment.json
```

### Dependencies

- Docker v24.0+
- Rust v1.83+ (locked via rust-toolchain.toml)
- Go v1.23+
- Python v3.12+
- GCC v13+
- Julia v1.10+
- Deno v2.1.4+ (TypeScript runtime with V8)
- perf (for CPU-level performance counters)
- fio (for I/O benchmarking)
- wrk (for HTTP load testing)

## Documentation

**Primary Specification**: `docs/specifications/docker-runtime-benchmarking-spec.md` (1,321 lines)
- Complete methodology
- 10 peer-reviewed citations
- Statistical rigor
- EXTREME TDD strategy

**Peer Review Response**: `docs/specifications/PEER_REVIEW_RESPONSE.md` (452 lines)
- Critical review by Gemini AI (Nov 5, 2025)
- 7 major improvements implemented
- Toyota Way principles

**Quick Reference**: `docs/specifications/README.md`

## Academic Foundation

This project is based on 10 peer-reviewed papers:

1. Blackburn et al. (OOPSLA 2007) - Benchmark evaluation methodology
2. Felter et al. (USENIX ATC 2015) - Container performance comparison
3. Gregg (2020) - BPF Performance Tools
4. Kalibera & Jones (PLDI 2013) - JIT benchmarking rigor
5. Fleming & Wallace (1986) - Benchmark result summarization
6. Eyerman & Eeckhout (IEEE CL 2018) - Geometric mean critique
7. Akinshin (2021) - Performance analysis
8. Chen & Patterson (1994) - I/O performance evaluation
9. Lockwood (2016) - I/O benchmarking best practices
10. Mytkowicz et al. (ASPLOS 2009) - Avoiding benchmark pitfalls

## Related Projects

- **ruchy-lambda** (`../ruchy-lambda`): AWS Lambda cold start benchmarking (10.09ms, 2.17x faster than custom runtimes)
- **ruchy-book** (`../ruchy-book`): Local execution benchmarking (15.12x faster than Python, 82% of C performance)

## Scope Acknowledgment

Performance claims are explicitly scoped to **compute-intensive workloads**. Microbenchmarks test specific computational kernels and may not reflect I/O-interleaved or network-bound workloads. The HTTP server macrobenchmark (BENCH-013) validates performance in a more realistic scenario.
