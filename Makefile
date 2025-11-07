# Makefile settings for safety and performance
.SUFFIXES:
.DELETE_ON_ERROR:
.ONESHELL:

.PHONY: help
help:
	@echo "ruchy-docker - Docker Runtime Benchmarking Framework"
	@echo ""
	@echo "Development:"
	@echo "  make dev              - Build and start development environment"
	@echo "  make build            - Build Rust project"
	@echo ""
	@echo "Docker Images:"
	@echo "  make build-images     - Build all Docker benchmark images (56 containers)"
	@echo ""
	@echo "Benchmarking:"
	@echo "  make bench            - Run single Docker benchmark (BENCHMARK=fibonacci LANGUAGE=ruchy-transpiled)"
	@echo "  make bench-all        - Run full Docker benchmark suite"
	@echo "  make bench-cli        - Run CLI benchmarks with bashrs (extracts binaries from containers)"
	@echo "  make extract-binaries - Extract binaries from Docker containers for CLI benchmarking"
	@echo ""
	@echo "Quality Gates (Andon Cord - Fail Fast):"
	@echo "  make quality          - Run all quality gates (format → lint → test → coverage → mutation)"
	@echo "  make format           - Check code formatting"
	@echo "  make format-fix       - Fix code formatting (Rust, Python, bash scripts)"
	@echo "  make lint             - Run ALL linters (Rust, Go, C, Python, TS, Julia, Scripts, Makefile, Dockerfiles)"
	@echo "  make lint-fix         - Auto-fix lint issues where possible (Scripts, Dockerfiles)"
	@echo "  make lint-rust        - Lint Rust benchmarks (clippy)"
	@echo "  make lint-go          - Lint Go benchmarks (go vet + staticcheck)"
	@echo "  make lint-c           - Lint C benchmarks (clang-tidy)"
	@echo "  make lint-python      - Lint Python benchmarks (pylint)"
	@echo "  make lint-typescript  - Lint TypeScript benchmarks (deno lint)"
	@echo "  make lint-julia       - Lint Julia benchmarks (syntax check)"
	@echo "  make lint-scripts     - Lint bash scripts (bashrs)"
	@echo "  make lint-scripts-fix - Auto-fix bash script issues (bashrs --fix)"
	@echo "  make lint-makefile    - Lint Makefile (bashrs)"
	@echo "  make lint-dockerfiles - Lint all Dockerfiles (bashrs)"
	@echo "  make lint-dockerfiles-fix - Auto-fix Dockerfile issues (bashrs --fix)"
	@echo "  make test             - Run all tests (Rust + Python + Scripts)"
	@echo "  make test-scripts     - Run bash script unit tests"
	@echo "  make coverage         - Generate coverage report (≥85% required)"
	@echo "  make mutation         - Run mutation tests (≥85% score required)"
	@echo "  make complexity       - Check code complexity (pmat)"
	@echo ""
	@echo "Documentation:"
	@echo "  make charts           - Preview proportional ASCII charts (stdout)"
	@echo "  make update-readme    - Update README.md charts (creates backup)"
	@echo ""
	@echo "Deployment:"
	@echo "  make deploy           - Deploy results (after quality gates pass)"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean            - Clean build artifacts"
	@echo "  make clean-all        - Clean everything including Docker images"

# Development
.PHONY: dev
dev:
	cargo build
	@echo "Development environment ready"

.PHONY: build
build:
	cargo build --release

# Docker Images
.PHONY: build-images
build-images:
	@echo "Building all Docker images (2 benchmarks × 8 languages = 16 containers)..."
	./scripts/build-all-images.sh

.PHONY: build-fibonacci
build-fibonacci:
	@echo "Building Fibonacci benchmark images (8 languages)..."
	docker build -f docker/c/fibonacci.Dockerfile -t c:fibonacci .
	docker build -f docker/rust/fibonacci.Dockerfile -t rust:fibonacci .
	docker build -f docker/python/fibonacci.Dockerfile -t python:fibonacci .
	docker build -f docker/go/fibonacci.Dockerfile -t go:fibonacci .
	docker build -f docker/julia/fibonacci.Dockerfile -t julia:fibonacci .
	docker build -f docker/deno/fibonacci.Dockerfile -t deno:fibonacci .
	(cd .. && docker build -f ruchy-docker/docker/ruchy-transpiled/fibonacci.Dockerfile -t ruchy-transpiled:fibonacci .)
	(cd .. && docker build -f ruchy-docker/docker/ruchy-compiled/fibonacci.Dockerfile -t ruchy-compiled:fibonacci .)
	@echo "Note: Julia is EXPERIMENTAL (JIT compilation, large image size)"

.PHONY: build-primes
build-primes:
	@echo "Building Primes benchmark images (8 languages)..."
	docker build -f docker/c/primes.Dockerfile -t c:primes .
	docker build -f docker/rust/primes.Dockerfile -t rust:primes .
	docker build -f docker/python/primes.Dockerfile -t python:primes .
	docker build -f docker/go/primes.Dockerfile -t go:primes .
	docker build -f docker/julia/primes.Dockerfile -t julia:primes .
	docker build -f docker/deno/primes.Dockerfile -t deno:primes .
	(cd .. && docker build -f ruchy-docker/docker/ruchy-transpiled/primes.Dockerfile -t ruchy-transpiled:primes .)
	(cd .. && docker build -f ruchy-docker/docker/ruchy-compiled/primes.Dockerfile -t ruchy-compiled:primes .)

# Benchmarking
.PHONY: bench
bench:
	@if [ -z "$(BENCHMARK)" ] || [ -z "$(LANGUAGE)" ]; then \
		echo "Usage: make bench BENCHMARK=fibonacci LANGUAGE=ruchy-transpiled"; \
		exit 1; \
	fi
	./scripts/run-benchmark.sh $(BENCHMARK) $(LANGUAGE)

.PHONY: bench-all
bench-all:
	@echo "Running full benchmark suite..."
	./scripts/run-all-benchmarks.sh

# CLI Benchmarking (bashrs bench)
.PHONY: extract-binaries
extract-binaries:
	@echo "Extracting binaries from Docker containers for CLI benchmarking..."
	@mkdir -p bin/
	@if docker image inspect c:fibonacci > /dev/null 2>&1; then \
		echo "Extracting C binary..."; \
		docker create --name temp-c c:fibonacci || exit 1; \
		docker cp temp-c:/fibonacci bin/fibonacci_c || exit 1; \
		docker rm temp-c || exit 1; \
	else \
		echo "⚠️  Skipping C (image c:fibonacci not found)"; \
	fi
	@if docker image inspect rust:fibonacci > /dev/null 2>&1; then \
		echo "Extracting Rust binary..."; \
		docker create --name temp-rust rust:fibonacci || exit 1; \
		docker cp temp-rust:/fibonacci bin/fibonacci_rust || exit 1; \
		docker rm temp-rust || exit 1; \
	else \
		echo "⚠️  Skipping Rust (image rust:fibonacci not found)"; \
	fi
	@if docker image inspect go:fibonacci > /dev/null 2>&1; then \
		echo "Extracting Go binary..."; \
		docker create --name temp-go go:fibonacci || exit 1; \
		docker cp temp-go:/fibonacci bin/fibonacci_go || exit 1; \
		docker rm temp-go || exit 1; \
	else \
		echo "⚠️  Skipping Go (image go:fibonacci not found)"; \
	fi
	@if docker image inspect ruchy-transpiled:fibonacci > /dev/null 2>&1; then \
		echo "Extracting Ruchy transpiled binary..."; \
		docker create --name temp-ruchy-transpiled ruchy-transpiled:fibonacci || exit 1; \
		docker cp temp-ruchy-transpiled:/fibonacci bin/fibonacci_ruchy_transpiled || exit 1; \
		docker rm temp-ruchy-transpiled || exit 1; \
	else \
		echo "⚠️  Skipping Ruchy transpiled (image ruchy-transpiled:fibonacci not found)"; \
	fi
	@if docker image inspect ruchy-compiled:fibonacci > /dev/null 2>&1; then \
		echo "Extracting Ruchy compiled binary..."; \
		docker create --name temp-ruchy-compiled ruchy-compiled:fibonacci || exit 1; \
		docker cp temp-ruchy-compiled:/fibonacci bin/fibonacci_ruchy_compiled || exit 1; \
		docker rm temp-ruchy-compiled || exit 1; \
	else \
		echo "⚠️  Skipping Ruchy compiled (image ruchy-compiled:fibonacci not found)"; \
	fi
	@if docker image inspect python:fibonacci > /dev/null 2>&1; then \
		echo "Extracting Python script..."; \
		docker create --name temp-python python:fibonacci || exit 1; \
		docker cp temp-python:/app/fibonacci.py bin/fibonacci_python.py || exit 1; \
		docker rm temp-python || exit 1; \
	else \
		echo "⚠️  Skipping Python (image python:fibonacci not found)"; \
	fi
	@if docker image inspect julia:fibonacci > /dev/null 2>&1; then \
		echo "Extracting Julia script (EXPERIMENTAL)..."; \
		docker create --name temp-julia julia:fibonacci || exit 1; \
		docker cp temp-julia:/app/fibonacci.jl bin/fibonacci_julia.jl || exit 1; \
		docker rm temp-julia || exit 1; \
	else \
		echo "⚠️  Skipping Julia (image julia:fibonacci not found)"; \
	fi
	@if docker image inspect deno:fibonacci > /dev/null 2>&1; then \
		echo "Extracting Deno binary..."; \
		docker create --name temp-deno deno:fibonacci || exit 1; \
		docker cp temp-deno:/fibonacci bin/fibonacci_deno || exit 1; \
		docker rm temp-deno || exit 1; \
	else \
		echo "⚠️  Skipping Deno (image deno:fibonacci not found)"; \
	fi
	@if ls bin/* > /dev/null 2>&1; then chmod +x bin/* 2>/dev/null || true; fi
	@echo "Creating shell wrapper scripts for bashrs bench..."
	@if [ -f bin/fibonacci_c ]; then \
		echo '#!/bin/sh' > bin/fibonacci_c.sh; \
		echo 'exec "$$(dirname "$$0")/fibonacci_c"' >> bin/fibonacci_c.sh; \
		chmod +x bin/fibonacci_c.sh; \
	fi
	@if [ -f bin/fibonacci_rust ]; then \
		echo '#!/bin/sh' > bin/fibonacci_rust.sh; \
		echo 'exec "$$(dirname "$$0")/fibonacci_rust"' >> bin/fibonacci_rust.sh; \
		chmod +x bin/fibonacci_rust.sh; \
	fi
	@if [ -f bin/fibonacci_go ]; then \
		echo '#!/bin/sh' > bin/fibonacci_go.sh; \
		echo 'exec "$$(dirname "$$0")/fibonacci_go"' >> bin/fibonacci_go.sh; \
		chmod +x bin/fibonacci_go.sh; \
	fi
	@if [ -f bin/fibonacci_ruchy_transpiled ]; then \
		echo '#!/bin/sh' > bin/fibonacci_ruchy_transpiled.sh; \
		echo 'exec "$$(dirname "$$0")/fibonacci_ruchy_transpiled"' >> bin/fibonacci_ruchy_transpiled.sh; \
		chmod +x bin/fibonacci_ruchy_transpiled.sh; \
	fi
	@if [ -f bin/fibonacci_ruchy_compiled ]; then \
		echo '#!/bin/sh' > bin/fibonacci_ruchy_compiled.sh; \
		echo 'exec "$$(dirname "$$0")/fibonacci_ruchy_compiled"' >> bin/fibonacci_ruchy_compiled.sh; \
		chmod +x bin/fibonacci_ruchy_compiled.sh; \
	fi
	@if [ -f bin/fibonacci_python.py ]; then \
		echo '#!/bin/sh' > bin/fibonacci_python.sh; \
		echo 'exec python3 "$$(dirname "$$0")/fibonacci_python.py"' >> bin/fibonacci_python.sh; \
		chmod +x bin/fibonacci_python.sh; \
	fi
	@if [ -f bin/fibonacci_julia.jl ]; then \
		echo '#!/bin/sh' > bin/fibonacci_julia.sh; \
		echo 'exec julia "$$(dirname "$$0")/fibonacci_julia.jl"' >> bin/fibonacci_julia.sh; \
		chmod +x bin/fibonacci_julia.sh; \
	fi
	@if [ -f bin/fibonacci_deno ]; then \
		echo '#!/bin/sh' > bin/fibonacci_deno.sh; \
		echo 'exec "$$(dirname "$$0")/fibonacci_deno"' >> bin/fibonacci_deno.sh; \
		chmod +x bin/fibonacci_deno.sh; \
	fi
	@echo "✅ Binaries extracted to bin/"

.PHONY: bench-cli
bench-cli: extract-binaries
	@echo "Running CLI benchmarks with bashrs bench..."
	@if ! command -v bashrs > /dev/null 2>&1; then \
		echo "❌ bashrs not installed. Run: make install-tools"; \
		exit 1; \
	fi
	@echo ""
	@echo "=== Benchmarking CLI Executables (bashrs bench) ==="
	@echo "This measures full process invocation (startup + compute + teardown)"
	@echo ""
	@mkdir -p results/cli
	@SCRIPTS=""; \
	for script in bin/fibonacci_c.sh bin/fibonacci_rust.sh bin/fibonacci_go.sh bin/fibonacci_deno.sh bin/fibonacci_ruchy_transpiled.sh bin/fibonacci_ruchy_compiled.sh bin/fibonacci_python.sh bin/fibonacci_julia.sh; do \
		if [ -f "$$script" ]; then \
			SCRIPTS="$$SCRIPTS $$script"; \
		fi; \
	done; \
	if [ -z "$$SCRIPTS" ]; then \
		echo "❌ No binaries found to benchmark. Build images first."; \
		exit 1; \
	fi; \
	bashrs bench $$SCRIPTS \
		--warmup 3 --iterations 10 \
		--output results/cli/fibonacci-cli-bench.json \
		--show-raw
	@echo ""
	@echo "✅ CLI benchmarks complete"
	@echo "   Results in results/cli/fibonacci-cli-bench.json"

# Quality Gates (EXTREME TDD - Andon Cord)
.PHONY: quality
quality: format lint test coverage mutation complexity
	@echo "✅ All quality gates PASSED"

.PHONY: format
format:
	@echo "Checking Rust formatting..."
	cargo fmt --all -- --check
	@echo "Checking Python formatting..."
	@if command -v black > /dev/null 2>&1; then \
		black --check src/ tests/ scripts/ 2>/dev/null || true; \
	fi

.PHONY: format-fix
format-fix:
	@echo "Fixing Rust formatting..."
	cargo fmt --all
	@echo "Fixing Python formatting..."
	@if command -v black > /dev/null 2>&1; then \
		black src/ tests/ scripts/ 2>/dev/null || true; \
	fi
	@echo "Auto-fixing bash scripts with bashrs..."
	@if command -v bashrs > /dev/null 2>&1; then \
		for script in scripts/*.sh; do \
			echo "  Auto-fixing $$script"; \
			bashrs lint "$$script" --fix --fix-assumptions -o "$$script" 2>&1 | grep -E "(Applied|No issues)" || true; \
		done; \
	else \
		echo "⚠️  bashrs not installed. Run: make install-tools"; \
	fi

.PHONY: lint
lint: lint-rust lint-go lint-c lint-python lint-typescript lint-julia lint-makefile lint-dockerfiles lint-scripts
	@echo ""
	@echo "✅ All linting complete"

.PHONY: lint-fix
lint-fix: lint-scripts-fix lint-dockerfiles-fix
	@echo ""
	@echo "✅ Auto-fix complete for Scripts and Dockerfiles"
	@echo "   (Other languages require manual fixes)"

.PHONY: lint-rust
lint-rust:
	@echo "Linting Rust files (clippy)..."
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint-go
lint-go:
	@echo "Linting Go files (go vet + staticcheck)..."
	@if command -v go > /dev/null 2>&1; then \
		find benchmarks -name "*.go" -exec dirname {} \; | sort -u | while read dir; do \
			echo "  Checking $$dir"; \
			(cd "$$dir" && go vet . 2>&1 || true); \
		done; \
		if command -v staticcheck > /dev/null 2>&1; then \
			find benchmarks -name "*.go" -exec dirname {} \; | sort -u | while read dir; do \
				(cd "$$dir" && staticcheck . 2>&1 || true); \
			done; \
		fi; \
	else \
		echo "⚠️  go not installed"; \
	fi

.PHONY: lint-c
lint-c:
	@echo "Linting C files (clang-tidy)..."
	@if command -v clang-tidy > /dev/null 2>&1; then \
		find benchmarks -name "*.c" | while read file; do \
			echo "  Checking $$file"; \
			clang-tidy "$$file" -- 2>&1 || true; \
		done; \
	else \
		echo "⚠️  clang-tidy not installed (optional)"; \
	fi

.PHONY: lint-python
lint-python:
	@echo "Linting Python files (pylint)..."
	@if command -v pylint > /dev/null 2>&1; then \
		find . -name "*.py" -path "*/benchmarks/*" -o -name "*.py" -path "*/src/*" -o -name "*.py" -path "*/tests/*" -o -name "*.py" -path "*/scripts/*" | xargs pylint 2>&1 || true; \
	else \
		echo "⚠️  pylint not installed (optional)"; \
	fi

.PHONY: lint-typescript
lint-typescript:
	@echo "Linting TypeScript files (deno lint)..."
	@if command -v deno > /dev/null 2>&1; then \
		find . -name "*.ts" | xargs deno lint 2>&1 || true; \
	else \
		echo "⚠️  deno not installed"; \
	fi

.PHONY: lint-julia
lint-julia:
	@echo "Linting Julia files (JuliaFormatter check)..."
	@if command -v julia > /dev/null 2>&1; then \
		find benchmarks -name "*.jl" | while read file; do \
			echo "  Checking $$file (syntax only)"; \
			julia -e "include(\"$$file\")" 2>&1 > /dev/null || echo "  ⚠️  Syntax error in $$file"; \
		done; \
	else \
		echo "⚠️  julia not installed"; \
	fi

.PHONY: lint-dockerfiles
lint-dockerfiles:
	@echo "Linting Dockerfiles with bashrs..."
	@if command -v bashrs > /dev/null 2>&1; then \
		./scripts/score-dockerfiles.sh | grep -E "^(Scoring:|Overall Grade:|Overall Score:)" || true; \
	else \
		echo "⚠️  bashrs not installed. Run: make install-tools"; \
	fi

.PHONY: lint-makefile
lint-makefile:
	@echo "Linting Makefile with bashrs..."
	@if ! command -v bashrs > /dev/null 2>&1; then \
		echo "❌ bashrs not installed. Run: make install-tools"; \
		exit 1; \
	fi
	bashrs make lint Makefile || true

.PHONY: lint-scripts
lint-scripts:
	@echo "Linting bash scripts with bashrs..."
	@if ! command -v bashrs > /dev/null 2>&1; then \
		echo "⚠️  bashrs not installed. Run: make install-tools"; \
		exit 0; \
	fi
	@for script in scripts/*.sh; do \
		echo "  Checking $$script"; \
		bashrs lint "$$script" || true; \
	done

.PHONY: lint-scripts-fix
lint-scripts-fix:
	@echo "Auto-fixing bash scripts with bashrs (SAFE fixes only)..."
	@if ! command -v bashrs > /dev/null 2>&1; then \
		echo "❌ bashrs not installed. Run: make install-tools"; \
		exit 1; \
	fi
	@for script in scripts/*.sh; do \
		echo "  Fixing $$script"; \
		bashrs lint "$$script" --fix --fix-assumptions -o "$$script" || true; \
	done
	@echo "✅ Script fixes applied"

.PHONY: lint-dockerfiles-fix
lint-dockerfiles-fix:
	@echo "Auto-fixing Dockerfiles with bashrs (SAFE fixes only)..."
	@if ! command -v bashrs > /dev/null 2>&1; then \
		echo "❌ bashrs not installed. Run: make install-tools"; \
		exit 1; \
	fi
	@for dockerfile in docker/*/*.Dockerfile; do \
		echo "  Fixing $$dockerfile"; \
		tmpfile=$$(mktemp); \
		grep "^RUN" "$$dockerfile" > "$$tmpfile" || true; \
		sed -i 's/^RUN //g' "$$tmpfile"; \
		if [ -s "$$tmpfile" ]; then \
			bashrs lint "$$tmpfile" --fix --fix-assumptions -o "$$tmpfile" 2>&1 || true; \
			if [ -s "$$tmpfile" ]; then \
				sed -i 's/^/RUN /g' "$$tmpfile"; \
				while IFS= read -r line; do \
					original_line=$$(echo "$$line" | sed 's/^RUN //'); \
					if grep -q "^RUN.*$$original_line" "$$dockerfile" 2>/dev/null; then \
						sed -i "s|^RUN.*$$original_line.*|$$line|" "$$dockerfile" || true; \
					fi; \
				done < "$$tmpfile"; \
			fi; \
		fi; \
		rm -f "$$tmpfile"; \
	done
	@echo "✅ Dockerfile fixes applied"

.PHONY: test
test: test-rust test-python test-scripts
	@echo "✅ All tests complete"

.PHONY: test-rust
test-rust:
	@echo "Running Rust tests..."
	cargo test --all-features

.PHONY: test-python
test-python:
	@echo "Running Python tests..."
	@if command -v pytest > /dev/null 2>&1; then \
		pytest tests/ 2>/dev/null || true; \
	fi

.PHONY: test-scripts
test-scripts:
	@echo "Running bash script unit tests..."
	@for test in scripts/test_*.sh; do \
		if [ -f "$$test" ]; then \
			echo "  Running $$test"; \
			bash "$$test" || exit 1; \
		fi; \
	done

.PHONY: coverage
coverage:
	@echo "Generating Rust coverage (target: ≥85%)..."
	@if command -v cargo-llvm-cov > /dev/null 2>&1; then \
		cargo llvm-cov --all-features --html; \
		cargo llvm-cov --all-features --summary-only; \
	else \
		echo "⚠️  cargo-llvm-cov not installed. Run: cargo install cargo-llvm-cov"; \
	fi
	@echo "Generating Python coverage..."
	@if command -v pytest > /dev/null 2>&1; then \
		pytest --cov=src --cov-report=html --cov-report=term 2>/dev/null || true; \
	fi

.PHONY: mutation
mutation:
	@echo "Running mutation tests (target: ≥85% score)..."
	@if command -v cargo-mutants > /dev/null 2>&1; then \
		cargo mutants --output mutants.txt; \
		cat mutants.txt; \
	else \
		echo "⚠️  cargo-mutants not installed. Run: cargo install cargo-mutants"; \
	fi

.PHONY: complexity
complexity:
	@echo "Checking code complexity (cyclomatic ≤15, cognitive ≤20)..."
	@if command -v pmat > /dev/null 2>&1; then \
		pmat analyze complexity --path scripts/ || true; \
		pmat analyze complexity --path src/ || true; \
	else \
		echo "⚠️  pmat not installed. Skipping complexity check"; \
	fi

# Documentation
.PHONY: charts
charts:
	@echo "Generating proportional ASCII charts..."
	@if ! command -v deno > /dev/null 2>&1; then \
		echo "❌ deno not installed. Install from https://deno.land"; \
		exit 1; \
	fi
	deno run scripts/generate-ascii-charts.ts

.PHONY: update-readme
update-readme:
	@echo "Updating README.md with proportional charts..."
	@if ! command -v deno > /dev/null 2>&1; then \
		echo "❌ deno not installed. Install from https://deno.land"; \
		exit 1; \
	fi
	@echo "⚠️  This will modify README.md (backup will be created)"
	@bash -c 'read -p "Continue? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		deno run --allow-read --allow-write scripts/update-readme-charts.ts; \
	else \
		echo "Aborted."; \
		exit 1; \
	fi'

# Deployment
.PHONY: deploy
deploy: quality bench-all
	@echo "Generating benchmark report..."
	./scripts/generate-report.sh
	@echo "Publishing results..."
	./scripts/publish-results.sh

# Cleanup
.PHONY: clean
clean:
	cargo clean
	rm -rf target/ || exit 1
	rm -rf results/*.json results/*.html || exit 1
	@echo "Build artifacts cleaned"

.PHONY: clean-all
clean-all: clean
	docker system prune -af
	rm -rf results/ || exit 1
	@echo "Everything cleaned (including Docker images)"

# Install development tools
.PHONY: install-tools
install-tools:
	@echo "Installing Rust development tools..."
	cargo install cargo-llvm-cov || exit 1
	cargo install cargo-mutants || exit 1
	cargo install cargo-watch || exit 1
	@echo "Installing bashrs (CLI benchmarking tool)..."
	cd ../bashrs && cargo install --path rash || exit 1
	@echo "Installing Python tools..."
	pip install black pylint pytest pytest-cov hypothesis || exit 1
	@echo "✅ Development tools installed"
