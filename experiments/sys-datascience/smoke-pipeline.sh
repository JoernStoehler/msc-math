#!/usr/bin/env bash
# Low-friction smoke run for the random/product sys-landscape datascience
# produce -> prepare surface. All outputs go to a temp directory.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WORKDIR="$(mktemp -d)"
PRODUCE_DIR="$WORKDIR/produce"
TABLES_DIR="$WORKDIR/tables"

mkdir -p "$PRODUCE_DIR" "$TABLES_DIR"

echo "Smoke workspace: $WORKDIR"

cargo run -p exp-sys-landscape --bin sys-dataset-random -- \
  --max-f 5 \
  --samples-per-f 1 \
  --out "$PRODUCE_DIR/random.jsonl" \
  --cache "$PRODUCE_DIR/shared-cache.jsonl"

cargo run -p exp-sys-landscape --bin sys-dataset-random-product -- \
  --max-sides 3 \
  --samples-per-bucket 1 \
  --out "$PRODUCE_DIR/random-product.jsonl" \
  --cache "$PRODUCE_DIR/shared-cache.jsonl"

cargo run -p exp-sys-landscape --bin sys-dataset -- \
  --random-only \
  --out-dir "$TABLES_DIR" \
  --produce-dir "$PRODUCE_DIR"

test -s "$TABLES_DIR/polytope-table.jsonl"
test -s "$TABLES_DIR/polytope-provenance-table.jsonl"

uv run --script "$ROOT/experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py" \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --computed-polytope-observation-table "$TABLES_DIR/computed-polytope-observation-table.jsonl" \
  --random-only

echo
echo "Smoke outputs:"
echo "  produce: $PRODUCE_DIR"
echo "  tables:  $TABLES_DIR"
