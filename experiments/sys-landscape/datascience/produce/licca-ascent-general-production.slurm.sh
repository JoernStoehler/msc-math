#!/bin/bash
# Production shard wave for datascience general fixed-F ascent.
# Submit from this directory with:
#   sbatch licca-ascent-general-production.slurm.sh
#
# Retry rule:
# - Rerun this same script with the same array index and constants.
# - The Rust binary reruns cheap shard control flow and uses read-only
#   expensive-computation cache hits to avoid repeated capacity/orbit search.

#SBATCH --job-name=ds-general
#SBATCH --partition=epyc
#SBATCH --array=0-3
#SBATCH --cpus-per-task=128
#SBATCH --mem=32G
#SBATCH --time=06:00:00
#SBATCH --output=logs/%x-%A_%a.out

# Resource justification:
# - partition=epyc is the CPU-only production partition.
# - 128 CPUs uses a full normal epyc node. The binary parallelizes over seed
#   indices, so this only makes sense with a large shard.
# - 1024 seeds per shard keeps 128 Rayon workers busy for most of the run.
# - 32G memory is conservative headroom above the historical 100 MB per active
#   seed estimate.
# - 6h wall time allows heavy-tail seeds. On timeout, rerun this same script.
# - array=0-3 writes a fresh fixed-F output directory with shard-local
#   expensive-computation cache additions and ascent events.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="$SLURM_CPUS_PER_TASK"

RUN_LABEL="computed-production-general-1024x128"
KIND="general"
SHARD_ID="${SLURM_ARRAY_TASK_ID:-0}"
BASE_N_START=0
SEEDS_PER_SHARD=1024
SEED_TIME_BUDGET_SECS=120
BINARY="$CARGO_TARGET_DIR/release/sys-dataset-ascent"
OUT_DIR="experiments/sys-landscape/datascience/produce/licca-shards/$KIND-computed-production-1024"
OUT="$OUT_DIR/general-shard-${SHARD_ID}.jsonl"
CACHE_IN="experiments/sys-landscape/datascience/produce/expensive-computations-cache.jsonl"
N_START=$((BASE_N_START + SHARD_ID * SEEDS_PER_SHARD))

if [[ ! -x "$BINARY" ]]; then
    echo "Missing executable: $BINARY" >&2
    echo "Build it first on LICCA with:" >&2
    echo "  cd ~/msc-math" >&2
    echo "  export CARGO_TARGET_DIR=$CARGO_TARGET_DIR" >&2
    echo "  cargo build --release -p exp-sys-landscape --bin sys-dataset-ascent --bin sys-dataset-ascent-product" >&2
    exit 1
fi

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
if [[ -s "$CACHE_IN" ]]; then
    echo "  expensive cache in:    $CACHE_IN"
    CACHE_ARGS=(--expensive-computations-cache "$CACHE_IN")
else
    echo "  expensive cache in:    <none>"
    CACHE_ARGS=()
fi

"$BINARY" \
    --no-db-update \
    --n "$SEEDS_PER_SHARD" \
    --n-start "$N_START" \
    --seed-time-budget-secs "$SEED_TIME_BUDGET_SECS" \
    "${CACHE_ARGS[@]}" \
    --fresh \
    --out "$OUT"
