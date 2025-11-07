#!/usr/bin/env bash
# Score Dockerfiles using bashrs score
#
# Extracts RUN commands from Dockerfiles and scores the embedded bash scripts
# Usage: ./scripts/score-dockerfiles.sh

set -euo pipefail

echo "=== Dockerfile Quality Scoring (bashrs score) ==="
echo ""

for dockerfile in docker/*/fibonacci.Dockerfile | sort; do
    lang="$(basename "$(dirname "$dockerfile")")"
    echo "Scoring: "${lang}""

    # Extract RUN commands to temporary file
    tmp_script="$(mktemp)"
    grep "^RUN" "$dockerfile" > "$tmp_script" || true
    sed -i 's/^RUN //g' "$tmp_script"

    if [ -s "$tmp_script" ]; then
        # Score with bashrs
        bashrs score "$tmp_script" --detailed 2>&1 || echo "  (No bash commands to score)"
    else
        echo "  No RUN commands found"
    fi

    rm -f "$tmp_script"
    echo ""
done
