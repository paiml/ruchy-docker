#!/usr/bin/env bash
# Unit tests for build-all-images.sh
# Uses bashrs test framework

set -euo pipefail

# Test: Script exists and is executable
test_script_exists() {
    [ -f "scripts/build-all-images.sh" ]
    [ -x "scripts/build-all-images.sh" ]
}

# Test: Script has shebang
test_has_shebang() {
    head -1 scripts/build-all-images.sh | grep -q "^#!/usr/bin/env bash"
}

# Test: Script uses set -euo pipefail
test_uses_strict_mode() {
    grep -q "set -euo pipefail" scripts/build-all-images.sh
}

# Test: Script builds all 16 images
test_builds_16_images() {
    local count
    count="$(grep -c "docker build" scripts/build-all-images.sh)"
    [ "$count" -eq 16 ]
}

# Test: Script changes directory for Ruchy builds
test_ruchy_builds_from_parent() {
    grep -q "cd .. && docker build.*ruchy-transpiled" scripts/build-all-images.sh
    grep -q "cd .. && docker build.*ruchy-compiled" scripts/build-all-images.sh
}

# Test: Script builds both fibonacci and primes
test_builds_both_benchmarks() {
    grep -q "fibonacci.Dockerfile" scripts/build-all-images.sh
    grep -q "primes.Dockerfile" scripts/build-all-images.sh
}

# Test: Script builds all 8 languages
test_builds_all_languages() {
    grep -q "c:fibonacci" scripts/build-all-images.sh
    grep -q "rust:fibonacci" scripts/build-all-images.sh
    grep -q "go:fibonacci" scripts/build-all-images.sh
    grep -q "python:fibonacci" scripts/build-all-images.sh
    grep -q "julia:fibonacci" scripts/build-all-images.sh
    grep -q "deno:fibonacci" scripts/build-all-images.sh
    grep -q "ruchy-transpiled:fibonacci" scripts/build-all-images.sh
    grep -q "ruchy-compiled:fibonacci" scripts/build-all-images.sh
}

# Test: Script has helpful output messages
test_has_output_messages() {
    grep -q "Building All Docker Images" scripts/build-all-images.sh
    grep -q "All 16 images built successfully" scripts/build-all-images.sh
}

# Run all tests
echo "Running tests for build-all-images.sh..."
test_script_exists && echo "✓ Script exists and is executable"
test_has_shebang && echo "✓ Has correct shebang"
test_uses_strict_mode && echo "✓ Uses strict mode (set -euo pipefail)"
test_builds_16_images && echo "✓ Builds all 16 images"
test_ruchy_builds_from_parent && echo "✓ Ruchy builds from parent directory"
test_builds_both_benchmarks && echo "✓ Builds both fibonacci and primes"
test_builds_all_languages && echo "✓ Builds all 8 languages"
test_has_output_messages && echo "✓ Has helpful output messages"

echo ""
echo "✅ All tests passed!"
