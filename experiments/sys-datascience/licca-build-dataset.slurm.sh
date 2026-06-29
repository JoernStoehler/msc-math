#!/bin/bash
# Build the random/product datascience tables on LICCA.
# Submit from this directory with:
#   sbatch licca-build-dataset.slurm.sh
#
# Inputs:
# - random/product producer files and shared-cache payloads under
#   experiments/sys-datascience/produce/
#
# Output:
# - experiments/sys-datascience/prepare/
# - a table fingerprint printed at the end of the Slurm log

#SBATCH --job-name=ds-table
#SBATCH --partition=epyc
#SBATCH --cpus-per-task=32
#SBATCH --mem=32G
#SBATCH --time=02:00:00
#SBATCH --output=%x-%j.out

# Resource justification:
# - partition=epyc is the CPU-only production partition.
# - The table builder parallelizes row feature construction with Rayon.
# - 32 CPUs gives useful parallelism without taking a full 128-core node and
#   without multiplying memory pressure as much as a 64-thread first try.
# - This job uses --random-only, so feature construction is scoped to the
#   4096 random + 10240 random-product rows needed by the current thesis slice.
# - 32G/2h is a bounded first production request for the richer feature schema;
#   cancel or resubmit only after inspecting sacct/log evidence.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="${SLURM_CPUS_PER_TASK:-32}"

echo "LICCA datascience table build"
echo "  host:           $(hostname)"
echo "  date:           $(date)"
echo "  repo:           $(git rev-parse --short HEAD)"
echo "  cpus:           ${SLURM_CPUS_PER_TASK:-unknown}"
echo "  rayon threads:  $RAYON_NUM_THREADS"
echo "  cargo target:   $CARGO_TARGET_DIR"
echo "  prepare dir:    experiments/sys-datascience/prepare"
echo "  mode:           random/product"
echo

cargo run -p exp-sys-landscape --release --bin sys-dataset -- \
    --random-only \
    --out-dir experiments/sys-datascience/prepare

echo
python3 experiments/sys-datascience/fingerprint-dataset.py \
    experiments/sys-datascience/prepare
