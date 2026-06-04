#!/bin/bash
# Production shard wave for datascience product fixed-F ascent.
# Submit from this directory with:
#   sbatch licca-ascent-production-product.sh

#SBATCH --job-name=ds-product
#SBATCH --partition=epyc
#SBATCH --array=10-13
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
# - 6h wall time allows heavy-tail seeds. On timeout, rerun this same wrapper;
#   completed summary rows are skipped.
# - array=10-13 continues after the first conservative wave's shard ids 0-9.

set -euo pipefail

export KIND=product
export RUN_LABEL=production-product-1024x128
export BASE_N_START=12
export SEEDS_PER_SHARD=1024
export SEED_TIME_BUDGET_SECS=120

bash "$(dirname "$0")/licca-ascent-array.sh"
