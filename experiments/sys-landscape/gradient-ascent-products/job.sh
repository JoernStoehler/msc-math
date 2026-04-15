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
# Caveat: precedence is documented slurm behavior but is site-config dependent
# (some sites wire lua job_submit plugins that rewrite or clamp --time). Before
# first production submit, verify on this cluster with a 1-seed test-partition
# run: `sbatch -p test --time=00:03:00 --export=ALL,N=1 job.sh` should hit its
# own 3-minute limit, not the 1-second #SBATCH directive.
# Before submission, create the log directory from the experiment directory:
# `mkdir -p logs`.
# Test-partition dry run: `sbatch -p test --time=00:03:00 --export=ALL,N=3 job.sh`.

set -euo pipefail
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
export RAYON_NUM_THREADS="$SLURM_CPUS_PER_TASK"

N="${N:-10000}"
EXP_DIR="experiments/sys-landscape/gradient-ascent-products"
BINARY="$CARGO_TARGET_DIR/release/sys-gradient-ascent-products"

if [[ ! -x "$BINARY" ]]; then
    echo "Missing executable: $BINARY" >&2
    echo "Build it first on LICCA with:" >&2
    echo "  cd ~/msc-math && CARGO_TARGET_DIR=$CARGO_TARGET_DIR cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-products" >&2
    exit 1
fi

mkdir -p "$EXP_DIR/data" "$EXP_DIR/logs"

"$BINARY" \
    --no-db-update \
    --n "$N" \
    --out "$EXP_DIR/data/licca.jsonl"
