# Demo Quick Start Guide

## 5-Minute Demo (Copy & Paste)

```bash
# 1. Show capabilities
make help

# 2. Build images (pre-build before video to save time)
make build-fibonacci

# 3. Show size comparison
docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}" | grep fibonacci

# 4. Run benchmarks with timing
echo "=== Ruchy (compiled) ===" && time docker run --rm ruchy-compiled:fibonacci
echo "=== Python (interpreted) ===" && time docker run --rm python:fibonacci

# 5. Show quality infrastructure
make lint-fix
make test

# 6. Show documentation
cat docs/runtime.md | head -80
```

## Key Statistics to Highlight

### Binary Size
- **Ruchy-compiled: 336 KB** ← SMALLEST
- Ruchy-transpiled: 414 KB
- Rust: 424 KB
- C: 695 KB
- Go: 1.41 MB
- Deno: 90.5 MB
- Python: 119 MB (382× larger)
- Julia: 711 MB (2,284× larger)

### Execution Time (Fibonacci 35)
- **C: 7.83 ms** ← FASTEST
- Rust: 19.28 ms
- **Ruchy: 21.00 ms** ← 91% of Rust performance
- Deno: 69.28 ms (3.3× slower)
- Julia: 252.91 ms (23.5× slower)
- Python: 610 ms (29× slower)

### Quality Infrastructure
- **9 linters** (Rust, Go, C, Python, TS, Julia, Scripts, Makefile, Dockerfiles)
- **3 auto-fix targets** (safe, automated corrections)
- **16 Docker containers** (8 languages × 2 benchmarks)
- **100% build success rate**

## One-Liner Demos

```bash
# Size comparison (visual)
docker images | grep fibonacci | awk '{printf "%-25s %10s\n", $1":"$2, $7$8}' | sort

# Quick benchmark comparison
for lang in c rust ruchy-compiled python; do
  echo "=== $lang ===" && docker run --rm $lang:fibonacci | grep COMPUTE_TIME_US
done

# Show auto-fix in action
make lint-scripts-fix && ls -lh scripts/*.bak

# Quality gate summary
make lint 2>&1 | grep "✅"
```

## Pre-Recording Checklist

- [ ] Pre-build all images: `make build-fibonacci`
- [ ] Verify images: `docker images | grep fibonacci`
- [ ] Test one benchmark: `docker run --rm ruchy-compiled:fibonacci`
- [ ] Clear terminal history: `clear && history -c`
- [ ] Set terminal font size: Large for visibility
- [ ] Close unnecessary applications
- [ ] Disable notifications

## Recommended Terminal Settings

```bash
# Increase font size for recording
# Terminal > Preferences > Profile > Text > Font Size: 16-18pt

# Set colors for better visibility
export PS1='\[\e[1;32m\]\u@\h\[\e[0m\]:\[\e[1;34m\]\w\[\e[0m\]\$ '

# Clear screen
clear
```

## Talking Points Cheat Sheet

**Opening:**
"This is ruchy-docker: a scientifically rigorous benchmarking framework comparing 8 programming languages in Docker containers."

**Quality Gates:**
"We have 9 linters with auto-fix capabilities, ensuring EXTREME TDD quality standards with zero tolerance for defects."

**Docker Builds:**
"Multi-stage builds create minimal images. Ruchy achieves 336 KB - that's 382 times smaller than Python's 119 MB."

**Performance:**
"Ruchy delivers 21ms compute time - that's 91% of Rust's performance and 29 times faster than Python."

**Scientific Rigor:**
"We use instrumented measurement, MAD outlier detection, and peer-reviewed methodology with 10 academic citations."

**Closing:**
"Ruchy demonstrates world-class performance: smallest binaries, near-Rust speed, with simpler syntax. All code and documentation available on GitHub."

## GitHub Issue Reference

Filed comprehensive enhancement request for bashrs auto-fix improvements:
- **Issue:** https://github.com/paiml/bashrs/issues/20
- **Status:** Open, awaiting review
- **Impact:** Would improve auto-fix coverage from 60% to 90%+

## File Locations for Screen Recording

```
/home/noah/src/ruchy-docker/
├── README.md                          # Project overview
├── Makefile                           # All demo commands
├── CLAUDE.md                          # Development guidance
├── docs/
│   ├── runtime.md                     # Technical analysis
│   └── images/
│       ├── runtime-comparison.png     # Visual comparison
│       └── runtime-problems.png       # 3-problem analysis
├── benchmarks/
│   ├── fibonacci/                     # 7 language implementations
│   └── primes/                        # 7 language implementations
└── docker/                            # 16 Dockerfiles
```

## Post-Demo Cleanup

```bash
# Remove backup files
rm -f scripts/*.bak

# Optional: Clean up Docker images
docker rmi $(docker images | grep -E "fibonacci|primes" | awk '{print $3}')

# Or keep them for next demo
docker images | grep -E "fibonacci|primes"
```
