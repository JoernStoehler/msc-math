#!/bin/bash
# Build the retained datascience table dataset on LICCA.
# Submit from this directory with:
#   sbatch licca-build-dataset.slurm.sh
#
# Inputs:
# - canonical producer files under experiments/sys-landscape/datascience/produce/
# - fixed-F ascent producer caches ascent-cache.jsonl and ascent-product-cache.jsonl
#
# Output:
# - experiments/sys-landscape/datascience/dataset/
# - a dataset fingerprint printed at the end of the Slurm log

#SBATCH --job-name=ds-table
#SBATCH --partition=epyc
#SBATCH --cpus-per-task=8
#SBATCH --mem=16G
#SBATCH --time=02:00:00
#SBATCH --output=%x-%j.out

# Resource justification:
# - partition=epyc is the CPU-only production partition.
# - The current table builder is mostly single-core, so 8 CPUs is modest
#   headroom rather than a full-node request.
# - 16G memory leaves room above the observed local 1G RSS table build.
# - 2h wall time covers the current slow feature stage with slack.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"

echo "LICCA datascience table build"
echo "  host:           $(hostname)"
echo "  date:           $(date)"
echo "  repo:           $(git rev-parse --short HEAD)"
echo "  cpus:           ${SLURM_CPUS_PER_TASK:-unknown}"
echo "  cargo target:   $CARGO_TARGET_DIR"
echo "  dataset dir:    experiments/sys-landscape/datascience/dataset"
echo

cargo run -p exp-sys-landscape --release --bin sys-dataset -- \
    --out-dir experiments/sys-landscape/datascience/dataset

echo
python3 experiments/sys-landscape/datascience/fingerprint-dataset.py \
    experiments/sys-landscape/datascience/dataset
