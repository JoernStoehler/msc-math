#!/bin/bash
# Status: dormant legacy in-place retained-table reproduction helper. It is not
# an active research or submission handoff. Prefer the run-local prepare script
# for new work; use this only for an explicit reproduction/schema-refresh task
# with comparison rules. See LICCA.md.
#
# Inputs:
# - random/product producer files and shared-cache payloads under
#   experiments/polytope-datasets/
#
# Output:
# - experiments/polytope-invariant-table/
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
# - The historical 32G/2h request is retained as provenance, not a current
#   resource recommendation. A new handoff must reassess it.

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
echo "  prepare dir:    experiments/polytope-invariant-table"
echo "  mode:           random/product"
echo

cargo run -p exp-polytope-invariant-table --release --bin sys-dataset -- \
    --random-only \
    --out-dir experiments/polytope-invariant-table

echo
python3 experiments/polytope-invariant-table/fingerprint-dataset.py \
    experiments/polytope-invariant-table
