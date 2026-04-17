#!/bin/bash
# Purpose: local smoke runner for the LICCA-bound product ascent job.
# Context: plain bash, no SLURM directives; writes untracked smoke data.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

BIN="$REPO_ROOT/target/release/sys-gradient-ascent-products"
if [[ ! -x "$BIN" ]]; then
    cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-products
fi

N="${N:-3}"
OUT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/sys-gradient-ascent-products-smoke.XXXXXX")"
cleanup() {
    if [[ "${KEEP_SMOKE_OUTPUTS:-0}" == "1" ]]; then
        echo "Retained smoke output: $OUT_DIR"
    else
        rm -rf "$OUT_DIR"
    fi
}
trap cleanup EXIT
"$BIN" \
    --fresh \
    --n "$N" \
    --n-start 0 \
    --no-db-update \
    --out "$OUT_DIR/smoke-gradient-ascent-products.jsonl"

if [[ "${KEEP_SMOKE_OUTPUTS:-0}" == "1" ]]; then
    echo "Smoke output: $OUT_DIR"
else
    echo "Smoke output written to temp dir and deleted on exit: $OUT_DIR"
fi
