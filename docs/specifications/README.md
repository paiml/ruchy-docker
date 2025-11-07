# Docker Runtime Benchmarking Specifications

**Project**: ruchy-docker
**Status**: Ready for Implementation
**Version**: 2.0.0 (Peer-Reviewed)

---

## Documents in This Directory

### 1. [docker-runtime-benchmarking-spec.md](./docker-runtime-benchmarking-spec.md)
**Main specification document** - 1,321 lines

**Contents**:
- Complete benchmarking methodology
- 8 benchmarks (7 micro + 1 macro)
- 7 languages (Ruchy transpiled/compiled, C, Rust, Go, Python, Julia, C++)
- Instrumented measurement approach
- Statistical rigor (geometric/arithmetic/harmonic mean, MAD outlier detection)
- EXTREME TDD strategy (85%+ coverage, 85%+ mutation score)
- Docker optimization (distroless images, <10MB target)
- PMAT integration for quality enforcement

**Key Sections**:
1. Executive Summary
2. Architecture Overview
3. Metrics and Measurements (instrumented + perf stat)
4. Docker Containerization Strategy
5. Benchmark Implementation Standards
6. Testing Strategy (EXTREME TDD)
7. PMAT Integration
8. Automation and Reproducibility
9. Results Reporting (JSON, Markdown, HTML)
10. Scientific Validity (10 peer-reviewed citations)

### 2. [PEER_REVIEW_RESPONSE.md](./PEER_REVIEW_RESPONSE.md)
**Peer review response document** - 452 lines

**Contents**:
- Summary of critical peer review by Gemini AI (November 5, 2025)
- Toyota Way principles (Genchi Genbutsu, Kaizen, Jidoka)
- All 7 recommendations implemented
- Before/after comparison (v1.0.0 → v2.0.0)
- Academic citations integrated
- Implementation roadmap

**Critical Improvements**:
1. ✅ Macrobenchmark (HTTP server) added
2. ✅ Instrumented measurement (isolated startup time)
3. ✅ Multiple aggregation metrics
4. ✅ MAD-based outlier detection
5. ✅ JIT steady-state warmup
6. ✅ Enhanced I/O methodology (fio + cache control)
7. ✅ Comprehensive environment specs (kernel, filesystem, I/O scheduler)

---

## Quick Start

### Read First
1. **Specification**: [docker-runtime-benchmarking-spec.md](./docker-runtime-benchmarking-spec.md)
   - Section 1: Executive Summary (understand goals)
   - Section 2.2: Benchmark Matrix (see what's being tested)
   - Section 3: Metrics (understand measurements)

### Understand Changes
2. **Peer Review Response**: [PEER_REVIEW_RESPONSE.md](./PEER_REVIEW_RESPONSE.md)
   - Executive Summary (what changed and why)
   - Critical Improvements sections (7 major enhancements)

### Implementation
3. **Follow the spec**:
   - Section 6: Testing Strategy (EXTREME TDD)
   - Section 7: PMAT Integration (quality gates)
   - Section 8: Automation (Makefile targets)

---

## Key Metrics

### Benchmarks
- **Total**: 8 benchmarks × 7 languages = 56 Docker containers
- **Microbenchmarks**: 7 (fibonacci, primes, array-sum, matrix-mult, hash-map, file-io, startup)
- **Macrobenchmark**: 1 (HTTP server - 10K requests, 100 concurrent)

### Quality Gates
- **Test Coverage**: ≥85%
- **Mutation Score**: ≥85%
- **Cyclomatic Complexity**: ≤15
- **Cognitive Complexity**: ≤20
- **SATD Violations**: 0

### Performance Targets
- **Cold Start**: <10ms (instrumented application startup, Ruchy)
- **Image Size**: <10MB (distroless/static)
- **Memory**: <64MB (peak RSS)

### Statistical Rigor
- **Warmup**: 3 iterations (compiled), steady-state (JIT)
- **Measurement**: 10 runs per benchmark
- **Aggregation**: Geometric mean + Arithmetic mean + Harmonic mean
- **Outlier Detection**: MAD (Median Absolute Deviation)
- **Visualization**: Box plots + violin plots
- **Reproducibility**: 3 separate days, 2+ machines

---

## Academic Foundation

### Peer-Reviewed Citations (10)
1. **Blackburn et al. (OOPSLA 2007)**: Benchmark evaluation methodology
2. **Felter et al. (USENIX ATC 2015)**: Container performance comparison
3. **Gregg (2020)**: BPF Performance Tools
4. **Kalibera & Jones (PLDI 2013)**: JIT benchmarking rigor
5. **Fleming & Wallace (1986)**: Benchmark result summarization
6. **Eyerman & Eeckhout (IEEE CL 2018)**: Geometric mean critique
7. **Akinshin (2021)**: Performance analysis
8. **Chen & Patterson (1994)**: I/O performance evaluation
9. **Lockwood (2016)**: I/O benchmarking
10. **Mytkowicz et al. (ASPLOS 2009)**: Avoiding benchmark pitfalls

### Toyota Way Principles
- **Genchi Genbutsu** (Go and See): Instrumented measurement, perf stat, root cause analysis
- **Kaizen** (Continuous Improvement): Multiple metrics, enhanced I/O, steady-state JIT
- **Jidoka** (Automation with Human Touch): Andon Cord quality gates, distribution visualizations

---

## Project Structure

```
ruchy-docker/
├── docs/
│   └── specifications/
│       ├── docker-runtime-benchmarking-spec.md  (Main spec - 1,321 lines)
│       ├── PEER_REVIEW_RESPONSE.md              (Review response - 452 lines)
│       └── README.md                            (This file)
├── benchmarks/
│   ├── fibonacci/       # BENCH-007
│   ├── primes/          # BENCH-008
│   ├── array-sum/       # BENCH-005
│   ├── matrix-mult/     # BENCH-006
│   ├── hash-map/        # BENCH-009
│   ├── file-io/         # BENCH-010
│   ├── startup/         # BENCH-012
│   └── http-server/     # BENCH-013 (NEW)
├── docker/              # Dockerfiles per language
├── src/                 # Test framework
├── tests/               # EXTREME TDD (unit, integration, property, mutation, fuzz)
├── scripts/             # Automation
│   ├── run-benchmark.sh
│   ├── run-all-benchmarks.sh
│   └── capture-environment.sh
└── Makefile             # Build, test, bench, quality, deploy
```

---

## Implementation Phases

### Phase 1: Test Framework (4 weeks)
- Docker orchestration with instrumented measurement
- Metrics collection (startup, compute, wall-clock)
- perf stat integration
- MAD-based outlier detection
- Distribution visualization

### Phase 2: Benchmark Implementation (4 weeks)
- 7 microbenchmarks (all languages)
- 1 macrobenchmark (HTTP server)
- fio-based I/O benchmarking
- JIT steady-state detection
- Cache clearing automation

### Phase 3: Docker Optimization (2 weeks)
- Distroless/static image conversions
- Binary stripping, LTO
- <10MB image sizes
- <10ms instrumented startup

### Phase 4: Results Publication (2 weeks)
- Geometric/arithmetic/harmonic mean reporting
- Distribution visualizations (box plots, violin plots)
- environment.json publication
- Public GitHub repository

---

## Usage

### Build All Images
```bash
make build-images
```

### Run Single Benchmark
```bash
make bench BENCHMARK=fibonacci LANGUAGE=ruchy-transpiled
```

### Run Full Suite
```bash
make bench-all
```

### Quality Gates
```bash
make quality  # format, lint, test, coverage, mutation, complexity
```

### Deploy Results
```bash
make deploy  # After quality gates pass
```

---

## Key Takeaways

1. **Scientifically Rigorous**: 10 peer-reviewed citations, Toyota Way principles
2. **Transparent**: Scope acknowledgment, multiple metrics, full distributions
3. **Reproducible**: Complete environment specs, multi-day/multi-machine validation
4. **Quality-Driven**: EXTREME TDD, 85%+ coverage/mutation, zero defects
5. **Production-Ready**: Automated via Makefile, CI/CD integrated
6. **World-Class**: Goal is to demonstrate Ruchy achieves top-tier performance for compute-intensive containerized workloads

---

## Status

- ✅ Specification v2.0.0: Complete (peer-reviewed)
- ✅ Peer review response: Complete
- 🔄 Implementation: Ready to begin
- ⏳ Phase 1: Not started
- ⏳ Phase 2: Not started
- ⏳ Phase 3: Not started
- ⏳ Phase 4: Not started

---

## Contact

For questions or clarifications, refer to:
1. Main specification: [docker-runtime-benchmarking-spec.md](./docker-runtime-benchmarking-spec.md)
2. Peer review response: [PEER_REVIEW_RESPONSE.md](./PEER_REVIEW_RESPONSE.md)
3. Project README: `/home/noah/src/ruchy-docker/README.md`

---

**Last Updated**: November 5, 2025
**Version**: 2.0.0
**Status**: Ready for Implementation
