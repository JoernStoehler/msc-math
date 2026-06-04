#!/bin/bash
# Smoke shard for datascience product fixed-F ascent.
# Submit from this directory with:
#   sbatch licca-ascent-smoke-product.slurm.sh

#SBATCH --job-name=ds-smoke-product
#SBATCH --partition=test
#SBATCH --array=0-0
#SBATCH --cpus-per-task=10
#SBATCH --mem=8G
#SBATCH --time=00:10:00
#SBATCH --output=logs/%x-%A_%a.out

# Resource justification:
# - partition=test and time=10m keep this as a bounded submit-path check.
# - 10 CPUs matches the conservative production thread count.
# - 2 seeds and 30s per-seed budget exercise output, trace, and resume plumbing.

set -euo pipefail

export KIND=product
export RUN_LABEL=smoke-product
export BASE_N_START=12
export SEEDS_PER_SHARD=2
export SEED_TIME_BUDGET_SECS=30

bash "$(dirname "$0")/_licca-ascent-runner.sh"
