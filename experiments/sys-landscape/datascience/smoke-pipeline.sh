#!/usr/bin/env bash
# Low-friction smoke run for the sys-landscape produce -> tables -> methods
# surface. All outputs go to a temp directory. The default path is intended to
# stay under about two minutes on the devcontainer and exercises cache/resume
# contracts by running ascent producers twice without deleting temp data.
#
# Set RUN_CONTINUATION_SMOKE=1 for the older slow continuation integration path.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
WORKDIR="$(mktemp -d)"
PRODUCE_DIR="$WORKDIR/produce"
DATASET_DIR="$WORKDIR/dataset"
METHOD_DIR="$WORKDIR/methods"
ASCENT_BUDGET_SECS="${ASCENT_BUDGET_SECS:-5}"
RUN_CONTINUATION_SMOKE="${RUN_CONTINUATION_SMOKE:-0}"

mkdir -p "$PRODUCE_DIR" "$DATASET_DIR" "$METHOD_DIR"

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

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent.jsonl" \
  --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent.jsonl" \
  --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-ascent-product -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent-product.jsonl" \
  --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-ascent-product -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent-product.jsonl" \
  --no-db-update

if [[ "$RUN_CONTINUATION_SMOKE" == "1" ]]; then
  cargo run -p exp-sys-landscape --bin sys-dataset-continuation -- \
    --smoke \
    --out "$PRODUCE_DIR/continuation.jsonl" \
    --cache "$PRODUCE_DIR/continuation-cache.jsonl" \
    --ascent-input "$PRODUCE_DIR/ascent.jsonl"
else
  : > "$PRODUCE_DIR/continuation.jsonl"
  : > "$PRODUCE_DIR/continuation-cache.jsonl"
fi

test -s "$PRODUCE_DIR/ascent-cache.jsonl"
test -s "$PRODUCE_DIR/ascent-product-cache.jsonl"
test "$(wc -l < "$PRODUCE_DIR/ascent.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-cache.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-product.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-product-cache.jsonl")" -eq 1

cargo run -p exp-sys-landscape --bin sys-dataset -- \
  --out-dir "$DATASET_DIR" \
  --produce-dir "$PRODUCE_DIR"

test -s "$DATASET_DIR/polytope-table.jsonl"
test -s "$DATASET_DIR/observation-table.jsonl"

uv run "$ROOT/experiments/sys-landscape/datascience/methods/eda.py" \
  --dataset-dir "$DATASET_DIR" \
  --out-dir "$METHOD_DIR"

echo
echo "Smoke outputs:"
echo "  ascent budget: ${ASCENT_BUDGET_SECS}s/seed"
echo "  continuation:  $RUN_CONTINUATION_SMOKE"
echo "  produce:    $PRODUCE_DIR"
echo "  dataset:    $DATASET_DIR"
echo "  methods:    $METHOD_DIR"
