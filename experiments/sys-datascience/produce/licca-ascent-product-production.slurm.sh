#!/bin/bash
# Production shard wave for datascience product fixed-F ascent.
# Submit from this directory with:
#   sbatch licca-ascent-product-production.slurm.sh
#
# Retry rule:
# - Rerun this same script with the same array index and constants.
# - The Rust binary reruns cheap shard control flow and uses read-only
#   expensive-computation cache hits to avoid repeated capacity/orbit search.

#SBATCH --job-name=ds-product
#SBATCH --partition=epyc
#SBATCH --array=0-3
#SBATCH --cpus-per-task=64
#SBATCH --mem=32G
#SBATCH --time=02:00:00
#SBATCH --output=logs/%x-%A_%a.out

# Resource justification:
# - partition=epyc is the CPU-only production partition.
# - 64 CPUs was chosen after `sbatch --test-only` showed the same start time as
#   lower-risk 32 CPU variants and a much earlier start than 128 CPU variants.
# - 1024 seeds per shard keeps the workers busy while preserving the reviewed
#   fixed-F shard topology.
# - 32G memory is conservative headroom above the historical 100 MB per active
#   seed estimate.
# - 2h wall time is conservative over the 1024/64*120s ideal budget bound. The
#   1h variant had the same scheduler start estimate, so 2h reduces timeout and
#   Jörn-loop risk without observed queue-start cost.
# - array=0-3 writes a fresh fixed-F output directory with shard-local
#   expensive-computation cache additions and ascent events.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="$SLURM_CPUS_PER_TASK"

RUN_LABEL="computed-production-product-1024x64"
KIND="product"
SHARD_ID="${SLURM_ARRAY_TASK_ID:-0}"
BASE_N_START=0
SEEDS_PER_SHARD=1024
SEED_TIME_BUDGET_SECS=120
BINARY="$CARGO_TARGET_DIR/release/sys-dataset-ascent-product"
OUT_DIR="experiments/sys-datascience/produce/licca-shards/$KIND-computed-production-1024"
OUT="$OUT_DIR/product-shard-${SHARD_ID}.jsonl"
CACHE_IN="experiments/sys-datascience/produce/expensive-computations-cache.jsonl"
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
