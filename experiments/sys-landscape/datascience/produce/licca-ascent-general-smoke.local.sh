#!/usr/bin/env bash
# Local smoke companion for licca-ascent-general-smoke.slurm.sh.
#
# This runs the same producer binary shape without Slurm and writes a
# LICCA-shaped shard directory under /tmp by default.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
cd "$ROOT"

export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-2}"

RUN_LABEL="local-smoke-general"
KIND="general"
SHARD_ID="${SHARD_ID:-0}"
BASE_N_START="${BASE_N_START:-10}"
SEEDS_PER_SHARD="${SEEDS_PER_SHARD:-1}"
SEED_TIME_BUDGET_SECS="${SEED_TIME_BUDGET_SECS:-1}"
OUT_ROOT="${OUT_ROOT:-$(mktemp -d)}"
OUT_DIR="${OUT_DIR:-$OUT_ROOT/produce/licca-shards/$KIND}"
OUT="$OUT_DIR/general-shard-${SHARD_ID}.jsonl"
CACHE_IN="${EXPENSIVE_COMPUTATIONS_CACHE_IN:-}"
N_START=$((BASE_N_START + SHARD_ID * SEEDS_PER_SHARD))

mkdir -p "$OUT_DIR"

echo "Local datascience ascent shard"
echo "  run label:             $RUN_LABEL"
echo "  kind:                  $KIND"
echo "  shard id:              $SHARD_ID"
echo "  seeds per shard:       $SEEDS_PER_SHARD"
echo "  n-start:               $N_START"
echo "  seed time budget secs: $SEED_TIME_BUDGET_SECS"
echo "  threads:               $RAYON_NUM_THREADS"
echo "  out:                   $OUT"
if [[ -n "$CACHE_IN" && -s "$CACHE_IN" ]]; then
  echo "  expensive cache in:    $CACHE_IN"
  CACHE_ARGS=(--expensive-computations-cache "$CACHE_IN")
else
  echo "  expensive cache in:    <none>"
  CACHE_ARGS=()
fi

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
    --no-db-update \
    --n "$SEEDS_PER_SHARD" \
    --n-start "$N_START" \
    --seed-time-budget-secs "$SEED_TIME_BUDGET_SECS" \
    "${CACHE_ARGS[@]}" \
    --fresh \
    --out "$OUT"

echo "  produce root:          $OUT_ROOT/produce"
