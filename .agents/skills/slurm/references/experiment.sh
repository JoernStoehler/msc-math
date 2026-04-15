#!/usr/bin/env bash
#===============================================================================
# SLURM job script for msc-math experiments on LICCA
#
# Based on LICCA official docs: https://collab.dvb.bayern/spaces/UniARZHPCKB/
#
# Usage:
#   cd ~/msc-math
#   sbatch experiments/<topic>/<experiment>/job.sh
#
# Copy this template to experiments/<topic>/<experiment>/job.sh and fill in the
# variables marked with TODO.
#===============================================================================

#SBATCH --job-name=TODO_EXPERIMENT_NAME
#SBATCH --partition=epyc
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1           # TODO: 1 for single-threaded, up to 128
#SBATCH --mem=16G                   # TODO: adjust (our experiments rarely need >8G)
#SBATCH --time=04:00:00             # TODO: wall time with 2x safety margin
#SBATCH --output=%x_%j.log          # <job-name>_<job-id>.log
#SBATCH --error=%x_%j.log           # merge stderr into same file

# --- Environment setup ---
set -euo pipefail
source "$HOME/.cargo/env"

# Limit threads to requested CPUs (LICCA best practice)
export OMP_NUM_THREADS=${SLURM_CPUS_PER_TASK:-1}

# --- Build (skip if already built) ---
cd "$HOME/msc-math"
echo "=== Building at $(date) ==="
cargo build --workspace --release 2>&1 | tail -5

# --- Run experiment ---
echo "=== Running at $(date) ==="
echo "Node: $(hostname), CPUs: $SLURM_CPUS_PER_TASK"

# TODO: Replace with the actual binary name and arguments.
# Binary names are defined in `experiments/<group>/Cargo.toml` `[[bin]]` sections.
# Use the compiled binary directly (cargo build already ran above).
srun "$CARGO_TARGET_DIR/release/TODO_BIN_NAME"

echo "=== Done at $(date) ==="
