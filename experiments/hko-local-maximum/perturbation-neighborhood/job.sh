#!/bin/bash
#SBATCH --job-name=hko-perturbation
#SBATCH --partition=epyc
#SBATCH --time=00:30:00
#SBATCH --cpus-per-task=1
#SBATCH --mem=4G
#SBATCH --output=logs/%x-%j.out

# Resource justification (per .claude/skills/slurm/SKILL.md):
# | flag          | value    | why                                                       |
# |---------------|----------|-----------------------------------------------------------|
# | partition     | epyc     | long-form sweep; single task, no queue pressure          |
# | time          | 00:30:00 | 10k * 3 buckets * ~31 ms mean (pentagon-perturb.jsonl     |
# |               |          | 2026-04-12, n=101) ~= 930 s; 2x cushion + build startup  |
# | cpus-per-task | 1        | Single-threaded; hk2017 is inherently sequential per     |
# |               |          | polytope (rust.md forbids rayon inside algorithms)       |
# | mem           | 4G       | Existing runs <50 MB RSS                                 |
# |               |          |                                                          |
# | (no --array)  |          | 3 eps buckets run sequentially in one task; total wall   |
# |               |          | time ~15 min is under the single-task budget             |
#
# Eps grid: 0.001 / 0.01 / 0.1 spans two orders of magnitude. Small enough that
# perturbations land in the same cell complex (no polytope rejection), large
# enough to reach the nonlinear regime at eps=0.1.
#
# Overriding for test-partition dry run:
#   sbatch -p test --time=00:03:00 --export=ALL,N_PER_BUCKET=3 job.sh

set -euo pipefail
cd "$HOME/msc-math/crates"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target

N_PER_BUCKET="${N_PER_BUCKET:-10000}"
EXP_DIR="exp-hko-local-maximum/perturbation-neighborhood"
mkdir -p "$EXP_DIR/data" "$EXP_DIR/logs"

for eps in 0.001 0.01 0.1; do
    echo "=== eps=${eps} ==="
    ./target/release/hko-perturbation \
        --eps "$eps" \
        --n "$N_PER_BUCKET" \
        --out "$EXP_DIR/data/licca-eps-${eps}.jsonl"
done
