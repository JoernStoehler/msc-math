#!/bin/bash
# Purpose: local smoke runner for the LICCA-bound general ascent job.
# Context: plain bash, no SLURM directives; writes the committed smoke data files.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

BIN="$REPO_ROOT/target/release/sys-gradient-ascent-general"
if [[ ! -x "$BIN" ]]; then
    cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-general
fi

N="${N:-3}"
"$BIN" \
    --fresh \
    --n "$N" \
    --n-start 0 \
    --no-db-update \
    --out "$SCRIPT_DIR/data/smoke.jsonl"
