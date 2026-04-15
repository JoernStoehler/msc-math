#!/bin/bash
# Purpose: local smoke runner for the LICCA-bound HKO perturbation job.
# Context: plain bash, no SLURM directives; writes the committed smoke data files.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

BIN="$REPO_ROOT/target/release/hko-perturbation"
if [[ ! -x "$BIN" ]]; then
    cargo build --release -p exp-hko-local-maximum --bin hko-perturbation
fi

N_PER_BUCKET="${N_PER_BUCKET:-20}"
mkdir -p "$SCRIPT_DIR/data"

for eps in 0.001 0.01 0.1; do
    "$BIN" \
        --eps "$eps" \
        --n "$N_PER_BUCKET" \
        --out "$SCRIPT_DIR/data/smoke-eps-${eps}.jsonl"
done
