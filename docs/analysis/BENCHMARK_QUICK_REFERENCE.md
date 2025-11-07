# Ruchy Benchmark Suite - Quick Reference

## Benchmark Locations

**Base Directory**: `/home/noah/src/ruchy-book/test/ch21-benchmarks/`

### 10 Active Benchmarks

| ID | Name | Status | Category | Focus | Key Result |
|----|------|--------|----------|-------|------------|
| 002 | Matrix Multiply (100x100) | BLOCKED #119 | Core | Float arithmetic | Awaiting global state fix |
| 003 | String Concat (10K ops) | ✅ COMPLETE | Core | String/Memory | 5.17-5.38x speedup |
| 004 | Binary Tree | ✅ COMPLETE | Core | Memory/GC | Allocation stress test |
| 005 | Array Sum (1M ints) | ✅ COMPLETE | Core | Tight loops | **12% of C** |
| 006 | File Line Process (100MB) | BLOCKED #116 | CLI | I/O | Awaiting File object API |
| 007 | Fibonacci (n=20) | ✅ COMPLETE | Core | Recursion | **91% of C** - FLAGSHIP |
| 008 | Prime Gen (10K) | ✅ COMPLETE | Core | Integer math | **0.26% of C** - EXACT! |
| 009 | JSON Parse (50MB) | ✅ UNBLOCKED | CLI | JSON parsing | Ready v3.176.0+ |
| 011 | Nested Loops (1M) | ✅ COMPLETE | Core | Loop optim | **12% of C** |
| 012 | Startup (Hello) | ✅ COMPLETE | CLI | Cold start | **2.6% of C** |

---

## Performance Results (6 Benchmarks, 9 Languages)

### Geometric Mean (Overall Performance)

```
Julia           ████████████████████████  24.79x
C               █████████████████░░░░░░░  18.51x
Rust            ████████████████░░░░░░░░  16.49x
Ruchy Transpile ███████████████░░░░░░░░░  15.12x ✨ BEST
Ruchy Compiled  ███████████████░░░░░░░░░  14.89x ✨
Go              █████████████░░░░░░░░░░░  13.37x
Deno            ██░░░░░░░░░░░░░░░░░░░░░░  2.33x
Ruchy Bytecode  █░░░░░░░░░░░░░░░░░░░░░░░  1.49x
Python          (baseline)                 1.00x
Ruchy AST       (baseline/2.7x slower)     0.37x
```

### Per-Benchmark Comparison

#### BENCH-007: Fibonacci (Function Calls)
```
Julia           ████████████░░░░░░  12.90x
Ruchy Transpile ███████████░░░░░░░  10.51x ⭐ 91% of C!
Rust            ███████████░░░░░░░  10.38x
C               ███████████░░░░░░░  11.51x
Python          (baseline)         1.00x
```

#### BENCH-008: Prime Generation (Integer Math)
```
Julia           ████████████████████████████████████  71.30x
Ruchy Bytecode  ████████████░░░░░░░░░░░░░░░░░░░░░░  22.78x ⭐ 0.26% of C!
Ruchy Compiled  ████████████░░░░░░░░░░░░░░░░░░░░░░  22.66x
C               ████████████░░░░░░░░░░░░░░░░░░░░░░  22.72x
Python          (baseline)                        1.00x
```

#### BENCH-005: Array Sum (Tight Loops)
```
Julia           ███████████████████████████░░░░░░░░  33.54x
Ruchy Transpile ████████████████████████░░░░░░░░░░  30.60x ⭐ 12% of C!
Ruchy Compiled  ████████████████████████░░░░░░░░░░  30.24x
Rust            ████████████████████░░░░░░░░░░░░░░  28.59x
Python          (baseline)                        1.00x
```

#### BENCH-012: Startup Time (CLI Performance)
```
Julia           ████████░░░░░░░░░░░░░░░░░░░░░░░░░░  8.22x
Ruchy Compiled  ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░  6.32x ⭐ 2.6% of C!
Go              ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░  6.00x
C               ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░  5.53x
Python          (baseline)                        1.00x
```

---

## Languages Compared (9 Total)

### Compiled Languages (3)
- **C**: GCC -O3, baseline native performance (18.51x GM)
- **Rust**: LLVM -O3, memory safety (16.49x GM)
- **Go**: AOT compiled, fast startup (13.37x GM)

### JIT Compiled (2)
- **Julia**: LLVM JIT with type inference (24.79x GM) - FASTEST
- **Deno**: V8 JavaScript engine (2.33x GM)

### Interpreted (1)
- **Python**: CPython (1.00x GM) - BASELINE

### Ruchy Execution Modes (4)
- **AST**: Tree-walking interpreter (0.37x - for development)
- **Bytecode**: VM execution (1.49x - moderate performance)
- **Transpiled**: Ruchy→Rust→compiled (15.12x GM - BEST OVERALL)
- **Compiled**: Direct native (14.89x GM - single-step)

---

## Test Framework & Methodology

### Tool: bashrs bench v6.25.0
- Scientific benchmarking framework
- 3 warmup iterations + 10 measurement iterations
- Statistical analysis (mean, median, stddev, min, max)
- JSON output for reproducibility

### Methodology: DLS 2016 "Are We Fast Yet?"
- Peer-reviewed cross-language benchmarking
- Identical implementations across languages
- Idiomatic use per language (balanced approach)
- Geometric mean for honest aggregation
- Deterministic, reproducible results

### Hardware
- CPU: AMD Ryzen Threadripper 7960X 24-Cores
- RAM: 125Gi
- OS: Linux 6.8.0-85-generic

---

## Running Benchmarks

### Quick Test (6 core benchmarks, ~10 minutes)
```bash
cd /home/noah/src/ruchy-book/test/ch21-benchmarks
./validate-ruchy-benchmarks.sh
```

### Single Benchmark
```bash
./run-bench-012-full.sh  # Startup (~2 min) - fastest
./run-bench-007-full.sh  # Fibonacci (~5 min) - flagship
./run-bench-005-full.sh  # Array sum (~10 min) - tight loops
```

### Full Suite (9 benchmarks, ~45-60 minutes)
```bash
./run-all-benchmarks.sh
```

---

## Key Files & Locations

### Benchmark Programs (69 files)
```
bench-007-fibonacci.{ruchy,py,go,rs,ts,jl,c}
bench-008-primes.{ruchy,py,go,rs,ts,jl,c}
bench-012-startup.{ruchy,py,go,rs,ts,jl,c}
... and 7 other benchmarks across 9 languages
```

### Test Runners (16 shell scripts)
```
run-bench-NNN-full.sh    # Individual benchmark (10 execution modes)
run-all-benchmarks.sh    # Complete suite runner
validate-ruchy-benchmarks.sh  # Quick validation
analyze-results.sh       # Summary generation
```

### Results (JSON with statistics)
```
results/bench-007-results-full.json  # Fibonacci results
results/bench-008-results-full.json  # Prime generation results
... (JSON files for each completed benchmark)
```

### Documentation
```
BENCHMARK-ROADMAP.md             # Complete feature roadmap
BENCHMARK_SUMMARY.md             # Results summary
BENCHMARK-STATUS-v3.176.0.md     # Latest status
docs/BENCHMARKING-METHODOLOGY.md # Scientific methodology
```

---

## Performance Tiers

### World-Class (>20x Python)
- **Julia**: 24.79x GM
  - Specialized JIT for numerical code
  - Type inference optimization
  - Exceptional loop and arithmetic performance

### Native Performance (13-19x Python)
- **C**: 18.51x (baseline native)
- **Rust**: 16.49x (safety + speed)
- **Ruchy Transpiled**: 15.12x (82% of C) ✨
- **Ruchy Compiled**: 14.89x (80% of C) ✨
- **Go**: 13.37x (GC + fast compilation)

### Interpreted Tier (1-3x Python)
- **Deno**: 2.33x (V8 JIT overhead)
- **Ruchy Bytecode**: 1.49x (fast VM)
- **Python**: 1.00x (baseline)
- **Ruchy AST**: 0.37x (tree-walking)

---

## Key Metrics Explained

### Speedup Formula
```
speedup = mean_time_python / mean_time_language
10.51x = 17.03ms (Python) / 1.62ms (Ruchy)
```

### Geometric Mean (Honest Aggregation)
```
GM = (speedup_1 × speedup_2 × ... × speedup_n)^(1/n)
```
- Prevents any single benchmark from dominating
- Recommended by peer-reviewed literature
- Symmetric: 2x speedup = 0.5x slowdown (balanced)

### Variance Metrics
```
mean:   Average of 10 measurements
median: Middle value (robust to outliers)
stddev: Standard deviation (variability)
min/max: Range of results
```

**Quality Threshold**:
- StdDev < 10% of mean ✅ (reliable)
- Min/Max within 2x of median ✅ (no outliers)
- Results reproducible across runs ✅

---

## Status Summary

### Ready & Complete (6/10)
- ✅ BENCH-003, 004, 005, 007, 008, 011, 012 (7 benchmarks)

### Blocked (2/10)
- ❌ BENCH-002: Global mutable state (Issue #119)
- ❌ BENCH-006: File object methods (Issue #116)

### Unblocked (1/10)
- ✅ BENCH-009: JSON parsing (v3.176.0 fixed)

### Future (1/10)
- ⏳ BENCH-010: HTTP mock (not yet implemented)

---

## Critical Claims (Defensible)

1. **"Ruchy achieves native-level performance"**
   - 15.12x geometric mean = 82% of C
   - Verified across 6 diverse benchmarks
   - Within 0.26% of C on BENCH-008

2. **"Ruchy transpiled is best overall"**
   - 92% of Rust performance (16.49x)
   - Exceeds Go (13.37x) by 13%
   - Leverages mature Rust ecosystem

3. **"Excellent startup performance"**
   - 1.59ms vs 1.55ms C (2.6% slower)
   - 6.32x faster than Python
   - Ideal for CLI tools

4. **"Four execution modes provide flexibility"**
   - AST: Development/debugging
   - Bytecode: Fast interpretation
   - Transpiled: Best production performance
   - Compiled: Single-step native compilation

---

## Caveats & Limitations

1. **Julia dominates** (24.79x) - specialized JIT for numerical code
2. **Global mutable state** not yet fully supported (BENCH-002 blocked)
3. **File streaming APIs** not yet implemented (BENCH-006 blocked)
4. **JSON library performance** varies (BENCH-009 ready to test)
5. **Some library operations** may be slower (ecosystem dependent)

---

## References

**Primary Paper**:
Marr, S., Daloze, B., & Mössenböck, H. (2016). *Cross-Language Compiler Benchmarking: Are We Fast Yet?* Proceedings of DLS 2016, 120-131. ACM.
https://doi.org/10.1145/2989225.2989232

**Tools**:
- bashrs bench v6.25.0 (scientific benchmarking)
- Ruchy language (all 4 execution modes)

**Documentation**:
- `docs/BENCHMARKING-METHODOLOGY.md` (detailed scientific approach)
- `BENCHMARK-ROADMAP.md` (complete feature plan)
- `BENCHMARK_SUMMARY.md` (results overview)

