#!/usr/bin/env bash
# Low-friction smoke run for the random/product sys-landscape datascience
# source-object produce -> validate -> prepare surface. All outputs go to a
# temp directory.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WORKDIR="$(mktemp -d)"
PRODUCE_DIR="$WORKDIR/produce"
TABLES_DIR="$WORKDIR/tables"

mkdir -p "$PRODUCE_DIR" "$TABLES_DIR"

echo "Smoke workspace: $WORKDIR"

cargo run -p exp-sys-datascience --bin sys-datascience-produce -- \
  --mode smoke \
  --producers random,random-product \
  --output-dir "$PRODUCE_DIR" \
  --parallelism "${DATASCIENCE_SMOKE_PARALLELISM:-4}" \
  --base-cache "$PRODUCE_DIR/base-cache.jsonl"

uv run --script "$ROOT/experiments/sys-datascience/produce/validate-datascience-produced.py" \
  --produce-dir "$PRODUCE_DIR" \
  --mode smoke \
  --producers random,random-product

cargo run -p exp-sys-datascience --bin sys-datascience-prepare -- \
  --produce-dir "$PRODUCE_DIR" \
  --out-dir "$TABLES_DIR"

test -s "$TABLES_DIR/polytope-table.jsonl"
test -s "$TABLES_DIR/polytope-provenance-table.jsonl"

uv run --script "$ROOT/experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py" \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --random-only

echo
echo "Smoke outputs:"
echo "  produce: $PRODUCE_DIR"
echo "  tables:  $TABLES_DIR"
