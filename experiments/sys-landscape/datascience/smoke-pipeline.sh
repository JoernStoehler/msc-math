#!/usr/bin/env bash
# Low-friction smoke run for the sys-landscape produce -> tables
# surface. All outputs go to a temp directory. The default path is intended to
# stay under about two minutes on the devcontainer and exercises cold/hot
# expensive-computation cache contracts by running ascent producers twice.
#
# Set RUN_CONTINUATION_SMOKE=1 for the older slow continuation integration path.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
WORKDIR="$(mktemp -d)"
PRODUCE_DIR="$WORKDIR/produce"
TABLES_DIR="$WORKDIR/tables"
ASCENT_BUDGET_SECS="${ASCENT_BUDGET_SECS:-5}"
RUN_CONTINUATION_SMOKE="${RUN_CONTINUATION_SMOKE:-0}"

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

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent-general-endpoints.jsonl" \
  --no-db-update \
  --fresh

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent-general-endpoints.jsonl" \
  --no-db-update \
  --fresh

cargo run -p exp-sys-landscape --bin sys-dataset-ascent-product -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent-product-endpoints.jsonl" \
  --no-db-update \
  --fresh

cargo run -p exp-sys-landscape --bin sys-dataset-ascent-product -- \
  --n 1 \
  --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
  --out "$PRODUCE_DIR/ascent-product-endpoints.jsonl" \
  --no-db-update \
  --fresh

if [[ "$RUN_CONTINUATION_SMOKE" == "1" ]]; then
  cargo run -p exp-sys-landscape --bin sys-dataset-continuation -- \
    --smoke \
    --out "$PRODUCE_DIR/continuation.jsonl" \
    --cache "$PRODUCE_DIR/continuation-cache.jsonl" \
    --ascent-input "$PRODUCE_DIR/ascent-general-endpoints.jsonl"
else
  : > "$PRODUCE_DIR/continuation.jsonl"
  : > "$PRODUCE_DIR/continuation-cache.jsonl"
fi

test -s "$PRODUCE_DIR/ascent-general-cache.jsonl"
test -s "$PRODUCE_DIR/ascent-general-computed-polytopes.jsonl"
test -s "$PRODUCE_DIR/ascent-general-ascent-events.jsonl"
test -s "$PRODUCE_DIR/ascent-general-expensive-computations-cache.jsonl"
test -s "$PRODUCE_DIR/ascent-product-cache.jsonl"
test -s "$PRODUCE_DIR/ascent-product-computed-polytopes.jsonl"
test -s "$PRODUCE_DIR/ascent-product-ascent-events.jsonl"
test -s "$PRODUCE_DIR/ascent-product-expensive-computations-cache.jsonl"
test "$(wc -l < "$PRODUCE_DIR/ascent-general-endpoints.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-general-cache.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-product-endpoints.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-product-cache.jsonl")" -eq 1

cargo run -p exp-sys-landscape --bin sys-dataset -- \
  --out-dir "$TABLES_DIR" \
  --produce-dir "$PRODUCE_DIR"

test -s "$TABLES_DIR/polytope-table.jsonl"
test -s "$TABLES_DIR/computed-polytope-observation-table.jsonl"
test -s "$TABLES_DIR/polytope-provenance-table.jsonl"
test -s "$TABLES_DIR/polytope-ascent-run-table.jsonl"
computed_input_rows="$(
  wc -l \
    "$PRODUCE_DIR/ascent-general-computed-polytopes.jsonl" \
    "$PRODUCE_DIR/ascent-product-computed-polytopes.jsonl" \
    | awk 'END {print $1}'
)"
computed_observation_rows="$(wc -l < "$TABLES_DIR/computed-polytope-observation-table.jsonl")"
test "$computed_observation_rows" -eq "$computed_input_rows"
python3 - "$TABLES_DIR/polytope-table.jsonl" "$TABLES_DIR/computed-polytope-observation-table.jsonl" <<'PY'
import json
import sys
from pathlib import Path

polytope_ids = {
    json.loads(line)["poly_id"]
    for line in Path(sys.argv[1]).read_text().splitlines()
    if line.strip()
}
missing = []
for line_number, line in enumerate(Path(sys.argv[2]).read_text().splitlines(), start=1):
    if not line.strip():
        continue
    row = json.loads(line)
    if row["poly_id"] not in polytope_ids:
        missing.append((line_number, row["result_id"], row["poly_id"]))
if missing:
    raise SystemExit(f"computed observations missing polytope rows: {missing[:5]}")
PY

uv run --script "$ROOT/experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py" \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --computed-polytope-observation-table "$TABLES_DIR/computed-polytope-observation-table.jsonl"

echo
echo "Smoke outputs:"
echo "  ascent budget: ${ASCENT_BUDGET_SECS}s/seed"
echo "  continuation:  $RUN_CONTINUATION_SMOKE"
echo "  produce:    $PRODUCE_DIR"
echo "  tables:     $TABLES_DIR"
