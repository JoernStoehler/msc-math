#!/bin/bash
# Status: dormant standalone retained-producer refresh helper. It is not a
# current research or reproduction handoff. Reactivate only after a selected
# C3/reproduction task defines comparison and promotion rules; see ../LICCA.md.
# This job writes review targets, not canonical producer files.

#SBATCH --job-name=ds-random
#SBATCH --partition=epyc
#SBATCH --cpus-per-task=32
#SBATCH --mem=16G
#SBATCH --time=02:00:00
#SBATCH --output=%x-%j.out

# Resource justification:
# - partition=epyc is the CPU-only production partition.
# - 32 CPUs matches the table build resource class and leaves room for Rayon
#   inside capacity/orbit code without asking for a full node.
# - 16G memory leaves headroom above the observed random/product table RSS.
# - 2h wall time is deliberately conservative for 4096 generic random rows plus
#   10240 product rows; timeout wastes a partial review target but does not
#   damage canonical producer files.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="${SLURM_CPUS_PER_TASK:-32}"

PRODUCE_DIR="experiments/sys-datascience/produce"
CANONICAL_CACHE="$PRODUCE_DIR/shared-cache.jsonl"
REVIEW_CACHE="$PRODUCE_DIR/shared-cache-licca-random-refresh.jsonl"
RANDOM_OUT="$PRODUCE_DIR/random-licca-refresh.jsonl"
RANDOM_PRODUCT_OUT="$PRODUCE_DIR/random-product-licca-refresh.jsonl"

GENERAL_SAMPLES_PER_F=512
GENERAL_MAX_F=12
PRODUCT_SAMPLES_PER_BUCKET=1024
PRODUCT_MAX_SIDES=6

mkdir -p "$PRODUCE_DIR"

echo "LICCA datascience random refresh"
echo "  host:                       $(hostname)"
echo "  date:                       $(date)"
echo "  repo:                       $(git rev-parse --short HEAD)"
echo "  cpus:                       ${SLURM_CPUS_PER_TASK:-unknown}"
echo "  rayon threads:              $RAYON_NUM_THREADS"
echo "  cargo target:               $CARGO_TARGET_DIR"
echo "  review random out:          $RANDOM_OUT"
echo "  review random-product out:  $RANDOM_PRODUCT_OUT"
echo "  review shared cache:        $REVIEW_CACHE"
echo "  general samples per F:      $GENERAL_SAMPLES_PER_F"
echo "  general max F:              $GENERAL_MAX_F"
echo "  product samples per bucket: $PRODUCT_SAMPLES_PER_BUCKET"
echo "  product max sides:          $PRODUCT_MAX_SIDES"
echo

if [[ -s "$CANONICAL_CACHE" ]]; then
    cp "$CANONICAL_CACHE" "$REVIEW_CACHE"
else
    : > "$REVIEW_CACHE"
fi

cargo build --release -p exp-sys-landscape \
    --bin sys-dataset-random \
    --bin sys-dataset-random-product

"$CARGO_TARGET_DIR/release/sys-dataset-random" \
    --samples-per-f "$GENERAL_SAMPLES_PER_F" \
    --max-f "$GENERAL_MAX_F" \
    --out "$RANDOM_OUT" \
    --cache "$REVIEW_CACHE"

"$CARGO_TARGET_DIR/release/sys-dataset-random-product" \
    --samples-per-bucket "$PRODUCT_SAMPLES_PER_BUCKET" \
    --max-sides "$PRODUCT_MAX_SIDES" \
    --out "$RANDOM_PRODUCT_OUT" \
    --cache "$REVIEW_CACHE"

echo
echo "Review target row counts:"
wc -l "$RANDOM_OUT" "$RANDOM_PRODUCT_OUT" "$REVIEW_CACHE"
