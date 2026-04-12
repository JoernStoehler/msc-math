#!/bin/bash
#SBATCH --job-name=ascent-products
#SBATCH --partition=epyc
#SBATCH --cpus-per-task=10
#SBATCH --mem=8G
#SBATCH --time=00:00:01
#SBATCH --output=logs/%x-%j.out

# Resource justification:
# | flag              | value    | why                                                            |
# |-------------------|----------|----------------------------------------------------------------|
# | partition         | epyc     | Production partition, no 4-min cap                             |
# | cpus-per-task     | 10       | rayon par_iter over 10k seeds; 10 cores = ~1.2h wall budget    |
# | mem               | 8G       | ~100MB RSS per seed worst case x 10 threads = ~1G; 8G headroom |
# | time              | 00:00:01 | TRIPWIRE: 1-second default dies instantly if --time= not       |
# |                   |          | passed on CLI. Real wall time must be overridden via           |
# |                   |          | `sbatch --time=HH:MM:SS job.sh`. Reviewer FATAL 2.             |
#
# Architecture B: one task, rayon at dataset level. Per-seed RNG streams guarantee
# byte-reproducibility regardless of thread. No shard-level fault tolerance — if
# the job is evicted, resubmit and load_completed_names resumes from licca.jsonl.
#
# Production submission: `sbatch --time=02:00:00 job.sh` (CLI --time overrides the
# 1-second directive per slurm precedence: CLI > #SBATCH > env > default).
# Test-partition dry run: `sbatch -p test --time=00:03:00 --export=ALL,N=3 job.sh`.

set -euo pipefail
cd "$HOME/msc-math/crates"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
export RAYON_NUM_THREADS="$SLURM_CPUS_PER_TASK"

N="${N:-10000}"
EXP_DIR="exp-sys-landscape/gradient-ascent-products"
mkdir -p "$EXP_DIR/data" "$EXP_DIR/logs"

./target/release/sys-gradient-ascent-products \
    --no-db-update \
    --n "$N" \
    --out "$EXP_DIR/data/licca.jsonl"
