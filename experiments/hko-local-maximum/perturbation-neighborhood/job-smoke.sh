#!/bin/bash
# Purpose: local smoke runner for the LICCA-bound HKO perturbation job.
# Context: plain bash, no SLURM directives; writes untracked smoke data.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

BIN="$REPO_ROOT/target/release/hko-perturbation"
if [[ ! -x "$BIN" ]]; then
    cargo build --release -p exp-hko-local-maximum --bin hko-perturbation
fi

N_PER_BUCKET="${N_PER_BUCKET:-20}"
OUT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hko-perturbation-smoke.XXXXXX")"
cleanup() {
    if [[ "${KEEP_SMOKE_OUTPUTS:-0}" == "1" ]]; then
        echo "Retained smoke output: $OUT_DIR"
    else
        rm -rf "$OUT_DIR"
    fi
}
trap cleanup EXIT

for eps in 0.001 0.01 0.1; do
    "$BIN" \
        --eps "$eps" \
        --n "$N_PER_BUCKET" \
        --out "$OUT_DIR/smoke-eps-${eps}.jsonl"
done

if [[ "${KEEP_SMOKE_OUTPUTS:-0}" == "1" ]]; then
    echo "Smoke output: $OUT_DIR"
else
    echo "Smoke output written to temp dir and deleted on exit: $OUT_DIR"
fi
