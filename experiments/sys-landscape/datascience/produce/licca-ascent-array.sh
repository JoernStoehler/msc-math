#!/bin/bash
# Slurm array runner for the datascience fixed-F ascent producer stage.
#
# Submit with explicit --time and --array. The #SBATCH time below is a
# tripwire so accidental production submits without a CLI wall time die quickly.
#
# Smoke examples:
#   sbatch -p test --time=00:10:00 --array=0-0 --export=ALL,KIND=general,SEEDS_PER_SHARD=2,SEED_TIME_BUDGET_SECS=30 licca-ascent-array.sh
#   sbatch -p test --time=00:10:00 --array=0-0 --export=ALL,KIND=product,SEEDS_PER_SHARD=2,SEED_TIME_BUDGET_SECS=30 licca-ascent-array.sh
#
# Production examples:
#   sbatch --time=02:00:00 --array=0-9 --export=ALL,KIND=general,SEEDS_PER_SHARD=50 licca-ascent-array.sh
#   sbatch --time=02:00:00 --array=0-9 --export=ALL,KIND=product,SEEDS_PER_SHARD=50 licca-ascent-array.sh

#SBATCH --job-name=ds-ascent
#SBATCH --partition=epyc
#SBATCH --cpus-per-task=10
#SBATCH --mem=8G
#SBATCH --time=00:00:01
#SBATCH --output=logs/%x-%A_%a.out

# Resource justification:
# - partition=epyc: CPU-only production partition with a 7-day limit.
# - cpus-per-task=10: the ascent binaries use rayon over seed indices; 10 cores
#   matches the historical LICCA fixed-F ascent scripts.
# - mem=8G: historical estimate was about 100 MB per active seed, so 10 threads
#   need about 1 GB; 8 GB leaves headroom for construction and solver spikes.
# - time=00:00:01: tripwire; pass real wall time on the sbatch CLI.
# - array shards: each task writes distinct output files, avoiding shared JSONL
#   write races and making failed shards easy to rerun.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="$SLURM_CPUS_PER_TASK"

KIND="${KIND:-general}"
SEEDS_PER_SHARD="${SEEDS_PER_SHARD:-50}"
SEED_TIME_BUDGET_SECS="${SEED_TIME_BUDGET_SECS:-120}"
SHARD_ID="${SLURM_ARRAY_TASK_ID:-0}"

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
