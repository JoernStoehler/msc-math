#!/bin/bash
# Shared implementation for LICCA fixed-F ascent Slurm wrappers.
#
# Do not submit this file directly. Submit one of:
# - licca-ascent-smoke-general.sh
# - licca-ascent-smoke-product.sh
# - licca-ascent-production-general.sh
# - licca-ascent-production-product.sh
#
# Resume rule:
# - Do not delete partial shard files after timeout.
# - Rerun the same wrapper with the same array index and constants.
# - The Rust binary skips completed summary rows and canonicalizes output after
#   a normal exit.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="$SLURM_CPUS_PER_TASK"

KIND="${KIND:-general}"
SEEDS_PER_SHARD="${SEEDS_PER_SHARD:-50}"
SEED_TIME_BUDGET_SECS="${SEED_TIME_BUDGET_SECS:-120}"
SHARD_ID="${SLURM_ARRAY_TASK_ID:-0}"
RUN_LABEL="${RUN_LABEL:-manual}"

case "$KIND" in
    general)
        BINARY="$CARGO_TARGET_DIR/release/sys-dataset-ascent"
        PREFIX="general"
        BASE_N_START="${BASE_N_START:-10}"
        ;;
    product)
        BINARY="$CARGO_TARGET_DIR/release/sys-dataset-ascent-product"
        PREFIX="product"
        BASE_N_START="${BASE_N_START:-12}"
        ;;
    *)
        echo "KIND must be 'general' or 'product', got: $KIND" >&2
        exit 2
        ;;
esac

if [[ ! -x "$BINARY" ]]; then
    echo "Missing executable: $BINARY" >&2
    echo "Build it first on LICCA with:" >&2
    echo "  cd ~/msc-math" >&2
    echo "  export CARGO_TARGET_DIR=$CARGO_TARGET_DIR" >&2
    echo "  cargo build --release -p exp-sys-landscape --bin sys-dataset-ascent --bin sys-dataset-ascent-product" >&2
    exit 1
fi

N_START=$((BASE_N_START + SHARD_ID * SEEDS_PER_SHARD))
OUT_DIR="experiments/sys-landscape/datascience/produce/licca-shards/$KIND"
OUT="$OUT_DIR/${PREFIX}-shard-${SHARD_ID}.jsonl"

mkdir -p "$OUT_DIR" logs

echo "LICCA datascience ascent shard"
echo "  host:                  $(hostname)"
echo "  date:                  $(date)"
echo "  repo:                  $(git rev-parse --short HEAD)"
echo "  run label:             $RUN_LABEL"
echo "  kind:                  $KIND"
echo "  shard id:              $SHARD_ID"
echo "  seeds per shard:       $SEEDS_PER_SHARD"
echo "  n-start:               $N_START"
echo "  seed time budget secs: $SEED_TIME_BUDGET_SECS"
echo "  threads:               $RAYON_NUM_THREADS"
echo "  binary:                $BINARY"
echo "  out:                   $OUT"

"$BINARY" \
    --no-db-update \
    --n "$SEEDS_PER_SHARD" \
    --n-start "$N_START" \
    --seed-time-budget-secs "$SEED_TIME_BUDGET_SECS" \
    --out "$OUT"
