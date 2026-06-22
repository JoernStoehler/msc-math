#!/bin/bash
# Build the retained datascience tables on LICCA.
# Submit from this directory with:
#   sbatch licca-build-dataset.slurm.sh
#
# Inputs:
# - canonical producer files under experiments/sys-datascience/produce/
# - fixed-F ascent producer caches ascent-general-cache.jsonl and ascent-product-cache.jsonl
#
# Output:
# - experiments/sys-datascience/prepare/
# - a table fingerprint printed at the end of the Slurm log

#SBATCH --job-name=ds-table
#SBATCH --partition=epyc
#SBATCH --cpus-per-task=32
#SBATCH --mem=64G
#SBATCH --time=04:00:00
#SBATCH --output=%x-%j.out

# Resource justification:
# - partition=epyc is the CPU-only production partition.
# - The table builder parallelizes row feature construction with Rayon.
# - 32 CPUs gives useful parallelism without taking a full 128-core node and
#   without multiplying memory pressure as much as a 64-thread first try.
# - 64G is a bounded production request for the current feature branch. A local
#   full rebuild on 2026-06-22 loaded the canonical producer caches and then hit
#   the local compute/memory guard while building the table; the older 16G
#   assumption is no longer trusted for the post-feature rebuild gate.
# - 4h wall time covers the current slow feature stage with slack; cancel or
#   resubmit only after inspecting sacct/log evidence against this estimate.

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
echo

cargo run -p exp-sys-landscape --release --bin sys-dataset -- \
    --out-dir experiments/sys-datascience/prepare

echo
python3 experiments/sys-datascience/fingerprint-dataset.py \
    experiments/sys-datascience/prepare
