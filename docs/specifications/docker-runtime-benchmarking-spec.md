# Docker Runtime Benchmarking Specification
## Ruchy Performance Validation in Containerized Environments

**Version**: 2.0.0
**Status**: SPECIFICATION (Peer-Reviewed)
**Project**: ruchy-docker
**Goal**: Demonstrate Ruchy compiled/transpiled binary achieves world-class performance for compute-intensive workloads in containerized environments, with scientifically rigorous measurement and transparent methodology

**Peer Review**: Gemini AI Assistant (November 5, 2025)
**Review Framework**: Toyota Way principles (Genchi Genbutsu, Kaizen, Jidoka)

---

## 1. Executive Summary

### 1.1 Project Objectives

This project establishes a **scientifically rigorous, production-grade benchmarking framework** to measure and compare programming language runtimes in Docker containerized environments. Building on the methodologies from **ruchy-lambda** (AWS Lambda cold start benchmarking) and **ruchy-book** (local execution benchmarking), this project adds Docker-specific metrics to demonstrate Ruchy's performance superiority in containerized deployments.

**Key Goals**:
1. Measure Docker **cold start time** with isolated, instrumented methodology
2. Measure **total execution time** (cold start + computation) across diverse workload types
3. Minimize Docker image size (targeting distroless/empty base images)
4. Compare Ruchy against Go, Rust, Python, C++, and Julia in Docker
5. Demonstrate **world-class performance** for compute-intensive containerized workloads with empirical evidence
6. Include **real-world application benchmarks** (HTTP server) alongside microbenchmarks
7. Implement EXTREME TDD with 85%+ coverage, mutation testing, property-based testing
8. Use PMAT for quality enforcement and roadmap management

**Scope Acknowledgment** (Genchi Genbutsu):
The performance claims in this specification are explicitly scoped to **compute-intensive workloads** as represented by the chosen benchmarks. Microbenchmarks test specific computational kernels (recursion, loops, memory access) and may not reflect the performance of I/O-interleaved, network-bound, or memory-access-patterned workloads typical of production containerized services. To address this limitation, the benchmark suite includes a real-world HTTP server benchmark (BENCH-013) to validate performance in a more representative scenario.

### 1.2 Success Criteria

- **Performance**: Ruchy transpiled/compiled achieves top-tier performance across 8 benchmarks (reported via multiple aggregation metrics)
- **Container Size**: <10MB Docker images using distroless/empty base images
- **Cold Start**: <50ms application startup time (instrumented, isolated from Docker overhead)
- **Quality**: 85%+ test coverage, 85%+ mutation score, zero critical defects
- **Statistical Rigor**: Results reported with geometric mean, arithmetic mean, harmonic mean, and full distribution visualizations
- **Reproducibility**: All benchmarks automated via Makefile, runnable on any system with Docker, with complete environment specifications

---

## 2. Architecture Overview

### 2.1 System Components

```
ruchy-docker/
├── benchmarks/           # Benchmark implementations
│   ├── fibonacci/        # BENCH-007: Recursive Fibonacci (microbenchmark)
│   ├── primes/           # BENCH-008: Prime number sieve (microbenchmark)
│   ├── array-sum/        # BENCH-005: Array summation (microbenchmark)
│   ├── matrix-mult/      # BENCH-006: Matrix multiplication (microbenchmark)
│   ├── hash-map/         # BENCH-009: HashMap operations (microbenchmark)
│   ├── file-io/          # BENCH-010: File I/O operations (microbenchmark)
│   ├── startup/          # BENCH-012: Runtime startup time (microbenchmark)
│   └── http-server/      # BENCH-013: HTTP "hello world" server (macrobenchmark)
├── docker/               # Dockerfiles per language/benchmark
│   ├── ruchy-transpiled/ # Ruchy → Rust transpilation
│   ├── ruchy-compiled/   # Ruchy native compilation
│   ├── go/
│   ├── rust/
│   ├── cpp/
│   ├── python/
│   └── julia/
├── src/                  # Test framework and orchestration
│   ├── runner/           # Docker benchmark runner
│   ├── metrics/          # Metrics collection and aggregation
│   ├── analyzer/         # Statistical analysis
│   └── reporter/         # Results reporting (JSON, Markdown, HTML)
├── tests/                # Test suite (EXTREME TDD)
│   ├── unit/
│   ├── integration/
│   ├── property/         # Property-based tests
│   ├── mutation/         # Mutation test configuration
│   └── fuzz/             # Fuzz testing targets
├── docs/
│   ├── specifications/   # This file
│   ├── methodology/      # Scientific methodology documentation
│   └── results/          # Benchmark results archive
└── Makefile              # Build, test, benchmark, deploy targets
```

### 2.2 Benchmark Matrix

#### Microbenchmarks (Compute-Intensive)
| Benchmark | Description | Input Size | Baseline | Source |
|-----------|-------------|------------|----------|--------|
| BENCH-007 | Recursive Fibonacci | fib(35) | 9,227,465 | ruchy-book |
| BENCH-008 | Prime Sieve | 100,000 primes | 1,299,709 | ruchy-book |
| BENCH-005 | Array Sum | 10M elements | 50,000,005,000,000 | ruchy-book |
| BENCH-006 | Matrix Multiply | 128x128 matrices | Validated checksum | ruchy-book |
| BENCH-009 | HashMap Operations | 100K inserts/lookups | Throughput | ruchy-book |
| BENCH-010 | File I/O (fio-based) | 100MB sequential read/write | Throughput (MB/s) | Enhanced |
| BENCH-012 | Startup Time | Minimal "hello world" | <10ms target | ruchy-book |

#### Macrobenchmark (Real-World Application)
| Benchmark | Description | Workload | Metrics | Rationale |
|-----------|-------------|----------|---------|-----------|
| BENCH-013 | HTTP Server | 10,000 requests (100 concurrent) | Requests/sec, p50/p95/p99 latency, cold start | Validates I/O-interleaved, syscall-heavy workload (Blackburn et al. OOPSLA 2007) |

**Total**: 8 benchmarks × 7 languages = **56 Docker containers**

**Benchmark Type Justification** (Kaizen):
- **Microbenchmarks (7)**: Test specific computational kernels, memory access patterns, and language runtime efficiency
- **Macrobenchmark (1)**: Tests realistic, I/O-interleaved workload with network syscalls, concurrency, and memory allocation patterns representative of containerized services
- **Coverage**: Addresses criticism from Mytkowicz et al. (ASPLOS 2009) regarding unrepresentative benchmarks

### 2.3 Language Comparison Matrix

| Language | Mode | Compiler/Interpreter | Optimization | Base Image |
|----------|------|---------------------|--------------|------------|
| Ruchy | Transpiled | Rust (rustc) | `-C opt-level=3` | distroless/static |
| Ruchy | Compiled | Native codegen | Maximum | distroless/static |
| C | Native | gcc 13 | `-O3 -march=x86-64` | distroless/static |
| Rust | Native | rustc 1.83 | `-C opt-level=3` | distroless/static |
| Go | Native | go 1.23 | Default | distroless/static |
| Python | Interpreted | CPython 3.12 | Bytecode | distroless/python |
| Julia | JIT | Julia 1.10 | JIT + LLVM | distroless/base |
| C++ | Native | g++ 13 | `-O3 -march=x86-64` | distroless/static |

---

## 3. Metrics and Measurements

### 3.1 Primary Metrics

#### A. Application Startup Time (Instrumented)
**Definition**: Time from application entry point (`main()`) to first computation, measured via instrumentation

**Measurement Method** (Genchi Genbutsu - Go and See):
```rust
// Embedded instrumentation in benchmark code
fn main() {
    let t0 = Instant::now();
    // Initialization code (imports, allocations, etc.)
    let t1 = Instant::now();
    let startup_time = t1.duration_since(t0);

    // Benchmark computation
    let result = benchmark_function();
    let t2 = Instant::now();
    let compute_time = t2.duration_since(t1);

    println!("STARTUP_TIME_US: {}", startup_time.as_micros());
    println!("COMPUTE_TIME_US: {}", compute_time.as_micros());
    println!("RESULT: {}", result);
}
```

**Rationale** (Felter et al. USENIX ATC 2015):
The `time docker run` approach conflates Docker daemon communication, container initialization (cgroups, namespaces), and application startup. Instrumented measurement isolates application startup from containerization overhead, providing precise, low-noise measurements.

**Components Measured**:
- **Runtime Initialization**: Language runtime startup (interpreter/JIT/stdlib loading)
- **Import Resolution**: Module/package imports
- **Global Initialization**: Global variables, static allocations
- **First Instruction**: Time to begin computation

**Components NOT Measured** (isolated):
- Docker client/daemon communication
- Container creation (cgroups, namespaces, filesystem)
- Image layer loading

**Target**: <10ms for Ruchy (instrumented application startup)

**Additional Container-Level Measurement**:
For end-to-end Docker overhead analysis, also measure:
```bash
perf stat -e task-clock,cycles,instructions,cache-misses docker run --rm benchmark:tag
```
This provides CPU-level performance counters (Gregg 2020, BPF Performance Tools) for deep analysis.

#### B. Total Execution Time (Wall-Clock + Instrumented)
**Definition**: End-to-end execution time including containerization overhead and computation

**Measurement Method**:
```bash
# Wall-clock measurement (includes Docker overhead)
/usr/bin/time -f "WALL_TIME_MS: %E\nMAX_RSS_KB: %M" docker run --rm benchmark:tag

# Plus perf stat for CPU-level insights (Gregg 2020)
perf stat -e cycles,instructions,cache-references,cache-misses,branches,branch-misses \
  docker run --rm benchmark:tag 2>&1 | tee perf_output.txt
```

**Dual-Level Reporting**:
1. **Instrumented Time**: Application startup + computation (from instrumented output)
2. **Wall-Clock Time**: Total time including Docker overhead
3. **Docker Overhead**: Wall-clock - (Startup + Compute)

**Example Output**:
```
STARTUP_TIME_US: 8234
COMPUTE_TIME_US: 23891
RESULT: 9227465
WALL_TIME_MS: 0:00.045
MAX_RSS_KB: 8192
```

**Target**: Top-tier performance across all 8 benchmarks (multiple aggregation metrics)

#### C. Docker Image Size
**Definition**: Compressed image size as reported by `docker images`

**Measurement Method**:
```bash
docker images --format "{{.Repository}}:{{.Tag}} {{.Size}}"
```

**Target**: <10MB per image (distroless/static preferred)

#### D. Memory Usage
**Definition**: Peak RSS memory during benchmark execution

**Measurement Method**:
```bash
docker stats --no-stream --format "{{.MemUsage}}"
```

**Target**: <64MB for Ruchy benchmarks

### 3.2 Secondary Metrics

- **Startup Isolation**: Pure startup time (BENCH-012) with no computation
- **Computation Isolation**: Execution time minus startup time
- **Image Build Time**: Time to build Docker image from source
- **Binary Size**: Size of compiled binary inside container
- **Image Layers**: Number of layers in final image (fewer = better)

### 3.3 Statistical Rigor (Enhanced - Kaizen)

Based on **DLS 2016** methodology with enhancements from peer review:

#### Warmup Strategy
- **Interpreted Languages (Python)**: 3 warmup iterations (bytecode compilation, module caching)
- **JIT Languages (Julia)**: **Steady-state detection** (Kalibera & Jones PLDI 2013)
  - Run iterations until performance variance stabilizes (<5% coefficient of variation over 5 consecutive runs)
  - Minimum 5 warmup iterations, maximum 50 iterations
  - Discard all warmup data before measurement phase
- **Compiled Languages (C, Rust, Go, Ruchy)**: 3 warmup iterations (disk cache, page cache)

#### Measurement Phase
- **Iterations**: 10 measurement runs per benchmark
- **Environment Reset**: Fresh Docker container for each run (no cross-contamination)
- **System Isolation**: No concurrent workloads, network disabled (except BENCH-013)

#### Aggregation Metrics (Fleming & Wallace 1986, Eyerman & Eeckhout 2018)
Report **all three** aggregation methods to provide complete picture:

1. **Geometric Mean**: Traditional metric from DLS 2016
   - Formula: `(∏ speedups)^(1/n)`
   - Use: Relative performance comparison
   - Limitation: Can obscure individual benchmark differences

2. **Arithmetic Mean**: Total work metric
   - Formula: `(∑ times) / n`
   - Use: Average execution time across benchmarks
   - Benefit: Intuitive, reflects total CPU time

3. **Harmonic Mean**: Speedup metric (for speedup ratios only)
   - Formula: `n / (∑ 1/speedups)`
   - Use: Average speedup across benchmarks
   - Benefit: Appropriate for rate-based metrics

**Primary Reporting**: Individual benchmark results + all three means

#### Outlier Detection (Robust Statistics)
Replace fixed 10% threshold with **Median Absolute Deviation (MAD)**:
```python
median = np.median(measurements)
mad = np.median(np.abs(measurements - median))
threshold = median + 3 * 1.4826 * mad  # 1.4826 scales MAD to match stddev
outliers = measurements > threshold
```
- **Rationale**: MAD is robust to non-normal distributions (Akinshin 2021)
- **Action**: Flag outliers but do NOT discard (report separately for transparency)

#### Confidence Intervals
- **95% Confidence Intervals**: Bootstrap method (10,000 resamples)
- **Visualization**: Box plots + violin plots showing full distributions (Jidoka)

#### Reproducibility
- **Multi-day Runs**: Execute full benchmark suite on 3 separate days
- **Multi-machine Validation**: Validate on at least 2 different hardware configurations
- **Statistical Tests**: Mann-Whitney U test for pairwise comparisons (non-parametric)

**Tool**: bashrs bench v6.25.0 + custom statistical analysis (Python scipy/numpy)

---

## 4. Docker Containerization Strategy

### 4.1 Multi-Stage Build Pattern

All Dockerfiles follow this pattern:

```dockerfile
# Stage 1: Build
FROM rust:1.83 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/benchmark

# Stage 2: Runtime (distroless/static)
FROM gcr.io/distroless/static-debian12:latest
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/benchmark /benchmark
ENTRYPOINT ["/benchmark"]
```

**Benefits**:
- Minimal runtime image (no build tools)
- Small attack surface (distroless = no shell, package manager, etc.)
- Optimized for size and security
- Fast cold start (fewer layers to load)

### 4.2 Base Image Strategy

| Base Image | Use Case | Size | Shell | Package Manager |
|------------|----------|------|-------|-----------------|
| `distroless/static` | Static binaries (Ruchy, Rust, Go, C, C++) | ~2MB | No | No |
| `distroless/python3-debian12` | Python runtime | ~50MB | No | No |
| `distroless/base-debian12` | Julia (requires glibc) | ~20MB | No | No |
| `scratch` | Fully static binaries (alternative) | ~0MB | No | No |

**Preference**: `distroless/static` for all static binaries, `scratch` if smaller is needed

### 4.3 Optimization Techniques

#### Static Linking (Ruchy, Rust, Go, C, C++)
```bash
# Rust
cargo build --target x86_64-unknown-linux-musl --release

# Go
CGO_ENABLED=0 go build -ldflags="-s -w"

# C/C++
gcc -static -O3 -march=x86-64 -o benchmark benchmark.c
strip benchmark
```

#### Symbol Stripping
```bash
strip --strip-all /path/to/binary
# Removes debug symbols, reduces size by 10-30%
```

#### Compression
```bash
# UPX compression (optional, may increase startup time)
upx --best --lzma /path/to/binary
```

#### Link-Time Optimization (LTO)
```toml
# Cargo.toml
[profile.release]
lto = "fat"
codegen-units = 1
opt-level = 3
```

---

## 5. Benchmark Implementation Standards

### 5.1 Code Requirements

Each benchmark must:
1. **Identical Algorithm**: Same logic across all languages
2. **No Standard Library Shortcuts**: Avoid language-specific optimizations (e.g., NumPy)
3. **Validated Output**: All benchmarks must produce identical, verifiable results
4. **Deterministic**: No random number generators (unless seeded identically)
5. **Single-Threaded**: No parallelism (unless part of benchmark goal)
6. **Minimal I/O**: Computation-focused (except BENCH-010)

### 5.2 Example: Fibonacci Benchmark

**Ruchy Implementation** (`benchmarks/fibonacci/fibonacci.ruchy`):
```ruby
fn fibonacci(n: i32) -> i32 {
  if n <= 1 {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
  let result = fibonacci(35);
  println!("Result: {}", result);
}
```

**Validation**:
- Input: `fibonacci(35)`
- Expected Output: `9227465`
- All languages must produce this exact result

**Dockerfile** (`docker/ruchy-transpiled/fibonacci.Dockerfile`):
```dockerfile
FROM rust:1.83 AS builder
WORKDIR /build
COPY benchmarks/fibonacci/fibonacci.ruchy .
RUN ruchy transpile fibonacci.ruchy --output fibonacci.rs
RUN rustc -C opt-level=3 --target x86_64-unknown-linux-musl fibonacci.rs
RUN strip fibonacci

FROM gcr.io/distroless/static-debian12:latest
COPY --from=builder /build/fibonacci /fibonacci
ENTRYPOINT ["/fibonacci"]
```

### 5.3 File I/O Benchmark Methodology (BENCH-010) - Enhanced

**Critical Requirements** (Chen & Patterson 1994, Lockwood 2016):

#### Cache Control (Genchi Genbutsu)
File I/O benchmarks are highly sensitive to filesystem caching. Without proper cache control, consecutive reads measure RAM speed, not disk I/O.

**Mandatory Pre-Benchmark Cache Clearing**:
```bash
# Clear page cache, dentries, and inodes (requires root)
sync
echo 3 > /proc/sys/vm/drop_caches

# Verify cache cleared
free -m  # Check available memory increased
```

**Integration into Benchmark Runner**:
```bash
#!/bin/bash
# scripts/run-io-benchmark.sh

for i in {1..10}; do
    # Clear caches before EACH run
    sudo sync
    sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches'

    # Brief delay for cache clearing to complete
    sleep 1

    # Run benchmark
    docker run --rm benchmark:file-io
done
```

#### I/O Tool: fio (Flexible I/O Tester)
Replace custom file I/O implementations with **fio** for scientific validity:

**fio Configuration** (`benchmarks/file-io/fio-sequential.job`):
```ini
[global]
ioengine=sync
direct=1          # Bypass page cache (O_DIRECT)
size=100M
numjobs=1
time_based=0

[sequential-read]
rw=read
bs=4k
filename=/tmp/testfile

[sequential-write]
rw=write
bs=4k
filename=/tmp/testfile
```

**Metrics Collected**:
- **Sequential Read Throughput**: MB/s
- **Sequential Write Throughput**: MB/s
- **IOPS**: Operations per second
- **Latency**: p50, p95, p99 (μs)

**Docker Integration**:
```dockerfile
FROM debian:12-slim AS builder
RUN apt-get update && apt-get install -y fio
COPY fio-sequential.job /fio.job

ENTRYPOINT ["fio", "/fio.job", "--output-format=json"]
```

#### Filesystem Specification
The underlying filesystem significantly impacts I/O performance. This must be documented:

**Required Environment Metadata**:
```bash
# Capture filesystem type
df -T /var/lib/docker | tail -1 | awk '{print $2}'  # e.g., ext4, xfs, btrfs

# Capture mount options
mount | grep /var/lib/docker
```

**Reporting**: Include filesystem type in benchmark results table

#### Alternative: In-Application I/O Benchmark
If using custom implementations instead of fio, follow these guidelines:

```rust
use std::fs::File;
use std::io::{Write, Read};
use std::os::unix::fs::OpenOptionsExt;

fn benchmark_io() {
    let path = "/tmp/benchmark_testfile";

    // Write test (O_DIRECT bypasses page cache)
    let t0 = Instant::now();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .custom_flags(libc::O_DIRECT)  // Bypass cache
        .open(path)
        .unwrap();
    file.write_all(&vec![0u8; 100 * 1024 * 1024]).unwrap();
    file.sync_all().unwrap();  // Ensure written to disk
    let write_time = t0.elapsed();

    // Clear caches externally here (via runner script)

    // Read test
    let t1 = Instant::now();
    let mut file = File::open(path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let read_time = t1.elapsed();

    println!("WRITE_THROUGHPUT_MBS: {:.2}", 100.0 / write_time.as_secs_f64());
    println!("READ_THROUGHPUT_MBS: {:.2}", 100.0 / read_time.as_secs_f64());
}
```

**Validation**: All languages must report throughput within 10% (filesystem-limited, not language-limited)

### 5.4 HTTP Server Benchmark Methodology (BENCH-013)

**Implementation**: Minimal "hello world" HTTP server on port 8080

**Example (Rust)**:
```rust
use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("SERVER_READY");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!").unwrap();
    }
}
```

**Load Testing** (via `wrk`):
```bash
# Start server container in background
docker run -d --name bench-http -p 8080:8080 benchmark:http-server

# Wait for "SERVER_READY" output
docker logs -f bench-http | grep -m1 "SERVER_READY"

# Run load test
wrk -t 4 -c 100 -d 30s http://localhost:8080

# Collect metrics: Req/sec, Latency (p50, p95, p99)
```

### 5.5 Output Format

All benchmarks must output results in standardized format for automated parsing:

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

**HTTP Server**:
```
REQUESTS_PER_SEC: 12543.21
LATENCY_P50_MS: 7.89
LATENCY_P95_MS: 12.34
LATENCY_P99_MS: 18.92
```

---

## 6. Testing Strategy (EXTREME TDD)

### 6.1 Test-Driven Development Phases

Following **Toyota Way + XP Principles**:

#### RED Phase
1. Write failing test first
2. Run test suite (should fail)
3. Commit failing test

#### GREEN Phase
1. Write minimal code to pass test
2. Run test suite (should pass)
3. Commit passing code

#### REFACTOR Phase
1. Improve code quality
2. Ensure all tests still pass
3. Commit refactored code

### 6.2 Test Coverage Targets

| Metric | Target | Tool |
|--------|--------|------|
| Line Coverage | ≥85% | cargo-llvm-cov / pytest-cov |
| Branch Coverage | ≥80% | cargo-llvm-cov |
| Mutation Score | ≥85% | cargo-mutants / mutmut |
| Complexity | ≤15 (cyclomatic) | PMAT |
| Cognitive Complexity | ≤20 | PMAT |

### 6.3 Test Types

#### Unit Tests
- **Scope**: Individual functions, modules
- **Location**: `tests/unit/`
- **Count Target**: 100+ tests
- **Example**:
  ```rust
  #[test]
  fn test_parse_benchmark_result() {
      let output = "Result: 9227465\nExecution time: 23.89ms";
      let result = parse_result(output).unwrap();
      assert_eq!(result.value, 9227465);
      assert_eq!(result.time_ms, 23.89);
  }
  ```

#### Integration Tests
- **Scope**: End-to-end Docker container execution
- **Location**: `tests/integration/`
- **Count Target**: 49 tests (7 benchmarks × 7 languages)
- **Example**:
  ```rust
  #[test]
  fn test_fibonacci_ruchy_transpiled_docker() {
      let output = run_docker_container("ruchy-transpiled:fibonacci").unwrap();
      assert_eq!(output.result, 9227465);
      assert!(output.cold_start_ms < 50.0);
      assert!(output.total_time_ms < 100.0);
  }
  ```

#### Property-Based Tests
- **Scope**: Invariants and mathematical properties
- **Location**: `tests/property/`
- **Tool**: proptest (Rust), Hypothesis (Python)
- **Example**:
  ```rust
  proptest! {
      #[test]
      fn test_fibonacci_monotonic(n in 2..40u32) {
          let fib_n = fibonacci(n);
          let fib_n_minus_1 = fibonacci(n - 1);
          prop_assert!(fib_n > fib_n_minus_1); // Fibonacci is monotonically increasing
      }
  }
  ```

#### Mutation Tests
- **Scope**: Test suite effectiveness
- **Location**: `tests/mutation/`
- **Tool**: cargo-mutants
- **Target**: 85% mutation score (85% of mutants caught by tests)
- **Example Mutants**:
  - Replace `+` with `-`
  - Replace `<` with `<=`
  - Replace `return n` with `return 0`

#### Fuzz Tests
- **Scope**: Edge cases and crashes
- **Location**: `tests/fuzz/`
- **Tool**: cargo-fuzz (libFuzzer)
- **Target**: 1M executions, zero crashes
- **Example**:
  ```rust
  fuzz_target!(|data: &[u8]| {
      if let Ok(s) = std::str::from_utf8(data) {
          let _ = parse_benchmark_result(s); // Should never crash
      }
  });
  ```

### 6.4 Quality Gates (Andon Cord)

All quality gates MUST pass before merge:

```makefile
quality: format lint test coverage mutation complexity

format:
	cargo fmt --check
	black --check src/

lint:
	cargo clippy -- -D warnings
	pylint src/

test:
	cargo test --all-features
	pytest tests/

coverage:
	cargo llvm-cov --fail-under-lines 85
	pytest --cov=src --cov-fail-under=85

mutation:
	cargo mutants -- --check
	mutmut run --fail-under 85

complexity:
	pmat analyze --max-cyclomatic 15 --max-cognitive 20
```

**Andon Cord Principle**: Any developer can stop the pipeline if quality gates fail.

---

## 7. PMAT Integration (Project Management)

### 7.1 PMAT Roadmap

PMAT (Project Management and Analysis Tool) enforces:
- **Milestone tracking**: Phases, deadlines, deliverables
- **Quality enforcement**: Coverage, mutation, complexity gates
- **Technical debt tracking**: SATD violations, TODO comments
- **Progress visibility**: Burndown charts, velocity metrics

**Example `.pmat.toml`**:
```toml
[project]
name = "ruchy-docker"
version = "1.0.0"

[quality]
min_coverage = 0.85
min_mutation_score = 0.85
max_cyclomatic_complexity = 15
max_cognitive_complexity = 20
max_satd_violations = 0

[roadmap]
milestones = [
    { name = "Phase 1: Test Framework", deadline = "2025-12-01" },
    { name = "Phase 2: Benchmark Implementation", deadline = "2025-12-15" },
    { name = "Phase 3: Docker Optimization", deadline = "2025-12-31" },
    { name = "Phase 4: Results Publication", deadline = "2026-01-15" }
]
```

### 7.2 Milestone Definitions

#### Phase 1: Test Framework (4 weeks)
- **Deliverables**:
  - Docker container orchestration framework
  - Metrics collection and aggregation
  - Test suite scaffolding (unit, integration, property)
  - CI/CD pipeline setup
- **Quality Gates**:
  - 85% test coverage
  - All integration tests passing
  - Zero critical bugs

#### Phase 2: Benchmark Implementation (4 weeks)
- **Deliverables**:
  - All 7 benchmarks implemented in 7 languages
  - 49 Dockerfiles created and validated
  - Output validation tests
  - Performance baseline established
- **Quality Gates**:
  - All benchmarks produce correct results
  - All Docker images build successfully
  - Image sizes <10MB (static languages)

#### Phase 3: Docker Optimization (2 weeks)
- **Deliverables**:
  - Distroless/static image conversions
  - Binary stripping and LTO optimization
  - Cold start time optimization
  - Memory usage profiling
- **Quality Gates**:
  - Cold start <50ms for Ruchy
  - Ruchy fastest geometric mean
  - Image sizes minimized

#### Phase 4: Results Publication (2 weeks)
- **Deliverables**:
  - Benchmark results report (Markdown, HTML)
  - Statistical analysis with confidence intervals
  - Performance comparison graphs
  - Public GitHub repository with reproducible results
- **Quality Gates**:
  - All results peer-reviewed
  - Reproducibility verified on 3 different machines
  - Documentation complete

---

## 8. Automation and Reproducibility

### 8.1 Makefile Targets

```makefile
# Development
.PHONY: dev
dev:
	cargo build
	docker-compose up -d

# Build all Docker images
.PHONY: build-images
build-images:
	./scripts/build-all-images.sh

# Run single benchmark
.PHONY: bench
bench:
	./scripts/run-benchmark.sh $(BENCHMARK) $(LANGUAGE)

# Run full benchmark suite
.PHONY: bench-all
bench-all:
	./scripts/run-all-benchmarks.sh

# Quality gates
.PHONY: quality
quality: format lint test coverage mutation complexity

# Testing
.PHONY: test
test:
	cargo test --all-features
	pytest tests/

# Coverage
.PHONY: coverage
coverage:
	cargo llvm-cov --html --open
	pytest --cov=src --cov-report=html

# Mutation testing
.PHONY: mutation
mutation:
	cargo mutants --output mutants.txt
	cat mutants.txt

# Deployment (results publication)
.PHONY: deploy
deploy: quality bench-all
	./scripts/generate-report.sh
	./scripts/publish-results.sh

# Clean
.PHONY: clean
clean:
	cargo clean
	docker system prune -af
	rm -rf target/ results/
```

### 8.2 CI/CD Pipeline

**GitHub Actions** (`.github/workflows/ci.yml`):

```yaml
name: CI

on: [push, pull_request]

jobs:
  quality-gates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install tools
        run: |
          cargo install cargo-llvm-cov cargo-mutants
          pip install pytest pytest-cov pylint black
      - name: Format check
        run: make format
      - name: Lint
        run: make lint
      - name: Test
        run: make test
      - name: Coverage
        run: make coverage
      - name: Mutation testing
        run: make mutation

  benchmark:
    runs-on: ubuntu-latest
    needs: quality-gates
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker images
        run: make build-images
      - name: Run benchmarks
        run: make bench-all
      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: benchmark-results
          path: results/
```

---

## 9. Results Reporting

### 9.1 Output Formats

#### JSON (Machine-Readable)
```json
{
  "benchmark": "fibonacci",
  "language": "ruchy-transpiled",
  "runs": [
    {
      "run_id": 1,
      "cold_start_ms": 12.34,
      "execution_time_ms": 23.89,
      "total_time_ms": 36.23,
      "result": 9227465,
      "image_size_mb": 3.2,
      "memory_mb": 8.5
    }
  ],
  "statistics": {
    "mean_total_time_ms": 36.45,
    "stddev_ms": 0.87,
    "min_ms": 35.12,
    "max_ms": 37.89,
    "confidence_interval_95": [35.89, 37.01]
  }
}
```

#### Markdown (Human-Readable)
```markdown
# Docker Benchmark Results

## Fibonacci (BENCH-007)

| Language | Cold Start | Execution | Total | Image Size | Memory |
|----------|-----------|-----------|-------|------------|--------|
| Ruchy (Transpiled) | 12.34ms | 23.89ms | 36.23ms | 3.2MB | 8.5MB |
| Ruchy (Compiled) | 11.98ms | 24.12ms | 36.10ms | 3.1MB | 8.3MB |
| C | 10.45ms | 12.73ms | 23.18ms | 2.8MB | 5.2MB |
| Rust | 13.67ms | 23.86ms | 37.53ms | 4.1MB | 9.1MB |
| Go | 18.34ms | 37.59ms | 55.93ms | 6.2MB | 12.3MB |
| Python | 89.12ms | 688.89ms | 778.01ms | 52.3MB | 45.6MB |
| Julia | 234.56ms | 182.72ms | 417.28ms | 78.4MB | 112.3MB |

**Winner**: Ruchy (Compiled) - 36.10ms total time
**Speedup vs Python**: 21.54x
**Speedup vs Go**: 1.55x
```

#### HTML (Web Dashboard)
- Interactive graphs (Chart.js)
- Sortable tables
- Confidence interval visualizations
- Historical trend analysis

---

## 10. Scientific Validity

### 10.1 Methodology Source

Based on peer-reviewed research:
- **Paper**: "Cross-Language Compiler Benchmarking: Are We Fast Yet?" (DLS 2016)
- **Authors**: Stefan Marr, Benoit Daloze, Hanspeter Mössenböck
- **Key Principles**:
  - Geometric mean for aggregation (prevents single-benchmark dominance)
  - Warmup iterations to eliminate cold start bias
  - Statistical rigor (confidence intervals, outlier detection)
  - Reproducibility (exact versions, build flags, hardware specs)

### 10.2 Transparency Requirements

All results must include:
1. **Hardware Specs**: CPU model, RAM, disk type, Docker version
2. **Software Versions**: Compiler versions, language versions, OS version
3. **Build Flags**: Exact compilation/transpilation flags
4. **Raw Data**: All 10 measurement runs (not just mean)
5. **Source Code**: All benchmark implementations public on GitHub
6. **Dockerfile**: All Docker configurations public
7. **Reproducibility Script**: One-command reproduction (`make bench-all`)

### 10.3 Fair Comparison

- **No Language-Specific Optimizations**: Avoid SIMD, vectorization, special libraries
- **Identical Algorithms**: Same logic across all languages
- **Same Input Data**: Validated, deterministic inputs
- **Same Hardware**: All benchmarks run on same machine
- **Same Docker Version**: Consistent container runtime
- **No Caching**: Fresh Docker runs (`docker run --rm`)

---

## 11. Deployment and Publication

### 11.1 GitHub Repository Structure

Public repository: `github.com/paiml/ruchy-docker`

- **README.md**: Project overview, quick start, results summary
- **docs/**: Full documentation (methodology, results, reproduction guide)
- **benchmarks/**: All source code (7 benchmarks × 7 languages)
- **docker/**: All Dockerfiles
- **src/**: Test framework and orchestration
- **results/**: Historical benchmark results (JSON, Markdown, HTML)
- **scripts/**: Automation scripts (build, test, benchmark)
- **.github/workflows/**: CI/CD configuration

### 11.2 Results Publication

- **Website**: https://ruchy.dev/docker-benchmarks
- **Blog Post**: Technical deep dive on optimization techniques
- **Academic Paper**: Submit to PL conference (OOPSLA, PLDI, etc.)
- **Conference Talk**: Present at DockerCon, KubeCon, or language-specific conferences

### 11.3 Continuous Benchmarking

- **Scheduled Runs**: Weekly benchmarks via GitHub Actions
- **Trend Analysis**: Track performance over time (regressions, improvements)
- **Automated Alerts**: Notify if Ruchy loses performance lead

---

## 12. Risk Management

### 12.1 Technical Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Docker overhead dominates measurement | High | Use BENCH-012 (startup) to isolate and subtract overhead |
| Non-deterministic results | High | Multiple runs, outlier detection, confidence intervals |
| Ruchy not fastest | Critical | Focus on optimization (LTO, stripping, distroless) |
| Image size >10MB | Medium | Aggressive stripping, distroless base images |
| CI/CD pipeline slow | Medium | Parallel builds, Docker layer caching |

### 12.2 Quality Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Coverage <85% | High | Andon Cord (block merge if coverage drops) |
| Mutation score <85% | High | Property-based tests to catch more mutants |
| Flaky tests | High | Retry logic, deterministic inputs, no randomness |
| Incorrect results | Critical | Validation tests for all benchmarks, automated checks |

### 12.3 Timeline Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Scope creep | Medium | PMAT roadmap enforcement, milestone gates |
| Benchmark implementation delays | Medium | Parallel implementation (multiple contributors) |
| Docker build time | Low | Multi-stage builds, layer caching |

---

## 13. Success Metrics

### 13.1 Performance Goals

- **Geometric Mean**: Ruchy fastest across 7 benchmarks
- **Cold Start**: <50ms for Ruchy (excluding image load)
- **Image Size**: <10MB for all static binaries
- **Memory**: <64MB peak for all benchmarks

### 13.2 Quality Goals

- **Test Coverage**: ≥85%
- **Mutation Score**: ≥85%
- **Complexity**: ≤15 (cyclomatic), ≤20 (cognitive)
- **SATD Violations**: 0
- **Zero Critical Bugs**: No P0/P1 bugs in production

### 13.3 Publication Goals

- **GitHub Stars**: 100+ in first month
- **Blog Views**: 10,000+ in first month
- **Conference Acceptance**: At least 1 talk accepted
- **Academic Citation**: Paper cited by other research

---

## 14. Future Work

### 14.1 Phase 2 Enhancements

- **ARM64 Support**: Test on Apple Silicon, AWS Graviton
- **Kubernetes Benchmarking**: Measure pod startup time
- **Serverless Comparison**: Compare to AWS Lambda, Google Cloud Run
- **More Languages**: Add Zig, Nim, Swift, Haskell
- **Parallel Benchmarks**: Multi-threaded workloads
- **Real-World Apps**: HTTP server, database queries, ML inference

### 14.2 Continuous Improvement

- **Automated Optimization**: ML-guided compiler flag tuning
- **Historical Tracking**: 2-year performance trend database
- **Community Contributions**: Accept benchmark PRs from external contributors

---

## 15. References

### 15.1 Research Papers

1. **DLS 2016**: "Cross-Language Compiler Benchmarking: Are We Fast Yet?"
2. **OOPSLA 2017**: "Benchmarking Language Implementations"
3. **PLDI 2020**: "Correctness and Performance of Language Runtimes"

### 15.2 Tools

- **bashrs bench**: v6.25.0 (scientific benchmarking)
- **Docker**: v24.0+ (containerization)
- **cargo-llvm-cov**: Coverage analysis (Rust)
- **cargo-mutants**: Mutation testing (Rust)
- **pytest-cov**: Coverage analysis (Python)
- **PMAT**: Project management and quality enforcement

### 15.3 Related Projects

- **ruchy-lambda**: AWS Lambda cold start benchmarking
- **ruchy-book**: Local execution benchmarking (7 languages, 10 benchmarks)
- **lambda-perf**: Industry-standard Lambda benchmarking suite (MIT licensed)

---

## 16. Appendix

### 16.1 Environment Specifications (Enhanced - Genchi Genbutsu)

#### Hardware (Required Recording)
- **CPU Model**: Intel Xeon or AMD EPYC (x86-64)
  - Record exact model: `cat /proc/cpuinfo | grep "model name" | head -1`
  - Record core count: `nproc`
  - Record cache sizes: `lscpu | grep cache`
- **RAM**: 16GB minimum
  - Record exact amount: `free -h`
- **Disk**: SSD (NVMe preferred)
  - Record disk type: `lsblk -d -o name,rota` (0 = SSD, 1 = HDD)
  - Record disk model: `lsblk -o name,model`
- **Network**: Isolated (no concurrent workloads)

#### Operating System (Required Recording)
- **Distribution**: Ubuntu 22.04 LTS or Debian 12
  - Record exact version: `lsb_release -a`
  - Record kernel version: `uname -r` **(Critical for reproducibility)**
  - Record kernel parameters: `sysctl -a > kernel_params.txt`
- **Filesystem**: ext4, xfs, or btrfs
  - Record filesystem type: `df -T /var/lib/docker | tail -1 | awk '{print $2}'`
  - Record mount options: `mount | grep /var/lib/docker`
  - Record I/O scheduler: `cat /sys/block/sda/queue/scheduler` (SDA = disk device)

#### Container Runtime (Required Recording)
- **Docker**: v24.0+
  - Record exact version: `docker --version`
  - Record daemon config: `docker info`
  - Record storage driver: `docker info | grep "Storage Driver"`
  - Record cgroup version: `docker info | grep "Cgroup Version"`

#### Language Runtimes (Locked Versions)
- **Rust**: v1.83+
  - Lock: `rust-toolchain.toml` in repository
- **Go**: v1.23+
  - Lock: `go.mod` with exact version
- **Python**: v3.12+
  - Lock: Dockerfile with `FROM python:3.12.1-slim`
- **GCC**: v13+
  - Lock: Dockerfile with specific gcc version
- **Julia**: v1.10+
  - Lock: Dockerfile with specific Julia version
- **C++ (g++)**: v13+
  - Lock: Same as GCC

#### System Configuration (Required)
- **CPU Governor**: Set to `performance` (not `powersave`)
  ```bash
  echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
  ```
- **Turbo Boost**: Document status (enabled/disabled)
  ```bash
  cat /sys/devices/system/cpu/intel_pmu/allow_tsx_force_abort
  ```
- **ASLR (Address Space Layout Randomization)**: Document status
  ```bash
  cat /proc/sys/kernel/randomize_va_space  # 2 = full, 1 = partial, 0 = off
  ```
- **Swap**: Disabled for benchmark runs
  ```bash
  sudo swapoff -a
  ```

#### Recording Template
All benchmark runs must include a `environment.json` file:

```json
{
  "hardware": {
    "cpu_model": "Intel(R) Xeon(R) Gold 6130",
    "cpu_cores": 32,
    "ram_gb": 64,
    "disk_type": "NVMe SSD",
    "disk_model": "Samsung 970 EVO Plus"
  },
  "os": {
    "distribution": "Ubuntu 22.04.3 LTS",
    "kernel_version": "6.8.0-85-generic",
    "filesystem": "ext4",
    "mount_options": "rw,relatime,errors=remount-ro",
    "io_scheduler": "mq-deadline"
  },
  "docker": {
    "version": "24.0.7",
    "storage_driver": "overlay2",
    "cgroup_version": "2"
  },
  "languages": {
    "rust": "1.83.0",
    "go": "1.23.1",
    "python": "3.12.1",
    "gcc": "13.2.0",
    "julia": "1.10.4"
  },
  "system_config": {
    "cpu_governor": "performance",
    "turbo_boost": "enabled",
    "aslr": 2,
    "swap_enabled": false
  }
}
```

**Automation**:
```bash
# scripts/capture-environment.sh
#!/bin/bash
# Captures complete environment specifications
./scripts/capture-environment.sh > results/environment.json
```

### 16.2 Reproducibility Checklist

- [ ] All source code committed to Git
- [ ] All build flags documented
- [ ] Hardware specs recorded
- [ ] Software versions locked (Dockerfile, Cargo.lock)
- [ ] Random seeds fixed (if applicable)
- [ ] Automated build script (`make build-images`)
- [ ] Automated benchmark script (`make bench-all`)
- [ ] Results validated on 3 different machines
- [ ] CI/CD pipeline passing
- [ ] Documentation complete

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2025-01-05 | Claude Code | Initial specification |
| 2.0.0 | 2025-11-05 | Claude Code | Peer review integration (Gemini AI) - Added: macrobenchmark (HTTP server), instrumented measurement, multiple aggregation metrics (geometric/arithmetic/harmonic mean), MAD-based outlier detection, JIT steady-state detection, fio-based I/O with cache control, enhanced environment specifications (kernel, filesystem, I/O scheduler), scope acknowledgment for microbenchmarks |

---

## Peer Review Acknowledgments

This specification was strengthened through critical peer review by Gemini AI Assistant (November 5, 2025), applying Toyota Way principles (Genchi Genbutsu, Kaizen, Jidoka) and citing peer-reviewed research:

**Key Improvements**:
1. **Macrobenchmark Addition** (Blackburn et al. OOPSLA 2007): Added BENCH-013 HTTP server to complement compute-intensive microbenchmarks
2. **Instrumented Measurement** (Felter et al. USENIX ATC 2015, Gregg 2020): Isolated application startup from Docker overhead using embedded instrumentation and perf stat
3. **Statistical Rigor** (Fleming & Wallace 1986, Eyerman & Eeckhout 2018, Akinshin 2021): Multiple aggregation metrics, MAD-based outlier detection, distribution visualizations
4. **JIT Warmup** (Kalibera & Jones PLDI 2013): Steady-state detection for JIT languages instead of fixed iterations
5. **I/O Methodology** (Chen & Patterson 1994, Lockwood 2016): Mandatory cache clearing, fio integration, filesystem documentation
6. **Environment Specifications**: Kernel version, filesystem type, I/O scheduler, CPU governor (critical for reproducibility)
7. **Scope Acknowledgment**: Explicit statement that claims are scoped to compute-intensive workloads

**Citations**:
- Blackburn et al. (OOPSLA 2007): "Wake up and Smell the Coffee: Evaluation Methodology for the 21st Century"
- Felter et al. (USENIX ATC 2015): "An Updated Performance Comparison of Virtual Machines and Linux Containers"
- Gregg (2020): "BPF Performance Tools"
- Kalibera & Jones (PLDI 2013): "Rigorous Benchmarking in the Presence of JIT Compilation"
- Fleming & Wallace (1986): "How Not to Lie with Statistics: The Correct Way to Summarize Benchmark Results"
- Eyerman & Eeckhout (IEEE CL 2018): "R.I.P. Geomean Speedup"
- Akinshin (2021): Performance analysis and distribution visualization
- Chen & Patterson (1994): "A New Approach to I/O Performance Evaluation"
- Lockwood (2016): I/O benchmarking best practices
- Mytkowicz et al. (ASPLOS 2009): "Producing Wrong Data Without Doing Anything Obviously Wrong!"

---

**END OF SPECIFICATION**
