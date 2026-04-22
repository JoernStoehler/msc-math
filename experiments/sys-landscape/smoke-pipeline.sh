#!/usr/bin/env bash
# Low-friction end-to-end smoke run for the sys-landscape raw -> datasets ->
# methods surface. All outputs go to a temp directory.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WORKDIR="$(mktemp -d)"
RAW_DIR="$WORKDIR/raw"
NORM_DIR="$WORKDIR/normalized"
FEATURE_DIR="$WORKDIR/features"
METHOD_DIR="$WORKDIR/methods"
ASCENT_BUDGET_SECS="${ASCENT_BUDGET_SECS:-5}"

mkdir -p "$RAW_DIR" "$NORM_DIR" "$FEATURE_DIR" "$METHOD_DIR"

echo "Smoke workspace: $WORKDIR"

cargo run -p exp-sys-landscape --bin sys-dataset-random -- \
  --max-f 5 \
  --samples-per-f 1 \
  --out "$RAW_DIR/random.jsonl" \
  --cache "$RAW_DIR/shared-cache.jsonl"

cargo run -p exp-sys-landscape --bin sys-dataset-random-product -- \
  --max-sides 3 \
  --samples-per-bucket 1 \
  --out "$RAW_DIR/random-product.jsonl" \
  --cache "$RAW_DIR/shared-cache.jsonl"

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$RAW_DIR/ascent.jsonl" \
  --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-ascent-product -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$RAW_DIR/ascent-product.jsonl" \
  --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-continuation -- \
  --smoke \
  --out "$RAW_DIR/continuation.jsonl" \
  --cache "$RAW_DIR/continuation-cache.jsonl" \
  --ascent-input "$RAW_DIR/ascent.jsonl"

cargo run -p exp-sys-landscape --bin sys-dataset-core-tables -- \
  --out-dir "$NORM_DIR" \
  --raw-dir "$RAW_DIR"

cargo run -p exp-sys-landscape --bin sys-dataset-features -- \
  --core-tables-dir "$NORM_DIR" \
  --out-dir "$FEATURE_DIR" \
  --continuation-cache "$RAW_DIR/continuation-cache.jsonl"

uv run "$ROOT/experiments/sys-landscape/methods/eda.py" \
  --core-tables-dir "$NORM_DIR" \
  --out-dir "$METHOD_DIR" \
  --polytope-features "$FEATURE_DIR/polytope-features.jsonl"

echo
echo "Smoke outputs:"
echo "  ascent budget: ${ASCENT_BUDGET_SECS}s/seed"
echo "  raw:        $RAW_DIR"
echo "  normalized: $NORM_DIR"
echo "  features:   $FEATURE_DIR"
echo "  methods:    $METHOD_DIR"
